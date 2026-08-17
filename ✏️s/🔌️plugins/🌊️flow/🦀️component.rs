//! 🔌️ Plugin root contract — typestate `Plugin::builder` registration for this owner.

use semio_framework_plugin::Plugin;

/// 🔌️ Builds the plugin surface for host registration.
pub fn plugin() -> Result<Plugin, semio_framework_plugin::PluginAssemblyError> {
    Plugin::builder("flow")
        .label("Flow")
        .version("0.1.0")
        .artifact(crate::artifacts::flow::declaration().map_err(semio_framework_plugin::PluginAssemblyError::definition)?)
        .editor::<crate::editor::flow::FlowPlayApp>(crate::editor::flow::create_flow_app())
        .editor_mutation_roster::<crate::editor::flow::FlowPlayApp>()
        .viewer::<crate::viewer::flow::FlowViewer>(crate::viewer::flow::create_flow_viewer())
        .viewer_mutation_roster::<crate::viewer::flow::FlowViewer>()
        .try_build()
}

//#region 🧪️SurfaceTests
#[cfg(test)]
mod surface_tests {
    use crate::editor::flow::FlowPlayApp;
    use crate::viewer::flow::FlowViewer;

    /// 👁️ A viewer instance never mutates the document store, even when dispatched.
    #[test]
    fn flow_viewer_never_mutates() {
        semio_framework_plugin::testkit::assert_viewer_never_mutates::<FlowViewer>();
    }

    /// 🤝️ Editor and viewer surfaces agree on the artifact dialect they address.
    #[test]
    fn flow_editor_and_viewer_share_dialect() {
        semio_framework_plugin::testkit::assert_editor_and_viewer_share_dialect::<FlowPlayApp, FlowViewer>();
    }
}
//#endregion 🧪️SurfaceTests
