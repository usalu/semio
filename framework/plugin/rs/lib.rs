//! 🔌 Declarative app plugin SDK — build fully declarative Rust apps bundled into hot-swappable WASM components.

pub mod app;
pub mod plugin_runtime;
pub mod scaffold;

pub use app::{
    App, AppBuilder, AppInstance, KeybindingSpec, ModeSpec, PanelTabSpec, Plugin, PluginApp, PluginBundle,
    WindowKindSpec,
};
pub use plugin_runtime::install_plugin_bundle;
pub use scaffold::{
    assert_standard_app_renders, register_standard_app, scene_kind_component_tag, standard_app,
    standard_factory, SceneKind, StandardApp, StandardPluginApp,
};
pub use semio_framework_core::*;

#[macro_export]
macro_rules! register_plugin {
    ($bundle:expr) => {
        $crate::plugin_runtime::install_plugin_bundle($bundle);
    };
}
