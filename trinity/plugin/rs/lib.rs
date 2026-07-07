//! 🔺 Trinity plugin — Jack and Rewrite apps in one hot-swappable WASM component.

pub mod app_jack;
pub mod app_rewrite;

use std::sync::LazyLock;

use semio_framework_plugin::{install_plugin_bundle, PluginBundle};

//#region 🔖Bundle
fn bundle() -> PluginBundle {
    PluginBundle::new("trinity", "Trinity", "0.1.0")
        .register_app(app_jack::create_trinity_jack_app(), || Box::new(app_jack::TrinityJackPlayApp))
        .register_app(app_rewrite::create_rewrite_app(), || Box::new(app_rewrite::TrinityRewritePlayApp))
}

static _PLUGIN_INIT: LazyLock<()> = LazyLock::new(|| install_plugin_bundle(bundle()));

semio_framework_plugin::plugin_exports!();
//#endregion 🔖Bundle
