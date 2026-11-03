//! 🔺️ `change-footing-settlement-limit` diff: patches `settlement_limit` of the addressed footing row.

use crate::change_footing_settlement_limit::ChangeFootingSettlementLimit;
use semio_s_artifact_norm_en1997_diff::{En1997Diff, En1997FootingsPatch, En1997FootingsRows};
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn diff(payload: &ChangeFootingSettlementLimit, base: &En1997Snapshot) -> protocol::MutationOutcome<En1997Diff> {
    if !payload.new_settlement_limit.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.footings.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("footing {} missing", payload.id), vec![payload.id.clone()]);
    };
    if r0.settlement_limit == payload.new_settlement_limit {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The footing already has this value.");
    }
    protocol::MutationOutcome::new(En1997Diff { footings: Some(En1997FootingsRows::modification(&payload.id, En1997FootingsPatch { settlement_limit: Some(payload.new_settlement_limit), ..Default::default() })), ..Default::default() })
}
