//! 🪚 Process plugin — subtractive/additive processing simulation in one hot-swappable WASM component.

pub mod app_3d;

use semio_framework_plugin::PluginBundle;

//#region 🔖Bundle
fn bundle() -> PluginBundle {
    app_3d::register_process3d_exports();
    PluginBundle::new("process", "Process", "0.1.0")
        .register_app(app_3d::create_process3d_app(), || Box::new(app_3d::Process3dPlayApp::default()))
}

#[cfg(all(target_arch = "wasm32", target_env = "p2"))]
semio_framework_plugin::plugin_exports!(bundle);
//#endregion 🔖Bundle
