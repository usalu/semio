//#region 🫴️Original aggregate receiving
use dsl::original_receiving::{OpBinaryReceiving, OpBinaryReceivingCursor, OriginalReceivingPrimary};
use semio_framework_dsl_record::receiving::OwnedSlot;
use semio_framework_pack_json::{JsonError, JsonGrammarCursor, JsonMemberPolicy};
use semio_framework_value::{
    DslValue, FromValue, NativeDecodeControl, OriginalRetirementPoll,
    RetainedAllocationEffect, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep,
    RetainedWorkOutcome, RetainedWorkPoll, RetirementDemand, ValueError, ValueRefusalKind,
    retained_clone::{NativeAllocationIdentity, retained_clone_grant_admits},
    retirement::{RetireOwned, RetirementCursor, RetirementStep, erased_cursor_turn},
};
use std::mem::{ManuallyDrop, size_of, size_of_val};

#[derive(Clone, Copy, PartialEq, Eq)]
enum FramePhase { Version, Tag, Length, Payload }

struct FramedMutation {
    position: usize,
    end: usize,
    phase: FramePhase,
    kind: Option<&'static str>,
}

fn receiving_invalid(message: &'static str) -> ValueError {
    ValueError::literal(ValueRefusalKind::InvalidValue, message)
}

impl FramedMutation {
    fn new(range: (usize, usize)) -> Self {
        Self { position: range.0, end: range.1, phase: FramePhase::Version, kind: None }
    }

    fn advance(&mut self, source: &[u8]) -> Result<(), ValueError> {
        let byte = *source.get(self.position).filter(|_| self.position < self.end)
            .ok_or_else(|| receiving_invalid("PDF mutation frame is truncated"))?;
        match self.phase {
            FramePhase::Version => {
                if byte != store::pack_rt::OP_BINARY_FORMAT { return Err(receiving_invalid("PDF mutation frame version is invalid")); }
                self.position += 1;
                self.phase = FramePhase::Tag;
            }
            FramePhase::Tag => {
                self.kind = BINARY_TAG_REGISTRY.iter().find(|(_, _, tag)| *tag == byte).map(|(_, kind, _)| *kind);
                if self.kind.is_none() { return Err(receiving_invalid("PDF mutation frame tag is not declared")); }
                self.position += 1;
                self.phase = FramePhase::Length;
            }
            FramePhase::Length => {
                let encoded = &source[self.position..self.end];
                let mut reader = dsl::ByteReader::new(encoded);
                let length = reader.read_varint_u64()
                    .map_err(|_| receiving_invalid("PDF mutation frame length varint is invalid"))?;
                if length > MAX_PAYLOAD_BYTES as u64 { return Err(ValueError::literal(ValueRefusalKind::WorkLimit, "PDF mutation frame length exceeds its declared bound")); }
                if length != reader.remaining() as u64 { return Err(receiving_invalid("PDF mutation payload extent differs from its frame")); }
                self.position += encoded.len() - reader.remaining();
                self.phase = FramePhase::Payload;
            }
            FramePhase::Payload => return Err(ValueError::invariant("PDF mutation frame already owns a payload range")),
        }
        Ok(())
    }

    fn payload<'a>(&self, source: &'a [u8]) -> &'a [u8] { &source[self.position..self.end] }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum ReceivingPhase { Frame, StartJson, Json, Identity(usize), Bind, Ready, Closing, Closed }

/// 🧾️ Receives the decoded aggregate over its original version, tag, length and JSON bytes.
pub struct PdfMutationReceiving {
    source: (usize, usize),
    frame: FramedMutation,
    phase: ReceivingPhase,
    authority: Option<(NativeAllocationIdentity, Option<u64>)>,
    parser: OwnedSlot<JsonGrammarCursor<DslValue>>,
    json_error: OwnedSlot<JsonError>,
    value: OwnedSlot<DslValue>,
    output: OwnedSlot<PdfMutation>,
    retained_json: usize,
    failed: bool,
}

fn receiving_metadata(copy_bytes: usize) -> RetirementDemand {
    RetirementDemand { copy_bytes, depth: 1, ..Default::default() }
}

fn receiving_progress(demand: RetirementDemand) -> RetainedCloneProgress {
    RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() }
}

fn receiving_work(effect: &mut RetainedAllocationEffect<'_>) -> Result<(), ValueError> {
    if effect.performed(1)? { Ok(()) }
    else { Err(ValueError::literal(ValueRefusalKind::WorkLimit, "PDF mutation receiving lacks one funded work unit")) }
}

fn receiving_child(grant: RetainedCloneGrant, copy: usize) -> RetainedCloneGrant {
    RetainedCloneGrant {
        maximum_copy_bytes: grant.maximum_copy_bytes.saturating_sub(copy),
        maximum_depth: grant.maximum_depth.saturating_sub(1),
        ..grant
    }
}

const JSON_HANDOFF: usize = size_of::<ReceivingPhase>() + size_of::<usize>()
    + 2 * size_of::<Option<DslValue>>() + size_of::<Option<JsonError>>();
const OUTPUT_HANDOFF: usize = 2 * size_of::<Option<PdfMutation>>();

impl OpBinaryReceiving for PdfMutation {
    type DomainFailure = ValueError;
    type Receiving = PdfMutationReceiving;

    fn admit_receiving(source: &[u8], range: (usize, usize), grant: RetainedCloneGrant)
        -> RetainedWorkOutcome<Option<(Self::Receiving, RetainedCloneProgress)>> {
        if range.0 > range.1 || range.1 > source.len() {
            return RetainedWorkOutcome { result: Err(receiving_invalid("PDF mutation escaped its original source")), work: 0 };
        }
        let demand = receiving_metadata(size_of::<PdfMutationReceiving>());
        if !retained_clone_grant_admits(grant, demand) {
            return RetainedWorkOutcome { result: Ok(None), work: 0 };
        }
        let receiver = PdfMutationReceiving {
            source: (source.as_ptr() as usize, source.len()), frame: FramedMutation::new(range),
            phase: ReceivingPhase::Frame, authority: None, parser: Default::default(), json_error: Default::default(),
            value: Default::default(), output: Default::default(), retained_json: 0, failed: false,
        };
        RetainedWorkOutcome { result: Ok(Some((receiver, receiving_progress(demand)))), work: 1 }
    }
}

impl PdfMutationReceiving {
    fn validate_source(&self, source: &[u8]) -> Result<(), ValueError> {
        if self.source == (source.as_ptr() as usize, source.len()) { Ok(()) }
        else { Err(receiving_invalid("PDF mutation original source identity changed")) }
    }

    fn advance_json(&mut self, source: &[u8], native: &mut NativeDecodeControl<'_>, effect: &mut RetainedAllocationEffect<'_>)
        -> Result<RetainedCloneProgress, OriginalReceivingPrimary> {
        let parser = self.parser.value.as_mut().expect("admitted PDF mutation JSON parser");
        let child = receiving_child(effect.grant(), JSON_HANDOFF);
        let step = parser.step_source(self.frame.payload(source), 1, native, &mut effect.nested(child), false);
        let mut progress = parser.normal_step_progress();
        if effect.performed_work() != 0 {
            progress.copied_bytes += JSON_HANDOFF;
            self.retained_json = self.retained_json.checked_add(progress.retained_capacity_bytes)
                .and_then(|bytes| bytes.checked_sub(progress.released_bytes))
                .ok_or_else(|| ValueError::invariant("PDF mutation JSON backing receipt has no original owner").with_retained_progress(progress))?;
        }
        match step {
            Ok(Some(value)) => {
                *self.value.value = Some(value);
                self.phase = ReceivingPhase::Identity(0);
            }
            Ok(None) => {}
            Err(JsonError::Native(error)) => return Err(error.with_retained_progress(progress).into()),
            Err(error) => {
                let kind = error.kind();
                *self.json_error.value = Some(error);
                return Err(ValueError::literal(kind, "PDF mutation JSON payload was refused").with_retained_progress(progress).into());
            }
        }
        Ok(progress)
    }

    fn check_identity(&mut self, index: usize) -> Result<(), ValueError> {
        let Some(DslValue::Object(entries)) = self.value.value.as_ref() else {
            return Err(receiving_invalid("PDF mutation payload is not an object"));
        };
        let (key, value) = entries.get(index).ok_or_else(|| receiving_invalid("PDF mutation payload has no mutation identity"))?;
        if key == "mutation" {
            if value.as_str() != self.frame.kind { return Err(receiving_invalid("PDF mutation tag differs from its JSON identity")); }
            self.phase = ReceivingPhase::Bind;
        } else {
            self.phase = ReceivingPhase::Identity(index + 1);
        }
        Ok(())
    }

    fn turn(&mut self, source: &[u8], native: &mut NativeDecodeControl<'_>, body: usize, effect: &mut RetainedAllocationEffect<'_>)
        -> Result<RetainedCloneProgress, OriginalReceivingPrimary> {
        if self.failed || matches!(self.phase, ReceivingPhase::Ready | ReceivingPhase::Closing | ReceivingPhase::Closed) { return Ok(Default::default()); }
        let demand = self.demands(source, body)?;
        if demand.copy_bytes > body || !retained_clone_grant_admits(effect.grant(), demand) { return Ok(Default::default()); }
        native.checkpoint()?;
        let authority = (native.ledger_identity(), native.retirement_recipient_identity());
        if self.authority.is_some_and(|bound| bound != authority) {
            return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit, "PDF mutation original receiving authority changed").into());
        }
        if self.phase == ReceivingPhase::Json { return self.advance_json(source, native, effect); }
        receiving_work(effect)?;
        if self.authority.is_none() { self.authority = Some(authority); }
        let progress = receiving_progress(demand);
        let result = match self.phase {
            ReceivingPhase::Frame => self.frame.advance(source).map(|()| {
                if self.frame.phase == FramePhase::Payload { self.phase = ReceivingPhase::StartJson; }
            }),
            ReceivingPhase::StartJson => {
                *self.parser.value = Some(JsonGrammarCursor::new(JsonMemberPolicy::Reject));
                self.phase = ReceivingPhase::Json;
                Ok(())
            }
            ReceivingPhase::Identity(index) => self.check_identity(index),
            ReceivingPhase::Bind => PdfMutation::from_value(self.value.value.take().expect("admitted PDF mutation JSON value")).map(|mutation| {
                *self.output.value = Some(mutation);
                self.phase = ReceivingPhase::Ready;
            }),
            _ => unreachable!(),
        };
        result.map(|()| progress).map_err(|error| error.with_retained_progress(progress).into())
    }

    fn close_demand(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        if self.phase == ReceivingPhase::Closed { return Ok(Default::default()); }
        if self.phase != ReceivingPhase::Closing { return Ok(receiving_metadata(size_of::<ReceivingPhase>())); }
        let demand = if !self.output.terminal_is_empty() { self.output.close_demands(body) }
        else if !self.value.terminal_is_empty() { self.value.close_demands(body) }
        else if !self.parser.terminal_is_empty() { self.parser.close_demands(body) }
        else if !self.json_error.terminal_is_empty() { self.json_error.close_demands(body) }
        else { return Ok(receiving_metadata(size_of::<ReceivingPhase>())); }?;
        Ok(RetirementDemand { depth: demand.depth + 1, ..demand })
    }

    fn close_work(&self, body: usize) -> Result<u64, ValueError> {
        if self.phase != ReceivingPhase::Closing { return Ok(1); }
        if !self.output.terminal_is_empty() { return self.output.close_work_demand(body); }
        if !self.value.terminal_is_empty() { return self.value.close_work_demand(body); }
        if !self.parser.terminal_is_empty() { return self.parser.close_work_demand(body); }
        if !self.json_error.terminal_is_empty() { return self.json_error.close_work_demand(body); }
        Ok(1)
    }

    fn close(&mut self, effect: &mut RetainedAllocationEffect<'_>) -> OriginalRetirementPoll {
        let grant = effect.grant();
        let done = |result, effect: &RetainedAllocationEffect<'_>| RetainedWorkPoll::Complete(RetainedWorkOutcome { result, work: effect.performed_work() });
        if self.phase == ReceivingPhase::Closed { return done(Ok(RetainedCloneStep::Complete(Default::default())), effect); }
        let demand = match self.close_demand(grant.maximum_copy_bytes) {
            Ok(demand) => demand,
            Err(error) => return done(Err(error.into()), effect),
        };
        if !retained_clone_grant_admits(grant, demand) {
            return done(Ok(RetainedCloneStep::Progress(Default::default())), effect);
        }
        if self.phase == ReceivingPhase::Closing {
            let effect = &mut effect.nested(receiving_child(grant, 0));
            let child = if !self.output.terminal_is_empty() { Some(self.output.close_step(effect)) }
            else if !self.value.terminal_is_empty() { Some(self.value.close_step(effect)) }
            else if !self.parser.terminal_is_empty() { Some(self.parser.close_step(effect)) }
            else if !self.json_error.terminal_is_empty() { Some(self.json_error.close_step(effect)) }
            else { None };
            if let Some(child) = child {
                return match child {
                    RetainedWorkPoll::Pending { work } => RetainedWorkPoll::Pending { work },
                    RetainedWorkPoll::Complete(step) => RetainedWorkPoll::Complete(RetainedWorkOutcome {
                        result: step.result.map(RetainedCloneStep::Progress), work: step.work,
                    }),
                };
            }
        }
        if let Err(error) = receiving_work(effect) { return done(Err(error.into()), effect); }
        let progress = receiving_progress(demand);
        if self.phase == ReceivingPhase::Closing {
            self.phase = ReceivingPhase::Closed;
            done(Ok(RetainedCloneStep::Complete(progress)), effect)
        } else {
            self.phase = ReceivingPhase::Closing;
            done(Ok(RetainedCloneStep::Progress(progress)), effect)
        }
    }
}

impl OpBinaryReceivingCursor<PdfMutation> for PdfMutationReceiving {
    fn demands(&self, source: &[u8], body: usize) -> Result<RetirementDemand, ValueError> {
        self.validate_source(source)?;
        Ok(match self.phase {
            ReceivingPhase::Frame => receiving_metadata(size_of::<FramedMutation>() + size_of::<ReceivingPhase>() + size_of_val(&self.authority)),
            ReceivingPhase::StartJson => receiving_metadata(2 * size_of::<Option<JsonGrammarCursor<DslValue>>>()),
            ReceivingPhase::Json => {
                let parser = self.parser.value.as_ref().expect("admitted PDF mutation JSON parser");
                let demand = parser.normal_step_demands(self.frame.payload(source))
                    .map_err(|error| ValueError::literal(error.kind(), "PDF mutation JSON demand was refused"))?;
                RetirementDemand { copy_bytes: demand.copy_bytes.saturating_add(JSON_HANDOFF), depth: demand.depth + 1, ..demand }
            }
            ReceivingPhase::Identity(_) => receiving_metadata(size_of::<ReceivingPhase>()),
            ReceivingPhase::Bind => receiving_metadata(self.retained_json.saturating_add(OUTPUT_HANDOFF + size_of::<DslValue>())),
            ReceivingPhase::Ready => receiving_metadata(OUTPUT_HANDOFF),
            ReceivingPhase::Closing | ReceivingPhase::Closed => return self.close_demand(body),
        })
    }

    fn step_in(&mut self, source: &[u8], native: &mut NativeDecodeControl<'_>, body: usize, effect: &mut RetainedAllocationEffect<'_>)
        -> RetainedWorkOutcome<RetainedCloneProgress, OriginalReceivingPrimary> {
        let result = self.turn(source, native, body, effect);
        if result.is_err() { self.failed = true; }
        RetainedWorkOutcome { result, work: effect.performed_work() }
    }

    fn domain_failure(&self) -> Option<&ValueError> { None }
    fn domain_failure_demand(&self) -> Result<RetirementDemand, ValueError> { Ok(Default::default()) }
    fn domain_failure_work_demand(&self) -> Result<u64, ValueError> { Ok(0) }
    fn take_domain_failure_into(&mut self, recipient: &mut Option<ValueError>, _: RetainedCloneGrant) -> RetainedWorkOutcome<RetainedCloneProgress> {
        let result = if recipient.is_some() { Err(receiving_invalid("PDF mutation refusal recipient is occupied")) } else { Ok(Default::default()) };
        RetainedWorkOutcome { result, work: 0 }
    }

    fn is_ready(&self) -> bool { !self.failed && self.phase == ReceivingPhase::Ready && self.output.value.is_some() }
    fn take_into(&mut self, recipient: &mut Option<PdfMutation>, grant: RetainedCloneGrant) -> RetainedWorkOutcome<RetainedCloneProgress> {
        let demand = receiving_metadata(OUTPUT_HANDOFF);
        if !self.is_ready() || recipient.is_some() || !retained_clone_grant_admits(grant, demand) {
            return RetainedWorkOutcome { result: Ok(Default::default()), work: 0 };
        }
        *recipient = self.output.value.take();
        RetainedWorkOutcome { result: Ok(receiving_progress(demand)), work: 1 }
    }

    fn close_demands(&self, body: usize) -> Result<RetirementDemand, ValueError> { self.close_demand(body) }
    fn close_step(&mut self, effect: &mut RetainedAllocationEffect<'_>) -> OriginalRetirementPoll { self.close(effect) }
    fn terminal_is_empty(&self) -> bool { self.phase == ReceivingPhase::Closed }
}

/// ♻️ Retires every unpublished JSON and mutation owner through the original allocation effect.
pub struct PdfMutationReceivingRetirement(ManuallyDrop<PdfMutationReceiving>);
impl RetireOwned for PdfMutationReceiving {
    type RetirementCursor = PdfMutationReceivingRetirement;
    fn retirement_seed(self) -> Self::RetirementCursor { PdfMutationReceivingRetirement(ManuallyDrop::new(self)) }
    fn controlled_retirement_supported() -> bool { true }
}
impl PdfMutationReceivingRetirement {
    fn close_step(&mut self, _: usize, effect: &mut RetainedAllocationEffect<'_>) -> RetirementStep { erased_cursor_turn(self.0.close(effect)) }
}
impl RetirementCursor for PdfMutationReceivingRetirement {
    semio_framework_value::native_retirement_reply!();
    fn terminal_is_empty(&self) -> bool { self.0.terminal_is_empty() }
    fn next_computational_work(&self, body: usize) -> Result<u64, ValueError> { self.0.close_work(body) }
    fn next_depth_demand(&self) -> Result<usize, ValueError> { Ok(self.0.close_demand(usize::MAX)?.depth) }
    fn next_selected_work_byte_demand(&self, body: usize) -> Result<usize, ValueError> { Ok(self.0.close_demand(body)?.copy_bytes) }
    fn next_close_byte_demand(&self) -> Option<usize> { self.0.close_demand(usize::MAX).ok().map(|demand| demand.release_bytes) }
    fn next_birth_bytes(&self, body: usize) -> Option<usize> { self.0.close_demand(body).ok().map(|demand| demand.capacity_bytes) }
}
impl Drop for PdfMutationReceivingRetirement {
    fn drop(&mut self) {
        assert!(self.0.terminal_is_empty() || std::thread::panicking(), "PDF original mutation receiving requires paid closure");
        if self.0.terminal_is_empty() { unsafe { ManuallyDrop::drop(&mut self.0) } }
    }
}
//#endregion 🫴️Original aggregate receiving
