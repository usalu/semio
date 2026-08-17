//! 🔌️ Plugin root contract — typestate `Plugin::builder` registration for this owner.

use semio_framework_plugin::Plugin;

/// 🔌️ Builds the plugin surface for host registration.
pub fn plugin() -> Result<Plugin, semio_framework_plugin::PluginAssemblyError> {
    Plugin::builder("animate")
        .label("Animate")
        .version("0.1.0")
        .artifact(crate::artifacts::present::declaration().map_err(semio_framework_plugin::PluginAssemblyError::definition)?)
        .editor::<crate::editor::animate::AnimatePresentPlayApp>(crate::editor::animate::create_animate_present_app())
        .editor_mutation_roster::<crate::editor::animate::AnimatePresentPlayApp>()
        .viewer::<crate::viewer::animate::AnimatePresentViewer>(crate::viewer::animate::create_animate_present_viewer())
        .viewer_mutation_roster::<crate::viewer::animate::AnimatePresentViewer>()
        .try_build()
}

//#region 🧪️SurfaceTests
#[cfg(test)]
mod surface_tests {
    //! 👁️✏️ Ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.5 — SDK gap now closed
    //! (`📓️w0-f-report.md`): `semio_framework_plugin::testkit::{assert_viewer_never_mutates,
    //! assert_editor_and_viewer_share_dialect}` are real, exercised directly here.
    use semio_framework_plugin::testkit::{assert_editor_and_viewer_share_dialect, assert_viewer_never_mutates};

    #[test]
    fn animate_viewer_never_mutates() {
        assert_viewer_never_mutates::<crate::viewer::animate::AnimatePresentViewer>();
    }

    #[test]
    fn animate_editor_and_viewer_share_dialect() {
        assert_editor_and_viewer_share_dialect::<crate::editor::animate::AnimatePresentPlayApp, crate::viewer::animate::AnimatePresentViewer>();
    }
}
//#endregion 🧪️SurfaceTests
