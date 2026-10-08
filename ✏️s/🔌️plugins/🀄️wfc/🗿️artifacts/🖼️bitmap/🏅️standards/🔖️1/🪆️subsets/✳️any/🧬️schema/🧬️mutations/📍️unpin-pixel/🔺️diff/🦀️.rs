//! 🔺️ Sparse diff builder for `UnpinPixel` — one id-keyed removal from `pinned`.

use crate::diff::{BitmapDiff, BitmapRows};
use crate::schema::snapshot::{pin_index, pin_key, BitmapSnapshot};

pub fn diff(payload: &super::UnpinPixel, base: &BitmapSnapshot) -> protocol::MutationOutcome<BitmapDiff> {
    let key = pin_key(payload.x, payload.y);
    if pin_index(base, payload.x, payload.y).is_none() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("({}, {}) carries no pin.", payload.x, payload.y), [key]);
    }
    protocol::MutationOutcome::new(BitmapDiff { pinned: BitmapRows { removed: vec![key], ..Default::default() }, ..Default::default() })
}
