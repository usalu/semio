//! 🎛️ S Studio plugin — standard scaffold app bundled as a hot-swappable WASM component.

use semio_framework_plugin::{
    install_plugin_bundle, register_standard_app, PluginBundle, SceneKind, StandardApp,
};
use std::sync::LazyLock;

static _PLUGIN_INIT: LazyLock<()> = LazyLock::new(|| install_plugin_bundle(bundle()));

fn bundle() -> PluginBundle {
    register_standard_app(
        PluginBundle::new("s", "S Studio", "0.1.0"),
        StandardApp {
            app_id: "s-play",
            label: "S Studio",
            program_id: Some("s"),
            yields: Some("studio"),
            surface_id: "s.play.composite",
            body_key: "s.play.composite",
            scene_kind: SceneKind::NodeGraph,
            initial_document_json: r#"{"schema":"s.studio","id":"s","graph":{"nodes":[],"edges":[]}}"#,
        },
    )
}

semio_framework_plugin::wasm_plugin_exports!();
