//! 🔌️ Plugin root contract — typestate `Plugin::builder` registration for this owner.

use semio_framework_plugin::Plugin;

/// 🔌️ Builds the plugin surface for host registration.
pub fn plugin() -> Result<Plugin, semio_framework_plugin::PluginAssemblyError> {
    Plugin::builder("process")
        .label("Process")
        .version("0.1.0")
        .artifact(crate::artifacts::process3d::declaration().map_err(semio_framework_plugin::PluginAssemblyError::definition)?)
        .editor::<crate::editor::process3d::Process3dPlayApp>(crate::editor::process3d::create_process3d_app())
        .editor_mutation_roster::<crate::editor::process3d::Process3dPlayApp>()
        .viewer::<crate::viewer::process3d::Process3dViewer>(crate::viewer::process3d::create_process3d_viewer())
        .viewer_mutation_roster::<crate::viewer::process3d::Process3dViewer>()
        .try_build()
}
