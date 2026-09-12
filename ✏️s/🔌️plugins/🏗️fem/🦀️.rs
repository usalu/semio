//! 🔌️ Plugin root contract — typestate `Plugin::builder` registration for this owner.

extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;

use semio_framework_plugin::__semio_dispatch_PluginApp;
use semio_framework_plugin::kernel::{ActivationEvent, CapabilityId, CapabilityRequest};
use semio_framework_plugin::plugin_app_close_prelude::*;
use semio_framework_plugin::{ExecutionMode, Plugin, PluginApp};

//#region 🗃️Apps
semio_framework_dispatch_macros::dyn_enum_close! {
    /// 🗃️ Closed runtime app fleet for the FEM 2D and 3D surfaces.
    pub enum FemApps: PluginApp {
        Fem2dEditor(VcsArtifactApp<EditorApp<semio_s_artifact_fem_2d::editor::fem2d::Fem2dPlayApp>>),
        Fem2dViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_fem_2d::viewer::fem2d::Fem2dViewer>>),
        Fem3dEditor(VcsArtifactApp<EditorApp<semio_s_artifact_fem_3d::editor::fem3d::Fem3dPlayApp>>),
        Fem3dViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_fem_3d::viewer::fem3d::Fem3dViewer>>),
    }
}
//#endregion 🗃️Apps

/// 🔌️ Builds the plugin surface for host registration. `.declare_artifact(…)` (ticket
/// `26/08/17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME`, `terra-descriptors` packet, following the
/// `terra-fleet-trinity-recipe` recipe) replaces the old `.artifact(declaration())`/`.editor()`/
/// `.viewer()` triad for both artifacts — one registration channel for schema/io/viewer/editor rows.
/// `editor::fem2d`/`editor::fem3d` (each still `Fem2dPlayApp`/`Fem3dPlayApp: ArtifactEditor`) and
/// `viewer::fem2d`/`viewer::fem3d` (`Fem2dViewer`/`Fem3dViewer: ArtifactViewer`) stay mounted at the
/// plugin's top-level `editor`/`viewer` modules — every subset registers one editor and one viewer
/// surface via `.editor_mutation_roster()`/`.viewer_mutation_roster()`, orthogonal opt-ins, not a
/// second registration channel. `.activation(…)`/
/// `.execution(…)`/`.requests(…)` (ticket 26/08/17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME
/// M6-remaining, `📓️design-abi.md` §3/§6) are this crate's migration proof, mirroring `🗒️note`'s and
/// `🗄️stdio`'s own shape: one `OnArtifactKind` event per owned kind, read live from each subset's own
/// `computation_artifact_kind().id` (never hardcoded), `Isolated` execution (nothing here justifies a
/// publisher-trusted mode), and one `documents.write` ask covering both editors' persisted mutations.
pub fn plugin() -> Result<Plugin<FemApps>, PluginAssemblyError> {
    semio_s_artifact_fem_2d::editor::fem2d::session::initialize();
    semio_s_artifact_fem_3d::live_visual::initialize();
    Plugin::<FemApps>::builder("fem")
        .label("FEM")
        .version("0.1.0")
        .package_id("semio:fem")
        .declare_artifact(semio_s_artifact_fem_2d::artifact::<FemApps>())
        .declare_artifact(semio_s_artifact_fem_3d::artifact::<FemApps>())
        .editor_mutation_roster::<semio_s_artifact_fem_2d::editor::fem2d::Fem2dPlayApp>()
        .viewer_mutation_roster::<semio_s_artifact_fem_2d::viewer::fem2d::Fem2dViewer>()
        .editor_mutation_roster::<semio_s_artifact_fem_3d::editor::fem3d::Fem3dPlayApp>()
        .viewer_mutation_roster::<semio_s_artifact_fem_3d::viewer::fem3d::Fem3dViewer>()
        .activation(ActivationEvent::OnArtifactKind { kind: semio_s_artifact_fem_2d::computation_artifact_kind().id })
        .activation(ActivationEvent::OnArtifactKind { kind: semio_s_artifact_fem_3d::computation_artifact_kind().id })
        .execution(ExecutionMode::Isolated)
        .requests(CapabilityRequest { id: CapabilityId("documents.write".into()), scope: "plugin".into(), reason: "persist fem2d/fem3d edits to the open document".into(), optional: false })
        .try_build()
}

//#region 🧪️SurfaceTests
/// 🧪️ Contract §2.3/§2.5: each artifact's editor and viewer share one `Dialect`, and the viewer can
/// never mutate the document store. Uses the real framework test context helpers (w0-f gap 2 closure), not
/// local stand-ins.
#[cfg(test)]
#[path = "🧪️tests/🔬️surface/🦀️.rs"]
mod surface_tests;
//#endregion 🧪️SurfaceTests

#[cfg(feature = "plugin-entry")]
semio_framework_plugin::plugin_exports!(plugin, FemApps);
