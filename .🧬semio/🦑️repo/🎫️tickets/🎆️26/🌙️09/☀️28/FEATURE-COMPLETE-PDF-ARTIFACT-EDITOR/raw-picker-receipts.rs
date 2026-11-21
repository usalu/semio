//! 🧭 Supervises raw fixed-page receipts on the same Original caller Source as the picked image.

use crate::{
    bridge::OriginalMediaCommandBinding,
    guest::{OriginalGuestMediaInput, OriginalMediaGuest, OriginalMediaPickerIntent},
    ingress::{OriginalMediaLoan, ORIGINAL_MEDIA_PAGE_BYTES, media_type_valid},
};
use semio_framework_job::{InteractiveJobCloseStep, OriginalSourceBinding, OriginalTurnFailure, StepContext};
use semio_framework_replication_command_ingress::{COMMAND_MAXIMUM_BYTES, CommandPageCursor, FixedCommandPage};
use semio_framework_value::{RetainedCloneProgress, RetirementDemand, ValueError, ValueRefusalKind};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Transport {
    pub owner: u64,
    pub seq: u64,
    pub kind: u8,
    pub extent: u32,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase { Begin, Page(u32), Complete, Cancel }

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ResultCode { Retry, Pending, Accepted, Stale, Refused }

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FormatResult { Published, Refused }

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Ack<'a> {
    pub loan: OriginalMediaLoan<'a>,
    pub transport: Transport,
    pub phase: Phase,
    pub result: ResultCode,
}

#[derive(Clone, Copy)]
pub enum Control<'a> {
    Begin { name: &'a str, media_type: &'a str },
    Complete,
    Cancel,
}

impl Control<'_> {
    fn phase(self) -> Phase {
        match self { Self::Begin { .. } => Phase::Begin, Self::Complete => Phase::Complete, Self::Cancel => Phase::Cancel }
    }
}

#[derive(Clone, Copy)]
struct Route {
    transport: Transport,
    phase: Phase,
    result: ResultCode,
}

/// 🧾 Pays the terminal route receipt before the document publication can advance its revision.
#[must_use]
pub struct FormatReceipt {
    source: OriginalSourceBinding,
    operation: u64,
    generation: u64,
    instance: u32,
    transport: Transport,
}

pub struct RawPicker<T, const N: usize> {
    intent: Option<OriginalMediaPickerIntent<T>>,
    source: Option<OriginalSourceBinding>,
    ingress: Option<OriginalMediaGuest<T, N>>,
    output: Option<OriginalGuestMediaInput<T, N>>,
    active: Option<Transport>,
    waiting: Option<Phase>,
    last: Option<Route>,
    next_page: u32,
    kind: u8,
    maximum: usize,
    invalidated: bool,
    superseding: bool,
    format_result: Option<FormatResult>,
    format_pending: bool,
}

impl<T: Copy, const N: usize> RawPicker<T, N> {
    pub fn empty() -> Self {
        Self { intent: None, source: None, ingress: None, output: None, active: None,
            waiting: None, last: None, next_page: 0, kind: 0, maximum: 0, invalidated: false, superseding: false, format_result: None, format_pending: false }
    }

    pub fn issue(
        &mut self, loan: OriginalMediaLoan<'_>, target: T, kind: u8, maximum: usize, cx: &mut StepContext<'_>,
    ) -> Result<bool, OriginalTurnFailure> {
        if self.intent.is_some() || !Self::pay_route(cx, false)? { return Ok(false); }
        let maximum = maximum.min(N).min(COMMAND_MAXIMUM_BYTES);
        if !OriginalMediaPickerIntent::issue(loan, target, kind, maximum, &mut self.intent, cx)? {
            return Ok(false);
        }
        self.source = Some(OriginalSourceBinding::borrow(cx));
        self.kind = kind;
        self.maximum = maximum;
        Ok(true)
    }

    /// 🔗 Adopts a paid typed picker after the retained command's own input Source may return.
    pub fn admit_issued(
        &mut self, issued: &mut Option<OriginalMediaPickerIntent<T>>, current: OriginalMediaLoan<'_>,
        cx: &mut StepContext<'_>,
    ) -> Result<bool, OriginalTurnFailure> {
        cx.original_source_parent()?.validate_borrowed_context(cx)?;
        let Some(intent) = issued.as_ref() else { return Ok(false); };
        if intent.loan() != current {
            return Err(ValueError::literal(ValueRefusalKind::Canceled, "Raw picker changed before publication").into());
        }
        if self.intent.is_some() { return Ok(false); }
        let progress = RetainedCloneProgress {
            copied_items: 1,
            copied_bytes: size_of::<Option<OriginalMediaPickerIntent<T>>>(),
            ..Default::default()
        };
        if !progress.fits(cx.retained_grant()) || !Self::pay_route(cx, false)? ||
            !progress.fits(cx.retained_grant()) || cx.fuel_remaining() == 0
        {
            return Ok(false);
        }
        cx.consume_retained(progress)?;
        cx.consume_fuel(1);
        self.kind = intent.kind();
        self.maximum = intent.maximum().min(N).min(COMMAND_MAXIMUM_BYTES);
        self.source = Some(OriginalSourceBinding::borrow(cx));
        self.intent = issued.take();
        Ok(true)
    }

    pub fn issued_loan(&self) -> Option<OriginalMediaLoan<'_>> {
        self.intent.as_ref().map(OriginalMediaPickerIntent::loan)
    }

    pub fn issued(&self) -> Option<(OriginalMediaLoan<'_>, u8, u32)> {
        Some((self.issued_loan()?, self.kind, self.maximum.try_into().ok()?))
    }

    fn validate_source(&self, cx: &StepContext<'_>) -> Result<(), OriginalTurnFailure> {
        cx.original_source_parent()?.validate_borrowed_context(cx)?;
        if !self.source.is_some_and(|source| source.matches(cx)) {
            return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit, "Raw picker changed its caller Source").into());
        }
        Ok(())
    }

    fn pay_route(cx: &mut StepContext<'_>, retirement: bool) -> Result<bool, OriginalTurnFailure> {
        let progress = RetainedCloneProgress {
            copied_items: 1,
            copied_bytes: size_of::<Option<Route>>() + size_of::<Option<Phase>>() + size_of::<Option<Transport>>()
                + size_of::<Option<OriginalSourceBinding>>() + size_of::<usize>() + size_of::<u32>() + size_of::<u8>()
                + 3 * size_of::<bool>() + size_of::<Option<FormatResult>>(),
            ..Default::default()
        };
        let ready = if retirement { !cx.retirement_should_yield() } else { !cx.should_yield() && !cx.is_cancelled() };
        let grant = cx.retained_grant();
        if !ready || grant.maximum_depth == 0 || cx.fuel_remaining() == 0 || !progress.fits(grant) { return Ok(false); }
        cx.consume_retained(progress)?;
        cx.consume_fuel(1);
        Ok(true)
    }

    fn observe(&self, transport: Transport, phase: Phase, result: ResultCode) -> Ack<'_> {
        Ack { loan: self.issued_loan().expect("receipt has a retained picker"), transport, phase, result }
    }

    fn record(&mut self, transport: Transport, phase: Phase, result: ResultCode) -> Ack<'_> {
        self.last = Some(Route { transport, phase, result });
        self.observe(transport, phase, result)
    }

    fn replay(&self, transport: Transport, phase: Phase) -> Option<ResultCode> {
        self.last.filter(|last| last.transport == transport && last.phase == phase).map(|last| last.result)
    }

    /// 🎮 Handles controls only after the reactor authenticates the exact live instance and caller.
    pub fn control<'a>(
        &'a mut self, offered: OriginalMediaLoan<'a>, current: OriginalMediaLoan<'_>, transport: Transport,
        control: Control<'_>, cx: &mut StepContext<'_>,
    ) -> Result<Ack<'a>, OriginalTurnFailure> {
        let phase = control.phase();
        let reject = |result| Ack { loan: offered, transport, phase, result };
        if self.issued_loan() != Some(offered) { return Ok(reject(ResultCode::Stale)); }
        self.validate_source(cx)?;
        if transport.owner != u64::from(offered.instance_id) || transport.seq == 0 || transport.kind != self.kind ||
            transport.extent as usize > self.maximum || self.active.is_some_and(|active| active != transport)
        {
            return Ok(reject(ResultCode::Refused));
        }
        let replay = self.replay(transport, phase);
        if phase == Phase::Cancel && self.format_result == Some(FormatResult::Published) {
            return Ok(reject(ResultCode::Stale));
        }
        if let Some(result) = replay.filter(|result| phase == Phase::Complete &&
            (self.format_pending || self.format_result.is_some() || matches!(result, ResultCode::Accepted | ResultCode::Refused)))
        {
            return Ok(self.observe(transport, phase, result));
        }
        if phase != Phase::Cancel && self.invalidated { return Ok(reject(ResultCode::Stale)); }
        if phase != Phase::Cancel && self.issued_loan() != Some(current) {
            if !Self::pay_route(cx, false)? { return Ok(reject(ResultCode::Retry)); }
            self.invalidated = true;
            return Ok(reject(ResultCode::Stale));
        }
        if let Some(result) = replay {
            return Ok(self.observe(transport, phase, result));
        }
        if self.last.is_some_and(|last| last.phase == Phase::Cancel || last.phase == Phase::Complete &&
            matches!(last.result, ResultCode::Accepted | ResultCode::Refused))
        {
            return Ok(reject(ResultCode::Stale));
        }
        if !Self::pay_route(cx, phase == Phase::Cancel)? { return Ok(reject(ResultCode::Retry)); }
        match control {
            Control::Begin { name, media_type } => {
                if transport.extent == 0 || self.active.is_some() || name.is_empty()
                    || name.chars().take(256).count() > 255 || !media_type_valid(media_type)
                {
                    return Ok(reject(ResultCode::Refused));
                }
                let binding = command_binding(offered, transport);
                let intent = self.intent.as_mut().expect("matched picker loan");
                if !OriginalMediaGuest::begin(intent, &binding, current, &mut self.ingress, cx)? {
                    return Ok(reject(ResultCode::Retry));
                }
                self.active = Some(transport);
                Ok(self.record(transport, phase, ResultCode::Accepted))
            }
            Control::Complete => {
                let Some(ingress) = self.ingress.as_mut() else { return Ok(reject(ResultCode::Refused)); };
                if self.waiting.is_some() || self.next_page != transport.extent.div_ceil(ORIGINAL_MEDIA_PAGE_BYTES as u32) {
                    return Ok(reject(ResultCode::Refused));
                }
                if !ingress.complete(current, &mut self.output, cx)? { return Ok(reject(ResultCode::Retry)); }
                self.waiting = Some(phase);
                Ok(self.record(transport, phase, ResultCode::Pending))
            }
            Control::Cancel => {
                self.active = Some(transport);
                self.waiting = Some(Phase::Cancel);
                if let Some(ingress) = self.ingress.as_mut() { ingress.cancel(offered, cx)?; }
                Ok(self.record(transport, phase, ResultCode::Pending))
            }
        }
    }

    /// 📥 Accepts a raw page before the generic byte-zero and zero-metadata command check.
    pub fn page(
        &mut self, current: OriginalMediaLoan<'_>, incoming: &mut Option<(CommandPageCursor, FixedCommandPage)>,
        cx: &mut StepContext<'_>,
    ) -> Result<Option<Ack<'_>>, OriginalTurnFailure> {
        let Some((cursor, page)) = incoming.as_ref() else { return Ok(None); };
        let Some(issued) = self.issued_loan() else { return Ok(None); };
        self.validate_source(cx)?;
        let transport = Transport { owner: cursor.owner, seq: cursor.seq, kind: cursor.kind, extent: cursor.metadata };
        let phase = Phase::Page(cursor.page_index);
        let result = if self.active != Some(transport) || command_binding(issued, transport).body(cursor, page).is_err() {
            Some(ResultCode::Refused)
        } else if self.invalidated || matches!(self.waiting, Some(Phase::Cancel)) ||
            self.last.is_some_and(|last| matches!(last.phase, Phase::Complete | Phase::Cancel)) {
            Some(ResultCode::Stale)
        } else if issued != current {
            if !Self::pay_route(cx, false)? { return Ok(Some(self.observe(transport, phase, ResultCode::Retry))); }
            self.invalidated = true;
            Some(ResultCode::Stale)
        } else if cursor.page_index < self.next_page {
            Some(ResultCode::Accepted)
        } else if self.waiting == Some(phase) {
            Some(ResultCode::Pending)
        } else if cursor.page_index != self.next_page {
            Some(ResultCode::Refused)
        } else { None };
        if let Some(result) = result { return Ok(Some(self.observe(transport, phase, result))); }
        if !Self::pay_route(cx, false)? { return Ok(Some(self.observe(transport, phase, ResultCode::Retry))); }
        let ingress = self.ingress.as_mut().expect("authenticated live route owns a receiver");
        if !ingress.stage_page(current, incoming, cx)? {
            return Ok(Some(self.observe(transport, phase, ResultCode::Retry)));
        }
        self.waiting = Some(phase);
        Ok(Some(self.record(transport, phase, ResultCode::Pending)))
    }

    /// 🔁 Returns a fresh transition only, while exact control/page retries can retrieve their receipts.
    pub fn poll(&mut self, current: OriginalMediaLoan<'_>, format_closed: bool, cx: &mut StepContext<'_>)
        -> Result<Option<Ack<'_>>, OriginalTurnFailure>
    {
        let (Some(phase), Some(transport)) = (self.waiting, self.active) else { return Ok(None); };
        self.validate_source(cx)?;
        let retiring = phase == Phase::Cancel || phase == Phase::Complete && self.format_result.is_some();
        let result = if retiring {
            if !format_closed || !self.close_owned(cx)? { return Ok(None); }
            if !Self::pay_route(cx, true)? { return Ok(None); }
            if phase == Phase::Complete && self.format_result == Some(FormatResult::Refused) {
                ResultCode::Refused
            } else {
                ResultCode::Accepted
            }
        } else {
            if phase == Phase::Complete && !self.invalidated && (self.format_pending || self.issued_loan() == Some(current)) { return Ok(None); }
            if !Self::pay_route(cx, false)? { return Ok(None); }
            if self.invalidated || self.issued_loan() != Some(current) {
                self.invalidated = true;
                self.waiting = None;
                return Ok(Some(self.record(transport, phase, ResultCode::Stale)));
            }
            let ingress = self.ingress.as_mut().expect("pending raw page owns a receiver");
            let Some(index) = ingress.page_progress(current, cx)? else { return Ok(None); };
            debug_assert_eq!(phase, Phase::Page(index));
            self.next_page += 1;
            ResultCode::Accepted
        };
        self.waiting = None;
        Ok(Some(self.record(transport, phase, result)))
    }

    pub fn format_input(&mut self) -> Option<&mut OriginalGuestMediaInput<T, N>> {
        if self.invalidated || self.waiting != Some(Phase::Complete) || self.format_result.is_some() {
            None
        } else {
            self.output.as_mut()
        }
    }

    /// 🎁 Moves the Original PNG input to its registered format recipient once its transport is complete.
    pub fn take_format_input(
        &mut self, current: OriginalMediaLoan<'_>, output: &mut Option<Box<OriginalGuestMediaInput<T, N>>>, cx: &mut StepContext<'_>,
    ) -> Result<bool, OriginalTurnFailure> {
        self.validate_source(cx)?;
        if output.is_some() || self.issued_loan() != Some(current) || self.format_input().is_none() {
            return Ok(false);
        }
        let progress = RetainedCloneProgress {
            copied_items: 1, copied_bytes: size_of::<Option<OriginalGuestMediaInput<T, N>>>() + size_of::<Option<Box<OriginalGuestMediaInput<T, N>>>>() + size_of::<bool>(),
            retained_capacity_bytes: size_of::<OriginalGuestMediaInput<T, N>>(),
            ..Default::default()
        };
        let grant = cx.retained_grant();
        if cx.should_yield() || cx.is_cancelled() || cx.fuel_remaining() == 0 || grant.maximum_depth == 0 || !progress.fits(grant) {
            return Ok(false);
        }
        cx.consume_retained(progress)?;
        cx.consume_fuel(1);
        *output = self.output.take().map(Box::new);
        self.format_pending = true;
        Ok(true)
    }

    pub fn cancellation_pending(&self) -> bool { self.superseding || self.waiting == Some(Phase::Cancel) }
    pub fn receipt_waiting(&self) -> bool { self.waiting.is_some() }
    pub fn receipt_due(&self) -> bool { self.superseding || self.waiting.is_some() }

    /// 🛑️ Extinguishes an unpublished format before a newer trusted picker is installed.
    pub fn request_supersession(&mut self, cx: &mut StepContext<'_>) -> Result<bool, OriginalTurnFailure> {
        self.validate_source(cx)?;
        if self.format_result == Some(FormatResult::Published) || self.cancellation_pending() { return Ok(true); }
        if !Self::pay_route(cx, true)? { return Ok(false); }
        self.invalidated = true;
        self.superseding = true;
        Ok(true)
    }

    /// 📐 Quotes the fixed receipt and its future result slot before Store publication.
    pub fn format_receipt_progress() -> RetainedCloneProgress {
        RetainedCloneProgress {
            copied_items: 1, copied_bytes: size_of::<FormatReceipt>() + size_of::<Option<FormatResult>>(), ..Default::default()
        }
    }

    /// 🪙 Reserves the complete terminal receipt for this exact pending picker publication.
    pub fn reserve_format_receipt(&self, cx: &mut StepContext<'_>) -> Result<Option<FormatReceipt>, OriginalTurnFailure> {
        self.validate_source(cx)?;
        if self.invalidated || self.waiting != Some(Phase::Complete) || self.format_result.is_some() { return Ok(None); }
        let (Some(issued), Some(transport)) = (self.issued_loan(), self.active) else { return Ok(None); };
        let progress = Self::format_receipt_progress();
        if cx.should_yield() || cx.is_cancelled() || cx.retained_grant().maximum_depth == 0 || !progress.fits(cx.retained_grant()) {
            return Ok(None);
        }
        cx.consume_retained(progress)?;
        cx.consume_fuel(1);
        Ok(Some(FormatReceipt {
            source: OriginalSourceBinding::borrow(cx), operation: issued.operation_id, generation: issued.generation,
            instance: issued.instance_id, transport,
        }))
    }

    /// 🏁 Records the result of the actual Store publication using the already funded receipt.
    pub fn publish_format_receipt(&mut self, receipt: FormatReceipt, result: FormatResult, cx: &StepContext<'_>) -> Result<bool, OriginalTurnFailure> {
        self.validate_source(cx)?;
        let Some(issued) = self.issued_loan() else { return Ok(false); };
        if !receipt.source.matches(cx) ||
            (receipt.operation, receipt.generation, receipt.instance) != (issued.operation_id, issued.generation, issued.instance_id)
        {
            return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit, "Original format result changed its publication caller").into());
        }
        if self.invalidated || self.waiting != Some(Phase::Complete) || self.active != Some(receipt.transport) || self.format_result.is_some() {
            return Ok(false);
        }
        self.format_result = Some(result);
        Ok(true)
    }

    /// 📬 Records format completion when no irreversible publication needs to reserve ahead.
    pub fn finish_format(&mut self, result: FormatResult, cx: &mut StepContext<'_>) -> Result<bool, OriginalTurnFailure> {
        self.validate_source(cx)?;
        if self.invalidated || self.waiting != Some(Phase::Complete) { return Ok(false); }
        if let Some(previous) = self.format_result { return Ok(previous == result); }
        let Some(receipt) = self.reserve_format_receipt(cx)? else { return Ok(false); };
        self.publish_format_receipt(receipt, result, cx)
    }

    pub fn terminal_is_empty(&self) -> bool { self.intent.is_none() }

    /// 📏 Quotes the next original retirement frontier before the instance can return its Source.
    pub fn close_demand(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        if let Some(ingress) = self.ingress.as_ref() {
            if let Some(demand) = ingress.close_demand(body)? { return Ok(demand); }
        }
        if let Some(output) = self.output.as_ref() {
            if let Some(demand) = output.close_demand(body)? { return Ok(demand); }
        }
        Ok(if self.terminal_is_empty() { Default::default() } else {
            RetirementDemand { copy_bytes: size_of::<Self>(), depth: 1, ..Default::default() }
        })
    }

    fn close_owned(&mut self, cx: &mut StepContext<'_>) -> Result<bool, OriginalTurnFailure> {
        cx.with_original_retirement(|cx| {
            if let Some(ingress) = self.ingress.as_mut() {
                if !matches!(ingress.close(cx)?, InteractiveJobCloseStep::Complete { .. }) { return Ok(false); }
            }
            if let Some(output) = self.output.as_mut() {
                if !matches!(output.close(cx)?, InteractiveJobCloseStep::Complete { .. }) { return Ok(false); }
            }
            Ok(true)
        })
    }

    /// ♻️ Runs before the authenticated instance returns its Original command-origin loan.
    pub fn close(&mut self, cx: &mut StepContext<'_>) -> Result<bool, OriginalTurnFailure> {
        if self.intent.is_none() { return Ok(true); }
        self.validate_source(cx)?;
        if !self.close_owned(cx)? { return Ok(false); }
        let progress = RetainedCloneProgress { copied_items: 1, copied_bytes: size_of::<Self>(), ..Default::default() };
        if cx.retirement_should_yield() || cx.fuel_remaining() == 0 || cx.retained_grant().maximum_depth == 0 ||
            !progress.fits(cx.retained_grant())
        {
            return Ok(false);
        }
        cx.consume_retained(progress)?;
        cx.consume_fuel(1);
        *self = Self::empty();
        Ok(true)
    }
}

fn command_binding(loan: OriginalMediaLoan<'_>, transport: Transport) -> OriginalMediaCommandBinding<'_> {
    OriginalMediaCommandBinding { loan, owner: transport.owner, seq: transport.seq, kind: transport.kind, declared: transport.extent }
}

