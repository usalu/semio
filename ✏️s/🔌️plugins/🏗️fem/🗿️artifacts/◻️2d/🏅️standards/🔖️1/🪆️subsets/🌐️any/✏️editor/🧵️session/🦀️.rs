//! 🧵️ Mounted Fem2d revision job: fixed session arena, retained worker step and live visual lease.

use semio_s_fem_analyses::{FemJobGraph, FemJobStage, FemStagePlan};
use semio_s_fem_analyses_assembly::{AssemblyCsrBuild, AssemblyJob, AssemblyJobConstruction};
use semio_s_fem_analyses_model::AnalysisModel;
use semio_s_artifact_fem_2d_editor_model_window::editor::fem2d::modes::edit::windows::model::Fem2dLiveVisual;
use semio_s_artifact_fem_2d_editor_model_window::editor::fem2d::modes::edit::windows::model::Fem2dMountedVisualLease;
use semio_s_artifact_fem_2d_editor_model_window::editor::fem2d::modes::edit::windows::model::Fem2dVisualFreshness;
use semio_s_artifact_fem_2d_editor_model_window::editor::fem2d::modes::edit::windows::model::Fem2dVisualJob;
use semio_s_artifact_fem_2d_editor_model_window::editor::fem2d::modes::edit::windows::model::FemVisualState;
use semio_s_artifact_fem_2d_editor_model_window::editor::fem2d::modes::edit::windows::model::RegionVisualQuality;
use semio_s_fem_mesh::{MeshJob, MeshOpts, PlanarDomain, TriMesh2};
use semio_s_fem_model::Element;
use semio_s_fem_sparse_pcg::{PcgJob, PcgJobConstruction};
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::snapshot::Fem2dSnapshot;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::FemElement;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::FemLoad;
use semio_framework::kernel::{Effect, JobPlacement};
#[cfg(test)]
use semio_framework_job::CommitValidation;
use semio_framework_job::{InteractiveJob, InteractiveJobCloseStep, JobOutcomeBorrow, Operation, OperationId, RetainedJobPayload, StepContext};
use semio_framework_plugin::reactor::jobs::{BoundedJob, BoundedJobFactory, IoRunControl, JobBudget, JobStep, SqliteSnapshotControl};
use semio_framework_plugin::{AppRenderOperationContext, ArtifactView, PluginLifecycleStep};
use semio_framework_value::{RetainedCloneGrant, RetainedCloneProgress, RetirementDemand, ValueError, ValueRefusalKind};
use std::cell::RefCell;
use std::rc::Rc;

#[path = "🪪️session-constants-and-owner-catalog/🦀️.rs"]
mod session_constants_and_owner_catalog;
pub use session_constants_and_owner_catalog::FEM2D_MOUNTED_JOB_KIND;
use session_constants_and_owner_catalog::{
    MountedProcessOwnerCatalog, FEM2D_JOB_COUNTER_MAXIMUM, FEM2D_JOB_TAG, INPUT_BYTES,
    SESSION_ACTIVE_CAPACITY, SESSION_MAXIMUM_BOUNDARY_POINTS, SESSION_MAXIMUM_ELEMENTS,
    SESSION_MAXIMUM_FAULT_BYTES, SESSION_MAXIMUM_INPUT_BYTES, SESSION_MAXIMUM_INPUT_ITEMS,
    SESSION_MAXIMUM_MESH_POINTS, SESSION_MAXIMUM_MESH_TRIANGLES, SESSION_MAXIMUM_NODES,
    SESSION_MAXIMUM_OUTPUT_BYTES, SESSION_MAXIMUM_REGION_HOLES, SESSION_MAXIMUM_STRING_BYTES,
    SESSION_MAXIMUM_SUPPORTS, SESSION_MAXIMUM_VISUAL_LOADS, SESSION_OWNER_PAGE_BYTES,
    SESSION_PREFLIGHT_UNITS_PER_OPPORTUNITY, SESSION_SHELL_CAPACITY, SESSION_STEP_CEILING_MS,
    SESSION_UNITS_PER_STEP,
};
#[cfg(test)]
use session_constants_and_owner_catalog::MountedOwnerClass;

#[path = "🏗️model-build-job/🦀️.rs"]
mod model_build_job;
use model_build_job::{
    bounded_string_capacities, ModelBuildStage, MountedIdentity, MountedModelBuild, MountedStage,
    SESSION_MAXIMUM_BYTES, SESSION_MAXIMUM_ITEMS,
};

#[path = "🏁️model-build-close/🦀️.rs"]
mod model_build_close;

#[path = "🗂️mounted-state-and-preparation/🦀️.rs"]
mod mounted_state_and_preparation;
use mounted_state_and_preparation::{
    child_end, session_child_failure, session_pending, ChildEnd, MountedState,
    SESSION_CLOSE_COPY_BYTES, SESSION_CLOSE_DEPTH, SESSION_CLOSE_RELEASE_BYTES,
};

#[path = "🔁️mounted-state-step/🦀️.rs"]
mod mounted_state_step;

#[path = "🧹️mounted-state-close/🦀️.rs"]
mod mounted_state_close;

#[path = "📍️snapshot-admission-and-registry/🦀️.rs"]
mod snapshot_admission_and_registry;
use snapshot_admission_and_registry::{
    CurrentSession, MountedRegistry, PendingAdmission, PendingSnapshotAdmission,
    PendingSnapshotFault, SnapshotAdmissionCursor,
};

#[path = "💼️job-bridge-and-live-mount/🦀️.rs"]
mod job_bridge_and_live_mount;
#[cfg(test)]
use job_bridge_and_live_mount::{decode_input, encode_input};
pub use job_bridge_and_live_mount::{
    close_demands, close_step, initialize, maintenance_step, prepare_snapshot_read, reconcile,
    terminal_is_empty, with_live_visual,
};
use job_bridge_and_live_mount::current_identity;

//#region 🧰️FixedArena

thread_local! {
    static MOUNTED: RefCell<MountedRegistry> = RefCell::new(MountedRegistry::new());
}
//#endregion 🧰️FixedArena

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
