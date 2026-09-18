//! 📤️ Resumable publication of bounded WFC state, output, and diagnostic bytes.

use semio_framework_job::{
    Checkpoint, CommitCandidate, InteractiveJobCloseStep, JobFault, JobPayloadAdmissionFault, JobPayloadCloseStep, JobPayloadStream, RetainedJobPayload, RetainedJobPayloadWriter, StepContext, StepOutcome, JOB_PAYLOAD_PAGE_BYTES,
};

#[derive(Clone, Copy)]
pub(super) enum Kind {
    Preview,
    Checkpoint(u64),
    Commit,
    Fault,
}

struct Payload {
    bytes: Vec<u8>,
    cursor: usize,
    writer: Option<RetainedJobPayloadWriter>,
    ready: Option<RetainedJobPayload>,
}

impl Payload {
    fn new(stream: JobPayloadStream, bytes: Vec<u8>) -> Self {
        Self { bytes, cursor: 0, writer: Some(RetainedJobPayloadWriter::new(stream)), ready: None }
    }

    fn advance(&mut self, context: &mut StepContext<'_>) -> Result<(), JobPayloadAdmissionFault> {
        let before = self.cursor;
        let complete = self.writer.as_mut().expect("pending publication writer").write_slice_page(context, &self.bytes, &mut self.cursor)?;
        if self.cursor != before {
            context.consume_fuel(1);
        }
        if complete {
            match self.writer.take().expect("completed publication writer").finish() {
                Ok(payload) => self.ready = Some(payload),
                Err(writer) => self.writer = Some(writer),
            }
        }
        Ok(())
    }

    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> InteractiveJobCloseStep {
        if let Some(writer) = self.writer.as_mut() {
            let step = writer.close_step(maximum_items, maximum_bytes);
            if writer.terminal_is_empty() {
                self.writer = None;
            }
            return close_result(step);
        }
        if let Some(payload) = self.ready.as_mut() {
            let step = payload.close_step(maximum_items, maximum_bytes);
            if payload.terminal_is_empty() {
                self.ready = None;
            }
            return close_result(step);
        }
        if !self.bytes.is_empty() {
            let released_bytes = maximum_bytes.min(self.bytes.len());
            if maximum_items == 0 || released_bytes == 0 {
                return InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
            }
            self.bytes.truncate(self.bytes.len() - released_bytes);
            return InteractiveJobCloseStep::Pending { released_items: 1, released_bytes };
        }
        InteractiveJobCloseStep::Complete
    }

    fn terminal_is_empty(&self) -> bool {
        self.writer.is_none() && self.ready.is_none() && self.bytes.is_empty()
    }
}

fn close_result(step: JobPayloadCloseStep) -> InteractiveJobCloseStep {
    match step {
        JobPayloadCloseStep::Pending { released_items, released_bytes } => InteractiveJobCloseStep::Pending { released_items, released_bytes },
        JobPayloadCloseStep::Complete => InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 },
    }
}

pub(super) struct Publication {
    kind: Kind,
    primary: Payload,
    secondary: Option<Payload>,
    failed: bool,
}

impl Publication {
    pub(super) fn new(kind: Kind, primary: Vec<u8>, secondary: Vec<u8>) -> Box<Self> {
        let stream = match kind {
            Kind::Preview => JobPayloadStream::Preview,
            Kind::Checkpoint(_) => JobPayloadStream::CheckpointState,
            Kind::Commit => JobPayloadStream::CommitState,
            Kind::Fault => JobPayloadStream::Fault,
        };
        let secondary = matches!(kind, Kind::Commit).then(|| Payload::new(JobPayloadStream::CommitOutput, secondary));
        Box::new(Self { kind, primary: Payload::new(stream, primary), secondary, failed: false })
    }

    pub(super) fn poll(slot: &mut Option<Box<Self>>, context: &mut StepContext<'_>) -> StepOutcome {
        let pending = slot.as_mut().expect("retained publication");
        if context.is_cancelled() {
            return StepOutcome::Cancelled;
        }
        if context.should_yield() {
            return StepOutcome::Yield;
        }
        if pending.failed {
            let _ = pending.close_step(1, JOB_PAYLOAD_PAGE_BYTES);
            context.consume_fuel(1);
            if pending.terminal_is_empty() {
                *slot = None;
                return StepOutcome::Fault(super::empty_job_fault());
            }
            return StepOutcome::Yield;
        }
        let payload = if pending.primary.ready.is_none() { Some(&mut pending.primary) } else { pending.secondary.as_mut().filter(|payload| payload.ready.is_none()) };
        if let Some(payload) = payload {
            match payload.advance(context) {
                Ok(()) => {}
                Err(JobPayloadAdmissionFault::WriterFull | JobPayloadAdmissionFault::WriterSealed | JobPayloadAdmissionFault::RejectedSourcePending) => pending.failed = true,
                Err(_) => {}
            }
        }
        if pending.failed || pending.primary.ready.is_none() || pending.secondary.as_ref().is_some_and(|payload| payload.ready.is_none()) {
            return StepOutcome::Yield;
        }
        let mut finished = slot.take().expect("finished publication");
        let state = finished.primary.ready.take().expect("published primary payload");
        match finished.kind {
            Kind::Preview => StepOutcome::PreviewReady(state),
            Kind::Checkpoint(applied_progress) => StepOutcome::CheckpointReady(Checkpoint { state, applied_progress }),
            Kind::Commit => StepOutcome::Complete(CommitCandidate { state, output: finished.secondary.as_mut().and_then(|payload| payload.ready.take()).expect("published commit output") }),
            Kind::Fault => StepOutcome::Fault(JobFault { detail: state }),
        }
    }

    pub(super) fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> InteractiveJobCloseStep {
        if !self.primary.terminal_is_empty() {
            return self.primary.close_step(maximum_items, maximum_bytes);
        }
        if let Some(secondary) = self.secondary.as_mut() {
            if !secondary.terminal_is_empty() {
                return secondary.close_step(maximum_items, maximum_bytes);
            }
        }
        InteractiveJobCloseStep::Complete
    }

    pub(super) fn terminal_is_empty(&self) -> bool {
        self.primary.terminal_is_empty() && self.secondary.as_ref().is_none_or(Payload::terminal_is_empty)
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
