//! 🔱 Trinity plugin — standard scaffold app bundled as a hot-swappable WASM component.

use semio_framework_plugin::{
    install_plugin_bundle, register_standard_app, PluginBundle, SceneKind, StandardApp,
};
use std::sync::LazyLock;

static _PLUGIN_INIT: LazyLock<()> = LazyLock::new(|| install_plugin_bundle(bundle()));

fn bundle() -> PluginBundle {
    register_standard_app(
        PluginBundle::new("trinity", "Trinity", "0.1.0"),
        StandardApp {
            app_id: "trinity-play",
            label: "Trinity",
            program_id: Some("trinity"),
            yields: Some("code"),
            surface_id: "trinity.play.composite",
            body_key: "trinity.play.composite",
            scene_kind: SceneKind::TextEditor,
            initial_document_json: r#"{"schema":"trinity.document","id":"trinity","source":""}"#,
        },
    )
}

semio_framework_plugin::wasm_plugin_exports!();
