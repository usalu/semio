//! 🎲 Procedural 3D plugin — standard scaffold app bundled as a hot-swappable WASM component.

use semio_framework_plugin::{
    install_plugin_bundle, register_standard_app, PluginBundle, SceneKind, StandardApp,
};
use std::sync::LazyLock;

static _PLUGIN_INIT: LazyLock<()> = LazyLock::new(|| install_plugin_bundle(bundle()));

fn bundle() -> PluginBundle {
    register_standard_app(
        PluginBundle::new("procedural3d", "Procedural 3D", "0.1.0"),
        StandardApp {
            app_id: "procedural3d-play",
            label: "Procedural 3D",
            program_id: Some("procedural3d"),
            yields: Some("model"),
            surface_id: "procedural3d.play.composite",
            body_key: "procedural3d.play.composite",
            scene_kind: SceneKind::World3d,
            initial_document_json: r#"{"schema":"procedural3d.document","id":"procedural3d","tiles":[]}"#,
        },
    )
}

semio_framework_plugin::wasm_plugin_exports!();
