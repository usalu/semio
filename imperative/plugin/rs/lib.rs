//! ⚡ Imperative plugin — standard scaffold app bundled as a hot-swappable WASM component.

use semio_framework_plugin::{
    install_plugin_bundle, register_standard_app, PluginBundle, SceneKind, StandardApp,
};
use std::sync::LazyLock;

static _PLUGIN_INIT: LazyLock<()> = LazyLock::new(|| install_plugin_bundle(bundle()));

fn bundle() -> PluginBundle {
    register_standard_app(
        PluginBundle::new("imperative", "Imperative", "0.1.0"),
        StandardApp {
            app_id: "imperative-play",
            label: "Imperative",
            program_id: Some("imperative"),
            yields: Some("graph"),
            surface_id: "imperative.play.composite",
            body_key: "imperative.play.composite",
            scene_kind: SceneKind::NodeGraph,
            initial_document_json: r#"{"nodes":[],"edges":[]}"#,
        },
    )
}

semio_framework_plugin::wasm_plugin_exports!();
