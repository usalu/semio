//! 📋 Forms plugin — standard scaffold app bundled as a hot-swappable WASM component.

use semio_framework_plugin::{
    install_plugin_bundle, register_standard_app, PluginBundle, SceneKind, StandardApp,
};
use std::sync::LazyLock;

static _PLUGIN_INIT: LazyLock<()> = LazyLock::new(|| install_plugin_bundle(bundle()));

fn bundle() -> PluginBundle {
    register_standard_app(
        PluginBundle::new("forms", "Forms", "0.1.0"),
        StandardApp {
            app_id: "forms-play",
            label: "Forms",
            program_id: Some("forms"),
            yields: Some("data"),
            surface_id: "forms.play.composite",
            body_key: "forms.play.composite",
            scene_kind: SceneKind::Table,
            initial_document_json: r#"{"schema":"forms.document","id":"forms","rows":[]}"#,
        },
    )
}

semio_framework_plugin::wasm_plugin_exports!();
