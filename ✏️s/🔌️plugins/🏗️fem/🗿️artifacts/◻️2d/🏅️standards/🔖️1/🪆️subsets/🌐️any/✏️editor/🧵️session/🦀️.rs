//! 🧵️ Mounted Fem2d revision job: fixed session arena, retained worker step and live visual lease.

use semio_s_fem_analyses_model::MountedAnalysisModel;
use semio_s_fem_analyses_model::MountedAnalysisSupport;
use semio_s_fem_analyses_assembly::AssemblyCsrBuild;
use semio_s_fem_analyses_assembly::AssemblyJob;
use semio_s_fem_analyses_assembly::AssemblyJobConstruction;
use semio_s_fem_analyses::FemJobGraph;
use semio_s_fem_analyses::FemJobStage;
use semio_s_fem_analyses::FemStagePlan;
use semio_s_fem_analyses_model::ResultSource;
use semio_s_artifact_fem_2d_editor_model_window::editor::fem2d::modes::edit::windows::model::Fem2dLiveVisual;
use semio_s_artifact_fem_2d_editor_model_window::editor::fem2d::modes::edit::windows::model::Fem2dMountedVisualLease;
use semio_s_artifact_fem_2d_editor_model_window::editor::fem2d::modes::edit::windows::model::Fem2dVisualFreshness;
use semio_s_artifact_fem_2d_editor_model_window::editor::fem2d::modes::edit::windows::model::Fem2dVisualJob;
use semio_s_artifact_fem_2d_editor_model_window::editor::fem2d::modes::edit::windows::model::FemVisualState;
use semio_s_artifact_fem_2d_editor_model_window::editor::fem2d::modes::edit::windows::model::RegionVisualQuality;
use semio_s_fem_mesh::MeshJob;
use semio_s_fem_mesh::MeshOpts;
use semio_s_fem_mesh::PlanarDomain;
use semio_s_fem_mesh::TriMesh2;
use semio_s_fem_model::Element;
use semio_s_fem_sparse::Csr;
use semio_s_fem_sparse::MountedScalarSlots;
use semio_s_fem_sparse_pcg::PcgJob;
use semio_s_fem_sparse_pcg::PcgJobConstruction;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::snapshot::Fem2dSnapshot;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::FemElement;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::FemLoad;
use semio_framework::kernel::Effect;
use semio_framework::kernel::JobPlacement;
#[cfg(test)]
use semio_framework_job::CommitValidation;
use semio_framework_job::InteractiveJob;
use semio_framework_job::InteractiveJobCloseStep;
use semio_framework_job::JobOutcomeBorrow;
use semio_framework_job::Operation;
use semio_framework_job::OperationId;
use semio_framework_job_payload::RetainedJobPayload;
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
use std::cell::RefCell;
use std::rc::Rc;

pub use semio_s_artifact_fem_2d_editor_session_catalog::{ConsumerKey, FEM2D_MOUNTED_JOB_KIND};
use semio_s_artifact_fem_2d_editor_session_catalog::{
    SESSION_MAXIMUM_BYTES, SESSION_MAXIMUM_ITEMS,
    MountedProcessOwnerCatalog, RequestKey, SESSION_CONSUMER_CAPACITY, FEM2D_JOB_COUNTER_MAXIMUM, FEM2D_JOB_TAG, INPUT_BYTES,
    SESSION_ACTIVE_CAPACITY, SESSION_MAXIMUM_BOUNDARY_POINTS, SESSION_MAXIMUM_ELEMENTS,
    SESSION_MAXIMUM_FAULT_BYTES, SESSION_MAXIMUM_INPUT_BYTES, SESSION_MAXIMUM_INPUT_ITEMS,
    SESSION_MAXIMUM_MESH_POINTS, SESSION_MAXIMUM_MESH_TRIANGLES, SESSION_MAXIMUM_NODES,
    SESSION_MAXIMUM_OUTPUT_BYTES, SESSION_MAXIMUM_REGION_HOLES, SESSION_MAXIMUM_STRING_BYTES,
    SESSION_MAXIMUM_SUPPORTS, SESSION_MAXIMUM_VISUAL_LOADS, SESSION_OWNER_PAGE_BYTES,
    SESSION_PREFLIGHT_UNITS_PER_OPPORTUNITY, SESSION_SHELL_CAPACITY, SESSION_STEP_CEILING_MS,
    SESSION_UNITS_PER_STEP,
};
#[cfg(test)]
use semio_s_artifact_fem_2d_editor_session_catalog::MountedOwnerClass;

#[path = "🏗️model-build-job/🦀️.rs"]
mod model_build_job;
use model_build_job::{
    bounded_string_capacities, ModelBuildStage, MountedIdentity, MountedModelBuild, MountedStage,
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
    CurrentSession, RetiringShell, MountedRegistry, PendingAdmission, PendingSnapshotAdmission,
    PendingSnapshotFault, SnapshotAdmissionCursor,
};

#[path = "💼️job-bridge-and-live-mount/🦀️.rs"]
mod job_bridge_and_live_mount;
#[cfg(test)]
use job_bridge_and_live_mount::{decode_input, encode_input};
pub use job_bridge_and_live_mount::{
    close_demands, close_step, initialize, maintenance_step, prepare_snapshot_read, reconcile,
    terminal_is_empty, with_live_visual, with_live_fields, MountedFieldsView, retain_result_windows, source_version,
};
#[cfg(test)]
pub use job_bridge_and_live_mount::result_request_key;
use job_bridge_and_live_mount::{current_identity, merge_close_demand};

#[derive(Clone, Copy, PartialEq, Eq)]
enum LoadBuildStage { Select, Expand, AdmitFull, AdmitFree, ZeroFull, ZeroFree, Nodal, Element, Positions, ElementLoads, ElementRows, Complete }

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ResolvedResultSource { LoadCase(usize), Combination(usize) }

struct MountedLoadBuild {
    source: Option<ResolvedResultSource>,
    stage: LoadBuildStage,
    factors: [f64; 64],
    stack: [(usize, usize, f64); 64],
    depth: usize,
    case: usize,
    load: usize,
    element: usize,
    position: usize,
    lookup: usize,
    positions: [[f64; 3]; 3],
    row: usize,
    udl: semio_s_fem_model::MemberUdl,
    pressure: f64,
    full: MountedScalarSlots,
    free: MountedScalarSlots,
    reactions: MountedScalarSlots,
}

impl MountedLoadBuild {
    fn new(source: Option<ResolvedResultSource>) -> Self {
        Self { source, stage: LoadBuildStage::Select, factors: [0.0; 64], stack: [(0, 0, 0.0); 64], depth: 0, case: 0, load: 0, element: 0, position: 0, lookup: 0, positions: [[0.0; 3]; 3], row: 0, udl: semio_s_fem_model::MemberUdl { wx: 0.0, wy: 0.0, wz: 0.0 }, pressure: 0.0, full: MountedScalarSlots::new(), free: MountedScalarSlots::new(), reactions: MountedScalarSlots::new() }
    }

    fn add(&mut self, assembly: &AssemblyJob<'static>, node: &str, dof: semio_s_fem_model::Dof, value: f64) -> Result<(), Vec<u8>> {
        if !value.is_finite() { return Err(b"fem2d.load-non-finite".to_vec()); }
        if let Some((full, free)) = assembly.visual_equation_indices(node, dof) {
            self.full.add_at(full, value).map_err(|_| b"fem2d.load-full-index".to_vec())?;
            if let Some(free) = free { self.free.add_at(free, value).map_err(|_| b"fem2d.load-free-index".to_vec())?; }
        } else if value != 0.0 { return Err(b"fem2d.load-inactive-node-dof".to_vec()); }
        Ok(())
    }

    fn step_one(&mut self, snapshot: &Fem2dSnapshot, assembly: &AssemblyJob<'static>) -> Result<bool, Vec<u8>> {
        if snapshot.load_cases.len() > self.factors.len() || snapshot.combinations.len() > self.stack.len() { return Err(b"fem2d.source-capacity".to_vec()); }
        match self.stage {
            LoadBuildStage::Select => {
                match self.source {
                    Some(ResolvedResultSource::LoadCase(index)) => {
                        if snapshot.load_cases.get(index).is_none() { return Err(b"fem2d.source-case-missing".to_vec()); }
                        self.factors[index] = 1.0;
                        self.stage = LoadBuildStage::AdmitFull;
                    }
                    Some(ResolvedResultSource::Combination(index)) => {
                        if snapshot.combinations.get(index).is_none() { return Err(b"fem2d.source-combination-missing".to_vec()); }
                        self.stack[0] = (index, 0, 1.0);
                        self.depth = 1;
                        self.stage = LoadBuildStage::Expand;
                    }
                    None => self.stage = LoadBuildStage::AdmitFull,
                }
            }
            LoadBuildStage::Expand => {
                if self.depth == 0 { self.stage = LoadBuildStage::AdmitFull; return Ok(false); }
                let (index, term, factor) = self.stack[self.depth - 1];
                if let Some(term) = snapshot.combinations[index].terms.get(term) {
                    self.stack[self.depth - 1].1 += 1;
                    let factor = factor * term.factor;
                    if !factor.is_finite() { return Err(b"fem2d.source-factor".to_vec()); }
                    if let Some(case) = snapshot.load_cases.iter().position(|case| case.id == term.case_id) {
                        self.factors[case] += factor;
                        if !self.factors[case].is_finite() { return Err(b"fem2d.source-factor".to_vec()); }
                    } else {
                        let nested = snapshot.combinations.iter().position(|combination| combination.id == term.case_id).ok_or_else(|| b"fem2d.source-term-missing".to_vec())?;
                        if self.depth == self.stack.len() || self.stack[..self.depth].iter().any(|frame| frame.0 == nested) { return Err(b"fem2d.source-cycle".to_vec()); }
                        self.stack[self.depth] = (nested, 0, factor);
                        self.depth += 1;
                    }
                } else { self.depth -= 1; }
            }
            LoadBuildStage::AdmitFull => if self.full.admit_one(assembly.visual_full_order()).map_err(|_| b"fem2d.load-full-capacity".to_vec())? { self.stage = LoadBuildStage::AdmitFree; },
            LoadBuildStage::AdmitFree => if self.free.admit_one(assembly.visual_free_order()).map_err(|_| b"fem2d.load-free-capacity".to_vec())? { self.stage = LoadBuildStage::ZeroFull; },
            LoadBuildStage::ZeroFull => if self.full.len() < assembly.visual_full_order() { self.full.push(0.0).map_err(|_| b"fem2d.load-full-owner".to_vec())?; } else { self.stage = LoadBuildStage::ZeroFree; },
            LoadBuildStage::ZeroFree => if self.free.len() < assembly.visual_free_order() { self.free.push(0.0).map_err(|_| b"fem2d.load-free-owner".to_vec())?; } else { self.stage = LoadBuildStage::Nodal; },
            LoadBuildStage::Nodal => {
                let Some(case) = snapshot.load_cases.get(self.case) else { self.stage = LoadBuildStage::Complete; return Ok(false); };
                if self.factors[self.case] == 0.0 { self.case += 1; self.load = 0; return Ok(false); }
                if let Some(load) = case.loads.get(self.load) {
                    match load {
                        FemLoad::Nodal { node_id, dof, value, .. } => self.add(assembly, node_id, (*dof).into(), value * self.factors[self.case])?,
                        FemLoad::MemberUdl { element_id, .. } if !snapshot.elements.iter().any(|element| semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::element_id(element) == element_id) => return Err(b"fem2d.load-member-missing".to_vec()),
                        FemLoad::Area { region_id, .. } if !snapshot.regions.iter().any(|region| region.id == *region_id) => return Err(b"fem2d.load-region-missing".to_vec()),
                        _ => {}
                    }
                    self.load += 1;
                } else { self.element = 0; self.stage = LoadBuildStage::Element; }
            }
            LoadBuildStage::Element => {
                if assembly.model_element(self.element).is_none() { self.case += 1; self.load = 0; self.stage = LoadBuildStage::Nodal; }
                else { self.position = 0; self.lookup = 0; self.udl = semio_s_fem_model::MemberUdl { wx: 0.0, wy: 0.0, wz: 0.0 }; self.pressure = 0.0; self.stage = LoadBuildStage::Positions; }
            }
            LoadBuildStage::Positions => {
                let element = assembly.model_element(self.element).ok_or_else(|| b"fem2d.load-element-owner".to_vec())?;
                if let Some(id) = element.mounted_node_id(self.position) {
                    let node = assembly.model_node(self.lookup).ok_or_else(|| b"fem2d.load-node-missing".to_vec())?;
                    if node.id == id { self.positions[self.position] = node.pos; self.position += 1; self.lookup = 0; } else { self.lookup += 1; }
                } else { self.load = 0; self.stage = LoadBuildStage::ElementLoads; }
            }
            LoadBuildStage::ElementLoads => {
                let element = assembly.model_element(self.element).ok_or_else(|| b"fem2d.load-element-owner".to_vec())?;
                if let Some(load) = snapshot.load_cases[self.case].loads.get(self.load) {
                    match load {
                        FemLoad::MemberUdl { element_id, wx, wy, .. } if element.id() == element_id => { self.udl.wx += wx; self.udl.wy += wy; }
                        FemLoad::Area { region_id, pressure, .. } if matches!(element, semio_s_fem_static::Elements::Tri3Cst(_)) => {
                            if element.id().strip_prefix(region_id).and_then(|suffix| suffix.strip_prefix("_t")).is_some_and(|suffix| suffix.parse::<usize>().is_ok()) { self.pressure += pressure; }
                        }
                        _ => {}
                    }
                    self.load += 1;
                } else { self.row = 0; self.stage = LoadBuildStage::ElementRows; }
            }
            LoadBuildStage::ElementRows => {
                let element = assembly.model_element(self.element).ok_or_else(|| b"fem2d.load-element-owner".to_vec())?;
                let dofs = element.dofs_per_node();
                let nodes = element.mounted_node_id_count().ok_or_else(|| b"fem2d.load-element-shape".to_vec())?;
                if self.row == nodes * dofs.len() { self.element += 1; self.stage = LoadBuildStage::Element; return Ok(false); }
                let gravity = [0.0, if snapshot.load_cases[self.case].self_weight { -9.81 } else { 0.0 }];
                let value = match element {
                    semio_s_fem_static::Elements::Bar2(element) => element.mounted_load_row(&self.positions[..nodes], self.row, &self.udl, gravity),
                    semio_s_fem_static::Elements::BeamEb2(element) => element.mounted_load_row(&self.positions[..nodes], self.row, &self.udl, gravity),
                    semio_s_fem_static::Elements::Tri3Cst(element) => element.mounted_load_row(&self.positions[..nodes], self.row, self.pressure, gravity),
                    _ => None,
                }.ok_or_else(|| b"fem2d.load-row-shape".to_vec())?;
                self.add(assembly, element.mounted_node_id(self.row / dofs.len()).ok_or_else(|| b"fem2d.load-node-owner".to_vec())?, dofs[self.row % dofs.len()], value * self.factors[self.case])?;
                self.row += 1;
            }
            LoadBuildStage::Complete => return Ok(true),
        }
        Ok(false)
    }

    fn close_step(&mut self) -> bool {
        if !self.full.close_step() || !self.free.close_step() || !self.reactions.close_step() { return false; }
        self.source = None;
        true
    }
}

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
