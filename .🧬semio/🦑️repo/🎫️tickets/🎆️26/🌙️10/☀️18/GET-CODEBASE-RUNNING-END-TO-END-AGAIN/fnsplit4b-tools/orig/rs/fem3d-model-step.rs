//! 🧱️ Numerical child, model half: the meshed solid, the child state and the model assembly step.

use super::*;

//#region 📦️PageVisualJob
pub(super) struct Fem3dMeshedSolid {
    pub(super) solid_index: usize,
    pub(super) node_ids: FixedSlots<String, MAXIMUM_FIELDS>,
    pub(super) top_offset: usize,
    pub(super) points: FixedSlots<[f64; 2], MAXIMUM_FIELDS>,
    pub(super) tris: FixedSlots<[u32; 3], MAXIMUM_ELEMENTS>,
}

pub struct Fem3dNumericalChild {
    pub stage: Fem3dNumericalStage,
    model: Option<MountedAnalysisModel>,
    analysis_node_ids: FixedSlots<String, MAXIMUM_FIELDS>,
    pub(super) meshed_solids: FixedSlots<Fem3dMeshedSolid, MAXIMUM_REGIONS>,
    node_cursor: usize,
    element_cursor: usize,
    support_cursor: usize,
    pub(super) dof_cursor: usize,
    pub(super) lookup_cursor: usize,
    material_cursor: usize,
    section_cursor: usize,
    resolved_material: usize,
    resolved_section: usize,
    pub(super) element_materials: [usize; MAXIMUM_ELEMENTS],
    pub(super) element_sections: [usize; MAXIMUM_ELEMENTS],
    pending_element_nodes: [usize; 2],
    pub(super) pending_support: Option<MountedAnalysisSupport>,
    pub(super) pending_element: Option<Elements>,
    mass_update_cursor: usize,
    pending_translational_mass: f64,
    pending_rotational_mass: f64,
    pub(super) assembly_build: Option<AssemblyJobConstruction>,
    pub(super) assembly: Option<AssemblyJob<'static>>,
    pub(super) csr_build: Option<AssemblyCsrBuild>,
    pub(super) pcg_build: Option<PcgJobConstruction>,
    pub(super) rejected_pcg_matrix: Option<Csr>,
    pub(super) rejected_pcg_rhs: Option<MountedScalarSlots>,
    pub pcg: Option<PcgJob>,
    pub(super) modal_build: Option<SymmetricPencilConstruction>,
    pub(super) ldlt: Option<LdltJob>,
    subspace: Option<SubspaceIterationJob>,
    pub(super) modal_mass: Option<Csr>,
    scalar_axis: usize,
    scalar: Fem3dSolverScalar,
    pub(super) equations: [[Option<usize>; 6]; MAXIMUM_FIELDS],
    pub(super) full_equations: [[Option<usize>; 6]; MAXIMUM_FIELDS],
    pub(super) full_rhs: [f64; MAXIMUM_FIELDS * 6],
    pub(super) full_reactions: [[f64; 6]; MAXIMUM_FIELDS],
    pub(super) stress_sums: [f64; MAXIMUM_FIELDS],
    pub(super) stress_counts: [usize; MAXIMUM_FIELDS],
    pub(super) free_order: usize,
    pub(super) rhs: MountedScalarSlots,
    pub(super) request: Fem3dResultRequest,
    pub(super) source_case: usize,
    pub(super) source_factor: f64,
    pub(super) load_cursor: usize,
    pub(super) load_node_cursor: usize,
    pub(super) load_solid_cursor: usize,
    pub(super) load_triangle_cursor: usize,
    pub(super) load_vertex_cursor: usize,
    pub(super) load_element_cursor: usize,
    pub(super) load_node_indices: [usize; 2],
    pub(super) load_positions: [[f64; 3]; 2],
    reaction_entry: usize,
    pub(super) reaction_accumulator: f64,
    pub(super) solid_cursor: usize,
    hole_cursor: usize,
    pub(super) point_cursor: usize,
    pub(super) solid_domain: Option<MountedPlanarDomain>,
    pub(super) mesh: Option<MeshJob>,
    pub(super) solid_points: FixedSlots<[f64; 2], MAXIMUM_FIELDS>,
    pub(super) solid_tris: FixedSlots<[u32; 3], MAXIMUM_ELEMENTS>,
    pub(super) solid_node_ids: FixedSlots<String, MAXIMUM_FIELDS>,
    pub(super) solid_node_analysis_indices: FixedSlots<usize, MAXIMUM_FIELDS>,
    volume_node_cursor: usize,
    pub(super) pending_node_id: Option<String>,
    pending_node_index: usize,
    pending_node_needs_analysis: bool,
    tet_cursor: usize,
    pub(super) tet_phase: usize,
    pub(super) pending_tet: Option<Elements>,
    pending_tet_indices: [usize; 4],
    pending_tet_mass: f64,
    solid_material_cursor: usize,
    resolved_solid_material: usize,
    pub(super) solid_materials: [usize; MAXIMUM_REGIONS],
    close_lane: u8,
    close_started_jobs: u8,
    operation: Option<semio_framework_job::Operation>,
    pub(super) solver_page_cursor: usize,
    pub(super) solver_page_lane: bool,
    pub(super) modal_lumped_mass: [f64; MAXIMUM_FIELDS * 6],
    pub(super) modal_free_mass: MountedScalarSlots,
}


impl Fem3dNumericalChild {
    pub fn close_demands(&self) -> Result<RetirementDemand, ValueError> {
        let mut demand = RetirementDemand { release_bytes: SESSION_CLOSE_RELEASE_BYTES, depth: SESSION_CLOSE_DEPTH, ..Default::default() };
        let mut merge = |child: RetirementDemand| {
            demand.copy_bytes = demand.copy_bytes.max(child.copy_bytes);
            demand.capacity_bytes = demand.capacity_bytes.max(child.capacity_bytes);
            demand.release_bytes = demand.release_bytes.max(child.release_bytes);
            demand.depth = demand.depth.max(child.depth);
        };
        if let Some(child) = self.mesh.as_ref() { merge(semio_s_fem_sparse::numerical_close_demand(child)?); }
        if let Some(child) = self.pcg.as_ref() { merge(semio_s_fem_sparse::numerical_close_demand(child)?); }
        if let Some(child) = self.ldlt.as_ref() { merge(semio_s_fem_sparse::numerical_close_demand(child)?); }
        if let Some(child) = self.subspace.as_ref() { merge(semio_s_fem_sparse::numerical_close_demand(child)?); }
        if let Some(child) = self.pcg_build.as_ref() { merge(child.close_demand()?); }
        Ok(demand)
    }

    pub fn new() -> Self {
        Self {
            stage: Fem3dNumericalStage::ReserveSolverPages,
            model: Some(MountedAnalysisModel::new()),
            analysis_node_ids: FixedSlots::new(),
            meshed_solids: FixedSlots::new(),
            node_cursor: 0,
            element_cursor: 0,
            support_cursor: 0,
            dof_cursor: 0,
            lookup_cursor: 0,
            material_cursor: 0,
            section_cursor: 0,
            resolved_material: 0,
            resolved_section: 0,
            element_materials: [0; MAXIMUM_ELEMENTS],
            element_sections: [0; MAXIMUM_ELEMENTS],
            pending_element_nodes: [0; 2],
            pending_support: None,
            pending_element: None,
            mass_update_cursor: 0,
            pending_translational_mass: 0.0,
            pending_rotational_mass: 0.0,
            assembly_build: None,
            assembly: None,
            csr_build: None,
            pcg_build: None,
            rejected_pcg_matrix: None,
            rejected_pcg_rhs: None,
            pcg: None,
            modal_build: None,
            ldlt: None,
            subspace: None,
            modal_mass: None,
            scalar_axis: 0,
            scalar: Fem3dSolverScalar::default(),
            equations: [[None; 6]; MAXIMUM_FIELDS],
            full_equations: [[None; 6]; MAXIMUM_FIELDS],
            full_rhs: [0.0; MAXIMUM_FIELDS * 6],
            full_reactions: [[0.0; 6]; MAXIMUM_FIELDS],
            stress_sums: [0.0; MAXIMUM_FIELDS],
            stress_counts: [0; MAXIMUM_FIELDS],
            free_order: 0,
            rhs: MountedScalarSlots::new(),
            request: Fem3dResultRequest::default(),
            source_case: 0,
            source_factor: 1.0,
            load_cursor: 0,
            load_node_cursor: 0,
            load_solid_cursor: 0,
            load_triangle_cursor: 0,
            load_vertex_cursor: 0,
            load_element_cursor: 0,
            load_node_indices: [0; 2],
            load_positions: [[0.0; 3]; 2],
            reaction_entry: 0,
            reaction_accumulator: 0.0,
            solid_cursor: 0,
            hole_cursor: 0,
            point_cursor: 0,
            solid_domain: None,
            mesh: None,
            solid_points: FixedSlots::new(),
            solid_tris: FixedSlots::new(),
            solid_node_ids: FixedSlots::new(),
            solid_node_analysis_indices: FixedSlots::new(),
            volume_node_cursor: 0,
            pending_node_id: None,
            pending_node_index: 0,
            pending_node_needs_analysis: false,
            tet_cursor: 0,
            tet_phase: 0,
            pending_tet: None,
            pending_tet_indices: [0; 4],
            pending_tet_mass: 0.0,
            solid_material_cursor: 0,
            resolved_solid_material: 0,
            solid_materials: [0; MAXIMUM_REGIONS],
            close_lane: 0,
            close_started_jobs: 0,
            operation: None,
            solver_page_cursor: 0,
            solver_page_lane: false,
            modal_lumped_mass: [0.0; MAXIMUM_FIELDS * 6],
            modal_free_mass: MountedScalarSlots::new(),
        }
    }

    fn solid_point(&self, doc: &Fem3dSnapshot, index: usize) -> Result<[f64; 3], Vec<u8>> {
        let solid = doc.solids.get(self.solid_cursor).ok_or_else(|| b"fem3d.numerical-solid".to_vec())?;
        let footprint = self.solid_points.len();
        if footprint == 0 {
            return Err(b"fem3d.numerical-solid-empty-mesh".to_vec());
        }
        let layers = solid.layers.max(1);
        let layer = index / footprint;
        let point = *self.solid_points.get(index % footprint).ok_or_else(|| b"fem3d.numerical-solid-point".to_vec())?;
        Ok(solid.axis.to_world(point[0], point[1], solid.base_z + solid.height * layer as f64 / layers as f64))
    }

    fn solid_tet_indices(&self) -> Result<[usize; 4], Vec<u8>> {
        let source = self.solid_tris.get(self.tet_cursor).ok_or_else(|| b"fem3d.numerical-solid-triangle".to_vec())?;
        Ok(Self::tet_indices(self.solid_points.len(), *source, self.point_cursor, self.tet_phase))
    }

    pub(super) fn tet_indices(footprint: usize, source: [u32; 3], layer: usize, phase: usize) -> [usize; 4] {
        let mut bottom = [source[0] as usize, source[1] as usize, source[2] as usize];
        if bottom[1] < bottom[0] && bottom[1] <= bottom[2] {
            bottom = [bottom[1], bottom[2], bottom[0]];
        } else if bottom[2] < bottom[0] && bottom[2] < bottom[1] {
            bottom = [bottom[2], bottom[0], bottom[1]];
        }
        let [n0, n1, n2] = bottom.map(|index| layer * footprint + index);
        let [n3, n4, n5] = bottom.map(|index| (layer + 1) * footprint + index);
        match (phase, n1 <= n2) {
            (0, true) => [n0, n1, n2, n5],
            (1, true) => [n0, n1, n5, n4],
            (0, false) => [n0, n1, n2, n4],
            (1, false) => [n0, n2, n5, n4],
            _ => [n0, n3, n4, n5],
        }
    }

    pub(super) fn apply_rhs_value(&mut self, node: usize, axis: usize, value: f64) -> Result<(), Vec<u8>> {
        let value = value * self.source_factor;
        if !value.is_finite() {
            return Err(b"fem3d.numerical-load-value".to_vec());
        }
        if let Some(equation) = self.full_equations[node][axis] {
            self.full_rhs[equation] += value;
        }
        if let Some(equation) = self.equations[node][axis] {
            self.rhs.add_at(equation, value).map_err(|_| b"fem3d.numerical-rhs-index".to_vec())?;
        }
        Ok(())
    }

    fn observe_child_end(&mut self, end: ChildEnd) -> Result<bool, Vec<u8>> {
        match end {
            ChildEnd::Cancelled => Err(b"fem3d.numerical-cancelled".to_vec()),
            ChildEnd::Fault => Err(b"fem3d.numerical-child-fault".to_vec()),
            ChildEnd::Complete => Ok(true),
            ChildEnd::Yield | ChildEnd::Lent => Ok(false),
        }
    }

    pub(super) fn step_model(&mut self, doc: &Fem3dSnapshot) -> Result<bool, Vec<u8>> {
        match self.stage {
            Fem3dNumericalStage::ReserveNodes => {
                if self.model.as_mut().ok_or_else(|| b"fem3d.numerical-model-owner".to_vec())?.admit_node_one(doc.nodes.len(), MOUNTED_ANALYSIS_BACKING_BYTES).map_err(|_| b"fem3d.numerical-node-admission".to_vec())?.complete {
                    self.stage = Fem3dNumericalStage::ReserveNodeIds;
                }
            }
            Fem3dNumericalStage::ReserveNodeIds => {
                if self.analysis_node_ids.admit_one(doc.nodes.len()).map_err(|_| b"fem3d.numerical-node-id-admission".to_vec())? {
                    self.stage = Fem3dNumericalStage::ReserveElements;
                }
            }
            Fem3dNumericalStage::ReserveElements => {
                if self.model.as_mut().ok_or_else(|| b"fem3d.numerical-model-owner".to_vec())?.admit_element_one(doc.elements.len(), MOUNTED_ANALYSIS_BACKING_BYTES).map_err(|_| b"fem3d.numerical-element-admission".to_vec())?.complete {
                    self.stage = Fem3dNumericalStage::ReserveSupports;
                }
            }
            Fem3dNumericalStage::ReserveSupports => {
                if self.model.as_mut().ok_or_else(|| b"fem3d.numerical-model-owner".to_vec())?.admit_support_one(doc.supports.len(), MOUNTED_ANALYSIS_BACKING_BYTES).map_err(|_| b"fem3d.numerical-support-admission".to_vec())?.complete {
                    self.stage = Fem3dNumericalStage::ReserveMeshedSolids;
                }
            }
            Fem3dNumericalStage::ReserveMeshedSolids => {
                if self.meshed_solids.admit_one(doc.solids.len()).map_err(|_| b"fem3d.numerical-solid-admission".to_vec())? {
                    self.stage = Fem3dNumericalStage::Nodes;
                }
            }
            Fem3dNumericalStage::Nodes => {
                if let Some(node) = doc.nodes.get(self.node_cursor) {
                    self.model.as_mut().ok_or_else(|| b"fem3d.numerical-model-owner".to_vec())?.push_node(Node { id: node.id.clone(), pos: [node.x, node.y, node.z] }).map_err(|_| b"fem3d.numerical-node-slot".to_vec())?;
                    self.stage = Fem3dNumericalStage::NodeId;
                } else {
                    self.stage = Fem3dNumericalStage::ElementMaterial;
                }
            }
            Fem3dNumericalStage::NodeId => {
                let node = doc.nodes.get(self.node_cursor).ok_or_else(|| b"fem3d.numerical-node".to_vec())?;
                self.analysis_node_ids.push(node.id.clone()).map_err(|_| b"fem3d.numerical-node-id-slot".to_vec())?;
                self.node_cursor += 1;
                self.stage = Fem3dNumericalStage::Nodes;
            }
            Fem3dNumericalStage::ElementMaterial => {
                let Some(element) = doc.elements.get(self.element_cursor) else {
                    self.stage = Fem3dNumericalStage::SolidDomainOuterReserve;
                    return Ok(false);
                };
                let material_id = match element {
                    FemElement::Bar { material_id, .. } | FemElement::Frame { material_id, .. } => material_id,
                };
                let Some(material) = doc.materials.get(self.material_cursor) else { return Err(b"fem3d.numerical-material".to_vec()) };
                if &material.id == material_id {
                    self.resolved_material = self.material_cursor;
                    self.material_cursor = 0;
                    self.stage = Fem3dNumericalStage::ElementSection;
                } else {
                    self.material_cursor += 1;
                }
            }
            Fem3dNumericalStage::ElementSection => {
                let Some(element) = doc.elements.get(self.element_cursor) else { return Err(b"fem3d.numerical-element".to_vec()) };
                let section_id = match element {
                    FemElement::Bar { section_id, .. } | FemElement::Frame { section_id, .. } => section_id,
                };
                let Some(section) = doc.sections.get(self.section_cursor) else { return Err(b"fem3d.numerical-section".to_vec()) };
                if &section.id == section_id {
                    self.resolved_section = self.section_cursor;
                    self.section_cursor = 0;
                    self.stage = Fem3dNumericalStage::ElementStart;
                } else {
                    self.section_cursor += 1;
                }
            }
            Fem3dNumericalStage::ElementStart | Fem3dNumericalStage::ElementEnd => {
                let Some(element) = doc.elements.get(self.element_cursor) else { return Err(b"fem3d.numerical-element".to_vec()) };
                let (start, end) = match element {
                    FemElement::Bar { start, end, .. } | FemElement::Frame { start, end, .. } => (start, end),
                };
                let target = if self.stage == Fem3dNumericalStage::ElementStart { start } else { end };
                let Some(node) = doc.nodes.get(self.lookup_cursor) else { return Err(b"fem3d.numerical-element-node".to_vec()) };
                if &node.id == target {
                    self.pending_element_nodes[usize::from(self.stage == Fem3dNumericalStage::ElementEnd)] = self.lookup_cursor;
                    self.lookup_cursor = 0;
                    self.stage = if self.stage == Fem3dNumericalStage::ElementStart { Fem3dNumericalStage::ElementEnd } else { Fem3dNumericalStage::ElementCommit };
                } else {
                    self.lookup_cursor += 1;
                }
            }
            Fem3dNumericalStage::ElementCommit => {
                let element = doc.elements.get(self.element_cursor).ok_or_else(|| b"fem3d.numerical-element".to_vec())?;
                let material = doc.materials.get(self.resolved_material).ok_or_else(|| b"fem3d.numerical-material".to_vec())?;
                let section = doc.sections.get(self.resolved_section).ok_or_else(|| b"fem3d.numerical-section".to_vec())?;
                let built: Elements = match element {
                    FemElement::Bar { id, start, end, .. } => Bar3 { id: id.clone(), node_a: start.clone(), node_b: end.clone(), e: material.e, a: section.area, density: material.rho }.into(),
                    FemElement::Frame { id, start, end, roll, .. } => {
                        Frame3 { id: id.clone(), node_a: start.clone(), node_b: end.clone(), e: material.e, g: material.g, a: section.area, iy: section.iy, iz: section.iz, j: section.j, roll: *roll, density: material.rho }.into()
                    }
                };
                self.element_materials[self.element_cursor] = self.resolved_material;
                self.element_sections[self.element_cursor] = self.resolved_section;
                let start = doc.nodes.get(self.pending_element_nodes[0]).ok_or_else(|| b"fem3d.numerical-element-node".to_vec())?;
                let end = doc.nodes.get(self.pending_element_nodes[1]).ok_or_else(|| b"fem3d.numerical-element-node".to_vec())?;
                let dx = end.x - start.x;
                let dy = end.y - start.y;
                let dz = end.z - start.z;
                let length = (dx * dx + dy * dy + dz * dz).sqrt();
                self.pending_translational_mass = material.rho * section.area * length * 0.5;
                self.pending_rotational_mass = if matches!(element, FemElement::Frame { .. }) { material.rho * (section.area * length.powi(3) / 105.0 + section.j * length / 3.0) } else { 0.0 };
                self.pending_element = Some(built);
                self.mass_update_cursor = 0;
                self.stage = Fem3dNumericalStage::ElementMass;
            }
            Fem3dNumericalStage::ElementMass => {
                let limit = if self.pending_rotational_mass > 0.0 { 12 } else { 6 };
                if self.mass_update_cursor < limit {
                    let node = self.pending_element_nodes[(self.mass_update_cursor / 3) % 2];
                    let axis = self.mass_update_cursor % 3 + usize::from(self.mass_update_cursor >= 6) * 3;
                    let value = if self.mass_update_cursor < 6 { self.pending_translational_mass } else { self.pending_rotational_mass };
                    self.modal_lumped_mass[node * 6 + axis] += value;
                    self.mass_update_cursor += 1;
                } else {
                    self.stage = Fem3dNumericalStage::ElementInsert;
                }
            }
            Fem3dNumericalStage::ElementInsert => {
                let built = self.pending_element.take().ok_or_else(|| b"fem3d.numerical-element-owner".to_vec())?;
                if let Err(built) = self.model.as_mut().ok_or_else(|| b"fem3d.numerical-model-owner".to_vec())?.push_element(built) {
                    self.pending_element = Some(built);
                    return Err(b"fem3d.numerical-element-slot".to_vec());
                }
                self.element_cursor += 1;
                self.stage = Fem3dNumericalStage::ElementMaterial;
            }
            Fem3dNumericalStage::SolidDomainOuterReserve => {
                let Some(solid) = doc.solids.get(self.solid_cursor) else {
                    self.stage = Fem3dNumericalStage::SupportDofReserve;
                    return Ok(false);
                };
                if solid.layers == 0 || solid.height <= 0.0 || solid.mesh_size <= 0.0 || solid.outline.len() < 3 {
                    return Err(b"fem3d.numerical-solid-domain".to_vec());
                }
                if self.solid_domain.is_none() {
                    self.solid_domain = Some(MountedPlanarDomain::new());
                }
                if self.solid_domain.as_mut().ok_or_else(|| b"fem3d.numerical-solid-domain-owner".to_vec())?.admit_outer_one(solid.outline.len()).map_err(|_| b"fem3d.numerical-solid-outline-admission".to_vec())? {
                    self.point_cursor = 0;
                    self.stage = Fem3dNumericalStage::SolidDomainOuterPoint;
                }
            }
            Fem3dNumericalStage::SolidDomainOuterPoint => {
                let solid = doc.solids.get(self.solid_cursor).ok_or_else(|| b"fem3d.numerical-solid".to_vec())?;
                if let Some(point) = solid.outline.get(self.point_cursor) {
                    self.solid_domain.as_mut().ok_or_else(|| b"fem3d.numerical-solid-domain-owner".to_vec())?.push_outer(*point).map_err(|_| b"fem3d.numerical-solid-outline-slot".to_vec())?;
                    self.point_cursor += 1;
                } else {
                    self.stage = Fem3dNumericalStage::SolidDomainHolesReserve;
                }
            }
            Fem3dNumericalStage::SolidDomainHolesReserve => {
                let solid = doc.solids.get(self.solid_cursor).ok_or_else(|| b"fem3d.numerical-solid".to_vec())?;
                if self.solid_domain.as_mut().ok_or_else(|| b"fem3d.numerical-solid-domain-owner".to_vec())?.admit_hole_one(solid.holes.len()).map_err(|_| b"fem3d.numerical-solid-holes-admission".to_vec())? {
                    self.hole_cursor = 0;
                    self.stage = Fem3dNumericalStage::SolidDomainHoleReserve;
                }
            }
            Fem3dNumericalStage::SolidDomainHoleReserve => {
                let solid = doc.solids.get(self.solid_cursor).ok_or_else(|| b"fem3d.numerical-solid".to_vec())?;
                let Some(hole) = solid.holes.get(self.hole_cursor) else {
                    self.stage = Fem3dNumericalStage::SolidMeshBegin;
                    return Ok(false);
                };
                let domain = self.solid_domain.as_mut().ok_or_else(|| b"fem3d.numerical-solid-domain-owner".to_vec())?;
                if self.point_cursor == 0 {
                    domain.begin_hole().map_err(|_| b"fem3d.numerical-solid-hole-slot".to_vec())?;
                    self.point_cursor = 1;
                    return Ok(false);
                }
                if domain.admit_hole_point_one(self.hole_cursor, hole.len()).map_err(|_| b"fem3d.numerical-solid-hole-admission".to_vec())? {
                    self.point_cursor = 0;
                    self.stage = Fem3dNumericalStage::SolidDomainHolePoint;
                }
            }
            Fem3dNumericalStage::SolidDomainHolePoint => {
                let solid = doc.solids.get(self.solid_cursor).ok_or_else(|| b"fem3d.numerical-solid".to_vec())?;
                let hole = solid.holes.get(self.hole_cursor).ok_or_else(|| b"fem3d.numerical-solid-hole".to_vec())?;
                if let Some(point) = hole.get(self.point_cursor) {
                    self.solid_domain.as_mut().ok_or_else(|| b"fem3d.numerical-solid-hole-owner".to_vec())?.push_hole_point(self.hole_cursor, *point).map_err(|_| b"fem3d.numerical-solid-hole-point-slot".to_vec())?;
                    self.point_cursor += 1;
                } else {
                    self.hole_cursor += 1;
                    self.point_cursor = 0;
                    self.stage = Fem3dNumericalStage::SolidDomainHoleReserve;
                }
            }
            Fem3dNumericalStage::SolidMaterial => {
                let solid = doc.solids.get(self.solid_cursor).ok_or_else(|| b"fem3d.numerical-solid".to_vec())?;
                let Some(material) = doc.materials.get(self.solid_material_cursor) else { return Err(b"fem3d.numerical-solid-material".to_vec()) };
                if material.id == solid.material_id {
                    self.resolved_solid_material = self.solid_material_cursor;
                    self.solid_material_cursor = 0;
                    self.point_cursor = 0;
                    self.stage = Fem3dNumericalStage::SolidOwnersReserve;
                } else {
                    self.solid_material_cursor += 1;
                }
            }
            Fem3dNumericalStage::SolidOwnersReserve => {
                let solid = doc.solids.get(self.solid_cursor).ok_or_else(|| b"fem3d.numerical-solid".to_vec())?;
                let node_count = self.solid_points.len().checked_mul(solid.layers + 1).ok_or_else(|| b"fem3d.numerical-solid-node-overflow".to_vec())?;
                let tet_count = self.solid_tris.len().checked_mul(solid.layers).and_then(|count| count.checked_mul(3)).ok_or_else(|| b"fem3d.numerical-solid-element-overflow".to_vec())?;
                let model = self.model.as_mut().ok_or_else(|| b"fem3d.numerical-model-owner".to_vec())?;
                if model.nodes_len().saturating_add(node_count) > MAXIMUM_FIELDS || model.elements_len().saturating_add(tet_count) > MAXIMUM_ELEMENTS {
                    return Err(b"fem3d.numerical-solid-capacity".to_vec());
                }
                match self.point_cursor {
                    0 if self.solid_node_ids.admit_one(node_count).map_err(|_| b"fem3d.numerical-solid-node-id-admission".to_vec())? => self.point_cursor += 1,
                    1 if self.solid_node_analysis_indices.admit_one(node_count).map_err(|_| b"fem3d.numerical-solid-node-index-admission".to_vec())? => self.point_cursor += 1,
                    2 if model.admit_node_one(model.nodes_len() + node_count, MOUNTED_ANALYSIS_BACKING_BYTES).map_err(|_| b"fem3d.numerical-solid-node-admission".to_vec())?.complete => self.point_cursor += 1,
                    3 if self.analysis_node_ids.admit_one(self.analysis_node_ids.len() + node_count).map_err(|_| b"fem3d.numerical-solid-analysis-id-admission".to_vec())? => self.point_cursor += 1,
                    4 if model.admit_element_one(model.elements_len() + tet_count, MOUNTED_ANALYSIS_BACKING_BYTES).map_err(|_| b"fem3d.numerical-solid-tet-admission".to_vec())?.complete => self.point_cursor += 1,
                    _ => {
                        if self.point_cursor < 5 {
                            return Ok(false);
                        }
                        self.volume_node_cursor = 0;
                        self.lookup_cursor = 0;
                        self.point_cursor = 0;
                        self.stage = Fem3dNumericalStage::SolidNodeLookup;
                        return Ok(false);
                    }
                }
            }
            Fem3dNumericalStage::SolidNodeLookup => {
                let solid = doc.solids.get(self.solid_cursor).ok_or_else(|| b"fem3d.numerical-solid".to_vec())?;
                let total = self.solid_points.len().checked_mul(solid.layers + 1).ok_or_else(|| b"fem3d.numerical-solid-node-overflow".to_vec())?;
                if self.volume_node_cursor == total {
                    self.point_cursor = 0;
                    self.tet_cursor = 0;
                    self.tet_phase = 0;
                    self.stage = Fem3dNumericalStage::SolidTet;
                } else if let Some(node) = self.model.as_ref().and_then(|model| model.node(self.lookup_cursor)) {
                    let point = self.solid_point(doc, self.volume_node_cursor)?;
                    if (node.pos[0] - point[0]).abs() < 1e-9 && (node.pos[1] - point[1]).abs() < 1e-9 && (node.pos[2] - point[2]).abs() < 1e-9 {
                        self.pending_node_id = Some(node.id.clone());
                        self.pending_node_index = self.lookup_cursor;
                        self.pending_node_needs_analysis = false;
                        self.lookup_cursor = 0;
                        self.stage = Fem3dNumericalStage::SolidNodeId;
                    } else {
                        self.lookup_cursor += 1;
                    }
                } else {
                    self.lookup_cursor = 0;
                    self.stage = Fem3dNumericalStage::SolidNodeCreate;
                }
            }
            Fem3dNumericalStage::SolidNodeCreate => {
                let solid = doc.solids.get(self.solid_cursor).ok_or_else(|| b"fem3d.numerical-solid".to_vec())?;
                let point = self.solid_point(doc, self.volume_node_cursor)?;
                let id = format!("{}_m{}", solid.id, self.volume_node_cursor);
                if id.len() > INPUT_BYTES {
                    return Err(b"fem3d.numerical-solid-node-id".to_vec());
                }
                let model = self.model.as_mut().ok_or_else(|| b"fem3d.numerical-model-owner".to_vec())?;
                self.pending_node_index = model.nodes_len();
                model.push_node(Node { id: id.clone(), pos: point }).map_err(|_| b"fem3d.numerical-solid-node-slot".to_vec())?;
                self.pending_node_id = Some(id);
                self.pending_node_needs_analysis = true;
                self.stage = Fem3dNumericalStage::SolidNodeId;
            }
            Fem3dNumericalStage::SolidNodeId => {
                let id = self.pending_node_id.as_ref().ok_or_else(|| b"fem3d.numerical-solid-node-owner".to_vec())?;
                match self.point_cursor {
                    0 => {
                        if self.pending_node_needs_analysis {
                            self.analysis_node_ids.push(id.clone()).map_err(|_| b"fem3d.numerical-solid-analysis-id-slot".to_vec())?;
                        }
                        self.point_cursor = 1;
                    }
                    1 => {
                        self.solid_node_ids.push(id.clone()).map_err(|_| b"fem3d.numerical-solid-node-id-slot".to_vec())?;
                        self.point_cursor = 2;
                    }
                    _ => {
                        self.solid_node_analysis_indices.push(self.pending_node_index).map_err(|_| b"fem3d.numerical-solid-node-index-slot".to_vec())?;
                        self.pending_node_id = None;
                        self.point_cursor = 0;
                        self.volume_node_cursor += 1;
                        self.lookup_cursor = 0;
                        self.stage = Fem3dNumericalStage::SolidNodeLookup;
                    }
                }
            }
            Fem3dNumericalStage::SolidTet => {
                let solid = doc.solids.get(self.solid_cursor).ok_or_else(|| b"fem3d.numerical-solid".to_vec())?;
                if self.point_cursor == solid.layers {
                    self.stage = Fem3dNumericalStage::SolidIndicesRetire;
                } else if self.tet_cursor == self.solid_tris.len() {
                    self.point_cursor += 1;
                    self.tet_cursor = 0;
                    self.tet_phase = 0;
                } else {
                    let material = doc.materials.get(self.resolved_solid_material).ok_or_else(|| b"fem3d.numerical-solid-material".to_vec())?;
                    let indices = self.solid_tet_indices()?;
                    let nodes = [
                        self.solid_node_ids.get(indices[0]).cloned().ok_or_else(|| b"fem3d.numerical-solid-node-id".to_vec())?,
                        self.solid_node_ids.get(indices[1]).cloned().ok_or_else(|| b"fem3d.numerical-solid-node-id".to_vec())?,
                        self.solid_node_ids.get(indices[2]).cloned().ok_or_else(|| b"fem3d.numerical-solid-node-id".to_vec())?,
                        self.solid_node_ids.get(indices[3]).cloned().ok_or_else(|| b"fem3d.numerical-solid-node-id".to_vec())?,
                    ];
                    let positions = [self.solid_point(doc, indices[0])?, self.solid_point(doc, indices[1])?, self.solid_point(doc, indices[2])?, self.solid_point(doc, indices[3])?];
                    let a = [positions[1][0] - positions[0][0], positions[1][1] - positions[0][1], positions[1][2] - positions[0][2]];
                    let b = [positions[2][0] - positions[0][0], positions[2][1] - positions[0][1], positions[2][2] - positions[0][2]];
                    let c = [positions[3][0] - positions[0][0], positions[3][1] - positions[0][1], positions[3][2] - positions[0][2]];
                    let volume = (a[0] * (b[1] * c[2] - b[2] * c[1]) - a[1] * (b[0] * c[2] - b[2] * c[0]) + a[2] * (b[0] * c[1] - b[1] * c[0])).abs() / 6.0;
                    self.pending_tet_mass = material.rho * volume / 4.0;
                    self.pending_tet_indices = indices;
                    self.pending_tet = Some(Tet4 { id: format!("{}_c{}_{}", solid.id, self.point_cursor * self.solid_tris.len() + self.tet_cursor, self.tet_phase), nodes, e: material.e, nu: material.nu, density: material.rho }.into());
                    self.mass_update_cursor = 0;
                    self.stage = Fem3dNumericalStage::SolidTetMass;
                }
            }
            Fem3dNumericalStage::SolidTetMass => {
                if self.mass_update_cursor < 12 {
                    let index = self.pending_tet_indices[self.mass_update_cursor / 3];
                    let analysis = *self.solid_node_analysis_indices.get(index).ok_or_else(|| b"fem3d.numerical-solid-node-index".to_vec())?;
                    self.modal_lumped_mass[analysis * 6 + self.mass_update_cursor % 3] += self.pending_tet_mass;
                    self.mass_update_cursor += 1;
                } else {
                    self.stage = Fem3dNumericalStage::SolidTetCommit;
                }
            }
            Fem3dNumericalStage::SolidTetCommit => {
                let tet = self.pending_tet.take().ok_or_else(|| b"fem3d.numerical-solid-tet-owner".to_vec())?;
                if let Err(tet) = self.model.as_mut().ok_or_else(|| b"fem3d.numerical-model-owner".to_vec())?.push_element(tet) {
                    self.pending_tet = Some(tet);
                    return Err(b"fem3d.numerical-solid-tet-slot".to_vec());
                }
                self.tet_phase += 1;
                if self.tet_phase == 3 {
                    self.tet_phase = 0;
                    self.tet_cursor += 1;
                }
                self.stage = Fem3dNumericalStage::SolidTet;
            }
            Fem3dNumericalStage::SolidIndicesRetire => {
                if self.solid_node_analysis_indices.pop().is_some() {
                    return Ok(false);
                }
                if !self.solid_node_analysis_indices.close_admission_one() {
                    return Ok(false);
                }
                self.stage = Fem3dNumericalStage::SolidCommit;
            }
            Fem3dNumericalStage::SolidCommit => {
                let solid = doc.solids.get(self.solid_cursor).ok_or_else(|| b"fem3d.numerical-solid".to_vec())?;
                let retained = Fem3dMeshedSolid {
                    solid_index: self.solid_cursor,
                    node_ids: std::mem::replace(&mut self.solid_node_ids, FixedSlots::new()),
                    top_offset: solid.layers * self.solid_points.len(),
                    points: std::mem::replace(&mut self.solid_points, FixedSlots::new()),
                    tris: std::mem::replace(&mut self.solid_tris, FixedSlots::new()),
                };
                if let Err(retained) = self.meshed_solids.push(retained) {
                    self.solid_node_ids = retained.node_ids;
                    self.solid_points = retained.points;
                    self.solid_tris = retained.tris;
                    return Err(b"fem3d.numerical-meshed-solid-slot".to_vec());
                }
                self.solid_materials[self.solid_cursor] = self.resolved_solid_material;
                self.solid_cursor += 1;
                self.stage = Fem3dNumericalStage::SolidDomainOuterReserve;
            }
            Fem3dNumericalStage::SupportDofReserve => {
                let Some(support) = doc.supports.get(self.support_cursor) else {
                    self.stage = Fem3dNumericalStage::PublishFieldCount;
                    return Ok(false);
                };
                if support.fixed.len() > 6 {
                    return Err(b"fem3d.numerical-support-dof-capacity".to_vec());
                }
                self.pending_support = Some(MountedAnalysisSupport::new(support.node_id.clone()));
                self.stage = Fem3dNumericalStage::SupportDof;
            }
            Fem3dNumericalStage::SupportDof => {
                let support = doc.supports.get(self.support_cursor).ok_or_else(|| b"fem3d.numerical-support".to_vec())?;
                if let Some(dof) = support.fixed.get(self.dof_cursor) {
                    self.pending_support.as_mut().ok_or_else(|| b"fem3d.numerical-support-owner".to_vec())?.push_fixed(Dof::from(*dof)).map_err(|_| b"fem3d.numerical-support-dof-slot".to_vec())?;
                    self.dof_cursor += 1;
                } else {
                    self.stage = Fem3dNumericalStage::SupportCommit;
                }
            }
            Fem3dNumericalStage::SupportCommit => {
                let support = self.pending_support.take().ok_or_else(|| b"fem3d.numerical-support-owner".to_vec())?;
                if let Err(support) = self.model.as_mut().ok_or_else(|| b"fem3d.numerical-model-owner".to_vec())?.push_support(support) {
                    self.pending_support = Some(support);
                    return Err(b"fem3d.numerical-support-slot".to_vec());
                }
                self.support_cursor += 1;
                self.dof_cursor = 0;
                self.stage = Fem3dNumericalStage::SupportDofReserve;
            }
            _ => return Ok(true),
        }
        Ok(false)
    }

}
//#endregion 📦️PageVisualJob
