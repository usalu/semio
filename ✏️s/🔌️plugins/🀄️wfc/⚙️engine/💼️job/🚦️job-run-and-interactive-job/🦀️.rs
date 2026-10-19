//! 🚦️ Job run and interactive job: the bounded search loop run, the interactive job implementation with its close and the remaining job operations. See [`super`].

use super::*;

//#region 🧩️Job
impl<T: Topology + Clone> WfcJob<T> {
    /// ⏭️ Runs one bounded span of the search loop; a materialized payload is staged, never lent from here.
    pub(super) fn run(&mut self, context: &mut StepContext<'_>) -> WfcRun {
        loop {
            if context.stage() != self.state.stage.label() {
                context.set_stage(self.state.stage.label());
            }
            match self.state.stage {
                WfcStage::InitializeDomains => self.initialize_one(),
                WfcStage::FindMinimumEntropySlot => self.find_slot(),
                WfcStage::ChooseCandidate => self.choose_one(),
                WfcStage::PropagateCompatibilityEdge => self.propagate_one(),
                WfcStage::DetectContradiction => self.detect(),
                WfcStage::BacktrackTrailEntry => self.backtrack_one(),
                WfcStage::CommitSlot => {
                    if self.state.observations == 1 || self.state.observations.is_multiple_of(CHECKPOINT_INTERVAL) {
                        if self.begin_checkpoint(false).is_err() {
                            return self.stage_fault(Vec::new());
                        }
                    } else {
                        self.state.stage = WfcStage::FindMinimumEntropySlot;
                    }
                    let Some(now_us) = context.now_us() else { return WfcRun::Yield };
                    return self.emit_preview(context, now_us / 1_000);
                }
                WfcStage::MaterializeCheckpoint => {
                    let checkpoint = match self.checkpoint_one() {
                        Ok(checkpoint) => checkpoint,
                        Err(_) => return self.stage_fault(Vec::new()),
                    };
                    if let Some(bytes) = checkpoint {
                        let terminal = self.checkpoint_build.as_ref().expect("checkpoint build").terminal;
                        self.checkpoint_build = None;
                        if terminal {
                            self.final_checkpoint = Some(bytes);
                            match CommitBuild::new(self.state.domains.len()) {
                                Ok(build) => self.commit_build = Some(build),
                                Err(_) => return self.stage_fault(Vec::new()),
                            }
                            self.state.stage = WfcStage::MaterializeCommit;
                        } else {
                            self.state.stage = WfcStage::FindMinimumEntropySlot;
                            self.publication = Some(Publication::new(PublicationKind::Checkpoint(self.state.observations), bytes, Vec::new()));
                            return WfcRun::Staged;
                        }
                    }
                }
                WfcStage::MaterializeCommit => {
                    let commit = match self.commit_one() {
                        Ok(commit) => commit,
                        Err(_) => return self.stage_fault(Vec::new()),
                    };
                    if let Some(output) = commit {
                        self.state.stage = WfcStage::Complete;
                        self.publication = Some(Publication::new(PublicationKind::Commit, self.final_checkpoint.take().expect("final checkpoint"), output));
                        return WfcRun::Staged;
                    }
                }
                WfcStage::Complete => {
                    if self.state.contradiction.is_some() || self.state.empty_count > 0 {
                        return self.stage_fault(b"wfc-unsatisfiable".to_vec());
                    }
                    if self.begin_checkpoint(true).is_err() {
                        return self.stage_fault(Vec::new());
                    }
                }
            }
            context.consume_fuel(1);
            self.preview_units = self.preview_units.saturating_add(1);
            if context.is_cancelled() {
                return WfcRun::Cancelled;
            }
            let now_us = context.now_us();
            if let Some(now_ms) = now_us.map(|now_us| now_us / 1_000).filter(|now_ms| self.preview_stage() && self.preview_due(*now_ms)) {
                return self.emit_preview(context, now_ms);
            }
            if context.fuel_exhausted() || now_us.is_none_or(|now_us| now_us >= context.deadline_us()) {
                return WfcRun::Yield;
            }
        }
    }
}

impl<T: Topology + Clone + Send> WfcJob<T> {
    /// 🪜️ The step body; its value refusals are lifted into the original failure at the trait boundary.
    fn step_value<'a>(&'a mut self, context: &mut StepContext<'_>) -> Result<Option<JobOutcomeBorrow<'a>>, ValueError> {
        if self.publication.as_ref().is_some_and(|publication| publication.is_delivered()) && !context.is_cancelled() && context.operation() == self.operation.operation && context.generation() == self.operation.generation {
            return self.retire_delivered_publication(context);
        }
        if context.is_cancelled() {
            return JobOutcomeBorrow::admit_cancelled(context);
        }
        if context.operation() != self.operation.operation || context.generation() != self.operation.generation {
            if self.publication.as_ref().is_some_and(|publication| !publication.is_fault()) {
                return self.retire_delivered_publication(context);
            }
            if self.publication.is_none() {
                self.stage_fault(Vec::new());
            }
        }
        if self.publication.is_some() {
            return self.drive_publication(context);
        }
        match self.run(context) {
            WfcRun::Yield | WfcRun::Staged => Ok(None),
            WfcRun::Cancelled => JobOutcomeBorrow::admit_cancelled(context),
        }
    }
}

impl<T: Topology + Clone + Send> InteractiveJob for WfcJob<T> {
    fn step<'a>(&'a mut self,context:&mut semio_framework_job::StepContext<'_>)->Result<Option<semio_framework_job::JobOutcomeBorrow<'a>>,semio_framework_job::OriginalTurnFailure>{self.step_value(context).map_err(semio_framework_job::OriginalTurnFailure::from)}

    fn borrow_outcome<'a>(&'a self, descriptor: &'a JobOutcomeDescriptor) -> Result<JobOutcomeView<'a>, ValueError> {
        match descriptor.kind() {
            JobOutcomeKind::Yield => descriptor.yielded(),
            JobOutcomeKind::Cancelled => descriptor.cancelled(),
            _ => self.publication.as_ref().ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "wfc outcome has no staged publication"))?.borrow_outcome(descriptor),
        }
    }

    fn begin_close(&mut self) {
        self.closing = true;
    }

    fn close_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<semio_framework_job::InteractiveJobCloseStep,semio_framework_job::OriginalTurnFailure>{
        semio_framework_job::close_original_granted_step(cx,|grant|{
            self.closing = true;
            let demand = match self.close_demands() {
                Ok(demand) => demand,
                Err(error) => return Err(error),
            };
            if self.terminal_is_empty() {
                return Ok(InteractiveJobCloseStep::Complete { progress: RetainedCloneProgress::default() });
            }
            if grant.maximum_items == 0 || grant.maximum_copy_bytes < demand.copy_bytes || grant.maximum_capacity_bytes < demand.capacity_bytes || grant.maximum_release_bytes < demand.release_bytes || grant.maximum_depth < demand.depth {
                return Ok(InteractiveJobCloseStep::Pending { progress: RetainedCloneProgress::default() });
            }
            let one = RetainedCloneProgress { copied_items: 1, ..Default::default() };
            if let Some(publication) = self.publication.as_mut() {
                let step = publication.close_step(grant)?;
                if publication.terminal_is_empty() {
                    self.publication = None;
                }
                return Ok(match step {
                    InteractiveJobCloseStep::Complete { progress } => InteractiveJobCloseStep::Pending { progress },
                    step => step,
                });
            }
            if let Some(bytes) = self.final_checkpoint.take() {
                let released_bytes = bytes.capacity();
                drop(bytes);
                return Ok(InteractiveJobCloseStep::Pending { progress: RetainedCloneProgress { released_bytes, ..one } });
            }
            macro_rules! pop_owner {
                ($owners:expr) => {
                    if $owners.pop().is_some() {
                        return Ok(InteractiveJobCloseStep::Pending { progress: one });
                    }
                };
            }
            if let Some(domains) = self.initial_domains.as_mut() {
                pop_owner!(domains);
                self.initial_domains = None;
                return Ok(InteractiveJobCloseStep::Pending { progress: one });
            }
            pop_owner!(self.fixed);
            pop_owner!(self.state.domains);
            pop_owner!(self.state.domain_counts);
            pop_owner!(self.state.domain_weight_sums);
            pop_owner!(self.state.domain_weighted_log_sums);
            pop_owner!(self.state.revisions);
            if self.state.queue.pop_front().is_some() {
                return Ok(InteractiveJobCloseStep::Pending { progress: one });
            }
            pop_owner!(self.state.queued_marks);
            pop_owner!(self.state.trail);
            pop_owner!(self.state.decisions);
            pop_owner!(self.state.propagation_wave);
            pop_owner!(self.state.changed_domains);
            pop_owner!(self.state.backtrack_path);
            pop_owner!(self.state.observed);
            if self.state.entropy_heap.pop().is_some() || self.checkpoint_build.take().is_some() || self.commit_build.take().is_some() || self.completed_commit.take().is_some() {
                return Ok(InteractiveJobCloseStep::Pending { progress: one });
            }
            Ok(InteractiveJobCloseStep::Complete { progress: RetainedCloneProgress::default() })
        })
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize, ValueError> {
        Ok(self.close_demands()?.copy_bytes)
    }

    fn next_close_capacity_byte_demand(&self, _maximum_copy_bytes: usize) -> Result<usize, ValueError> {
        Ok(self.close_demands()?.capacity_bytes)
    }

    fn next_close_release_byte_demand(&self) -> Result<usize, ValueError> {
        Ok(self.close_demands()?.release_bytes)
    }

    fn next_close_depth_demand(&self) -> Result<usize, ValueError> {
        Ok(self.close_demands()?.depth)
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing
            && self.publication.is_none()
            && self.initial_domains.is_none()
            && self.fixed.is_empty()
            && self.state.domains.is_empty()
            && self.state.domain_counts.is_empty()
            && self.state.domain_weight_sums.is_empty()
            && self.state.domain_weighted_log_sums.is_empty()
            && self.state.revisions.is_empty()
            && self.state.queue.is_empty()
            && self.state.queued_marks.is_empty()
            && self.state.trail.is_empty()
            && self.state.decisions.is_empty()
            && self.state.entropy_heap.is_empty()
            && self.state.propagation_wave.is_empty()
            && self.state.changed_domains.is_empty()
            && self.state.backtrack_path.is_empty()
            && self.state.observed.is_empty()
            && self.checkpoint_build.is_none()
            && self.final_checkpoint.is_none()
            && self.commit_build.is_none()
            && self.completed_commit.is_none()
    }
}

impl<T: Topology + Clone + Send> WfcJob<T> {
    /// 🤝️ Closes the lent payload turn by turn from the next step's own wallet before the search resumes.
    pub(super) fn retire_delivered_publication<'a>(&'a mut self, context: &mut StepContext<'_>) -> Result<Option<JobOutcomeBorrow<'a>>, ValueError> {
        let publication = self.publication.as_mut().expect("a delivered wfc publication is staged");
        let step = match publication.close_step(context.retained_grant()) { Ok(step) => step, Err(error) => { context.consume_retained(error.retained_progress())?; return Err(error) } };
        context.consume_retained(step.progress())?;
        if publication.terminal_is_empty() {
            self.publication = None;
        }
        Ok(None)
    }

    pub(super) fn drive_publication<'a>(&'a mut self, context: &mut StepContext<'_>) -> Result<Option<JobOutcomeBorrow<'a>>, ValueError> {
        self.publication.as_mut().expect("a staged wfc publication").poll(context)
    }

    /// 📏️ Quotes the next close turn: the staged publication, the final checkpoint bytes, or one popped owner.
    pub(super) fn close_demands(&self) -> Result<RetirementDemand, ValueError> {
        if let Some(publication) = self.publication.as_ref() {
            return publication.retirement_demands();
        }
        if let Some(bytes) = self.final_checkpoint.as_ref() {
            return Ok(RetirementDemand { release_bytes: bytes.capacity(), depth: 1, ..Default::default() });
        }
        Ok(RetirementDemand { depth: usize::from(!self.terminal_is_empty()), ..Default::default() })
    }
}
//#endregion 🧩️Job
