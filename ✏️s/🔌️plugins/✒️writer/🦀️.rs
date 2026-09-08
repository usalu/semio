//! 🔌️ Plugin root contract — typestate `Plugin::builder` registration for this owner.

use semio_framework_plugin::__semio_dispatch_PluginApp;
use semio_framework_plugin::kernel::{ActivationEvent, CapabilityId, CapabilityRequest};
use semio_framework_plugin::plugin_app_close_prelude::*;
use semio_framework_plugin::{ExecutionMode, Plugin, PluginApp};

//#region 🗃️Apps
semio_framework_dispatch_macros::dyn_enum_close! {
    /// 🗃️ Closed runtime app fleet for the declaration-owned writer surfaces.
    pub enum WriterApps: PluginApp {
        Editor(VcsArtifactApp<EditorApp<crate::editor::writer::WriterPlayApp>>),
        Viewer(VcsArtifactApp<ViewerApp<crate::viewer::writer::WriterViewer>>),
    }
}
//#endregion 🗃️Apps

/// 🔌️ Builds the plugin surface for host registration. `.activation(…)`/`.execution(…)`/
/// `.requests(…)` (ticket 26/08/17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME M6-remaining,
/// `📓️design-abi.md` §3/§6) are this crate's migration proof, mirroring `🗒️note`'s shape. No
/// `.handler(…)` and no `🧩️extensions/` dir anywhere in this crate, so `Isolated` (the SDK default)
/// is honest.
pub fn plugin() -> Result<Plugin<WriterApps>, PluginAssemblyError> {
    Plugin::<WriterApps>::builder("writer")
        .label("Writer")
        .version("0.1.0")
        .package_id("semio:writer")
        .declare_artifact(crate::artifacts::writer::artifact())
        .editor_mutation_roster::<crate::editor::writer::WriterPlayApp>()
        .viewer_mutation_roster::<crate::viewer::writer::WriterViewer>()
        .activation(ActivationEvent::OnArtifactKind { kind: crate::artifacts::writer::artifact_kind().id })
        .execution(ExecutionMode::Isolated)
        .requests(CapabilityRequest { id: CapabilityId("documents.write".into()), scope: "plugin".into(), reason: "persist writer edits to the open document".into(), optional: false })
        .try_build()
}

//#region 🧪️SurfaceTests
#[cfg(test)]
#[path = "🧪️tests/🔬️surface/🦀️.rs"]
mod surface_tests;
//#endregion 🧪️SurfaceTests
