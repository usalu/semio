//! 🧩 Puzzle 2D plugin — standard scaffold app bundled as a hot-swappable WASM component.

use semio_framework_plugin::{
    install_plugin_bundle, register_standard_app, PluginBundle, SceneKind, StandardApp,
};
use std::sync::LazyLock;

static _PLUGIN_INIT: LazyLock<()> = LazyLock::new(|| install_plugin_bundle(bundle()));

fn bundle() -> PluginBundle {
    register_standard_app(
        PluginBundle::new("puzzle2d", "Puzzle 2D", "0.1.0"),
        StandardApp {
            app_id: "puzzle2d-play",
            label: "Puzzle 2D",
            program_id: Some("puzzle2d"),
            yields: Some("layout"),
            surface_id: "puzzle2d.play.composite",
            body_key: "puzzle2d.play.composite",
            scene_kind: SceneKind::Canvas2d,
            initial_document_json: r#"{"schema":"puzzle2d.document","id":"puzzle2d","cells":[]}"#,
        },
    )
}

semio_framework_plugin::wasm_plugin_exports!();
