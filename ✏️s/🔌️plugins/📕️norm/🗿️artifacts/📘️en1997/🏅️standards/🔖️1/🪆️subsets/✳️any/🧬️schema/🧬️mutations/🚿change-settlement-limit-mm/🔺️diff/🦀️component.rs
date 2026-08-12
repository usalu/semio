//! 🔺️ `change-settlement-limit-mm` sparse diff construction — writes only `En1997Diff.settlement_limit_mm` from the payload.

use crate::artifacts::en1997::diff::En1997Diff;
use crate::artifacts::en1997::mutations::change_settlement_limit_mm::mutation::ChangeSettlementLimitMm;
use crate::artifacts::en1997::En1997Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeSettlementLimitMm, _base: &En1997Snapshot) -> En1997Diff {
    En1997Diff { settlement_limit_mm: Some(payload.new_settlement_limit_mm.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
