//! 🔺️ Sparse diff builder for `ResizeInput` — the two extent fields AND the re-laid-out buffer.
//!
//! Carrying the buffer is deliberate. A pad/crop changes every pixel's position, so a diff that
//! declared only the extent would leave a reader to re-derive the layout rule before it could
//! reconstruct `after` — and the footprint law a fixture asserts ("every field that differs is
//! declared") would not hold at the field level. The buffer is bounded by the same ceiling the
//! document itself is, and a resize is not a per-frame gesture.

use crate::diff::{BitmapDiff, BitmapInputOp};
use crate::schema::snapshot::{BitmapSnapshot, BITMAP_MAX_EDGE};

pub fn diff(payload: &super::ResizeInput, base: &BitmapSnapshot) -> protocol::MutationOutcome<BitmapDiff> {
    if payload.width == 0 || payload.height == 0 {
        return protocol::MutationOutcome::fatal("mutation.invariant", "An input bitmap may not have a zero edge.".to_string(), ["input".to_string()]);
    }
    if payload.width > BITMAP_MAX_EDGE || payload.height > BITMAP_MAX_EDGE {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("An input edge may not exceed {BITMAP_MAX_EDGE}."), ["input".to_string()]);
    }
    if base.input.width == payload.width && base.input.height == payload.height {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("The input is already {}×{}.", payload.width, payload.height));
    }
    if base.input.indices().is_none() {
        return protocol::MutationOutcome::fatal("mutation.apply.invalid-base", "The base input pixel buffer does not decode.".to_string(), ["input".to_string()]);
    }
    protocol::MutationOutcome::new(BitmapDiff { input_ops: vec![BitmapInputOp::Resize { width: payload.width, height: payload.height }], ..Default::default() })
}
