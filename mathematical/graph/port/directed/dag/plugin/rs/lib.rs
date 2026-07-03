//! 🔀 DAG plugin — standard scaffold app bundled as a hot-swappable WASM component.

use semio_framework_plugin::{
    install_plugin_bundle, register_standard_app, PluginBundle, SceneKind, StandardApp,
};
use std::sync::LazyLock;

static _PLUGIN_INIT: LazyLock<()> = LazyLock::new(|| install_plugin_bundle(bundle()));

fn bundle() -> PluginBundle {
    register_standard_app(
        PluginBundle::new("dag", "DAG", "0.1.0"),
        StandardApp {
            app_id: "dag-play",
            label: "DAG",
            program_id: Some("dag"),
            yields: Some("graph"),
            surface_id: "dag.play.composite",
            body_key: "dag.play.composite",
            scene_kind: SceneKind::NodeGraph,
            initial_document_json: r#"{"nodes":[],"edges":[]}"#,
        },
    )
}

semio_framework_plugin::wasm_plugin_exports!();
