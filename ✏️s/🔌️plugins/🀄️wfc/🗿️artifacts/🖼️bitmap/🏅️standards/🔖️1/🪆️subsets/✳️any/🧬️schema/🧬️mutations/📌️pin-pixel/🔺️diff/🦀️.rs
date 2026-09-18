//! 🔺️ Sparse diff builder for `PinPixel` — one id-keyed upsert into `pinned`, inserted at the
//! CANONICAL row-major position rather than appended, so `unpin-pixel` removes exactly what this
//! inserted (the point-invertibility law).

use crate::diff::BitmapDiff;
use crate::schema::snapshot::{ordered_pin_index, pin_index, BitmapPinnedPixel, BitmapSnapshot};

pub fn diff(payload: &super::PinPixel, base: &BitmapSnapshot) -> protocol::MutationOutcome<BitmapDiff> {
    if payload.x >= base.output.width || payload.y >= base.output.height {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("({}, {}) falls outside the {}×{} output.", payload.x, payload.y, base.output.width, base.output.height), [crate::schema::snapshot::pin_key(payload.x, payload.y)]);
    }
    if payload.color as usize >= base.input.palette.len() {
        return protocol::MutationOutcome::fatal("mutation.unknown-palette-color", format!("Palette index {} is not in this document's palette.", payload.color), [crate::schema::snapshot::pin_key(payload.x, payload.y)]);
    }
    let pin = BitmapPinnedPixel { x: payload.x, y: payload.y, color: payload.color };
    match pin_index(base, payload.x, payload.y) {
        Some(at) if base.pinned[at] == pin => protocol::MutationOutcome::empty().warn("mutation.no-op", format!("({}, {}) is already pinned to colour {}.", payload.x, payload.y, payload.color)),
        Some(at) => protocol::MutationOutcome::new(BitmapDiff { pinned_upserted: vec![(at, pin)], ..Default::default() }),
        None => protocol::MutationOutcome::new(BitmapDiff { pinned_upserted: vec![(ordered_pin_index(&base.pinned, payload.x, payload.y), pin)], ..Default::default() }),
    }
}
