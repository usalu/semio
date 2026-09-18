//! 🔺️ Sparse diff builder for `ResizeOutput` — the output lane plus the pin rows a shrink strands.

use crate::diff::BitmapDiff;
use crate::schema::snapshot::{pin_key, BitmapOutputSpec, BitmapSnapshot, BITMAP_MAX_EDGE};

pub fn diff(payload: &super::ResizeOutput, base: &BitmapSnapshot) -> protocol::MutationOutcome<BitmapDiff> {
    if payload.width == 0 || payload.height == 0 {
        return protocol::MutationOutcome::fatal("mutation.invariant", "An output bitmap may not have a zero edge.".to_string(), ["output".to_string()]);
    }
    if payload.width > BITMAP_MAX_EDGE || payload.height > BITMAP_MAX_EDGE {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("An output edge may not exceed {BITMAP_MAX_EDGE}."), ["output".to_string()]);
    }
    let output = BitmapOutputSpec { width: payload.width, height: payload.height, periodic: payload.periodic };
    if base.output == output {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("The output is already {}×{}.", payload.width, payload.height));
    }
    let pinned_removed: Vec<String> = base.pinned.iter().filter(|pin| pin.x >= payload.width || pin.y >= payload.height).map(|pin| pin_key(pin.x, pin.y)).collect();
    let cascaded = pinned_removed.len();
    let outcome = protocol::MutationOutcome::new(BitmapDiff { output: Some(output), pinned_removed, ..Default::default() });
    if cascaded == 0 {
        outcome
    } else {
        outcome.info("mutation.cascade", format!("{cascaded} pinned pixel(s) fell outside the new output and were removed."))
    }
}
