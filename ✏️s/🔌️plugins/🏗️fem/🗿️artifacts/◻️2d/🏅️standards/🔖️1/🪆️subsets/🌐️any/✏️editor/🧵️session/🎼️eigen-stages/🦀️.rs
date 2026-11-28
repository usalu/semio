//! 🎼️ Bounded modal and buckling stages over the original free-equation order.

use super::*;
use semio_s_fem_analyses::{LdltJob, SubspaceIterationJob, SubspaceSpectrum};
use semio_s_fem_analyses_assembly::SecondaryAssembly;
use semio_s_fem_app_surface::ResultMode;
use semio_s_fem_payload_turns::NumericalStep;
use semio_s_fem_sparse::SymmetricPencilConstruction;

#[cfg(test)]
#[path = "../🧪️tests/🎼️eigen/🦀️.rs"]
mod tests;

#[derive(Clone, Copy)]
enum Phase {
    Assemble,
    Pencil,
    Factor,
    StartSubspace,
    Iterate,
    Read,
}

pub(super) struct MountedEigen {
    phase: Phase,
    stiffness: Option<Csr>,
    secondary: SecondaryAssembly,
    pencil: Option<SymmetricPencilConstruction>,
    second: Option<Csr>,
    factor: Option<LdltJob>,
    subspace: Option<SubspaceIterationJob>,
}

impl MountedEigen {
    /// 📏️ Bounds the simultaneous pencils, factor, subspace matrices, and retained payload pages.
    pub(super) fn admits(order: usize, index: usize) -> bool {
        const TRIPLETS_PER_ELEMENT: usize = 36;
        const MATRIX_STORAGE_PER_PAIR: usize = 32 * size_of::<f64>();
        const PAGE_OVERHEAD: usize = 128 * SESSION_OWNER_PAGE_BYTES;
        const ELEMENT_STORAGE: usize = TRIPLETS_PER_ELEMENT * SESSION_MAXIMUM_ELEMENTS * size_of::<semio_s_fem_sparse::SparseTriplet>();
        let limit = semio_s_artifact_fem_2d_editor_session_catalog::SESSION_EIGEN_OWNER_BYTES;
        index.checked_add(1).is_some_and(|modes| SubspaceIterationJob::admits_shape(order, modes))
            && order.checked_mul(order)
                .and_then(|pairs| pairs.checked_mul(MATRIX_STORAGE_PER_PAIR))
                .is_some_and(|storage| storage <= limit - PAGE_OVERHEAD - ELEMENT_STORAGE)
    }

    pub(super) fn new(stiffness: Csr) -> Self {
        Self {
            phase: Phase::Assemble,
            stiffness: Some(stiffness),
            secondary: SecondaryAssembly::default(),
            pencil: None,
            second: None,
            factor: None,
            subspace: None,
        }
    }

    pub(super) fn close_demands(&self, demand: &mut RetirementDemand) -> Result<(), ValueError> {
        if let Some(factor) = self.factor.as_ref() {
            merge_close_demand(demand, semio_s_fem_sparse::numerical_close_demand(factor)?);
        }
        if let Some(subspace) = self.subspace.as_ref() {
            merge_close_demand(demand, semio_s_fem_sparse::numerical_close_demand(subspace)?);
        }
        Ok(())
    }

    pub(super) fn close_step(&mut self, cx: &mut StepContext<'_>, grant: RetainedCloneGrant) -> Result<Option<PluginLifecycleStep>, PluginLifecycleFailure> {
        if let Some(owner) = self.subspace.as_mut() {
            return if owner.terminal_is_empty() {
                self.subspace = None;
                Ok(Some(session_pending(1, 0)))
            } else {
                let step = cx.with_retained_loan(grant, |same| owner.close_step(same))?;
                Ok(Some(session_child_failure(step).unwrap_or_else(|| PluginLifecycleStep::Progress(step.progress()))))
            };
        }
        if let Some(owner) = self.factor.as_mut() {
            return if owner.terminal_is_empty() {
                self.factor = None;
                Ok(Some(session_pending(1, 0)))
            } else {
                let step = cx.with_retained_loan(grant, |same| owner.close_step(same))?;
                Ok(Some(session_child_failure(step).unwrap_or_else(|| PluginLifecycleStep::Progress(step.progress()))))
            };
        }
        if let Some(owner) = self.pencil.as_mut() {
            let (done, items, bytes) = owner.close_step(grant.maximum_release_bytes);
            if done { self.pencil = None; }
            return Ok(Some(session_pending(items.max(usize::from(done)), bytes)));
        }
        for slot in [&mut self.second, &mut self.stiffness] {
            if let Some(owner) = slot.as_mut() {
                let (done, items, bytes) = owner.close_step(grant.maximum_release_bytes);
                if done { *slot = None; }
                return Ok(Some(session_pending(items.max(usize::from(done)), bytes)));
            }
        }
        let (done, items, bytes) = self.secondary.close_step(grant.maximum_release_bytes);
        Ok((!done).then(|| session_pending(items, bytes)))
    }
}

fn geometric_cell(
    element: &semio_s_fem_static::Elements,
    positions: &[[f64; 3]],
    displacement: &[f64],
    row: usize,
    column: usize,
) -> Option<f64> {
    use semio_s_fem_static::Elements;
    match element {
        Elements::Bar2(element) => element.mounted_geometric_cell(positions, displacement, row, column),
        Elements::BeamEb2(element) => element.mounted_geometric_cell(positions, displacement, row, column),
        Elements::Tri3Cst(element) => element.mounted_geometric_cell(positions, displacement, row, column),
        _ => None,
    }.map(|value| -value)
}

impl MountedState {
    pub(super) fn step_eigen(&mut self, cx: &mut StepContext<'_>) -> Result<JobStep, semio_framework_job::OriginalTurnFailure> {
        let eigen = self.eigen.as_mut().expect("mounted eigen owner");
        let mode = self.mode_index;
        let operation = self.identity.operation(cx.operation(), cx.generation());
        match eigen.phase {
            Phase::Assemble => {
                let coefficient = |element: &semio_s_fem_static::Elements, positions: &[[f64; 3]], displacement: &[f64], row: usize, column: usize| match self.family {
                    ResultMode::Modal => element.mounted_mass_cell(positions, row, column),
                    ResultMode::Buckling => geometric_cell(element, positions, displacement, row, column),
                    ResultMode::Static => None,
                };
                let reference = |index| self.pcg.as_ref()?.visual_scalar(index).map(|scalar| scalar.displacement);
                let reference = (self.family == ResultMode::Buckling).then_some(&reference as &dyn Fn(usize) -> Option<f64>);
                let assembled = self.csr_build.as_ref().expect("original stiffness assembly retained");
                if !eigen.secondary.step(assembled, reference, coefficient, cx)? {
                    return Ok(self.progress(b"fem2d.eigen-assembly"));
                }
                let Some((second, progress)) = eigen.secondary.take_complete(cx.retained_grant()) else {
                    return Ok(self.progress(b"fem2d.eigen-secondary-transfer"));
                };
                cx.consume_retained(progress)?;
                let stiffness = eigen.stiffness.take().expect("stiffness retained until both matrices are ready");
                eigen.pencil = Some(SymmetricPencilConstruction::new_pair(stiffness, second));
                eigen.phase = Phase::Pencil;
            }
            Phase::Pencil => {
                let pencil = eigen.pencil.as_mut().expect("pencil retained");
                let budget = cx.retained_grant();
                let step = pencil.step_one(cx);
                if cx.retained_grant() != budget && cx.fuel_remaining() != 0 {
                    cx.consume_fuel(1);
                }
                match step {
                    Ok(false) => return Ok(self.progress(b"fem2d.eigen-pencil")),
                    Ok(true) => {}
                    Err(detail) => return Ok(self.fail(detail.to_vec())),
                }
                let Some(((stiffness, second), progress)) = pencil.take_complete(cx.retained_grant()) else {
                    return Ok(self.progress(b"fem2d.eigen-pencil-transfer"));
                };
                cx.consume_retained(progress)?;
                eigen.second = Some(second);
                eigen.factor = Some(LdltJob::new(operation, stiffness).without_publication());
                eigen.phase = Phase::Factor;
            }
            Phase::Factor => {
                match child_end(eigen.factor.as_mut().expect("factor retained").step_plain(cx))? {
                    ChildEnd::Complete(_) => eigen.phase = Phase::StartSubspace,
                    ChildEnd::Cancelled => return Ok(self.fail(b"fem2d.eigen-cancelled".to_vec())),
                    ChildEnd::Fault(detail) => return Ok(self.fail(detail.unwrap_or_else(|| b"fem2d.eigen-factor".to_vec()))),
                    _ => {}
                }
            }
            Phase::StartSubspace => {
                let order = eigen.second.as_ref().expect("second matrix retained").n;
                let Some(count) = mode.checked_add(1).filter(|count| SubspaceIterationJob::admits_shape(order, *count)) else {
                    return Ok(self.fail(b"fem2d.eigen-mode-out-of-range".to_vec()));
                };
                let Some((factor, progress)) = eigen.factor.as_mut().and_then(|factor| factor.take_factor(cx.retained_grant())) else {
                    return Ok(self.progress(b"fem2d.eigen-factor-transfer"));
                };
                cx.consume_retained(progress)?;
                let second = eigen.second.take().expect("second matrix retained");
                let job = SubspaceIterationJob::new(operation, factor, second, order, count, 30).without_publication();
                eigen.subspace = Some(if self.family == ResultMode::Buckling { job } else { job.with_positive_semidefinite_right() });
                eigen.phase = Phase::Iterate;
            }
            Phase::Iterate => {
                let subspace = eigen.subspace.as_mut().expect("subspace retained");
                let end = child_end(subspace.step_plain(cx))?;
                let (completed, total, residual, converged) = subspace.visual_progress();
                self.visual.progress_completed = completed;
                self.visual.progress_total = total;
                self.visual.residual_norm = residual;
                self.visual.converged = converged;
                match end {
                    ChildEnd::Complete(_) => {
                        match subspace.spectrum() {
                            SubspaceSpectrum::Positive { available } if mode < available => {}
                            SubspaceSpectrum::Positive { .. } | SubspaceSpectrum::NoPositive => {
                                self.stage = MountedStage::CommitReady;
                                return Ok(self.progress(b"fem2d.eigen-mode-unavailable"));
                            }
                            SubspaceSpectrum::Unresolved => return Ok(self.fail(b"fem2d.eigen-did-not-converge".to_vec())),
                        }
                        self.visual_field_cursor = 0;
                        self.eigenvalue = subspace.visual_mode_scalar(mode, 0).map(|(_, eigenvalue)| eigenvalue);
                        eigen.phase = Phase::Read;
                    }
                    ChildEnd::Cancelled => return Ok(self.fail(b"fem2d.eigen-cancelled".to_vec())),
                    ChildEnd::Fault(detail) => return Ok(self.fail(detail.unwrap_or_else(|| b"fem2d.eigen-subspace".to_vec()))),
                    _ => {}
                }
            }
            Phase::Read => {
                let assembly = self.csr_build.as_ref().expect("stiffness equations retained");
                let Some(node) = assembly.model_node(self.visual_field_cursor) else {
                    self.stage = MountedStage::CommitReady;
                    cx.consume_fuel(1);
                    return Ok(self.progress(b"fem2d.eigen-fields-ready"));
                };
                let subspace = eigen.subspace.as_ref().expect("converged eigenvectors retained");
                let mut mode_shape = [0.0; 2];
                for (axis, dof) in [semio_s_fem_model::Dof::Tx, semio_s_fem_model::Dof::Ty].into_iter().enumerate() {
                    if let Some(index) = assembly.visual_equation_indices(&node.id, dof).and_then(|(_, free)| free) {
                        let Some((scalar, _)) = subspace.visual_mode_scalar(mode, index) else {
                            return Ok(self.fail(b"fem2d.eigen-visual-scalar".to_vec()));
                        };
                        mode_shape[axis] = scalar;
                    }
                }
                let field = NodeLiveField {
                    node_id: String::new(),
                    displacement: [0.0; 2],
                    residual: [0.0; 2],
                    reaction: [0.0; 2],
                    contour: 0.0,
                    mode_shape,
                };
                if !publish_visual_field(&mut self.visual.fields, self.visual_field_cursor, &node.id, field, cx)? {
                    return Ok(self.progress(b"fem2d.eigen-field-admission"));
                }
                self.mode_shape_peak = self.mode_shape_peak.max(mode_shape[0].hypot(mode_shape[1]));
                self.visual_field_cursor += 1;
            }
        }
        Ok(self.progress(b"fem2d.eigen-progress"))
    }
}
