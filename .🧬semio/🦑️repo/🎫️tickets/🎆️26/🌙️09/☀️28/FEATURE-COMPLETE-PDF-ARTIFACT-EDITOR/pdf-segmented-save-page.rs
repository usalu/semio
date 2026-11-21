//! 🌊️ Feeds borrowed PDF writer spans to the mounted artifact’s actual segmented output queue.

use std::io::{self, Write};
use semio_framework_job::{OriginalConstructionDemand, OriginalTurnFailure, StepContext};
use semio_framework_value::{RetainedCloneProgress, ValueError, ValueRefusalKind};
use semio_plugin_decl_app::media_contract::MediaConstruction;
use semio_plugin_decl_media::{ArtifactMediaExportCredit, ArtifactOutputChunks};

/// 🌱️ Settles the birth of one writable output page before the sequential writer can borrow it.
/// Completes the same attempt across turns. Begin the next attempt after donating the settled page.
pub fn admit_pdf_save_page(
    page: &mut Option<Vec<u8>>,
    construction: &mut MediaConstruction,
    cx: &mut StepContext<'_>,
) -> Result<bool, OriginalTurnFailure> {
    if page.is_some() && construction.terminal_is_empty() { return Ok(true); }
    let capacity = ArtifactOutputChunks::CHUNK_BYTES;
    let copy = size_of::<Vec<u8>>();
    construction.built(cx, page, OriginalConstructionDemand {
        items: 1, copy_bytes: copy, capacity_bytes: capacity, release_bytes: 0, depth: 1,
    }, || {
        let mut buffer = Vec::new();
        buffer.try_reserve_exact(capacity).map_err(|_| ValueError::literal(ValueRefusalKind::AllocationFailed, "PDF output page allocation refused"))?;
        let receipt = RetainedCloneProgress { copied_items: 1, copied_bytes: copy, retained_capacity_bytes: buffer.capacity(), ..Default::default() };
        Ok((buffer, receipt))
    })
}

/// 🖋️ Borrows only the paid output frontier, then accounts for the actual emitted byte prefix.
pub fn write_pdf_save_turn<R>(
    page: &mut Vec<u8>,
    cx: &mut StepContext<'_>,
    write: impl FnOnce(&mut dyn Write, usize) -> R,
) -> Result<Option<R>, OriginalTurnFailure> {
    cx.original_source_parent()?;
    let cap = ArtifactOutputChunks::CHUNK_BYTES;
    if page.len() > cap || page.capacity() < cap {
        return Err(ValueError::invariant("PDF writer lost its admitted output page").into());
    }
    let grant = cx.retained_grant();
    if cx.should_yield() || cx.is_cancelled() || grant.maximum_items == 0 || grant.maximum_depth == 0 { return Ok(None); }
    let before = page.len();
    let allowance = (cap - before).min(grant.maximum_copy_bytes)
        .min(usize::try_from(cx.fuel_remaining()).unwrap_or(usize::MAX));
    if allowance == 0 { return Ok(None); }
    let mut sink = OutputWindow { page, remaining: allowance };
    let output = write(&mut sink, allowance);
    let total = allowance - sink.remaining;
    if total != 0 {
        cx.consume_retained(RetainedCloneProgress { copied_items: 1, copied_bytes: total, ..Default::default() })?;
        cx.consume_fuel(total as u64);
    }
    Ok(Some(output))
}

struct OutputWindow<'a> {
    page: &'a mut Vec<u8>,
    remaining: usize,
}

impl Write for OutputWindow<'_> {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        if bytes.is_empty() { return Ok(0); }
        let copied = bytes.len().min(self.remaining);
        if copied == 0 { return Err(io::ErrorKind::WouldBlock.into()); }
        self.page.extend_from_slice(&bytes[..copied]);
        self.remaining -= copied;
        Ok(copied)
    }

    fn flush(&mut self) -> io::Result<()> { Ok(()) }
}

/// 📦️ Donates the exact physical page after it fills or the PDF writer reaches its terminal xref.
pub fn donate_pdf_save_page(
    page: &mut Option<Vec<u8>>,
    finished: bool,
    output: &ArtifactOutputChunks,
    credit: Option<&ArtifactMediaExportCredit>,
    cx: &mut StepContext<'_>,
) -> Result<bool, OriginalTurnFailure> {
    let Some(buffer) = page.as_ref() else { return Ok(true); };
    if buffer.is_empty() || !finished && buffer.len() != ArtifactOutputChunks::CHUNK_BYTES { return Ok(false); }
    output.push_original(page, credit, cx)
}
