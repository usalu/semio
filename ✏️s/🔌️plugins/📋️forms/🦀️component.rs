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
/// openQuestion 3) — not a second registration of the artifact/schema/io itself.
/// `.activation(…)`/`.execution(…)`/`.requests(…)` (ticket
/// 26/08/17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME M1, `📓️design-abi.md` §3/§6): the host
/// activates one instance whenever a `"form.dictionary"` artifact
/// (`crate::artifacts::forms::artifact_kind().id`, the old-channel `ArtifactKindSpec` id — kept per
/// debt D1, orthogonal to `.declare_artifact(...)`'s own `ArtifactKindId`) is opened, this plugin's
/// actor runs `Isolated` (no cross-plugin extension attachment, no `.handler(...)` — the SDK
/// default holds), and it asks the broker for document write access because `FormsPlayApp`
/// persists question/field edits back to the open document.
pub fn plugin() -> Result<Plugin, semio_framework_plugin::PluginAssemblyError> {
    Plugin::builder("forms")
        .label("Forms")
        .version("0.1.0")
        .declare_artifact(crate::artifacts::forms::artifact())
        .editor_mutation_roster::<crate::editor::forms::FormsPlayApp>()
        .viewer_mutation_roster::<crate::viewer::forms::FormsViewer>()
        .activation(ActivationEvent::OnArtifactKind { kind: crate::artifacts::forms::artifact_kind().id })
        .execution(ExecutionMode::Isolated)
        .requests(CapabilityRequest { id: CapabilityId("documents.write".into()), scope: "plugin".into(), reason: "persist form dictionary edits to the open document".into(), optional: false })
        .try_build()
}
