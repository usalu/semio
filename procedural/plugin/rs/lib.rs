//! 🔧 Procedural plugin — 2D and 3D flow apps in one hot-swappable WASM component.

pub mod app_2d;
pub mod app_3d;

use semio_framework_plugin::PluginBundle;

//#region 🔖Bundle
fn bundle() -> PluginBundle {
    app_2d::register_procedural2d_exports();
    app_3d::register_procedural3d_exports();
    PluginBundle::new("procedural", "Procedural", "0.1.0")
        .register_app(app_2d::create_procedural2d_app(), || Box::new(app_2d::Procedural2dPlayApp))
        .register_app(app_3d::create_procedural3d_app(), || Box::new(app_3d::Procedural3dPlayApp))
}

#[cfg(all(target_arch = "wasm32", target_env = "p2"))]
semio_framework_plugin::plugin_exports!(bundle);
//#endregion 🔖Bundle
