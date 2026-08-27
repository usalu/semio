//! 🧵️ Mounted Fem2d revision job: fixed session arena, retained worker step and live visual lease.

use crate::analyses::{AnalysisModel, AssemblyCsrBuild, AssemblyJob, AssemblyJobConstruction, FemJobGraph, FemJobStage, FemStagePlan};
use crate::artifacts::fem2d::{Fem2dSnapshot, FemElement, FemLoad};
use crate::editor::fem2d::modes::edit::windows::model::{Fem2dLiveVisual, Fem2dMountedVisualLease, Fem2dVisualFreshness, Fem2dVisualJob, FemVisualState, RegionVisualQuality};
use crate::mesh::{MeshJob, MeshOpts, PlanarDomain, TriMesh2};
use crate::model::Element;
use crate::sparse::{PcgJob, PcgJobConstruction};
use semio_framework::kernel::{Effect, JobPlacement};
use semio_framework_job::{CommitValidation, InteractiveJob, Operation, OperationId, RetainedJobPayload, StepBudget, StepContext, StepOutcome};
use semio_framework_plugin::reactor::jobs::{BoundedJob, BoundedJobFactory, JobBudget, JobStep};
use semio_framework_plugin::{AppRenderOperationContext, ArtifactView, PluginCloseStep};
use std::cell::RefCell;
use std::rc::Rc;

//#region 🔖️Contract
pub const FEM2D_MOUNTED_JOB_KIND: &str = "semio.fem2d.mounted-analysis";
const SESSION_ACTIVE_CAPACITY: usize = 32;
const SESSION_SHELL_CAPACITY: usize = 64;
const SESSION_MAXIMUM_INPUT_ITEMS: usize = 4_096;
const SESSION_MAXIMUM_INPUT_BYTES: usize = 4 * 1_024 * 1_024;
const SESSION_MAXIMUM_NODES: usize = 8;
const SESSION_MAXIMUM_ELEMENTS: usize = 2;
const SESSION_MAXIMUM_SUPPORTS: usize = 64;
const SESSION_MAXIMUM_MESH_POINTS: usize = 8;
const SESSION_MAXIMUM_MESH_TRIANGLES: usize = 2;
const SESSION_MAXIMUM_REGION_HOLES: usize = 16;
const SESSION_MAXIMUM_BOUNDARY_POINTS: usize = 64;
const SESSION_MAXIMUM_OUTPUT_BYTES: usize = 16 * 1_024;
const SESSION_MAXIMUM_VISUAL_LOADS: usize = 64;
const SESSION_MAXIMUM_FAULT_BYTES: usize = 4_096;
const SESSION_OWNER_PAGE_BYTES: usize = 4_096;
const SESSION_MAXIMUM_STRING_BYTES: usize = SESSION_OWNER_PAGE_BYTES;
const INPUT_BYTES: usize = 63;
const FEM2D_JOB_TAG: u64 = 0xf2d0_0000_0000_0000;
const FEM2D_JOB_COUNTER_MAXIMUM: u64 = 0x000f_ffff_ffff_ffff;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum MountedOwnerClass {
    GraphPlans,
    DomainOuter,
    DomainHoleList,
    DomainHoleVectors,
    ModelNodes,
    ModelElements,
    ModelSupports,
    ModelRegionNodeIds,
    ModelStrings,
    SupportDofs,
    MeshPreparationVectors,
    MeshPreparationIndexVector,
    MeshPreparedVectors,
    MeshTriangulationVectors,
    MeshTriangulationWorkspaceVectors,
    MeshConstraintVector,
    MeshEdgeIndexVectors,
    MeshOutputVectors,
    AssemblyDofOrderVector,
    AssemblyDofStrings,
    AssemblyPlanVectors,
    AssemblyPartitionVectors,
    AssemblyMergeVectors,
    AssemblyPendingVectors,
    CsrVectors,
    PcgVectors,
    VisualVectors,
    VisualStrings,
    OutputPages,
    FaultPages,
}

#[derive(Clone, Copy)]
struct MountedOwnerClaim {
    class: MountedOwnerClass,
    roots: usize,
    items: usize,
}

#[derive(Clone, Copy)]
struct MountedProcessOwnerCatalog {
    claims: [MountedOwnerClaim; 30],
}

impl MountedProcessOwnerCatalog {
    const fn fixed() -> Self {
        Self {
            claims: [
                MountedOwnerClaim { class: MountedOwnerClass::GraphPlans, roots: 1, items: 8 },
                MountedOwnerClaim { class: MountedOwnerClass::DomainOuter, roots: 1, items: SESSION_MAXIMUM_BOUNDARY_POINTS },
                MountedOwnerClaim { class: MountedOwnerClass::DomainHoleList, roots: 1, items: SESSION_MAXIMUM_REGION_HOLES },
                MountedOwnerClaim { class: MountedOwnerClass::DomainHoleVectors, roots: SESSION_MAXIMUM_REGION_HOLES, items: SESSION_MAXIMUM_REGION_HOLES * SESSION_MAXIMUM_BOUNDARY_POINTS },
                MountedOwnerClaim { class: MountedOwnerClass::ModelNodes, roots: 1, items: SESSION_MAXIMUM_NODES + SESSION_MAXIMUM_MESH_POINTS },
                MountedOwnerClaim { class: MountedOwnerClass::ModelElements, roots: 1, items: SESSION_MAXIMUM_ELEMENTS + SESSION_MAXIMUM_MESH_TRIANGLES },
                MountedOwnerClaim { class: MountedOwnerClass::ModelSupports, roots: 1, items: SESSION_MAXIMUM_SUPPORTS },
                MountedOwnerClaim { class: MountedOwnerClass::ModelRegionNodeIds, roots: 1, items: SESSION_MAXIMUM_MESH_POINTS },
                MountedOwnerClaim {
                    class: MountedOwnerClass::ModelStrings,
                    roots: 2 * (SESSION_MAXIMUM_NODES + SESSION_MAXIMUM_MESH_POINTS) + 3 * SESSION_MAXIMUM_ELEMENTS + 4 * SESSION_MAXIMUM_MESH_TRIANGLES + SESSION_MAXIMUM_SUPPORTS + 2,
                    items: 0,
                },
                MountedOwnerClaim { class: MountedOwnerClass::SupportDofs, roots: SESSION_MAXIMUM_SUPPORTS, items: 6 * SESSION_MAXIMUM_SUPPORTS },
                MountedOwnerClaim { class: MountedOwnerClass::MeshPreparationVectors, roots: 2, items: SESSION_MAXIMUM_MESH_POINTS + 3 * SESSION_MAXIMUM_MESH_TRIANGLES },
                MountedOwnerClaim { class: MountedOwnerClass::MeshPreparationIndexVector, roots: 1, items: SESSION_MAXIMUM_MESH_POINTS },
                MountedOwnerClaim { class: MountedOwnerClass::MeshPreparedVectors, roots: 2, items: SESSION_MAXIMUM_MESH_POINTS + 3 * SESSION_MAXIMUM_MESH_TRIANGLES },
                MountedOwnerClaim { class: MountedOwnerClass::MeshTriangulationVectors, roots: 3, items: SESSION_MAXIMUM_MESH_POINTS + 2 * (4 * SESSION_MAXIMUM_MESH_TRIANGLES + 1) },
                MountedOwnerClaim { class: MountedOwnerClass::MeshTriangulationWorkspaceVectors, roots: 3, items: 3 * (12 * SESSION_MAXIMUM_MESH_TRIANGLES + 3) },
                MountedOwnerClaim { class: MountedOwnerClass::MeshConstraintVector, roots: 1, items: 3 * SESSION_MAXIMUM_MESH_TRIANGLES },
                MountedOwnerClaim { class: MountedOwnerClass::MeshEdgeIndexVectors, roots: 3, items: 3 * SESSION_MAXIMUM_MESH_TRIANGLES + 2 * (12 * SESSION_MAXIMUM_MESH_TRIANGLES + 3) },
                MountedOwnerClaim { class: MountedOwnerClass::MeshOutputVectors, roots: 3, items: 2 * SESSION_MAXIMUM_MESH_POINTS + SESSION_MAXIMUM_MESH_TRIANGLES },
                MountedOwnerClaim { class: MountedOwnerClass::AssemblyDofOrderVector, roots: 1, items: 3 * (SESSION_MAXIMUM_NODES + SESSION_MAXIMUM_MESH_POINTS) },
                MountedOwnerClaim { class: MountedOwnerClass::AssemblyDofStrings, roots: 3 * (SESSION_MAXIMUM_NODES + SESSION_MAXIMUM_MESH_POINTS), items: 0 },
                MountedOwnerClaim { class: MountedOwnerClass::AssemblyPlanVectors, roots: 4, items: 12 * (SESSION_MAXIMUM_NODES + SESSION_MAXIMUM_MESH_POINTS) },
                MountedOwnerClaim { class: MountedOwnerClass::AssemblyPartitionVectors, roots: 3, items: 1 + 2 * 144 },
                MountedOwnerClaim { class: MountedOwnerClass::AssemblyMergeVectors, roots: 4, items: 2 + 2 * 144 },
                MountedOwnerClaim { class: MountedOwnerClass::AssemblyPendingVectors, roots: 3, items: 6 + 3 + 36 },
                MountedOwnerClaim { class: MountedOwnerClass::CsrVectors, roots: 5, items: 512 },
                MountedOwnerClaim { class: MountedOwnerClass::PcgVectors, roots: 10, items: 512 },
                MountedOwnerClaim {
                    class: MountedOwnerClass::VisualVectors,
                    roots: 5,
                    items: 2 * (SESSION_MAXIMUM_NODES + SESSION_MAXIMUM_MESH_POINTS + SESSION_MAXIMUM_ELEMENTS + SESSION_MAXIMUM_MESH_TRIANGLES)
                        + 64
                        + SESSION_MAXIMUM_ELEMENTS
                        + SESSION_MAXIMUM_MESH_TRIANGLES
                        + SESSION_MAXIMUM_NODES
                        + SESSION_MAXIMUM_MESH_POINTS,
                },
                MountedOwnerClaim { class: MountedOwnerClass::VisualStrings, roots: 64 + SESSION_MAXIMUM_NODES + SESSION_MAXIMUM_MESH_POINTS + SESSION_MAXIMUM_ELEMENTS + SESSION_MAXIMUM_MESH_TRIANGLES + 1, items: 0 },
                MountedOwnerClaim { class: MountedOwnerClass::OutputPages, roots: 3 * (SESSION_MAXIMUM_OUTPUT_BYTES / SESSION_OWNER_PAGE_BYTES), items: 6 },
                MountedOwnerClaim { class: MountedOwnerClass::FaultPages, roots: 2, items: 0 },
            ],
        }
    }

    const fn roots(self) -> usize {
        let mut roots = 0;
        let mut index = 0;
        while index < self.claims.len() {
            roots += self.claims[index].roots;
            index += 1;
        }
        roots
    }

    const fn items(self) -> usize {
        let mut items = 0;
        let mut index = 0;
        while index < self.claims.len() {
            items += self.claims[index].items;
            index += 1;
        }
        items
    }

    fn credit(self, input_items: usize, input_bytes: usize) -> Result<(usize, usize), &'static [u8]> {
        if input_items > SESSION_MAXIMUM_INPUT_ITEMS || input_bytes > SESSION_MAXIMUM_INPUT_BYTES {
            return Err(b"fem2d.session-process-input-credit-exceeded");
        }
        let pages = self.roots();
        let items = input_items.checked_add(self.items()).and_then(|items| items.checked_add(pages)).ok_or(b"fem2d.session-process-item-overflow" as &'static [u8])?;
        let bytes = input_bytes.checked_add(pages.checked_mul(SESSION_OWNER_PAGE_BYTES).ok_or(b"fem2d.session-process-page-byte-overflow" as &'static [u8])?).ok_or(b"fem2d.session-process-byte-overflow" as &'static [u8])?;
        if items > SESSION_MAXIMUM_ITEMS || bytes > SESSION_MAXIMUM_BYTES {
            return Err(b"fem2d.session-process-admission-exceeded");
        }
        Ok((items, bytes))
    }
}

const SESSION_MAXIMUM_ITEMS: usize = SESSION_MAXIMUM_INPUT_ITEMS + MountedProcessOwnerCatalog::fixed().items() + MountedProcessOwnerCatalog::fixed().roots();
const SESSION_MAXIMUM_BYTES: usize = SESSION_MAXIMUM_INPUT_BYTES + MountedProcessOwnerCatalog::fixed().roots() * SESSION_OWNER_PAGE_BYTES;

fn bounded_string_capacities(owners: &[&String]) -> Result<usize, &'static [u8]> {
    owners.iter().try_fold(0usize, |total, owner| {
        if owner.capacity() > SESSION_MAXIMUM_STRING_BYTES {
            return Err(b"fem2d.session-string-owner-capacity" as &'static [u8]);
        }
        total.checked_add(owner.capacity()).ok_or(b"fem2d.session-string-owner-overflow" as &'static [u8])
    })
}

fn bounded_derived_string(owner: &String) -> Result<(), Vec<u8>> {
    if owner.capacity() > SESSION_MAXIMUM_STRING_BYTES {
        return Err(b"fem2d.session-derived-string-capacity".to_vec());
    }
    Ok(())
}

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
    PrepareGraph,
    Graph,
    PrepareDomain,
    Mesh,
    BuildModel,
    PrepareAssembly,
    Assembly,
    BuildCsr,
    PreparePcg,
    Pcg,
    SyncPcgVisual,
    CommitReady,
    PublishFinalVisual,
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
    NodeCommit,
    ElementStart,
    ElementEnd,
    ElementMaterial,
    ElementSection,
    ElementIdOwner,
    ElementStartOwner,
    ElementEndOwner,
    ElementCommit,
    Supports,
    SupportReserveDofs,
    SupportDofs,
    SupportCommit,
    RegionNodeCoordinate,
    RegionNodeIdReserve,
    RegionNodeIdWrite,
    RegionNodeId,
    RegionNodeModelOwner,
    RegionNodeCommit,
    RegionElementMaterial,
    RegionElementIdReserve,
    RegionElementIdOwner,
    RegionElementNodeOwner,
    RegionElementCommit,
    Complete,
}

struct MountedModelBuild {
    stage: ModelBuildStage,
    node_cursor: usize,
    element_cursor: usize,
    support_cursor: usize,
    region_point_cursor: usize,
    region_triangle_cursor: usize,
    lookup_cursor: usize,
    resolved_material: usize,
    resolved_section: usize,
    pending_region_id: Option<String>,
    pending_node_id: Option<String>,
    pending_element_id: Option<String>,
    pending_element_start: Option<String>,
    pending_element_end: Option<String>,
    pending_support: Option<crate::model::Support>,
    pending_region_element_id: Option<String>,
    pending_region_element_nodes: [Option<String>; 3],
    pending_region_element_node_cursor: usize,
    pending_region_insert: bool,
    mesh: Option<TriMesh2>,
    region_node_ids: Vec<String>,
    model: AnalysisModel,
    close_lane: u8,
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
            lookup_cursor: 0,
            resolved_material: 0,
            resolved_section: 0,
            pending_region_id: None,
            pending_node_id: None,
            pending_element_id: None,
            pending_element_start: None,
            pending_element_end: None,
            pending_support: None,
            pending_region_element_id: None,
            pending_region_element_nodes: std::array::from_fn(|_| None),
            pending_region_element_node_cursor: 0,
            pending_region_insert: false,
            mesh,
            region_node_ids: Vec::new(),
            model: AnalysisModel { nodes: Vec::new(), elements: Vec::new(), supports: Vec::new() },
            close_lane: 0,
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
                if self.model.nodes.capacity() * std::mem::size_of::<crate::model::Node>() > SESSION_OWNER_PAGE_BYTES {
                    return Err(b"fem2d.model-node-page-capacity".to_vec());
                }
                self.stage = ModelBuildStage::ReserveElements;
            }
            ModelBuildStage::ReserveElements => {
                self.model.elements.try_reserve_exact(snapshot.elements.len() + mesh_triangles).map_err(|_| b"fem2d.model-element-allocation".to_vec())?;
                if self.model.elements.capacity() * std::mem::size_of::<crate::model::Elements>() > SESSION_OWNER_PAGE_BYTES {
                    return Err(b"fem2d.model-element-page-capacity".to_vec());
                }
                self.stage = ModelBuildStage::ReserveSupports;
            }
            ModelBuildStage::ReserveSupports => {
                self.model.supports.try_reserve_exact(snapshot.supports.len()).map_err(|_| b"fem2d.model-support-allocation".to_vec())?;
                if self.model.supports.capacity() * std::mem::size_of::<crate::model::Support>() > SESSION_OWNER_PAGE_BYTES {
                    return Err(b"fem2d.model-support-page-capacity".to_vec());
                }
                self.stage = ModelBuildStage::ReserveRegionNodeIds;
            }
            ModelBuildStage::ReserveRegionNodeIds => {
                self.region_node_ids.try_reserve_exact(mesh_points).map_err(|_| b"fem2d.model-region-node-allocation".to_vec())?;
                if self.region_node_ids.capacity() * std::mem::size_of::<String>() > SESSION_OWNER_PAGE_BYTES {
                    return Err(b"fem2d.model-region-node-page-capacity".to_vec());
                }
                self.stage = ModelBuildStage::Nodes;
            }
            ModelBuildStage::Nodes => {
                if let Some(node) = snapshot.nodes.get(self.node_cursor) {
                    self.pending_node_id = Some(node.id.clone());
                    self.stage = ModelBuildStage::NodeCommit;
                } else {
                    self.stage = ModelBuildStage::ElementStart;
                }
            }
            ModelBuildStage::NodeCommit => {
                let node = snapshot.nodes.get(self.node_cursor).ok_or_else(|| b"fem2d.model-node-owner-missing".to_vec())?;
                bounded_derived_string(self.pending_node_id.as_ref().ok_or_else(|| b"fem2d.model-node-id-owner-missing".to_vec())?)?;
                let id = self.pending_node_id.take().ok_or_else(|| b"fem2d.model-node-id-owner-missing".to_vec())?;
                self.model.nodes.push(crate::model::Node { id, pos: [node.x, node.y, 0.0] });
                self.node_cursor += 1;
                self.stage = ModelBuildStage::Nodes;
            }
            ModelBuildStage::ElementStart => {
                let Some(element) = snapshot.elements.get(self.element_cursor) else {
                    self.stage = ModelBuildStage::Supports;
                    return Ok(false);
                };
                let start = match element {
                    FemElement::Bar { start, .. } | FemElement::Beam { start, .. } => start,
                };
                if let Some(node) = snapshot.nodes.get(self.lookup_cursor) {
                    if node.id == *start {
                        self.lookup_cursor = 0;
                        self.stage = ModelBuildStage::ElementEnd;
                    } else {
                        self.lookup_cursor += 1;
                    }
                } else {
                    return Err(b"fem2d.model-unknown-start-node".to_vec());
                }
            }
            ModelBuildStage::ElementEnd => {
                let element = &snapshot.elements[self.element_cursor];
                let end = match element {
                    FemElement::Bar { end, .. } | FemElement::Beam { end, .. } => end,
                };
                if let Some(node) = snapshot.nodes.get(self.lookup_cursor) {
                    if node.id == *end {
                        self.lookup_cursor = 0;
                        self.stage = ModelBuildStage::ElementMaterial;
                    } else {
                        self.lookup_cursor += 1;
                    }
                } else {
                    return Err(b"fem2d.model-unknown-end-node".to_vec());
                }
            }
            ModelBuildStage::ElementMaterial => {
                let material_id = match &snapshot.elements[self.element_cursor] {
                    FemElement::Bar { material_id, .. } | FemElement::Beam { material_id, .. } => material_id,
                };
                if let Some(material) = snapshot.materials.get(self.lookup_cursor) {
                    if material.id == *material_id {
                        self.resolved_material = self.lookup_cursor;
                        self.lookup_cursor = 0;
                        self.stage = ModelBuildStage::ElementSection;
                    } else {
                        self.lookup_cursor += 1;
                    }
                } else {
                    return Err(b"fem2d.model-unknown-material".to_vec());
                }
            }
            ModelBuildStage::ElementSection => {
                let section_id = match &snapshot.elements[self.element_cursor] {
                    FemElement::Bar { section_id, .. } | FemElement::Beam { section_id, .. } => section_id,
                };
                if let Some(section) = snapshot.sections.get(self.lookup_cursor) {
                    if section.id == *section_id {
                        self.resolved_section = self.lookup_cursor;
                        self.lookup_cursor = 0;
                        self.stage = ModelBuildStage::ElementIdOwner;
                    } else {
                        self.lookup_cursor += 1;
                    }
                } else {
                    return Err(b"fem2d.model-unknown-section".to_vec());
                }
            }
            ModelBuildStage::ElementIdOwner => {
                let element = &snapshot.elements[self.element_cursor];
                self.pending_element_id = Some(match element {
                    FemElement::Bar { id, .. } | FemElement::Beam { id, .. } => id.clone(),
                });
                self.stage = ModelBuildStage::ElementStartOwner;
            }
            ModelBuildStage::ElementStartOwner => {
                bounded_derived_string(self.pending_element_id.as_ref().ok_or_else(|| b"fem2d.model-element-id-owner-missing".to_vec())?)?;
                let element = &snapshot.elements[self.element_cursor];
                self.pending_element_start = Some(match element {
                    FemElement::Bar { start, .. } | FemElement::Beam { start, .. } => start.clone(),
                });
                self.stage = ModelBuildStage::ElementEndOwner;
            }
            ModelBuildStage::ElementEndOwner => {
                bounded_derived_string(self.pending_element_start.as_ref().ok_or_else(|| b"fem2d.model-element-start-owner-missing".to_vec())?)?;
                let element = &snapshot.elements[self.element_cursor];
                self.pending_element_end = Some(match element {
                    FemElement::Bar { end, .. } | FemElement::Beam { end, .. } => end.clone(),
                });
                self.stage = ModelBuildStage::ElementCommit;
            }
            ModelBuildStage::ElementCommit => {
                bounded_derived_string(self.pending_element_end.as_ref().ok_or_else(|| b"fem2d.model-element-end-owner-missing".to_vec())?)?;
                let element = &snapshot.elements[self.element_cursor];
                let material = &snapshot.materials[self.resolved_material];
                let section = &snapshot.sections[self.resolved_section];
                let resolved = match element {
                    FemElement::Bar { .. } => crate::elements2d::Bar2 {
                        id: self.pending_element_id.take().ok_or_else(|| b"fem2d.model-element-id-owner-missing".to_vec())?,
                        start: self.pending_element_start.take().ok_or_else(|| b"fem2d.model-element-start-owner-missing".to_vec())?,
                        end: self.pending_element_end.take().ok_or_else(|| b"fem2d.model-element-end-owner-missing".to_vec())?,
                        e: material.e,
                        area: section.area,
                        density: material.rho,
                    }
                    .into(),
                    FemElement::Beam { .. } => crate::elements2d::BeamEb2 {
                        id: self.pending_element_id.take().ok_or_else(|| b"fem2d.model-element-id-owner-missing".to_vec())?,
                        start: self.pending_element_start.take().ok_or_else(|| b"fem2d.model-element-start-owner-missing".to_vec())?,
                        end: self.pending_element_end.take().ok_or_else(|| b"fem2d.model-element-end-owner-missing".to_vec())?,
                        e: material.e,
                        area: section.area,
                        iy: section.iy,
                        density: material.rho,
                    }
                    .into(),
                };
                self.model.elements.push(resolved);
                self.element_cursor += 1;
                self.stage = ModelBuildStage::ElementStart;
            }
            ModelBuildStage::Supports => {
                if let Some(support) = snapshot.supports.get(self.support_cursor) {
                    self.pending_support = Some(crate::model::Support { node_id: support.node_id.clone(), fixed: Vec::new() });
                    self.stage = ModelBuildStage::SupportReserveDofs;
                } else {
                    self.stage = ModelBuildStage::RegionNodeCoordinate;
                }
            }
            ModelBuildStage::SupportReserveDofs => {
                let source = snapshot.supports.get(self.support_cursor).ok_or_else(|| b"fem2d.model-support-owner-missing".to_vec())?;
                let owner = self.pending_support.as_mut().ok_or_else(|| b"fem2d.model-support-candidate-missing".to_vec())?;
                bounded_derived_string(&owner.node_id)?;
                owner.fixed.try_reserve_exact(source.fixed.len()).map_err(|_| b"fem2d.model-support-dof-allocation".to_vec())?;
                if owner.fixed.capacity() * std::mem::size_of::<crate::model::Dof>() > SESSION_OWNER_PAGE_BYTES {
                    return Err(b"fem2d.model-support-dof-page-capacity".to_vec());
                }
                self.lookup_cursor = 0;
                self.stage = ModelBuildStage::SupportDofs;
            }
            ModelBuildStage::SupportDofs => {
                let source = snapshot.supports.get(self.support_cursor).ok_or_else(|| b"fem2d.model-support-owner-missing".to_vec())?;
                if let Some(dof) = source.fixed.get(self.lookup_cursor).copied() {
                    self.pending_support.as_mut().ok_or_else(|| b"fem2d.model-support-candidate-missing".to_vec())?.fixed.push(dof.into());
                    self.lookup_cursor += 1;
                } else {
                    self.lookup_cursor = 0;
                    self.stage = ModelBuildStage::SupportCommit;
                }
            }
            ModelBuildStage::SupportCommit => {
                self.model.supports.push(self.pending_support.take().ok_or_else(|| b"fem2d.model-support-candidate-missing".to_vec())?);
                self.support_cursor += 1;
                self.stage = ModelBuildStage::Supports;
            }
            ModelBuildStage::RegionNodeCoordinate => {
                let Some(region) = snapshot.regions.first() else {
                    self.stage = ModelBuildStage::Complete;
                    return Ok(false);
                };
                let Some(point) = self.mesh.as_ref().and_then(|mesh| mesh.points.get(self.region_point_cursor)).copied() else {
                    self.stage = ModelBuildStage::RegionElementMaterial;
                    return Ok(false);
                };
                if let Some(node) = snapshot.nodes.get(self.lookup_cursor) {
                    if (node.x - point[0]).abs() < 1e-9 && (node.y - point[1]).abs() < 1e-9 {
                        self.pending_region_id = Some(node.id.clone());
                        self.pending_region_insert = false;
                        self.lookup_cursor = 0;
                        self.stage = ModelBuildStage::RegionNodeCommit;
                    } else {
                        self.lookup_cursor += 1;
                    }
                } else {
                    self.stage = ModelBuildStage::RegionNodeIdReserve;
                }
            }
            ModelBuildStage::RegionNodeIdReserve => {
                let region = snapshot.regions.first().ok_or_else(|| b"fem2d.model-region-owner-missing".to_vec())?;
                let required = region.id.len().checked_add(24).ok_or_else(|| b"fem2d.model-generated-node-id-overflow".to_vec())?;
                if required > SESSION_MAXIMUM_STRING_BYTES {
                    return Err(b"fem2d.model-generated-node-id-capacity".to_vec());
                }
                let mut id = String::new();
                id.try_reserve_exact(required).map_err(|_| b"fem2d.model-generated-node-id-allocation".to_vec())?;
                if id.capacity() > SESSION_OWNER_PAGE_BYTES {
                    return Err(b"fem2d.model-generated-node-id-page-capacity".to_vec());
                }
                self.pending_region_id = Some(id);
                self.stage = ModelBuildStage::RegionNodeIdWrite;
            }
            ModelBuildStage::RegionNodeIdWrite => {
                let region = snapshot.regions.first().ok_or_else(|| b"fem2d.model-region-owner-missing".to_vec())?;
                let id = self.pending_region_id.as_mut().ok_or_else(|| b"fem2d.model-generated-node-id-owner-missing".to_vec())?;
                use std::fmt::Write as _;
                write!(id, "{}_m{}", region.id, self.region_point_cursor).map_err(|_| b"fem2d.model-generated-node-id-format".to_vec())?;
                self.pending_region_insert = true;
                self.lookup_cursor = 0;
                self.stage = ModelBuildStage::RegionNodeId;
            }
            ModelBuildStage::RegionNodeId => {
                let id = self.pending_region_id.as_ref().expect("region node id retained");
                if let Some(node) = self.model.nodes.get(self.lookup_cursor) {
                    if node.id == *id {
                        return Err(b"fem2d.model-generated-node-id-collision".to_vec());
                    }
                    self.lookup_cursor += 1;
                } else {
                    self.lookup_cursor = 0;
                    self.stage = ModelBuildStage::RegionNodeCommit;
                }
            }
            ModelBuildStage::RegionNodeModelOwner => {
                let point = self.mesh.as_ref().and_then(|mesh| mesh.points.get(self.region_point_cursor)).copied().ok_or_else(|| b"fem2d.model-region-point-missing".to_vec())?;
                let id = self.pending_region_id.as_ref().ok_or_else(|| b"fem2d.model-region-node-owner-missing".to_vec())?.clone();
                self.model.nodes.push(crate::model::Node { id, pos: [point[0], point[1], 0.0] });
                self.pending_region_insert = false;
                self.stage = ModelBuildStage::RegionNodeCommit;
            }
            ModelBuildStage::RegionNodeCommit => {
                bounded_derived_string(self.pending_region_id.as_ref().ok_or_else(|| b"fem2d.model-region-node-owner-missing".to_vec())?)?;
                let id = self.pending_region_id.take().expect("region node id retained");
                if self.pending_region_insert {
                    self.pending_region_id = Some(id);
                    self.stage = ModelBuildStage::RegionNodeModelOwner;
                    return Ok(false);
                }
                self.region_node_ids.push(id);
                self.pending_region_insert = false;
                self.region_point_cursor += 1;
                self.stage = ModelBuildStage::RegionNodeCoordinate;
            }
            ModelBuildStage::RegionElementMaterial => {
                let Some(region) = snapshot.regions.first() else {
                    self.stage = ModelBuildStage::Complete;
                    return Ok(false);
                };
                if self.mesh.as_ref().and_then(|mesh| mesh.tris.get(self.region_triangle_cursor)).is_none() {
                    self.stage = ModelBuildStage::Complete;
                } else if let Some(material) = snapshot.materials.get(self.lookup_cursor) {
                    if material.id == region.material_id {
                        self.resolved_material = self.lookup_cursor;
                        self.lookup_cursor = 0;
                        self.stage = ModelBuildStage::RegionElementIdReserve;
                    } else {
                        self.lookup_cursor += 1;
                    }
                } else {
                    return Err(b"fem2d.model-unknown-region-material".to_vec());
                }
            }
            ModelBuildStage::RegionElementIdReserve => {
                let region = snapshot.regions.first().ok_or_else(|| b"fem2d.model-region-owner-missing".to_vec())?;
                let required = region.id.len().checked_add(24).ok_or_else(|| b"fem2d.model-generated-element-id-overflow".to_vec())?;
                if required > SESSION_MAXIMUM_STRING_BYTES {
                    return Err(b"fem2d.model-generated-element-id-capacity".to_vec());
                }
                let mut id = String::new();
                id.try_reserve_exact(required).map_err(|_| b"fem2d.model-generated-element-id-allocation".to_vec())?;
                if id.capacity() > SESSION_OWNER_PAGE_BYTES {
                    return Err(b"fem2d.model-generated-element-id-page-capacity".to_vec());
                }
                self.pending_region_element_id = Some(id);
                self.stage = ModelBuildStage::RegionElementIdOwner;
            }
            ModelBuildStage::RegionElementIdOwner => {
                let region = snapshot.regions.first().ok_or_else(|| b"fem2d.model-region-owner-missing".to_vec())?;
                let id = self.pending_region_element_id.as_mut().ok_or_else(|| b"fem2d.model-generated-element-id-owner-missing".to_vec())?;
                use std::fmt::Write as _;
                write!(id, "{}_t{}", region.id, self.region_triangle_cursor).map_err(|_| b"fem2d.model-generated-element-id-format".to_vec())?;
                self.pending_region_element_node_cursor = 0;
                self.stage = ModelBuildStage::RegionElementNodeOwner;
            }
            ModelBuildStage::RegionElementNodeOwner => {
                bounded_derived_string(self.pending_region_element_id.as_ref().ok_or_else(|| b"fem2d.model-region-element-id-owner-missing".to_vec())?)?;
                if self.pending_region_element_node_cursor != 0 {
                    bounded_derived_string(self.pending_region_element_nodes[self.pending_region_element_node_cursor - 1].as_ref().ok_or_else(|| b"fem2d.model-region-element-node-owner-missing".to_vec())?)?;
                }
                let triangle = self.mesh.as_ref().and_then(|mesh| mesh.tris.get(self.region_triangle_cursor)).copied().ok_or_else(|| b"fem2d.model-region-triangle-missing".to_vec())?;
                if self.pending_region_element_node_cursor < 3 {
                    let source = triangle[self.pending_region_element_node_cursor] as usize;
                    self.pending_region_element_nodes[self.pending_region_element_node_cursor] = Some(self.region_node_ids.get(source).ok_or_else(|| b"fem2d.model-region-node-index".to_vec())?.clone());
                    self.pending_region_element_node_cursor += 1;
                } else {
                    self.stage = ModelBuildStage::RegionElementCommit;
                }
            }
            ModelBuildStage::RegionElementCommit => {
                for owner in &self.pending_region_element_nodes {
                    bounded_derived_string(owner.as_ref().ok_or_else(|| b"fem2d.model-region-element-node-owner-missing".to_vec())?)?;
                }
                let region = snapshot.regions.first().expect("region element retains region");
                let material = &snapshot.materials[self.resolved_material];
                let nodes = std::array::from_fn(|index| self.pending_region_element_nodes[index].take().expect("region element node owner retained"));
                self.model.elements.push(
                    crate::elements2d::Tri3Cst {
                        id: self.pending_region_element_id.take().ok_or_else(|| b"fem2d.model-region-element-id-owner-missing".to_vec())?,
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
                self.stage = ModelBuildStage::RegionElementMaterial;
            }
            ModelBuildStage::Complete => return Ok(true),
        }
        Ok(false)
    }

    fn take_complete(&mut self) -> Option<AnalysisModel> {
        (self.stage == ModelBuildStage::Complete).then(|| std::mem::replace(&mut self.model, AnalysisModel { nodes: Vec::new(), elements: Vec::new(), supports: Vec::new() }))
    }

    fn close_step(&mut self, maximum_bytes: usize) -> (bool, usize, usize) {
        for owner in [&mut self.pending_node_id, &mut self.pending_element_id, &mut self.pending_element_start, &mut self.pending_element_end, &mut self.pending_region_id, &mut self.pending_region_element_id] {
            if let Some(value) = owner.as_mut() {
                let bytes = value.capacity();
                if bytes > maximum_bytes {
                    return (false, 0, 0);
                }
                *value = String::new();
                *owner = None;
                return (false, 1, bytes);
            }
        }
        for owner in &mut self.pending_region_element_nodes {
            if let Some(value) = owner.as_mut() {
                let bytes = value.capacity();
                if bytes > maximum_bytes {
                    return (false, 0, 0);
                }
                *value = String::new();
                *owner = None;
                return (false, 1, bytes);
            }
        }
        if let Some(support) = self.pending_support.as_mut() {
            if support.node_id.capacity() != 0 {
                let bytes = support.node_id.capacity();
                if bytes > maximum_bytes {
                    return (false, 0, 0);
                }
                support.node_id = String::new();
                return (false, 1, bytes);
            }
            if support.fixed.pop().is_some() {
                return (false, 1, 0);
            }
            let bytes = support.fixed.capacity() * std::mem::size_of::<crate::model::Dof>();
            if bytes != 0 {
                if bytes > maximum_bytes {
                    return (false, 0, 0);
                }
                support.fixed = Vec::new();
                return (false, 1, bytes);
            }
            self.pending_support = None;
            return (false, 1, 0);
        }
        loop {
            let released_bytes = match self.close_lane {
                0 => {
                    let Some(node) = self.model.nodes.last_mut() else {
                        self.close_lane += 1;
                        continue;
                    };
                    if node.id.capacity() != 0 {
                        let bytes = node.id.capacity();
                        if bytes > maximum_bytes {
                            return (false, 0, 0);
                        }
                        node.id = String::new();
                        return (false, 1, bytes);
                    }
                    self.model.nodes.pop();
                    return (false, 1, 0);
                }
                1 => {
                    let bytes = self.model.nodes.capacity() * std::mem::size_of::<crate::model::Node>();
                    if bytes > maximum_bytes {
                        return (false, 0, 0);
                    }
                    self.model.nodes = Vec::new();
                    self.close_lane += 1;
                    bytes
                }
                2 => {
                    let Some(element) = self.model.elements.last_mut() else {
                        self.close_lane += 1;
                        continue;
                    };
                    if let Some(next_bytes) = element.mounted_next_string_bytes() {
                        if next_bytes > maximum_bytes {
                            return (false, 0, 0);
                        }
                        let bytes = element.close_mounted_string_step().expect("mounted element next-owner witness changed without mutation");
                        return (false, 1, bytes);
                    }
                    if !element.mounted_strings_terminal_is_empty() {
                        return (false, 0, 0);
                    }
                    self.model.elements.pop();
                    return (false, 1, 0);
                }
                3 => {
                    let bytes = self.model.elements.capacity() * std::mem::size_of::<crate::model::Elements>();
                    if bytes > maximum_bytes {
                        return (false, 0, 0);
                    }
                    self.model.elements = Vec::new();
                    self.close_lane += 1;
                    bytes
                }
                4 => {
                    let Some(support) = self.model.supports.last_mut() else {
                        self.close_lane += 1;
                        continue;
                    };
                    if support.node_id.capacity() != 0 {
                        let bytes = support.node_id.capacity();
                        if bytes > maximum_bytes {
                            return (false, 0, 0);
                        }
                        support.node_id = String::new();
                        return (false, 1, bytes);
                    }
                    if support.fixed.pop().is_some() {
                        return (false, 1, 0);
                    }
                    let bytes = support.fixed.capacity() * std::mem::size_of::<crate::model::Dof>();
                    if bytes != 0 {
                        if bytes > maximum_bytes {
                            return (false, 0, 0);
                        }
                        support.fixed = Vec::new();
                        return (false, 1, bytes);
                    }
                    self.model.supports.pop();
                    return (false, 1, 0);
                }
                5 => {
                    let bytes = self.model.supports.capacity() * std::mem::size_of::<crate::model::Support>();
                    if bytes > maximum_bytes {
                        return (false, 0, 0);
                    }
                    self.model.supports = Vec::new();
                    self.close_lane += 1;
                    bytes
                }
                6 => {
                    self.close_lane += 1;
                    continue;
                }
                7 => {
                    let Some(owner) = self.region_node_ids.last_mut() else {
                        self.close_lane += 1;
                        continue;
                    };
                    if owner.capacity() != 0 {
                        let bytes = owner.capacity();
                        if bytes > maximum_bytes {
                            return (false, 0, 0);
                        }
                        *owner = String::new();
                        return (false, 1, bytes);
                    }
                    self.region_node_ids.pop();
                    return (false, 1, 0);
                }
                8 => {
                    let bytes = self.region_node_ids.capacity() * std::mem::size_of::<String>();
                    if bytes > maximum_bytes {
                        return (false, 0, 0);
                    }
                    self.region_node_ids = Vec::new();
                    self.close_lane += 1;
                    bytes
                }
                9 => {
                    let Some(mesh) = self.mesh.as_mut() else {
                        self.close_lane += 2;
                        continue;
                    };
                    if mesh.points.pop().is_some() {
                        return (false, 1, 0);
                    }
                    let bytes = mesh.points.capacity() * std::mem::size_of::<[f64; 2]>();
                    if bytes != 0 {
                        if bytes > maximum_bytes {
                            return (false, 0, 0);
                        }
                        mesh.points = Vec::new();
                        return (false, 1, bytes);
                    }
                    self.close_lane += 1;
                    continue;
                }
                10 => {
                    let Some(mesh) = self.mesh.as_mut() else {
                        self.close_lane += 1;
                        continue;
                    };
                    if mesh.tris.pop().is_some() {
                        return (false, 1, 0);
                    }
                    let bytes = mesh.tris.capacity() * std::mem::size_of::<[u32; 3]>();
                    if bytes != 0 {
                        if bytes > maximum_bytes {
                            return (false, 0, 0);
                        }
                        mesh.tris = Vec::new();
                        return (false, 1, bytes);
                    }
                    self.mesh = None;
                    self.close_lane += 1;
                    0
                }
                _ => return (true, 0, 0),
            };
            return (false, 1, released_bytes);
        }
    }
}
//#endregion 🔖️Contract

//#region 🧰️FixedArena
struct MountedState {
    identity: MountedIdentity,
    snapshot: Option<store::SnapshotRead<Fem2dSnapshot>>,
    snapshot_return: Option<store::SnapshotReadReturn>,
    cancel: semio_framework_job::CancelToken,
    stage: MountedStage,
    admitted_items: usize,
    admitted_bytes: usize,
    graph_plans: Vec<FemStagePlan>,
    graph: Option<FemJobGraph>,
    domain: Option<PlanarDomain>,
    domain_outer_cursor: usize,
    domain_hole_cursor: usize,
    domain_hole_point_cursor: usize,
    domain_close_lane: u8,
    mesh: Option<MeshJob>,
    model_build: Option<MountedModelBuild>,
    assembly_build: Option<AssemblyJobConstruction>,
    assembly: Option<AssemblyJob<'static>>,
    csr_build: Option<AssemblyCsrBuild>,
    pcg_build: Option<PcgJobConstruction>,
    pcg: Option<PcgJob>,
    visual: Fem2dLiveVisual,
    visual_region_owner: Option<(String, RegionVisualQuality)>,
    visual_job_candidate: Option<Fem2dVisualJob>,
    visual_rejected: Option<Fem2dVisualJob>,
    visual_current: Option<Fem2dMountedVisualLease>,
    visual_displaced: Option<Fem2dMountedVisualLease>,
    visual_dirty: bool,
    visual_field_cursor: usize,
    visual_pcg_complete: bool,
    preview_sequence: u64,
    close_cursor: u8,
    fault: Option<Vec<u8>>,
}

fn close_retained_payload(payload: &mut RetainedJobPayload) {
    while !payload.terminal_is_empty() {
        let _ = payload.close_step(1, usize::MAX);
    }
}

fn take_retained_payload(mut payload: RetainedJobPayload, maximum_bytes: usize) -> Option<Vec<u8>> {
    if payload.len() > maximum_bytes {
        close_retained_payload(&mut payload);
        return None;
    }
    let mut bytes = Vec::with_capacity(payload.len());
    for page in 0..payload.page_count() {
        bytes.extend_from_slice(payload.page(page)?);
    }
    close_retained_payload(&mut payload);
    Some(bytes)
}

fn retained_payload_byte(payload: &RetainedJobPayload, index: usize) -> Option<u8> {
    let mut offset = index;
    for page in 0..payload.page_count() {
        let bytes = payload.page(page)?;
        if offset < bytes.len() {
            return bytes.get(offset).copied();
        }
        offset -= bytes.len();
    }
    None
}

impl MountedState {
    fn new(identity: MountedIdentity, snapshot: store::SnapshotRead<Fem2dSnapshot>, admitted_items: usize, admitted_bytes: usize) -> Self {
        Self {
            identity,
            snapshot: Some(snapshot),
            snapshot_return: None,
            cancel: semio_framework_job::root_cancel_token(),
            stage: MountedStage::PrepareGraph,
            admitted_items,
            admitted_bytes,
            graph_plans: Vec::new(),
            graph: None,
            domain: None,
            domain_outer_cursor: 0,
            domain_hole_cursor: 0,
            domain_hole_point_cursor: 0,
            domain_close_lane: 0,
            mesh: None,
            model_build: None,
            assembly_build: None,
            assembly: None,
            csr_build: None,
            pcg_build: None,
            pcg: None,
            visual: Fem2dLiveVisual::default(),
            visual_region_owner: None,
            visual_job_candidate: None,
            visual_rejected: None,
            visual_current: None,
            visual_displaced: None,
            visual_dirty: true,
            visual_field_cursor: 0,
            visual_pcg_complete: false,
            preview_sequence: 0,
            close_cursor: 0,
            fault: None,
        }
    }

    fn fail(&mut self, detail: impl Into<Vec<u8>>) -> JobStep {
        let mut detail = detail.into();
        if detail.capacity() > SESSION_MAXIMUM_FAULT_BYTES {
            detail = b"fem2d.session-fault-capacity".to_vec();
        }
        self.fault = Some(detail.clone());
        self.visual.state = FemVisualState::FaultedCancelled;
        self.stage = MountedStage::Fault;
        JobStep::Failed(detail)
    }

    fn progress(&self, _label: &'static [u8]) -> JobStep {
        JobStep::Running(None)
    }

    fn prepare_domain_one(&mut self) -> Result<bool, &'static [u8]> {
        let snapshot = self.snapshot.as_ref().ok_or(b"fem2d.session-snapshot-missing" as &'static [u8])?;
        let Some(region) = snapshot.regions.first() else { return Ok(true) };
        if self.domain.is_none() {
            let mut outer = Vec::new();
            outer.try_reserve_exact(region.outline.len()).map_err(|_| b"fem2d.session-domain-outer-allocation" as &'static [u8])?;
            let mut holes = Vec::new();
            holes.try_reserve_exact(region.holes.len()).map_err(|_| b"fem2d.session-domain-holes-allocation" as &'static [u8])?;
            if outer.capacity() * std::mem::size_of::<[f64; 2]>() > SESSION_OWNER_PAGE_BYTES || holes.capacity() * std::mem::size_of::<Vec<[f64; 2]>>() > SESSION_OWNER_PAGE_BYTES {
                return Err(b"fem2d.session-domain-page-capacity");
            }
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
                hole.try_reserve_exact(region.holes[self.domain_hole_cursor].len()).map_err(|_| b"fem2d.session-domain-hole-allocation" as &'static [u8])?;
                if hole.capacity() * std::mem::size_of::<[f64; 2]>() > SESSION_OWNER_PAGE_BYTES {
                    return Err(b"fem2d.session-domain-hole-page-capacity");
                }
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

    fn publish_visual_candidate(&mut self, candidate: Fem2dMountedVisualLease) -> Result<(), Fem2dMountedVisualLease> {
        if self.visual_displaced.is_some() {
            return Err(candidate);
        }
        self.visual_displaced = self.visual_current.take();
        self.visual_current = Some(candidate);
        Ok(())
    }

    fn visual_freshness(&self) -> Fem2dVisualFreshness {
        Fem2dVisualFreshness {
            app_instance_id: self.identity.app_instance_id,
            model_revision: self.identity.base_revision.0,
            document_generation: self.identity.generation.0,
            operation: self.identity.operation.0,
            numerical_preview_sequence: self.preview_sequence,
            surface_generation: self.identity.generation.0,
            renderer_scene_generation: self.identity.generation.0,
        }
    }

    fn drive_visual_one(&mut self, cx: &mut StepContext<'_>) -> Option<JobStep> {
        if cx.should_yield() {
            return Some(JobStep::Running(None));
        }
        if let Some(rejected) = self.visual_rejected.as_mut() {
            cx.consume_fuel(1);
            let (terminal, _, _) = rejected.close_step(SESSION_OWNER_PAGE_BYTES);
            if terminal {
                self.visual_rejected = None;
            }
            return Some(self.progress(b"fem2d.visual-rejected-close"));
        }
        if let Some(displaced) = self.visual_displaced.as_mut() {
            cx.consume_fuel(1);
            let (terminal, _, _) = displaced.close_step(SESSION_OWNER_PAGE_BYTES);
            if terminal {
                self.visual_displaced = None;
            }
            return Some(self.progress(b"fem2d.visual-displaced-close"));
        }
        if self.visual_dirty && self.visual_job_candidate.is_some() {
            cx.consume_fuel(1);
            self.visual_rejected = self.visual_job_candidate.take();
            return Some(self.progress(b"fem2d.visual-candidate-displaced"));
        }
        if self.visual_dirty && self.visual_job_candidate.is_none() {
            cx.consume_fuel(1);
            self.visual_job_candidate = Some(Fem2dVisualJob::new(self.visual_freshness()));
            self.visual_dirty = false;
            return Some(self.progress(b"fem2d.visual-candidate-open"));
        }
        let freshness = self.visual_freshness();
        let candidate = self.visual_job_candidate.as_mut()?;
        let Some(snapshot) = self.snapshot.as_ref() else { return Some(self.fail(b"fem2d.visual-snapshot-owner".to_vec())) };
        cx.consume_fuel(1);
        match candidate.step_one(snapshot, &self.visual, freshness) {
            Ok(false) => Some(self.progress(b"fem2d.visual-candidate")),
            Ok(true) => {
                let Some(lease) = candidate.take_complete() else { return Some(self.fail(b"fem2d.visual-complete-owner".to_vec())) };
                let live = current_identity(self.identity.app_instance_id) == Some(self.identity) && snapshot.commit_authority_matches(self.identity.generation.0, self.identity.canonical_base_revision) && !self.cancel.is_cancelled_now();
                if !live {
                    self.visual_displaced = Some(lease);
                    self.visual_job_candidate = None;
                    return Some(self.progress(b"fem2d.visual-stale-close"));
                }
                if !lease.matches_freshness(freshness) {
                    self.visual_displaced = Some(lease);
                    self.visual_job_candidate = None;
                    return Some(self.progress(b"fem2d.visual-generation-stale-close"));
                }
                if let Err(exact_candidate) = self.publish_visual_candidate(lease) {
                    self.visual_displaced = Some(exact_candidate);
                    return Some(self.progress(b"fem2d.visual-publication-full"));
                }
                self.visual_job_candidate = None;
                Some(self.progress(b"fem2d.visual-published"))
            }
            Err(detail) => Some(self.fail(detail)),
        }
    }

    fn step(&mut self, budget: JobBudget) -> JobStep {
        if self.cancel.is_cancelled_now() {
            self.stage = MountedStage::Closing;
            return self.fail(b"fem2d.session-cancelled".to_vec());
        }
        if budget.fuel == 0 || budget.deadline_ms == 0 {
            return JobStep::Running(None);
        }
        let Some(now) = semio_framework_job::default_now_us() else { return JobStep::Running(None) };
        let deadline = now.saturating_add(u64::from(budget.deadline_ms).min(8));
        let mut preview_sequence = self.preview_sequence;
        let result = (|| {
            let mut cx = StepContext::new(self.identity.operation, self.identity.generation, StepBudget::new(budget.fuel.max(1), deadline), self.cancel.clone(), semio_framework_job::default_now_us, &mut preview_sequence);
            if cx.should_yield() {
                return JobStep::Running(None);
            }
            if let Some(step) = self.drive_visual_one(&mut cx) {
                return step;
            }
            match self.stage {
                MountedStage::PrepareGraph => {
                    const PLANS: [FemStagePlan; 8] = [
                        FemStagePlan { stage: FemJobStage::ValidateReferences, units: 1 },
                        FemStagePlan { stage: FemJobStage::BuildDofMap, units: 1 },
                        FemStagePlan { stage: FemJobStage::OrderEquations, units: 1 },
                        FemStagePlan { stage: FemJobStage::Assemble, units: 1 },
                        FemStagePlan { stage: FemJobStage::Factor, units: 1 },
                        FemStagePlan { stage: FemJobStage::Solve, units: 1 },
                        FemStagePlan { stage: FemJobStage::Recover, units: 1 },
                        FemStagePlan { stage: FemJobStage::Finalize, units: 1 },
                    ];
                    if self.graph_plans.capacity() == 0 {
                        if self.graph_plans.try_reserve_exact(PLANS.len()).is_err() || self.graph_plans.capacity() * std::mem::size_of::<FemStagePlan>() > SESSION_OWNER_PAGE_BYTES {
                            return self.fail(b"fem2d.graph-plan-allocation".to_vec());
                        }
                    } else if let Some(plan) = PLANS.get(self.graph_plans.len()).cloned() {
                        self.graph_plans.push(plan);
                    } else {
                        self.graph = Some(FemJobGraph::new(self.identity.operation(), std::mem::take(&mut self.graph_plans), 1));
                        self.stage = MountedStage::Graph;
                    }
                    cx.consume_fuel(1);
                    self.progress(b"fem2d.graph-admitted")
                }
                MountedStage::Graph => match self.graph.as_mut().expect("graph stage owns graph").step(&mut cx) {
                    StepOutcome::Complete(candidate) => {
                        self.stage = MountedStage::PrepareDomain;
                        let semio_framework_job::CommitCandidate { mut state, output } = candidate;
                        close_retained_payload(&mut state);
                        match take_retained_payload(output, SESSION_MAXIMUM_OUTPUT_BYTES) {
                            Some(bytes) => JobStep::Running(Some(bytes)),
                            None => self.fail(b"fem2d.graph-output-capacity".to_vec()),
                        }
                    }
                    StepOutcome::PreviewReady(payload) | StepOutcome::CheckpointReady(semio_framework_job::Checkpoint { state: payload, .. }) => match take_retained_payload(payload, SESSION_MAXIMUM_OUTPUT_BYTES) {
                        Some(bytes) => JobStep::Running(Some(bytes)),
                        None => self.fail(b"fem2d.graph-output-capacity".to_vec()),
                    },
                    StepOutcome::Yield => self.progress(b"fem2d.graph-yield"),
                    StepOutcome::Cancelled => self.fail(b"fem2d.graph-cancelled".to_vec()),
                    StepOutcome::Fault(fault) => self.fail(take_retained_payload(fault.detail, SESSION_MAXIMUM_FAULT_BYTES).unwrap_or_else(|| b"fem2d.graph-fault-capacity".to_vec())),
                },
                MountedStage::PrepareDomain => match self.prepare_domain_one() {
                    Ok(false) => {
                        cx.consume_fuel(1);
                        self.progress(b"fem2d.domain")
                    }
                    Ok(true) => {
                        let snapshot = self.snapshot.as_ref().expect("preflight retains snapshot");
                        if let (Some(region), Some(domain)) = (snapshot.regions.first(), self.domain.take()) {
                            if self.visual.region_quality.insert(region.id.clone(), RegionVisualQuality::Unmeshed).is_err() {
                                return self.fail(b"fem2d.visual-region-capacity".to_vec());
                            }
                            self.visual.state = FemVisualState::Unmeshed;
                            self.visual_dirty = true;
                            self.mesh = Some(MeshJob::new_bounded(domain, MeshOpts { max_edge: region.mesh_size, min_angle_deg: 20.0 }, self.identity.operation(), SESSION_MAXIMUM_MESH_POINTS, SESSION_MAXIMUM_MESH_TRIANGLES));
                            self.stage = MountedStage::Mesh;
                        } else {
                            self.stage = MountedStage::BuildModel;
                        }
                        cx.consume_fuel(1);
                        self.progress(b"fem2d.domain-complete")
                    }
                    Err(detail) => self.fail(detail.to_vec()),
                },
                MountedStage::Mesh => match self.mesh.as_mut().expect("mesh stage owns mesh").step(&mut cx) {
                    StepOutcome::Complete(candidate) => {
                        if let Some(region) = self.snapshot.as_ref().and_then(|snapshot| snapshot.regions.first()) {
                            if !self.visual.region_quality.update(&region.id, RegionVisualQuality::Final) {
                                return self.fail(b"fem2d.visual-region-capacity".to_vec());
                            }
                            self.visual.state = FemVisualState::Refined;
                            self.visual_dirty = true;
                        }
                        self.model_build = Some(MountedModelBuild::new(self.mesh.as_mut().and_then(MeshJob::take_completed_mesh)));
                        self.stage = MountedStage::BuildModel;
                        let semio_framework_job::CommitCandidate { mut state, output } = candidate;
                        close_retained_payload(&mut state);
                        match take_retained_payload(output, SESSION_MAXIMUM_OUTPUT_BYTES) {
                            Some(bytes) => JobStep::Running(Some(bytes)),
                            None => self.progress(b"fem2d.mesh-complete"),
                        }
                    }
                    StepOutcome::PreviewReady(payload) => {
                        if let Some(region) = self.snapshot.as_ref().and_then(|snapshot| snapshot.regions.first()) {
                            let quality = match retained_payload_byte(&payload, 8) {
                                Some(0) => RegionVisualQuality::Coarse,
                                Some(1) => RegionVisualQuality::Refined,
                                Some(2) => RegionVisualQuality::Final,
                                _ => RegionVisualQuality::Unmeshed,
                            };
                            if !self.visual.region_quality.update(&region.id, quality) {
                                return self.fail(b"fem2d.visual-region-capacity".to_vec());
                            }
                            self.visual.state = match quality {
                                RegionVisualQuality::Unmeshed => FemVisualState::Unmeshed,
                                RegionVisualQuality::Coarse => FemVisualState::Coarse,
                                RegionVisualQuality::Refined | RegionVisualQuality::Final => FemVisualState::Refined,
                            };
                            self.visual_dirty = true;
                        }
                        match take_retained_payload(payload, SESSION_MAXIMUM_OUTPUT_BYTES) {
                            Some(bytes) => JobStep::Running(Some(bytes)),
                            None => self.progress(b"fem2d.mesh-preview"),
                        }
                    }
                    StepOutcome::CheckpointReady(checkpoint) => match take_retained_payload(checkpoint.state, SESSION_MAXIMUM_OUTPUT_BYTES) {
                        Some(bytes) => JobStep::Running(Some(bytes)),
                        None => self.progress(b"fem2d.mesh-yield"),
                    },
                    StepOutcome::Yield => self.progress(b"fem2d.mesh-yield"),
                    StepOutcome::Cancelled => self.fail(b"fem2d.mesh-cancelled".to_vec()),
                    StepOutcome::Fault(fault) => self.fail(take_retained_payload(fault.detail, SESSION_MAXIMUM_FAULT_BYTES).unwrap_or_else(|| b"fem2d.mesh-fault-capacity".to_vec())),
                },
                MountedStage::BuildModel => {
                    let snapshot = self.snapshot.as_ref().expect("preflight retains snapshot");
                    if self.model_build.is_none() {
                        self.model_build = Some(MountedModelBuild::new(None));
                        cx.consume_fuel(1);
                        return self.progress(b"fem2d.model-admitted");
                    }
                    match self.model_build.as_mut().expect("model builder admitted above").step_one(snapshot) {
                        Ok(false) => {
                            cx.consume_fuel(1);
                            self.progress(b"fem2d.model-building")
                        }
                        Ok(true) => {
                            let model = std::sync::Arc::new(self.model_build.as_mut().and_then(MountedModelBuild::take_complete).expect("complete model transfers exactly once"));
                            self.assembly_build = Some(AssemblyJobConstruction::new_owned(model, self.identity.operation(), 1));
                            self.stage = MountedStage::PrepareAssembly;
                            self.visual.state = FemVisualState::Assembling;
                            cx.consume_fuel(1);
                            self.progress(b"fem2d.model-built")
                        }
                        Err(error) => self.fail(error),
                    }
                }
                MountedStage::PrepareAssembly => match self.assembly_build.as_mut().expect("assembly construction retained").step_one() {
                    Ok(false) => {
                        cx.consume_fuel(1);
                        self.progress(b"fem2d.assembly-preparing")
                    }
                    Ok(true) => {
                        self.assembly = self.assembly_build.as_mut().and_then(AssemblyJobConstruction::take_complete);
                        if self.assembly.is_none() {
                            return self.fail(b"fem2d.assembly-construction-false-terminal".to_vec());
                        }
                        self.stage = MountedStage::Assembly;
                        cx.consume_fuel(1);
                        self.progress(b"fem2d.assembly-admitted")
                    }
                    Err(error) => self.fail(error.to_string().into_bytes()),
                },
                MountedStage::Assembly => match self.assembly.as_mut().expect("assembly stage owns assembly").step(&mut cx) {
                    StepOutcome::Complete(candidate) => {
                        let assembly = self.assembly.take().expect("assembly owner retained");
                        let csr_build = match AssemblyCsrBuild::new(assembly) {
                            Ok(builder) => builder,
                            Err(assembly) => {
                                self.assembly = Some(assembly);
                                return self.fail(b"fem2d.assembly-false-terminal".to_vec());
                            }
                        };
                        self.csr_build = Some(csr_build);
                        self.stage = MountedStage::BuildCsr;
                        let semio_framework_job::CommitCandidate { mut state, output } = candidate;
                        close_retained_payload(&mut state);
                        match take_retained_payload(output, SESSION_MAXIMUM_OUTPUT_BYTES) {
                            Some(bytes) => JobStep::Running(Some(bytes)),
                            None => self.progress(b"fem2d.assembly-complete"),
                        }
                    }
                    StepOutcome::PreviewReady(payload) | StepOutcome::CheckpointReady(semio_framework_job::Checkpoint { state: payload, .. }) => match take_retained_payload(payload, SESSION_MAXIMUM_OUTPUT_BYTES) {
                        Some(bytes) => JobStep::Running(Some(bytes)),
                        None => self.progress(b"fem2d.assembly-yield"),
                    },
                    StepOutcome::Yield => self.progress(b"fem2d.assembly-yield"),
                    StepOutcome::Cancelled => self.fail(b"fem2d.assembly-cancelled".to_vec()),
                    StepOutcome::Fault(fault) => self.fail(take_retained_payload(fault.detail, SESSION_MAXIMUM_FAULT_BYTES).unwrap_or_else(|| b"fem2d.assembly-fault-capacity".to_vec())),
                },
                MountedStage::BuildCsr => match self.csr_build.as_mut().expect("CSR builder retained").step_one() {
                    Ok(false) => {
                        cx.consume_fuel(1);
                        self.progress(b"fem2d.csr-building")
                    }
                    Ok(true) => {
                        let matrix = self.csr_build.as_mut().and_then(AssemblyCsrBuild::take_complete).expect("complete CSR transfers exactly once");
                        self.pcg_build = Some(PcgJobConstruction::new(self.identity.operation(), matrix));
                        self.stage = MountedStage::PreparePcg;
                        cx.consume_fuel(1);
                        self.progress(b"fem2d.csr-complete")
                    }
                    Err(detail) => self.fail(detail.to_vec()),
                },
                MountedStage::PreparePcg => match self.pcg_build.as_mut().expect("PCG builder retained").step_one() {
                    Ok(false) => {
                        cx.consume_fuel(1);
                        self.progress(b"fem2d.pcg-preparing")
                    }
                    Ok(true) => {
                        self.pcg = self.pcg_build.as_mut().and_then(PcgJobConstruction::take_complete);
                        if self.pcg.is_none() {
                            return self.fail(b"fem2d.pcg-false-terminal".to_vec());
                        }
                        self.stage = MountedStage::Pcg;
                        cx.consume_fuel(1);
                        self.progress(b"fem2d.pcg-admitted")
                    }
                    Err(detail) => self.fail(detail.to_vec()),
                },
                MountedStage::Pcg => match self.pcg.as_mut().expect("pcg stage owns pcg").step(&mut cx) {
                    StepOutcome::Complete(candidate) => {
                        if let Some(job) = self.pcg.as_ref() {
                            let (completed, total, residual, tolerance, converged) = job.visual_progress();
                            self.visual.progress_completed = completed;
                            self.visual.progress_total = total;
                            self.visual.residual_norm = residual;
                            self.visual.tolerance = tolerance;
                            self.visual.converged = converged;
                            self.visual.state = if converged { FemVisualState::SolvingConverged } else { FemVisualState::SolvingUnconverged };
                        }
                        self.visual_field_cursor = 0;
                        self.visual_pcg_complete = true;
                        self.stage = MountedStage::SyncPcgVisual;
                        let semio_framework_job::CommitCandidate { mut state, output } = candidate;
                        close_retained_payload(&mut state);
                        match take_retained_payload(output, SESSION_MAXIMUM_OUTPUT_BYTES) {
                            Some(bytes) => JobStep::Running(Some(bytes)),
                            None => self.progress(b"fem2d.pcg-complete"),
                        }
                    }
                    StepOutcome::PreviewReady(payload) | StepOutcome::CheckpointReady(semio_framework_job::Checkpoint { state: payload, .. }) => {
                        if let Some(job) = self.pcg.as_ref() {
                            let (completed, total, residual, tolerance, converged) = job.visual_progress();
                            self.visual.progress_completed = completed;
                            self.visual.progress_total = total;
                            self.visual.residual_norm = residual;
                            self.visual.tolerance = tolerance;
                            self.visual.converged = converged;
                            self.visual.state = if converged { FemVisualState::SolvingConverged } else { FemVisualState::SolvingUnconverged };
                        }
                        self.visual_field_cursor = 0;
                        self.visual_pcg_complete = false;
                        self.stage = MountedStage::SyncPcgVisual;
                        match take_retained_payload(payload, SESSION_MAXIMUM_OUTPUT_BYTES) {
                            Some(bytes) => JobStep::Running(Some(bytes)),
                            None => self.progress(b"fem2d.pcg-output-capacity"),
                        }
                    }
                    StepOutcome::Yield => self.progress(b"fem2d.pcg-yield"),
                    StepOutcome::Cancelled => self.fail(b"fem2d.pcg-cancelled".to_vec()),
                    StepOutcome::Fault(fault) => self.fail(take_retained_payload(fault.detail, SESSION_MAXIMUM_FAULT_BYTES).unwrap_or_else(|| b"fem2d.pcg-fault-capacity".to_vec())),
                },
                MountedStage::SyncPcgVisual => {
                    let Some(snapshot) = self.snapshot.as_ref() else { return self.fail(b"fem2d.visual-snapshot-owner".to_vec()) };
                    let Some(node) = snapshot.nodes.get(self.visual_field_cursor) else {
                        self.visual_dirty = true;
                        self.stage = if self.visual_pcg_complete { MountedStage::CommitReady } else { MountedStage::Pcg };
                        cx.consume_fuel(1);
                        return self.progress(b"fem2d.visual-field-page-complete");
                    };
                    let base = self.visual_field_cursor * 3;
                    let Some(tx) = self.pcg.as_ref().and_then(|job| job.visual_scalar(base)) else { return self.fail(b"fem2d.visual-tx-scalar".to_vec()) };
                    let Some(ty) = self.pcg.as_ref().and_then(|job| job.visual_scalar(base + 1)) else { return self.fail(b"fem2d.visual-ty-scalar".to_vec()) };
                    let field = crate::editor::fem2d::modes::edit::windows::model::NodeLiveField {
                        node_id: node.id.clone(),
                        displacement: [tx.displacement, ty.displacement],
                        residual: [tx.residual, ty.residual],
                        reaction: [tx.reaction, ty.reaction],
                        contour: tx.contour.max(ty.contour),
                        mode_shape: [tx.mode_estimate, ty.mode_estimate],
                    };
                    if let Some(target) = self.visual.fields.get_mut(self.visual_field_cursor) {
                        *target = field;
                    } else {
                        self.visual.fields.push(field);
                    }
                    self.visual_field_cursor += 1;
                    cx.consume_fuel(1);
                    self.progress(b"fem2d.visual-field-entry")
                }
                MountedStage::CommitReady => {
                    let validation = current_identity(self.identity.app_instance_id);
                    let store_is_current = self.snapshot.as_ref().is_some_and(|snapshot| snapshot.commit_authority_matches(self.identity.generation.0, self.identity.canonical_base_revision));
                    if validation != Some(self.identity) || !store_is_current || self.cancel.is_cancelled_now() {
                        return self.fail(b"fem2d.session-stale-commit".to_vec());
                    }
                    self.visual.validated_final = self.visual.converged;
                    self.visual.state = if self.visual.validated_final { FemVisualState::ValidatedFinal } else { FemVisualState::SolvingUnconverged };
                    self.visual_dirty = true;
                    self.stage = MountedStage::PublishFinalVisual;
                    cx.consume_fuel(1);
                    self.progress(b"fem2d.final-visual-requested")
                }
                MountedStage::PublishFinalVisual => {
                    let validation = current_identity(self.identity.app_instance_id);
                    let store_is_current = self.snapshot.as_ref().is_some_and(|snapshot| snapshot.commit_authority_matches(self.identity.generation.0, self.identity.canonical_base_revision));
                    if validation != Some(self.identity) || !store_is_current || self.cancel.is_cancelled_now() {
                        return self.fail(b"fem2d.session-stale-final-visual".to_vec());
                    }
                    self.stage = MountedStage::Complete;
                    let mut output = Vec::with_capacity(32);
                    output.extend_from_slice(&self.identity.operation.0.to_le_bytes());
                    output.extend_from_slice(&self.identity.base_revision.0.to_le_bytes());
                    output.extend_from_slice(&self.identity.generation.0.to_le_bytes());
                    output.extend_from_slice(&(self.admitted_items as u64).to_le_bytes());
                    cx.consume_fuel(1);
                    JobStep::Done(output)
                }
                MountedStage::Complete => JobStep::Done(Vec::new()),
                MountedStage::Fault => JobStep::Failed(self.fault.clone().unwrap_or_else(|| b"fem2d.session-fault".to_vec())),
                MountedStage::Closing | MountedStage::Empty => JobStep::Failed(b"fem2d.session-closed".to_vec()),
            }
        })();
        self.preview_sequence = preview_sequence;
        result
    }

    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> PluginCloseStep {
        if maximum_items == 0 {
            return PluginCloseStep::Pending { released_items: 0, released_bytes: 0 };
        }
        self.cancel.cancel_now();
        self.stage = MountedStage::Closing;
        loop {
            match self.close_cursor {
                0 => {
                    if self.graph_plans.pop().is_some() {
                        return PluginCloseStep::Pending { released_items: 1, released_bytes: 0 };
                    }
                    let plan_bytes = self.graph_plans.capacity() * std::mem::size_of::<FemStagePlan>();
                    if plan_bytes != 0 {
                        if plan_bytes > maximum_bytes {
                            return PluginCloseStep::Pending { released_items: 0, released_bytes: 0 };
                        }
                        self.graph_plans = Vec::new();
                        return PluginCloseStep::Pending { released_items: 1, released_bytes: plan_bytes };
                    }
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
                        let (terminal, released_items, released_bytes) = model_build.close_step(maximum_bytes);
                        if !terminal {
                            return PluginCloseStep::Pending { released_items, released_bytes };
                        }
                        self.model_build = None;
                        return PluginCloseStep::Pending { released_items: 1, released_bytes: std::mem::size_of::<MountedModelBuild>() };
                    }
                    self.close_cursor += 1;
                }
                3 => {
                    if let Some(assembly_build) = self.assembly_build.as_mut() {
                        let (terminal, released_items, released_bytes) = assembly_build.close_step(maximum_bytes);
                        if !terminal {
                            return PluginCloseStep::Pending { released_items, released_bytes };
                        }
                        self.assembly_build = None;
                        return PluginCloseStep::Pending { released_items: 1, released_bytes: std::mem::size_of::<AssemblyJobConstruction>() };
                    }
                    self.close_cursor += 1;
                }
                4 => {
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
                5 => {
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
                6 => {
                    if let Some(pcg_build) = self.pcg_build.as_mut() {
                        let (terminal, released_items, released_bytes) = pcg_build.close_step(maximum_bytes);
                        if !terminal {
                            return PluginCloseStep::Pending { released_items, released_bytes };
                        }
                        self.pcg_build = None;
                        return PluginCloseStep::Pending { released_items: 1, released_bytes: std::mem::size_of::<PcgJobConstruction>() };
                    }
                    self.close_cursor += 1;
                }
                7 => {
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
                8 => {
                    if let Some(domain) = self.domain.as_mut() {
                        match self.domain_close_lane {
                            0 if domain.outer.pop().is_some() => return PluginCloseStep::Pending { released_items: 1, released_bytes: 0 },
                            0 => {
                                let bytes = domain.outer.capacity() * std::mem::size_of::<[f64; 2]>();
                                if bytes > maximum_bytes {
                                    return PluginCloseStep::Pending { released_items: 0, released_bytes: 0 };
                                }
                                domain.outer = Vec::new();
                                self.domain_close_lane = 1;
                                return PluginCloseStep::Pending { released_items: 1, released_bytes: bytes };
                            }
                            1 => {
                                if let Some(hole) = domain.holes.last_mut() {
                                    if hole.pop().is_some() {
                                        return PluginCloseStep::Pending { released_items: 1, released_bytes: 0 };
                                    }
                                    let bytes = hole.capacity() * std::mem::size_of::<[f64; 2]>();
                                    if bytes > maximum_bytes {
                                        return PluginCloseStep::Pending { released_items: 0, released_bytes: 0 };
                                    }
                                    *hole = Vec::new();
                                    domain.holes.pop();
                                    return PluginCloseStep::Pending { released_items: 1, released_bytes: bytes };
                                }
                                self.domain_close_lane = 2;
                                continue;
                            }
                            2 => {
                                let bytes = domain.holes.capacity() * std::mem::size_of::<Vec<[f64; 2]>>();
                                if bytes > maximum_bytes {
                                    return PluginCloseStep::Pending { released_items: 0, released_bytes: 0 };
                                }
                                domain.holes = Vec::new();
                                self.domain_close_lane = 3;
                                return PluginCloseStep::Pending { released_items: 1, released_bytes: bytes };
                            }
                            _ => {
                                self.domain = None;
                                return PluginCloseStep::Pending { released_items: 1, released_bytes: 0 };
                            }
                        }
                    }
                    self.close_cursor += 1;
                }
                9 => {
                    if let Some(field) = self.visual.fields.last_mut() {
                        if field.node_id.capacity() != 0 {
                            let bytes = field.node_id.capacity();
                            if bytes > maximum_bytes {
                                return PluginCloseStep::Pending { released_items: 0, released_bytes: 0 };
                            }
                            field.node_id = String::new();
                            return PluginCloseStep::Pending { released_items: 1, released_bytes: bytes };
                        }
                        self.visual.fields.pop();
                        return PluginCloseStep::Pending { released_items: 1, released_bytes: 0 };
                    }
                    let bytes = self.visual.fields.capacity() * std::mem::size_of::<crate::editor::fem2d::modes::edit::windows::model::NodeLiveField>();
                    if bytes != 0 {
                        if bytes > maximum_bytes {
                            return PluginCloseStep::Pending { released_items: 0, released_bytes: 0 };
                        }
                        self.visual.fields = Vec::new();
                        return PluginCloseStep::Pending { released_items: 1, released_bytes: bytes };
                    }
                    self.close_cursor += 1;
                }
                10 => {
                    if let Some(owner) = self.visual.assembling_element_ids.last_mut() {
                        if owner.capacity() != 0 {
                            let bytes = owner.capacity();
                            if bytes > maximum_bytes {
                                return PluginCloseStep::Pending { released_items: 0, released_bytes: 0 };
                            }
                            *owner = String::new();
                            return PluginCloseStep::Pending { released_items: 1, released_bytes: bytes };
                        }
                        self.visual.assembling_element_ids.pop();
                        return PluginCloseStep::Pending { released_items: 1, released_bytes: 0 };
                    }
                    let bytes = self.visual.assembling_element_ids.capacity() * std::mem::size_of::<String>();
                    if bytes != 0 {
                        if bytes > maximum_bytes {
                            return PluginCloseStep::Pending { released_items: 0, released_bytes: 0 };
                        }
                        self.visual.assembling_element_ids = Vec::new();
                        return PluginCloseStep::Pending { released_items: 1, released_bytes: bytes };
                    }
                    self.close_cursor += 1;
                }
                11 => {
                    if self.visual_region_owner.is_none() {
                        if let Some(owner) = self.visual.region_quality.take_one() {
                            self.visual_region_owner = Some(owner);
                            return PluginCloseStep::Pending { released_items: 1, released_bytes: 0 };
                        }
                    }
                    if let Some((id, _)) = self.visual_region_owner.as_mut() {
                        if id.capacity() != 0 {
                            let bytes = id.capacity();
                            if bytes > maximum_bytes {
                                return PluginCloseStep::Pending { released_items: 0, released_bytes: 0 };
                            }
                            *id = String::new();
                            return PluginCloseStep::Pending { released_items: 1, released_bytes: bytes };
                        }
                        self.visual_region_owner = None;
                        return PluginCloseStep::Pending { released_items: 1, released_bytes: 0 };
                    }
                    self.close_cursor += 1;
                }
                12 => {
                    if let Some(candidate) = self.visual_rejected.as_mut() {
                        let (terminal, released_items, released_bytes) = candidate.close_step(maximum_bytes);
                        if !terminal {
                            return PluginCloseStep::Pending { released_items, released_bytes };
                        }
                        if !candidate.terminal_is_empty() {
                            return PluginCloseStep::Blocked { reason: "mounted FEM rejected visual reported false terminal" };
                        }
                        self.visual_rejected = None;
                        return PluginCloseStep::Pending { released_items: 1, released_bytes: 0 };
                    }
                    if let Some(candidate) = self.visual_job_candidate.as_mut() {
                        let (terminal, released_items, released_bytes) = candidate.close_step(maximum_bytes);
                        if !terminal {
                            return PluginCloseStep::Pending { released_items, released_bytes };
                        }
                        if !candidate.terminal_is_empty() {
                            return PluginCloseStep::Blocked { reason: "mounted FEM live visual candidate reported false terminal" };
                        }
                        self.visual_job_candidate = None;
                        return PluginCloseStep::Pending { released_items: 1, released_bytes: 0 };
                    }
                    self.close_cursor += 1;
                }
                13 => {
                    if let Some(displaced) = self.visual_displaced.as_mut() {
                        let (terminal, released_items, released_bytes) = displaced.close_step(maximum_bytes);
                        if !terminal {
                            return PluginCloseStep::Pending { released_items, released_bytes };
                        }
                        if !displaced.terminal_is_empty() {
                            return PluginCloseStep::Blocked { reason: "mounted FEM displaced visual reported false terminal" };
                        }
                        self.visual_displaced = None;
                        return PluginCloseStep::Pending { released_items: 1, released_bytes: 0 };
                    }
                    self.close_cursor += 1;
                }
                14 => {
                    if let Some(current) = self.visual_current.as_mut() {
                        let (terminal, released_items, released_bytes) = current.close_step(maximum_bytes);
                        if !terminal {
                            return PluginCloseStep::Pending { released_items, released_bytes };
                        }
                        if !current.terminal_is_empty() {
                            return PluginCloseStep::Blocked { reason: "mounted FEM current visual reported false terminal" };
                        }
                        self.visual_current = None;
                        return PluginCloseStep::Pending { released_items: 1, released_bytes: 0 };
                    }
                    self.close_cursor += 1;
                }
                15 => {
                    if let Some(snapshot) = self.snapshot.take() {
                        let Some(witness) = snapshot.return_to_registry_witness() else {
                            return PluginCloseStep::Blocked { reason: "mounted FEM snapshot lease was already returned" };
                        };
                        self.snapshot_return = Some(witness);
                        return PluginCloseStep::Pending { released_items: 1, released_bytes: std::mem::size_of::<store::SnapshotRead<Fem2dSnapshot>>() };
                    }
                    if self.snapshot_return.as_ref().is_some_and(|witness| !witness.terminal_is_empty()) {
                        return PluginCloseStep::Pending { released_items: 0, released_bytes: 0 };
                    }
                    if self.snapshot_return.take().is_some() {
                        return PluginCloseStep::Pending { released_items: 1, released_bytes: std::mem::size_of::<store::SnapshotReadReturn>() };
                    }
                    self.close_cursor += 1;
                }
                16 => {
                    if let Some(fault) = self.fault.as_mut() {
                        if fault.pop().is_some() {
                            return PluginCloseStep::Pending { released_items: 1, released_bytes: 0 };
                        }
                        let bytes = fault.capacity();
                        if bytes > maximum_bytes {
                            return PluginCloseStep::Pending { released_items: 0, released_bytes: 0 };
                        }
                        *fault = Vec::new();
                        self.fault = None;
                        return PluginCloseStep::Pending { released_items: 1, released_bytes: bytes };
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
            && self.snapshot_return.is_none()
            && self.graph_plans.capacity() == 0
            && self.graph.is_none()
            && self.mesh.is_none()
            && self.model_build.is_none()
            && self.assembly_build.is_none()
            && self.assembly.is_none()
            && self.csr_build.is_none()
            && self.pcg_build.is_none()
            && self.pcg.is_none()
            && self.domain.is_none()
            && self.visual.region_quality.is_empty()
            && self.visual_region_owner.is_none()
            && self.visual.assembling_element_ids.is_empty()
            && self.visual.fields.is_empty()
            && self.visual_job_candidate.is_none()
            && self.visual_rejected.is_none()
            && self.visual_current.is_none()
            && self.visual_displaced.is_none()
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
    admitted_items: usize,
    admitted_bytes: usize,
}

#[derive(Clone, Copy)]
struct SnapshotAdmissionCursor {
    lane: u8,
    outer: usize,
    inner: usize,
    deep: usize,
    owner_opened: bool,
    items: usize,
    bytes: usize,
    visual_loads: usize,
}

impl SnapshotAdmissionCursor {
    fn new() -> Self {
        Self { lane: 0, outer: 0, inner: 0, deep: 0, owner_opened: false, items: 1, bytes: std::mem::size_of::<Fem2dSnapshot>(), visual_loads: 0 }
    }

    fn charge(&mut self, items: usize, bytes: usize) -> Result<(), &'static [u8]> {
        let items = self.items.checked_add(items).ok_or(b"fem2d.session-item-overflow" as &'static [u8])?;
        let bytes = self.bytes.checked_add(bytes).ok_or(b"fem2d.session-byte-overflow" as &'static [u8])?;
        if items > SESSION_MAXIMUM_INPUT_ITEMS || bytes > SESSION_MAXIMUM_INPUT_BYTES {
            return Err(b"fem2d.session-admission-exceeded");
        }
        self.items = items;
        self.bytes = bytes;
        Ok(())
    }

    fn reset_lane(&mut self) {
        self.lane += 1;
        self.outer = 0;
        self.inner = 0;
        self.deep = 0;
        self.owner_opened = false;
    }

    fn step_one(&mut self, snapshot: &Fem2dSnapshot) -> Result<bool, &'static [u8]> {
        if !self.owner_opened && !matches!(self.lane, 4 | 9 | 11) {
            let bytes = match self.lane {
                0 => snapshot.nodes.capacity() * std::mem::size_of::<crate::artifacts::fem2d::FemNode>(),
                1 => snapshot.elements.capacity() * std::mem::size_of::<FemElement>(),
                2 => snapshot.regions.capacity() * std::mem::size_of::<crate::artifacts::fem2d::FemRegion>(),
                3 => 0,
                5 => snapshot.materials.capacity() * std::mem::size_of::<crate::artifacts::fem2d::FemMaterial>(),
                6 => snapshot.sections.capacity() * std::mem::size_of::<crate::artifacts::fem2d::FemSection>(),
                7 => snapshot.supports.capacity() * std::mem::size_of::<crate::artifacts::fem2d::FemSupport>(),
                8 => snapshot.load_cases.capacity() * std::mem::size_of::<crate::artifacts::fem2d::FemLoadCase>(),
                10 => snapshot.combinations.capacity() * std::mem::size_of::<crate::artifacts::fem2d::FemCombination>(),
                _ => 0,
            };
            self.owner_opened = true;
            self.charge(1, bytes)?;
            return Ok(false);
        }
        let item = match self.lane {
            0 => {
                if snapshot.nodes.len() > SESSION_MAXIMUM_NODES {
                    return Err(b"fem2d.session-node-capacity");
                }
                match snapshot.nodes.get(self.outer) {
                    Some(node) => Some(bounded_string_capacities(&[&node.id])?),
                    None => None,
                }
            }
            1 => {
                if snapshot.elements.len() > SESSION_MAXIMUM_ELEMENTS {
                    return Err(b"fem2d.session-element-capacity");
                }
                match snapshot.elements.get(self.outer) {
                    Some(FemElement::Bar { id, start, end, material_id, section_id }) | Some(FemElement::Beam { id, start, end, material_id, section_id }) => Some(bounded_string_capacities(&[id, start, end, material_id, section_id])?),
                    None => None,
                }
            }
            2 => {
                if snapshot.regions.len() > 1 {
                    return Err(b"fem2d.session-region-capacity");
                }
                match snapshot.regions.get(self.outer) {
                    Some(region) => Some(if region.outline.len() > SESSION_MAXIMUM_BOUNDARY_POINTS || region.holes.len() > SESSION_MAXIMUM_REGION_HOLES {
                        SESSION_MAXIMUM_INPUT_BYTES + 1
                    } else {
                        bounded_string_capacities(&[&region.id, &region.name, &region.material_id])? + region.outline.capacity() * 16 + region.holes.capacity() * std::mem::size_of::<Vec<[f64; 2]>>()
                    }),
                    None => None,
                }
            }
            3 => {
                let Some(region) = snapshot.regions.get(self.outer) else {
                    self.reset_lane();
                    return Ok(false);
                };
                if self.inner < region.outline.len() {
                    self.inner += 1;
                    Some(0)
                } else {
                    self.outer += 1;
                    self.inner = 0;
                    return Ok(false);
                }
            }
            4 => {
                let Some(region) = snapshot.regions.get(self.outer) else {
                    self.reset_lane();
                    return Ok(false);
                };
                let Some(hole) = region.holes.get(self.inner) else {
                    self.outer += 1;
                    self.inner = 0;
                    self.deep = 0;
                    self.owner_opened = false;
                    return Ok(false);
                };
                if hole.len() > SESSION_MAXIMUM_BOUNDARY_POINTS {
                    return Err(b"fem2d.session-hole-capacity");
                }
                if !self.owner_opened {
                    self.owner_opened = true;
                    self.charge(1, hole.capacity() * 16)?;
                    return Ok(false);
                }
                if self.deep < hole.len() {
                    self.deep += 1;
                    Some(0)
                } else {
                    self.inner += 1;
                    self.deep = 0;
                    self.owner_opened = false;
                    return Ok(false);
                }
            }
            5 => match snapshot.materials.get(self.outer) {
                Some(material) => Some(bounded_string_capacities(&[&material.id, &material.name])?),
                None => None,
            },
            6 => match snapshot.sections.get(self.outer) {
                Some(section) => Some(bounded_string_capacities(&[&section.id, &section.name])?),
                None => None,
            },
            7 => {
                if snapshot.supports.len() > SESSION_MAXIMUM_SUPPORTS {
                    return Err(b"fem2d.session-support-capacity");
                }
                match snapshot.supports.get(self.outer) {
                    Some(support) => Some(bounded_string_capacities(&[&support.id, &support.node_id])? + support.fixed.capacity() * std::mem::size_of::<crate::artifacts::fem2d::FemDof>()),
                    None => None,
                }
            }
            8 => match snapshot.load_cases.get(self.outer) {
                Some(case) => Some(bounded_string_capacities(&[&case.id, &case.name])?),
                None => None,
            },
            9 => {
                let Some(case) = snapshot.load_cases.get(self.outer) else {
                    self.reset_lane();
                    return Ok(false);
                };
                if !self.owner_opened {
                    self.owner_opened = true;
                    self.charge(1, case.loads.capacity() * std::mem::size_of::<FemLoad>())?;
                    return Ok(false);
                }
                if let Some(load) = case.loads.get(self.inner) {
                    self.visual_loads = self.visual_loads.checked_add(1).ok_or(b"fem2d.visual-load-count-overflow" as &'static [u8])?;
                    if self.visual_loads > SESSION_MAXIMUM_VISUAL_LOADS {
                        return Err(b"fem2d.visual-load-maximum-plus-one");
                    }
                    self.inner += 1;
                    Some(match load {
                        FemLoad::Nodal { id, node_id, .. } => bounded_string_capacities(&[id, node_id])?,
                        FemLoad::MemberUdl { id, element_id, .. } => bounded_string_capacities(&[id, element_id])?,
                        FemLoad::Area { id, region_id, .. } => bounded_string_capacities(&[id, region_id])?,
                    })
                } else {
                    self.outer += 1;
                    self.inner = 0;
                    self.owner_opened = false;
                    return Ok(false);
                }
            }
            10 => match snapshot.combinations.get(self.outer) {
                Some(combination) => Some(bounded_string_capacities(&[&combination.id, &combination.name])?),
                None => None,
            },
            11 => {
                let Some(combination) = snapshot.combinations.get(self.outer) else {
                    self.reset_lane();
                    return Ok(false);
                };
                if !self.owner_opened {
                    self.owner_opened = true;
                    self.charge(1, combination.terms.capacity() * std::mem::size_of::<crate::artifacts::fem2d::FemCombinationTerm>())?;
                    return Ok(false);
                }
                if let Some(term) = combination.terms.get(self.inner) {
                    self.inner += 1;
                    Some(bounded_string_capacities(&[&term.case_id])?)
                } else {
                    self.outer += 1;
                    self.inner = 0;
                    self.owner_opened = false;
                    return Ok(false);
                }
            }
            _ => return Ok(true),
        };
        if let Some(bytes) = item {
            self.charge(1, bytes)?;
            if matches!(self.lane, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 10) {
                self.outer += 1;
            }
            return Ok(false);
        }
        self.reset_lane();
        Ok(self.lane > 11)
    }

    fn process_credit(self) -> Result<(usize, usize), &'static [u8]> {
        let (items, bytes) = MountedProcessOwnerCatalog::fixed().credit(self.items, self.bytes)?;
        if items > SESSION_MAXIMUM_ITEMS || bytes > SESSION_MAXIMUM_BYTES {
            return Err(b"fem2d.session-process-admission-exceeded");
        }
        Ok((items, bytes))
    }
}

#[derive(Clone, Copy)]
struct PendingSnapshotAdmission {
    app_instance_id: u32,
    render: AppRenderOperationContext,
    cursor: SnapshotAdmissionCursor,
}

#[derive(Clone, Copy)]
struct PendingSnapshotFault {
    app_instance_id: u32,
    render: AppRenderOperationContext,
    detail: &'static str,
    emitted: bool,
}

struct MountedRegistry {
    shells: [Rc<RefCell<Option<MountedState>>>; SESSION_SHELL_CAPACITY],
    current: [Option<CurrentSession>; SESSION_ACTIVE_CAPACITY],
    preflight: [Option<PendingSnapshotAdmission>; SESSION_ACTIVE_CAPACITY],
    preflight_fault: [Option<PendingSnapshotFault>; SESSION_ACTIVE_CAPACITY],
    pending: [Option<PendingAdmission>; SESSION_ACTIVE_CAPACITY],
    retiring: [Option<u16>; SESSION_SHELL_CAPACITY],
    free: [u16; SESSION_SHELL_CAPACITY],
    free_read: usize,
    free_len: usize,
    credit_items: [usize; SESSION_SHELL_CAPACITY],
    credit_bytes: [usize; SESSION_SHELL_CAPACITY],
    reserved_items: usize,
    reserved_bytes: usize,
    next_job: u64,
}

impl MountedRegistry {
    fn new() -> Self {
        Self {
            shells: std::array::from_fn(|_| Rc::new(RefCell::new(None))),
            current: [None; SESSION_ACTIVE_CAPACITY],
            preflight: [None; SESSION_ACTIVE_CAPACITY],
            preflight_fault: [None; SESSION_ACTIVE_CAPACITY],
            pending: [None; SESSION_ACTIVE_CAPACITY],
            retiring: [None; SESSION_SHELL_CAPACITY],
            free: std::array::from_fn(|index| index as u16),
            free_read: 0,
            free_len: SESSION_SHELL_CAPACITY,
            credit_items: [0; SESSION_SHELL_CAPACITY],
            credit_bytes: [0; SESSION_SHELL_CAPACITY],
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
        assert_eq!(self.credit_items[shell as usize], 0, "mounted FEM shell released before its process item credit");
        assert_eq!(self.credit_bytes[shell as usize], 0, "mounted FEM shell released before its process byte credit");
        let write = (self.free_read + self.free_len) % SESSION_SHELL_CAPACITY;
        self.free[write] = shell;
        self.free_len += 1;
    }

    fn reserve_credit(&mut self, shell: u16, admitted_items: usize, admitted_bytes: usize) -> bool {
        if self.credit_items[shell as usize] != 0 || self.credit_bytes[shell as usize] != 0 || admitted_items == 0 || admitted_bytes == 0 || admitted_items > SESSION_MAXIMUM_ITEMS || admitted_bytes > SESSION_MAXIMUM_BYTES {
            return false;
        }
        let Some(items) = self.reserved_items.checked_add(admitted_items) else { return false };
        let Some(bytes) = self.reserved_bytes.checked_add(admitted_bytes) else { return false };
        if items > SESSION_SHELL_CAPACITY * SESSION_MAXIMUM_ITEMS || bytes > SESSION_SHELL_CAPACITY * SESSION_MAXIMUM_BYTES {
            return false;
        }
        self.credit_items[shell as usize] = admitted_items;
        self.credit_bytes[shell as usize] = admitted_bytes;
        self.reserved_items = items;
        self.reserved_bytes = bytes;
        true
    }

    fn release_credit(&mut self, shell: u16) {
        let items = std::mem::take(&mut self.credit_items[shell as usize]);
        let bytes = std::mem::take(&mut self.credit_bytes[shell as usize]);
        assert!(items != 0 && bytes != 0, "mounted FEM shell released a missing process credit");
        self.reserved_items -= items;
        self.reserved_bytes -= bytes;
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

/// 🔍️ Advances exactly one schema owner/scalar census before the shared application may
/// request an opaque snapshot lease. The fixed shell and exact process credit are reserved only
/// after the census reaches its complete witness.
pub fn prepare_snapshot_read(render: AppRenderOperationContext, snapshot: &Fem2dSnapshot) -> bool {
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
        let matches_render = |pending: PendingSnapshotAdmission| {
            pending.app_instance_id == render.app_instance_id && pending.render.base_revision == render.base_revision && pending.render.generation == render.generation && pending.render.canonical_base_revision == render.canonical_base_revision
        };
        if let Some(fault) = registry.preflight_fault[current_slot] {
            if fault.app_instance_id == render.app_instance_id && fault.render.base_revision == render.base_revision && fault.render.generation == render.generation && fault.render.canonical_base_revision == render.canonical_base_revision {
                return false;
            }
            registry.preflight_fault[current_slot] = None;
        }
        if !registry.preflight[current_slot].is_some_and(matches_render) {
            registry.preflight[current_slot] = Some(PendingSnapshotAdmission { app_instance_id: render.app_instance_id, render, cursor: SnapshotAdmissionCursor::new() });
            return false;
        }
        let completed = {
            let preflight = registry.preflight[current_slot].as_mut().expect("matching FEM preflight retained");
            match preflight.cursor.step_one(snapshot) {
                Ok(completed) => completed,
                Err(detail) => {
                    registry.preflight[current_slot] = None;
                    registry.preflight_fault[current_slot] = Some(PendingSnapshotFault { app_instance_id: render.app_instance_id, render, detail: std::str::from_utf8(detail).unwrap_or("fem2d.session-preflight-fault"), emitted: false });
                    return false;
                }
            }
        };
        if !completed {
            return false;
        }
        let preflight = registry.preflight[current_slot].take().expect("completed FEM preflight retained");
        let (process_items, process_bytes) = match preflight.cursor.process_credit() {
            Ok(credit) => credit,
            Err(detail) => {
                registry.preflight_fault[current_slot] = Some(PendingSnapshotFault { app_instance_id: render.app_instance_id, render, detail: std::str::from_utf8(detail).unwrap_or("fem2d.session-process-credit-fault"), emitted: false });
                return false;
            }
        };
        let Some(shell) = registry.allocate() else { return false };
        if !registry.reserve_credit(shell, process_items, process_bytes) {
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
        registry.pending[current_slot] = Some(PendingAdmission { app_instance_id: render.app_instance_id, shell, identity, admitted_items: process_items, admitted_bytes: process_bytes });
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
        if let Some(fault) = registry.preflight_fault[current_slot].as_mut().filter(|fault| {
            fault.app_instance_id == render.app_instance_id && fault.render.base_revision == render.base_revision && fault.render.generation == render.generation && fault.render.canonical_base_revision == render.canonical_base_revision
        }) {
            if !fault.emitted {
                fault.emitted = true;
                return vec![Effect::Notify { message: fault.detail.to_string() }];
            }
            return Vec::new();
        }
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
        *registry.shells[shell as usize].borrow_mut() = Some(MountedState::new(identity, snapshot, pending.admitted_items, pending.admitted_bytes));
        registry.current[current_slot] = Some(CurrentSession { app_instance_id: render.app_instance_id, shell, identity });
        let mut effects = Vec::with_capacity(2);
        if let Some(previous) = previous {
            effects.push(Effect::CancelJob { job: previous.identity.job });
        }
        effects.push(Effect::SpawnJob { job, kind: FEM2D_MOUNTED_JOB_KIND.to_string(), input: encode_input(shell, identity), placement: JobPlacement::Isolated });
        effects
    })
}

/// 👁️ Borrows the immutable generation-exact mounted visual lease only while the renderer adopts it.
pub fn with_live_visual<R>(render: Option<AppRenderOperationContext>, build: impl FnOnce(Option<&Fem2dMountedVisualLease>) -> R) -> R {
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
    build(owner.as_ref().and_then(|state| state.visual_current.as_ref()).filter(|lease| lease.matches(render.app_instance_id, render.base_revision.0, render.generation.0)))
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
        if registry.preflight_fault[slot].is_some_and(|fault| fault.app_instance_id == app_instance_id) {
            registry.preflight_fault[slot] = None;
            return PluginCloseStep::Pending { released_items: 1, released_bytes: std::mem::size_of::<PendingSnapshotFault>() };
        }
        if registry.preflight[slot].is_some_and(|preflight| preflight.app_instance_id == app_instance_id) {
            registry.preflight[slot] = None;
            return PluginCloseStep::Pending { released_items: 1, released_bytes: std::mem::size_of::<PendingSnapshotAdmission>() };
        }
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
        registry.preflight_fault[slot].is_none_or(|fault| fault.app_instance_id != app_instance_id)
            && registry.preflight[slot].is_none_or(|preflight| preflight.app_instance_id != app_instance_id)
            && registry.pending[slot].is_none_or(|pending| pending.app_instance_id != app_instance_id)
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
        let snapshot = Fem2dSnapshot::default();
        assert!((0..64).any(|_| prepare_snapshot_read(render, &snapshot)), "empty schema census completes incrementally");
        let first = MOUNTED.with(|registry| registry.borrow().pending[render.app_instance_id as usize % SESSION_ACTIVE_CAPACITY].expect("pending admission"));
        for _ in 0..1_025 {
            assert!(prepare_snapshot_read(render, &snapshot));
        }
        let retained = MOUNTED.with(|registry| registry.borrow().pending[render.app_instance_id as usize % SESSION_ACTIVE_CAPACITY].expect("same pending admission"));
        assert_eq!(first.shell, retained.shell);
        assert_eq!(first.identity, retained.identity);
        assert!(matches!(close_step(render.app_instance_id, 1, 4_096), PluginCloseStep::Pending { released_items: 1, .. }));
        assert!(terminal_is_empty(render.app_instance_id));
    }

    #[test]
    fn snapshot_census_advances_one_owner_and_rejects_exact_plus_one_without_partial_credit() {
        let render = AppRenderOperationContext { app_instance_id: 2_000_000_006, base_revision: semio_framework_job::RevisionId(29), generation: semio_framework_job::Generation(31), canonical_base_revision: [37; 32] };
        let snapshot = Fem2dSnapshot::default();
        assert!(!prepare_snapshot_read(render, &snapshot));
        MOUNTED.with(|registry| {
            let registry = registry.borrow();
            let slot = render.app_instance_id as usize % SESSION_ACTIVE_CAPACITY;
            let cursor = registry.preflight[slot].expect("one retained census cursor").cursor;
            assert_eq!((cursor.lane, cursor.outer, cursor.inner, cursor.deep), (0, 0, 0, 0));
            assert!(cursor.owner_opened);
            assert!(registry.pending[slot].is_none(), "no lease shell is admitted during the first schema-owner opportunity");
        });

        let mut exact = SnapshotAdmissionCursor::new();
        exact.items = SESSION_MAXIMUM_INPUT_ITEMS - 1;
        exact.bytes = SESSION_MAXIMUM_INPUT_BYTES - 1;
        assert_eq!(exact.charge(1, 1), Ok(()));
        let before = (exact.items, exact.bytes);
        assert_eq!(exact.charge(1, 0), Err(b"fem2d.session-admission-exceeded" as &'static [u8]));
        assert_eq!((exact.items, exact.bytes), before, "plus one returns the exact unchanged census owner");
        assert!(matches!(close_step(render.app_instance_id, 1, 4_096), PluginCloseStep::Pending { released_items: 1, .. }));
        assert!(terminal_is_empty(render.app_instance_id));
    }

    #[test]
    fn process_owner_inventory_admits_exact_maximum_and_returns_exact_credit() {
        let catalog = MountedProcessOwnerCatalog::fixed();
        let mut seen = [false; 30];
        for claim in catalog.claims {
            let index = claim.class as usize;
            assert!(!seen[index], "every simultaneous owner class is inventoried exactly once");
            assert!(claim.roots != 0, "every class retains at least one fixed backing root");
            seen[index] = true;
        }
        assert!(seen.into_iter().all(|present| present));
        let exact = catalog.credit(SESSION_MAXIMUM_INPUT_ITEMS, SESSION_MAXIMUM_INPUT_BYTES).expect("the enumerated working set exactly fits its process authority");
        assert_eq!(exact, (SESSION_MAXIMUM_ITEMS, SESSION_MAXIMUM_BYTES));
        assert_eq!(catalog.credit(SESSION_MAXIMUM_INPUT_ITEMS + 1, SESSION_MAXIMUM_INPUT_BYTES), Err(b"fem2d.session-process-input-credit-exceeded" as &'static [u8]));
        assert_eq!(catalog.credit(SESSION_MAXIMUM_INPUT_ITEMS, SESSION_MAXIMUM_INPUT_BYTES + 1), Err(b"fem2d.session-process-input-credit-exceeded" as &'static [u8]));

        let mut overflow = catalog;
        overflow.claims[MountedOwnerClass::OutputPages as usize].roots += 1;
        assert_eq!(overflow.credit(SESSION_MAXIMUM_INPUT_ITEMS, SESSION_MAXIMUM_INPUT_BYTES), Err(b"fem2d.session-process-admission-exceeded" as &'static [u8]));

        let mut registry = MountedRegistry::new();
        let shell = registry.allocate().expect("fixed process shell");
        assert!(registry.reserve_credit(shell, exact.0, exact.1));
        let rejected = registry.allocate().expect("distinct fixed shell retains a failed request");
        assert!(!registry.reserve_credit(rejected, exact.0 + 1, exact.1), "plus one cannot partially change the registry credit");
        assert_eq!((registry.credit_items[rejected as usize], registry.credit_bytes[rejected as usize]), (0, 0));
        registry.release(rejected);
        registry.release_credit(shell);
        registry.release(shell);
        assert_eq!((registry.reserved_items, registry.reserved_bytes), (0, 0));
        let returned: [u16; SESSION_SHELL_CAPACITY] = std::array::from_fn(|_| registry.allocate().expect("all fixed shells returned"));
        assert_eq!(&returned[SESSION_SHELL_CAPACITY - 2..], &[rejected, shell], "failed and completed admissions return their exact shells in FIFO order");
    }

    #[test]
    fn interrupted_model_close_releases_one_retained_root_per_grant() {
        let mut build = MountedModelBuild::new(None);
        build.model.nodes.push(crate::model::Node { id: "deep-node".repeat(128), pos: [0.0; 3] });
        build.region_node_ids.push("deep-region-node".repeat(128));
        let first = build.close_step(4_096);
        assert!(!first.0);
        assert_eq!(build.model.nodes.len(), 1);
        assert_eq!(build.model.nodes[0].id.capacity(), 0, "the first grant releases only the nested string backing");
        assert_eq!(build.region_node_ids.len(), 1, "a distinct retained root survives the interrupted close");
        while !build.model.nodes.is_empty() {
            assert!(!build.close_step(4_096).0);
        }
        assert_eq!(build.region_node_ids.len(), 1, "later close lanes remain untouched while the model page retires");
        while !build.close_step(4_096).0 {}
    }

    #[test]
    fn mounted_revision_restart_keeps_cancel_before_spawn() {
        let source = include_str!("component.rs");
        let admission = &source[source.find("pub fn prepare_snapshot_read(").expect("admission")..source.find("pub fn reconcile(").expect("reconcile")];
        let reconcile = &source[source.find("pub fn reconcile(").expect("reconcile")..source.find("pub fn with_live_visual").expect("visual boundary")];
        assert!(reconcile.find("Effect::CancelJob").expect("cancel effect") < reconcile.find("Effect::SpawnJob").expect("spawn effect"));
        assert!(admission.contains("checked_add(1)"));
        assert!(admission.contains("reserve_credit(shell, process_items, process_bytes)"));
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
            "AssemblyJobConstruction::new_owned",
            "PcgJobConstruction::new",
            "commit_authority_matches",
            "Effect::SpawnJob",
            "Effect::CancelJob",
            "JobPlacement::Isolated",
            "with_live_visual",
            "take_snapshot_read",
            "SESSION_MAXIMUM_ITEMS",
            "SESSION_MAXIMUM_BYTES",
            "MountedProcessOwnerCatalog",
            "MountedOwnerClass::AssemblyDofStrings",
            "mounted_node_id",
            "reserve_exact_owner_page",
        ] {
            assert!(source.contains(needle), "missing mounted FEM contract {needle}");
        }
        for forbidden in ["crate::fem2d_engine::build_model(", "AssemblyJob::new_owned(", ".into_full_matrix(", "PcgJob::new(", "pending_node_ids", ".fixed.iter().map"] {
            assert!(!source.contains(forbidden), "mounted FEM route restored bulk constructor {forbidden}");
        }
    }
}
//#endregion 🧪️Tests
