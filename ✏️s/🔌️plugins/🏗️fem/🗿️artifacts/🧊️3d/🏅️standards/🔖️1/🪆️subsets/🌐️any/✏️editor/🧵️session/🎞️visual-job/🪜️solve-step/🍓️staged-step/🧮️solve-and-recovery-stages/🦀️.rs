//! 🧮️ Solve and recovery stages: the CSR build, the PCG solve and the node scalar, reaction and stress recovery stages of the staged numerical step. See [`super`].

use semio_s_fem_payload_turns::NumericalStep;
use super::*;

impl Fem3dNumericalChild {
    /// 🗜 Builds the CSR matrix of the assembled system.
    pub(super) fn step_build_csr(&mut self, context: &mut StepContext<'_>) -> Result<bool, Fem3dNumericalFailure> {
        let build = self.csr_build.as_mut().ok_or_else(|| b"fem3d.numerical-csr".to_vec())?;
        for _ in 0..64 {
            if context.should_yield() || context.is_cancelled() { break; }
            let grant = context.retained_grant();
            if grant.maximum_items == 0 { break; }
            let Some((result, work)) = context.with_original_effect(grant, |_, effect| build.step_one_in(effect))
                .map_err(|_| b"fem3d.numerical-csr-grant".to_vec())? else { break; };
            semio_framework_job::receive_original_retained_work(context, semio_framework_value::RetainedWorkOutcome {
                result: Ok::<_, semio_framework_value::ValueError>(semio_framework_value::RetainedCloneStep::Progress(Default::default())),
                work,
            }).map_err(|_| b"fem3d.numerical-csr-grant".to_vec())?;
            let (complete, progress) = result.map_err(|detail| detail.to_vec())?;
            context.consume_retained(progress).map_err(|_| b"fem3d.numerical-csr-grant".to_vec())?;
            if complete {
                self.stage = Fem3dNumericalStage::BeginPcg;
                break;
            }
            if progress == RetainedCloneProgress::default() { break; }
            if work == 0 { context.consume_fuel(1); }
        }
        Ok(false)
    }

    /// 🚦 Begins the preconditioned conjugate gradient solve.
    pub(super) fn step_begin_pcg(&mut self, operation: semio_framework_job::Operation, context: &mut StepContext<'_>) -> Result<bool, Fem3dNumericalFailure> {
        let build = self.csr_build.as_mut().ok_or_else(|| b"fem3d.numerical-csr-terminal".to_vec())?;
        if build.take_demands().is_none() { return Err((b"fem3d.numerical-csr-terminal".to_vec()).into()); }
        let Some((matrix, progress)) = build.take_complete(context.retained_grant()) else { return Ok(false); };
        context.consume_retained(progress).map_err(|_| b"fem3d.numerical-csr-transfer-grant".to_vec())?;
        match PcgJobConstruction::new_with_mounted_rhs(operation, matrix, std::mem::take(&mut self.equation_values)) {
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
    pub(super) fn step_prepare_pcg(&mut self, context: &mut StepContext<'_>) -> Result<bool, Fem3dNumericalFailure> {
        let build = self.pcg_build.as_mut().ok_or_else(|| b"fem3d.numerical-pcg-build".to_vec())?;
        if build.step_funded(context)? {
            self.pcg = build.take_complete();
            if self.pcg.is_none() {
                return Err((b"fem3d.numerical-pcg-false-terminal".to_vec()).into());
            }
            self.stage = Fem3dNumericalStage::Pcg;
        }
        Ok(self.stage == Fem3dNumericalStage::Complete)
    }

    /// 🔁 Steps the preconditioned conjugate gradient solve.
    pub(super) fn step_pcg(&mut self, context: &mut StepContext<'_>) -> Result<bool, Fem3dNumericalFailure> {
        let end = child_end(self.pcg.as_mut().ok_or_else(|| b"fem3d.numerical-pcg".to_vec())?.step_plain(context))?;
        if self.observe_child_end(end)? {
            self.stage = Fem3dNumericalStage::ReserveReactionRows;
        }
        Ok(false)
    }

    /// 🧺 Admits the paged accumulator used by the full equation rows.
    pub(super) fn step_reserve_reaction_rows(&mut self, context: &mut StepContext<'_>) -> Result<bool, Fem3dNumericalFailure> {
        let order = self.reaction_order()?;
        if self.equation_values.admit_in(order, context).map_err(|_| b"fem3d.numerical-reaction-admission".to_vec())? {
            self.stage = Fem3dNumericalStage::InitializeReactionRows;
        }
        Ok(false)
    }

    /// 🏁 Initializes one full equation row before visiting the original triplets.
    pub(super) fn step_initialize_reaction_rows(&mut self) -> Result<bool, Fem3dNumericalFailure> {
        if self.equation_values.len() == self.reaction_order()? {
            self.reaction_entry = 0;
            self.stage = Fem3dNumericalStage::RecoverReaction;
        } else {
            self.equation_values.push(0.0).map_err(|_| b"fem3d.numerical-reaction-row".to_vec())?;
        }
        Ok(false)
    }

    fn reaction_order(&self) -> Result<usize, Fem3dNumericalFailure> {
        self.csr_build.as_ref().and_then(AssemblyCsrBuild::visual_full_order)
            .ok_or_else(|| b"fem3d.numerical-reaction-assembly".to_vec().into())
    }

    /// ↩ Accumulates each original stiffness contribution in its full equation row.
    pub(super) fn step_recover_reaction(&mut self, context: &mut StepContext<'_>) -> Result<bool, Fem3dNumericalFailure> {
        let build = self.csr_build.as_ref().ok_or_else(|| b"fem3d.numerical-reaction-assembly".to_vec())?;
        let pcg = self.pcg.as_ref().ok_or_else(|| b"fem3d.numerical-reaction-solution".to_vec())?;
        for _ in 0..256 {
            if context.should_yield() || context.is_cancelled() { break; }
            let Some((row, column, value)) = build.visual_full_entry(self.reaction_entry) else {
                self.node_cursor = 0;
                self.scalar_axis = 0;
                self.stage = Fem3dNumericalStage::ReadNodeScalar;
                break;
            };
            let displacement = build.visual_compact_index(column).and_then(|index| pcg.visual_scalar(index))
                .map_or(0.0, |scalar| scalar.displacement);
            self.equation_values.add_at(row, value * displacement).map_err(|_| b"fem3d.numerical-reaction-row".to_vec())?;
            self.reaction_entry += 1;
            context.consume_fuel(1);
        }
        Ok(false)
    }

    /// 📖 Reads one node scalar of the solution.
    pub(super) fn step_read_node_scalar(&mut self) -> Result<bool, Fem3dNumericalFailure> {
        if self.node_cursor == self.analysis_node_ids.len() {
            self.element_cursor = 0;
            self.stage = Fem3dNumericalStage::RecoverStressElement;
        } else {
            if let Some(equation) = self.equation(self.node_cursor, self.scalar_axis) {
                let value = self.pcg.as_ref().and_then(|pcg| pcg.visual_scalar(equation)).ok_or_else(|| b"fem3d.numerical-node-scalar".to_vec())?;
                if self.scalar_axis < 3 {
                    self.scalar.displacement[self.scalar_axis] = value.displacement;
                    self.scalar.residual[self.scalar_axis] = value.residual;
                }
            }
            let fields = self.node_values.get_mut(self.node_cursor).ok_or_else(|| b"fem3d.numerical-reaction-node-values".to_vec())?;
            if let Some(row) = fields.full_equations[self.scalar_axis] {
                let internal = self.equation_values.get(row).ok_or_else(|| b"fem3d.numerical-reaction-row".to_vec())?;
                let reaction = internal - fields.applied[self.scalar_axis];
                fields.reactions[self.scalar_axis] = reaction;
                if self.scalar_axis < 3 { self.scalar.reaction[self.scalar_axis] = reaction; }
            }
            self.scalar_axis += 1;
            if self.scalar_axis == 6 {
                self.stage = Fem3dNumericalStage::PublishNodeScalar;
            }
        }
        Ok(false)
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
            for axis in 0..3 { displacements[local * 3 + axis] = self.equation(index, axis).and_then(|equation| self.pcg.as_ref()?.visual_scalar(equation)).map_or(0.0, |value| value.displacement); }
        }
        let stress = tetrahedron.mounted_recover_stress(&positions, &displacements).ok_or_else(|| b"fem3d.numerical-stress-geometry".to_vec())?.von_mises;
        if !stress.is_finite() { return Err((b"fem3d.numerical-stress-value".to_vec()).into()); }
        for index in indices {
            let fields = self.node_values.get_mut(index).ok_or_else(|| b"fem3d.numerical-stress-node-values".to_vec())?;
            fields.stress_sum += stress;
            fields.stress_count += 1;
        }
        Ok(self.stage == Fem3dNumericalStage::Complete)
    }

    /// 📡 Publishes one stress scalar.
    pub(super) fn step_publish_stress_scalar(&mut self, solver: &mut Fem3dSolverView, freshness: Fem3dVisualFreshness) -> Result<bool, Fem3dNumericalFailure> {
        if self.node_cursor == self.analysis_node_ids.len() {
            self.stage = if self.request.buckling.is_some() { Fem3dNumericalStage::BeginBuckling } else if self.request.mode.is_some() { Fem3dNumericalStage::BeginModal } else { Fem3dNumericalStage::PublishProgress };
        } else {
            let mut scalar = solver.scalar(self.node_cursor).ok_or_else(|| b"fem3d.numerical-stress-scalar".to_vec())?;
            scalar.contour = self.stress(self.node_cursor).unwrap_or(0.0);
            solver.publish_scalar(freshness, self.node_cursor, scalar).map_err(|_| b"fem3d.numerical-stress-publication".to_vec())?;
            self.node_cursor += 1;
        }
        Ok(self.stage == Fem3dNumericalStage::Complete)
    }
}
