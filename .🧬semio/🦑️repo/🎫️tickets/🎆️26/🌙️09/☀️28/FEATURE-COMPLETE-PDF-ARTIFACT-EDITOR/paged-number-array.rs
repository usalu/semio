//! 🔢️ Projects a borrowed JSON number-array span into a paged native sequence.

use semio_framework_pack_json::{JsonNumberCursor, Number as JsonNumber};
use semio_framework_value::{
    Number, OriginalRetirementPoll, RetainedAllocationEffect, RetainedCloneGrant,
    RetainedCloneProgress, RetainedCloneStep, RetainedWorkOutcome,
    RetirementDemand, ValueError, ValueRefusalKind,
    retained_clone::{RetainedCloneClose, retained_clone_grant_admits},
};
use semio_framework_value_native_owned::native_decoding::{
    OriginalEffectReply, OriginalEffectReplyComplete,
    owned::tree::{NativeScalarQuery, NativeSourceScalar, NativeTreeKind, NativeValueSource},
};
use semio_framework_value_persistent::list::{
    PagedList, PagedListReservationFailure, PagedListReservationOutcome,
};
use std::mem::ManuallyDrop;

#[derive(Clone, Copy)]
struct NumericNode(Number);
semio_framework_value::artifact_retire_leaf!(NumericNode);

/// 🗂️ Native number children backed by individually funded pages in wire order.
#[derive(semio_framework_value::RetireOwned)]
pub struct PagedJsonNumbers {
    children: PagedList<NumericNode, { usize::MAX }>,
}

macro_rules! scalar_queries {
    ($kind:ident, $query:ident, $answer:expr) => {
        fn as_source(&self) -> &dyn NativeValueSource { self }
        fn tree_kind(&self) -> Option<NativeTreeKind> { Some(NativeTreeKind::$kind) }
        fn query_scalar<'r>(
            &self, $query: NativeScalarQuery, effect: &mut RetainedAllocationEffect<'_>,
            reply: OriginalEffectReply<'r, NativeSourceScalar, ValueError>,
        ) -> Result<OriginalEffectReplyComplete<'r, NativeSourceScalar, ValueError>, ValueError> {
            if !effect.performed(1)? {
                return Ok(reply.complete(Err(ValueError::literal(ValueRefusalKind::WorkLimit, "JSON scalar source was not funded"))));
            }
            Ok(reply.complete($answer(self, $query)))
        }
        fn recover_scalar<'r>(
            &self, _: NativeScalarQuery, _: &mut RetainedAllocationEffect<'_>,
            reply: OriginalEffectReply<'r, NativeSourceScalar, ValueError>,
        ) -> Result<OriginalEffectReplyComplete<'r, NativeSourceScalar, ValueError>, ValueError> { Ok(reply.pending()) }
    };
}

impl NativeValueSource for NumericNode {
    scalar_queries!(Number, query, |node: &Self, query| match query {
        NativeScalarQuery::Kind => Ok(NativeSourceScalar::Kind(node.tree_kind())),
        NativeScalarQuery::Null => Ok(NativeSourceScalar::Null(false)),
        NativeScalarQuery::Number => node.number().map(NativeSourceScalar::Number),
        _ => Err(invalid()),
    });
    fn number(&self) -> Result<Number, ValueError> { Ok(self.0) }
}

impl NativeValueSource for PagedJsonNumbers {
    scalar_queries!(Array, query, |node: &Self, query| match query {
        NativeScalarQuery::Kind => Ok(NativeSourceScalar::Kind(node.tree_kind())),
        NativeScalarQuery::Null => Ok(NativeSourceScalar::Null(false)),
        NativeScalarQuery::SequenceLength => node.sequence_len().map(NativeSourceScalar::Length),
        _ => Err(invalid()),
    });
    fn sequence_len(&self) -> Result<usize, ValueError> { Ok(self.children.len()) }
    fn sequence_child(&self, index: usize) -> Result<&dyn NativeValueSource, ValueError> {
        self.children.get(index).map(NativeValueSource::as_source).ok_or_else(invalid)
    }
}

#[derive(Clone, Copy)]
enum Phase {
    Open,
    Value { empty: bool },
    Number { parser: JsonNumberCursor, start: usize },
    Deposit,
    Delimiter,
    Tail,
    Ready,
    Failed,
    Closing,
    Closed,
}

/// 🎟️ Quotes exact array extent and bounded item count before the cursor starts.
pub struct JsonNumberArrayCursor<'a> {
    source: &'a [u8],
    bytes: &'a [u8],
    position: usize,
    limit: usize,
    phase: Phase,
    pending: Option<NumericNode>,
    output: ManuallyDrop<Option<PagedJsonNumbers>>,
    retirement: RetainedCloneClose,
}

fn invalid() -> ValueError {
    ValueError::literal(ValueRefusalKind::InvalidValue, "invalid JSON number-array span")
}

fn fixed(copy_bytes: usize) -> RetirementDemand {
    RetirementDemand { copy_bytes, depth: 1, ..Default::default() }
}

fn moved(copy_bytes: usize) -> RetainedCloneProgress {
    RetainedCloneProgress { copied_items: 1, copied_bytes: copy_bytes, ..Default::default() }
}

impl<'a> JsonNumberArrayCursor<'a> {
    /// 📍️ Maintains an immutable borrow of the full source and its exact array span.
    pub fn admit(source: &'a [u8], range: std::ops::Range<usize>, maximum_bytes: usize, maximum_items: usize, grant: RetainedCloneGrant) -> Result<Option<Self>, ValueError> {
        let bytes = source.get(range).ok_or_else(invalid)?;
        if bytes.len() > maximum_bytes {
            return Err(ValueError::literal(ValueRefusalKind::WorkLimit, "JSON number-array span exceeds its byte limit"));
        }
        if !retained_clone_grant_admits(grant, Self::admission_demand()) { return Ok(None); }
        let output = ManuallyDrop::new(Some(PagedJsonNumbers { children: PagedList::new() }));
        Ok(Some(Self { source, bytes, position: 0, limit: maximum_items, phase: Phase::Open, pending: None, output, retirement: Default::default() }))
    }

    pub fn admission_demand() -> RetirementDemand { fixed(size_of::<Self>()) }
    pub fn source(&self) -> &[u8] { self.source }
    pub fn is_ready(&self) -> bool { matches!(self.phase, Phase::Ready) && self.output.is_some() }
    fn output(&self) -> &PagedJsonNumbers { self.output.as_ref().expect("number-array cursor owns its unpublished output") }

    pub fn demand(&self) -> Result<(RetirementDemand, u64), ValueError> {
        let state = size_of::<Phase>() + size_of::<usize>() + size_of::<Option<NumericNode>>();
        Ok(match self.phase {
            Phase::Ready | Phase::Failed | Phase::Closing | Phase::Closed => (Default::default(), 0),
            Phase::Deposit => {
                let children = &self.output().children;
                if children.has_reserved_slot() {
                    let mut demand = children.push_reserved_source_demand()?;
                    demand.copy_bytes += state;
                    (demand, children.push_reserved_source_work_demand()?)
                } else {
                    (RetirementDemand {
                        copy_bytes: children.next_reserve_copy_byte_demand()?,
                        capacity_bytes: children.next_allocation_bytes()?,
                        depth: children.next_reserve_depth_demand()?, ..Default::default()
                    }, children.reserve_one_work_demand()?)
                }
            }
            Phase::Number { parser, start } => {
                let end = start + parser.position();
                let token_byte = matches!(self.bytes.get(end), Some(b'0'..=b'9' | b'e' | b'E' | b'.' | b'-' | b'+'));
                let work = if !token_byte && parser.position() > 1 { 2_500 } else { 1 };
                (fixed(state), work)
            }
            _ => (fixed(state), 1),
        })
    }

    /// ⏯️ Runs exactly one admitted syntax, reservation or placement frontier.
    pub fn advance(&mut self, effect: &mut RetainedAllocationEffect<'_>, checkpoint: impl FnOnce() -> bool) -> RetainedWorkOutcome<RetainedCloneProgress> {
        let boundary = effect.performed_boundary();
        let result = self.advance_one(effect, checkpoint);
        if result.is_err() && !matches!(self.phase, Phase::Closing | Phase::Closed) {
            self.phase = Phase::Failed;
        }
        let work = effect.performed_since(boundary).expect("number-array step keeps its work receipt");
        RetainedWorkOutcome { result, work }
    }

    fn advance_one(&mut self, effect: &mut RetainedAllocationEffect<'_>, checkpoint: impl FnOnce() -> bool) -> Result<RetainedCloneProgress, ValueError> {
        let (demand, work) = self.demand()?;
        let grant = effect.grant();
        if work == 0 || !retained_clone_grant_admits(grant, demand) || effect.remaining_work_units()? < work {
            return Ok(Default::default());
        }
        if !checkpoint() { return Err(ValueError::literal(ValueRefusalKind::Canceled, "JSON number-array projection canceled")); }
        if matches!(self.phase, Phase::Deposit) {
            let children = &mut self.output.as_mut().expect("number-array owner").children;
            if !children.has_reserved_slot() {
                return match children.reserve_one_funded(effect).result {
                    Ok(PagedListReservationOutcome::Ready(receipt)) => Ok(receipt),
                    Ok(PagedListReservationOutcome::Denied) => Ok(Default::default()),
                    Err(PagedListReservationFailure::Work(error)) => Err(error),
                    Err(PagedListReservationFailure::Allocation { error, copied_bytes }) => Err(ValueError::from(error.refusal()).with_retained_progress(RetainedCloneProgress {
                        copied_items: 1, copied_bytes, retained_capacity_bytes: error.allocated_bytes, ..Default::default()
                    })),
                };
            }
            if !effect.performed(work)? { return Ok(Default::default()); }
            let turn = children.push_reserved_source_step(&mut self.pending, work, grant);
            turn.result.map_err(|error| error.with_retained_progress(moved(demand.copy_bytes)))?;
            if self.pending.is_some() { return Err(ValueError::invariant("number-array push lost its selected page")); }
            self.phase = Phase::Delimiter;
            return Ok(moved(demand.copy_bytes));
        }
        if !effect.performed(work)? { return Ok(Default::default()); }
        self.advance_grammar().map(|()| moved(demand.copy_bytes))
            .map_err(|error| error.with_retained_progress(moved(demand.copy_bytes)))
    }

    fn advance_grammar(&mut self) -> Result<(), ValueError> {
        if let Phase::Number { mut parser, start } = self.phase {
            self.phase = match parser.step(&self.bytes[start..]).map_err(|error| ValueError::literal(error.kind(), "invalid JSON numeric literal"))? {
                None => Phase::Number { parser, start },
                Some(value) => {
                    self.pending = Some(NumericNode(match value {
                        JsonNumber::UInt(value) => Number::UInt(value),
                        JsonNumber::Int(value) => Number::Int(value),
                        JsonNumber::Float(value) => Number::Float(value),
                    }));
                    self.position = start + parser.position();
                    Phase::Deposit
                }
            };
            return Ok(());
        }
        let byte = self.bytes.get(self.position).copied();
        if matches!(byte, Some(b' ' | b'\n' | b'\r' | b'\t')) {
            self.position += 1;
            return Ok(());
        }
        self.phase = match (self.phase, byte) {
            (Phase::Open, Some(b'[')) => Phase::Value { empty: true },
            (Phase::Value { empty: true } | Phase::Delimiter, Some(b']')) => Phase::Tail,
            (Phase::Value { .. }, Some(b'0'..=b'9' | b'-')) => {
                if self.output().children.len() >= self.limit {
                    return Err(ValueError::literal(ValueRefusalKind::WorkLimit, "JSON number-array item limit reached"));
                }
                self.phase = Phase::Number { parser: JsonNumberCursor::new(), start: self.position };
                return Ok(());
            }
            (Phase::Delimiter, Some(b',')) => Phase::Value { empty: false },
            (Phase::Tail, None) => { self.phase = Phase::Ready; return Ok(()); }
            _ => return Err(invalid()),
        };
        self.position += 1;
        Ok(())
    }

    pub fn take_demand() -> RetirementDemand { fixed(2 * size_of::<Option<PagedJsonNumbers>>()) }

    /// 📤️ Transfers the complete paged source without copying children or source bytes.
    pub fn take(&mut self, output: &mut Option<PagedJsonNumbers>, effect: &mut RetainedAllocationEffect<'_>) -> Result<RetainedCloneProgress, ValueError> {
        let demand = Self::take_demand();
        if !self.is_ready() || output.is_some() { return Err(invalid()); }
        if !retained_clone_grant_admits(effect.grant(), demand) || effect.remaining_work_units()? == 0 || !effect.performed(1)? {
            return Ok(Default::default());
        }
        *output = self.output.take();
        self.phase = Phase::Closed;
        Ok(moved(demand.copy_bytes))
    }

    pub fn close_demand(&self, body: usize) -> Result<(RetirementDemand, u64), ValueError> {
        if matches!(self.phase, Phase::Closed) { return Ok((Default::default(), 0)); }
        if matches!(self.phase, Phase::Closing) && (self.output.is_some() || !self.retirement.is_empty()) {
            let owned = self.output.is_some();
            return Ok((self.retirement.next_owner_demands::<PagedJsonNumbers>(owned, &None, body)?,
                self.retirement.next_owner_work::<PagedJsonNumbers>(owned, &None, body)?));
        }
        Ok((fixed(size_of::<Phase>() + size_of::<Option<NumericNode>>()), 1))
    }

    /// ♻️ Drops each unpublished numeric page under the original retirement effect.
    pub fn close(&mut self, effect: &mut RetainedAllocationEffect<'_>) -> OriginalRetirementPoll {
        let body = effect.grant().maximum_copy_bytes;
        let run = (|| {
            let (demand, work) = self.close_demand(body)?;
            if work == 0 { return Ok(RetainedCloneStep::Complete(Default::default())); }
            if !retained_clone_grant_admits(effect.grant(), demand) || effect.remaining_work_units()? < work {
                return Ok(RetainedCloneStep::Progress(Default::default()));
            }
            if matches!(self.phase, Phase::Closing) {
                if !self.retirement.is_empty() {
                    return semio_framework_value::RetirementPollExit::resume(self.retirement.step_granted(body, effect));
                }
                if self.output.is_some() {
                    let turn = self.retirement.begin_granted(&mut self.output, effect);
                    return Ok(turn.result?.unwrap_or(RetainedCloneStep::Progress(Default::default())));
                }
            }
            if !effect.performed(1)? { return Ok(RetainedCloneStep::Progress(Default::default())); }
            self.pending = None;
            if matches!(self.phase, Phase::Closing) {
                self.phase = Phase::Closed;
                Ok(RetainedCloneStep::Complete(moved(demand.copy_bytes)))
            } else {
                self.phase = Phase::Closing;
                Ok(RetainedCloneStep::Progress(moved(demand.copy_bytes)))
            }
        })();
        semio_framework_value::RetirementPollExit::settle(effect, run)
    }

    pub fn terminal_is_empty(&self) -> bool { matches!(self.phase, Phase::Closed) }
}

impl Drop for JsonNumberArrayCursor<'_> {
    fn drop(&mut self) {
        assert!(self.terminal_is_empty() || std::thread::panicking(), "JSON number-array source requires paid closure");
    }
}
