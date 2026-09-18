//! 🔌️ Plugin root contract — typestate `Plugin::builder` registration for this owner.

extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;

use semio_framework_plugin::__semio_dispatch_PluginApp;
use semio_framework_plugin::kernel::{ActivationEvent, CapabilityId, CapabilityRequest};
use semio_framework_plugin::plugin_app_close_prelude::*;
use semio_framework_plugin::{ExecutionMode, Plugin, PluginApp};

//#region 🗃️Apps
semio_framework_dispatch_macros::dyn_enum_close! {
    /// 🗃️ Closed runtime app fleet for the five wave-function-collapse surfaces — the bitmap
    /// overlapping model, the two tiled grids (2D, 3D) and the two slot graphs (2D, 3D).
    pub enum WfcApps: PluginApp {
        BitmapEditor(VcsArtifactApp<EditorApp<semio_s_artifact_wfc_bitmap::editor::bitmap::BitmapEditor>>),
        BitmapViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_wfc_bitmap::viewer::bitmap::BitmapViewer>>),
        Grid2dEditor(VcsArtifactApp<EditorApp<semio_s_artifact_wfc_grid2d::editor::grid2d::Grid2dEditor>>),
        Grid2dViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_wfc_grid2d::viewer::grid2d::Grid2dViewer>>),
        Wfc2dEditor(VcsArtifactApp<EditorApp<semio_s_artifact_wfc_2d::editor::wfc2d::Wfc2dEditor>>),
        Wfc2dViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_wfc_2d::viewer::wfc2d::Wfc2dViewer>>),
        Grid3dEditor(VcsArtifactApp<EditorApp<semio_s_artifact_wfc_grid3d::editor::grid3d::Grid3dEditor>>),
        Grid3dViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_wfc_grid3d::viewer::grid3d::Grid3dViewer>>),
        Wfc3dEditor(VcsArtifactApp<EditorApp<semio_s_artifact_wfc_3d::editor::wfc3d::Wfc3dEditor>>),
        Wfc3dViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_wfc_3d::viewer::wfc3d::Wfc3dViewer>>),
    }
}
//#endregion 🗃️Apps

/// 💡️ Installs the five resumable solve jobs on the production action bus. Every artifact owns its
/// own `semio.infer` factory (`s.wfc.<ident>.solve`), and a factory that is not installed before the
/// builder runs leaves its `.routed_inference(…)` row addressing a tool nothing can execute.
fn register_inference_factories() -> Result<(), PluginAssemblyError> {
    let bus = ActionBus::production();
    semio_s_artifact_wfc_bitmap::standards::v1::subsets::any::schema::inferences::register_bitmap_inference_factory(&bus).map_err(|error| PluginAssemblyError::new("bitmap-inference-factory", error.to_string()))?;
    semio_s_artifact_wfc_grid2d::standards::v1::subsets::any::schema::inferences::register_grid2d_inference_factory(&bus).map_err(|error| PluginAssemblyError::new("grid2d-inference-factory", error.to_string()))?;
    semio_s_artifact_wfc_2d::standards::v1::subsets::any::schema::inferences::register_wfc2d_inference_factory(&bus).map_err(|error| PluginAssemblyError::new("wfc2d-inference-factory", error.to_string()))?;
    semio_s_artifact_wfc_grid3d::standards::v1::subsets::any::schema::inferences::register_grid3d_inference_factory(&bus).map_err(|error| PluginAssemblyError::new("grid3d-inference-factory", error.to_string()))?;
    semio_s_artifact_wfc_3d::standards::v1::subsets::any::schema::inferences::register_wfc3d_inference_factory(&bus).map_err(|error| PluginAssemblyError::new("wfc3d-inference-factory", error.to_string()))?;
    Ok(())
}

/// 🔌️ Builds the plugin surface for host registration. One `.declare_artifact(…)` channel per
/// artifact (bitmap, grid2d, wfc2d, grid3d, wfc3d), each built by its own artifact engine over the
/// shared `semio-s-plugin-wfc-engine` solver, and one activation per owned artifact kind read live
/// from that kind's own `artifact_kind().id`.
pub fn plugin() -> Result<Plugin<WfcApps>, PluginAssemblyError> {
    register_inference_factories()?;
    Plugin::<WfcApps>::builder("wfc")
        .label("WFC")
        .version("0.1.0")
        .package_id("semio:wfc")
        .routed_inference(semio_s_artifact_wfc_bitmap::standards::v1::subsets::any::schema::inferences::bitmap_inference_metadata())
        .routed_inference(semio_s_artifact_wfc_grid2d::standards::v1::subsets::any::schema::inferences::grid2d_inference_metadata())
        .routed_inference(semio_s_artifact_wfc_2d::standards::v1::subsets::any::schema::inferences::wfc2d_inference_metadata())
        .routed_inference(semio_s_artifact_wfc_grid3d::standards::v1::subsets::any::schema::inferences::grid3d_inference_metadata())
        .routed_inference(semio_s_artifact_wfc_3d::standards::v1::subsets::any::schema::inferences::wfc3d_inference_metadata())
        .declare_artifact(semio_s_artifact_wfc_bitmap::artifact::<WfcApps>())
        .declare_artifact(semio_s_artifact_wfc_grid2d::artifact::<WfcApps>())
        .declare_artifact(semio_s_artifact_wfc_2d::artifact::<WfcApps>())
        .declare_artifact(semio_s_artifact_wfc_grid3d::artifact::<WfcApps>())
        .declare_artifact(semio_s_artifact_wfc_3d::artifact::<WfcApps>())
        .editor_mutation_roster::<semio_s_artifact_wfc_bitmap::editor::bitmap::BitmapEditor>()
        .viewer_mutation_roster::<semio_s_artifact_wfc_bitmap::viewer::bitmap::BitmapViewer>()
        .editor_mutation_roster::<semio_s_artifact_wfc_grid2d::editor::grid2d::Grid2dEditor>()
        .viewer_mutation_roster::<semio_s_artifact_wfc_grid2d::viewer::grid2d::Grid2dViewer>()
        .editor_mutation_roster::<semio_s_artifact_wfc_2d::editor::wfc2d::Wfc2dEditor>()
        .viewer_mutation_roster::<semio_s_artifact_wfc_2d::viewer::wfc2d::Wfc2dViewer>()
        .editor_mutation_roster::<semio_s_artifact_wfc_grid3d::editor::grid3d::Grid3dEditor>()
        .viewer_mutation_roster::<semio_s_artifact_wfc_grid3d::viewer::grid3d::Grid3dViewer>()
        .editor_mutation_roster::<semio_s_artifact_wfc_3d::editor::wfc3d::Wfc3dEditor>()
        .viewer_mutation_roster::<semio_s_artifact_wfc_3d::viewer::wfc3d::Wfc3dViewer>()
        .activation(ActivationEvent::OnArtifactKind { kind: semio_s_artifact_wfc_bitmap::artifact_kind().id })
        .activation(ActivationEvent::OnArtifactKind { kind: semio_s_artifact_wfc_grid2d::artifact_kind().id })
        .activation(ActivationEvent::OnArtifactKind { kind: semio_s_artifact_wfc_2d::artifact_kind().id })
        .activation(ActivationEvent::OnArtifactKind { kind: semio_s_artifact_wfc_grid3d::artifact_kind().id })
        .activation(ActivationEvent::OnArtifactKind { kind: semio_s_artifact_wfc_3d::artifact_kind().id })
        .execution(ExecutionMode::Isolated)
        .requests(CapabilityRequest {
            id: CapabilityId("artifacts.write".into()),
            scope: "plugin".into(),
            reason: "persist bitmap/grid2d/wfc2d/grid3d/wfc3d editor edits (tile authoring, adjacency rules, pins, masks and solve commits) to the open document".into(),
            optional: false,
        })
        .try_build()
}

//#region 🔖️SurfaceTests
#[cfg(test)]
#[path = "🧪️tests/🔬️surface/🦀️.rs"]
mod surface_tests;
//#endregion 🔖️SurfaceTests

#[cfg(feature = "plugin-entry")]
semio_framework_plugin::plugin_exports!(plugin, WfcApps);
