//! 🔌️ Plugin root contract — typestate `Plugin::builder` registration for this owner.

use semio_framework_plugin::__semio_dispatch_PluginApp;
use semio_framework_plugin::kernel::{ActivationEvent, CapabilityId, CapabilityRequest};
use semio_framework_plugin::plugin_app_close_prelude::*;
use semio_framework_plugin::{ExecutionMode, HostMediaHandlerDeclaration, Plugin, PluginApp};

//#region 🗃️Apps
semio_framework_dispatch_macros::dyn_enum_close! {
    /// 🗃️ Closed runtime app fleet for the CAD editor and viewer.
    pub enum CadApps: PluginApp {
        Editor(VcsArtifactApp<EditorApp<crate::editor::cad::CadPlayApp>>),
        Viewer(VcsArtifactApp<ViewerApp<crate::viewer::cad::CadViewer>>),
    }
}
//#endregion 🗃️Apps

/// 🔌️ Builds the plugin surface for host registration. `.activation(…)`/`.execution(…)`/
/// `.requests(…)` (ticket 26/08/17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME M2, `📓️design-abi.md`
/// §3) are this crate's proof-of-migration: the host activates one instance whenever a `"3d.cad"`
/// artifact (`crate::artifacts::cad::artifact_kind().id`) is opened, this plugin's actor runs
/// `Isolated` (no publisher trust assumed beyond the sandbox default — nothing in this crate's own
/// effects, all UI-chrome/RPC `Effect` variants with no documented `CapabilityId`, justifies
/// otherwise), and it asks the broker for document write access to persist edits.
pub fn plugin() -> Result<Plugin<CadApps>, PluginAssemblyError> {
    Plugin::<CadApps>::builder("cad")
        .label("CAD")
        .version("0.1.0")
        .package_id("semio:cad")
        .artifact(crate::artifacts::cad::declaration().map_err(PluginAssemblyError::definition)?)
        .host_media_handler(HostMediaHandlerDeclaration::mesh_import("s.cad.host-media.mesh-import", crate::artifacts::cad::artifact_kind(), crate::artifacts::cad::CAD_DOCUMENT_SCHEMA, crate::artifacts::cad::io::cad_document_from_mesh)?)
        .editor::<crate::editor::cad::CadPlayApp>(crate::editor::cad::create_cad_app())
        .editor_mutation_roster::<crate::editor::cad::CadPlayApp>()
        .viewer::<crate::viewer::cad::CadViewer>(crate::viewer::cad::create_cad_viewer())
        .viewer_mutation_roster::<crate::viewer::cad::CadViewer>()
        .activation(ActivationEvent::OnArtifactKind { kind: crate::artifacts::cad::artifact_kind().id })
        .execution(ExecutionMode::Isolated)
        .requests(CapabilityRequest { id: CapabilityId("documents.write".into()), scope: "plugin".into(), reason: "persist cad edits to the open document".into(), optional: false })
        .try_build()
}

//#region 🧪️SurfaceTests
#[cfg(test)]
#[path = "🧪️tests/🔬️surface/🦀️.rs"]
mod surface_tests;
//#endregion 🧪️SurfaceTests

//#region 🧪️AssemblyTests
/// 🧪️ Ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET, lane E2E-ASSEMBLY: `plugin()` must
/// assemble for real (not fall back to the WASM wire's `"assembly-failed"` manifest stub minted by
/// `require_declared_capability_or_record` in `🔌️plugin/🦀️.rs`) and must carry both the
/// editor and viewer app surfaces.
#[cfg(test)]
#[path = "🧪️tests/🔬️assembly/🦀️.rs"]
mod assembly_tests;
//#endregion 🧪️AssemblyTests
