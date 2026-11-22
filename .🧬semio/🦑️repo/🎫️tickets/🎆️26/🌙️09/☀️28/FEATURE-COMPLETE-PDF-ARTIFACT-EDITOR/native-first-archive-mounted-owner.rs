//! 🎟️ Owns one funded first-archive cursor while borrowing the exact Store Pack per request.
use super::FirstGenesisArchiveCursor;
use semio_framework_job::{OriginalTurnFailure, StepContext};
use semio_framework_value::{ByteReadSource, RetainedCloneProgress, RetirementDemand, ValueError};
use semio_os_spr::channel::{FirstArchiveAction, FirstArchiveKey, FirstArchiveReply, FirstArchiveRequest, FIRST_GENESIS_ARCHIVE_PAGE_BYTES};
use semio_os_spr::history::FirstGenesisHistory;
use semio_os_vcs::io::binary::genesis::ArtifactGenesisPack;

#[derive(Default)]
pub struct MountedFirstArchive {
    live: Option<Box<Resident>>,
    closed: Option<FirstArchiveKey>,
}

struct Resident {
    key: FirstArchiveKey,
    cursor: FirstGenesisArchiveCursor,
    spr: Vec<u8>,
}

impl MountedFirstArchive {
    pub fn has_resident(&self) -> bool { self.live.is_some() }

    pub fn retirement_demand(&self) -> RetirementDemand {
        RetirementDemand {
            copy_bytes: size_of::<Self>(), depth: 1,
            release_bytes: self.live.as_ref().map_or(0, |live| size_of::<Resident>() + live.spr.capacity()),
            ..Default::default()
        }
    }

    /// 🧹 Pays for releasing the small SPR and cursor together. Store retains sole Pack custody.
    pub fn retire(&mut self, cx: &mut StepContext<'_>) -> Result<bool, OriginalTurnFailure> {
        let Some(live) = self.live.as_ref() else { return Ok(true); };
        let key = live.key;
        let demand = self.retirement_demand();
        let charge = RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes,
            released_bytes: demand.release_bytes, ..Default::default() };
        let grant = cx.retained_grant();
        if cx.retirement_should_yield() || cx.fuel_remaining() == 0 || grant.maximum_depth < demand.depth || !charge.fits(grant) {
            return Ok(false);
        }
        cx.consume_retained(charge)?;
        cx.consume_fuel(1);
        self.live.take();
        self.closed = Some(key);
        Ok(true)
    }

    /// 📤 Retains one bounded reply frontier through Preview, exact Ack, Seal and Close.
    pub fn exchange(
        &mut self, request: FirstArchiveRequest, document: &str, current: (u64, [u8; 32]),
        committed: (u64, [u8; 32]), pack: &ArtifactGenesisPack, history: Option<&FirstGenesisHistory<'_>>,
        output: &mut Vec<u8>, cx: &mut StepContext<'_>,
    ) -> Result<(), OriginalTurnFailure> {
        use FirstArchiveAction as Action;
        use FirstArchiveReply as Reply;
        let closing = matches!(request.action, Action::Close | Action::Cancel);
        let reply = if self.closed.is_some() {
            if self.closed == Some(request.key) && closing { Reply::Closed } else { Reply::Refused }
        } else if self.live.as_ref().is_some_and(|live| live.key != request.key) {
            Reply::Refused
        } else if matches!(request.action, Action::Cancel) {
            if self.live.is_none() {
                let charge = RetainedCloneProgress { copied_items: 1, copied_bytes: size_of_val(&self.closed), ..Default::default() };
                if cx.retirement_should_yield() || cx.fuel_remaining() == 0 || cx.retained_grant().maximum_depth == 0
                    || !charge.fits(cx.retained_grant()) { return Ok(()); }
                cx.consume_retained(charge)?;
                cx.consume_fuel(1);
                self.closed = Some(request.key);
                Reply::Closed
            } else if self.retire(cx)? { Reply::Closed } else { Reply::Pending }
        } else if current != committed || document.is_empty() || document.chars().count() > 512 || pack.contiguous().is_some() {
            Reply::Refused
        } else if matches!(request.action, Action::Open) {
            if self.live.is_none() {
                let Some(plan) = history else { return write(Reply::Refused, request, output); };
                let Ok(cursor_extent) = FirstGenesisArchiveCursor::extent_for(pack.byte_len(), plan.extent())
                else { return write(Reply::Refused, request, output); };
                let charge = RetainedCloneProgress {
                    copied_items: 2, copied_bytes: size_of::<Self>() + size_of::<Resident>() + 2 * plan.extent(),
                    retained_capacity_bytes: size_of::<Resident>() + plan.extent(), ..Default::default()
                };
                let work = (4 * plan.extent() + size_of::<Resident>()) as u64;
                let grant = cx.retained_grant();
                if cx.should_yield() || cx.is_cancelled() || cx.fuel_remaining() < work
                    || grant.maximum_depth == 0 || !charge.fits(grant) { return Ok(()); }
                let mut spr = Vec::new();
                spr.try_reserve_exact(plan.extent()).map_err(|_| ValueError::invariant("funded first SPR allocation refused"))?;
                spr.resize(plan.extent(), 0);
                plan.encode(&mut spr).map_err(|_| ValueError::invariant("funded first SPR did not fit its exact framing"))?;
                let cursor = FirstGenesisArchiveCursor::new(pack.byte_len(), spr.len())?;
                debug_assert_eq!(cursor.extent(), cursor_extent);
                self.live = Some(Box::new(Resident { key: request.key, cursor, spr }));
                cx.consume_retained(charge)?;
                cx.consume_fuel(work);
            }
            let live = self.live.as_ref().expect("opened first archive has a producer");
            Reply::Opened { extent: live.cursor.extent() as u32, document, generation: current.0, revision: current.1 }
        } else if matches!(request.action, Action::Close)
            && self.live.as_ref().and_then(|live| live.cursor.digest()).is_some() {
            if self.retire(cx)? { Reply::Closed } else { Reply::Pending }
        } else if let Some(live) = self.live.as_mut() {
            let cursor = &mut live.cursor;
            let expected = cursor.accepted();
            match request.action {
                Action::Preview { offset } if offset as usize == expected && expected < cursor.extent() =>
                    cursor.offer(pack, &live.spr, cx)?.map_or(Reply::Pending, |(_, bytes)| Reply::Page(bytes)),
                Action::Acknowledge { offset, length } if offset as usize % FIRST_GENESIS_ARCHIVE_PAGE_BYTES == 0
                    && (offset as usize) < cursor.extent()
                    && length as usize == (cursor.extent() - offset as usize).min(FIRST_GENESIS_ARCHIVE_PAGE_BYTES) => {
                        if offset as usize + length as usize == expected { Reply::Acknowledged }
                        else if offset as usize != expected || !cursor.has_pending() { Reply::Refused }
                        else if cursor.acknowledge(offset as usize, length as usize, cx)? { Reply::Acknowledged }
                        else { Reply::Pending }
                    }
                Action::Seal => cursor.digest().map_or(Reply::Refused, Reply::Sealed),
                _ => Reply::Refused,
            }
        } else {
            Reply::Refused
        };
        write(reply, request, output)
    }
}

fn write(reply: FirstArchiveReply<'_>, request: FirstArchiveRequest, output: &mut Vec<u8>) -> Result<(), OriginalTurnFailure> {
    if !reply.encode_into(request, output) { return Err(ValueError::invariant("funded first archive response recipient is absent").into()); }
    Ok(())
}
