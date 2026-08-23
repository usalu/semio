//! 🧵️ Mounted Fem2d revision job: fixed session arena, retained worker step and live visual lease.

use crate::analyses::{AnalysisModel, AssemblyCsrBuild, AssemblyJob, FemJobGraph, FemJobStage, FemStagePlan};
use crate::artifacts::fem2d::{Fem2dSnapshot, FemElement, FemLoad};
use crate::editor::fem2d::modes::edit::windows::model::{Fem2dLiveVisual, RegionVisualQuality};
use crate::mesh::{MeshJob, MeshOpts, PlanarDomain, TriMesh2};
use crate::sparse::{PcgJob, PcgJobConstruction};
use semio_framework::kernel::{Effect, JobPlacement};
use semio_framework_job::{CommitValidation, InteractiveJob, Operation, OperationId, StepBudget, StepContext, StepOutcome};
use semio_framework_plugin::reactor::jobs::{BoundedJob, BoundedJobFactory, JobBudget, JobStep};
use semio_framework_plugin::{AppRenderOperationContext, ArtifactView, PluginCloseStep};
use std::cell::RefCell;
use std::rc::Rc;

//#region 🔖️Contract
pub const FEM2D_MOUNTED_JOB_KIND: &str = "semio.fem2d.mounted-analysis";
const SESSION_ACTIVE_CAPACITY: usize = 32;
const SESSION_SHELL_CAPACITY: usize = 64;
const SESSION_MAXIMUM_ITEMS: usize = 4_096;
const SESSION_MAXIMUM_BYTES: usize = 4 * 1_024 * 1_024;
const SESSION_MAXIMUM_NODES: usize = 64;
const SESSION_MAXIMUM_ELEMENTS: usize = 128;
const SESSION_MAXIMUM_SUPPORTS: usize = 64;
const SESSION_MAXIMUM_MESH_POINTS: usize = 64;
const SESSION_MAXIMUM_MESH_TRIANGLES: usize = 128;
const SESSION_MAXIMUM_REGION_HOLES: usize = 16;
const SESSION_MAXIMUM_BOUNDARY_POINTS: usize = 64;
const SESSION_MAXIMUM_OUTPUT_BYTES: usize = 16 * 1_024;
const INPUT_BYTES: usize = 63;
const FEM2D_JOB_TAG: u64 = 0xf2d0_0000_0000_0000;
const FEM2D_JOB_COUNTER_MAXIMUM: u64 = 0x000f_ffff_ffff_ffff;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct MountedIdentity {
    app_instance_id: u32,
    base_revision: semio_framework_job::RevisionId,
    generation: semio_framework_job::Generation,
    canonical_base_revision: [u8; 32],
    operation: OperationId,
    job: u64,
}

impl MountedIdentity {
    fn operation(self) -> Operation {
        Operation::new(self.operation, self.base_revision, self.generation, self.operation.0.rotate_left(17) ^ self.base_revision.0)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum MountedStage {
    Preflight,
    PrepareGraph,
    Graph,
    PrepareDomain,
    Mesh,
    BuildModel,
    Assembly,
    BuildCsr,
    PreparePcg,
    Pcg,
    CommitReady,
    Complete,
    Fault,
    Closing,
    Empty,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ModelBuildStage {
    ReserveNodes,
    ReserveElements,
    ReserveSupports,
    ReserveRegionNodeIds,
    Nodes,
    Elements,
    Supports,
    RegionNodes,
    RegionElements,
    Complete,
}

struct MountedModelBuild {
    stage: ModelBuildStage,
    node_cursor: usize,
    element_cursor: usize,
    support_cursor: usize,
    region_point_cursor: usize,
    region_triangle_cursor: usize,
    mesh: Option<TriMesh2>,
    region_node_ids: Vec<String>,
    model: AnalysisModel,
}

impl MountedModelBuild {
    fn new(mesh: Option<TriMesh2>) -> Self {
        Self {
            stage: ModelBuildStage::ReserveNodes,
            node_cursor: 0,
            element_cursor: 0,
            support_cursor: 0,
            region_point_cursor: 0,
            region_triangle_cursor: 0,
            mesh,
            region_node_ids: Vec::new(),
            model: AnalysisModel { nodes: Vec::new(), elements: Vec::new(), supports: Vec::new() },
        }
    }

    fn step_one(&mut self, snapshot: &Fem2dSnapshot) -> Result<bool, Vec<u8>> {
        let mesh_points = self.mesh.as_ref().map_or(0, |mesh| mesh.points.len());
        let mesh_triangles = self.mesh.as_ref().map_or(0, |mesh| mesh.tris.len());
        if snapshot.nodes.len() > SESSION_MAXIMUM_NODES
            || snapshot.elements.len() > SESSION_MAXIMUM_ELEMENTS
            || snapshot.supports.len() > SESSION_MAXIMUM_SUPPORTS
            || mesh_points > SESSION_MAXIMUM_MESH_POINTS
            || mesh_triangles > SESSION_MAXIMUM_MESH_TRIANGLES
            || snapshot.regions.len() > 1
        {
            return Err(b"fem2d.model-fixed-capacity".to_vec());
        }
        match self.stage {
            ModelBuildStage::ReserveNodes => {
                self.model.nodes.try_reserve_exact(snapshot.nodes.len() + mesh_points).map_err(|_| b"fem2d.model-node-allocation".to_vec())?;
                self.stage = ModelBuildStage::ReserveElements;
            }
            ModelBuildStage::ReserveElements => {
                self.model.elements.try_reserve_exact(snapshot.elements.len() + mesh_triangles).map_err(|_| b"fem2d.model-element-allocation".to_vec())?;
                self.stage = ModelBuildStage::ReserveSupports;
            }
            ModelBuildStage::ReserveSupports => {
                self.model.supports.try_reserve_exact(snapshot.supports.len()).map_err(|_| b"fem2d.model-support-allocation".to_vec())?;
                self.stage = ModelBuildStage::ReserveRegionNodeIds;
            }
            ModelBuildStage::ReserveRegionNodeIds => {
                self.region_node_ids.try_reserve_exact(mesh_points).map_err(|_| b"fem2d.model-region-node-allocation".to_vec())?;
                self.stage = ModelBuildStage::Nodes;
            }
            ModelBuildStage::Nodes => {
                if let Some(node) = snapshot.nodes.get(self.node_cursor) {
                    self.model.nodes.push(crate::model::Node { id: node.id.clone(), pos: [node.x, node.y, 0.0] });
                    self.node_cursor += 1;
                } else {
                    self.stage = ModelBuildStage::Elements;
                }
            }
            ModelBuildStage::Elements => {
                if let Some(element) = snapshot.elements.get(self.element_cursor) {
                    let (id, start, end, material_id, section_id) = match element {
                        FemElement::Bar { id, start, end, material_id, section_id, .. } | FemElement::Beam { id, start, end, material_id, section_id, .. } => (id, start, end, material_id, section_id),
                    };
                    if !snapshot.nodes.iter().any(|node| node.id == *start) || !snapshot.nodes.iter().any(|node| node.id == *end) {
                        return Err(b"fem2d.model-unknown-node".to_vec());
                    }
                    let material = snapshot.materials.iter().find(|material| material.id == *material_id).ok_or_else(|| b"fem2d.model-unknown-material".to_vec())?;
                    let section = snapshot.sections.iter().find(|section| section.id == *section_id).ok_or_else(|| b"fem2d.model-unknown-section".to_vec())?;
                    let resolved = match element {
                        FemElement::Bar { .. } => crate::elements2d::Bar2 { id: id.clone(), start: start.clone(), end: end.clone(), e: material.e, area: section.area, density: material.rho }.into(),
                        FemElement::Beam { .. } => crate::elements2d::BeamEb2 { id: id.clone(), start: start.clone(), end: end.clone(), e: material.e, area: section.area, iy: section.iy, density: material.rho }.into(),
                    };
                    self.model.elements.push(resolved);
                    self.element_cursor += 1;
                } else {
                    self.stage = ModelBuildStage::Supports;
                }
            }
            ModelBuildStage::Supports => {
                if let Some(support) = snapshot.supports.get(self.support_cursor) {
                    self.model.supports.push(crate::model::Support { node_id: support.node_id.clone(), fixed: support.fixed.iter().map(|dof| (*dof).into()).collect() });
                    self.support_cursor += 1;
                } else {
                    self.stage = ModelBuildStage::RegionNodes;
                }
            }
            ModelBuildStage::RegionNodes => {
                let Some(region) = snapshot.regions.first() else {
                    self.stage = ModelBuildStage::Complete;
                    return Ok(false);
                };
                let Some(point) = self.mesh.as_ref().and_then(|mesh| mesh.points.get(self.region_point_cursor)).copied() else {
                    self.stage = ModelBuildStage::RegionElements;
                    return Ok(false);
                };
                let id = snapshot.nodes.iter().find(|node| (node.x - point[0]).abs() < 1e-9 && (node.y - point[1]).abs() < 1e-9).map(|node| node.id.clone()).unwrap_or_else(|| format!("{}_m{}", region.id, self.region_point_cursor));
                if !self.model.nodes.iter().any(|node| node.id == id) {
                    self.model.nodes.push(crate::model::Node { id: id.clone(), pos: [point[0], point[1], 0.0] });
                }
                self.region_node_ids.push(id);
                self.region_point_cursor += 1;
            }
            ModelBuildStage::RegionElements => {
                let Some(region) = snapshot.regions.first() else {
                    self.stage = ModelBuildStage::Complete;
                    return Ok(false);
                };
                let material = snapshot.materials.iter().find(|material| material.id == region.material_id).ok_or_else(|| b"fem2d.model-unknown-region-material".to_vec())?;
                if let Some(triangle) = self.mesh.as_ref().and_then(|mesh| mesh.tris.get(self.region_triangle_cursor)).copied() {
                    let nodes = [self.region_node_ids[triangle[0] as usize].clone(), self.region_node_ids[triangle[1] as usize].clone(), self.region_node_ids[triangle[2] as usize].clone()];
                    self.model.elements.push(
                        crate::elements2d::Tri3Cst {
                            id: format!("{}_t{}", region.id, self.region_triangle_cursor),
                            nodes,
                            e: material.e,
                            nu: material.nu,
                            thickness: region.thickness,
                            kind: crate::elements2d::PlaneKind::Stress,
                            density: material.rho,
                        }
                        .into(),
                    );
                    self.region_triangle_cursor += 1;
                } else {
                    self.stage = ModelBuildStage::Complete;
                }
            }
            ModelBuildStage::Complete => return Ok(true),
        }
        Ok(false)
    }

    fn take_complete(&mut self) -> Option<AnalysisModel> {
        (self.stage == ModelBuildStage::Complete).then(|| std::mem::replace(&mut self.model, AnalysisModel { nodes: Vec::new(), elements: Vec::new(), supports: Vec::new() }))
    }

    fn close_step(&mut self) -> (bool, usize) {
        if self.model.nodes.pop().is_some() {
            return (false, std::mem::size_of::<crate::model::Node>());
        }
        if self.model.elements.pop().is_some() {
            return (false, std::mem::size_of::<crate::model::Elements>());
        }
        if self.model.supports.pop().is_some() {
            return (false, std::mem::size_of::<crate::model::Support>());
        }
        if self.region_node_ids.pop().is_some() {
            return (false, std::mem::size_of::<String>());
        }
        if let Some(mesh) = self.mesh.as_mut() {
            if mesh.points.pop().is_some() {
                return (false, std::mem::size_of::<[f64; 2]>());
            }
            if mesh.tris.pop().is_some() {
                return (false, std::mem::size_of::<[u32; 3]>());
            }
            self.mesh = None;
            return (false, std::mem::size_of::<TriMesh2>());
        }
        (true, 0)
    }
}
//#endregion 🔖️Contract

//#region 🧰️FixedArena
struct MountedState {
    identity: MountedIdentity,
    snapshot: Option<store::SnapshotRead<Fem2dSnapshot>>,
    cancel: semio_framework_job::CancelToken,
    stage: MountedStage,
    preflight_lane: u8,
    preflight_cursor: usize,
    preflight_inner_cursor: usize,
    preflight_deep_cursor: usize,
    preflight_owner_opened: bool,
    admitted_items: usize,
    admitted_bytes: usize,
    graph: Option<FemJobGraph>,
    domain: Option<PlanarDomain>,
    domain_outer_cursor: usize,
    domain_hole_cursor: usize,
    domain_hole_point_cursor: usize,
    mesh: Option<MeshJob>,
    model_build: Option<MountedModelBuild>,
    model: Option<std::sync::Arc<AnalysisModel>>,
    assembly: Option<AssemblyJob<'static>>,
    csr_build: Option<AssemblyCsrBuild>,
    pcg_build: Option<PcgJobConstruction>,
    pcg: Option<PcgJob>,
    visual: Fem2dLiveVisual,
    preview_sequence: u64,
    close_cursor: u8,
    fault: Option<Vec<u8>>,
}

impl MountedState {
    fn new(identity: MountedIdentity, snapshot: store::SnapshotRead<Fem2dSnapshot>) -> Self {
        Self {
            identity,
            snapshot: Some(snapshot),
            cancel: semio_framework_job::root_cancel_token(),
            stage: MountedStage::Preflight,
            preflight_lane: 0,
            preflight_cursor: 0,
            preflight_inner_cursor: 0,
            preflight_deep_cursor: 0,
            preflight_owner_opened: false,
            admitted_items: 0,
            admitted_bytes: 0,
            graph: None,
            domain: None,
            domain_outer_cursor: 0,
            domain_hole_cursor: 0,
            domain_hole_point_cursor: 0,
            mesh: None,
            model_build: None,
            model: None,
            assembly: None,
            csr_build: None,
            pcg_build: None,
            pcg: None,
            visual: Fem2dLiveVisual::default(),
            preview_sequence: 0,
            close_cursor: 0,
            fault: None,
        }
    }

    fn fail(&mut self, detail: impl Into<Vec<u8>>) -> JobStep {
        let detail = detail.into();
        self.fault = Some(detail.clone());
        self.stage = MountedStage::Fault;
        JobStep::Failed(detail)
    }

    fn progress(&self, label: &'static [u8]) -> JobStep {
        let mut bytes = Vec::with_capacity(label.len() + 32);
        bytes.extend_from_slice(label);
        bytes.extend_from_slice(&self.identity.operation.0.to_le_bytes());
        bytes.extend_from_slice(&self.identity.base_revision.0.to_le_bytes());
        bytes.extend_from_slice(&self.identity.generation.0.to_le_bytes());
        JobStep::Running(Some(bytes))
    }

    fn charge(&mut self, items: usize, bytes: usize) -> Result<(), &'static [u8]> {
        let next_items = self.admitted_items.checked_add(items).ok_or(b"fem2d.session-item-overflow" as &'static [u8])?;
        let next_bytes = self.admitted_bytes.checked_add(bytes).ok_or(b"fem2d.session-byte-overflow" as &'static [u8])?;
        if next_items > SESSION_MAXIMUM_ITEMS || next_bytes > SESSION_MAXIMUM_BYTES {
            return Err(b"fem2d.session-admission-exceeded");
        }
        self.admitted_items = next_items;
        self.admitted_bytes = next_bytes;
        Ok(())
    }

    fn preflight_one(&mut self) -> Result<bool, &'static [u8]> {
        let snapshot = self.snapshot.as_ref().ok_or(b"fem2d.session-snapshot-missing" as &'static [u8])?;
        if self.preflight_lane != 4 && !self.preflight_owner_opened {
            self.preflight_owner_opened = true;
            let bytes = match self.preflight_lane {
                0 => snapshot.nodes.capacity() * std::mem::size_of::<crate::artifacts::fem2d::FemNode>(),
                1 => snapshot.elements.capacity() * std::mem::size_of::<FemElement>(),
                2 | 3 => snapshot.regions.capacity() * std::mem::size_of::<crate::artifacts::fem2d::FemRegion>(),
                5 => snapshot.materials.capacity() * std::mem::size_of::<crate::artifacts::fem2d::FemMaterial>(),
                6 => snapshot.sections.capacity() * std::mem::size_of::<crate::artifacts::fem2d::FemSection>(),
                7 => snapshot.supports.capacity() * std::mem::size_of::<crate::artifacts::fem2d::FemSupport>(),
                8 | 9 => snapshot.load_cases.capacity() * std::mem::size_of::<crate::artifacts::fem2d::FemLoadCase>(),
                10 | 11 => snapshot.combinations.capacity() * std::mem::size_of::<crate::artifacts::fem2d::FemCombination>(),
                _ => 0,
            };
            self.charge(1, bytes)?;
            return Ok(false);
        }
        let item = match self.preflight_lane {
            0 => {
                if snapshot.nodes.len() > SESSION_MAXIMUM_NODES {
                    return Err(b"fem2d.session-node-capacity");
                }
                snapshot.nodes.get(self.preflight_cursor).map(|node| node.id.capacity() + node.name.capacity() + 24)
            }
            1 => snapshot.elements.get(self.preflight_cursor).map(|element| match element {
                FemElement::Bar { id, name, start, end, material_id, section_id } | FemElement::Beam { id, name, start, end, material_id, section_id } => {
                    id.capacity() + name.capacity() + start.capacity() + end.capacity() + material_id.capacity() + section_id.capacity()
                }
            }),
            2 => {
                if snapshot.regions.len() > 1 {
                    return Err(b"fem2d.session-region-capacity");
                }
                snapshot.regions.get(self.preflight_cursor).map(|region| {
                    if region.outline.len() > SESSION_MAXIMUM_BOUNDARY_POINTS || region.holes.len() > SESSION_MAXIMUM_REGION_HOLES || region.holes.iter().any(|hole| hole.len() > SESSION_MAXIMUM_BOUNDARY_POINTS) {
                        return SESSION_MAXIMUM_BYTES + 1;
                    }
                    region.id.capacity() + region.name.capacity() + region.material_id.capacity() + 24
                })
            }
            3 => {
                if let Some(region) = snapshot.regions.get(self.preflight_cursor) {
                    if self.preflight_inner_cursor < region.outline.len() {
                        Some(16)
                    } else {
                        self.preflight_cursor += 1;
                        self.preflight_inner_cursor = 0;
                        return Ok(false);
                    }
                } else {
                    None
                }
            }
            4 => {
                if let Some(region) = snapshot.regions.get(self.preflight_cursor) {
                    if let Some(hole) = region.holes.get(self.preflight_inner_cursor) {
                        if !self.preflight_owner_opened {
                            self.preflight_owner_opened = true;
                            Some(0)
                        } else if self.preflight_deep_cursor < hole.len() {
                            self.preflight_deep_cursor += 1;
                            return self.charge(1, 16).map(|()| false);
                        } else {
                            self.preflight_inner_cursor += 1;
                            self.preflight_deep_cursor = 0;
                            self.preflight_owner_opened = false;
                            return Ok(false);
                        }
                    } else {
                        self.preflight_cursor += 1;
                        self.preflight_inner_cursor = 0;
                        self.preflight_deep_cursor = 0;
                        self.preflight_owner_opened = false;
                        return Ok(false);
                    }
                } else {
                    None
                }
            }
            5 => snapshot.materials.get(self.preflight_cursor).map(|material| material.id.capacity() + material.name.capacity() + 32),
            6 => snapshot.sections.get(self.preflight_cursor).map(|section| section.id.capacity() + section.name.capacity() + 16),
            7 => snapshot.supports.get(self.preflight_cursor).map(|support| support.id.capacity() + support.node_id.capacity() + support.fixed.capacity() * std::mem::size_of::<crate::artifacts::fem2d::FemDof>()),
            8 => snapshot.load_cases.get(self.preflight_cursor).map(|case| case.id.capacity() + case.name.capacity() + 1),
            9 => {
                if let Some(case) = snapshot.load_cases.get(self.preflight_cursor) {
                    if let Some(load) = case.loads.get(self.preflight_inner_cursor) {
                        self.preflight_inner_cursor += 1;
                        Some(match load {
                            FemLoad::Nodal { id, node_id, .. } => id.capacity() + node_id.capacity() + 16,
                            FemLoad::MemberUdl { id, element_id, .. } => id.capacity() + element_id.capacity() + 24,
                            FemLoad::Area { id, region_id, .. } => id.capacity() + region_id.capacity() + 16,
                        })
                    } else {
                        self.preflight_cursor += 1;
                        self.preflight_inner_cursor = 0;
                        return Ok(false);
                    }
                } else {
                    None
                }
            }
            10 => snapshot.combinations.get(self.preflight_cursor).map(|combination| combination.id.capacity() + combination.name.capacity()),
            11 => {
                if let Some(combination) = snapshot.combinations.get(self.preflight_cursor) {
                    if let Some(term) = combination.terms.get(self.preflight_inner_cursor) {
                        self.preflight_inner_cursor += 1;
                        Some(term.case_id.capacity() + 8)
                    } else {
                        self.preflight_cursor += 1;
                        self.preflight_inner_cursor = 0;
                        return Ok(false);
                    }
                } else {
                    None
                }
            }
            _ => return Ok(true),
        };
        if let Some(bytes) = item {
            self.charge(1, bytes)?;
            if matches!(self.preflight_lane, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 10) {
                self.preflight_cursor += 1;
            } else if self.preflight_lane == 3 {
                self.preflight_inner_cursor += 1;
            }
            return Ok(false);
        }
        self.preflight_lane += 1;
        self.preflight_cursor = 0;
        self.preflight_inner_cursor = 0;
        self.preflight_deep_cursor = 0;
        self.preflight_owner_opened = false;
        Ok(self.preflight_lane > 11)
    }

    fn prepare_domain_one(&mut self) -> Result<bool, &'static [u8]> {
        let snapshot = self.snapshot.as_ref().ok_or(b"fem2d.session-snapshot-missing" as &'static [u8])?;
        let Some(region) = snapshot.regions.first() else { return Ok(true) };
        if self.domain.is_none() {
            let mut outer = Vec::new();
            outer.try_reserve_exact(SESSION_MAXIMUM_BOUNDARY_POINTS).map_err(|_| b"fem2d.session-domain-outer-allocation" as &'static [u8])?;
            let mut holes = Vec::new();
            holes.try_reserve_exact(SESSION_MAXIMUM_REGION_HOLES).map_err(|_| b"fem2d.session-domain-holes-allocation" as &'static [u8])?;
            self.domain = Some(PlanarDomain { outer, holes });
            return Ok(false);
        }
        let domain = self.domain.as_mut().expect("domain initialized above");
        if let Some(point) = region.outline.get(self.domain_outer_cursor).copied() {
            domain.outer.push(point);
            self.domain_outer_cursor += 1;
            return Ok(false);
        }
        if self.domain_hole_cursor < region.holes.len() {
            if self.domain_hole_cursor == domain.holes.len() {
                let mut hole = Vec::new();
                hole.try_reserve_exact(SESSION_MAXIMUM_BOUNDARY_POINTS).map_err(|_| b"fem2d.session-domain-hole-allocation" as &'static [u8])?;
                domain.holes.push(hole);
                return Ok(false);
            }
            if let Some(point) = region.holes[self.domain_hole_cursor].get(self.domain_hole_point_cursor).copied() {
                domain.holes[self.domain_hole_cursor].push(point);
                self.domain_hole_point_cursor += 1;
                return Ok(false);
            }
            self.domain_hole_cursor += 1;
            self.domain_hole_point_cursor = 0;
            return Ok(false);
        }
        Ok(true)
    }

    fn step(&mut self, budget: JobBudget) -> JobStep {
        if self.cancel.is_cancelled_now() {
            self.stage = MountedStage::Closing;
            return self.fail(b"fem2d.session-cancelled".to_vec());
        }
        if budget.fuel == 0 || budget.deadline_ms == 0 {
            return JobStep::Running(None);
        }
        let now = semio_framework_job::default_now_ms();
        let deadline = now.saturating_add(u64::from(budget.deadline_ms).min(8));
        let mut cx = StepContext::new(self.identity.operation, self.identity.generation, StepBudget::new(budget.fuel.max(1), deadline), self.cancel.clone(), semio_framework_job::default_now_ms, &mut self.preview_sequence);
        match self.stage {
            MountedStage::Preflight => match self.preflight_one() {
                Ok(true) => {
                    self.stage = MountedStage::PrepareGraph;
                    self.progress(b"fem2d.preflight-complete")
                }
                Ok(false) => self.progress(b"fem2d.preflight"),
                Err(detail) => self.fail(detail.to_vec()),
            },
            MountedStage::PrepareGraph => {
                let plans = vec![
                    FemStagePlan { stage: FemJobStage::ValidateReferences, units: 1 },
                    FemStagePlan { stage: FemJobStage::BuildDofMap, units: 1 },
                    FemStagePlan { stage: FemJobStage::OrderEquations, units: 1 },
                    FemStagePlan { stage: FemJobStage::Assemble, units: 1 },
                    FemStagePlan { stage: FemJobStage::Factor, units: 1 },
                    FemStagePlan { stage: FemJobStage::Solve, units: 1 },
                    FemStagePlan { stage: FemJobStage::Recover, units: 1 },
                    FemStagePlan { stage: FemJobStage::Finalize, units: 1 },
                ];
                self.graph = Some(FemJobGraph::new(self.identity.operation(), plans, 1));
                self.stage = MountedStage::Graph;
                self.progress(b"fem2d.graph-admitted")
            }
            MountedStage::Graph => match self.graph.as_mut().expect("graph stage owns graph").step(&mut cx) {
                StepOutcome::Complete(candidate) => {
                    self.stage = MountedStage::PrepareDomain;
                    JobStep::Running(Some(candidate.output))
                }
                StepOutcome::PreviewReady(bytes) | StepOutcome::CheckpointReady(semio_framework_job::Checkpoint { state: bytes, .. }) if bytes.len() <= SESSION_MAXIMUM_OUTPUT_BYTES => JobStep::Running(Some(bytes)),
                StepOutcome::PreviewReady(_) | StepOutcome::CheckpointReady(_) => self.fail(b"fem2d.graph-output-capacity".to_vec()),
                StepOutcome::Yield => self.progress(b"fem2d.graph-yield"),
                StepOutcome::Cancelled => self.fail(b"fem2d.graph-cancelled".to_vec()),
                StepOutcome::Fault(fault) => self.fail(fault.detail),
            },
            MountedStage::PrepareDomain => match self.prepare_domain_one() {
                Ok(false) => self.progress(b"fem2d.domain"),
                Ok(true) => {
                    let snapshot = self.snapshot.as_ref().expect("preflight retains snapshot");
                    if let (Some(region), Some(domain)) = (snapshot.regions.first(), self.domain.take()) {
                        self.visual.region_quality.insert(region.id.clone(), RegionVisualQuality::Unmeshed);
                        self.mesh = Some(MeshJob::new_bounded(domain, MeshOpts { max_edge: region.mesh_size, min_angle_deg: 20.0 }, self.identity.operation(), SESSION_MAXIMUM_MESH_POINTS, SESSION_MAXIMUM_MESH_TRIANGLES));
                        self.stage = MountedStage::Mesh;
                    } else {
                        self.stage = MountedStage::BuildModel;
                    }
                    self.progress(b"fem2d.domain-complete")
                }
                Err(detail) => self.fail(detail.to_vec()),
            },
            MountedStage::Mesh => match self.mesh.as_mut().expect("mesh stage owns mesh").step(&mut cx) {
                StepOutcome::Complete(candidate) => {
                    if let Some(region) = self.snapshot.as_ref().and_then(|snapshot| snapshot.regions.first()) {
                        self.visual.region_quality.insert(region.id.clone(), RegionVisualQuality::Final);
                    }
                    self.model_build = Some(MountedModelBuild::new(self.mesh.as_mut().and_then(MeshJob::take_completed_mesh)));
                    self.stage = MountedStage::BuildModel;
                    if candidate.output.len() > SESSION_MAXIMUM_OUTPUT_BYTES {
                        self.progress(b"fem2d.mesh-complete")
                    } else {
                        JobStep::Running(Some(candidate.output))
                    }
                }
                StepOutcome::PreviewReady(bytes) => {
                    if let Some(region) = self.snapshot.as_ref().and_then(|snapshot| snapshot.regions.first()) {
                        let quality = match bytes.get(8) {
                            Some(0) => RegionVisualQuality::Coarse,
                            Some(1) => RegionVisualQuality::Refined,
                            Some(2) => RegionVisualQuality::Final,
                            _ => RegionVisualQuality::Unmeshed,
                        };
                        self.visual.region_quality.insert(region.id.clone(), quality);
                    }
                    if bytes.len() <= SESSION_MAXIMUM_OUTPUT_BYTES {
                        JobStep::Running(Some(bytes))
                    } else {
                        self.progress(b"fem2d.mesh-preview")
                    }
                }
                StepOutcome::CheckpointReady(checkpoint) if checkpoint.state.len() <= SESSION_MAXIMUM_OUTPUT_BYTES => JobStep::Running(Some(checkpoint.state)),
                StepOutcome::CheckpointReady(_) | StepOutcome::Yield => self.progress(b"fem2d.mesh-yield"),
                StepOutcome::Cancelled => self.fail(b"fem2d.mesh-cancelled".to_vec()),
                StepOutcome::Fault(fault) => self.fail(fault.detail),
            },
            MountedStage::BuildModel => {
                let snapshot = self.snapshot.as_ref().expect("preflight retains snapshot");
                if self.model_build.is_none() {
                    self.model_build = Some(MountedModelBuild::new(None));
                    return self.progress(b"fem2d.model-admitted");
                }
                match self.model_build.as_mut().expect("model builder admitted above").step_one(snapshot) {
                    Ok(false) => self.progress(b"fem2d.model-building"),
                    Ok(true) => {
                        let model = std::sync::Arc::new(self.model_build.as_mut().and_then(MountedModelBuild::take_complete).expect("complete model transfers exactly once"));
                        match AssemblyJob::new_owned(std::sync::Arc::clone(&model), self.identity.operation(), 1) {
                            Ok(assembly) => {
                                self.model = Some(model);
                                self.assembly = Some(assembly);
                                self.stage = MountedStage::Assembly;
                                self.progress(b"fem2d.model-built")
                            }
                            Err(error) => self.fail(error.to_string().into_bytes()),
                        }
                    }
                    Err(error) => self.fail(error),
                }
            }
            MountedStage::Assembly => match self.assembly.as_mut().expect("assembly stage owns assembly").step(&mut cx) {
                StepOutcome::Complete(candidate) => {
                    let assembly = self.assembly.take().expect("assembly owner retained");
                    self.csr_build = Some(AssemblyCsrBuild::new(assembly).expect("completed assembly begins retained CSR conversion"));
                    self.stage = MountedStage::BuildCsr;
                    if candidate.output.len() <= SESSION_MAXIMUM_OUTPUT_BYTES {
                        JobStep::Running(Some(candidate.output))
                    } else {
                        self.progress(b"fem2d.assembly-complete")
                    }
                }
                StepOutcome::PreviewReady(bytes) | StepOutcome::CheckpointReady(semio_framework_job::Checkpoint { state: bytes, .. }) if bytes.len() <= SESSION_MAXIMUM_OUTPUT_BYTES => JobStep::Running(Some(bytes)),
                StepOutcome::PreviewReady(_) | StepOutcome::CheckpointReady(_) | StepOutcome::Yield => self.progress(b"fem2d.assembly-yield"),
                StepOutcome::Cancelled => self.fail(b"fem2d.assembly-cancelled".to_vec()),
                StepOutcome::Fault(fault) => self.fail(fault.detail),
            },
            MountedStage::BuildCsr => match self.csr_build.as_mut().expect("CSR builder retained").step_one() {
                Ok(false) => self.progress(b"fem2d.csr-building"),
                Ok(true) => {
                    let matrix = self.csr_build.as_mut().and_then(AssemblyCsrBuild::take_complete).expect("complete CSR transfers exactly once");
                    self.pcg_build = Some(PcgJobConstruction::new(self.identity.operation(), matrix));
                    self.stage = MountedStage::PreparePcg;
                    self.progress(b"fem2d.csr-complete")
                }
                Err(detail) => self.fail(detail.to_vec()),
            },
            MountedStage::PreparePcg => match self.pcg_build.as_mut().expect("PCG builder retained").step_one() {
                Ok(false) => self.progress(b"fem2d.pcg-preparing"),
                Ok(true) => {
                    self.pcg = self.pcg_build.as_mut().and_then(PcgJobConstruction::take_complete);
                    if self.pcg.is_none() {
                        return self.fail(b"fem2d.pcg-false-terminal".to_vec());
                    }
                    self.stage = MountedStage::Pcg;
                    self.progress(b"fem2d.pcg-admitted")
                }
                Err(detail) => self.fail(detail.to_vec()),
            },
            MountedStage::Pcg => match self.pcg.as_mut().expect("pcg stage owns pcg").step(&mut cx) {
                StepOutcome::Complete(candidate) => {
                    self.visual.converged = self.pcg.as_ref().is_some_and(|job| job.solution().1.converged);
                    self.stage = MountedStage::CommitReady;
                    if candidate.output.len() <= SESSION_MAXIMUM_OUTPUT_BYTES {
                        JobStep::Running(Some(candidate.output))
                    } else {
                        self.progress(b"fem2d.pcg-complete")
                    }
                }
                StepOutcome::PreviewReady(bytes) | StepOutcome::CheckpointReady(semio_framework_job::Checkpoint { state: bytes, .. }) if bytes.len() <= SESSION_MAXIMUM_OUTPUT_BYTES => JobStep::Running(Some(bytes)),
                StepOutcome::PreviewReady(_) | StepOutcome::CheckpointReady(_) | StepOutcome::Yield => self.progress(b"fem2d.pcg-yield"),
                StepOutcome::Cancelled => self.fail(b"fem2d.pcg-cancelled".to_vec()),
                StepOutcome::Fault(fault) => self.fail(fault.detail),
            },
            MountedStage::CommitReady => {
                let validation = current_identity(self.identity.app_instance_id);
                let store_is_current = self.snapshot.as_ref().is_some_and(|snapshot| snapshot.commit_authority_matches(self.identity.generation.0, self.identity.canonical_base_revision));
                if validation != Some(self.identity) || !store_is_current || self.cancel.is_cancelled_now() {
                    return self.fail(b"fem2d.session-stale-commit".to_vec());
                }
                self.visual.validated_final = self.visual.converged;
                self.stage = MountedStage::Complete;
                let mut output = Vec::with_capacity(32);
                output.extend_from_slice(&self.identity.operation.0.to_le_bytes());
                output.extend_from_slice(&self.identity.base_revision.0.to_le_bytes());
                output.extend_from_slice(&self.identity.generation.0.to_le_bytes());
                output.extend_from_slice(&(self.admitted_items as u64).to_le_bytes());
                JobStep::Done(output)
            }
            MountedStage::Complete => JobStep::Done(Vec::new()),
            MountedStage::Fault => JobStep::Failed(self.fault.clone().unwrap_or_else(|| b"fem2d.session-fault".to_vec())),
            MountedStage::Closing | MountedStage::Empty => JobStep::Failed(b"fem2d.session-closed".to_vec()),
        }
    }

    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> PluginCloseStep {
        if maximum_items == 0 || maximum_bytes < 128 {
            return PluginCloseStep::Pending { released_items: 0, released_bytes: 0 };
        }
        self.cancel.cancel_now();
        self.stage = MountedStage::Closing;
        loop {
            match self.close_cursor {
                0 => {
                    if let Some(graph) = self.graph.as_mut() {
                        let (terminal, released_items, released_bytes) = graph.close_step(maximum_bytes);
                        if !terminal {
                            return PluginCloseStep::Pending { released_items, released_bytes };
                        }
                        self.graph = None;
                        return PluginCloseStep::Pending { released_items: 1, released_bytes: std::mem::size_of::<FemJobGraph>() };
                    }
                    self.close_cursor += 1;
                }
                1 => {
                    if let Some(mesh) = self.mesh.as_mut() {
                        let (terminal, released_items, released_bytes) = mesh.close_step(maximum_bytes);
                        if !terminal {
                            return PluginCloseStep::Pending { released_items, released_bytes };
                        }
                        self.mesh = None;
                        return PluginCloseStep::Pending { released_items: 1, released_bytes: std::mem::size_of::<MeshJob>() };
                    }
                    self.close_cursor += 1;
                }
                2 => {
                    if let Some(model_build) = self.model_build.as_mut() {
                        let (terminal, released_bytes) = model_build.close_step();
                        if !terminal {
                            return PluginCloseStep::Pending { released_items: 1, released_bytes };
                        }
                        self.model_build = None;
                        return PluginCloseStep::Pending { released_items: 1, released_bytes: std::mem::size_of::<MountedModelBuild>() };
                    }
                    self.close_cursor += 1;
                }
                3 => {
                    if let Some(assembly) = self.assembly.as_mut() {
                        let (terminal, released_items, released_bytes) = assembly.close_step(maximum_bytes);
                        if !terminal {
                            return PluginCloseStep::Pending { released_items, released_bytes };
                        }
                        self.assembly = None;
                        return PluginCloseStep::Pending { released_items: 1, released_bytes: std::mem::size_of::<AssemblyJob<'static>>() };
                    }
                    self.close_cursor += 1;
                }
                4 => {
                    if let Some(csr_build) = self.csr_build.as_mut() {
                        let (terminal, released_items, released_bytes) = csr_build.close_step(maximum_bytes);
                        if !terminal {
                            return PluginCloseStep::Pending { released_items, released_bytes };
                        }
                        self.csr_build = None;
                        return PluginCloseStep::Pending { released_items: 1, released_bytes: std::mem::size_of::<AssemblyCsrBuild>() };
                    }
                    self.close_cursor += 1;
                }
                5 => {
                    if let Some(pcg_build) = self.pcg_build.as_mut() {
                        let (terminal, released_bytes) = pcg_build.close_step();
                        if !terminal {
                            return PluginCloseStep::Pending { released_items: 1, released_bytes };
                        }
                        self.pcg_build = None;
                        return PluginCloseStep::Pending { released_items: 1, released_bytes: std::mem::size_of::<PcgJobConstruction>() };
                    }
                    self.close_cursor += 1;
                }
                6 => {
                    if let Some(pcg) = self.pcg.as_mut() {
                        let (terminal, released_items, released_bytes) = pcg.close_step(maximum_bytes);
                        if !terminal {
                            return PluginCloseStep::Pending { released_items, released_bytes };
                        }
                        self.pcg = None;
                        return PluginCloseStep::Pending { released_items: 1, released_bytes: std::mem::size_of::<PcgJob>() };
                    }
                    self.close_cursor += 1;
                }
                7 => {
                    if let Some(model) = self.model.as_mut() {
                        let Some(model) = std::sync::Arc::get_mut(model) else {
                            return PluginCloseStep::Blocked { reason: "mounted FEM model root is still held by a child job" };
                        };
                        if model.nodes.pop().is_some() {
                            return PluginCloseStep::Pending { released_items: 1, released_bytes: std::mem::size_of::<crate::model::Node>() };
                        }
                        if model.elements.pop().is_some() {
                            return PluginCloseStep::Pending { released_items: 1, released_bytes: std::mem::size_of::<crate::model::Elements>() };
                        }
                        if model.supports.pop().is_some() {
                            return PluginCloseStep::Pending { released_items: 1, released_bytes: std::mem::size_of::<crate::model::Support>() };
                        }
                        self.model = None;
                        return PluginCloseStep::Pending { released_items: 1, released_bytes: std::mem::size_of::<std::sync::Arc<AnalysisModel>>() };
                    }
                    self.close_cursor += 1;
                }
                8 => {
                    if let Some(domain) = self.domain.as_mut() {
                        if domain.outer.pop().is_some() {
                            return PluginCloseStep::Pending { released_items: 1, released_bytes: std::mem::size_of::<[f64; 2]>() };
                        }
                        if let Some(hole) = domain.holes.last_mut() {
                            if hole.pop().is_some() {
                                return PluginCloseStep::Pending { released_items: 1, released_bytes: std::mem::size_of::<[f64; 2]>() };
                            }
                            domain.holes.pop();
                            return PluginCloseStep::Pending { released_items: 1, released_bytes: std::mem::size_of::<Vec<[f64; 2]>>() };
                        }
                        self.domain = None;
                        return PluginCloseStep::Pending { released_items: 1, released_bytes: std::mem::size_of::<PlanarDomain>() };
                    }
                    self.close_cursor += 1;
                }
                9 => {
                    if self.visual.fields.pop().is_some() {
                        return PluginCloseStep::Pending { released_items: 1, released_bytes: std::mem::size_of_val(&self.visual.fields) };
                    }
                    self.close_cursor += 1;
                }
                10 => {
                    if self.visual.assembling_element_ids.pop().is_some() {
                        return PluginCloseStep::Pending { released_items: 1, released_bytes: std::mem::size_of::<String>() };
                    }
                    self.close_cursor += 1;
                }
                11 => {
                    if self.visual.region_quality.extract_if(|_, _| true).next().is_some() {
                        return PluginCloseStep::Pending { released_items: 1, released_bytes: std::mem::size_of::<(String, RegionVisualQuality)>() };
                    }
                    self.close_cursor += 1;
                }
                12 => {
                    if self.snapshot.take().is_some() {
                        return PluginCloseStep::Pending { released_items: 1, released_bytes: std::mem::size_of::<store::SnapshotRead<Fem2dSnapshot>>() };
                    }
                    self.close_cursor += 1;
                }
                13 => {
                    if self.fault.take().is_some() {
                        return PluginCloseStep::Pending { released_items: 1, released_bytes: std::mem::size_of::<Vec<u8>>() };
                    }
                    self.close_cursor += 1;
                }
                _ => {
                    self.stage = MountedStage::Empty;
                    return PluginCloseStep::Complete;
                }
            }
        }
    }

    fn terminal_is_empty(&self) -> bool {
        self.stage == MountedStage::Empty
            && self.snapshot.is_none()
            && self.graph.is_none()
            && self.mesh.is_none()
            && self.model_build.is_none()
            && self.model.is_none()
            && self.assembly.is_none()
            && self.csr_build.is_none()
            && self.pcg_build.is_none()
            && self.pcg.is_none()
            && self.domain.is_none()
            && self.visual.region_quality.is_empty()
            && self.visual.assembling_element_ids.is_empty()
            && self.visual.fields.is_empty()
            && self.fault.is_none()
    }
}

#[derive(Clone, Copy)]
struct CurrentSession {
    app_instance_id: u32,
    shell: u16,
    identity: MountedIdentity,
}

#[derive(Clone, Copy)]
struct PendingAdmission {
    app_instance_id: u32,
    shell: u16,
    identity: MountedIdentity,
}

struct MountedRegistry {
    shells: [Rc<RefCell<Option<MountedState>>>; SESSION_SHELL_CAPACITY],
    current: [Option<CurrentSession>; SESSION_ACTIVE_CAPACITY],
    pending: [Option<PendingAdmission>; SESSION_ACTIVE_CAPACITY],
    retiring: [Option<u16>; SESSION_SHELL_CAPACITY],
    free: [u16; SESSION_SHELL_CAPACITY],
    free_read: usize,
    free_len: usize,
    credits: [bool; SESSION_SHELL_CAPACITY],
    reserved_items: usize,
    reserved_bytes: usize,
    next_job: u64,
}

impl MountedRegistry {
    fn new() -> Self {
        Self {
            shells: std::array::from_fn(|_| Rc::new(RefCell::new(None))),
            current: [None; SESSION_ACTIVE_CAPACITY],
            pending: [None; SESSION_ACTIVE_CAPACITY],
            retiring: [None; SESSION_SHELL_CAPACITY],
            free: std::array::from_fn(|index| index as u16),
            free_read: 0,
            free_len: SESSION_SHELL_CAPACITY,
            credits: [false; SESSION_SHELL_CAPACITY],
            reserved_items: 0,
            reserved_bytes: 0,
            next_job: 0,
        }
    }

    fn allocate(&mut self) -> Option<u16> {
        if self.free_len == 0 {
            return None;
        }
        let shell = self.free[self.free_read];
        self.free_read = (self.free_read + 1) % SESSION_SHELL_CAPACITY;
        self.free_len -= 1;
        Some(shell)
    }

    fn release(&mut self, shell: u16) {
        assert!(!self.credits[shell as usize], "mounted FEM shell released before its process credit");
        let write = (self.free_read + self.free_len) % SESSION_SHELL_CAPACITY;
        self.free[write] = shell;
        self.free_len += 1;
    }

    fn reserve_credit(&mut self, shell: u16) -> bool {
        if self.credits[shell as usize] {
            return false;
        }
        let Some(items) = self.reserved_items.checked_add(SESSION_MAXIMUM_ITEMS) else { return false };
        let Some(bytes) = self.reserved_bytes.checked_add(SESSION_MAXIMUM_BYTES) else { return false };
        if items > SESSION_SHELL_CAPACITY * SESSION_MAXIMUM_ITEMS || bytes > SESSION_SHELL_CAPACITY * SESSION_MAXIMUM_BYTES {
            return false;
        }
        self.credits[shell as usize] = true;
        self.reserved_items = items;
        self.reserved_bytes = bytes;
        true
    }

    fn release_credit(&mut self, shell: u16) {
        assert!(self.credits[shell as usize], "mounted FEM shell released a missing process credit");
        self.credits[shell as usize] = false;
        self.reserved_items -= SESSION_MAXIMUM_ITEMS;
        self.reserved_bytes -= SESSION_MAXIMUM_BYTES;
    }

    fn retain_retirement(&mut self, shell: u16) -> bool {
        let Some(slot) = self.retiring.iter_mut().find(|slot| slot.is_none()) else { return false };
        *slot = Some(shell);
        true
    }
}

thread_local! {
    static MOUNTED: RefCell<MountedRegistry> = RefCell::new(MountedRegistry::new());
}
//#endregion 🧰️FixedArena

//#region 💼️JobBridge
struct MountedBoundedJob {
    shell: Rc<RefCell<Option<MountedState>>>,
    identity: MountedIdentity,
}

impl BoundedJob for MountedBoundedJob {
    fn step(&mut self, budget: JobBudget) -> JobStep {
        let Ok(mut shell) = self.shell.try_borrow_mut() else { return JobStep::Running(None) };
        let Some(state) = shell.as_mut() else { return JobStep::Failed(b"fem2d.session-owner-missing".to_vec()) };
        if state.identity != self.identity {
            return JobStep::Failed(b"fem2d.session-aba".to_vec());
        }
        state.step(budget)
    }

    fn cancel(&mut self) {
        if let Ok(shell) = self.shell.try_borrow() {
            if let Some(state) = shell.as_ref() {
                state.cancel.cancel_now();
            }
        }
    }

    fn checkpoint(&self) -> Option<Vec<u8>> {
        let shell = self.shell.try_borrow().ok()?;
        let state = shell.as_ref()?;
        let mut bytes = Vec::with_capacity(24);
        bytes.extend_from_slice(&state.identity.operation.0.to_le_bytes());
        bytes.extend_from_slice(&state.identity.base_revision.0.to_le_bytes());
        bytes.extend_from_slice(&state.identity.generation.0.to_le_bytes());
        Some(bytes)
    }

    fn terminal_drop_is_shallow(&self) -> bool {
        true
    }
}

fn encode_input(shell: u16, identity: MountedIdentity) -> Vec<u8> {
    let mut input = Vec::with_capacity(INPUT_BYTES);
    input.push(1);
    input.extend_from_slice(&shell.to_le_bytes());
    input.extend_from_slice(&identity.app_instance_id.to_le_bytes());
    input.extend_from_slice(&identity.base_revision.0.to_le_bytes());
    input.extend_from_slice(&identity.generation.0.to_le_bytes());
    input.extend_from_slice(&identity.canonical_base_revision);
    input.extend_from_slice(&identity.operation.0.to_le_bytes());
    input
}

fn decode_input(job: u64, input: &[u8]) -> Option<(u16, MountedIdentity)> {
    if input.len() != INPUT_BYTES || input[0] != 1 {
        return None;
    }
    let shell = u16::from_le_bytes(input[1..3].try_into().ok()?);
    let app_instance_id = u32::from_le_bytes(input[3..7].try_into().ok()?);
    let base_revision = semio_framework_job::RevisionId(u64::from_le_bytes(input[7..15].try_into().ok()?));
    let generation = semio_framework_job::Generation(u64::from_le_bytes(input[15..23].try_into().ok()?));
    let canonical_base_revision = input[23..55].try_into().ok()?;
    let operation = OperationId(u64::from_le_bytes(input[55..63].try_into().ok()?));
    if operation.0 != job || job & !FEM2D_JOB_COUNTER_MAXIMUM != FEM2D_JOB_TAG {
        return None;
    }
    Some((shell, MountedIdentity { app_instance_id, base_revision, generation, canonical_base_revision, operation, job }))
}

fn mounted_job_factory(job: u64, input: &[u8]) -> Result<Box<dyn BoundedJob>, Vec<u8>> {
    let (shell, identity) = decode_input(job, input).ok_or_else(|| b"fem2d.session-input".to_vec())?;
    MOUNTED.with(|registry| {
        let registry = registry.borrow();
        let shell_owner = registry.shells.get(shell as usize).ok_or_else(|| b"fem2d.session-shell".to_vec())?.clone();
        let matches = shell_owner.try_borrow().is_ok_and(|owner| owner.as_ref().is_some_and(|state| state.identity == identity));
        if !matches {
            return Err(b"fem2d.session-stale-factory".to_vec());
        }
        Ok(Box::new(MountedBoundedJob { shell: shell_owner, identity }) as Box<dyn BoundedJob>)
    })
}

/// 🏭️ Preallocates the fixed arena at plugin installation and registers the explicit job factory.
pub fn initialize() {
    MOUNTED.with(|registry| {
        let _ = registry.borrow().free_len;
    });
    semio_framework_plugin::reactor::jobs::register_bounded_job_kind(FEM2D_MOUNTED_JOB_KIND, mounted_job_factory as BoundedJobFactory);
}
//#endregion 💼️JobBridge

//#region 🎛️LiveMount
fn current_identity(app_instance_id: u32) -> Option<MountedIdentity> {
    MOUNTED.with(|registry| {
        let registry = registry.borrow();
        let slot = app_instance_id as usize % SESSION_ACTIVE_CAPACITY;
        registry.current[slot].filter(|current| current.app_instance_id == app_instance_id).map(|current| current.identity)
    })
}

/// 🔍️ Proves that this operation would admit a distinct mounted owner before the shared
/// application requests an opaque snapshot lease. Unchanged renders and occupied collisions issue
/// no lease, so idle frame polling cannot consume the fixed lease registry.
pub fn needs_snapshot_read(render: AppRenderOperationContext) -> bool {
    if render.app_instance_id == 0 {
        return false;
    }
    MOUNTED.with(|registry| {
        let mut registry = registry.borrow_mut();
        let current_slot = render.app_instance_id as usize % SESSION_ACTIVE_CAPACITY;
        let previous = match registry.current[current_slot] {
            Some(current) if current.app_instance_id != render.app_instance_id => return false,
            Some(current) if current.identity.base_revision == render.base_revision && current.identity.generation == render.generation => return false,
            Some(current) => Some(current),
            None => None,
        };
        if let Some(pending) = registry.pending[current_slot] {
            if pending.app_instance_id != render.app_instance_id {
                return false;
            }
            if pending.identity.base_revision == render.base_revision && pending.identity.generation == render.generation && pending.identity.canonical_base_revision == render.canonical_base_revision {
                return true;
            }
            registry.pending[current_slot] = None;
            registry.release_credit(pending.shell);
            registry.release(pending.shell);
        }
        if previous.is_some() && !registry.retiring.iter().any(Option::is_none) {
            return false;
        }
        let Some(shell) = registry.allocate() else { return false };
        if !registry.reserve_credit(shell) {
            registry.release(shell);
            return false;
        }
        let Some(counter) = registry.next_job.checked_add(1).filter(|counter| *counter <= FEM2D_JOB_COUNTER_MAXIMUM) else {
            registry.release_credit(shell);
            registry.release(shell);
            return false;
        };
        registry.next_job = counter;
        let job = FEM2D_JOB_TAG | counter;
        let identity = MountedIdentity { app_instance_id: render.app_instance_id, base_revision: render.base_revision, generation: render.generation, canonical_base_revision: render.canonical_base_revision, operation: OperationId(job), job };
        registry.pending[current_slot] = Some(PendingAdmission { app_instance_id: render.app_instance_id, shell, identity });
        true
    })
}

/// 🔁️ Reconciles one live store revision and returns only the exact host job effects needed.
pub fn reconcile(doc: &ArtifactView<'_, Fem2dSnapshot>) -> Vec<Effect> {
    let Some(render) = doc.render_operation() else { return Vec::new() };
    if render.app_instance_id == 0 {
        return Vec::new();
    }
    MOUNTED.with(|registry| {
        let mut registry = registry.borrow_mut();
        let current_slot = render.app_instance_id as usize % SESSION_ACTIVE_CAPACITY;
        let previous = if let Some(current) = registry.current[current_slot] {
            if current.app_instance_id != render.app_instance_id {
                return Vec::new();
            }
            if current.identity.base_revision == render.base_revision && current.identity.generation == render.generation {
                return Vec::new();
            }
            if !registry.retiring.iter().any(Option::is_none) {
                return Vec::new();
            }
            Some(current)
        } else {
            None
        };
        let Some(pending) = registry.pending[current_slot].filter(|pending| {
            pending.app_instance_id == render.app_instance_id && pending.identity.base_revision == render.base_revision && pending.identity.generation == render.generation && pending.identity.canonical_base_revision == render.canonical_base_revision
        }) else {
            return Vec::new();
        };
        let shell = pending.shell;
        let snapshot = match doc.take_snapshot_read() {
            Ok(snapshot) => snapshot,
            Err(_) => return Vec::new(),
        };
        registry.pending[current_slot] = None;
        if let Some(previous) = previous {
            assert!(registry.retain_retirement(previous.shell), "retirement slot was preflighted under the registry owner");
            if let Ok(shell) = registry.shells[previous.shell as usize].try_borrow() {
                if let Some(state) = shell.as_ref() {
                    state.cancel.cancel_now();
                }
            }
            registry.current[current_slot] = None;
        }
        let identity = pending.identity;
        let job = identity.job;
        *registry.shells[shell as usize].borrow_mut() = Some(MountedState::new(identity, snapshot));
        registry.current[current_slot] = Some(CurrentSession { app_instance_id: render.app_instance_id, shell, identity });
        let mut effects = Vec::with_capacity(2);
        if let Some(previous) = previous {
            effects.push(Effect::CancelJob { job: previous.identity.job });
        }
        effects.push(Effect::SpawnJob { job, kind: FEM2D_MOUNTED_JOB_KIND.to_string(), input: encode_input(shell, identity), placement: JobPlacement::Isolated });
        effects
    })
}

/// 👁️ Borrows the exact latest visual only while the renderer builds its node.
pub fn with_live_visual<R>(render: Option<AppRenderOperationContext>, build: impl FnOnce(Option<&Fem2dLiveVisual>) -> R) -> R {
    let Some(render) = render else { return build(None) };
    let shell = MOUNTED.with(|registry| {
        let registry = registry.borrow();
        let current = registry.current[render.app_instance_id as usize % SESSION_ACTIVE_CAPACITY]?;
        if current.app_instance_id != render.app_instance_id || current.identity.base_revision != render.base_revision || current.identity.generation != render.generation {
            return None;
        }
        Some(registry.shells[current.shell as usize].clone())
    });
    let Some(shell) = shell else { return build(None) };
    let Ok(owner) = shell.try_borrow() else { return build(None) };
    build(owner.as_ref().map(|state| &state.visual))
}

fn retire_one(app_instance_id: u32, maximum_items: usize, maximum_bytes: usize) -> PluginCloseStep {
    MOUNTED.with(|registry| {
        let mut registry = registry.borrow_mut();
        let Some(index) = registry.retiring.iter().position(|shell| shell.is_some_and(|shell| registry.shells[shell as usize].try_borrow().is_ok_and(|owner| owner.as_ref().is_some_and(|state| state.identity.app_instance_id == app_instance_id))))
        else {
            return PluginCloseStep::Complete;
        };
        let shell = registry.retiring[index].expect("matched retirement shell");
        let step = {
            let mut owner = match registry.shells[shell as usize].try_borrow_mut() {
                Ok(owner) => owner,
                Err(_) => return PluginCloseStep::Blocked { reason: "mounted FEM job owner is checked out by its worker turn" },
            };
            let Some(state) = owner.as_mut() else { return PluginCloseStep::Complete };
            state.close_step(maximum_items.min(1), maximum_bytes)
        };
        if step == PluginCloseStep::Complete {
            let terminal = registry.shells[shell as usize].try_borrow().is_ok_and(|owner| owner.as_ref().is_some_and(MountedState::terminal_is_empty));
            if !terminal {
                return PluginCloseStep::Blocked { reason: "mounted FEM job reported a false terminal shell" };
            }
            *registry.shells[shell as usize].borrow_mut() = None;
            registry.retiring[index] = None;
            registry.release_credit(shell);
            registry.release(shell);
            return PluginCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        step
    })
}

pub fn maintenance_step(app_instance_id: u32, maximum_items: usize, maximum_bytes: usize) -> PluginCloseStep {
    retire_one(app_instance_id, maximum_items, maximum_bytes)
}

pub fn close_step(app_instance_id: u32, maximum_items: usize, maximum_bytes: usize) -> PluginCloseStep {
    MOUNTED.with(|registry| {
        let mut registry = registry.borrow_mut();
        let slot = app_instance_id as usize % SESSION_ACTIVE_CAPACITY;
        if let Some(pending) = registry.pending[slot].filter(|pending| pending.app_instance_id == app_instance_id) {
            registry.pending[slot] = None;
            registry.release_credit(pending.shell);
            registry.release(pending.shell);
            return PluginCloseStep::Pending { released_items: 1, released_bytes: INPUT_BYTES };
        }
        if let Some(current) = registry.current[slot].filter(|current| current.app_instance_id == app_instance_id) {
            if !registry.retain_retirement(current.shell) {
                return PluginCloseStep::Blocked { reason: "mounted FEM retirement arena is saturated" };
            }
            if let Ok(owner) = registry.shells[current.shell as usize].try_borrow() {
                if let Some(state) = owner.as_ref() {
                    state.cancel.cancel_now();
                }
            }
            registry.current[slot] = None;
            return PluginCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        drop(registry);
        retire_one(app_instance_id, maximum_items, maximum_bytes)
    })
}

pub fn terminal_is_empty(app_instance_id: u32) -> bool {
    MOUNTED.with(|registry| {
        let registry = registry.borrow();
        let slot = app_instance_id as usize % SESSION_ACTIVE_CAPACITY;
        registry.pending[slot].is_none_or(|pending| pending.app_instance_id != app_instance_id)
            && registry.current[slot].is_none_or(|current| current.app_instance_id != app_instance_id)
            && registry.retiring.iter().all(|shell| shell.is_none_or(|shell| registry.shells[shell as usize].try_borrow().is_ok_and(|owner| owner.as_ref().is_none_or(|state| state.identity.app_instance_id != app_instance_id))))
    })
}
//#endregion 🎛️LiveMount

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_authority_round_trips_and_rejects_hostile_lengths() {
        let job = FEM2D_JOB_TAG | 19;
        let identity = MountedIdentity { app_instance_id: 7, base_revision: semio_framework_job::RevisionId(11), generation: semio_framework_job::Generation(13), canonical_base_revision: [23; 32], operation: OperationId(job), job };
        assert_eq!(decode_input(job, &encode_input(3, identity)), Some((3, identity)));
        assert_eq!(decode_input(job, &encode_input(3, identity)[..INPUT_BYTES - 1]), None);
        assert_eq!(decode_input(job + 1, &encode_input(3, identity)), None, "a host job-id substitution must not inherit the retained owner");
    }

    #[test]
    fn mounted_close_and_capacity_are_fixed_and_terminal_witnessed() {
        let mut registry = MountedRegistry::new();
        let mut admitted = Vec::new();
        for _ in 0..SESSION_SHELL_CAPACITY {
            admitted.push(registry.allocate().expect("exact fixed slot"));
        }
        assert!(registry.allocate().is_none(), "capacity plus one must fail without growing");
        let returned = admitted.pop().expect("slot");
        registry.release(returned);
        assert_eq!(registry.allocate(), Some(returned), "fixed free ring deterministically reuses the returned slot");
        let identity = MountedIdentity { app_instance_id: 1, base_revision: semio_framework_job::RevisionId(2), generation: semio_framework_job::Generation(3), canonical_base_revision: [4; 32], operation: OperationId(5), job: 5 };
        registry.current[1] = Some(CurrentSession { app_instance_id: 1, shell: returned, identity });
        assert!(registry.current[33 % SESSION_ACTIVE_CAPACITY].filter(|current| current.app_instance_id == 33).is_none(), "a modulo collision must not inherit another app's generation authority");
        assert_eq!(maintenance_step(u32::MAX, 1, 4_096), PluginCloseStep::Complete);
        assert!(terminal_is_empty(u32::MAX));
    }

    #[test]
    fn snapshot_lease_is_preceded_by_a_fixed_pending_admission_and_idle_polling_reuses_it() {
        let render = AppRenderOperationContext { app_instance_id: 2_000_000_007, base_revision: semio_framework_job::RevisionId(17), generation: semio_framework_job::Generation(19), canonical_base_revision: [23; 32] };
        assert!(needs_snapshot_read(render));
        let first = MOUNTED.with(|registry| registry.borrow().pending[render.app_instance_id as usize % SESSION_ACTIVE_CAPACITY].expect("pending admission"));
        for _ in 0..1_025 {
            assert!(needs_snapshot_read(render));
        }
        let retained = MOUNTED.with(|registry| registry.borrow().pending[render.app_instance_id as usize % SESSION_ACTIVE_CAPACITY].expect("same pending admission"));
        assert_eq!(first.shell, retained.shell);
        assert_eq!(first.identity, retained.identity);
        assert!(matches!(close_step(render.app_instance_id, 1, 4_096), PluginCloseStep::Pending { released_items: 1, .. }));
        assert!(terminal_is_empty(render.app_instance_id));
    }

    #[test]
    fn mounted_revision_restart_keeps_cancel_before_spawn() {
        let source = include_str!("component.rs");
        let admission = &source[source.find("pub fn needs_snapshot_read(").expect("admission")..source.find("pub fn reconcile(").expect("reconcile")];
        let reconcile = &source[source.find("pub fn reconcile(").expect("reconcile")..source.find("pub fn with_live_visual").expect("visual boundary")];
        assert!(reconcile.find("Effect::CancelJob").expect("cancel effect") < reconcile.find("Effect::SpawnJob").expect("spawn effect"));
        assert!(admission.contains("checked_add(1)"));
        assert!(admission.contains("reserve_credit(shell)"));
        assert!(reconcile.contains("take_snapshot_read()"));
        assert!(reconcile.contains("retiring.iter().any(Option::is_none)"));
    }

    #[test]
    fn commit_identity_requires_base_revision_and_generation() {
        let operation = Operation::new(OperationId(1), semio_framework_job::RevisionId(2), semio_framework_job::Generation(3), 4);
        assert_eq!(semio_framework_job::validate_commit(&operation, operation.base_revision, operation.generation), CommitValidation::Accepted);
        assert!(matches!(semio_framework_job::validate_commit(&operation, semio_framework_job::RevisionId(9), operation.generation), CommitValidation::Stale { .. }));
        assert!(matches!(semio_framework_job::validate_commit(&operation, operation.base_revision, semio_framework_job::Generation(9)), CommitValidation::Stale { .. }));
    }

    #[test]
    fn source_contract_keeps_one_semantic_child_step_and_live_visual_consumer() {
        let source = include_str!("component.rs");
        for needle in [
            "FemJobGraph::new",
            "MeshJob::new",
            "AssemblyJob::new_owned",
            "PcgJob::new",
            "commit_authority_matches",
            "Effect::SpawnJob",
            "Effect::CancelJob",
            "JobPlacement::Isolated",
            "with_live_visual",
            "take_snapshot_read",
            "SESSION_MAXIMUM_ITEMS",
            "SESSION_MAXIMUM_BYTES",
        ] {
            assert!(source.contains(needle), "missing mounted FEM contract {needle}");
        }
    }
}
//#endregion 🧪️Tests
