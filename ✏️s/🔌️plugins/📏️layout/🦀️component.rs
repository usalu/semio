//! 🔌️ Plugin root contract — typestate `Plugin::builder` registration for this owner.

use semio_framework_plugin::Plugin;

/// 🔌️ Builds the plugin surface for host registration. `.artifact(…)` (ticket
/// 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE M1) replaces the old `.setup(engine::register)`
/// escape hatch; `.setup()` itself is gone (W1c) — `LayoutPlayApp::app_schema()` now answers the
/// one thing it used to survive for, registered automatically by `.editor(…)` below. `.editor(…)` +
/// `.viewer(…)` (ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET) replace the old single
/// `.document_app(…)` registration — the subset's mutation-capable and read-only surfaces are now two
/// independently addressable apps sharing one `LAYOUT_DIALECT` coordinate.
pub fn plugin() -> Result<Plugin, semio_framework_plugin::PluginAssemblyError> {
    Plugin::builder("layout")
        .label("Layout")
        .version("0.1.0")
        .artifact(crate::artifacts::layout::declaration().map_err(semio_framework_plugin::PluginAssemblyError::definition)?)
        .editor::<crate::editor::layout::LayoutPlayApp>(crate::editor::layout::create_layout_app())
        .editor_mutation_roster::<crate::editor::layout::LayoutPlayApp>()
        .viewer::<crate::viewer::layout::LayoutViewer>(crate::viewer::layout::create_layout_viewer())
        .viewer_mutation_roster::<crate::viewer::layout::LayoutViewer>()
        .try_build()
}

//#region 🧪️SurfaceTests
#[cfg(test)]
mod surface_tests {
    //! 👁️✏️ Ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.5 canonical helpers
    //! (`semio_framework_plugin::testkit::{assert_viewer_never_mutates,
    //! assert_editor_and_viewer_share_dialect, new_viewer}`) — closed by lane 0-F (`📓️w0-f-report.md`
    //! Gap 2), used directly here rather than local stand-ins.
    use semio_framework_plugin::testkit::{assert_editor_and_viewer_share_dialect, assert_viewer_never_mutates};

    #[test]
    fn layout_viewer_never_mutates() {
        assert_viewer_never_mutates::<crate::viewer::layout::LayoutViewer>();
    }

    #[test]
    fn layout_editor_and_viewer_share_dialect() {
        assert_editor_and_viewer_share_dialect::<crate::editor::layout::LayoutPlayApp, crate::viewer::layout::LayoutViewer>();
    }
}
//#endregion 🧪️SurfaceTests
