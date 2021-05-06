use bevy::prelude::*;
use bevy_inspector_egui::{Inspectable, WorldInspectorPlugin};

// if this was defined in another crate, you couldn't implement `Inspectable` for `ThirdPartyComponent`
mod third_party {
    pub struct ThirdPartyComponent;
}

#[derive(Inspectable, Default)]
pub struct MyComponent {
    foo: f32,
    bar: usize,
}

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        .add_startup_system(setup.system())
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn().insert(MyComponent::default());
}
