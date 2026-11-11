//! 📤️ Resumable publication of bounded WFC state, output, and diagnostic bytes.

use semio_framework_job_publication::{RetainedJobPublication,JobPublicationKind};
use semio_framework_job::{JobOutcomeBorrow, JobOutcomeDescriptor, JobOutcomeView, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep, StepContext};
use semio_framework_job_payload::{JobPayloadStream, RetainedJobPayload, RetainedPayloadBuilder};
use semio_framework_value::{retirement::record_retirement_work, RetainedAllocationEffect, RetirementDemand, RetainedWorkOutcome, ValueError, ValueRefusalKind};

#[derive(Clone, Copy)]
pub enum Kind {
    Preview,
    Checkpoint(u64),
    Commit,
    Fault,
}

/// 🧳️ One publication owns its source bytes and the original payload headers it lends to the consumer.
pub struct Publication {
    kind: Kind,
    single: RetainedJobPublication,
    state: RetainedPayloadBuilder,
    output: RetainedPayloadBuilder,
    primary: Vec<u8>,
    secondary: Vec<u8>,
    state_cursor: usize,
    output_cursor: usize,
    delivered: bool,
    retiring: bool,
}

impl Publication {
    pub fn new(kind: Kind, primary: Vec<u8>, secondary: Vec<u8>) -> Box<Self> {
        Box::new(Self {
            kind,
            single: RetainedJobPublication::new(),
            state: RetainedPayloadBuilder::new(JobPayloadStream::CommitState),
            output: RetainedPayloadBuilder::new(JobPayloadStream::CommitOutput),
            primary,
            secondary,
            state_cursor: 0,
            output_cursor: 0,
            delivered: false,
            retiring: false,
        })
    }

    pub fn needs_retirement(&self) -> bool {
        self.delivered || self.retiring
    }

    /// 🚨️ Whether this publication already carries the operation's fault.
    pub fn is_fault(&self) -> bool {
        matches!(self.kind, Kind::Fault)
    }

    /// 🤝️ The sealed commit state payload, lent only after the whole commit was delivered.
    pub fn commit_state(&self) -> Option<&RetainedJobPayload> {
        if self.delivered && matches!(self.kind, Kind::Commit) { self.state.published() } else { None }
    }

    /// ✍️ Advances one paid unit toward the lent outcome and reports it once the payload can be borrowed.
    pub fn poll<'a>(&'a mut self, cx: &mut StepContext<'_>) -> Result<Option<JobOutcomeBorrow<'a>>, semio_framework_job::OriginalTurnFailure> {
        let kind = match self.kind {
            Kind::Preview => JobPublicationKind::Preview,
            Kind::Checkpoint(applied_progress) => JobPublicationKind::Checkpoint { applied_progress },
            Kind::Fault => JobPublicationKind::Fault,
            Kind::Commit => return self.poll_commit(cx),
        };
        let result = self.single.advance_from_source(kind, &self.primary, cx)?;
        if result.is_some() {
            self.delivered = true;
        }
        Ok(result)
    }

    fn poll_commit<'a>(&'a mut self, cx: &mut StepContext<'_>) -> Result<Option<JobOutcomeBorrow<'a>>, semio_framework_job::OriginalTurnFailure> {
        if !self.state.is_initialized() {
            let turn=self.state.advance_initialization(cx);
            semio_framework_job::receive_original_retained_work(cx, turn)?;
            return Ok(None);
        }
        if self.state_cursor < self.primary.len() {
            let turn=self.state.append_original(cx, &self.primary, &mut self.state_cursor);
            semio_framework_job::receive_original_retained_work(cx, turn)?;
            return Ok(None);
        }
        if self.state.published().is_none() {
            let turn=self.state.seal(cx);
            semio_framework_job::receive_original_retained_work(cx, turn)?;
            return Ok(None);
        }
        if !self.output.is_initialized() {
            let turn=self.output.advance_initialization(cx);
            semio_framework_job::receive_original_retained_work(cx, turn)?;
            return Ok(None);
        }
        if self.output_cursor < self.secondary.len() {
            let turn=self.output.append_original(cx, &self.secondary, &mut self.output_cursor);
            semio_framework_job::receive_original_retained_work(cx, turn)?;
            return Ok(None);
        }
        if self.output.published().is_none() {
            let turn=self.output.seal(cx);
            semio_framework_job::receive_original_retained_work(cx, turn)?;
            return Ok(None);
        }
        let result = JobOutcomeBorrow::admit_complete(cx, self.state.published(), self.output.published())?;
        if result.is_some() {
            self.delivered = true;
        }
        Ok(result)
    }

    /// 🤝️ Resolves the immutable descriptor against these same original payload headers.
    pub fn borrow_outcome<'a>(&'a self, descriptor: &'a JobOutcomeDescriptor) -> Result<JobOutcomeView<'a>, ValueError> {
        match self.kind {
            Kind::Commit if self.delivered => descriptor.complete(self.state.published(), self.output.published()),
            Kind::Commit => Err(ValueError::literal(ValueRefusalKind::InvariantViolated, "wfc commit publication has not lent its payloads")),
            _ => self.single.borrow_outcome(descriptor),
        }
    }

    /// 📏️ Quotes the next close frontier: pages and headers first, then the source bytes.
    pub fn retirement_demands(&self) -> Result<RetirementDemand, ValueError> {
        if !self.single.terminal_is_empty() {
            return self.single.retirement_demands();
        }
        if !self.state.terminal_is_empty() {
            return self.state.retirement_demands();
        }
        if !self.output.terminal_is_empty() {
            return self.output.retirement_demands();
        }
        if self.primary.capacity() != 0 {
            return Ok(RetirementDemand { release_bytes: self.primary.capacity(), depth: 1, ..Default::default() });
        }
        if self.secondary.capacity() != 0 {
            return Ok(RetirementDemand { release_bytes: self.secondary.capacity(), depth: 1, ..Default::default() });
        }
        Ok(RetirementDemand { depth: usize::from(!self.terminal_is_empty()), ..Default::default() })
    }

    /// ♻️ Advances the next close frontier inside the caller's performed-work effect; a child never completes its parent.
    pub fn close_step(&mut self, effect: &mut RetainedAllocationEffect<'_>) -> RetainedWorkOutcome {
        let grant = effect.grant();
        let pending = |progress: RetainedCloneProgress| Ok(RetainedCloneStep::Progress(progress));
        let result = (|| {
            let demand = self.retirement_demands()?;
            if grant.maximum_items == 0 || grant.maximum_copy_bytes < demand.copy_bytes || grant.maximum_capacity_bytes < demand.capacity_bytes || grant.maximum_release_bytes < demand.release_bytes || grant.maximum_depth < demand.depth {
                return pending(RetainedCloneProgress::default());
            }
            self.retiring = true;
            let nested = grant.maximum_depth.checked_sub(1).map(|maximum_depth| RetainedCloneGrant { maximum_depth, ..grant });
            let child = |nested: Option<RetainedCloneGrant>| nested.ok_or_else(|| ValueError::literal(ValueRefusalKind::DepthLimit, "wfc publication child close exceeds admitted depth"));
            if !self.single.terminal_is_empty() {
                let turn = self.single.close_step(&mut effect.nested(child(nested)?));
                return turn.result.map(|step| RetainedCloneStep::Progress(step.progress()));
            }
            if !self.state.terminal_is_empty() {
                let turn = self.state.close_step(&mut effect.nested(child(nested)?));
                return turn.result.map(|step| RetainedCloneStep::Progress(step.progress()));
            }
            if !self.output.terminal_is_empty() {
                let turn = self.output.close_step(&mut effect.nested(child(nested)?));
                return turn.result.map(|step| RetainedCloneStep::Progress(step.progress()));
            }
            if self.primary.capacity() != 0 {
                record_retirement_work(effect, 1).map_err(|(error, _)| error)?;
                let released_bytes = self.primary.capacity();
                self.primary = Vec::new();
                return pending(RetainedCloneProgress { copied_items: 1, released_bytes, ..Default::default() });
            }
            if self.secondary.capacity() != 0 {
                record_retirement_work(effect, 1).map_err(|(error, _)| error)?;
                let released_bytes = self.secondary.capacity();
                self.secondary = Vec::new();
                return pending(RetainedCloneProgress { copied_items: 1, released_bytes, ..Default::default() });
            }
            if self.delivered {
                record_retirement_work(effect, 1).map_err(|(error, _)| error)?;
                self.delivered = false;
                return pending(RetainedCloneProgress { copied_items: 1, ..Default::default() });
            }
            pending(RetainedCloneProgress::default())
        })();
        RetainedWorkOutcome { result, work: effect.performed_work() }
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.single.terminal_is_empty() && self.state.terminal_is_empty() && self.output.terminal_is_empty() && self.primary.capacity() == 0 && self.secondary.capacity() == 0 && !self.delivered
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
