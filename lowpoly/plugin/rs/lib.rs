//! 🔺 Lowpoly plugin — standard scaffold app bundled as a hot-swappable WASM component.

use semio_framework_plugin::{
    install_plugin_bundle, register_standard_app, PluginBundle, SceneKind, StandardApp,
};
use std::sync::LazyLock;

static _PLUGIN_INIT: LazyLock<()> = LazyLock::new(|| install_plugin_bundle(bundle()));

fn bundle() -> PluginBundle {
    register_standard_app(
        PluginBundle::new("lowpoly", "Lowpoly", "0.1.0"),
        StandardApp {
            app_id: "lowpoly-play",
            label: "Lowpoly",
            program_id: Some("lowpoly"),
            yields: Some("mesh"),
            surface_id: "lowpoly.play.composite",
            body_key: "lowpoly.play.composite",
            scene_kind: SceneKind::World3d,
            initial_document_json: r#"{"schema":"lowpoly.document","id":"lowpoly","meshes":[]}"#,
        },
    )
}

semio_framework_plugin::wasm_plugin_exports!();
