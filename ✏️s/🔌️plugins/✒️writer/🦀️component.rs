//! 🔌️ Plugin root contract — typestate `Plugin::builder` registration for this owner.

use semio_framework_plugin::kernel::{ActivationEvent, CapabilityId, CapabilityRequest};
use semio_framework_plugin::{ExecutionMode, Plugin};

/// 🔌️ Builds the plugin surface for host registration. `.activation(…)`/`.execution(…)`/
/// `.requests(…)` (ticket 26/08/17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME M6-remaining,
/// `📓️design-abi.md` §3/§6) are this crate's migration proof, mirroring `🗒️note`'s shape. No
/// `.handler(…)` and no `🧩️extensions/` dir anywhere in this crate, so `Isolated` (the SDK default)
/// is honest.
pub fn plugin() -> Result<Plugin, semio_framework_plugin::PluginAssemblyError> {
    Plugin::builder("writer")
        .label("Writer")
        .version("0.1.0")
        .artifact(crate::artifacts::writer::declaration().map_err(semio_framework_plugin::PluginAssemblyError::definition)?)
        .editor::<crate::editor::writer::WriterPlayApp>(crate::editor::writer::create_writer_app())
        .editor_mutation_roster::<crate::editor::writer::WriterPlayApp>()
        .viewer::<crate::viewer::writer::WriterViewer>(crate::viewer::writer::create_writer_viewer())
        .viewer_mutation_roster::<crate::viewer::writer::WriterViewer>()
        .activation(ActivationEvent::OnArtifactKind { kind: crate::artifacts::writer::artifact_kind().id })
        .execution(ExecutionMode::Isolated)
        .requests(CapabilityRequest { id: CapabilityId("documents.write".into()), scope: "plugin".into(), reason: "persist writer edits to the open document".into(), optional: false })
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
