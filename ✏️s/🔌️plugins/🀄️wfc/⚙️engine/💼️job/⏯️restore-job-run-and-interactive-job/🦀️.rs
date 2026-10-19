//! ⏯️ Restore job run and interactive job: the restore run outcome, the restore job driving and its interactive job implementation. See [`super`].

use super::*;

//#region 🧩️Job
//#region 🔄️RestoreJob
/// 🚦️ What one bounded restore run decided before any outcome is lent.
enum WfcRestoreRun {
    Yield,
    Cancelled,
    Staged,
    Complete,
}

impl<T: Topology + Clone + Send> WfcRestore<T> {
    /// ⏭️ Decodes under the step's fuel and deadline; payloads are staged, never lent from here.
    pub(super) fn run(&mut self, context: &mut StepContext<'_>) -> WfcRestoreRun {
        loop {
            context.set_stage("wfc.restore");
            if let Err(error) = self.decode_one() {
                self.publication = Some(Publication::new(PublicationKind::Fault, error.into_bytes(), Vec::new()));
                return WfcRestoreRun::Staged;
            }
            if self.stage == RestoreStage::Complete && self.restored.is_some() {
                return WfcRestoreRun::Complete;
            }
            context.consume_fuel(1);
            self.preview_units = self.preview_units.saturating_add(1);
            if context.is_cancelled() {
                return WfcRestoreRun::Cancelled;
            }
            let Some(now_us) = context.now_us() else { return WfcRestoreRun::Yield };
            let now_ms = now_us / 1_000;
            if self.last_preview_ms.is_none() || self.preview_units >= PREVIEW_UNIT_INTERVAL || self.last_preview_ms.is_some_and(|last| now_ms.saturating_sub(last) >= PREVIEW_TIME_INTERVAL_MS) {
                let (completed, total) = self.progress();
                let sequence = match context.next_preview_sequence() {
                    Ok(sequence) => sequence,
                    Err(_) => {
                        self.publication = Some(Publication::new(PublicationKind::Fault, Vec::new(), Vec::new()));
                        return WfcRestoreRun::Staged;
                    }
                };
                let preview = RestorePreview { sequence, stage: self.stage, completed, total };
                self.preview_units = 0;
                self.last_preview_ms = Some(now_ms);
                let bytes = semio_framework_pack_json::to_json_string(&preview).into_bytes();
                self.publication = Some(Publication::new(PublicationKind::Preview, bytes, Vec::new()));
                return WfcRestoreRun::Staged;
            }
            if context.fuel_exhausted() || now_us >= context.deadline_us() {
                return WfcRestoreRun::Yield;
            }
        }
    }

    /// 🤝️ Closes the lent payload turn by turn from the next step's own wallet before decoding resumes.
    pub(super) fn retire_delivered_publication<'a>(&'a mut self, context: &mut StepContext<'_>) -> Result<Option<JobOutcomeBorrow<'a>>, ValueError> {
        let publication = self.publication.as_mut().expect("a delivered wfc restore publication is staged");
        let step = match publication.close_step(context.retained_grant()) { Ok(step) => step, Err(error) => { context.consume_retained(error.retained_progress())?; return Err(error) } };
        context.consume_retained(step.progress())?;
        if publication.terminal_is_empty() {
            self.publication = None;
        }
        Ok(None)
    }

    pub(super) fn drive_publication<'a>(&'a mut self, context: &mut StepContext<'_>) -> Result<Option<JobOutcomeBorrow<'a>>, ValueError> {
        self.publication.as_mut().expect("a staged wfc restore publication").poll(context)
    }

    /// 📏️ Quotes the next close turn: the staged publication, the restored job's own frontier, the raw bytes, or one popped owner.
    pub(super) fn close_demands(&self) -> Result<RetirementDemand, ValueError> {
        if let Some(publication) = self.publication.as_ref() {
            return publication.retirement_demands();
        }
        if let Some(restored) = self.restored.as_ref() {
            return Ok(RetirementDemand { copy_bytes: restored.next_close_copy_byte_demand()?, capacity_bytes: restored.next_close_capacity_byte_demand(0)?, release_bytes: restored.next_close_release_byte_demand()?, depth: restored.next_close_depth_demand()?.saturating_add(1) });
        }
        if self.bytes.capacity() != 0 {
            return Ok(RetirementDemand { release_bytes: self.bytes.capacity(), depth: 1, ..Default::default() });
        }
        Ok(RetirementDemand { depth: usize::from(!self.terminal_is_empty()), ..Default::default() })
    }
}

impl<T: Topology + Clone + Send> WfcRestore<T> {
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
                self.publication = Some(Publication::new(PublicationKind::Fault, Vec::new(), Vec::new()));
            }
        }
        if self.publication.is_some() {
            return self.drive_publication(context);
        }
        match self.run(context) {
            WfcRestoreRun::Yield | WfcRestoreRun::Staged => Ok(None),
            WfcRestoreRun::Cancelled => JobOutcomeBorrow::admit_cancelled(context),
            WfcRestoreRun::Complete => JobOutcomeBorrow::admit_complete(context, None, None),
        }
    }
}

impl<T: Topology + Clone + Send> InteractiveJob for WfcRestore<T> {
    fn step<'a>(&'a mut self,context:&mut semio_framework_job::StepContext<'_>)->Result<Option<semio_framework_job::JobOutcomeBorrow<'a>>,semio_framework_job::OriginalTurnFailure>{self.step_value(context).map_err(semio_framework_job::OriginalTurnFailure::from)}

    fn borrow_outcome<'a>(&'a self, descriptor: &'a JobOutcomeDescriptor) -> Result<JobOutcomeView<'a>, ValueError> {
        match descriptor.kind() {
            JobOutcomeKind::Yield => descriptor.yielded(),
            JobOutcomeKind::Cancelled => descriptor.cancelled(),
            JobOutcomeKind::Complete => descriptor.complete(None, None),
            _ => self.publication.as_ref().ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "wfc restore outcome has no staged publication"))?.borrow_outcome(descriptor),
        }
    }

    fn begin_close(&mut self) {
        self.closing = true;
        if let Some(restored) = self.restored.as_mut() {
            restored.begin_close();
        }
    }

    fn close_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<semio_framework_job::InteractiveJobCloseStep,semio_framework_job::OriginalTurnFailure>{
        semio_framework_job::close_original_nested_step(cx,|cx,grant|{
            self.begin_close();
            let demand = match self.close_demands() {
                Ok(demand) => demand,
                Err(error) => return Err(semio_framework_job::OriginalTurnFailure::debit_original(error,cx)),
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
            if let Some(restored) = self.restored.as_mut() {
                let child = RetainedCloneGrant { maximum_depth: grant.maximum_depth.saturating_sub(1), ..grant };
                let step = cx.with_retained_loan(child, |same| restored.close_step(same))?;
                if restored.terminal_is_empty() {
                    self.restored = None;
                    return Ok(InteractiveJobCloseStep::Pending { progress: RetainedCloneProgress { copied_items: step.progress().copied_items.max(1), ..step.progress() } });
                }
                return Ok(match step {
                    InteractiveJobCloseStep::Complete { progress } => InteractiveJobCloseStep::Pending { progress },
                    step => step,
                });
            }
            if self.bytes.capacity() != 0 {
                let released_bytes = self.bytes.capacity();
                self.bytes = Vec::new();
                return Ok(InteractiveJobCloseStep::Pending { progress: RetainedCloneProgress { released_bytes, ..one } });
            }
            macro_rules! pop_owner {
                ($owners:expr) => {
                    if $owners.pop().is_some() {
                        return Ok(InteractiveJobCloseStep::Pending { progress: one });
                    }
                };
            }
            pop_owner!(self.fixed);
            if let Some(domains) = self.initial_domains.as_mut() {
                pop_owner!(domains);
                self.initial_domains = None;
                return Ok(InteractiveJobCloseStep::Pending { progress: one });
            }
            pop_owner!(self.domains);
            pop_owner!(self.domain_words);
            pop_owner!(self.trail);
            pop_owner!(self.decisions);
            pop_owner!(self.observed);
            pop_owner!(self.domain_counts);
            pop_owner!(self.weight_sums);
            pop_owner!(self.weighted_log_sums);
            pop_owner!(self.revisions);
            pop_owner!(self.queued_marks);
            if self.entropy_heap.pop().is_some() || self.header.take().is_some() || self.model.take().is_some() || self.topology.take().is_some() {
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
            && self.model.is_none()
            && self.topology.is_none()
            && self.initial_domains.is_none()
            && self.fixed.is_empty()
            && self.bytes.is_empty()
            && self.header.is_none()
            && self.domains.is_empty()
            && self.domain_words.is_empty()
            && self.trail.is_empty()
            && self.decisions.is_empty()
            && self.observed.is_empty()
            && self.domain_counts.is_empty()
            && self.weight_sums.is_empty()
            && self.weighted_log_sums.is_empty()
            && self.revisions.is_empty()
            && self.queued_marks.is_empty()
            && self.entropy_heap.is_empty()
            && self.restored.is_none()
    }
}
//#endregion 🔄️RestoreJob
//#endregion 🧩️Job
