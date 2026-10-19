//! 🧮️ Solve and recovery stages: the CSR build, the PCG solve and the node scalar, reaction and stress recovery stages of the staged numerical step. See [`super`].

use super::*;

impl Fem3dNumericalChild {
    /// 🗜 Builds the CSR matrix of the assembled system.
    pub(super) fn step_build_csr(&mut self, context: &mut StepContext<'_>) -> Result<bool, Fem3dNumericalFailure> {
        match self.csr_build.as_mut().ok_or_else(|| b"fem3d.numerical-csr".to_vec())?.step_one_funded(context.retained_grant()).and_then(|(complete, progress)| {
            context.consume_retained(progress).map_err(|_| b"fem3d.numerical-csr-grant" as &'static [u8])?;
            Ok(complete)
        }) {
            Ok(false) => {}
            Ok(true) => {
                self.stage = Fem3dNumericalStage::BeginPcg;
            }
            Err(detail) => return Err((detail.to_vec()).into()),
        }
        Ok(self.stage == Fem3dNumericalStage::Complete)
    }

    /// 🚦 Begins the preconditioned conjugate gradient solve.
    pub(super) fn step_begin_pcg(&mut self, operation: semio_framework_job::Operation, context: &mut StepContext<'_>) -> Result<bool, Fem3dNumericalFailure> {
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
        Ok(self.stage == Fem3dNumericalStage::Complete)
    }

    /// 🔧 Prepares the preconditioned conjugate gradient solve.
    pub(super) fn step_prepare_pcg(&mut self) -> Result<bool, Fem3dNumericalFailure> {
        match self.pcg_build.as_mut().ok_or_else(|| b"fem3d.numerical-pcg-build".to_vec())?.step_one() {
            Ok(false) => {}
            Ok(true) => {
                self.pcg = self.pcg_build.as_mut().and_then(PcgJobConstruction::take_complete);
                if self.pcg.is_none() {
                    return Err((b"fem3d.numerical-pcg-false-terminal".to_vec()).into());
                }
                self.stage = Fem3dNumericalStage::Pcg;
            }
            Err(detail) => return Err((detail.to_vec()).into()),
        }
        Ok(self.stage == Fem3dNumericalStage::Complete)
    }

    /// 🔁 Steps the preconditioned conjugate gradient solve.
    pub(super) fn step_pcg(&mut self, context: &mut StepContext<'_>) -> Result<bool, Fem3dNumericalFailure> {
        let end = child_end(self.pcg.as_mut().ok_or_else(|| b"fem3d.numerical-pcg".to_vec())?.step(context))?;
        if self.observe_child_end(end)? {
            self.stage = Fem3dNumericalStage::ReadNodeScalar;
            self.node_cursor = 0;
            self.scalar_axis = 0;
        }
        return Ok(false);
        Ok(self.stage == Fem3dNumericalStage::Complete)
    }

    /// 📖 Reads one node scalar of the solution.
    pub(super) fn step_read_node_scalar(&mut self) -> Result<bool, Fem3dNumericalFailure> {
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
        Ok(self.stage == Fem3dNumericalStage::Complete)
    }

    /// ↩ Recovers one support reaction.
    pub(super) fn step_recover_reaction(&mut self) -> Result<bool, Fem3dNumericalFailure> {
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
        Ok(self.stage == Fem3dNumericalStage::Complete)
    }

    /// 📤 Publishes one node scalar.
    pub(super) fn step_publish_node_scalar(&mut self, solver: &mut Fem3dSolverView, freshness: Fem3dVisualFreshness) -> Result<bool, Fem3dNumericalFailure> {
        solver.publish_scalar(freshness, self.node_cursor, self.scalar).map_err(|_| b"fem3d.numerical-solver-publication".to_vec())?;
        self.node_cursor += 1;
        self.scalar_axis = 0;
        self.scalar = Fem3dSolverScalar::default();
        self.stage = Fem3dNumericalStage::ReadNodeScalar;
        Ok(self.stage == Fem3dNumericalStage::Complete)
    }

    /// 💥 Recovers the stress of one element.
    pub(super) fn step_recover_stress_element(&mut self) -> Result<bool, Fem3dNumericalFailure> {
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
        Ok(self.stage == Fem3dNumericalStage::Complete)
    }

    /// 📡 Publishes one stress scalar.
    pub(super) fn step_publish_stress_scalar(&mut self, solver: &mut Fem3dSolverView, freshness: Fem3dVisualFreshness) -> Result<bool, Fem3dNumericalFailure> {
        if self.node_cursor == self.analysis_node_ids.len() {
            if self.request.buckling.is_some() { return Err((b"fem3d.numerical-buckling-owner-missing".to_vec()).into()); }
            self.stage = if self.request.mode.is_some() { Fem3dNumericalStage::BeginModal } else { Fem3dNumericalStage::PublishProgress };
        } else {
            let mut scalar = solver.scalar(self.node_cursor).ok_or_else(|| b"fem3d.numerical-stress-scalar".to_vec())?;
            scalar.contour = self.stress_sums[self.node_cursor] / self.stress_counts[self.node_cursor].max(1) as f64;
            solver.publish_scalar(freshness, self.node_cursor, scalar).map_err(|_| b"fem3d.numerical-stress-publication".to_vec())?;
            self.node_cursor += 1;
        }
        Ok(self.stage == Fem3dNumericalStage::Complete)
    }
}
