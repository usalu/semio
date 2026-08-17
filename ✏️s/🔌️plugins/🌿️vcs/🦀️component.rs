//! 🔌️ Plugin root contract — typestate `Plugin::builder` registration for this owner.

use semio_framework_plugin::Plugin;

/// 🔌️ Builds the plugin surface for host registration.
pub fn plugin() -> Result<Plugin, semio_framework_plugin::PluginAssemblyError> {
    Plugin::builder("vcs")
        .label("VCS")
        .version("0.1.0")
        .artifact(crate::artifacts::vcs::declaration().map_err(semio_framework_plugin::PluginAssemblyError::definition)?)
        .editor::<crate::editor::vcs::VcsPlayApp>(crate::editor::vcs::create_vcs_app())
        .editor_mutation_roster::<crate::editor::vcs::VcsPlayApp>()
        .viewer::<crate::viewer::vcs::VcsViewer>(crate::viewer::vcs::create_vcs_viewer())
        .viewer_mutation_roster::<crate::viewer::vcs::VcsViewer>()
        .try_build()
}

//#region 🧪️SurfaceTests
#[cfg(test)]
mod surface_tests {
    //! 👁️✏️ Ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.5 —
    //! `semio_framework_plugin::testkit::{assert_viewer_never_mutates, assert_editor_and_viewer_share_dialect}`
    //! now exist for real (landed by lane 0-F, see `📓️w0-f-report.md`), so this uses them directly
    //! rather than writing local stand-ins.
    use semio_framework_plugin::testkit::{assert_editor_and_viewer_share_dialect, assert_viewer_never_mutates};

    #[test]
    fn vcs_viewer_never_mutates() {
        assert_viewer_never_mutates::<crate::viewer::vcs::VcsViewer>();
    }

    #[test]
    fn vcs_editor_and_viewer_share_dialect() {
        assert_editor_and_viewer_share_dialect::<crate::editor::vcs::VcsPlayApp, crate::viewer::vcs::VcsViewer>();
    }
}
//#endregion 🧪️SurfaceTests
