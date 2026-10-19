//! 🍓️ Staged step: the numerical child staged solve step across solid mesh, assembly, PCG and the finishing stages. See [`super`].

use super::*;

#[path = "🏗️mesh-and-assembly-stages/🦀️.rs"]
mod mesh_and_assembly_stages;

#[path = "⚖️load-application-stages/🦀️.rs"]
mod load_application_stages;

#[path = "🧮️solve-and-recovery-stages/🦀️.rs"]
mod solve_and_recovery_stages;

#[path = "🎼️modal-and-publication-stages/🦀️.rs"]
mod modal_and_publication_stages;

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
            Fem3dNumericalStage::PublishFieldCount => return self.step_publish_field_count(solver, freshness),
            Fem3dNumericalStage::MountAssembly => return self.step_mount_assembly(operation),
            Fem3dNumericalStage::PrepareAssembly => return self.step_prepare_assembly(context),
            Fem3dNumericalStage::Assembly => return self.step_assembly(context),
            Fem3dNumericalStage::ReserveModalMass => return self.step_reserve_modal_mass(),
            Fem3dNumericalStage::InitializeModalMass => return self.step_initialize_modal_mass(),
            Fem3dNumericalStage::MapEquation => return self.step_map_equation(),
            Fem3dNumericalStage::ReserveRhs => return self.step_reserve_rhs(),
            Fem3dNumericalStage::InitializeRhs => return self.step_initialize_rhs(),
            Fem3dNumericalStage::ResolveSourceCase => return self.step_resolve_source_case(doc),
            Fem3dNumericalStage::ApplyLoad => return self.step_apply_load(doc),
            Fem3dNumericalStage::ApplyNodalLoad => return self.step_apply_nodal_load(doc),
            Fem3dNumericalStage::ResolveMemberLoad => return self.step_resolve_member_load(doc),
            Fem3dNumericalStage::ResolveMemberNode => return self.step_resolve_member_node(doc),
            Fem3dNumericalStage::ApplyMemberScalar => return self.step_apply_member_scalar(doc),
            Fem3dNumericalStage::ResolveAreaLoad => return self.step_resolve_area_load(doc),
            Fem3dNumericalStage::ApplyAreaNode => return self.step_apply_area_node(doc),
            Fem3dNumericalStage::ResolveSelfWeightMemberNode => return self.step_resolve_self_weight_member_node(doc),
            Fem3dNumericalStage::ApplySelfWeightMember => return self.step_apply_self_weight_member(doc),
            Fem3dNumericalStage::ApplySelfWeightSolid => return self.step_apply_self_weight_solid(doc),
            Fem3dNumericalStage::BuildCsr => return self.step_build_csr(context),
            Fem3dNumericalStage::BeginPcg => return self.step_begin_pcg(operation, context),
            Fem3dNumericalStage::PreparePcg => return self.step_prepare_pcg(),
            Fem3dNumericalStage::Pcg => return self.step_pcg(context),
            Fem3dNumericalStage::ReadNodeScalar => return self.step_read_node_scalar(),
            Fem3dNumericalStage::RecoverReaction => return self.step_recover_reaction(),
            Fem3dNumericalStage::PublishNodeScalar => return self.step_publish_node_scalar(solver, freshness),
            Fem3dNumericalStage::RecoverStressElement => return self.step_recover_stress_element(),
            Fem3dNumericalStage::PublishStressScalar => return self.step_publish_stress_scalar(solver, freshness),
            Fem3dNumericalStage::BeginModal => return self.step_begin_modal(),
            Fem3dNumericalStage::PrepareModal => return self.step_prepare_modal(context),
            Fem3dNumericalStage::BeginLdlt => return self.step_begin_ldlt(operation, context),
            Fem3dNumericalStage::Ldlt => return self.step_ldlt(context),
            Fem3dNumericalStage::BeginSubspace => return self.step_begin_subspace(operation, context),
            Fem3dNumericalStage::Subspace => return self.step_subspace(context),
            Fem3dNumericalStage::ReadModeScalar => return self.step_read_mode_scalar(),
            Fem3dNumericalStage::PublishModeScalar => return self.step_publish_mode_scalar(solver, freshness),
            Fem3dNumericalStage::PublishProgress => return self.step_publish_progress(solver, freshness),
            Fem3dNumericalStage::Complete => return Ok(true),
            _ => {}
        }
        Ok(self.stage == Fem3dNumericalStage::Complete)
    }



}
//#endregion 📦️PageVisualJob
