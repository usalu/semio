//! 🔌️ Plugin root contract — typestate `Plugin::builder` registration for this owner.

use semio_framework_plugin::__semio_dispatch_PluginApp;
use semio_framework_plugin::kernel::{ActivationEvent, CapabilityId, CapabilityRequest};
use semio_framework_plugin::plugin_app_close_prelude::*;
use semio_framework_plugin::{ExecutionMode, Plugin, PluginApp};

//#region 🗃️Apps
semio_framework_dispatch_macros::dyn_enum_close! {
    /// 🗃️ Closed runtime app fleet for the flow editor and viewer surfaces.
    pub enum FlowApps: PluginApp {
        FlowEditor(VcsArtifactApp<EditorApp<crate::editor::flow::FlowPlayApp>, semio_s_artifact_stdio_semio::SemioMembers>),
        FlowViewer(VcsArtifactApp<ViewerApp<crate::viewer::flow::FlowViewer>, semio_s_artifact_stdio_semio::SemioMembers>),
    }
}
//#endregion 🗃️Apps

/// 🔌️ Builds the plugin surface for host registration. `.activation(…)`/`.execution(…)`/
/// `.requests(…)` (ticket 26/08/17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME M3, `📓️design-abi.md`
/// §5/§6) are this crate's proof-of-migration: the host activates one instance whenever a
/// `"computation.flow"` artifact (`crate::artifacts::flow::artifact_kind().id`) is opened, this plugin's
/// own actor runs `Isolated` (its 9 `🧩️extensions/` run `Linked` instead — see each extension's own
/// `bundle()`), and it asks the broker for document write access to persist edits.
pub fn plugin() -> Result<Plugin<FlowApps>, PluginAssemblyError> {
    Plugin::<FlowApps>::builder("flow")
        .label("Flow")
        .version("0.1.0")
        .package_id("semio:flow")
        .artifact(crate::artifacts::flow::declaration().map_err(PluginAssemblyError::definition)?)
        .editor_with_members::<crate::editor::flow::FlowPlayApp, semio_s_artifact_stdio_semio::SemioMembers>(crate::editor::flow::create_flow_app())
        .editor_mutation_roster::<crate::editor::flow::FlowPlayApp>()
        .viewer_with_members::<crate::viewer::flow::FlowViewer, semio_s_artifact_stdio_semio::SemioMembers>(crate::viewer::flow::create_flow_viewer())
        .viewer_mutation_roster::<crate::viewer::flow::FlowViewer>()
        .activation(ActivationEvent::OnArtifactKind { kind: crate::artifacts::flow::artifact_kind().id })
        .execution(ExecutionMode::Isolated)
        .requests(CapabilityRequest { id: CapabilityId("documents.write".into()), scope: "plugin".into(), reason: "persist flow graph edits to the open document".into(), optional: false })
        .try_build()
}

//#region 🧪️SurfaceTests
#[cfg(test)]
#[path = "🧪️tests/🔬️surface/🦀️.rs"]
mod surface_tests;
//#endregion 🧪️SurfaceTests
