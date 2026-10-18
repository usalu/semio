//! 🏗️ Model build job: the bounded string helpers, the mounted identity and stages, the model build stages and the mounted model build with its stepping. See [`super`].

use super::*;

//#region 🔖️Contract
pub(super) const SESSION_MAXIMUM_ITEMS: usize = SESSION_MAXIMUM_INPUT_ITEMS + MountedProcessOwnerCatalog::fixed().items() + MountedProcessOwnerCatalog::fixed().roots();

pub(super) const SESSION_MAXIMUM_BYTES: usize = SESSION_MAXIMUM_INPUT_BYTES + MountedProcessOwnerCatalog::fixed().roots() * SESSION_OWNER_PAGE_BYTES;

pub(super) fn bounded_string_capacities(owners: &[&String]) -> Result<usize, &'static [u8]> {
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
pub(super) struct MountedIdentity {
    pub(super) app_instance_id: u32,
    pub(super) base_revision: semio_framework_job::RevisionId,
    pub(super) generation: semio_framework_job::Generation,
    pub(super) canonical_base_revision: [u8; 32],
    pub(super) operation: OperationId,
    pub(super) job: u64,
}

impl MountedIdentity {
    pub(super) fn operation(self) -> Operation {
        Operation::new(self.operation, self.base_revision, self.generation, self.operation.0.rotate_left(17) ^ self.base_revision.0)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum MountedStage {
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
pub(super) enum ModelBuildStage {
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

pub(super) struct MountedModelBuild {
    pub(super) stage: ModelBuildStage,
    node_cursor: usize,
    element_cursor: usize,
    support_cursor: usize,
    region_point_cursor: usize,
    region_triangle_cursor: usize,
    lookup_cursor: usize,
    resolved_material: usize,
    resolved_section: usize,
    pub(super) pending_region_id: Option<String>,
    pub(super) pending_node_id: Option<String>,
    pub(super) pending_element_id: Option<String>,
    pub(super) pending_element_start: Option<String>,
    pub(super) pending_element_end: Option<String>,
    pub(super) pending_support: Option<semio_s_fem_model::Support>,
    pub(super) pending_region_element_id: Option<String>,
    pub(super) pending_region_element_nodes: [Option<String>; 3],
    pending_region_element_node_cursor: usize,
    pending_region_insert: bool,
    pub(super) mesh: Option<TriMesh2>,
    pub(super) region_node_ids: Vec<String>,
    pub(super) model: AnalysisModel,
    pub(super) close_lane: u8,
}

impl MountedModelBuild {
    pub(super) fn new(mesh: Option<TriMesh2>) -> Self {
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

    pub(super) fn step_one(&mut self, snapshot: &Fem2dSnapshot) -> Result<bool, Vec<u8>> {
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
                if self.model.nodes.capacity() * size_of::<semio_s_fem_model::Node>() > SESSION_OWNER_PAGE_BYTES {
                    return Err(b"fem2d.model-node-page-capacity".to_vec());
                }
                self.stage = ModelBuildStage::ReserveElements;
            }
            ModelBuildStage::ReserveElements => {
                self.model.elements.try_reserve_exact(snapshot.elements.len() + mesh_triangles).map_err(|_| b"fem2d.model-element-allocation".to_vec())?;
                if self.model.elements.capacity() * size_of::<semio_s_fem_static::Elements>() > SESSION_OWNER_PAGE_BYTES {
                    return Err(b"fem2d.model-element-page-capacity".to_vec());
                }
                self.stage = ModelBuildStage::ReserveSupports;
            }
            ModelBuildStage::ReserveSupports => {
                self.model.supports.try_reserve_exact(snapshot.supports.len()).map_err(|_| b"fem2d.model-support-allocation".to_vec())?;
                if self.model.supports.capacity() * size_of::<semio_s_fem_model::Support>() > SESSION_OWNER_PAGE_BYTES {
                    return Err(b"fem2d.model-support-page-capacity".to_vec());
                }
                self.stage = ModelBuildStage::ReserveRegionNodeIds;
            }
            ModelBuildStage::ReserveRegionNodeIds => {
                self.region_node_ids.try_reserve_exact(mesh_points).map_err(|_| b"fem2d.model-region-node-allocation".to_vec())?;
                if self.region_node_ids.capacity() * size_of::<String>() > SESSION_OWNER_PAGE_BYTES {
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
                self.model.nodes.push(semio_s_fem_model::Node { id, pos: [node.x, node.y, 0.0] });
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
                    FemElement::Bar { .. } => semio_s_fem_elements2d::Bar2 {
                        id: self.pending_element_id.take().ok_or_else(|| b"fem2d.model-element-id-owner-missing".to_vec())?,
                        start: self.pending_element_start.take().ok_or_else(|| b"fem2d.model-element-start-owner-missing".to_vec())?,
                        end: self.pending_element_end.take().ok_or_else(|| b"fem2d.model-element-end-owner-missing".to_vec())?,
                        e: material.e,
                        area: section.area,
                        density: material.rho,
                    }
                    .into(),
                    FemElement::Beam { .. } => semio_s_fem_elements2d::BeamEb2 {
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
                    self.pending_support = Some(semio_s_fem_model::Support { node_id: support.node_id.clone(), fixed: Vec::new() });
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
                if owner.fixed.capacity() * size_of::<semio_s_fem_model::Dof>() > SESSION_OWNER_PAGE_BYTES {
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
                if snapshot.regions.is_empty() {
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
                self.model.nodes.push(semio_s_fem_model::Node { id, pos: [point[0], point[1], 0.0] });
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
                    semio_s_fem_elements2d::Tri3Cst {
                        id: self.pending_region_element_id.take().ok_or_else(|| b"fem2d.model-region-element-id-owner-missing".to_vec())?,
                        nodes,
                        e: material.e,
                        nu: material.nu,
                        thickness: region.thickness,
                        kind: semio_s_fem_elements2d::PlaneKind::Stress,
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

}
//#endregion 🔖️Contract
