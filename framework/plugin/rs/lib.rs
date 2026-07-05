//! 🔌 Declarative app plugin SDK — build fully declarative Rust apps bundled into hot-swappable WASM components.

pub mod app;
pub mod plugin_runtime;
pub mod scaffold;
pub mod world3d_host;

pub use app::{
    App, AppBuilder, AppInstance, KeybindingSpec, ModeSpec, PanelTabSpec, Plugin, PluginApp, PluginBundle,
    WindowKindSpec,
};
pub use plugin_runtime::install_plugin_bundle;
pub use scaffold::{
    assert_standard_app_renders, register_standard_app, scene_kind_component_tag, standard_app,
    standard_factory, SceneKind, StandardApp, StandardPluginApp,
};
pub use world3d_host::{
    default_world3d_selection, export_mesh_glb_bytes, export_mesh_obj, merge_world_selection_ids,
    mesh_kind_from_json, world3d_default_camera, world3d_mesh_id_from_url,
    world3d_meshes_json_from_kinds, world3d_meshes_json_from_kinds_and_urls,
    world3d_meshes_json_from_urls, world3d_scene,
    world3d_selection_json,
};
pub use semio_framework_core::*;

#[macro_export]
macro_rules! register_plugin {
    ($bundle:expr) => {
        $crate::plugin_runtime::install_plugin_bundle($bundle);
    };
}
