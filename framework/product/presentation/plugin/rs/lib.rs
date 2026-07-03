//! 📽️ Presentation plugin — standard scaffold app bundled as a hot-swappable WASM component.

use semio_framework_plugin::{
    install_plugin_bundle, register_standard_app, PluginBundle, SceneKind, StandardApp,
};
use std::sync::LazyLock;

static _PLUGIN_INIT: LazyLock<()> = LazyLock::new(|| install_plugin_bundle(bundle()));

fn bundle() -> PluginBundle {
    register_standard_app(
        PluginBundle::new("presentation", "Presentation", "0.1.0"),
        StandardApp {
            app_id: "presentation-play",
            label: "Presentation",
            program_id: Some("presentation"),
            yields: Some("deck"),
            surface_id: "presentation.play.composite",
            body_key: "presentation.play.composite",
            scene_kind: SceneKind::Canvas2d,
            initial_document_json: r#"{"schema":"presentation.deck","id":"presentation","tiles":[]}"#,
        },
    )
}

semio_framework_plugin::wasm_plugin_exports!();
