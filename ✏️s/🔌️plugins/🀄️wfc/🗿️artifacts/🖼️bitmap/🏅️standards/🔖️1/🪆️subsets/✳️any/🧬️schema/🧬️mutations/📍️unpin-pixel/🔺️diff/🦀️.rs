//! 🔺️ Sparse diff builder for `UnpinPixel` — one id-keyed removal from `pinned`.

use crate::diff::BitmapDiff;
use crate::schema::snapshot::{pin_index, pin_key, BitmapSnapshot};

pub fn diff(payload: &super::UnpinPixel, base: &BitmapSnapshot) -> protocol::MutationOutcome<BitmapDiff> {
    let key = pin_key(payload.x, payload.y);
    if pin_index(base, payload.x, payload.y).is_none() {
        return protocol::MutationOutcome::fatal("mutation.missing-target", format!("({}, {}) carries no pin.", payload.x, payload.y), [key]);
    }
    protocol::MutationOutcome::new(BitmapDiff { pinned_removed: vec![key], ..Default::default() })
}
