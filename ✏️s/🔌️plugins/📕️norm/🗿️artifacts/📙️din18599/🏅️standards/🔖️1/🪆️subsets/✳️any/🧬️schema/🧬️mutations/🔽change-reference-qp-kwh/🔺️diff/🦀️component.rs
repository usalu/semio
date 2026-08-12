//! 🔺️ `change-reference-qp-kwh` sparse diff construction — writes only `Din18599Diff.reference_q_p_kwh` from the payload.

use crate::artifacts::din18599::diff::Din18599Diff;
use crate::artifacts::din18599::mutations::change_reference_q_p_kwh::mutation::ChangeReferenceQPKwh;
use crate::artifacts::din18599::Din18599Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeReferenceQPKwh, _base: &Din18599Snapshot) -> Din18599Diff {
    Din18599Diff { reference_q_p_kwh: Some(payload.new_reference_q_p_kwh.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
