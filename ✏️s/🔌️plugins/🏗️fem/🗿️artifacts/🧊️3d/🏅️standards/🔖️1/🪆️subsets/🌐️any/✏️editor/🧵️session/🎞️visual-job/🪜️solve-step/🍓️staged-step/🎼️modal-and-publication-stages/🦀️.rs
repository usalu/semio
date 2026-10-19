//! 🎼️ Modal and publication stages: the modal, LDLT and subspace solves and the mode scalar and progress publication stages of the staged numerical step. See [`super`].

use super::*;

impl Fem3dNumericalChild {
    /// 🎵 Begins the modal analysis.
    pub(super) fn step_begin_modal(&mut self) -> Result<bool, Fem3dNumericalFailure> {
        let matrix = self.pcg.as_mut().and_then(PcgJob::take_completed_matrix).ok_or_else(|| b"fem3d.numerical-modal-matrix".to_vec())?;
        self.modal_build = Some(SymmetricPencilConstruction::new_mounted(matrix, std::mem::take(&mut self.modal_free_mass)));
        self.stage = Fem3dNumericalStage::PrepareModal;
        Ok(self.stage == Fem3dNumericalStage::Complete)
    }

    /// 🎶 Prepares the modal analysis.
    pub(super) fn step_prepare_modal(&mut self, context: &mut StepContext<'_>) -> Result<bool, Fem3dNumericalFailure> {
        match self.modal_build.as_mut().ok_or_else(|| b"fem3d.numerical-modal-build".to_vec())?.step_one(context) {
            Ok(false) => {}
            Ok(true) => {
                self.stage = Fem3dNumericalStage::BeginLdlt;
            }
            Err(detail) => return Err((detail.to_vec()).into()),
        }
        Ok(self.stage == Fem3dNumericalStage::Complete)
    }

    /// 🪜 Begins the LDLT factorization.
    pub(super) fn step_begin_ldlt(&mut self, operation: semio_framework_job::Operation, context: &mut StepContext<'_>) -> Result<bool, Fem3dNumericalFailure> {
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
        Ok(self.stage == Fem3dNumericalStage::Complete)
    }

    /// 🔨 Steps the LDLT factorization.
    pub(super) fn step_ldlt(&mut self, context: &mut StepContext<'_>) -> Result<bool, Fem3dNumericalFailure> {
        let end = child_end(self.ldlt.as_mut().ok_or_else(|| b"fem3d.numerical-ldlt".to_vec())?.step(context))?;
        if self.observe_child_end(end)? {
            self.stage = Fem3dNumericalStage::BeginSubspace;
        }
        return Ok(false);
        Ok(self.stage == Fem3dNumericalStage::Complete)
    }

    /// 🌌 Begins the subspace iteration.
    pub(super) fn step_begin_subspace(&mut self, operation: semio_framework_job::Operation, context: &mut StepContext<'_>) -> Result<bool, Fem3dNumericalFailure> {
        let Some((factor, progress)) = self.ldlt.as_mut().and_then(|owner| owner.take_factor(context.retained_grant())) else { return Ok(false); };
        context.consume_retained(progress).map_err(|_| b"fem3d.numerical-factor-transfer".to_vec())?;
        let mass = self.modal_mass.take().ok_or_else(|| b"fem3d.numerical-modal-mass".to_vec())?;
        let order = mass.n;
        self.subspace = Some(SubspaceIterationJob::new(operation, factor, mass, order, self.request.mode.unwrap_or(0) + 1, 30));
        self.stage = Fem3dNumericalStage::Subspace;
        Ok(self.stage == Fem3dNumericalStage::Complete)
    }

    /// 🛰 Steps the subspace iteration.
    pub(super) fn step_subspace(&mut self, context: &mut StepContext<'_>) -> Result<bool, Fem3dNumericalFailure> {
        let end = child_end(self.subspace.as_mut().ok_or_else(|| b"fem3d.numerical-subspace".to_vec())?.step(context))?;
        if self.observe_child_end(end)? {
            self.node_cursor = 0;
            self.scalar_axis = 0;
            self.stage = Fem3dNumericalStage::ReadModeScalar;
        }
        return Ok(false);
        Ok(self.stage == Fem3dNumericalStage::Complete)
    }

    /// 📚 Reads one mode scalar.
    pub(super) fn step_read_mode_scalar(&mut self) -> Result<bool, Fem3dNumericalFailure> {
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
        Ok(self.stage == Fem3dNumericalStage::Complete)
    }

    /// 🎙 Publishes one mode scalar.
    pub(super) fn step_publish_mode_scalar(&mut self, solver: &mut Fem3dSolverView, freshness: Fem3dVisualFreshness) -> Result<bool, Fem3dNumericalFailure> {
        let mut scalar = solver.scalar(self.node_cursor).ok_or_else(|| b"fem3d.numerical-static-scalar".to_vec())?;
        scalar.mode_shape = self.scalar.mode_shape;
        scalar.eigen_estimate = self.scalar.eigen_estimate;
        solver.publish_scalar(freshness, self.node_cursor, scalar).map_err(|_| b"fem3d.numerical-mode-publication".to_vec())?;
        self.node_cursor += 1;
        self.scalar_axis = 0;
        self.scalar = Fem3dSolverScalar::default();
        self.stage = Fem3dNumericalStage::ReadModeScalar;
        Ok(self.stage == Fem3dNumericalStage::Complete)
    }

    /// 🏁 Publishes the final progress and completes the step.
    pub(super) fn step_publish_progress(&mut self, solver: &mut Fem3dSolverView, freshness: Fem3dVisualFreshness) -> Result<bool, Fem3dNumericalFailure> {
        let (_, _, residual, tolerance, converged) = self.pcg.as_ref().ok_or_else(|| b"fem3d.numerical-pcg".to_vec())?.visual_progress();
        let (_, _, mode_residual, modes_converged) = self.subspace.as_ref().map_or((0, 0, 0.0, true), SubspaceIterationJob::visual_progress);
        let state = if converged && modes_converged { Fem3dVisualState::ValidatedFinal } else { Fem3dVisualState::SolvingUnconverged };
        if !solver.publish_progress(freshness, state, residual.max(mode_residual), tolerance, self.analysis_node_ids.len(), self.analysis_node_ids.len()) {
            return Err((b"fem3d.numerical-progress-publication".to_vec()).into());
        }
        self.stage = Fem3dNumericalStage::Complete;
        Ok(self.stage == Fem3dNumericalStage::Complete)
    }
}
