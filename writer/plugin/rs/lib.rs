//! ✍️ Writer plugin — standard scaffold app bundled as a hot-swappable WASM component.

use semio_framework_plugin::{
    install_plugin_bundle, register_standard_app, PluginBundle, SceneKind, StandardApp,
};
use std::sync::LazyLock;

static _PLUGIN_INIT: LazyLock<()> = LazyLock::new(|| install_plugin_bundle(bundle()));

fn bundle() -> PluginBundle {
    register_standard_app(
        PluginBundle::new("writer", "Writer", "0.1.0"),
        StandardApp {
            app_id: "writer-play",
            label: "Writer",
            program_id: Some("writer"),
            yields: Some("text"),
            surface_id: "writer.play.composite",
            body_key: "writer.play.composite",
            scene_kind: SceneKind::TextEditor,
            initial_document_json: r#"{"schema":"writer.document","id":"writer","text":""}"#,
        },
    )
}

semio_framework_plugin::wasm_plugin_exports!();
