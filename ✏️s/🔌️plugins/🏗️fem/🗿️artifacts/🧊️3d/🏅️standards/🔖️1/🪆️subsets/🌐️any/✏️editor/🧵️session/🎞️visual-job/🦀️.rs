//! publication — what one mounted session runs per document revision.
//! 🎞️ The FEM3D page visual job: its admission contract, staged mesh/solve pipeline and paged world-snapshot

use semio_s_fem_analyses_assembly::AssemblyCsrBuild;

use semio_s_fem_analyses_assembly::AssemblyJob;

use semio_s_fem_analyses_assembly::AssemblyJobConstruction;
use semio_s_fem_analyses_model::MountedAnalysisModel;
use semio_s_fem_analyses_model::MountedAnalysisSupport;
use semio_s_fem_analyses_model::MOUNTED_ANALYSIS_BACKING_BYTES;
use semio_s_fem_elements3d::Tet4;
use semio_s_fem_mesh::MeshJob;
use semio_s_fem_mesh::MeshOpts;
use semio_s_fem_mesh::MountedPlanarDomain;
use semio_s_fem_elements3d::Bar3;
use semio_s_fem_elements3d::Frame3;
use semio_s_fem_model::Dof;
use semio_s_fem_model::Node;
use semio_s_fem_static::Elements;
use semio_s_fem_sparse::Csr;
use semio_s_fem_sparse::MountedScalarSlots;
use semio_s_fem_sparse_ldlt::LdltJob;
use semio_s_fem_sparse_pcg::PcgJob;
use semio_s_fem_sparse_pcg::PcgJobConstruction;
use semio_s_fem_sparse::SymmetricPencilConstruction;
use semio_s_fem_sparse_subspace::SubspaceIterationJob;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::structure::element_id;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::structure::load_id;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::structure::FemElement;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::structure::FemLoad;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::snapshot::Fem3dSnapshot;
use semio_framework_job::InteractiveJob;
use semio_framework_job::JobOutcomeBorrow;
use semio_framework_job::StepContext;
use semio_framework_plugin::PluginLifecycleStep;
use semio_framework_value::RetainedCloneGrant;
use semio_framework_value::RetainedCloneProgress;
use semio_framework_value::ValueError;
use semio_framework_ui_scene::world3d_snapshot_abort_write;
use semio_framework_ui_scene::world3d_snapshot_abort_write_step;
use semio_framework_ui_scene::world3d_snapshot_admit_page;
use semio_framework_ui_scene::world3d_snapshot_begin;
use semio_framework_ui_scene::world3d_snapshot_begin_close;
use semio_framework_ui_scene::world3d_snapshot_close_step;
use semio_framework_ui_scene::world3d_snapshot_recover_lease;
use semio_framework_ui_scene::world3d_snapshot_recover_page;
use semio_framework_ui_scene::world3d_snapshot_recover_write;
use semio_framework_ui_scene::world3d_snapshot_seal;
use semio_framework_ui_scene::world3d_snapshot_terminal_is_empty;
use semio_framework_ui_scene::world3d_snapshot_write_terminal_is_empty;
use semio_framework_ui_scene::World3dSnapshotDescriptor;
use semio_framework_ui_scene::World3dSnapshotItem;
use semio_framework_ui_scene::World3dSnapshotLease;
use semio_framework_ui_scene::World3dSnapshotPage;
use semio_framework_ui_scene::World3dSnapshotPageKind;
use semio_framework_ui_scene::World3dSnapshotWriteToken;
use semio_framework_ui_scene::WORLD3D_SNAPSHOT_PAGE_BYTE_CAPACITY;
use semio_framework_ui_scene::WORLD3D_SNAPSHOT_PAGE_ITEM_CAPACITY;
use std::cell::RefCell;

#[path = "🧾️contract/🦀️.rs"]
mod contract;
pub use self::contract::*;
#[path = "🧮️solver-view/🦀️.rs"]
mod solver_view;
pub use self::solver_view::*;
#[path = "🧱️model-step/🦀️.rs"]
mod model_step;
pub use self::model_step::*;
#[path = "🪜️solve-step/🦀️.rs"]
mod solve_step;
#[path = "📄️page-job/🦀️.rs"]
mod page_job;
pub use self::page_job::*;
