//! 🗂️ Raw picker command child held by one authenticated native app lifetime.

use semio_framework::kernel::{
    CommandPageCursor, FixedCommandPage, OriginalMediaControlEvent, OriginalMediaWireAck,
    OriginalMediaWireControl, OriginalMediaWireLoan, OriginalMediaWirePhase, OriginalMediaWireResult,
    ORIGINAL_MEDIA_COMMAND_KIND,
};
use semio_framework_job::{OriginalTurnFailure, StepContext};
use semio_framework_value::{RetainedCloneProgress, ValueError, ValueRefusalKind};
use crate::original_media::{
    Ack, Control, DecodedLoan, OriginalMediaLoan, Phase, ResultCode, Transport,
};

pub use crate::original_media::{RawMediaIntent, RawMediaPicker, RawMediaTarget, RAW_MEDIA_BYTES};

pub type RawLiveDocument<'a> = (&'a str, [u8; 32]);

pub struct OriginalMediaAdmission {
    pub picker: RawMediaPicker,
    pending_receipt: Option<OriginalMediaWireLoan>,
}

impl OriginalMediaAdmission {
    pub fn empty() -> Self { Self { picker: RawMediaPicker::empty(), pending_receipt: None } }

    /// 📮️ Admits the exact retained result produced by the mounted application.
    pub fn admit_issued(
        &mut self, offered: &mut Option<RawMediaIntent>, current: OriginalMediaLoan<'_>, cx: &mut StepContext<'_>,
    ) -> Result<bool, OriginalTurnFailure> {
        if offered.as_ref().is_some_and(|intent| intent.kind() != ORIGINAL_MEDIA_COMMAND_KIND) {
            return Err(ValueError::literal(ValueRefusalKind::InvalidValue, "Original media used an unregistered fixed-page kind").into());
        }
        self.picker.admit_issued(offered, current, cx)
    }

    pub fn terminal_is_empty(&self) -> bool { self.picker.terminal_is_empty() && self.pending_receipt.is_none() }

    pub fn close(&mut self, cx: &mut StepContext<'_>) -> Result<bool, OriginalTurnFailure> {
        if let Some(loan) = self.pending_receipt.as_ref() {
            let progress = RetainedCloneProgress { copied_items: 1, copied_bytes: size_of::<Option<OriginalMediaWireLoan>>(),
                released_bytes: receipt_capacity(loan), ..Default::default() };
            let grant = cx.retained_grant();
            if cx.retirement_should_yield() || grant.maximum_depth == 0 || cx.fuel_remaining() == 0 || !progress.fits(grant) {
                return Ok(false);
            }
            cx.consume_retained(progress)?;
            cx.consume_fuel(1);
            self.pending_receipt.take();
        }
        self.picker.close(cx)
    }

    pub fn control(
        &mut self, event: &OriginalMediaControlEvent, live: Option<&RawLiveDocument<'_>>, cx: &mut StepContext<'_>,
    ) -> Result<Option<OriginalMediaWireAck>, OriginalTurnFailure> {
        let transport = Transport { owner: event.owner, seq: event.seq, kind: event.kind, extent: event.extent };
        let (command, phase) = match &event.control {
            OriginalMediaWireControl::Begin { name, media_type } => (Control::Begin { name, media_type }, Phase::Begin),
            OriginalMediaWireControl::Complete => (Control::Complete, Phase::Complete),
            OriginalMediaWireControl::Cancel => (Control::Cancel, Phase::Cancel),
        };
        let decoded = decode_loan(&event.loan)?;
        let offered = decoded.as_loan();
        let Some(home) = receipt_home(offered, cx)? else { return Ok(None); };
        let ack = if event.kind != ORIGINAL_MEDIA_COMMAND_KIND || self.terminal_is_empty() {
            Ack { loan: offered, transport, phase, result: ResultCode::Stale }
        } else {
            self.picker.control(offered, observe(offered, live), transport, command, cx)?
        };
        Ok(Some(receipt(home, ack)))
    }

    pub fn page(
        &mut self, live: Option<&RawLiveDocument<'_>>, page: &mut Option<(CommandPageCursor, FixedCommandPage)>, cx: &mut StepContext<'_>,
    ) -> Result<Option<OriginalMediaWireAck>, OriginalTurnFailure> {
        let Some(issued) = self.picker.issued_loan() else { return Ok(None); };
        if self.pending_receipt.is_none() { self.pending_receipt = receipt_home(issued, cx)?; }
        if self.pending_receipt.is_none() { return Ok(None); }
        let active = current_root(issued, live);
        let ack = self.picker.page(active.loan(), page, cx)?;
        Ok(ack.map(|ack| receipt(self.pending_receipt.take().expect("funded raw page receipt"), ack)))
    }

    pub fn poll(
        &mut self, live: Option<&RawLiveDocument<'_>>, format_closed: bool, cx: &mut StepContext<'_>,
    ) -> Result<Option<OriginalMediaWireAck>, OriginalTurnFailure> {
        let Some(issued) = self.picker.issued_loan() else { return Ok(None); };
        if !self.picker.receipt_due() { return Ok(None); }
        if self.pending_receipt.is_none() { self.pending_receipt = receipt_home(issued, cx)?; }
        if self.pending_receipt.is_none() { return Ok(None); }
        let active = current_root(issued, live);
        let ack = self.picker.poll(active.loan(), format_closed, cx)?;
        Ok(ack.map(|ack| receipt(self.pending_receipt.take().expect("funded raw continuation receipt"), ack)))
    }
}

fn decode_loan(value: &OriginalMediaWireLoan) -> Result<DecodedLoan<'_>, OriginalTurnFailure> {
    DecodedLoan::read(&value.operation_id, &value.generation, value.instance_id, &value.document_id, &value.base_revision)
        .ok_or_else(|| ValueError::literal(ValueRefusalKind::InvalidValue, "Original media control loan is outside its wire bounds").into())
}

fn observe<'a>(issued: OriginalMediaLoan<'a>, live: Option<&'a RawLiveDocument<'a>>) -> OriginalMediaLoan<'a> {
    match live {
        Some((document, revision)) => OriginalMediaLoan { document_id: document, base_revision: revision, ..issued },
        None => OriginalMediaLoan { document_id: "", ..issued },
    }
}

struct LiveRoot<'a> {
    operation_id: u64,
    generation: u64,
    instance_id: u32,
    document: Option<&'a RawLiveDocument<'a>>,
}

impl LiveRoot<'_> {
    fn loan(&self) -> OriginalMediaLoan<'_> {
        let (document_id, base_revision) = self.document.map_or(("", &[0; 32]), |(id, revision)| (*id, revision));
        OriginalMediaLoan { operation_id: self.operation_id, generation: self.generation,
            instance_id: self.instance_id, document_id, base_revision }
    }
}

fn current_root<'a>(issued: OriginalMediaLoan<'_>, document: Option<&'a RawLiveDocument<'a>>) -> LiveRoot<'a> {
    LiveRoot { operation_id: issued.operation_id, generation: issued.generation, instance_id: issued.instance_id, document }
}

fn receipt_home(
    value: OriginalMediaLoan<'_>, cx: &mut StepContext<'_>,
) -> Result<Option<OriginalMediaWireLoan>, OriginalTurnFailure> {
    let bytes = 96 + value.document_id.len();
    let place = RetainedCloneProgress { copied_items: 1, copied_bytes: bytes + size_of::<OriginalMediaWireAck>(),
        retained_capacity_bytes: bytes, ..Default::default() };
    let grant = cx.retained_grant();
    if cx.retirement_should_yield() || grant.maximum_depth == 0 || cx.fuel_remaining() == 0 || !place.fits(grant) {
        return Ok(None);
    }
    let revision: [u8; 64] = std::array::from_fn(|index| nibble(value.base_revision[index / 2] >> (4 * (1 - index % 2)) & 15));
    let operation: [u8; 16] = std::array::from_fn(|index| nibble((value.operation_id >> (4 * (15 - index))) as u8 & 15));
    let generation: [u8; 16] = std::array::from_fn(|index| nibble((value.generation >> (4 * (15 - index))) as u8 & 15));
    let loan = OriginalMediaWireLoan {
        operation_id: ascii(&operation)?, generation: ascii(&generation)?, instance_id: value.instance_id,
        document_id: copy_text(value.document_id)?, base_revision: ascii(&revision)?,
    };
    cx.consume_retained(RetainedCloneProgress { retained_capacity_bytes: receipt_capacity(&loan), ..place })?;
    cx.consume_fuel(1);
    Ok(Some(loan))
}

fn receipt_capacity(loan: &OriginalMediaWireLoan) -> usize {
    loan.operation_id.capacity() + loan.generation.capacity() + loan.base_revision.capacity() + loan.document_id.capacity()
}

fn nibble(n: u8) -> u8 { b"0123456789abcdef"[usize::from(n)] }

fn ascii(value: &[u8]) -> Result<String, OriginalTurnFailure> {
    copy_text(std::str::from_utf8(value).expect("hexadecimal receipt text"))
}

fn copy_text(value: &str) -> Result<String, OriginalTurnFailure> {
    let mut output = String::new();
    output.try_reserve_exact(value.len())
        .map_err(|_| ValueError::literal(ValueRefusalKind::AllocationFailed, "Original media receipt allocation refused"))?;
    output.push_str(value);
    Ok(output)
}

fn receipt(loan: OriginalMediaWireLoan, ack: Ack<'_>) -> OriginalMediaWireAck {
    let (phase, page_index) = match ack.phase {
        Phase::Begin => (OriginalMediaWirePhase::Begin, None),
        Phase::Page(index) => (OriginalMediaWirePhase::Page, Some(index)),
        Phase::Complete => (OriginalMediaWirePhase::Complete, None),
        Phase::Cancel => (OriginalMediaWirePhase::Cancel, None),
    };
    let result = match ack.result {
        ResultCode::Retry => OriginalMediaWireResult::Retry,
        ResultCode::Pending => OriginalMediaWireResult::Pending,
        ResultCode::Accepted => OriginalMediaWireResult::Accepted,
        ResultCode::Stale => OriginalMediaWireResult::Stale,
        ResultCode::Refused => OriginalMediaWireResult::Refused,
    };
    let Transport { owner, seq, kind, extent } = ack.transport;
    OriginalMediaWireAck { loan, owner, seq, kind, extent, phase, result, page_index }
}
