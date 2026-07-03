//! 📝 Note plugin — standard scaffold app bundled as a hot-swappable WASM component.

use semio_framework_plugin::{
    install_plugin_bundle, register_standard_app, PluginBundle, SceneKind, StandardApp,
};
use std::sync::LazyLock;

static _PLUGIN_INIT: LazyLock<()> = LazyLock::new(|| install_plugin_bundle(bundle()));

fn bundle() -> PluginBundle {
    register_standard_app(
        PluginBundle::new("note", "Note", "0.1.0"),
        StandardApp {
            app_id: "note-play",
            label: "Note",
            program_id: Some("note"),
            yields: Some("document"),
            surface_id: "note.play.composite",
            body_key: "note.play.composite",
            scene_kind: SceneKind::Canvas2d,
            initial_document_json: r#"{"schema":"note.document","id":"note","blocks":[]}"#,
        },
    )
}

semio_framework_plugin::wasm_plugin_exports!();
