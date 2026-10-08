//! 🔺️ Sparse diff builder for `PinPixel` — one id-keyed upsert into `pinned`, inserted at the
//! CANONICAL row-major position rather than appended, so `unpin-pixel` removes exactly what this
//! inserted (the point-invertibility law).

use crate::diff::{BitmapDiff, BitmapPinnedPatch, BitmapRow, BitmapPinnedDelta, BitmapPinnedModification};
use crate::schema::snapshot::{pin_index, pin_key, BitmapPinnedPixel, BitmapSnapshot};

pub fn diff(payload: &super::PinPixel, base: &BitmapSnapshot) -> protocol::MutationOutcome<BitmapDiff> {
    if payload.x >= base.output.width || payload.y >= base.output.height {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("({}, {}) falls outside the {}×{} output.", payload.x, payload.y, base.output.width, base.output.height), [crate::schema::snapshot::pin_key(payload.x, payload.y)]);
    }
    if payload.color as usize >= base.input.palette.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Palette index {} is not in this document's palette.", payload.color), [crate::schema::snapshot::pin_key(payload.x, payload.y)]);
    }
    let pin = BitmapPinnedPixel { x: payload.x, y: payload.y, color: payload.color };
    match pin_index(base, payload.x, payload.y) {
        Some(at) if base.pinned[at] == pin => protocol::MutationOutcome::empty().warning("mutation.no-op", format!("({}, {}) is already pinned to colour {}.", payload.x, payload.y, payload.color)),
        Some(_) => protocol::MutationOutcome::new(BitmapDiff { pinned: BitmapPinnedDelta { modified: vec![BitmapPinnedModification { id: pin_key(payload.x, payload.y), patch: BitmapPinnedPatch { color: Some(payload.color) } }], ..Default::default() }, ..Default::default() }),
        None => protocol::MutationOutcome::new(BitmapDiff { pinned: BitmapPinnedDelta::insertion(BitmapRow::insert_at(&base.pinned, &pin), pin), ..Default::default() }),
    }
}
