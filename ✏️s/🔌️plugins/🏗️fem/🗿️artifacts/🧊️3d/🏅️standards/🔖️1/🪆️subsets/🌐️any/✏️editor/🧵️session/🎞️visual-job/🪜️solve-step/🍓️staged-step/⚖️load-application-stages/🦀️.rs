//! ⚖️ Load application stages: the right hand side, the source cases and the nodal, member, area and self weight load stages of the staged numerical step. See [`super`].

use super::*;

impl Fem3dNumericalChild {
    /// 📦 Reserves the right hand side storage.
    pub(super) fn step_reserve_rhs(&mut self) -> Result<bool, Fem3dNumericalFailure> {
        if self.rhs.admit_one(self.free_order).map_err(|_| b"fem3d.numerical-rhs-admission".to_vec())? {
            self.stage = Fem3dNumericalStage::InitializeRhs;
        }
        Ok(self.stage == Fem3dNumericalStage::Complete)
    }

    /// 🧹 Initializes the right hand side storage.
    pub(super) fn step_initialize_rhs(&mut self) -> Result<bool, Fem3dNumericalFailure> {
        if self.rhs.len() < self.free_order {
            self.rhs.push(0.0).map_err(|_| b"fem3d.numerical-rhs-slot".to_vec())?;
        } else {
            self.node_cursor = 0;
            self.stage = Fem3dNumericalStage::ResolveSourceCase;
        }
        Ok(self.stage == Fem3dNumericalStage::Complete)
    }

    /// 🎯 Resolves the load case that feeds the right hand side.
    pub(super) fn step_resolve_source_case(&mut self, doc: &Fem3dSnapshot) -> Result<bool, Fem3dNumericalFailure> {
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
        Ok(self.stage == Fem3dNumericalStage::Complete)
    }

    /// ⬇ Dispatches the next load of the case to its application stage.
    pub(super) fn step_apply_load(&mut self, doc: &Fem3dSnapshot) -> Result<bool, Fem3dNumericalFailure> {
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
        Ok(self.stage == Fem3dNumericalStage::Complete)
    }

    /// 📍 Applies one nodal load.
    pub(super) fn step_apply_nodal_load(&mut self, doc: &Fem3dSnapshot) -> Result<bool, Fem3dNumericalFailure> {
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
        Ok(self.stage == Fem3dNumericalStage::Complete)
    }

    /// 📏 Resolves one member load.
    pub(super) fn step_resolve_member_load(&mut self, doc: &Fem3dSnapshot) -> Result<bool, Fem3dNumericalFailure> {
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
        Ok(self.stage == Fem3dNumericalStage::Complete)
    }

    /// 🔗 Resolves one node of a member load.
    pub(super) fn step_resolve_member_node(&mut self, doc: &Fem3dSnapshot) -> Result<bool, Fem3dNumericalFailure> {
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
        Ok(self.stage == Fem3dNumericalStage::Complete)
    }

    /// 🪢 Applies one scalar of a member load.
    pub(super) fn step_apply_member_scalar(&mut self, doc: &Fem3dSnapshot) -> Result<bool, Fem3dNumericalFailure> {
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
        Ok(self.stage == Fem3dNumericalStage::Complete)
    }

    /// 🟦 Resolves one area load.
    pub(super) fn step_resolve_area_load(&mut self, doc: &Fem3dSnapshot) -> Result<bool, Fem3dNumericalFailure> {
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
        Ok(self.stage == Fem3dNumericalStage::Complete)
    }

    /// 🧱 Applies one node of an area load.
    pub(super) fn step_apply_area_node(&mut self, doc: &Fem3dSnapshot) -> Result<bool, Fem3dNumericalFailure> {
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
        Ok(self.stage == Fem3dNumericalStage::Complete)
    }

    /// ⚓ Resolves one node of a member self weight.
    pub(super) fn step_resolve_self_weight_member_node(&mut self, doc: &Fem3dSnapshot) -> Result<bool, Fem3dNumericalFailure> {
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
        Ok(self.stage == Fem3dNumericalStage::Complete)
    }

    /// 🪨 Applies the self weight of one member.
    pub(super) fn step_apply_self_weight_member(&mut self, doc: &Fem3dSnapshot) -> Result<bool, Fem3dNumericalFailure> {
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
        Ok(self.stage == Fem3dNumericalStage::Complete)
    }

    /// 🏛 Applies the self weight of one solid element.
    pub(super) fn step_apply_self_weight_solid(&mut self, doc: &Fem3dSnapshot) -> Result<bool, Fem3dNumericalFailure> {
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
        Ok(self.stage == Fem3dNumericalStage::Complete)
    }
}
