//! 🧠 Mindmap Wires plugin — standard scaffold app bundled as a hot-swappable WASM component.

use semio_framework_plugin::{
    install_plugin_bundle, register_standard_app, PluginBundle, SceneKind, StandardApp,
};
use std::sync::LazyLock;

static _PLUGIN_INIT: LazyLock<()> = LazyLock::new(|| install_plugin_bundle(bundle()));

fn bundle() -> PluginBundle {
    register_standard_app(
        PluginBundle::new("reasoning-wires", "Mindmap Wires", "0.1.0"),
        StandardApp {
            app_id: "reasoning-wires-play",
            label: "Mindmap Wires",
            program_id: Some("reasoning-wires"),
            yields: Some("graph"),
            surface_id: "reasoning.wires.composite",
            body_key: "reasoning.wires.composite",
            scene_kind: SceneKind::Canvas2d,
            initial_document_json: r#"{"schema":"reasoning.wires","id":"wires","nodes":[]}"#,
        },
    )
}

semio_framework_plugin::wasm_plugin_exports!();
