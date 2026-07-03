//! 🗺️ GIS 2D plugin — standard scaffold app bundled as a hot-swappable WASM component.

use semio_framework_plugin::{
    install_plugin_bundle, register_standard_app, PluginBundle, SceneKind, StandardApp,
};
use std::sync::LazyLock;

static _PLUGIN_INIT: LazyLock<()> = LazyLock::new(|| install_plugin_bundle(bundle()));

fn bundle() -> PluginBundle {
    register_standard_app(
        PluginBundle::new("gis2d", "GIS 2D", "0.1.0"),
        StandardApp {
            app_id: "gis2d-play",
            label: "GIS 2D",
            program_id: Some("gis2d"),
            yields: Some("map"),
            surface_id: "gis2d.play.composite",
            body_key: "gis2d.play.composite",
            scene_kind: SceneKind::Canvas2d,
            initial_document_json: r#"{"schema":"gis2d.document","id":"gis2d","features":[]}"#,
        },
    )
}

semio_framework_plugin::wasm_plugin_exports!();
