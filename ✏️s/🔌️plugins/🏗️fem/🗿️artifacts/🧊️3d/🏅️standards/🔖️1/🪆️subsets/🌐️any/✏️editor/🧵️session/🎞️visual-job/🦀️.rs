//! 🎞️ The FEM3D page visual job: its admission contract, staged mesh/solve pipeline and paged world-snapshot
//! publication — what one mounted session runs per document revision.

use semio_s_fem_analyses_assembly::{AssemblyCsrBuild, AssemblyJob, AssemblyJobConstruction};
use semio_s_fem_analyses_model::{MountedAnalysisModel, MountedAnalysisSupport, MOUNTED_ANALYSIS_BACKING_BYTES};
use semio_s_fem_elements3d::Tet4;
use semio_s_fem_mesh::{MeshJob, MeshOpts, MountedPlanarDomain};
use semio_s_fem_elements3d::{Bar3, Frame3};
use semio_s_fem_model::{Dof, Node};
use semio_s_fem_static::Elements;
use semio_s_fem_sparse::{Csr, MountedScalarSlots};
use semio_s_fem_sparse_ldlt::LdltJob;
use semio_s_fem_sparse_pcg::{PcgJob, PcgJobConstruction};
use semio_s_fem_sparse_subspace::{ModalInputConstruction, SubspaceIterationJob};
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::structure::element_id;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::structure::load_id;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::structure::FemElement;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::structure::FemLoad;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::snapshot::Fem3dSnapshot;
use semio_framework_job::{InteractiveJob, JobOutcomeBorrow, StepContext};
use semio_framework_plugin::PluginLifecycleStep;
use semio_framework_value::{RetainedCloneGrant, RetainedCloneProgress, ValueError};
use semio_framework_ui_scene::{world3d_snapshot_abort_write, world3d_snapshot_abort_write_step, world3d_snapshot_admit_page, world3d_snapshot_begin, world3d_snapshot_begin_close, world3d_snapshot_close_step, world3d_snapshot_recover_lease, world3d_snapshot_recover_page, world3d_snapshot_recover_write, world3d_snapshot_seal, world3d_snapshot_terminal_is_empty, world3d_snapshot_write_terminal_is_empty, World3dSnapshotDescriptor, World3dSnapshotItem, World3dSnapshotLease, World3dSnapshotPage, World3dSnapshotPageKind, World3dSnapshotWriteToken, WORLD3D_SNAPSHOT_PAGE_BYTE_CAPACITY, WORLD3D_SNAPSHOT_PAGE_ITEM_CAPACITY};
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
