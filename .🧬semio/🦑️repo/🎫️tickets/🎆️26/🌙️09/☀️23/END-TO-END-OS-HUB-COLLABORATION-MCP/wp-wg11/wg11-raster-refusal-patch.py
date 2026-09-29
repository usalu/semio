#!/usr/bin/env python3
"""🖼️ WG11 session 14d — set for T6 (after `wg11-renderer-product-2-patch.py`, whose GraphTimeline law this proves too): an
undecodable image is a typed per-image refusal that never blocks the raster lane (coordinator 09:0x, open design item 2).

Measured (overlay build 4, `graph_timeline_paints_every_author_and_never_reuses_a_replaced_avatar_source`, `rasters=0`): a source
whose dimensions or decode fail rejected its raster reservation into the surface's ONE `rejected` slot. That slot (a) closed
admission for every later image on the surface until the frame's upload cursor retired it, and (b) retired as
`PendingRasterUploadStep::Fault`, which faults the WHOLE frame transaction (`record_frame_fault`). The ui-image resolver re-offered
the same bad `data:` URL on every paint, so one broken avatar faulted every frame and starved every image after it. A saturated
process ledger (pure back-pressure) took the same fault path.

1. `ui::prepared::PreparedRasterRejected` is classified: `is_content_refusal` (the source's own — undecodable, oversized) versus an
   authority fault; `try_reserve_source` marks an oversized source, `into_content_refusal` re-classifies a measured refusal.
2. Scenes: `queue_canvas_image_upload_with` answers `Result<String, RasterUploadRefusal>` — `Busy` (back-pressure, offer again) or
   `Invalid(reason)` (the image's own refusal). Every refused owner parks in a fixed per-surface refusal ring
   (`RASTER_REFUSALS_PER_SURFACE`) that admission keeps room for and the upload cursor (and the realm close) retires silently —
   only an authority fault (a reservation abandoned mid-admission, a finalize refusal) still faults the frame.
3. Interpreter: the ui-image resolver keeps an `Invalid` source as that image's refusal (`UI_IMAGE_REFUSED`, id → source) and never
   re-offers it; a fetched image whose upload is merely `Busy` is fetched again instead of being recorded as a miss.
Laws: a refusal never blocks the next image and never faults the upload cursor (raster frame cost); ledger saturation is `Busy` and
retires without a fault (render-plan validator); the GraphTimeline avatar law (`rasters == 1` after an invalid paint).

Dry run by default; `--write` backs every edited file up under `.🧬semio/🌐hub/s14-wg11-backup/raster-refusal/` and applies;
`--revert` restores.
"""

import difflib
import shutil
import sys
from pathlib import Path

ROOT = Path("/Users/ueli/Documents/semio")
WORK = Path("/Users/ueli" + "/Documents/semio/.tmp-ticket/wp-wg11/raster")
PREPARED = ROOT / "🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🎟️prepared/🦀️.rs"
ELEMENTS = ROOT / "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements"
SCENES = ELEMENTS / "🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs"
RASTER_LAWS = ELEMENTS / "🎞️Scenes/🧪️tests/🔬️wgpu-raster-frame-cost/🦀️.rs"
INTERPRETER = ELEMENTS / "🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs"
VALIDATOR_LAWS = ELEMENTS / "🗣️Interpreter/🧪️tests/🔬️wgpu-render-plan-validator/🦀️.rs"
BACKUP = ROOT / ".🧬semio/🌐hub/s14-wg11-backup/raster-refusal"

UPLOAD_WITH_START = "pub(crate) fn queue_canvas_image_upload_with("
UPLOAD_WITH_END = "/** 🖼️ Reserves first, then decodes one encoded Canvas image backing exactly once. */\n"

PREPARED_EDITS = [
    (
        '''pub struct PreparedRasterRejected {
    fault: &'static str,''',
        '''pub struct PreparedRasterRejected {
    fault: &'static str,
    /// 🖼️ Whether the refusal is the SOURCE's own ([`Self::is_content_refusal`]).
    content: bool,''',
    ),
    (
        '''impl PreparedRasterRejected {
    pub fn fault(&self) -> &'static str {
        self.fault
    }
''',
        '''impl PreparedRasterRejected {
    pub fn fault(&self) -> &'static str {
        self.fault
    }

    /// 🖼️ Whether this refusal is the source's own — undecodable or oversized, a typed per-image outcome its host retires
    /// silently — rather than a raster-authority fault.
    pub fn is_content_refusal(&self) -> bool {
        self.content
    }

    /// 🖼️ This refusal classified as the source's own ([`Self::is_content_refusal`]) — what a host does with a refusal its
    /// measured source caused.
    pub fn into_content_refusal(mut self) -> Self {
        self.content = true;
        self
    }
''',
    ),
    (
        '''        let reject = |fault, key| PreparedRasterRejected { fault, key, source: Vec::new(), retained_source: Vec::new(), credit: None, source_released: false, retained_source_released: false, key_released: false };''',
        '''        let reject = |fault, key| PreparedRasterRejected { fault, content: false, key, source: Vec::new(), retained_source: Vec::new(), credit: None, source_released: false, retained_source_released: false, key_released: false };''',
    ),
    (
        '''        if source_bytes > PREPARED_RASTER_ITEM_BYTES {
            return Err(reject("raster producer source exceeded fixed credits", key));
        }''',
        '''        if source_bytes > PREPARED_RASTER_ITEM_BYTES {
            return Err(reject("raster producer source exceeded fixed credits", key).into_content_refusal());
        }''',
    ),
    (
        '''        PreparedRasterRejected { fault, key: std::mem::take(&mut self.key), source, retained_source, credit: self.credit.take(), source_released: false, retained_source_released: false, key_released: false }''',
        '''        PreparedRasterRejected { fault, content: false, key: std::mem::take(&mut self.key), source, retained_source, credit: self.credit.take(), source_released: false, retained_source_released: false, key_released: false }''',
    ),
]

SCENES_EDITS = [
    (
        '''const RASTER_UPLOAD_BYTE_CAPACITY: usize = 1024 * 1024;
''',
        '''const RASTER_UPLOAD_BYTE_CAPACITY: usize = 1024 * 1024;
/// 🖼️ Per-image refusals one surface parks for silent retirement between two upload-cursor passes; admission keeps room for one.
const RASTER_REFUSALS_PER_SURFACE: usize = 4;

/// 🖼️ Why one image was not queued for upload ([`queue_canvas_image_upload_with`]).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum RasterUploadRefusal {
    /// ⏳️ Back-pressure — no free slot, the process ledger at capacity, an owner still retiring; offer the source again later.
    Busy,
    /// 🚫️ The source's own refusal (undecodable, oversized) — a per-image outcome to keep, never to re-offer.
    Invalid(&'static str),
}
''',
    ),
    (
        '''#[derive(Default)]
struct PendingRasterSurface {
    queue: PendingRasterQueue,
    admission: Option<PreparedRasterReservation>,
    rejected: Option<PreparedRasterRejected>,
    retiring: Option<PreparedRasterRejected>,
    closing: Option<PreparedRasterProducer>,
}
''',
        '''#[derive(Default)]
struct PendingRasterSurface {
    queue: PendingRasterQueue,
    admission: Option<PreparedRasterReservation>,
    rejected: Option<PreparedRasterRejected>,
    retiring: Option<PreparedRasterRejected>,
    closing: Option<PreparedRasterProducer>,
    /// 🖼️ Per-image refusals awaiting silent bounded retirement ([`PendingRasterSurface::park_refusal`]).
    refused: [Option<PreparedRasterRejected>; RASTER_REFUSALS_PER_SURFACE],
}

impl PendingRasterSurface {
    /// 🚦️ Whether one more image may be admitted: a free FIFO slot, no admission in flight, no authority owner retained or
    /// retiring, and room in the refusal ring for the refusal the admission could end in.
    fn admits_upload(&self) -> bool {
        !self.queue.is_full() && self.admission.is_none() && self.rejected.is_none() && self.retiring.is_none() && self.closing.is_none() && self.refused.iter().any(Option::is_none)
    }

    /// 🖼️ Parks one refused owner for silent retirement and answers the typed outcome; admission reserved the ring slot (a full
    /// ring falls back to the authority slot).
    fn park_refusal(&mut self, refusal: PreparedRasterRejected) -> RasterUploadRefusal {
        let outcome = if refusal.is_content_refusal() { RasterUploadRefusal::Invalid(refusal.fault()) } else { RasterUploadRefusal::Busy };
        match self.refused.iter_mut().find(|slot| slot.is_none()) {
            Some(slot) => *slot = Some(refusal),
            None => self.rejected = Some(refusal),
        }
        outcome
    }

    /// 🖼️ One bounded retirement step of the first parked refusal; `false` when the ring is empty.
    fn retire_refusal_step(&mut self) -> bool {
        let Some(slot) = self.refused.iter_mut().find(|slot| slot.is_some()) else { return false };
        if slot.as_mut().is_some_and(PreparedRasterRejected::close_step) {
            debug_assert!(slot.as_ref().is_some_and(PreparedRasterRejected::terminal_is_empty), "a retired refusal is terminal-empty");
            *slot = None;
        }
        true
    }
}
''',
    ),
    (
        '''            if let Some(reservation) = state.admission.take() {
                state.rejected = Some(reservation.reject("raster reservation was abandoned before publication", Vec::new()));
                return PendingRasterUploadStep::Pending;
            }''',
        '''            if let Some(reservation) = state.admission.take() {
                state.rejected = Some(reservation.reject("raster reservation was abandoned before publication", Vec::new()));
                return PendingRasterUploadStep::Pending;
            }
            if state.retire_refusal_step() {
                return PendingRasterUploadStep::Pending;
            }''',
    ),
    (
        '''        if let Some(rejected) = self.surface.rejected.as_mut() {
            if !rejected.close_step() {
                return false;
            }
            assert!(rejected.terminal_is_empty(), "realm-rejected raster reservation must be terminal-empty");''',
        '''        if self.surface.retire_refusal_step() {
            return false;
        }
        if let Some(rejected) = self.surface.rejected.as_mut() {
            if !rejected.close_step() {
                return false;
            }
            assert!(rejected.terminal_is_empty(), "realm-rejected raster reservation must be terminal-empty");''',
    ),
    (
        '''            && self.surface.rejected.is_none()
            && self.surface.closing.is_none()''',
        '''            && self.surface.rejected.is_none()
            && self.surface.refused.iter().all(Option::is_none)
            && self.surface.closing.is_none()''',
    ),
    (
        '''        surfaces
            .get_or_insert_with(surface_id.to_string(), PendingRasterSurface::default)
            .is_some_and(|surface| !surface.queue.is_full() && surface.admission.is_none() && surface.rejected.is_none() && surface.retiring.is_none() && surface.closing.is_none())''',
        '''        surfaces.get_or_insert_with(surface_id.to_string(), PendingRasterSurface::default).is_some_and(|surface| surface.admits_upload())''',
    ),
    (
        '''pub(crate) fn queue_canvas_image_upload_sized(surface_id: &str, layer_id: &str, data_url: &str) -> (Option<String>, Option<(u32, u32)>) {''',
        '''pub(crate) fn queue_canvas_image_upload_sized(surface_id: &str, layer_id: &str, data_url: &str) -> (Result<String, RasterUploadRefusal>, Option<(u32, u32)>) {''',
    ),
    (
        '''pub(crate) fn queue_canvas_image_upload(surface_id: &str, layer_id: &str, data_url: &str) -> Option<String> {
    queue_canvas_image_upload_sized(surface_id, layer_id, data_url).0
}''',
        '''pub(crate) fn queue_canvas_image_upload(surface_id: &str, layer_id: &str, data_url: &str) -> Option<String> {
    queue_canvas_image_upload_sized(surface_id, layer_id, data_url).0.ok()
}''',
    ),
]

RASTER_LAW_EDITS = [
    (
        '''            if let Some(mut rejected) = surface.rejected.take() {
                while !rejected.close_step() {}
            }''',
        '''            if let Some(mut rejected) = surface.rejected.take() {
                while !rejected.close_step() {}
            }
            while surface.retire_refusal_step() {}''',
    ),
    (
        '''    assert!(result.is_none());
    assert!(!dimensions_called.get());''',
        '''    assert_eq!(result, Err(RasterUploadRefusal::Busy), "a saturated ledger is back-pressure, not the image's refusal");
    assert!(!dimensions_called.get());''',
    ),
    (
        '''#[test]
fn realm_close_retires_pending_rasters_before_terminal_and_allows_clean_reopen_fixture() {''',
        '''/// 🖼️ LAW (ticket 26/09/23 session 14d, WG11): an undecodable source is a TYPED per-image refusal — it never blocks the next image
/// on the same surface, and the frame's upload cursor retires it without faulting the frame.
#[test]
fn an_undecodable_source_is_a_typed_refusal_that_never_blocks_the_next_image() {
    let mut authority = begin_pending_raster_authority_close();
    while !authority.close_step() {}
    reset_pending_raster_authority();
    let surface_id = "raster-undecodable-source";
    let refused = queue_canvas_image_upload_sized(surface_id, "broken", "data:image/png;base64,AAAA").0;
    assert!(matches!(refused, Err(RasterUploadRefusal::Invalid(_))), "an undecodable source is the image's own typed refusal: {refused:?}");
    assert!(queue_canvas_image_upload(surface_id, "valid", &tiny_png_data_url(4, 5, 6)).is_some(), "the refusal does not block the next image on the same surface");
    let mut cursor = PendingRasterUploadCursor::default();
    let mut uploads = 0;
    for _ in 0..65_536 {
        match cursor.step() {
            PendingRasterUploadStep::Pending => {}
            PendingRasterUploadStep::Upload(checked) => {
                let mut producer = checked.take().unwrap_or_else(|_| panic!("the valid image uploads"));
                producer.begin_close();
                while !producer.close_step() {}
                uploads += 1;
            }
            PendingRasterUploadStep::Complete => break,
            PendingRasterUploadStep::Fault(fault) => panic!("a per-image refusal never faults the frame: {fault}"),
        }
    }
    assert_eq!(uploads, 1, "exactly the valid image uploads");
    assert!(PENDING_RASTER_STATE.with(|cell| cell.borrow().get(surface_id).is_some_and(|surface| surface.refused.iter().all(Option::is_none))), "the refusal retired within the pass");
}

#[test]
fn realm_close_retires_pending_rasters_before_terminal_and_allows_clean_reopen_fixture() {''',
    ),
]

INTERPRETER_EDITS = [
    (
        '''use crate::scenes::{queue_canvas_image_upload_sized, queue_canvas_image_upload_with, render_component_scene_step};''',
        '''use crate::scenes::{queue_canvas_image_upload_sized, queue_canvas_image_upload_with, render_component_scene_step, RasterUploadRefusal};''',
    ),
    (
        '''static UI_IMAGE_SIZES: WorkerCell<std::collections::HashMap<String, (u32, u32)>> = WorkerCell::new();
''',
        '''static UI_IMAGE_SIZES: WorkerCell<std::collections::HashMap<String, (u32, u32)>> = WorkerCell::new();
/// 🚫️ Per image id, the inline source the raster authority refused as the image's own ([`RasterUploadRefusal::Invalid`]) — kept, not
/// re-offered each paint; the image paints its fallback until its source changes.
static UI_IMAGE_REFUSED: WorkerCell<std::collections::HashMap<String, String>> = WorkerCell::new();
''',
    ),
    (
        '''    let key = queue_canvas_image_upload_with("ui-image", id, bytes, dimensions, decode);
    let Some(key) = key else {
        UI_IMAGE_FETCH_MISS.with(|cell| {
            cell.borrow_mut().insert(id.to_string(), url.to_string());
        });
        return;
    };''',
        '''    let key = match queue_canvas_image_upload_with("ui-image", id, bytes, dimensions, decode) {
        Ok(key) => key,
        Err(RasterUploadRefusal::Invalid(_)) => {
            UI_IMAGE_FETCH_MISS.with(|cell| {
                cell.borrow_mut().insert(id.to_string(), url.to_string());
            });
            return;
        }
        Err(RasterUploadRefusal::Busy) => return,
    };''',
    ),
    (
        '''fn resolve_ui_image_svg(id: &str, src: &str) -> (Option<String>, Option<(u32, u32)>) {''',
        '''fn resolve_ui_image_svg(id: &str, src: &str) -> (Result<String, RasterUploadRefusal>, Option<(u32, u32)>) {''',
    ),
    (
        '''    let current = UI_IMAGE_LAST_URL.with(|cell| cell.borrow().get(id).is_some_and(|last| last == src));
    if current {
        return;
    }
    let (key, size) = if src.starts_with("data:image/svg+xml") { resolve_ui_image_svg(id, src) } else { queue_canvas_image_upload_sized("ui-image", id, src) };
    let (Some(key), Some((width, height))) = (key, size) else { return };''',
        '''    let current = UI_IMAGE_LAST_URL.with(|cell| cell.borrow().get(id).is_some_and(|last| last == src));
    if current || UI_IMAGE_REFUSED.with(|cell| cell.borrow().get(id).is_some_and(|refused| refused == src)) {
        return;
    }
    let (key, size) = if src.starts_with("data:image/svg+xml") { resolve_ui_image_svg(id, src) } else { queue_canvas_image_upload_sized("ui-image", id, src) };
    let key = match key {
        Ok(key) => key,
        Err(RasterUploadRefusal::Invalid(_)) => {
            UI_IMAGE_REFUSED.with(|cell| {
                cell.borrow_mut().insert(id.to_string(), src.to_string());
            });
            return;
        }
        Err(RasterUploadRefusal::Busy) => return,
    };
    let Some((width, height)) = size else { return };
    UI_IMAGE_REFUSED.with(|cell| {
        cell.borrow_mut().remove(id);
    });''',
    ),
    (
        '''    if src.starts_with("data:image/svg+xml") {
        return resolve_ui_image_svg(id, src);
    }
    if src.starts_with("data:") {
        return queue_canvas_image_upload_sized("ui-image", id, src);
    }
    resolve_ui_image_url(id, src)''',
        '''    if src.starts_with("data:image/svg+xml") {
        let (key, size) = resolve_ui_image_svg(id, src);
        return (key.ok(), size);
    }
    if src.starts_with("data:") {
        let (key, size) = queue_canvas_image_upload_sized("ui-image", id, src);
        return (key.ok(), size);
    }
    resolve_ui_image_url(id, src)''',
    ),
]

VALIDATOR_LAW_EDITS = [
    (
        '''    let mut cursor = crate::scenes::PendingRasterUploadCursor::default();
    while !matches!(cursor.step(), crate::scenes::PendingRasterUploadStep::Fault(_)) {}
}''',
        '''    let mut cursor = crate::scenes::PendingRasterUploadCursor::default();
    for _ in 0..65_536 {
        match cursor.step() {
            crate::scenes::PendingRasterUploadStep::Complete => return,
            crate::scenes::PendingRasterUploadStep::Fault(fault) => panic!("back-pressure retires without faulting the frame: {fault}"),
            _ => {}
        }
    }
    panic!("the upload cursor retires the parked back-pressure refusal within its bounded pass");
}''',
    )
]


def replaced(path: Path, source: str, edits) -> str:
    for old, new in edits:
        count = source.count(old)
        if count != 1:
            sys.exit(f"anchor occurs {count}x in {path.parent.name}/{path.name}: {old[:100]!r}")
        source = source.replace(old, new)
    return source


def scenes_after(source: str) -> str:
    start = source.find(UPLOAD_WITH_START)
    end = source.find(UPLOAD_WITH_END, start)
    if start < 0 or end < 0 or source.count(UPLOAD_WITH_START) != 1:
        sys.exit("Scenes: queue_canvas_image_upload_with markers moved")
    body = source[start:end]
    if body.count("return None") != 11 or "surface.rejected = Some(reservation.reject_with_retained(\"raster source dimensions failed\"" not in body:
        sys.exit("Scenes: queue_canvas_image_upload_with changed since the set was written — re-measure")
    source = source[:start] + (WORK / "upload-with-new.rs").read_text(encoding="utf-8") + source[end + len(UPLOAD_WITH_END):]
    return replaced(SCENES, source, SCENES_EDITS)


EDITS = {PREPARED: PREPARED_EDITS, RASTER_LAWS: RASTER_LAW_EDITS, INTERPRETER: INTERPRETER_EDITS, VALIDATOR_LAWS: VALIDATOR_LAW_EDITS}


def main():
    files = [SCENES] + list(EDITS)
    if "--revert" in sys.argv:
        for path in files:
            backup = BACKUP / path.relative_to(ROOT)
            if not backup.exists():
                sys.exit(f"no backup for {path.relative_to(ROOT)}")
            shutil.copyfile(backup, path)
        print(f"REVERTED: {len(files)} files restored from backups")
        return
    write = "--write" in sys.argv
    scenes = SCENES.read_text(encoding="utf-8")
    planned = [(SCENES, scenes, scenes_after(scenes))]
    for path, edits in EDITS.items():
        source = path.read_text(encoding="utf-8")
        planned.append((path, source, replaced(path, source, edits)))
    for path, before, after in planned:
        sys.stdout.writelines(difflib.unified_diff(before.splitlines(True), after.splitlines(True), f"{path.parent.name}/{path.name}", f"{path.parent.name}/{path.name} (patched)", n=1))
    if write:
        for path, before, _ in planned:
            backup = BACKUP / path.relative_to(ROOT)
            backup.parent.mkdir(parents=True, exist_ok=True)
            backup.write_bytes(before.encode("utf-8"))
        for path, _, after in planned:
            path.write_text(after, encoding="utf-8")
    print(f"\n{'WRITTEN' if write else 'DRY RUN'}: {len(planned)} files (crates: semio-framework-ui, semio-framework-os-renderer-wgpu)")


if __name__ == "__main__":
    main()
