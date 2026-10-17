//! 🔁️ Job lifecycle of the WFC 3D inference: the checkpoint, progress and publication members and the `InteractiveJob` implementation. See [`super`].

use super::*;

impl Wfc3dInferenceJob {
    /// ⏭️ Consumes the child's end: its commit is taken, its sealed state payload stays lent from the child until close.
    fn finish_child(&mut self, context: &mut semio_framework_job::StepContext<'_>, _end: InferenceChildEnd) -> Wfc3dInferenceJobRun {
        self.child_commit = self.child.as_mut().expect("WFC child").take_completed_commit();
        self.cursor = 0;
        self.stage = Wfc3dInferenceStage::MapCommit;
        self.stage_preview(context)
    }

    /// ⏭️ Runs the bounded compile, map and encode spans; a child or restore turn is requested, never taken here.
    fn run(&mut self, context: &mut semio_framework_job::StepContext<'_>) -> Wfc3dInferenceJobRun {
        loop {
            context.set_stage(match self.stage {
                Wfc3dInferenceStage::Tiles => "wfc3d.infer.tiles",
                Wfc3dInferenceStage::Relations => "wfc3d.infer.relations",
                Wfc3dInferenceStage::Rules => "wfc3d.infer.rules",
                Wfc3dInferenceStage::Model => "wfc3d.infer.model",
                Wfc3dInferenceStage::Compile => "wfc3d.infer.compile",
                Wfc3dInferenceStage::Slots => "wfc3d.infer.slots",
                Wfc3dInferenceStage::Edges => "wfc3d.infer.edges",
                Wfc3dInferenceStage::Topology => "wfc3d.infer.topology",
                Wfc3dInferenceStage::Fixed => "wfc3d.infer.fixed",
                Wfc3dInferenceStage::Restore => "wfc3d.infer.restore",
                Wfc3dInferenceStage::Solve => "wfc3d.infer.solve",
                Wfc3dInferenceStage::MapCommit => "wfc3d.infer.map-commit",
                Wfc3dInferenceStage::EncodeCommit => "wfc3d.infer.encode-commit",
                Wfc3dInferenceStage::Complete => "wfc3d.infer.complete",
            });
            match self.stage {
                Wfc3dInferenceStage::Tiles
                | Wfc3dInferenceStage::Relations
                | Wfc3dInferenceStage::Rules
                | Wfc3dInferenceStage::Model
                | Wfc3dInferenceStage::Compile
                | Wfc3dInferenceStage::Slots
                | Wfc3dInferenceStage::Edges
                | Wfc3dInferenceStage::Topology
                | Wfc3dInferenceStage::Fixed => {
                    if let Err(error) = self.advance_compile() {
                        return self.stage_fault(error.into_bytes());
                    }
                }
                Wfc3dInferenceStage::Restore => {
                    if self.restore_done {
                        self.child = self.restore.as_mut().expect("restore job").take_job();
                        if let Some(mut restore) = self.restore.take() {
                            engine::job::close_job(&mut restore);
                        }
                        self.restore_done = false;
                        self.stage = Wfc3dInferenceStage::Solve;
                        return self.stage_preview(context);
                    }
                    return Wfc3dInferenceJobRun::StepRestore;
                }
                Wfc3dInferenceStage::Solve => {
                    let Some(end) = self.child_end.take() else {
                        return Wfc3dInferenceJobRun::StepChild;
                    };
                    return self.finish_child(context, end);
                }
                Wfc3dInferenceStage::MapCommit => {
                    if let Err(error) = self.map_one() {
                        return self.stage_fault(error.into_bytes());
                    }
                }
                Wfc3dInferenceStage::EncodeCommit => match self.encode_one(context) {
                    Ok(true) => {
                        let output = match self.output.take().expect("wfc3d output writer").finish() {
                            Ok(output) => output,
                            Err(mut writer) => {
                                writer.begin_close();
                                self.output = Some(writer);
                                return Wfc3dInferenceJobRun::Yield;
                            }
                        };
                        self.output_payload = Some(output);
                        return Wfc3dInferenceJobRun::Yield;
                    }
                    Ok(false) => return Wfc3dInferenceJobRun::Yield,
                    Err(error) => return self.stage_fault(error.into_bytes()),
                },
                Wfc3dInferenceStage::Complete => unreachable!("complete returns immediately"),
            }
            context.consume_fuel(1);
            self.preview_units = self.preview_units.saturating_add(1);
            if context.is_cancelled() {
                return Wfc3dInferenceJobRun::Cancelled;
            }
            if context.now_us().is_some_and(|now_us| self.preview_due(now_us / 1_000)) {
                return self.stage_preview(context);
            }
            if context.should_yield() {
                return Wfc3dInferenceJobRun::Yield;
            }
        }
    }

    /// 🤝️ Closes the lent preview or fault turn by turn from the next step's own wallet.
    fn retire_delivered<'a>(&'a mut self, context: &mut semio_framework_job::StepContext<'_>) -> Result<Option<semio_framework_job::JobOutcomeBorrow<'a>>, semio_framework_value::ValueError> {
        let publication = self.publication.as_mut().expect("a delivered inference publication is staged");
        let step = publication.close_step(context.retained_grant());
        context.consume_retained(step.progress())?;
        if let semio_framework_job::InteractiveJobCloseStep::Refused { kind, progress } = step {
            return Err(semio_framework_value::ValueError::literal(kind, "inference publication close was refused").with_retained_progress(progress));
        }
        if publication.terminal_is_empty() {
            self.publication = None;
        }
        Ok(None)
    }

    /// 📏️ Quotes the next close turn in the order the ladder spends them.
    fn close_demands(&self) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        let nested = |mut demand: semio_framework_value::RetirementDemand| -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
            demand.depth = demand.depth.checked_add(1).ok_or_else(|| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "inference close depth overflow"))?;
            Ok(demand)
        };
        if let Some(publication) = self.publication.as_ref() {
            return publication.retirement_demands();
        }
        if let Some(output) = self.output.as_ref() {
            return if output.terminal_is_empty() { Ok(semio_framework_value::RetirementDemand { depth: 1, ..Default::default() }) } else { nested(output.retirement_demands()?) };
        }
        if let Some(payload) = self.output_payload.as_ref() {
            return if payload.terminal_is_empty() { Ok(semio_framework_value::RetirementDemand { depth: 1, ..Default::default() }) } else { nested(payload.retirement_demands()?) };
        }
        if let Some(restore) = self.restore.as_ref() {
            return Ok(semio_framework_value::RetirementDemand { copy_bytes: restore.next_close_copy_byte_demand()?, capacity_bytes: restore.next_close_capacity_byte_demand(0)?, release_bytes: restore.next_close_release_byte_demand()?, depth: restore.next_close_depth_demand()?.saturating_add(1) });
        }
        if let Some(child) = self.child.as_ref() {
            return Ok(semio_framework_value::RetirementDemand { copy_bytes: child.next_close_copy_byte_demand()?, capacity_bytes: child.next_close_capacity_byte_demand(0)?, release_bytes: child.next_close_release_byte_demand()?, depth: child.next_close_depth_demand()?.saturating_add(1) });
        }
        if let Some(source) = self.rejected_output_page.as_ref() {
            return Ok(semio_framework_value::RetirementDemand { release_bytes: source.allocated_capacity_bytes(), depth: 1, ..Default::default() });
        }
        Ok(semio_framework_value::RetirementDemand::default())
    }
}

impl InteractiveJob for Wfc3dInferenceJob {
    fn step<'a>(&'a mut self, context: &mut semio_framework_job::StepContext<'_>) -> Result<Option<semio_framework_job::JobOutcomeBorrow<'a>>, semio_framework_value::ValueError> {
        use semio_framework_job::JobOutcomeBorrow;
        if self.publication.as_ref().is_some_and(|publication| publication.is_delivered()) {
            return self.retire_delivered(context);
        }
        if context.is_cancelled() {
            return JobOutcomeBorrow::admit_cancelled(context);
        }
        if context.operation() != self.operation.operation || context.generation() != self.operation.generation {
            self.stage_fault(b"stale-wfc3d-inference-operation".to_vec());
        }
        if self.publication.is_some() {
            self.lent = Wfc3dInferenceJobLent::Own;
            return self.publication.as_mut().expect("a staged inference publication").poll(context);
        }
        if self.output_payload.is_some() {
            return JobOutcomeBorrow::admit_complete(context, self.child.as_ref().and_then(|child| child.commit_state()), self.output_payload.as_ref());
        }
        match self.run(context) {
            Wfc3dInferenceJobRun::Yield | Wfc3dInferenceJobRun::Staged => Ok(None),
            Wfc3dInferenceJobRun::Cancelled => JobOutcomeBorrow::admit_cancelled(context),
            Wfc3dInferenceJobRun::StepRestore => {
                let result = self.restore.as_mut().expect("restore job").step(context)?;
                if matches!(result, Some(JobOutcomeBorrow::Complete { .. })) {
                    self.restore_done = true;
                    return Ok(None);
                }
                self.lent = Wfc3dInferenceJobLent::Restore;
                Ok(result)
            }
            Wfc3dInferenceJobRun::StepChild => {
                let result = self.child.as_mut().expect("WFC child").step(context)?;
                let end = if matches!(result, Some(JobOutcomeBorrow::Complete { .. })) {
                    Some(InferenceChildEnd::Complete)
                } else {
                    None
                };
                if let Some(end) = end {
                    self.child_end = Some(end);
                    return Ok(None);
                }
                self.lent = Wfc3dInferenceJobLent::Child;
                Ok(result)
            }
        }
    }

    fn borrow_outcome<'a>(&'a self, descriptor: &'a semio_framework_job::JobOutcomeDescriptor) -> Result<semio_framework_job::JobOutcomeView<'a>, semio_framework_value::ValueError> {
        use semio_framework_job::JobOutcomeKind;
        let absent = || semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "inference outcome has no retained owner");
        match descriptor.kind() {
            JobOutcomeKind::Yield => descriptor.yielded(),
            JobOutcomeKind::Cancelled => descriptor.cancelled(),
            JobOutcomeKind::Complete => descriptor.complete(self.child.as_ref().and_then(|child| child.commit_state()), self.output_payload.as_ref()),
            _ => match self.lent {
                Wfc3dInferenceJobLent::Own => self.publication.as_ref().ok_or_else(absent)?.borrow_outcome(descriptor),
                Wfc3dInferenceJobLent::Restore => self.restore.as_ref().ok_or_else(absent)?.borrow_outcome(descriptor),
                Wfc3dInferenceJobLent::Child => self.child.as_ref().ok_or_else(absent)?.borrow_outcome(descriptor),
            },
        }
    }

    fn begin_close(&mut self) {
        if let Some(restore) = self.restore.as_mut() {
            restore.begin_close();
        }
        if let Some(child) = self.child.as_mut() {
            child.begin_close();
        }
        if let Some(output) = self.output.as_mut() {
            output.begin_close();
        }
    }

    fn close_step(&mut self, grant: semio_framework_job::RetainedCloneGrant) -> semio_framework_job::InteractiveJobCloseStep {
        use semio_framework_job::{InteractiveJobCloseStep, RetainedCloneProgress};
        let refused = |error: semio_framework_value::ValueError| InteractiveJobCloseStep::Refused { kind: error.kind, progress: error.retained_progress() };
        let demand = match self.close_demands() {
            Ok(demand) => demand,
            Err(error) => return refused(error),
        };
        if self.terminal_is_empty() {
            return InteractiveJobCloseStep::Complete { progress: RetainedCloneProgress::default() };
        }
        if grant.maximum_items == 0 || grant.maximum_copy_bytes < demand.copy_bytes || grant.maximum_capacity_bytes < demand.capacity_bytes || grant.maximum_release_bytes < demand.release_bytes || grant.maximum_depth < demand.depth {
            return InteractiveJobCloseStep::Pending { progress: RetainedCloneProgress::default() };
        }
        let one = RetainedCloneProgress { copied_items: 1, ..Default::default() };
        let child_grant = semio_framework_job::RetainedCloneGrant { maximum_depth: grant.maximum_depth.saturating_sub(1), ..grant };
        if let Some(publication) = self.publication.as_mut() {
            let step = publication.close_step(grant);
            if publication.terminal_is_empty() {
                self.publication = None;
            }
            return match step {
                InteractiveJobCloseStep::Complete { progress } => InteractiveJobCloseStep::Pending { progress },
                step => step,
            };
        }
        if let Some(output) = self.output.as_mut() {
            if output.terminal_is_empty() {
                self.output = None;
                return InteractiveJobCloseStep::Pending { progress: one };
            }
            return match output.close_step(child_grant) {
                Ok(step) => InteractiveJobCloseStep::Pending { progress: step.progress() },
                Err(error) => refused(error),
            };
        }
        if let Some(payload) = self.output_payload.as_mut() {
            if payload.terminal_is_empty() {
                self.output_payload = None;
                return InteractiveJobCloseStep::Pending { progress: one };
            }
            return match payload.close_step(child_grant) {
                Ok(step) => InteractiveJobCloseStep::Pending { progress: step.progress() },
                Err(error) => refused(error),
            };
        }
        if let Some(restore) = self.restore.as_mut() {
            let step = restore.close_step(child_grant);
            if restore.terminal_is_empty() {
                self.restore = None;
                return InteractiveJobCloseStep::Pending { progress: RetainedCloneProgress { copied_items: step.progress().copied_items.max(1), ..step.progress() } };
            }
            return match step {
                InteractiveJobCloseStep::Complete { progress } => InteractiveJobCloseStep::Pending { progress },
                step => step,
            };
        }
        if let Some(child) = self.child.as_mut() {
            let step = child.close_step(child_grant);
            if child.terminal_is_empty() {
                self.child = None;
                return InteractiveJobCloseStep::Pending { progress: RetainedCloneProgress { copied_items: step.progress().copied_items.max(1), ..step.progress() } };
            }
            return match step {
                InteractiveJobCloseStep::Complete { progress } => InteractiveJobCloseStep::Pending { progress },
                step => step,
            };
        }
        if let Some(source) = self.rejected_output_page.take() {
            let released_bytes = source.allocated_capacity_bytes();
            drop(source);
            return InteractiveJobCloseStep::Pending { progress: RetainedCloneProgress { released_bytes, ..one } };
        }
        InteractiveJobCloseStep::Complete { progress: RetainedCloneProgress::default() }
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_demands()?.copy_bytes)
    }

    fn next_close_capacity_byte_demand(&self, _maximum_copy_bytes: usize) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_demands()?.capacity_bytes)
    }

    fn next_close_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_demands()?.release_bytes)
    }

    fn next_close_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_demands()?.depth)
    }

    /// 🚪️ Ownership ONLY — deliberately NOT gated on a closing flag: the retained session calls `begin_close`
    /// itself, and a flag-gated predicate would spin its own close ladder forever.
    fn terminal_is_empty(&self) -> bool {
        self.publication.is_none() && self.output.is_none() && self.output_payload.is_none() && self.restore.is_none() && self.child.is_none() && self.rejected_output_page.is_none()
    }
}
