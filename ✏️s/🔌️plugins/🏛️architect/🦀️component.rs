//! 🔌️ Plugin root contract — typestate `Plugin::builder` registration for this owner.

use semio_framework_plugin::Plugin;

/// 🔌️ Builds the plugin surface for host registration. `.editor(…)`/`.viewer(…)` (ticket
/// 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET) replace the old single `.document_app(…)`
/// registration with the two role-carrying surfaces for `s.architect.program@1/*`.
pub fn plugin() -> Result<Plugin, semio_framework_plugin::PluginAssemblyError> {
    Plugin::builder("architect")
        .label("Architect")
        .version("0.1.0")
        .artifact(crate::artifacts::program::declaration().map_err(semio_framework_plugin::PluginAssemblyError::definition)?)
        .editor::<crate::editor::architect::ArchitectPlayApp>(crate::editor::architect::create_architect_app())
        .editor_mutation_roster::<crate::editor::architect::ArchitectPlayApp>()
        .viewer::<crate::viewer::architect::ArchitectViewer>(crate::viewer::architect::create_architect_viewer())
        .viewer_mutation_roster::<crate::viewer::architect::ArchitectViewer>()
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
    fn architect_viewer_never_mutates() {
        assert_viewer_never_mutates::<crate::viewer::architect::ArchitectViewer>();
    }

    #[test]
    fn architect_editor_and_viewer_share_dialect() {
        assert_editor_and_viewer_share_dialect::<crate::editor::architect::ArchitectPlayApp, crate::viewer::architect::ArchitectViewer>();
    }
}
//#endregion 🧪️SurfaceTests
