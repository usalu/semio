//! 🔌️ Plugin root contract — typestate `Plugin::builder` registration for this owner.

use semio_framework_plugin::kernel::{ActivationEvent, CapabilityId, CapabilityRequest};
use semio_framework_plugin::{ExecutionMode, Plugin};

/// 🔌️ Builds the plugin surface for host registration. Atomic cutover (ticket
/// 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM): `.declare_artifact(...)` (new declaration
/// tree) replaces `.artifact(...)`/`.editor::<>()`/`.viewer::<>()` outright — the old channel is
/// NOT kept alongside it (a second parallel registration channel is the compatibility layer this
/// ticket forbids). `.editor_mutation_roster()`/`.viewer_mutation_roster()` stay: they are an
/// orthogonal, still-supported opt-in (`contributor.list-artifact-mutations`) the new declaration
/// tree's `SurfaceDeclaration.mutation_roster` field does not yet wire live (`📓️w1-c-report.md`
/// openQuestion 3) — not a second registration of the artifact/schema/io itself. `.activation(…)`/
/// `.execution(…)`/`.requests(…)` (ticket 26/08/17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME
/// M6-remaining, `📓️design-abi.md` §3/§6) are this crate's migration proof, mirroring `🗒️note`'s
/// shape.
pub async fn plugin() -> Result<Plugin, semio_framework_plugin::PluginAssemblyError> {
    Plugin::builder("sequence")
        .label("Sequence")
        .version("0.1.0")
        .declare_artifact(crate::artifacts::sequence::artifact())
        .editor_mutation_roster::<crate::editor::sequence::SequencePlayApp>()
        .viewer_mutation_roster::<crate::viewer::sequence::SequenceViewer>()
        .activation(ActivationEvent::OnArtifactKind { kind: crate::artifacts::sequence::artifact_kind().id })
        .execution(ExecutionMode::Isolated)
        .requests(CapabilityRequest { id: CapabilityId("documents.write".into()), scope: "plugin".into(), reason: "persist sequence edits to the open document".into(), optional: false })
        .try_build()
}

//#region 🧪️SurfaceTests
/// 🧪️ Contract §2.5's canonical surface testkit — landed by the ticket's W0-F SDK-gap-closure lane
/// (`📓️w0-f-report.md`), used directly rather than a local stand-in.
#[cfg(test)]
mod surface_tests {
    #[semio_framework_async_macros::async_test]
    async fn sequence_viewer_never_mutates() {
        semio_framework_plugin::testkit::assert_viewer_never_mutates::<crate::viewer::sequence::SequenceViewer>();
    }

    #[semio_framework_async_macros::async_test]
    async fn sequence_editor_and_viewer_share_dialect() {
        semio_framework_plugin::testkit::assert_editor_and_viewer_share_dialect::<crate::editor::sequence::SequencePlayApp, crate::viewer::sequence::SequenceViewer>();
    }
}
//#endregion 🧪️SurfaceTests
