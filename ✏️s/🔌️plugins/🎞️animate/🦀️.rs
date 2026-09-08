//! 🔌️ Plugin root contract — typestate `Plugin::builder` registration for this owner.

use semio_framework_plugin::__semio_dispatch_PluginApp;
use semio_framework_plugin::kernel::{ActivationEvent, CapabilityId, CapabilityRequest};
use semio_framework_plugin::plugin_app_close_prelude::*;
use semio_framework_plugin::{EditorApp, ExecutionMode, Plugin, PluginApp, PluginAssemblyError, VcsArtifactApp, ViewerApp};

//#region 🗃️Apps
// 🗃️ Closed runtime app fleet for the declaration-owned animate surfaces.
semio_framework_dispatch_macros::dyn_enum_close! {
    pub enum AnimateApps: PluginApp {
        PresentationEditor(VcsArtifactApp<EditorApp<crate::editor::animate::AnimatePresentationPlayApp>>),
        PresentationViewer(VcsArtifactApp<ViewerApp<crate::viewer::animate::AnimatePresentationViewer>>),
    }
}
//#endregion 🗃️Apps

/// 🔌️ Builds the plugin surface for host registration. `.activation(…)`/`.execution(…)`/
/// `.requests(…)` (ticket 26/08/17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME M6-remaining,
/// `📓️design-abi.md` §3/§6) are this crate's migration proof, mirroring `🗒️note`'s shape.
pub fn plugin() -> Result<Plugin<AnimateApps>, PluginAssemblyError> {
    Plugin::<AnimateApps>::builder("animate")
        .label("Animate")
        .version("0.1.0")
        .package_id("semio:animate")
        .declare_artifact(crate::artifacts::presentation::artifact::<AnimateApps>())
        .editor_mutation_roster::<crate::editor::animate::AnimatePresentationPlayApp>()
        .viewer_mutation_roster::<crate::viewer::animate::AnimatePresentationViewer>()
        .activation(ActivationEvent::OnArtifactKind { kind: crate::artifacts::presentation::artifact_kind().id })
        .execution(ExecutionMode::Isolated)
        .requests(CapabilityRequest { id: CapabilityId("documents.write".into()), scope: "plugin".into(), reason: "persist animate presentation edits to the open document".into(), optional: false })
        .try_build()
}

//#region 🧪️SurfaceTests
#[cfg(test)]
#[path = "🧪️tests/🔬️surface/🦀️.rs"]
mod surface_tests;
//#endregion 🧪️SurfaceTests
