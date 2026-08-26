//! 🔺️ `change-reference-qp-kwh` sparse diff construction — writes only `Din18599Diff.reference_q_p_kwh` from the payload.

use crate::artifacts::din18599::diff::Din18599Diff;
use crate::artifacts::din18599::mutations::change_reference_q_p_kwh::mutation::ChangeReferenceQPKwh;
use crate::artifacts::din18599::Din18599Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeReferenceQPKwh, base: &Din18599Snapshot) -> protocol::MutationOutcome<Din18599Diff> {
    if !payload.new_reference_q_p_kwh.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "Reference qp kwh must be a finite number.", Vec::<String>::new());
    }
    if base.reference_q_p_kwh == payload.new_reference_q_p_kwh {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Reference qp kwh already has this value.");
    }
    protocol::MutationOutcome::new(Din18599Diff { reference_q_p_kwh: Some(payload.new_reference_q_p_kwh.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
