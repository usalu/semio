//! 🧊 Puzzle 3D plugin — standard scaffold app bundled as a hot-swappable WASM component.

use semio_framework_plugin::{
    install_plugin_bundle, register_standard_app, PluginBundle, SceneKind, StandardApp,
};
use std::sync::LazyLock;

static _PLUGIN_INIT: LazyLock<()> = LazyLock::new(|| install_plugin_bundle(bundle()));

fn bundle() -> PluginBundle {
    register_standard_app(
        PluginBundle::new("puzzle3d", "Puzzle 3D", "0.1.0"),
        StandardApp {
            app_id: "puzzle3d-play",
            label: "Puzzle 3D",
            program_id: Some("puzzle3d"),
            yields: Some("model"),
            surface_id: "puzzle3d.play.composite",
            body_key: "puzzle3d.play.composite",
            scene_kind: SceneKind::World3d,
            initial_document_json: r#"{"schema":"puzzle3d.document","id":"puzzle3d","cells":[]}"#,
        },
    )
}

semio_framework_plugin::wasm_plugin_exports!();
