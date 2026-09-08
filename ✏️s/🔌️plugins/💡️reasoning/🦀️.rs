//! 🔌️ Plugin root contract — typestate `Plugin::builder` registration for this owner.

use semio_framework_plugin::__semio_dispatch_PluginApp;
use semio_framework_plugin::kernel::{ActivationEvent, CapabilityId, CapabilityRequest};
use semio_framework_plugin::plugin_app_close_prelude::*;
use semio_framework_plugin::{ExecutionMode, Plugin, PluginApp};

//#region 🗃️Apps
semio_framework_dispatch_macros::dyn_enum_close! {
    /// 🗃️ Closed runtime app fleet for the declaration-owned reasoning surfaces.
    pub enum ReasoningApps: PluginApp {
        Editor(VcsArtifactApp<EditorApp<crate::editor::wires::ReasoningWiresPlayApp>>),
        Viewer(VcsArtifactApp<ViewerApp<crate::viewer::wires::WiresViewer>>),
    }
}
//#endregion 🗃️Apps

/// 🔌️ Builds the plugin surface for host registration. `.activation(…)`/`.execution(…)`/
/// `.requests(…)` (ticket 26/08/17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME M6-remaining,
/// `📓️design-abi.md` §3/§6) are this crate's migration proof, mirroring `🗒️note`'s shape. No
/// `.handler(…)` and no `🧩️extensions/` dir anywhere in this crate, so `Isolated` (the SDK default)
/// is honest.
pub fn plugin() -> Result<Plugin<ReasoningApps>, PluginAssemblyError> {
    Plugin::<ReasoningApps>::builder("reasoning")
        .label("Mindmap")
        .version("0.1.0")
        .package_id("semio:reasoning")
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
#[path = "🧪️tests/🔬️surface/🦀️.rs"]
mod surface_tests;
//#endregion 🧪️SurfaceTests

//#region 🪪️IdentityTests
/// 🪪️ One law joining every authority that names this plugin, driven by the language-agnostic tuple
/// `🧫️fixtures/🧫️plugin-identity/🔣️.json` (mirrored from the TypeScript side by the registry's
/// `🪪️plugin-identity.test.ts`, which runs the same join for all 59 rows).
#[cfg(test)]
#[path = "🧪️tests/🔬️identity/🦀️.rs"]
mod identity_tests;
//#endregion 🪪️IdentityTests
