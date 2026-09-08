//! 🔌️ Plugin root contract — typestate `Plugin::builder` registration for this owner.

use semio_framework_plugin::__semio_dispatch_PluginApp;
use semio_framework_plugin::kernel::{ActivationEvent, CapabilityId, CapabilityRequest};
use semio_framework_plugin::plugin_app_close_prelude::*;
use semio_framework_plugin::{ExecutionMode, Plugin, PluginApp};

//#region 🗃️Apps
semio_framework_dispatch_macros::dyn_enum_close! {
    /// 🗃️ Closed runtime app fleet for the shooting editor and viewer surfaces.
    pub enum ShootingApps: PluginApp {
        ShootingEditor(VcsArtifactApp<EditorApp<crate::editor::shooting::ShootingPlayApp>>),
        ShootingViewer(VcsArtifactApp<ViewerApp<crate::viewer::shooting::ShootingViewer>>),
    }
}
//#endregion 🗃️Apps

/// 🔌️ Builds the plugin surface for host registration. `.editor()`/`.viewer()` (ticket
/// 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET) replace the old single `.document_app(…)` call —
/// `ShootingPlayApp::app_schema()` still registers its own CONFIG/PRESENCE schema automatically.
/// `.activation(…)`/`.execution(…)`/`.requests(…)` (ticket
/// 26/08/17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME M6-remaining, `📓️design-abi.md` §3/§6) are this
/// crate's migration proof, mirroring `🗒️note`'s shape.
pub fn plugin() -> Result<Plugin<ShootingApps>, PluginAssemblyError> {
    Plugin::<ShootingApps>::builder("shooting")
        .label("Shooting")
        .version("0.1.0")
        .package_id("semio:shooting")
        .artifact(crate::artifacts::shooting::declaration().map_err(PluginAssemblyError::definition)?)
        .editor::<crate::editor::shooting::ShootingPlayApp>(crate::editor::shooting::create_shooting_app())
        .editor_mutation_roster::<crate::editor::shooting::ShootingPlayApp>()
        .viewer::<crate::viewer::shooting::ShootingViewer>(crate::viewer::shooting::create_shooting_viewer())
        .viewer_mutation_roster::<crate::viewer::shooting::ShootingViewer>()
        .activation(ActivationEvent::OnArtifactKind { kind: crate::artifacts::shooting::artifact_kind().id })
        .execution(ExecutionMode::Isolated)
        .requests(CapabilityRequest { id: CapabilityId("documents.write".into()), scope: "plugin".into(), reason: "persist shooting edits to the open document".into(), optional: false })
        .try_build()
}

//#region 🧪️SurfaceTests
#[cfg(test)]
#[path = "🧪️tests/🔬️surface/🦀️.rs"]
mod surface_tests;
//#endregion 🧪️SurfaceTests
