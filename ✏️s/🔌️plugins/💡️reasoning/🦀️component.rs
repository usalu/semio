//! 🔌️ Plugin root contract — typestate `Plugin::builder` registration for this owner.

use semio_framework_plugin::kernel::{ActivationEvent, CapabilityId, CapabilityRequest};
use semio_framework_plugin::{ExecutionMode, Plugin};

/// 🔌️ Builds the plugin surface for host registration. `.activation(…)`/`.execution(…)`/
/// `.requests(…)` (ticket 26/08/17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME M6-remaining,
/// `📓️design-abi.md` §3/§6) are this crate's migration proof, mirroring `🗒️note`'s shape. No
/// `.handler(…)` and no `🧩️extensions/` dir anywhere in this crate, so `Isolated` (the SDK default)
/// is honest.
pub async fn plugin() -> Result<Plugin, semio_framework_plugin::PluginAssemblyError> {
    Plugin::builder("reasoning-mindmap")
        .label("Mindmap")
        .version("0.1.0")
        .declare_artifact(crate::artifacts::wires::artifact())
        .editor_mutation_roster::<crate::editor::wires::ReasoningWiresPlayApp>()
        .viewer_mutation_roster::<crate::viewer::wires::WiresViewer>()
        .activation(ActivationEvent::OnArtifactKind { kind: crate::artifacts::wires::artifact_kind().id })
        .execution(ExecutionMode::Isolated)
        .requests(CapabilityRequest { id: CapabilityId("documents.write".into()), scope: "plugin".into(), reason: "persist reasoning wires edits to the open document".into(), optional: false })
        .try_build()
}

//#region 🧪️SurfaceTests
/// 🧪️ Contract §2.5 surface-pair proofs, using the canonical `semio_framework_plugin::testkit`
/// functions (ticket 26/08/16 lane 0-F closed this SDK gap — see `📓️w0-f-report.md`).
#[cfg(test)]
mod surface_tests {
    use semio_framework_plugin::testkit::{assert_editor_and_viewer_share_dialect, assert_viewer_never_mutates};

    #[semio_framework_async_macros::async_test]
    async fn wires_viewer_never_mutates() {
        assert_viewer_never_mutates::<crate::viewer::wires::WiresViewer>();
    }

    #[semio_framework_async_macros::async_test]
    async fn wires_editor_and_viewer_share_dialect() {
        assert_editor_and_viewer_share_dialect::<crate::editor::wires::ReasoningWiresPlayApp, crate::viewer::wires::WiresViewer>();
    }
}
//#endregion 🧪️SurfaceTests
