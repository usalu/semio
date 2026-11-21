//! 🔐 Reads original JSON samples directly into their exact typed vector.

use crate::{JsonNumberArraySpan, JsonNumberCursor, Number as JsonNumber};
use semio_framework_value::{
    Number, OriginalRetirementPoll, RetainedAllocationEffect, RetainedCloneGrant, RetainedCloneProgress,
    RetainedCloneStep, RetainedWorkOutcome, RetirementDemand, RetirementPollExit, ValueError, ValueRefusalKind,
    retained_clone::retained_clone_grant_admits,
};
use std::mem::ManuallyDrop;

#[derive(Clone, Copy)]
enum Phase {
    Open, Value { empty: bool }, Number { parser: JsonNumberCursor, start: usize },
    Delimiter, Reserve, Ready, Failed, Closed,
}

/// 📍 Borrows the original payload while counting and filling one selected JSON array.
pub struct JsonU32ArrayCursor<'a> {
    source: &'a [u8],
    start: usize,
    position: usize,
    extent: Option<usize>,
    count: usize,
    maximum_count: usize,
    values: ManuallyDrop<Vec<u32>>,
    phase: Phase,
}

fn invalid() -> ValueError {
    ValueError::literal(ValueRefusalKind::InvalidValue, "expected a JSON array of exact u32 samples")
}

fn fixed(copy_bytes: usize) -> RetirementDemand {
    RetirementDemand { copy_bytes, depth: 1, ..Default::default() }
}

fn progress(demand: RetirementDemand) -> RetainedCloneProgress {
    RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes,
        retained_capacity_bytes: demand.capacity_bytes, released_bytes: demand.release_bytes, ..Default::default() }
}

impl<'a> JsonU32ArrayCursor<'a> {
    /// 🎟️ Caps final backing to the admitted allocation and cancellation release policies.
    pub fn admit(source: &'a [u8], start: usize, capacity: usize, cancellation_release: usize, grant: RetainedCloneGrant) -> Result<Option<Self>, ValueError> {
        if source.get(start) != Some(&b'[') { return Err(invalid()); }
        if !retained_clone_grant_admits(grant, Self::admission_demand()) { return Ok(None); }
        Ok(Some(Self {
            source, start, position: start, extent: None, count: 0,
            maximum_count: capacity.min(cancellation_release) / size_of::<u32>(),
            values: ManuallyDrop::new(Vec::new()), phase: Phase::Open,
        }))
    }

    pub fn admission_demand() -> RetirementDemand { fixed(size_of::<Self>()) }
    pub fn is_ready(&self) -> bool { matches!(self.phase, Phase::Ready) }
    pub fn terminal_is_empty(&self) -> bool { matches!(self.phase, Phase::Closed) }

    pub fn demand(&self) -> Result<(RetirementDemand, u64), ValueError> {
        let state = size_of::<Phase>() + size_of::<usize>() * 2 + size_of::<Option<usize>>();
        Ok(match self.phase {
            Phase::Ready | Phase::Failed | Phase::Closed => (Default::default(), 0),
            Phase::Reserve => (RetirementDemand {
                capacity_bytes: self.count * size_of::<u32>(), ..fixed(state + size_of::<Vec<u32>>())
            }, 1),
            Phase::Number { parser, start } => {
                let byte = self.source.get(start + parser.position());
                let continued = matches!(byte, Some(b'0'..=b'9')) || (parser.position() == 0 && byte == Some(&b'-'));
                (fixed(state + if self.extent.is_some() && !continued { size_of::<Vec<u32>>() + size_of::<u32>() } else { 0 }),
                    if continued { 1 } else { 4 })
            }
            _ => (fixed(state), 1),
        })
    }

    /// ⏯️ Advances one original byte, numeric conversion, allocation, or typed sample placement.
    pub fn advance(&mut self, effect: &mut RetainedAllocationEffect<'_>, checkpoint: impl FnOnce() -> bool) -> RetainedWorkOutcome<RetainedCloneProgress> {
        let before = effect.performed_boundary();
        let result = self.advance_one(effect, checkpoint);
        let work = effect.performed_since(before).expect("typed sample step keeps its work receipt");
        RetainedWorkOutcome { result, work }
    }

    fn advance_one(&mut self, effect: &mut RetainedAllocationEffect<'_>, checkpoint: impl FnOnce() -> bool) -> Result<RetainedCloneProgress, ValueError> {
        let (demand, work) = self.demand()?;
        if work == 0 || !retained_clone_grant_admits(effect.grant(), demand) || effect.remaining_work_units()? < work {
            return Ok(Default::default());
        }
        if !checkpoint() {
            return Err(ValueError::literal(ValueRefusalKind::Canceled, "typed sample receiving canceled"));
        }
        if !effect.performed(work)? { return Ok(Default::default()); }
        let action = if matches!(self.phase, Phase::Reserve) {
            self.reserve(demand, effect)
        } else {
            self.advance_grammar()
        };
        match action {
            Ok(()) => Ok(progress(demand)),
            Err(error) => {
                self.phase = Phase::Failed;
                let mut receipt = progress(demand);
                receipt.retained_capacity_bytes = effect.actual().unwrap_or(0);
                Err(error.with_retained_progress(receipt))
            }
        }
    }

    fn reserve(&mut self, demand: RetirementDemand, effect: &mut RetainedAllocationEffect<'_>) -> Result<(), ValueError> {
        if demand.capacity_bytes != 0 {
            if effect.actual().is_some() { return Err(ValueError::invariant("typed sample allocation marker occupied")); }
            *effect.actual() = Some(0);
            self.values.try_reserve_exact(self.count).map_err(|_| ValueError::literal(ValueRefusalKind::AllocationFailed, "sample vector reserve failed"))?;
            let bytes = self.values.capacity() * size_of::<u32>();
            *effect.actual() = Some(bytes);
            if bytes != demand.capacity_bytes { return Err(ValueError::invariant("typed sample allocation disagrees with its quote")); }
        }
        self.position = self.start;
        self.phase = Phase::Open;
        Ok(())
    }

    fn advance_grammar(&mut self) -> Result<(), ValueError> {
        if let Phase::Number { mut parser, start } = self.phase {
            let bytes = &self.source[start..];
            let next = bytes.get(parser.position());
            let sign = usize::from(bytes.first() == Some(&b'-'));
            let digits = parser.position().saturating_sub(sign);
            if matches!(next, Some(b'.' | b'e' | b'E'))
                || (matches!(next, Some(b'0'..=b'9'))
                    && (digits >= 10 || (digits == 1 && bytes.get(sign) == Some(&b'0'))))
            {
                return Err(invalid());
            }
            self.phase = match parser.step(bytes).map_err(|e| ValueError::literal(e.kind(), "invalid JSON sample number"))? {
                None => Phase::Number { parser, start },
                Some(number) => {
                    let number = match number {
                        JsonNumber::UInt(value) => Number::UInt(value),
                        JsonNumber::Int(value) => Number::Int(value),
                        JsonNumber::Float(value) => Number::Float(value),
                    };
                    let value = number.as_u64().and_then(|n| u32::try_from(n).ok()).ok_or_else(invalid)?;
                    if self.extent.is_some() {
                        if self.values.len() >= self.count { return Err(ValueError::invariant("typed sample recount exceeded its exact reservation")); }
                        self.values.push(value);
                    } else {
                        self.count = self.count.checked_add(1).ok_or_else(invalid)?;
                    }
                    self.position = start + parser.position();
                    Phase::Delimiter
                }
            };
            return Ok(());
        }
        let byte = self.source.get(self.position).copied();
        if matches!(byte, Some(b' ' | b'\n' | b'\r' | b'\t')) {
            self.position += 1;
            return Ok(());
        }
        self.phase = match (self.phase, byte) {
            (Phase::Open, Some(b'[')) => Phase::Value { empty: true },
            (Phase::Value { empty: true } | Phase::Delimiter, Some(b']')) => {
                let end = self.position + 1;
                if let Some(extent) = self.extent {
                    if extent != end || self.values.len() != self.count {
                        return Err(ValueError::invariant("typed sample recount differs from its original span"));
                    }
                    Phase::Ready
                } else {
                    if self.count > self.maximum_count {
                        return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit, "sample vector exceeds its cancelable backing policy"));
                    }
                    self.extent = Some(end);
                    Phase::Reserve
                }
            }
            (Phase::Value { .. }, Some(b'0'..=b'9' | b'-')) => {
                self.phase = Phase::Number { parser: JsonNumberCursor::new(), start: self.position };
                return Ok(());
            }
            (Phase::Delimiter, Some(b',')) => Phase::Value { empty: false },
            _ => return Err(invalid()),
        };
        self.position += 1;
        Ok(())
    }

    pub fn finished_span(&self) -> Option<JsonNumberArraySpan<'a>> {
        self.is_ready().then(|| JsonNumberArraySpan::finished(self.source, self.start..self.position))
    }

    pub fn take_demand() -> RetirementDemand { fixed(size_of::<Vec<u32>>() + size_of::<Option<Vec<u32>>>() + size_of::<Phase>()) }

    /// 📥 Transfers the exact typed vector without copying sample backing.
    pub fn take(&mut self, output: &mut Option<Vec<u32>>, effect: &mut RetainedAllocationEffect<'_>) -> Result<RetainedCloneProgress, ValueError> {
        let demand = Self::take_demand();
        if !self.is_ready() || output.is_some() { return Err(ValueError::invariant("typed sample destination changed")); }
        if !retained_clone_grant_admits(effect.grant(), demand) || effect.remaining_work_units()? == 0 || !effect.performed(1)? {
            return Ok(Default::default());
        }
        *output = Some(std::mem::take(&mut *self.values));
        self.phase = Phase::Closed;
        Ok(progress(demand))
    }

    pub fn close_demand(&self) -> RetirementDemand {
        if self.terminal_is_empty() { return Default::default(); }
        RetirementDemand { release_bytes: self.values.capacity() * size_of::<u32>(), ..Self::take_demand() }
    }

    /// ♻️ Releases the uncopied contiguous u32 backing under the selected release loan.
    pub fn close(&mut self, effect: &mut RetainedAllocationEffect<'_>) -> OriginalRetirementPoll {
        let result = if self.terminal_is_empty() {
            Ok(RetainedCloneStep::Complete(Default::default()))
        } else {
            let demand = self.close_demand();
            (|| {
                if !retained_clone_grant_admits(effect.grant(), demand) || effect.remaining_work_units()? == 0 || !effect.performed(1)? {
                    return Ok(RetainedCloneStep::Progress(Default::default()));
                }
                drop(std::mem::take(&mut *self.values));
                self.phase = Phase::Closed;
                Ok(RetainedCloneStep::Complete(progress(demand)))
            })()
        };
        RetirementPollExit::settle(effect, result)
    }
}

impl Drop for JsonU32ArrayCursor<'_> {
    fn drop(&mut self) {
        assert!(self.terminal_is_empty() || std::thread::panicking(), "typed sample cursor requires paid closure");
    }
}
