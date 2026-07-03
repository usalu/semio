//! 📐 Layout plugin — standard scaffold app bundled as a hot-swappable WASM component.

use semio_framework_plugin::{
    install_plugin_bundle, register_standard_app, PluginBundle, SceneKind, StandardApp,
};
use std::sync::LazyLock;

static _PLUGIN_INIT: LazyLock<()> = LazyLock::new(|| install_plugin_bundle(bundle()));

fn bundle() -> PluginBundle {
    register_standard_app(
        PluginBundle::new("layout", "Layout", "0.1.0"),
        StandardApp {
            app_id: "layout-play",
            label: "Layout",
            program_id: Some("layout"),
            yields: Some("layout"),
            surface_id: "layout.play.composite",
            body_key: "layout.play.composite",
            scene_kind: SceneKind::Canvas2d,
            initial_document_json: r#"{"schema":"layout.document","id":"layout","nodes":[]}"#,
        },
    )
}

semio_framework_plugin::wasm_plugin_exports!();
