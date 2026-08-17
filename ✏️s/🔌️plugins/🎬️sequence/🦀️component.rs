//! 🔌️ Plugin root contract — typestate `Plugin::builder` registration for this owner.

use semio_framework_plugin::Plugin;

/// 🔌️ Builds the plugin surface for host registration.
pub fn plugin() -> Result<Plugin, semio_framework_plugin::PluginAssemblyError> {
    Plugin::builder("sequence")
        .label("Sequence")
        .version("0.1.0")
        .artifact(crate::artifacts::sequence::declaration().map_err(semio_framework_plugin::PluginAssemblyError::definition)?)
        .editor::<crate::editor::sequence::SequencePlayApp>(crate::editor::sequence::create_sequence_app())
        .editor_mutation_roster::<crate::editor::sequence::SequencePlayApp>()
        .viewer::<crate::viewer::sequence::SequenceViewer>(crate::viewer::sequence::create_sequence_viewer())
        .viewer_mutation_roster::<crate::viewer::sequence::SequenceViewer>()
        .try_build()
}

//#region 🧪️SurfaceTests
/// 🧪️ Contract §2.5's canonical surface testkit — landed by the ticket's W0-F SDK-gap-closure lane
/// (`📓️w0-f-report.md`), used directly rather than a local stand-in.
#[cfg(test)]
mod surface_tests {
    #[test]
    fn sequence_viewer_never_mutates() {
        semio_framework_plugin::testkit::assert_viewer_never_mutates::<crate::viewer::sequence::SequenceViewer>();
    }

    #[test]
    fn sequence_editor_and_viewer_share_dialect() {
        semio_framework_plugin::testkit::assert_editor_and_viewer_share_dialect::<crate::editor::sequence::SequencePlayApp, crate::viewer::sequence::SequenceViewer>();
    }
}
//#endregion 🧪️SurfaceTests
