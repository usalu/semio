//! 📥️ Drives the retained atomic receiving pages on the exact attached document port.

use super::*;
use semio_framework_value::retirement::controlled::ControlledRetirement;
use semio_os_spr::{CausalPayload, VerifiedAtomicEdit};

/// 🐚️ Parks an incomplete physical group and resumes funded verification after its last part.
#[derive(Default)]
pub struct AtomicBackboneInbox {
    hot: Option<AtomicPortFrame>,
    frame: Option<CausalPayload>,
    receiver: Option<AtomicBackboneReceive>,
    retiring_frame: Option<ControlledRetirement<CausalPayload>>,
    ready: Option<VerifiedAtomicEdit>,
}

struct AtomicPortFrame {
    original: Option<Vec<u8>>,
    body_start: usize,
    body: Option<CausalPayload>,
}

#[derive(Default)]
pub struct AtomicDocumentBackboneWork {
    pub inbox: AtomicBackboneInbox,
    pub original: Option<Vec<u8>>,
    pub outbound: Option<AtomicBackboneDelivery>,
}

impl AtomicDocumentBackboneWork {
    pub fn input_pending(&self) -> bool { self.original.is_some() || self.inbox.runnable() }

    pub fn pending(&self) -> bool { self.input_pending() || self.outbound.is_some() }

    pub fn terminal(&self) -> bool { self.original.is_none() && self.inbox.original_frames_terminal() && self.inbox.ready().is_none() && self.outbound.is_none() }
}

pub enum AtomicInboxStep {
    Waiting,
    Runnable,
    Ready,
}

impl AtomicBackboneInbox {
    pub fn recognizes_hot(wire: &[u8]) -> bool { atomic_wire_front(wire) }

    /// 📩️ Takes the original event buffer only after its exact transport structure and owner are live.
    pub fn take_hot(&mut self, original: &mut Option<Vec<u8>>, effect: &mut RetainedAllocationEffect<'_>) -> Result<RetainedCloneProgress, VcsError> {
        let wire = original.as_ref().ok_or_else(|| VcsError::InverseRefused(invalid()))?;
        let body_start = parse_atomic_hot(wire)?;
        if self.hot.is_some() || self.frame.is_some() || self.ready.is_some() || self.retiring_frame.is_some() {
            return Err(VcsError::InverseRefused(invalid()));
        }
        let demand = RetirementDemand { copy_bytes: size_of::<AtomicPortFrame>() + size_of::<Option<Vec<u8>>>(), depth: 1, ..Default::default() };
        if !retained_clone_grant_admits(effect.grant(), demand) || !effect.performed(1).map_err(VcsError::InverseRefused)? {
            return Ok(Default::default());
        }
        self.hot = Some(AtomicPortFrame { original: original.take(), body_start, body: Some(CausalPayload::default()) });
        Ok(progress(demand))
    }

    /// 🕰️ Arms another funded actor turn only when a real receive or close frontier remains.
    pub fn runnable(&self) -> bool {
        self.hot.is_some() || self.frame.is_some() || self.retiring_frame.is_some() || self.ready.is_some() ||
            self.receiver.as_ref().is_some_and(AtomicBackboneReceive::can_seal)
    }

    fn advance_hot(&mut self, effect: &mut RetainedAllocationEffect<'_>) -> Result<RetainedCloneProgress, VcsError> {
        let hot = self.hot.as_mut().unwrap();
        let wire = hot.original.as_ref().unwrap();
        let body = hot.body.as_mut().unwrap();
        let from = hot.body_start + body.len();
        if from == wire.len() {
            let demand = RetirementDemand {
                copy_bytes: size_of::<Option<AtomicPortFrame>>() + size_of::<Option<CausalPayload>>(),
                release_bytes: wire.capacity(), depth: 1, ..Default::default()
            };
            if !retained_clone_grant_admits(effect.grant(), demand) || !effect.performed(1).map_err(VcsError::InverseRefused)? {
                return Ok(Default::default());
            }
            self.frame = hot.body.take();
            self.hot = None;
            return Ok(progress(demand));
        }
        let demand = body.byte_backing_demand().map_err(VcsError::InverseRefused)?;
        if demand != Default::default() {
            if !retained_clone_grant_admits(effect.grant(), demand) || !effect.performed(1).map_err(VcsError::InverseRefused)? {
                return Ok(Default::default());
            }
            let mut receipt = body.reserve_byte_backing(effect.grant()).map_err(VcsError::InverseRefused)?;
            receipt.copied_bytes += demand.copy_bytes;
            return Ok(receipt);
        }
        let window = wire.len().saturating_sub(from).min(effect.grant().maximum_copy_bytes).min(4096);
        if effect.grant().maximum_items == 0 || effect.grant().maximum_depth == 0 { return Ok(Default::default()); }
        let mut copied = 0;
        for &byte in &wire[from..from + window] {
            if body.byte_backing_demand().map_err(VcsError::InverseRefused)? != Default::default() ||
                !effect.performed(1).map_err(VcsError::InverseRefused)? { break; }
            body.push_reserved_byte(byte).map_err(VcsError::InverseRefused)?;
            copied += 1;
        }
        Ok(RetainedCloneProgress { copied_items: usize::from(copied != 0), copied_bytes: copied, ..Default::default() })
    }

    /// 📥️ Holds only one frontier from the authenticated port and preserves it under a short grant.
    pub fn advance(
        &mut self, input: &mut Option<CausalPayload>, binding: [u8; 32], document: &str, effect: &mut RetainedAllocationEffect<'_>,
    ) -> Result<(AtomicInboxStep, RetainedCloneProgress), VcsError> {
        if self.ready.is_some() { return Ok((AtomicInboxStep::Ready, Default::default())); }
        if self.hot.is_some() {
            return self.advance_hot(effect).map(|receipt| (AtomicInboxStep::Runnable, receipt));
        }
        if self.receiver.as_ref().is_some_and(AtomicBackboneReceive::is_sealed) {
            return Ok((AtomicInboxStep::Runnable, Default::default()));
        }
        if self.frame.is_none() && !self.receiver.as_ref().is_some_and(AtomicBackboneReceive::can_seal) {
            if input.is_none() { return Ok((AtomicInboxStep::Waiting, Default::default())); }
            let demand = RetirementDemand { copy_bytes: 2 * size_of::<Option<CausalPayload>>(), depth: 1, ..Default::default() };
            if !retained_clone_grant_admits(effect.grant(), demand) || !effect.performed(1).map_err(VcsError::InverseRefused)? {
                return Ok((AtomicInboxStep::Runnable, Default::default()));
            }
            self.frame = input.take();
            return Ok((AtomicInboxStep::Runnable, progress(demand)));
        }
        if self.receiver.is_none() {
            let original = self.frame.as_ref().ok_or_else(|| VcsError::InverseRefused(invalid()))?;
            if let Some((owner, receipt)) = AtomicBackboneReceive::admit(original, binding, effect).map_err(VcsError::InverseRefused)? {
                self.receiver = Some(owner);
                return Ok((AtomicInboxStep::Runnable, receipt));
            }
            return Ok((AtomicInboxStep::Runnable, Default::default()));
        }
        let receiver = self.receiver.as_mut().unwrap();
        if self.frame.is_some() {
            let receipt = receiver.push(&binding, &mut self.frame, effect).map_err(VcsError::InverseRefused)?;
            return Ok((AtomicInboxStep::Runnable, receipt));
        }
        let (ready, receipt) = receiver.seal_at_document(&binding, document, effect)?;
        self.ready = ready;
        Ok((if self.ready.is_some() { AtomicInboxStep::Ready } else { AtomicInboxStep::Runnable }, receipt))
    }

    /// 🎟️ Lends the whole authored group to the Store until its single publication accepts.
    pub fn ready(&self) -> Option<&VerifiedAtomicEdit> { self.ready.as_ref() }

    pub fn accept_ready(&mut self) -> Option<VerifiedAtomicEdit> { self.ready.take() }

    pub fn cleanup_due(&self) -> bool {
        self.receiver.as_ref().is_some_and(AtomicBackboneReceive::is_sealed)
    }

    pub fn close(&mut self, effect: &mut RetainedAllocationEffect<'_>) -> OriginalRetirementPoll {
        let result = self.close_original_frames(effect).map(|receipt| {
            if self.original_frames_terminal() { RetainedCloneStep::Complete(receipt) } else { RetainedCloneStep::Progress(receipt) }
        }).map_err(|error| ValueError::new(ValueRefusalKind::InvariantViolated, error.to_string()).into());
        RetirementPollExit::settle(effect, result)
    }

    pub fn close_receiver(&mut self, effect: &mut RetainedAllocationEffect<'_>) -> Result<RetainedCloneProgress, VcsError> {
        let Some(owner) = self.receiver.as_mut() else { return Ok(Default::default()); };
        let receipt = match RetirementPollExit::resume(owner.close(4096, effect)) {
            Ok(step) => step.progress(),
            Err(RetirementPollExit::Pending(_)) => Default::default(),
            Err(RetirementPollExit::Failed(failure)) => return Err(VcsError::InverseRefused(failure.into_value_error())),
        };
        if owner.terminal_is_empty() { self.receiver = None; }
        Ok(receipt)
    }

    pub fn close_original_frames(&mut self, effect: &mut RetainedAllocationEffect<'_>) -> Result<RetainedCloneProgress, VcsError> {
        if let Some(hot) = self.hot.as_mut() {
            let demand = RetirementDemand { copy_bytes: size_of::<Option<AtomicPortFrame>>() + size_of::<Option<CausalPayload>>(),
                release_bytes: hot.original.as_ref().map_or(0, Vec::capacity), depth: 1, ..Default::default() };
            if !retained_clone_grant_admits(effect.grant(), demand) || !effect.performed(1).map_err(VcsError::InverseRefused)? {
                return Ok(Default::default());
            }
            self.frame = hot.body.take();
            self.hot = None;
            return Ok(progress(demand));
        }
        if self.frame.is_some() {
            let demand = RetirementDemand { copy_bytes: size_of::<Option<CausalPayload>>() + size_of::<Option<ControlledRetirement<CausalPayload>>>(), depth: 1, ..Default::default() };
            if !retained_clone_grant_admits(effect.grant(), demand) || !effect.performed(1).map_err(VcsError::InverseRefused)? {
                return Ok(Default::default());
            }
            self.retiring_frame = Some(ControlledRetirement::new(self.frame.take().unwrap()).map_err(|(error, _)| VcsError::InverseRefused(error))?);
            return Ok(progress(demand));
        }
        if let Some(frame) = self.retiring_frame.as_mut() {
            let receipt = match RetirementPollExit::resume(frame.step(4096, effect)) {
                Ok(step) => step.progress(),
                Err(RetirementPollExit::Pending(_)) => Default::default(),
                Err(RetirementPollExit::Failed(failure)) => return Err(VcsError::InverseRefused(failure.into_value_error())),
            };
            if frame.terminal_is_empty() { self.retiring_frame = None; }
            return Ok(receipt);
        }
        self.close_receiver(effect)
    }

    pub fn original_frames_terminal(&self) -> bool { self.hot.is_none() && self.frame.is_none() && self.receiver.is_none() && self.retiring_frame.is_none() }
}

fn parse_atomic_hot(wire: &[u8]) -> Result<usize, VcsError> {
    let invalid = || VcsError::InverseRefused(invalid());
    let prefix = [1, 1, 0, 1, 0, 8];
    let tail = wire.strip_prefix(&prefix).ok_or_else(invalid)?;
    let mut extent = 0usize;
    for (index, &byte) in tail.iter().take(3).enumerate() {
        extent |= usize::from(byte & 127) << (index * 7);
        if byte & 128 != 0 { continue; }
        let body = &tail[index + 1..];
        if (index != 0 && byte == 0) || extent != body.len() { return Err(invalid()); }
        semio_os_spr::atomic_group::AtomicBackbonePart::parse(body).map_err(VcsError::InverseRefused)?;
        return Ok(prefix.len() + index + 1);
    }
    Err(invalid())
}
