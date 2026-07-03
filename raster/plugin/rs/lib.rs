//! 🖼️ Raster plugin — standard scaffold app bundled as a hot-swappable WASM component.

use semio_framework_plugin::{
    install_plugin_bundle, register_standard_app, PluginBundle, SceneKind, StandardApp,
};
use std::sync::LazyLock;

static _PLUGIN_INIT: LazyLock<()> = LazyLock::new(|| install_plugin_bundle(bundle()));

fn bundle() -> PluginBundle {
    register_standard_app(
        PluginBundle::new("raster", "Raster", "0.1.0"),
        StandardApp {
            app_id: "raster-play",
            label: "Raster",
            program_id: Some("raster"),
            yields: Some("image"),
            surface_id: "raster.play.composite",
            body_key: "raster.play.composite",
            scene_kind: SceneKind::Raster,
            initial_document_json: r#"{"schema":"raster.document","id":"raster","width":1,"height":1}"#,
        },
    )
}

semio_framework_plugin::wasm_plugin_exports!();
