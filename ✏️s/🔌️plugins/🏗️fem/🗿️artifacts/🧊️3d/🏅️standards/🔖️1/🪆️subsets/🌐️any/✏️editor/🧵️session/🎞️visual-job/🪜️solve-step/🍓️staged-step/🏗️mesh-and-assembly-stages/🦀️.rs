//! 🏗️ Mesh and assembly stages: the field count, assembly mounting and build, the assembly steps, the modal mass and the equation map of the staged numerical step. See [`super`].

use super::*;

impl Fem3dNumericalChild {
    /// 🔢 Publishes the analysis node field count.
    pub(super) fn step_publish_field_count(&mut self, solver: &mut Fem3dSolverView, freshness: Fem3dVisualFreshness) -> Result<bool, Fem3dNumericalFailure> {
        if !solver.set_len(freshness, self.analysis_node_ids.len()) {
            return Err((b"fem3d.numerical-field-count".to_vec()).into());
        }
        self.stage = Fem3dNumericalStage::MountAssembly;
        Ok(self.stage == Fem3dNumericalStage::Complete)
    }

    /// 🧷 Mounts the assembly build.
    pub(super) fn step_mount_assembly(&mut self, operation: semio_framework_job::Operation) -> Result<bool, Fem3dNumericalFailure> {
        let model = self.model.take().ok_or_else(|| b"fem3d.numerical-model-owner".to_vec())?;
        self.assembly_build = Some(AssemblyJobConstruction::new_mounted(model, operation, 1));
        self.stage = Fem3dNumericalStage::PrepareAssembly;
        Ok(self.stage == Fem3dNumericalStage::Complete)
    }

    /// 🧰 Advances the funded assembly build until the assembly is complete.
    pub(super) fn step_prepare_assembly(&mut self, context: &mut StepContext<'_>) -> Result<bool, Fem3dNumericalFailure> {
        match self.assembly_build.as_mut().ok_or_else(|| b"fem3d.numerical-assembly-build".to_vec())?.step_one_funded(context.retained_grant()).and_then(|(complete, progress)| {
            context.consume_retained(progress).map_err(|_| semio_s_fem_model::FemError::ResourceAdmission)?;
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
        }
        Ok(self.stage == Fem3dNumericalStage::Complete)
    }

    /// 🔩 Steps the assembly child to its end.
    pub(super) fn step_assembly(&mut self, context: &mut StepContext<'_>) -> Result<bool, Fem3dNumericalFailure> {
        let end = child_end(self.assembly.as_mut().ok_or_else(|| b"fem3d.numerical-assembly".to_vec())?.step(context))?;
        if self.observe_child_end(end)? {
            self.node_cursor = 0;
            self.dof_cursor = 0;
            self.free_order = self.assembly.as_ref().map_or(0, AssemblyJob::visual_free_order);
            self.stage = Fem3dNumericalStage::ReserveModalMass;
        }
        return Ok(false);
        Ok(self.stage == Fem3dNumericalStage::Complete)
    }

    /// 🏋 Reserves the modal mass storage.
    pub(super) fn step_reserve_modal_mass(&mut self) -> Result<bool, Fem3dNumericalFailure> {
        if self.modal_free_mass.admit_one(self.free_order).map_err(|_| b"fem3d.numerical-modal-mass-admission".to_vec())? {
            self.stage = Fem3dNumericalStage::InitializeModalMass;
        }
        Ok(self.stage == Fem3dNumericalStage::Complete)
    }

    /// 🧊 Initializes the modal mass storage.
    pub(super) fn step_initialize_modal_mass(&mut self) -> Result<bool, Fem3dNumericalFailure> {
        if self.modal_free_mass.len() < self.free_order {
            self.modal_free_mass.push(0.0).map_err(|_| b"fem3d.numerical-modal-mass-slot".to_vec())?;
        } else {
            self.node_cursor = 0;
            self.dof_cursor = 0;
            self.stage = Fem3dNumericalStage::MapEquation;
        }
        Ok(self.stage == Fem3dNumericalStage::Complete)
    }

    /// 🗺 Maps the equation numbers of the free degrees of freedom.
    pub(super) fn step_map_equation(&mut self) -> Result<bool, Fem3dNumericalFailure> {
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
        Ok(self.stage == Fem3dNumericalStage::Complete)
    }
}
