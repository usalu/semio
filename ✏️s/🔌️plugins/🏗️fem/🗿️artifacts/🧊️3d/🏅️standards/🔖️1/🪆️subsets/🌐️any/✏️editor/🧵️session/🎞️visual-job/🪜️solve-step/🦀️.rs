//! 🪜️ Numerical child, solve half: the staged solve step, the interactive close and the close ladder.

use super::*;

//#region 📦️PageVisualJob
impl Fem3dNumericalChild {
    pub fn step(&mut self, doc: &Fem3dSnapshot, solver: &mut Fem3dSolverView, backing: &mut Fem3dBackingCredit, freshness: Fem3dVisualFreshness, operation: semio_framework_job::Operation, context: &mut StepContext<'_>) -> Result<bool, Fem3dNumericalFailure> {
        context.set_stage(self.stage.progress_stage());
        self.operation = Some(operation);
        let delegated = matches!(self.stage, Fem3dNumericalStage::SolidMesh | Fem3dNumericalStage::Assembly | Fem3dNumericalStage::Pcg | Fem3dNumericalStage::Ldlt | Fem3dNumericalStage::Subspace);
        if !delegated {
            if context.is_cancelled() {
                return Err((b"fem3d.numerical-cancelled".to_vec()).into());
            }
            if context.should_yield() {
                return Ok(false);
            }
            context.consume_fuel(1);
            if context.is_cancelled() {
                return Err((b"fem3d.numerical-cancelled".to_vec()).into());
            }
        }
        if self.stage == Fem3dNumericalStage::ReserveSolverPages {
            if self.solver_page_cursor == FEM3D_SOLVER_PAGE_COUNT {
                self.stage = Fem3dNumericalStage::ReserveNodes;
            } else {
                if !solver.admit_page(self.solver_page_cursor, self.solver_page_lane, backing) {
                    return Err((b"fem3d.numerical-solver-page-admission".to_vec()).into());
                }
                if self.solver_page_lane {
                    self.solver_page_cursor += 1;
                }
                self.solver_page_lane = !self.solver_page_lane;
            }
            return Ok(false);
        }
        if self.stage == Fem3dNumericalStage::SolidMeshBegin {
            let solid = doc.solids.get(self.solid_cursor).ok_or_else(|| b"fem3d.numerical-solid".to_vec())?;
            let layers = solid.layers.max(1);
            let model = self.model.as_ref().ok_or_else(|| b"fem3d.numerical-model-owner".to_vec())?;
            let maximum_points = MAXIMUM_FIELDS.saturating_sub(model.nodes_len()) / (layers + 1);
            let maximum_triangles = MAXIMUM_ELEMENTS.saturating_sub(model.elements_len()) / (layers * 3);
            if maximum_points < 3 || maximum_triangles == 0 {
                return Err((b"fem3d.numerical-solid-admission".to_vec()).into());
            }
            let domain = self.solid_domain.take().ok_or_else(|| b"fem3d.numerical-solid-domain-owner".to_vec())?;
            self.mesh = Some(MeshJob::new_mounted_bounded(domain, MeshOpts { max_edge: solid.mesh_size, min_angle_deg: 20.0 }, operation, maximum_points, maximum_triangles));
            self.stage = Fem3dNumericalStage::SolidMesh;
            return Ok(false);
        }
        if self.stage == Fem3dNumericalStage::SolidMesh {
            let end = child_end(self.mesh.as_mut().ok_or_else(|| b"fem3d.numerical-solid-mesh-child".to_vec())?.step(context))?;
            if self.observe_child_end(end)? {
                self.point_cursor = 0;
                self.stage = Fem3dNumericalStage::SolidMeshReservePoints;
            }
            return Ok(false);
        }
        if self.stage == Fem3dNumericalStage::SolidMeshReservePoints {
            let (points, _) = self.mesh.as_ref().and_then(MeshJob::completed_counts).ok_or_else(|| b"fem3d.numerical-solid-mesh-false-terminal".to_vec())?;
            if self.solid_points.admit_one(points).map_err(|_| b"fem3d.numerical-solid-mesh-point-admission".to_vec())? {
                self.point_cursor = 0;
                self.stage = Fem3dNumericalStage::SolidMeshCopyPoint;
            }
            return Ok(false);
        }
        if self.stage == Fem3dNumericalStage::SolidMeshCopyPoint {
            let (points, _) = self.mesh.as_ref().and_then(MeshJob::completed_counts).ok_or_else(|| b"fem3d.numerical-solid-mesh-false-terminal".to_vec())?;
            if self.point_cursor < points {
                let point = self.mesh.as_ref().and_then(|mesh| mesh.completed_point(self.point_cursor)).ok_or_else(|| b"fem3d.numerical-solid-mesh-point".to_vec())?;
                self.solid_points.push(point).map_err(|_| b"fem3d.numerical-solid-mesh-point-slot".to_vec())?;
                self.point_cursor += 1;
            } else {
                self.stage = Fem3dNumericalStage::SolidMeshReserveTriangles;
            }
            return Ok(false);
        }
        if self.stage == Fem3dNumericalStage::SolidMeshReserveTriangles {
            let (_, triangles) = self.mesh.as_ref().and_then(MeshJob::completed_counts).ok_or_else(|| b"fem3d.numerical-solid-mesh-false-terminal".to_vec())?;
            if self.solid_tris.admit_one(triangles).map_err(|_| b"fem3d.numerical-solid-mesh-triangle-admission".to_vec())? {
                self.point_cursor = 0;
                self.stage = Fem3dNumericalStage::SolidMeshCopyTriangle;
            }
            return Ok(false);
        }
        if self.stage == Fem3dNumericalStage::SolidMeshCopyTriangle {
            let (_, triangles) = self.mesh.as_ref().and_then(MeshJob::completed_counts).ok_or_else(|| b"fem3d.numerical-solid-mesh-false-terminal".to_vec())?;
            if self.point_cursor < triangles {
                let triangle = self.mesh.as_ref().and_then(|mesh| mesh.completed_triangle(self.point_cursor)).ok_or_else(|| b"fem3d.numerical-solid-mesh-triangle".to_vec())?;
                self.solid_tris.push(triangle).map_err(|_| b"fem3d.numerical-solid-mesh-triangle-slot".to_vec())?;
                self.point_cursor += 1;
            } else {
                self.stage = Fem3dNumericalStage::SolidMeshRetire;
            }
            return Ok(false);
        }
        if self.stage == Fem3dNumericalStage::SolidMeshRetire {
            let mesh = self.mesh.as_mut().ok_or_else(|| b"fem3d.numerical-solid-mesh-child".to_vec())?;
            InteractiveJob::close_step(mesh, context).map_err(|_| b"fem3d.numerical-solid-mesh-retire-refused".to_vec())?;
            semio_framework_job::close_original_retained_step(context, RetirementDemand { depth: 1, ..Default::default() }, |_| {
                self.mesh = None;
                self.stage = Fem3dNumericalStage::SolidMaterial;
                Ok(semio_framework_job::RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..Default::default() }))
            })?;
            return Ok(false);
        }
        if matches!(
            self.stage,
            Fem3dNumericalStage::ReserveNodes
                | Fem3dNumericalStage::ReserveNodeIds
                | Fem3dNumericalStage::ReserveElements
                | Fem3dNumericalStage::ReserveSupports
                | Fem3dNumericalStage::ReserveMeshedSolids
                | Fem3dNumericalStage::Nodes
                | Fem3dNumericalStage::NodeId
                | Fem3dNumericalStage::ElementMaterial
                | Fem3dNumericalStage::ElementSection
                | Fem3dNumericalStage::ElementStart
                | Fem3dNumericalStage::ElementEnd
                | Fem3dNumericalStage::ElementCommit
                | Fem3dNumericalStage::ElementMass
                | Fem3dNumericalStage::ElementInsert
                | Fem3dNumericalStage::SolidDomainOuterReserve
                | Fem3dNumericalStage::SolidDomainOuterPoint
                | Fem3dNumericalStage::SolidDomainHolesReserve
                | Fem3dNumericalStage::SolidDomainHoleReserve
                | Fem3dNumericalStage::SolidDomainHolePoint
                | Fem3dNumericalStage::SolidMaterial
                | Fem3dNumericalStage::SolidOwnersReserve
                | Fem3dNumericalStage::SolidNodeLookup
                | Fem3dNumericalStage::SolidNodeCreate
                | Fem3dNumericalStage::SolidNodeId
                | Fem3dNumericalStage::SolidTet
                | Fem3dNumericalStage::SolidTetMass
                | Fem3dNumericalStage::SolidTetCommit
                | Fem3dNumericalStage::SolidIndicesRetire
                | Fem3dNumericalStage::SolidCommit
                | Fem3dNumericalStage::SupportDofReserve
                | Fem3dNumericalStage::SupportDof
                | Fem3dNumericalStage::SupportCommit
        ) {
            self.step_model(doc)?;
            return Ok(false);
        }
        match self.stage {
            Fem3dNumericalStage::PublishFieldCount => {
                if !solver.set_len(freshness, self.analysis_node_ids.len()) {
                    return Err((b"fem3d.numerical-field-count".to_vec()).into());
                }
                self.stage = Fem3dNumericalStage::MountAssembly;
            }
            Fem3dNumericalStage::MountAssembly => {
                let model = self.model.take().ok_or_else(|| b"fem3d.numerical-model-owner".to_vec())?;
                self.assembly_build = Some(AssemblyJobConstruction::new_mounted(model, operation, 1));
                self.stage = Fem3dNumericalStage::PrepareAssembly;
            }
            Fem3dNumericalStage::PrepareAssembly => match self.assembly_build.as_mut().ok_or_else(|| b"fem3d.numerical-assembly-build".to_vec())?.step_one_funded(context.retained_grant()).and_then(|(complete, progress)| {
                context.consume_retained(progress).map_err(|_| crate::model::FemError::ResourceAdmission)?;
                Ok(complete)
            }) {
                Ok(false) => {}
                Ok(true) => {
                    self.assembly = self.assembly_build.as_mut().and_then(AssemblyJobConstruction::take_complete);
                    if self.assembly.is_none() {
                        return Err((b"fem3d.numerical-assembly-false-terminal".to_vec()).into());
                    }
                    self.stage = Fem3dNumericalStage::Assembly;
                }
                Err(error) => return Err((error.to_string().into_bytes()).into()),
            },
            Fem3dNumericalStage::Assembly => {
                let end = child_end(self.assembly.as_mut().ok_or_else(|| b"fem3d.numerical-assembly".to_vec())?.step(context))?;
                if self.observe_child_end(end)? {
                    self.node_cursor = 0;
                    self.dof_cursor = 0;
                    self.free_order = self.assembly.as_ref().map_or(0, AssemblyJob::visual_free_order);
                    self.stage = Fem3dNumericalStage::ReserveModalMass;
                }
                return Ok(false);
            }
            Fem3dNumericalStage::ReserveModalMass => {
                if self.modal_free_mass.admit_one(self.free_order).map_err(|_| b"fem3d.numerical-modal-mass-admission".to_vec())? {
                    self.stage = Fem3dNumericalStage::InitializeModalMass;
                }
            }
            Fem3dNumericalStage::InitializeModalMass => {
                if self.modal_free_mass.len() < self.free_order {
                    self.modal_free_mass.push(0.0).map_err(|_| b"fem3d.numerical-modal-mass-slot".to_vec())?;
                } else {
                    self.node_cursor = 0;
                    self.dof_cursor = 0;
                    self.stage = Fem3dNumericalStage::MapEquation;
                }
            }
            Fem3dNumericalStage::MapEquation => {
                if self.node_cursor == self.analysis_node_ids.len() {
                    self.stage = Fem3dNumericalStage::ReserveRhs;
                } else if self.dof_cursor == 6 {
                    self.node_cursor += 1;
                    self.dof_cursor = 0;
                } else {
                    let dof = [Dof::Tx, Dof::Ty, Dof::Tz, Dof::Rx, Dof::Ry, Dof::Rz][self.dof_cursor];
                    if let Some((full, compact)) = self.assembly.as_ref().and_then(|assembly| self.analysis_node_ids.get(self.node_cursor).and_then(|node_id| assembly.visual_equation_indices(node_id, dof))) {
                        self.full_equations[self.node_cursor][self.dof_cursor] = Some(full);
                        self.equations[self.node_cursor][self.dof_cursor] = compact;
                        if let Some(compact) = compact {
                            self.modal_free_mass.add_at(compact, self.modal_lumped_mass[self.node_cursor * 6 + self.dof_cursor]).map_err(|_| b"fem3d.numerical-modal-mass-index".to_vec())?;
                        }
                    }
                    self.dof_cursor += 1;
                }
            }
            Fem3dNumericalStage::ReserveRhs => {
                if self.rhs.admit_one(self.free_order).map_err(|_| b"fem3d.numerical-rhs-admission".to_vec())? {
                    self.stage = Fem3dNumericalStage::InitializeRhs;
                }
            }
            Fem3dNumericalStage::InitializeRhs => {
                if self.rhs.len() < self.free_order {
                    self.rhs.push(0.0).map_err(|_| b"fem3d.numerical-rhs-slot".to_vec())?;
                } else {
                    self.node_cursor = 0;
                    self.stage = Fem3dNumericalStage::ResolveSourceCase;
                }
            }
            Fem3dNumericalStage::ResolveSourceCase => {
                if self.source_case >= doc.load_cases.len() {
                    let assembly = self.assembly.take().ok_or_else(|| b"fem3d.numerical-assembly-owner".to_vec())?;
                    self.csr_build = Some(AssemblyCsrBuild::new_free(assembly).map_err(|_| b"fem3d.numerical-assembly-terminal".to_vec())?);
                    self.stage = Fem3dNumericalStage::BuildCsr;
                } else {
                    self.source_factor = match self.request.source {
                        Fem3dResultSource::FirstCase => if self.source_case == 0 { 1.0 } else { 0.0 },
                        Fem3dResultSource::Case(index) => if self.source_case == index { 1.0 } else { 0.0 },
                        Fem3dResultSource::Combination(index) => *doc.combinations.get(index).ok_or_else(|| b"fem3d.numerical-combination-missing".to_vec())?.terms.get(&doc.load_cases[self.source_case].id).unwrap_or(&0.0),
                    };
                    if !self.source_factor.is_finite() { return Err((b"fem3d.numerical-source-factor".to_vec()).into()); }
                    if self.source_factor == 0.0 {
                        self.source_case += 1;
                    } else {
                        self.load_cursor = 0;
                        self.stage = Fem3dNumericalStage::ApplyLoad;
                    }
                }
            }
            Fem3dNumericalStage::ApplyLoad => {
                let Some(case) = doc.load_cases.get(self.source_case) else { return Err((b"fem3d.numerical-load-case".to_vec()).into()) };
                let Some(load) = case.loads.get(self.load_cursor) else {
                    if case.self_weight {
                        self.load_element_cursor = 0;
                        self.load_triangle_cursor = 0;
                        self.load_node_cursor = 0;
                        self.stage = Fem3dNumericalStage::ResolveSelfWeightMemberNode;
                        return Ok(false);
                    }
                    self.source_case += 1;
                    self.stage = Fem3dNumericalStage::ResolveSourceCase;
                    return Ok(false);
                };
                self.load_node_cursor = 0;
                self.stage = match load {
                    FemLoad::Nodal { .. } => Fem3dNumericalStage::ApplyNodalLoad,
                    FemLoad::MemberUdl { .. } => {
                        self.load_element_cursor = 0;
                        Fem3dNumericalStage::ResolveMemberLoad
                    }
                    FemLoad::Area { .. } => {
                        self.load_solid_cursor = 0;
                        Fem3dNumericalStage::ResolveAreaLoad
                    }
                };
            }
            Fem3dNumericalStage::ApplyNodalLoad => {
                let case = doc.load_cases.get(self.source_case).ok_or_else(|| b"fem3d.numerical-load-case".to_vec())?;
                let FemLoad::Nodal { node_id, dof, value, .. } = case.loads.get(self.load_cursor).ok_or_else(|| b"fem3d.numerical-load".to_vec())? else {
                    return Err((b"fem3d.numerical-load-kind".to_vec()).into());
                };
                let Some(candidate) = self.analysis_node_ids.get(self.load_node_cursor) else { return Err((b"fem3d.numerical-load-node".to_vec()).into()) };
                if candidate == node_id {
                    let axis = Dof::from(*dof).index();
                    self.apply_rhs_value(self.load_node_cursor, axis, *value)?;
                    self.load_cursor += 1;
                    self.load_node_cursor = 0;
                    self.stage = Fem3dNumericalStage::ApplyLoad;
                } else {
                    self.load_node_cursor += 1;
                }
            }
            Fem3dNumericalStage::ResolveMemberLoad => {
                let case = doc.load_cases.get(self.source_case).ok_or_else(|| b"fem3d.numerical-load-case".to_vec())?;
                let FemLoad::MemberUdl { element_id, .. } = case.loads.get(self.load_cursor).ok_or_else(|| b"fem3d.numerical-load".to_vec())? else {
                    return Err((b"fem3d.numerical-load-kind".to_vec()).into());
                };
                let Some(element) = doc.elements.get(self.load_element_cursor) else { return Err((b"fem3d.numerical-member-load-element".to_vec()).into()) };
                if semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::structure::element_id(element) == element_id {
                    self.load_triangle_cursor = 0;
                    self.load_node_cursor = 0;
                    self.stage = Fem3dNumericalStage::ResolveMemberNode;
                } else {
                    self.load_element_cursor += 1;
                }
            }
            Fem3dNumericalStage::ResolveMemberNode => {
                let element = doc.elements.get(self.load_element_cursor).ok_or_else(|| b"fem3d.numerical-member-load-element".to_vec())?;
                let (start, end) = match element {
                    FemElement::Bar { start, end, .. } | FemElement::Frame { start, end, .. } => (start, end),
                };
                let target = if self.load_triangle_cursor == 0 { start } else { end };
                let Some(node) = doc.nodes.get(self.load_node_cursor) else { return Err((b"fem3d.numerical-member-load-node".to_vec()).into()) };
                if &node.id == target {
                    self.load_node_indices[self.load_triangle_cursor] = self.load_node_cursor;
                    self.load_positions[self.load_triangle_cursor] = [node.x, node.y, node.z];
                    self.load_triangle_cursor += 1;
                    self.load_node_cursor = 0;
                    if self.load_triangle_cursor == 2 {
                        self.load_vertex_cursor = 0;
                        self.stage = Fem3dNumericalStage::ApplyMemberScalar;
                    }
                } else {
                    self.load_node_cursor += 1;
                }
            }
            Fem3dNumericalStage::ApplyMemberScalar => {
                let case = doc.load_cases.get(self.source_case).ok_or_else(|| b"fem3d.numerical-load-case".to_vec())?;
                let FemLoad::MemberUdl { wx, wy, wz, .. } = case.loads.get(self.load_cursor).ok_or_else(|| b"fem3d.numerical-load".to_vec())? else {
                    return Err((b"fem3d.numerical-load-kind".to_vec()).into());
                };
                if self.load_vertex_cursor == 12 {
                    self.load_cursor += 1;
                    self.load_element_cursor = 0;
                    self.stage = Fem3dNumericalStage::ApplyLoad;
                } else {
                    let endpoint = self.load_vertex_cursor / 6;
                    let axis = self.load_vertex_cursor % 6;
                    let d = [self.load_positions[1][0] - self.load_positions[0][0], self.load_positions[1][1] - self.load_positions[0][1], self.load_positions[1][2] - self.load_positions[0][2]];
                    let length = (d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt();
                    if length <= f64::EPSILON {
                        return Err((b"fem3d.numerical-member-load-length".to_vec()).into());
                    }
                    let force = [*wx * length * 0.5, *wy * length * 0.5, *wz * length * 0.5];
                    let moment = [(d[1] * *wz - d[2] * *wy) * length / 12.0, (d[2] * *wx - d[0] * *wz) * length / 12.0, (d[0] * *wy - d[1] * *wx) * length / 12.0];
                    let value = if axis < 3 {
                        force[axis]
                    } else if endpoint == 0 {
                        moment[axis - 3]
                    } else {
                        -moment[axis - 3]
                    };
                    self.apply_rhs_value(self.load_node_indices[endpoint], axis, value)?;
                    self.load_vertex_cursor += 1;
                }
            }
            Fem3dNumericalStage::ResolveAreaLoad => {
                let case = doc.load_cases.get(self.source_case).ok_or_else(|| b"fem3d.numerical-load-case".to_vec())?;
                let FemLoad::Area { solid_id, .. } = case.loads.get(self.load_cursor).ok_or_else(|| b"fem3d.numerical-load".to_vec())? else {
                    return Err((b"fem3d.numerical-load-kind".to_vec()).into());
                };
                let Some(solid) = self.meshed_solids.get(self.load_solid_cursor) else { return Err((b"fem3d.numerical-area-load-solid".to_vec()).into()) };
                if doc.solids.get(solid.solid_index).is_some_and(|candidate| &candidate.id == solid_id) {
                    self.load_triangle_cursor = 0;
                    self.load_vertex_cursor = 0;
                    self.load_node_cursor = 0;
                    self.stage = Fem3dNumericalStage::ApplyAreaNode;
                } else {
                    self.load_solid_cursor += 1;
                }
            }
            Fem3dNumericalStage::ApplyAreaNode => {
                let case = doc.load_cases.get(self.source_case).ok_or_else(|| b"fem3d.numerical-load-case".to_vec())?;
                let FemLoad::Area { pressure, .. } = case.loads.get(self.load_cursor).ok_or_else(|| b"fem3d.numerical-load".to_vec())? else {
                    return Err((b"fem3d.numerical-load-kind".to_vec()).into());
                };
                let solid = self.meshed_solids.get(self.load_solid_cursor).ok_or_else(|| b"fem3d.numerical-area-load-solid".to_vec())?;
                let Some(triangle) = solid.tris.get(self.load_triangle_cursor) else {
                    self.load_cursor += 1;
                    self.load_solid_cursor = 0;
                    self.stage = Fem3dNumericalStage::ApplyLoad;
                    return Ok(false);
                };
                let p0 = *solid.points.get(triangle[0] as usize).ok_or_else(|| b"fem3d.numerical-area-load-point".to_vec())?;
                let p1 = *solid.points.get(triangle[1] as usize).ok_or_else(|| b"fem3d.numerical-area-load-point".to_vec())?;
                let p2 = *solid.points.get(triangle[2] as usize).ok_or_else(|| b"fem3d.numerical-area-load-point".to_vec())?;
                let source = doc.solids.get(solid.solid_index).ok_or_else(|| b"fem3d.numerical-area-load-source".to_vec())?;
                let [p0, p1, p2] = [p0, p1, p2].map(|point| source.axis.to_world(point[0], point[1], source.base_z + source.height));
                let area = (0.5 * ((p1[0] - p0[0]) * (p2[1] - p0[1]) - (p2[0] - p0[0]) * (p1[1] - p0[1]))).abs();
                let point = triangle[self.load_vertex_cursor] as usize;
                let node_id = solid.node_ids.get(solid.top_offset + point).ok_or_else(|| b"fem3d.numerical-area-load-node".to_vec())?;
                let Some(candidate) = self.analysis_node_ids.get(self.load_node_cursor) else { return Err((b"fem3d.numerical-area-load-node".to_vec()).into()) };
                if candidate == node_id {
                    self.apply_rhs_value(self.load_node_cursor, Dof::Tz.index(), -*pressure * area / 3.0)?;
                    self.load_node_cursor = 0;
                    self.load_vertex_cursor += 1;
                    if self.load_vertex_cursor == 3 {
                        self.load_vertex_cursor = 0;
                        self.load_triangle_cursor += 1;
                    }
                } else {
                    self.load_node_cursor += 1;
                }
            }
            Fem3dNumericalStage::ResolveSelfWeightMemberNode => {
                let Some(element) = doc.elements.get(self.load_element_cursor) else {
                    self.load_solid_cursor = 0;
                    self.load_triangle_cursor = 0;
                    self.load_vertex_cursor = 0;
                    self.load_node_cursor = 0;
                    self.point_cursor = 0;
                    self.tet_phase = 0;
                    self.stage = Fem3dNumericalStage::ApplySelfWeightSolid;
                    return Ok(false);
                };
                let (start, end) = match element {
                    FemElement::Bar { start, end, .. } | FemElement::Frame { start, end, .. } => (start, end),
                };
                let target = if self.load_triangle_cursor == 0 { start } else { end };
                let Some(node) = doc.nodes.get(self.load_node_cursor) else { return Err((b"fem3d.numerical-self-weight-node".to_vec()).into()) };
                if &node.id == target {
                    self.load_node_indices[self.load_triangle_cursor] = self.load_node_cursor;
                    self.load_positions[self.load_triangle_cursor] = [node.x, node.y, node.z];
                    self.load_triangle_cursor += 1;
                    self.load_node_cursor = 0;
                    if self.load_triangle_cursor == 2 {
                        self.load_vertex_cursor = 0;
                        self.stage = Fem3dNumericalStage::ApplySelfWeightMember;
                    }
                } else {
                    self.load_node_cursor += 1;
                }
            }
            Fem3dNumericalStage::ApplySelfWeightMember => {
                if self.load_vertex_cursor == 2 {
                    self.load_element_cursor += 1;
                    self.load_triangle_cursor = 0;
                    self.load_node_cursor = 0;
                    self.stage = Fem3dNumericalStage::ResolveSelfWeightMemberNode;
                } else {
                    let material = doc.materials.get(self.element_materials[self.load_element_cursor]).ok_or_else(|| b"fem3d.numerical-self-weight-material".to_vec())?;
                    let section = doc.sections.get(self.element_sections[self.load_element_cursor]).ok_or_else(|| b"fem3d.numerical-self-weight-section".to_vec())?;
                    let d = [self.load_positions[1][0] - self.load_positions[0][0], self.load_positions[1][1] - self.load_positions[0][1], self.load_positions[1][2] - self.load_positions[0][2]];
                    let length = (d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt();
                    self.apply_rhs_value(self.load_node_indices[self.load_vertex_cursor], Dof::Tz.index(), -material.rho * section.area * length * 9.81 * 0.5)?;
                    self.load_vertex_cursor += 1;
                }
            }
            Fem3dNumericalStage::ApplySelfWeightSolid => {
                let Some(meshed) = self.meshed_solids.get(self.load_solid_cursor) else {
                    self.source_case += 1;
                    self.stage = Fem3dNumericalStage::ResolveSourceCase;
                    return Ok(false);
                };
                let solid = doc.solids.get(meshed.solid_index).ok_or_else(|| b"fem3d.numerical-solid".to_vec())?;
                if self.point_cursor == solid.layers {
                    self.load_solid_cursor += 1;
                    self.load_triangle_cursor = 0;
                    self.load_vertex_cursor = 0;
                    self.load_node_cursor = 0;
                    self.point_cursor = 0;
                    self.tet_phase = 0;
                    return Ok(false);
                }
                if self.load_triangle_cursor == meshed.tris.len() {
                    self.point_cursor += 1;
                    self.load_triangle_cursor = 0;
                    self.load_vertex_cursor = 0;
                    self.tet_phase = 0;
                    return Ok(false);
                }
                if self.tet_phase == 3 {
                    self.load_triangle_cursor += 1;
                    self.load_vertex_cursor = 0;
                    self.tet_phase = 0;
                    return Ok(false);
                }
                if self.load_vertex_cursor == 4 {
                    self.tet_phase += 1;
                    self.load_vertex_cursor = 0;
                    self.load_node_cursor = 0;
                    return Ok(false);
                }
                let triangle = *meshed.tris.get(self.load_triangle_cursor).ok_or_else(|| b"fem3d.numerical-self-weight-solid-triangle".to_vec())?;
                let indices = Self::tet_indices(meshed.points.len(), triangle, self.point_cursor, self.tet_phase);
                let node_id = meshed.node_ids.get(indices[self.load_vertex_cursor]).ok_or_else(|| b"fem3d.numerical-self-weight-solid-node".to_vec())?;
                let Some(candidate) = self.analysis_node_ids.get(self.load_node_cursor) else { return Err((b"fem3d.numerical-self-weight-solid-node".to_vec()).into()) };
                if candidate == node_id {
                    let position = |index: usize| -> Result<[f64; 3], Vec<u8>> {
                        let layer = index / meshed.points.len();
                        let point = *meshed.points.get(index % meshed.points.len()).ok_or_else(|| b"fem3d.numerical-self-weight-solid-point".to_vec())?;
                        Ok(solid.axis.to_world(point[0], point[1], solid.base_z + solid.height * layer as f64 / solid.layers as f64))
                    };
                    let p0 = position(indices[0])?;
                    let p1 = position(indices[1])?;
                    let p2 = position(indices[2])?;
                    let p3 = position(indices[3])?;
                    let a = [p1[0] - p0[0], p1[1] - p0[1], p1[2] - p0[2]];
                    let b = [p2[0] - p0[0], p2[1] - p0[1], p2[2] - p0[2]];
                    let c = [p3[0] - p0[0], p3[1] - p0[1], p3[2] - p0[2]];
                    let volume = (a[0] * (b[1] * c[2] - b[2] * c[1]) - a[1] * (b[0] * c[2] - b[2] * c[0]) + a[2] * (b[0] * c[1] - b[1] * c[0])).abs() / 6.0;
                    let material = doc.materials.get(self.solid_materials[meshed.solid_index]).ok_or_else(|| b"fem3d.numerical-self-weight-material".to_vec())?;
                    self.apply_rhs_value(self.load_node_cursor, Dof::Tz.index(), -material.rho * volume * 9.81 / 4.0)?;
                    self.load_vertex_cursor += 1;
                    self.load_node_cursor = 0;
                } else {
                    self.load_node_cursor += 1;
                }
            }
            Fem3dNumericalStage::BuildCsr => match self.csr_build.as_mut().ok_or_else(|| b"fem3d.numerical-csr".to_vec())?.step_one_funded(context.retained_grant()).and_then(|(complete, progress)| {
                context.consume_retained(progress).map_err(|_| b"fem3d.numerical-csr-grant" as &'static [u8])?;
                Ok(complete)
            }) {
                Ok(false) => {}
                Ok(true) => {
                    self.stage = Fem3dNumericalStage::BeginPcg;
                }
                Err(detail) => return Err((detail.to_vec()).into()),
            },
            Fem3dNumericalStage::BeginPcg => {
                let build = self.csr_build.as_mut().ok_or_else(|| b"fem3d.numerical-csr-terminal".to_vec())?;
                if build.take_demands().is_none() { return Err((b"fem3d.numerical-csr-terminal".to_vec()).into()); }
                let Some((matrix, progress)) = build.take_complete(context.retained_grant()) else { return Ok(false); };
                context.consume_retained(progress).map_err(|_| b"fem3d.numerical-csr-transfer-grant".to_vec())?;
                match PcgJobConstruction::new_with_mounted_rhs(operation, matrix, std::mem::take(&mut self.rhs)) {
                    Ok(construction) => self.pcg_build = Some(construction),
                    Err((matrix, rhs)) => {
                        self.rejected_pcg_matrix = Some(matrix);
                        self.rejected_pcg_rhs = Some(rhs);
                        return Err((b"fem3d.numerical-pcg-rhs".to_vec()).into());
                    }
                }
                self.stage = Fem3dNumericalStage::PreparePcg;
            }
            Fem3dNumericalStage::PreparePcg => match self.pcg_build.as_mut().ok_or_else(|| b"fem3d.numerical-pcg-build".to_vec())?.step_one() {
                Ok(false) => {}
                Ok(true) => {
                    self.pcg = self.pcg_build.as_mut().and_then(PcgJobConstruction::take_complete);
                    if self.pcg.is_none() {
                        return Err((b"fem3d.numerical-pcg-false-terminal".to_vec()).into());
                    }
                    self.stage = Fem3dNumericalStage::Pcg;
                }
                Err(detail) => return Err((detail.to_vec()).into()),
            },
            Fem3dNumericalStage::Pcg => {
                let end = child_end(self.pcg.as_mut().ok_or_else(|| b"fem3d.numerical-pcg".to_vec())?.step(context))?;
                if self.observe_child_end(end)? {
                    self.stage = Fem3dNumericalStage::ReadNodeScalar;
                    self.node_cursor = 0;
                    self.scalar_axis = 0;
                }
                return Ok(false);
            }
            Fem3dNumericalStage::ReadNodeScalar => {
                if self.node_cursor == self.analysis_node_ids.len() {
                    self.element_cursor = 0;
                    self.stage = Fem3dNumericalStage::RecoverStressElement;
                } else {
                    if let Some(equation) = self.equations[self.node_cursor][self.scalar_axis] {
                        let value = self.pcg.as_ref().and_then(|pcg| pcg.visual_scalar(equation)).ok_or_else(|| b"fem3d.numerical-node-scalar".to_vec())?;
                        if self.scalar_axis < 3 {
                            self.scalar.displacement[self.scalar_axis] = value.displacement;
                            self.scalar.residual[self.scalar_axis] = value.residual;
                        }
                    }
                    self.reaction_entry = 0;
                    self.reaction_accumulator = 0.0;
                    self.stage = Fem3dNumericalStage::RecoverReaction;
                }
            }
            Fem3dNumericalStage::RecoverReaction => {
                let Some(row) = self.full_equations[self.node_cursor][self.scalar_axis] else {
                    self.scalar_axis += 1;
                    self.stage = if self.scalar_axis == 6 { Fem3dNumericalStage::PublishNodeScalar } else { Fem3dNumericalStage::ReadNodeScalar };
                    return Ok(false);
                };
                if let Some((entry_row, entry_col, value)) = self.csr_build.as_ref().and_then(|build| build.visual_full_entry(self.reaction_entry)) {
                    if entry_row == row {
                        let displacement = self.csr_build.as_ref().and_then(|build| build.visual_compact_index(entry_col)).and_then(|compact| self.pcg.as_ref().and_then(|pcg| pcg.visual_scalar(compact))).map_or(0.0, |scalar| scalar.displacement);
                        self.reaction_accumulator += value * displacement;
                    }
                    self.reaction_entry += 1;
                } else {
                    let reaction = self.reaction_accumulator - self.full_rhs[row];
                    self.full_reactions[self.node_cursor][self.scalar_axis] = reaction;
                    if self.scalar_axis < 3 { self.scalar.reaction[self.scalar_axis] = reaction; }
                    self.scalar_axis += 1;
                    self.stage = if self.scalar_axis == 6 { Fem3dNumericalStage::PublishNodeScalar } else { Fem3dNumericalStage::ReadNodeScalar };
                }
            }
            Fem3dNumericalStage::PublishNodeScalar => {
                solver.publish_scalar(freshness, self.node_cursor, self.scalar).map_err(|_| b"fem3d.numerical-solver-publication".to_vec())?;
                self.node_cursor += 1;
                self.scalar_axis = 0;
                self.scalar = Fem3dSolverScalar::default();
                self.stage = Fem3dNumericalStage::ReadNodeScalar;
            }
            Fem3dNumericalStage::RecoverStressElement => {
                let build = self.csr_build.as_ref().ok_or_else(|| b"fem3d.numerical-stress-assembly".to_vec())?;
                let Some(element) = build.model_element(self.element_cursor) else {
                    self.node_cursor = 0;
                    self.stage = Fem3dNumericalStage::PublishStressScalar;
                    return Ok(false);
                };
                self.element_cursor += 1;
                let Elements::Tet4(tetrahedron) = element else { return Ok(false) };
                let mut indices = [0usize; 4];
                let mut positions = [[0.0; 3]; 4];
                let mut displacements = [0.0; 12];
                for local in 0..4 {
                    let id = element.mounted_node_id(local).ok_or_else(|| b"fem3d.numerical-stress-node".to_vec())?;
                    let index = (0..self.analysis_node_ids.len()).find(|index| self.analysis_node_ids.get(*index).is_some_and(|candidate| candidate == id)).ok_or_else(|| b"fem3d.numerical-stress-node".to_vec())?;
                    indices[local] = index;
                    positions[local] = build.model_node(index).ok_or_else(|| b"fem3d.numerical-stress-position".to_vec())?.pos;
                    for axis in 0..3 { displacements[local * 3 + axis] = self.equations[index][axis].and_then(|equation| self.pcg.as_ref()?.visual_scalar(equation)).map_or(0.0, |value| value.displacement); }
                }
                let stress = tetrahedron.mounted_recover_stress(&positions, &displacements).ok_or_else(|| b"fem3d.numerical-stress-geometry".to_vec())?.von_mises;
                if !stress.is_finite() { return Err((b"fem3d.numerical-stress-value".to_vec()).into()); }
                for index in indices { self.stress_sums[index] += stress; self.stress_counts[index] += 1; }
            }
            Fem3dNumericalStage::PublishStressScalar => {
                if self.node_cursor == self.analysis_node_ids.len() {
                    if self.request.buckling.is_some() { return Err((b"fem3d.numerical-buckling-owner-missing".to_vec()).into()); }
                    self.stage = if self.request.mode.is_some() { Fem3dNumericalStage::BeginModal } else { Fem3dNumericalStage::PublishProgress };
                } else {
                    let mut scalar = solver.scalar(self.node_cursor).ok_or_else(|| b"fem3d.numerical-stress-scalar".to_vec())?;
                    scalar.contour = self.stress_sums[self.node_cursor] / self.stress_counts[self.node_cursor].max(1) as f64;
                    solver.publish_scalar(freshness, self.node_cursor, scalar).map_err(|_| b"fem3d.numerical-stress-publication".to_vec())?;
                    self.node_cursor += 1;
                }
            }
            Fem3dNumericalStage::BeginModal => {
                let matrix = self.pcg.as_mut().and_then(PcgJob::take_completed_matrix).ok_or_else(|| b"fem3d.numerical-modal-matrix".to_vec())?;
                self.modal_build = Some(SymmetricPencilConstruction::new_mounted(matrix, std::mem::take(&mut self.modal_free_mass)));
                self.stage = Fem3dNumericalStage::PrepareModal;
            }
            Fem3dNumericalStage::PrepareModal => match self.modal_build.as_mut().ok_or_else(|| b"fem3d.numerical-modal-build".to_vec())?.step_one(context) {
                Ok(false) => {}
                Ok(true) => {
                    self.stage = Fem3dNumericalStage::BeginLdlt;
                }
                Err(detail) => return Err((detail.to_vec()).into()),
            },
            Fem3dNumericalStage::BeginLdlt => {
                let order = self.modal_build.as_ref().and_then(SymmetricPencilConstruction::completed_order).ok_or_else(|| b"fem3d.numerical-modal-terminal".to_vec())?;
                let modes = self.request.mode.unwrap_or(0).checked_add(1).ok_or_else(|| b"fem3d.numerical-modal-order".to_vec())?;
                if !SubspaceIterationJob::admits_shape(order, modes) {
                    return Err((b"fem3d.numerical-modal-order".to_vec()).into());
                }
                let Some(((stiffness, mass), progress)) = self.modal_build.as_mut().and_then(|owner| owner.take_complete(context.retained_grant())) else { return Ok(false); };
                context.consume_retained(progress).map_err(|_| b"fem3d.numerical-modal-transfer".to_vec())?;
                self.modal_mass = Some(mass);
                self.ldlt = Some(LdltJob::new(operation, stiffness, 1));
                self.stage = Fem3dNumericalStage::Ldlt;
            }
            Fem3dNumericalStage::Ldlt => {
                let end = child_end(self.ldlt.as_mut().ok_or_else(|| b"fem3d.numerical-ldlt".to_vec())?.step(context))?;
                if self.observe_child_end(end)? {
                    self.stage = Fem3dNumericalStage::BeginSubspace;
                }
                return Ok(false);
            }
            Fem3dNumericalStage::BeginSubspace => {
                let Some((factor, progress)) = self.ldlt.as_mut().and_then(|owner| owner.take_factor(context.retained_grant())) else { return Ok(false); };
                context.consume_retained(progress).map_err(|_| b"fem3d.numerical-factor-transfer".to_vec())?;
                let mass = self.modal_mass.take().ok_or_else(|| b"fem3d.numerical-modal-mass".to_vec())?;
                let order = mass.n;
                self.subspace = Some(SubspaceIterationJob::new(operation, factor, mass, order, self.request.mode.unwrap_or(0) + 1, 30));
                self.stage = Fem3dNumericalStage::Subspace;
            }
            Fem3dNumericalStage::Subspace => {
                let end = child_end(self.subspace.as_mut().ok_or_else(|| b"fem3d.numerical-subspace".to_vec())?.step(context))?;
                if self.observe_child_end(end)? {
                    self.node_cursor = 0;
                    self.scalar_axis = 0;
                    self.stage = Fem3dNumericalStage::ReadModeScalar;
                }
                return Ok(false);
            }
            Fem3dNumericalStage::ReadModeScalar => {
                if self.node_cursor == self.analysis_node_ids.len() {
                    self.stage = Fem3dNumericalStage::PublishProgress;
                } else {
                    if let Some(equation) = self.equations[self.node_cursor][self.scalar_axis] {
                        let (component, eigenvalue) = self.subspace.as_ref().and_then(|subspace| subspace.visual_mode_scalar(self.request.mode.unwrap_or(0), equation)).ok_or_else(|| b"fem3d.numerical-mode-scalar".to_vec())?;
                        self.scalar.mode_shape[self.scalar_axis] = component;
                        self.scalar.eigen_estimate = eigenvalue;
                    }
                    self.scalar_axis += 1;
                    if self.scalar_axis == 3 {
                        self.stage = Fem3dNumericalStage::PublishModeScalar;
                    }
                }
            }
            Fem3dNumericalStage::PublishModeScalar => {
                let mut scalar = solver.scalar(self.node_cursor).ok_or_else(|| b"fem3d.numerical-static-scalar".to_vec())?;
                scalar.mode_shape = self.scalar.mode_shape;
                scalar.eigen_estimate = self.scalar.eigen_estimate;
                solver.publish_scalar(freshness, self.node_cursor, scalar).map_err(|_| b"fem3d.numerical-mode-publication".to_vec())?;
                self.node_cursor += 1;
                self.scalar_axis = 0;
                self.scalar = Fem3dSolverScalar::default();
                self.stage = Fem3dNumericalStage::ReadModeScalar;
            }
            Fem3dNumericalStage::PublishProgress => {
                let (_, _, residual, tolerance, converged) = self.pcg.as_ref().ok_or_else(|| b"fem3d.numerical-pcg".to_vec())?.visual_progress();
                let (_, _, mode_residual, modes_converged) = self.subspace.as_ref().map_or((0, 0, 0.0, true), SubspaceIterationJob::visual_progress);
                let state = if converged && modes_converged { Fem3dVisualState::ValidatedFinal } else { Fem3dVisualState::SolvingUnconverged };
                if !solver.publish_progress(freshness, state, residual.max(mode_residual), tolerance, self.analysis_node_ids.len(), self.analysis_node_ids.len()) {
                    return Err((b"fem3d.numerical-progress-publication".to_vec()).into());
                }
                self.stage = Fem3dNumericalStage::Complete;
            }
            Fem3dNumericalStage::Complete => return Ok(true),
            _ => {}
        }
        Ok(self.stage == Fem3dNumericalStage::Complete)
    }



    pub fn terminal_is_empty(&self) -> bool {
        self.close_lane > 21
            && self.model.is_none()
            && self.pending_support.is_none()
            && self.pending_element.is_none()
            && self.pending_tet.is_none()
            && self.pending_node_id.is_none()
            && self.mesh.is_none()
            && self.assembly_build.is_none()
            && self.assembly.is_none()
            && self.csr_build.is_none()
            && self.pcg_build.is_none()
            && self.rejected_pcg_matrix.is_none()
            && self.rejected_pcg_rhs.is_none()
            && self.pcg.is_none()
            && self.modal_build.is_none()
            && self.ldlt.is_none()
            && self.subspace.is_none()
            && self.modal_mass.is_none()
            && self.solid_domain.is_none()
            && self.analysis_node_ids.terminal_is_empty()
            && self.meshed_solids.terminal_is_empty()
            && self.solid_points.terminal_is_empty()
            && self.solid_tris.terminal_is_empty()
            && self.solid_node_ids.terminal_is_empty()
            && self.solid_node_analysis_indices.terminal_is_empty()
            && self.rhs.terminal_is_empty()
            && self.modal_free_mass.terminal_is_empty()
    }

    fn close_rung((terminal, copied_items, released_bytes): (bool, usize, usize)) -> (bool, RetainedCloneProgress) {
        (terminal, RetainedCloneProgress { copied_items, released_bytes, ..Default::default() })
    }

    pub fn close_step(&mut self, cx: &mut StepContext<'_>, grant: RetainedCloneGrant) -> Result<(bool, RetainedCloneProgress), semio_framework_job::OriginalTurnFailure> {
        let maximum_bytes = grant.maximum_release_bytes;
        let child = RetainedCloneGrant { maximum_items: 1, ..grant };
        if self.terminal_is_empty() { return Ok(Self::close_rung((true, 0, 0))); }
        if maximum_bytes == 0 { return Ok(Self::close_rung((false, 0, 0))); }
        if let Some(matrix) = self.rejected_pcg_matrix.as_mut() {
            let step = matrix.close_step(maximum_bytes);
            if step.0 {
                self.rejected_pcg_matrix = None;
            }
            return Ok(Self::close_rung((false, step.1, step.2)));
        }
        if let Some(rhs) = self.rejected_pcg_rhs.as_mut() {
            if rhs.close_step() {
                self.rejected_pcg_rhs = None;
            }
            return Ok(Self::close_rung((false, 1, 0)));
        }
        let step = match self.close_lane {
            0 => {
                let Some(job) = self.mesh.as_mut() else {
                    self.close_lane += 1;
                    return Ok(Self::close_rung((false, 0, 0)));
                };
                if !job.terminal_is_empty() {
                    let step = cx.with_retained_loan(child, |same| InteractiveJob::close_step(job, same))?;
                    return Ok((false, step.progress()));
                }
                self.mesh = None;
                let step = (true, 1, 0);
                step
            }
            1 => {
                let Some(job) = self.assembly_build.as_mut() else {
                    self.close_lane += 1;
                    return Ok(Self::close_rung((false, 0, 0)));
                };
                let step = job.close_step(maximum_bytes);
                if step.0 {
                    self.assembly_build = None;
                }
                step
            }
            2 => {
                let Some(job) = self.assembly.as_mut() else {
                    self.close_lane += 1;
                    return Ok(Self::close_rung((false, 0, 0)));
                };
                let step = job.close_retained_step(maximum_bytes);
                if step.0 {
                    self.assembly = None;
                }
                step
            }
            3 => {
                let Some(job) = self.csr_build.as_mut() else {
                    self.close_lane += 1;
                    return Ok(Self::close_rung((false, 0, 0)));
                };
                let step = job.close_step(maximum_bytes);
                if step.0 {
                    self.csr_build = None;
                }
                step
            }
            4 => {
                let Some(job) = self.pcg_build.as_mut() else {
                    self.close_lane += 1;
                    return Ok(Self::close_rung((false, 0, 0)));
                };
                if !job.terminal_is_empty() {
                    let step = cx.with_retained_loan(child, |same| job.close_step(same))?;
                    return Ok((false, step.progress()));
                }
                self.pcg_build = None;
                let step = (true, 1, 0);
                step
            }
            5 => {
                let Some(job) = self.pcg.as_mut() else {
                    self.close_lane += 1;
                    return Ok(Self::close_rung((false, 0, 0)));
                };
                if !job.terminal_is_empty() {
                    let step = cx.with_retained_loan(child, |same| InteractiveJob::close_step(job, same))?;
                    return Ok((false, step.progress()));
                }
                self.pcg = None;
                let step = (true, 1, 0);
                step
            }
            6 => {
                let Some(job) = self.modal_build.as_mut() else {
                    self.close_lane += 1;
                    return Ok(Self::close_rung((false, 0, 0)));
                };
                let step = job.close_step(maximum_bytes);
                if step.0 {
                    self.modal_build = None;
                }
                step
            }
            7 => {
                let Some(job) = self.ldlt.as_mut() else {
                    self.close_lane += 1;
                    return Ok(Self::close_rung((false, 0, 0)));
                };
                if self.close_started_jobs & 1 == 0 {
                    job.begin_close();
                    self.close_started_jobs |= 1;
                    return Ok(Self::close_rung((false, 1, 0)));
                }
                if !job.terminal_is_empty() {
                    let step = cx.with_retained_loan(child, |same| InteractiveJob::close_step(job, same))?;
                    return Ok((false, step.progress()));
                }
                self.ldlt = None;
                let step = (true, 1, 0);
                step
            }
            8 => {
                let Some(job) = self.subspace.as_mut() else {
                    self.close_lane += 1;
                    return Ok(Self::close_rung((false, 0, 0)));
                };
                if self.close_started_jobs & 2 == 0 {
                    job.begin_close();
                    self.close_started_jobs |= 2;
                    return Ok(Self::close_rung((false, 1, 0)));
                }
                if !job.terminal_is_empty() {
                    let step = cx.with_retained_loan(child, |same| InteractiveJob::close_step(job, same))?;
                    return Ok((false, step.progress()));
                }
                self.subspace = None;
                let step = (true, 1, 0);
                step
            }
            9 => {
                let Some(mass) = self.modal_mass.as_mut() else {
                    self.close_lane += 1;
                    return Ok(Self::close_rung((false, 0, 0)));
                };
                let step = mass.close_step(maximum_bytes);
                if step.0 {
                    self.modal_mass = None;
                }
                step
            }
            10 => {
                let Some(domain) = self.solid_domain.as_mut() else {
                    self.close_lane += 1;
                    return Ok(Self::close_rung((false, 0, 0)));
                };
                if domain.close_step() {
                    self.solid_domain = None;
                    (true, 1, 0)
                } else {
                    (false, 1, 0)
                }
            }
            11 => {
                if let Some(solid) = self.meshed_solids.len().checked_sub(1).and_then(|index| self.meshed_solids.get_mut(index)) {
                    if let Some(value) = solid.node_ids.len().checked_sub(1).and_then(|index| solid.node_ids.get_mut(index)) {
                        let bytes = value.capacity();
                        if bytes != 0 {
                            if bytes > maximum_bytes {
                                return Ok(Self::close_rung((false, 0, 0)));
                            }
                            *value = String::new();
                            return Ok(Self::close_rung((false, 1, bytes)));
                        }
                        solid.node_ids.pop();
                        return Ok(Self::close_rung((false, 1, 0)));
                    }
                    if !solid.node_ids.close_admission_one() {
                        return Ok(Self::close_rung((false, 1, 0)));
                    }
                    if solid.tris.pop().is_some() || !solid.tris.close_admission_one() {
                        return Ok(Self::close_rung((false, 1, 0)));
                    }
                    if solid.points.pop().is_some() || !solid.points.close_admission_one() {
                        return Ok(Self::close_rung((false, 1, 0)));
                    }
                    self.meshed_solids.pop();
                    return Ok(Self::close_rung((false, 1, 0)));
                }
                (self.meshed_solids.close_admission_one(), 1, 0)
            }
            12 => {
                if let Some(value) = self.pending_node_id.as_mut() {
                    let bytes = value.capacity();
                    if bytes != 0 {
                        if bytes > maximum_bytes {
                            return Ok(Self::close_rung((false, 0, 0)));
                        }
                        *value = String::new();
                        return Ok(Self::close_rung((false, 1, bytes)));
                    }
                    self.pending_node_id = None;
                    return Ok(Self::close_rung((false, 1, 0)));
                }
                (true, 0, 0)
            }
            13 => {
                if let Some(value) = self.solid_node_ids.len().checked_sub(1).and_then(|index| self.solid_node_ids.get_mut(index)) {
                    let bytes = value.capacity();
                    if bytes != 0 {
                        if bytes > maximum_bytes {
                            return Ok(Self::close_rung((false, 0, 0)));
                        }
                        *value = String::new();
                        return Ok(Self::close_rung((false, 1, bytes)));
                    }
                    self.solid_node_ids.pop();
                    return Ok(Self::close_rung((false, 1, 0)));
                }
                if !self.solid_node_ids.close_admission_one() {
                    return Ok(Self::close_rung((false, 1, 0)));
                }
                if self.solid_node_analysis_indices.pop().is_some() || !self.solid_node_analysis_indices.close_admission_one() {
                    return Ok(Self::close_rung((false, 1, 0)));
                }
                (true, 0, 0)
            }
            14 => {
                let Some(support) = self.pending_support.as_mut() else {
                    self.close_lane += 1;
                    return Ok(Self::close_rung((false, 0, 0)));
                };
                let step = support.close_step(maximum_bytes);
                if step.0 {
                    self.pending_support = None;
                }
                step
            }
            15 => {
                if let Some(value) = self.analysis_node_ids.len().checked_sub(1).and_then(|index| self.analysis_node_ids.get_mut(index)) {
                    let bytes = value.capacity();
                    if bytes != 0 {
                        if bytes > maximum_bytes {
                            return Ok(Self::close_rung((false, 0, 0)));
                        }
                        *value = String::new();
                        return Ok(Self::close_rung((false, 1, bytes)));
                    }
                    self.analysis_node_ids.pop();
                    return Ok(Self::close_rung((false, 1, 0)));
                }
                (self.analysis_node_ids.close_admission_one(), 1, 0)
            }
            16 => {
                if self.solid_tris.pop().is_some() || !self.solid_tris.close_admission_one() {
                    return Ok(Self::close_rung((false, 1, 0)));
                }
                if self.solid_points.pop().is_some() || !self.solid_points.close_admission_one() {
                    return Ok(Self::close_rung((false, 1, 0)));
                }
                (true, 0, 0)
            }
            17 => (self.rhs.close_step(), 1, 0),
            18 => (self.modal_free_mass.close_step(), 1, 0),
            19 => {
                let Some(element) = self.pending_element.as_mut() else {
                    self.close_lane += 1;
                    return Ok(Self::close_rung((false, 0, 0)));
                };
                if let Some(bytes) = element.mounted_next_string_bytes() {
                    if bytes > maximum_bytes {
                        return Ok(Self::close_rung((false, 0, 0)));
                    }
                    let released = element.close_mounted_string_step().unwrap_or(0);
                    return Ok(Self::close_rung((false, 1, released)));
                }
                self.pending_element = None;
                (true, 1, 0)
            }
            20 => {
                let Some(element) = self.pending_tet.as_mut() else {
                    self.close_lane += 1;
                    return Ok(Self::close_rung((false, 0, 0)));
                };
                if let Some(bytes) = element.mounted_next_string_bytes() {
                    if bytes > maximum_bytes {
                        return Ok(Self::close_rung((false, 0, 0)));
                    }
                    let released = element.close_mounted_string_step().unwrap_or(0);
                    return Ok(Self::close_rung((false, 1, released)));
                }
                self.pending_tet = None;
                (true, 1, 0)
            }
            21 => {
                let Some(model) = self.model.as_mut() else {
                    self.close_lane += 1;
                    return Ok(Self::close_rung((false, 0, 0)));
                };
                let step = model.close_step(maximum_bytes);
                if step.0 {
                    self.model = None;
                }
                step
            }
            _ => return Ok(Self::close_rung((self.terminal_is_empty(), 0, 0))),
        };
        if step.0 {
            self.close_lane += 1;
            return Ok(Self::close_rung((false, step.1, step.2)));
        }
        Ok(Self::close_rung(step))
    }
}
//#endregion 📦️PageVisualJob
