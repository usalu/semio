//! 🧵️ Mounted FEM3D visual publication on the shared bounded-job reactor.

use semio_s_fem_analyses_assembly::AssemblyCsrBuild;
use semio_s_fem_analyses_assembly::AssemblyJob;
use semio_s_fem_analyses_assembly::AssemblyJobConstruction;
use semio_s_fem_analyses_model::MountedAnalysisModel;
use semio_s_fem_analyses_model::MountedAnalysisSupport;
use semio_s_fem_analyses_model::MOUNTED_ANALYSIS_BACKING_BYTES;
use semio_s_fem_mesh::MeshJob;
use semio_s_fem_mesh::MeshOpts;
use semio_s_fem_mesh::MountedPlanarDomain;
use semio_s_fem_model::Dof;
use semio_s_fem_model::Element;
use semio_s_fem_static::Elements;
use semio_s_fem_model::Node;
use semio_s_fem_sparse_ldlt::LdltJob;
use semio_s_fem_sparse::SymmetricPencilConstruction;
use semio_s_fem_sparse::MountedScalarSlots;
use semio_s_fem_sparse_pcg::PcgJob;
use semio_s_fem_sparse_pcg::PcgJobConstruction;
use semio_s_fem_sparse_subspace::SubspaceIterationJob;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::structure::element_id;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::structure::load_id;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::snapshot::Fem3dSnapshot;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::structure::FemElement;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::structure::FemLoad;
use semio_framework::kernel::Effect;
use semio_framework::kernel::JobPlacement;
use semio_framework_job::Generation;
use semio_framework_job::OperationId;
use semio_framework_job::RevisionId;
use semio_framework_job::StepContext;
use semio_framework_plugin::reactor::jobs::BoundedJob;
use semio_framework_plugin::reactor::jobs::BoundedJobFactory;
use semio_framework_plugin::reactor::jobs::IoRunControl;
use semio_framework_plugin::reactor::jobs::JobStep;
use semio_framework_plugin::reactor::jobs::SqliteSnapshotControl;
use semio_framework_plugin::AppRenderOperationContext;
use semio_framework_plugin::ArtifactView;
use semio_framework_plugin::{PluginLifecycleStep, PluginLifecycleFailure};
use semio_framework_value::RetainedCloneGrant;
use semio_framework_value::RetainedCloneProgress;
use semio_framework_value::RetirementDemand;
use semio_framework_value::ValueError;
use semio_framework_value::ValueRefusalKind;
#[cfg(test)]
use semio_framework_ui_scene::world3d_snapshot_with_page;
use semio_framework_ui_scene::world3d_snapshot_recovery_close_step;
use semio_framework_ui_scene::WORLD3D_SNAPSHOT_PAGE_BYTE_CAPACITY;
use semio_framework_ui_scene::WORLD3D_SNAPSHOT_PAGE_ITEM_CAPACITY;
use std::cell::Cell;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Mutex;
use semio_s_artifact_fem_3d_editor_visual_job::editor::fem3d::session::visual_job::FEM3D_MOUNTED_VISUAL_JOB_KIND;
use semio_s_artifact_fem_3d_editor_visual_job::editor::fem3d::session::visual_job::ACTIVE_CAPACITY;
use semio_s_artifact_fem_3d_editor_visual_job::editor::fem3d::session::visual_job::SHELL_CAPACITY;
use semio_s_artifact_fem_3d_editor_visual_job::editor::fem3d::session::visual_job::MAXIMUM_REGIONS;
use semio_s_artifact_fem_3d_editor_visual_job::editor::fem3d::session::visual_job::MAXIMUM_NODES;
use semio_s_artifact_fem_3d_editor_visual_job::editor::fem3d::session::visual_job::MAXIMUM_ELEMENTS;
use semio_s_artifact_fem_3d_editor_visual_job::editor::fem3d::session::visual_job::MAXIMUM_SUPPORTS;
use semio_s_artifact_fem_3d_editor_visual_job::editor::fem3d::session::visual_job::MAXIMUM_LOADS;
use semio_s_artifact_fem_3d_editor_visual_job::editor::fem3d::session::visual_job::MAXIMUM_FIELDS;
use semio_s_artifact_fem_3d_editor_visual_job::editor::fem3d::session::visual_job::STEP_CEILING_MS;
use semio_s_artifact_fem_3d_editor_visual_job::editor::fem3d::session::visual_job::FAULT_BYTES;
use semio_s_artifact_fem_3d_editor_visual_job::editor::fem3d::session::visual_job::JOB_TAG;
use semio_s_artifact_fem_3d_editor_visual_job::editor::fem3d::session::visual_job::JOB_COUNTER_MAXIMUM;
use semio_s_artifact_fem_3d_editor_visual_job::editor::fem3d::session::visual_job::INPUT_BYTES;
use semio_s_artifact_fem_3d_editor_visual_job::editor::fem3d::session::visual_job::FEM3D_SOLVER_SCALAR_PAGE_BYTES;
use semio_s_artifact_fem_3d_editor_visual_job::editor::fem3d::session::visual_job::FEM3D_SOLVER_INITIALIZED_PAGE_BYTES;
use semio_s_artifact_fem_3d_editor_visual_job::editor::fem3d::session::visual_job::FEM3D_REGION_ORDER_BYTES;
use semio_s_artifact_fem_3d_editor_visual_job::editor::fem3d::session::visual_job::FEM3D_ELEMENT_ORDER_BYTES;
use semio_s_artifact_fem_3d_editor_visual_job::editor::fem3d::session::visual_job::FEM3D_PROCESS_BACKING_ITEMS;
use semio_s_artifact_fem_3d_editor_visual_job::editor::fem3d::session::visual_job::FEM3D_PROCESS_BACKING_BYTES;
use semio_s_artifact_fem_3d_editor_visual_job::editor::fem3d::session::visual_job::Fem3dBackingCredit;
use semio_s_artifact_fem_3d_editor_visual_job::editor::fem3d::session::visual_job::Fem3dVisualState;
use semio_s_artifact_fem_3d_editor_visual_job::editor::fem3d::session::visual_job::Fem3dVisualFreshness;
use semio_s_artifact_fem_3d_editor_visual_job::editor::fem3d::session::visual_job::Fem3dVisualJobStage;
use semio_s_artifact_fem_3d_editor_visual_job::editor::fem3d::session::visual_job::FEM3D_VISUAL_PAGES;
use semio_s_artifact_fem_3d_editor_visual_job::editor::fem3d::session::visual_job::FEM3D_VISUAL_LABEL_EN;
use semio_s_artifact_fem_3d_editor_visual_job::editor::fem3d::session::visual_job::FEM3D_VISUAL_LABEL_DE;
use semio_s_artifact_fem_3d_editor_visual_job::editor::fem3d::session::visual_job::Fem3dSolverScalar;
use semio_s_artifact_fem_3d_editor_visual_job::editor::fem3d::session::visual_job::Fem3dSolverView;
use semio_s_artifact_fem_3d_editor_visual_job::editor::fem3d::session::visual_job::Fem3dNumericalStage;
use semio_s_artifact_fem_3d_editor_visual_job::editor::fem3d::session::visual_job::FixedSlots;
use semio_s_artifact_fem_3d_editor_visual_job::editor::fem3d::session::visual_job::ChildEnd;
use semio_s_artifact_fem_3d_editor_visual_job::editor::fem3d::session::visual_job::session_pending;
use semio_s_artifact_fem_3d_editor_visual_job::editor::fem3d::session::visual_job::SESSION_CLOSE_COPY_BYTES;
use semio_s_artifact_fem_3d_editor_visual_job::editor::fem3d::session::visual_job::SESSION_CLOSE_DEPTH;
use semio_s_artifact_fem_3d_editor_visual_job::editor::fem3d::session::visual_job::SESSION_CLOSE_RELEASE_BYTES;
use semio_s_artifact_fem_3d_editor_visual_job::editor::fem3d::session::visual_job::Fem3dNumericalChild;
use semio_s_artifact_fem_3d_editor_visual_job::editor::fem3d::session::visual_job::Fem3dNumericalFailure;
use semio_s_artifact_fem_3d_editor_visual_job::editor::fem3d::session::visual_job::FixedOrder;
use semio_s_artifact_fem_3d_editor_visual_job::editor::fem3d::session::visual_job::close_recovered_fem3d_backing;
use semio_s_artifact_fem_3d_editor_visual_job::editor::fem3d::session::visual_job::Fem3dPageCredit;
use semio_s_artifact_fem_3d_editor_visual_job::editor::fem3d::session::visual_job::Fem3dPageVisualLease;
use semio_s_artifact_fem_3d_editor_visual_job::editor::fem3d::session::visual_job::Fem3dPageVisualJob;

#[path = "🧷️identity-and-recovery-slot/🦀️.rs"]
mod identity_and_recovery_slot;
use identity_and_recovery_slot::*;

#[path = "💼️mounted-state/🦀️.rs"]
mod mounted_state;
use mounted_state::*;

#[path = "📸️snapshot-preflight/🦀️.rs"]
mod snapshot_preflight;
use snapshot_preflight::*;

#[path = "🗃️registry/🦀️.rs"]
mod registry;
use registry::*;

#[path = "⚙️mounted-job/🦀️.rs"]
mod mounted_job;
use mounted_job::*;

#[path = "🚀️session-api/🦀️.rs"]
mod session_api;
pub use session_api::*;
use session_api::{identity_live, source_version, record_terminal, RequestKey, PhysicalRequestSpec};

#[path = "🧹️recovery-and-close/🦀️.rs"]
mod recovery_and_close;
pub use recovery_and_close::*;
use recovery_and_close::merge_close_demand;

//#region 🧪️Laws
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Laws
