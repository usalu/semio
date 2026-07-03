//! 🎲 Procedural 2D plugin — standard scaffold app bundled as a hot-swappable WASM component.

use semio_framework_plugin::{
    install_plugin_bundle, register_standard_app, PluginBundle, SceneKind, StandardApp,
};
use std::sync::LazyLock;

static _PLUGIN_INIT: LazyLock<()> = LazyLock::new(|| install_plugin_bundle(bundle()));

fn bundle() -> PluginBundle {
    register_standard_app(
        PluginBundle::new("procedural2d", "Procedural 2D", "0.1.0"),
        StandardApp {
            app_id: "procedural2d-play",
            label: "Procedural 2D",
            program_id: Some("procedural2d"),
            yields: Some("layout"),
            surface_id: "procedural2d.play.composite",
            body_key: "procedural2d.play.composite",
            scene_kind: SceneKind::Canvas2d,
            initial_document_json: r#"{"schema":"procedural2d.document","id":"procedural2d","tiles":[]}"#,
        },
    )
}

semio_framework_plugin::wasm_plugin_exports!();
