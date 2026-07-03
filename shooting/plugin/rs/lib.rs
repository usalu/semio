//! 🎯 Shooting plugin — standard scaffold app bundled as a hot-swappable WASM component.

use semio_framework_plugin::{
    install_plugin_bundle, register_standard_app, PluginBundle, SceneKind, StandardApp,
};
use std::sync::LazyLock;

static _PLUGIN_INIT: LazyLock<()> = LazyLock::new(|| install_plugin_bundle(bundle()));

fn bundle() -> PluginBundle {
    register_standard_app(
        PluginBundle::new("shooting", "Shooting", "0.1.0"),
        StandardApp {
            app_id: "shooting-play",
            label: "Shooting",
            program_id: Some("shooting"),
            yields: Some("scene"),
            surface_id: "shooting.play.composite",
            body_key: "shooting.play.composite",
            scene_kind: SceneKind::World3d,
            initial_document_json: r#"{"schema":"shooting.document","id":"shooting","entities":[]}"#,
        },
    )
}

semio_framework_plugin::wasm_plugin_exports!();
