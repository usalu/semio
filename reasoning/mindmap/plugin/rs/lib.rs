//! 🗺️ Mindmap plugin — WIRES app in a hot-swappable WASM component.

pub mod app_wires;

use std::sync::LazyLock;

use semio_framework_plugin::{install_plugin_bundle, PluginBundle};

//#region 🔖Bundle
fn bundle() -> PluginBundle {
    PluginBundle::new("reasoning-mindmap", "Mindmap", "0.1.0")
        .register_app(app_wires::create_wires_app(), || Box::new(app_wires::WiresPlayApp))
}

static _PLUGIN_INIT: LazyLock<()> = LazyLock::new(|| install_plugin_bundle(bundle()));

semio_framework_plugin::plugin_exports!();
//#endregion 🔖Bundle
