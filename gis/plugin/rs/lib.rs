//! 🌐 GIS plugin — 2D map app in a hot-swappable WASM component.

pub mod app_2d;

use std::sync::LazyLock;

use semio_framework_plugin::{install_plugin_bundle, PluginBundle};

//#region 🔖Bundle
fn bundle() -> PluginBundle {
    app_2d::register_gis2d_exports();
    PluginBundle::new("gis", "GIS", "0.1.0")
        .register_app(app_2d::create_gis2d_app(), || Box::new(app_2d::Gis2dPlayApp))
}

static _PLUGIN_INIT: LazyLock<()> = LazyLock::new(|| install_plugin_bundle(bundle()));

semio_framework_plugin::plugin_exports!();
//#endregion 🔖Bundle
