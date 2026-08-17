//! 🔌️ Plugin root contract — typestate `Plugin::builder` registration for this owner.

use semio_framework_plugin::Plugin;

/// 🔌️ Builds the plugin surface for host registration.
pub fn plugin() -> Result<Plugin, semio_framework_plugin::PluginAssemblyError> {
    Plugin::builder("writer")
        .label("Writer")
        .version("0.1.0")
        .artifact(crate::artifacts::writer::declaration().map_err(semio_framework_plugin::PluginAssemblyError::definition)?)
        .editor::<crate::editor::writer::WriterPlayApp>(crate::editor::writer::create_writer_app())
        .editor_mutation_roster::<crate::editor::writer::WriterPlayApp>()
        .viewer::<crate::viewer::writer::WriterViewer>(crate::viewer::writer::create_writer_viewer())
        .viewer_mutation_roster::<crate::viewer::writer::WriterViewer>()
        .try_build()
}

//#region 🧪️SurfaceTests
#[cfg(test)]
mod surface_tests {
    //! 👁️✏️ Ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.5's
    //! `semio_framework_plugin::testkit::{assert_viewer_never_mutates, assert_editor_and_viewer_share_dialect,
    //! new_viewer}` landed (W0-F gap closure) — used directly here, no local stand-ins.

    #[test]
    fn writer_viewer_never_mutates() {
        semio_framework_plugin::testkit::assert_viewer_never_mutates::<crate::viewer::writer::WriterViewer>();
    }

    #[test]
    fn writer_editor_and_viewer_share_dialect() {
        semio_framework_plugin::testkit::assert_editor_and_viewer_share_dialect::<crate::editor::writer::WriterPlayApp, crate::viewer::writer::WriterViewer>();
    }
}
//#endregion 🧪️SurfaceTests
