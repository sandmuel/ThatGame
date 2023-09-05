// Engine components
use bevy::prelude::*;


fn main() {
    let mut app = App::new();
    app.insert_resource(ClearColor(Color::rgb(0., 0., 0.)));
    app.add_plugins(DefaultPlugins
        .set(ImagePlugin::default_nearest())
        .set(WindowPlugin {
            primary_window: Some(Window {
                title: "ThatGame".into(),
                ..default()
            }),
            ..default()
        })
    );
    app.add_systems(Startup, setup);
    app.run();
}

fn setup(mut commands: Commands) {
  commands.spawn(Camera2dBundle::default());
}
