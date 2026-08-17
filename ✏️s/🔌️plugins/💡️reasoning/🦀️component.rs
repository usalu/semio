//! 🔌️ Plugin root contract — typestate `Plugin::builder` registration for this owner.

use semio_framework_plugin::Plugin;

/// 🔌️ Builds the plugin surface for host registration.
pub fn plugin() -> Result<Plugin, semio_framework_plugin::PluginAssemblyError> {
    Plugin::builder("reasoning-mindmap")
        .label("Mindmap")
        .version("0.1.0")
        .artifact(crate::artifacts::wires::declaration().map_err(semio_framework_plugin::PluginAssemblyError::definition)?)
        .editor::<crate::editor::wires::ReasoningWiresPlayApp>(crate::editor::wires::create_wires_app())
        .editor_mutation_roster::<crate::editor::wires::ReasoningWiresPlayApp>()
        .viewer::<crate::viewer::wires::WiresViewer>(crate::viewer::wires::create_wires_viewer())
        .viewer_mutation_roster::<crate::viewer::wires::WiresViewer>()
        .try_build()
}

//#region 🧪️SurfaceTests
/// 🧪️ Contract §2.5 surface-pair proofs, using the canonical `semio_framework_plugin::testkit`
/// functions (ticket 26/08/16 lane 0-F closed this SDK gap — see `📓️w0-f-report.md`).
#[cfg(test)]
mod surface_tests {
    use semio_framework_plugin::testkit::{assert_editor_and_viewer_share_dialect, assert_viewer_never_mutates};

    #[test]
    fn wires_viewer_never_mutates() {
        assert_viewer_never_mutates::<crate::viewer::wires::WiresViewer>();
    }

    #[test]
    fn wires_editor_and_viewer_share_dialect() {
        assert_editor_and_viewer_share_dialect::<crate::editor::wires::ReasoningWiresPlayApp, crate::viewer::wires::WiresViewer>();
    }
}
//#endregion 🧪️SurfaceTests
