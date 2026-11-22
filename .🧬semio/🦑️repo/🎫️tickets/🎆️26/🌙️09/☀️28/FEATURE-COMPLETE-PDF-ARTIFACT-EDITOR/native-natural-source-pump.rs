//! 📂 Streams a selected natural file through exact guest-issued source receipts.
use semio_framework::kernel::{
    CommandIngressStatus, CommandPageCursor, Event, FixedCommandPage, NaturalSourceControlEvent,
    NaturalSourcePrepare, NaturalSourceWireAck, NaturalSourceWireControl, NaturalSourceWireLoan,
    NaturalSourceWirePhase as Phase, NaturalSourceWireResult as Receipt, COMMAND_PAGE_MAXIMUM_BYTES,
    NATURAL_SOURCE_COMMAND_KIND,
};
use std::{future::Future, io::Read, time::{Duration, Instant}};

enum Crossing<'a> {
    Prepare,
    Begin,
    Page(u32, &'a [u8]),
    Complete,
    Cancel,
}

struct Transfer {
    prepare: NaturalSourcePrepare,
    loan: Option<NaturalSourceWireLoan>,
    generation: u64,
}

/// 🎛️ Keeps one read page until the guest admits its exact position and reports progress.
pub(super) async fn stream<F: Future<Output = Result<CommandIngressStatus, String>>>(
    prepare: NaturalSourcePrepare,
    mut source: impl Read,
    mut cross: impl FnMut(Event, bool) -> F,
    current: impl Fn() -> bool,
    mut progress: impl FnMut(u64, u64),
) -> Result<Option<NaturalSourceWireAck>, String> {
    if prepare.instance_id == 0 || prepare.seq == 0 || prepare.extent == 0 {
        return Err("natural-source.invalid-prepare".into());
    }
    let mut transfer = Transfer { prepare, loan: None, generation: 0 };
    let extent = u64::from(transfer.prepare.extent);
    let result = async {
        transfer.cross(Crossing::Prepare, &mut cross, &current).await?;
        transfer.cross(Crossing::Begin, &mut cross, &current).await?;
        let mut page = [0; COMMAND_PAGE_MAXIMUM_BYTES];
        let mut offset = 0;
        let mut index = 0;
        while offset < extent {
            if !current() { return Ok(None); }
            let count = (extent - offset).min(COMMAND_PAGE_MAXIMUM_BYTES as u64) as usize;
            source.read_exact(&mut page[..count]).map_err(|error| error.to_string())?;
            transfer.cross(Crossing::Page(index, &page[..count]), &mut cross, &current).await?;
            offset += count as u64;
            index += 1;
            progress(offset, extent);
        }
        if !current() { return Ok(None); }
        if source.read(&mut page[..1]).map_err(|error| error.to_string())? != 0 {
            return Err("natural-source.changed-extent".to_owned());
        }
        let complete = transfer.cross(Crossing::Complete, &mut cross, &current).await?;
        Ok(current().then_some(complete))
    }.await;
    if matches!(result, Ok(Some(_))) { return result; }
    if transfer.loan.is_some() {
        if let Err(close) = transfer.cross(Crossing::Cancel, &mut cross, &|| true).await {
            return Err(match result {
                Err(failure) => format!("{failure}; natural-source close: {close}"),
                _ => close,
            });
        }
    }
    if current() { result } else { Ok(None) }
}

impl Transfer {
    async fn cross<F: Future<Output = Result<CommandIngressStatus, String>>>(
        &mut self, crossing: Crossing<'_>, cross: &mut impl FnMut(Event, bool) -> F, current: &impl Fn() -> bool,
    ) -> Result<NaturalSourceWireAck, String> {
        let deadline = Instant::now() + Duration::from_secs(300);
        let (phase, index) = match crossing {
            Crossing::Prepare => (Phase::Prepare, None),
            Crossing::Begin => (Phase::Begin, None),
            Crossing::Page(index, _) => (Phase::Page, Some(index)),
            Crossing::Complete => (Phase::Complete, None),
            Crossing::Cancel => (Phase::Cancel, None),
        };
        let mut offer = true;
        loop {
            if !current() { return Err("natural-source.cancelled".into()); }
            if Instant::now() >= deadline { return Err("natural-source.ack-stalled".into()); }
            let event = if offer { self.event(&crossing)? } else { Event::Wake };
            offer = match cross(event, phase == Phase::Cancel).await? {
                CommandIngressStatus::NaturalSourceAck(ack) => {
                    self.check_ack(&ack, phase, index)?;
                    match ack.result {
                        Receipt::Accepted => return Ok(ack),
                        Receipt::Stale if phase == Phase::Cancel => return Ok(ack),
                        Receipt::Stale => return Err("natural-source.stale".into()),
                        Receipt::Refused => return Err("natural-source.refused".into()),
                        Receipt::Retry => true,
                        Receipt::Pending => false,
                    }
                }
                CommandIngressStatus::Idle => false,
                CommandIngressStatus::Backpressure(cursor) if index.is_some_and(|i| cursor == self.cursor(i)) => true,
                _ => return Err("natural-source.foreign-ingress".into()),
            };
        }
    }

    fn check_ack(&mut self, ack: &NaturalSourceWireAck, phase: Phase, index: Option<u32>) -> Result<(), String> {
        let invalid = "natural-source.foreign-ack";
        if ack.owner != u64::from(self.prepare.instance_id) || ack.seq != self.prepare.seq
            || ack.kind != NATURAL_SOURCE_COMMAND_KIND || ack.extent != self.prepare.extent
            || ack.phase != phase || ack.page_index != index
        { return Err(invalid.into()); }
        if phase != Phase::Prepare {
            return if ack.loan.is_some() && ack.loan == self.loan { Ok(()) } else { Err(invalid.into()) };
        }
        let Some(loan) = ack.loan.as_ref() else {
            return if matches!(ack.result, Receipt::Refused | Receipt::Stale) { Ok(()) } else { Err(invalid.into()) };
        };
        let generation = nonzero_hex(&loan.generation).ok_or(invalid)?;
        if nonzero_hex(&loan.operation_id).is_none() || loan.instance_id != self.prepare.instance_id
            || loan.declaration != self.prepare.declaration || loan.extent != self.prepare.extent
            || self.loan.as_ref().is_some_and(|existing| existing != loan)
        { return Err(invalid.into()); }
        if self.loan.is_none() {
            self.loan = Some(loan.clone());
            self.generation = generation;
        }
        Ok(())
    }

    fn event(&self, crossing: &Crossing<'_>) -> Result<Event, String> {
        let control = match crossing {
            Crossing::Prepare => return Ok(Event::NaturalSourcePrepare(self.prepare.clone())),
            Crossing::Page(index, bytes) => return Ok(Event::CommandIngressPage {
                cursor: self.cursor(*index),
                bytes: FixedCommandPage::try_copy_from(bytes).map_err(|error| error.describe())?,
            }),
            Crossing::Begin => NaturalSourceWireControl::Begin,
            Crossing::Complete => NaturalSourceWireControl::Complete,
            Crossing::Cancel => NaturalSourceWireControl::Cancel,
        };
        Ok(Event::NaturalSourceControl(NaturalSourceControlEvent {
            loan: self.loan.clone().ok_or("natural-source.missing-loan")?,
            owner: u64::from(self.prepare.instance_id), seq: self.prepare.seq,
            kind: NATURAL_SOURCE_COMMAND_KIND, extent: self.prepare.extent, control,
        }))
    }

    fn cursor(&self, index: u32) -> CommandPageCursor {
        CommandPageCursor {
            owner: u64::from(self.prepare.instance_id), generation: self.generation,
            instance: self.prepare.instance_id, seq: self.prepare.seq, kind: NATURAL_SOURCE_COMMAND_KIND,
            command_index: 0, command_count: 1, item_count: 1, page_index: index,
            page_count: self.prepare.extent.div_ceil(COMMAND_PAGE_MAXIMUM_BYTES as u32), metadata: self.prepare.extent,
        }
    }
}

fn nonzero_hex(value: &str) -> Option<u64> {
    if value.len() != 16 || !value.bytes().all(|byte| matches!(byte, b'0'..=b'9' | b'a'..=b'f')) { return None; }
    u64::from_str_radix(value, 16).ok().filter(|value| *value != 0)
}
