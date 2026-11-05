//! 🔺️ Exact restoration keeps the previous pin sequence rather than sorting it again.
use crate::diff::BitmapDiff;
use crate::schema::snapshot::BitmapSnapshot;

pub fn diff(payload: &super::RestorePins, base: &BitmapSnapshot) -> protocol::MutationOutcome<BitmapDiff> {
    crate::schema::mutations::pin_solution::diff::replacement_diff(&payload.pins, base, false)
}
