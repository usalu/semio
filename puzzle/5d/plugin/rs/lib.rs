//! 🌀 Puzzle 5D plugin — standard scaffold app bundled as a hot-swappable WASM component.

use semio_framework_plugin::{
    install_plugin_bundle, register_standard_app, PluginBundle, SceneKind, StandardApp,
};
use std::sync::LazyLock;

static _PLUGIN_INIT: LazyLock<()> = LazyLock::new(|| install_plugin_bundle(bundle()));

fn bundle() -> PluginBundle {
    register_standard_app(
        PluginBundle::new("puzzle5d", "Puzzle 5D", "0.1.0"),
        StandardApp {
            app_id: "puzzle5d-play",
            label: "Puzzle 5D",
            program_id: Some("puzzle5d"),
            yields: Some("topology"),
            surface_id: "puzzle5d.play.composite",
            body_key: "puzzle5d.play.composite",
            scene_kind: SceneKind::World3d,
            initial_document_json: r#"{"schema":"puzzle5d.document","id":"puzzle5d","cells":[]}"#,
        },
    )
}

semio_framework_plugin::wasm_plugin_exports!();
