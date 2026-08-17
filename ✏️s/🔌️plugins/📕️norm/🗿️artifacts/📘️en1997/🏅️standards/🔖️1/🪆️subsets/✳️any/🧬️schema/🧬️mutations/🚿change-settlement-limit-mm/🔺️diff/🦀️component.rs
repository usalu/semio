//! 🔺️ `change-settlement-limit-mm` sparse diff construction — writes only `En1997Diff.settlement_limit_mm` from the payload.

use crate::artifacts::en1997::diff::En1997Diff;
use crate::artifacts::en1997::mutations::change_settlement_limit_mm::mutation::ChangeSettlementLimitMm;
use crate::artifacts::en1997::En1997Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeSettlementLimitMm, base: &En1997Snapshot) -> protocol::MutationOutcome<En1997Diff> {
    if !payload.new_settlement_limit_mm.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Settlement limit [mm] must be a finite number, got {}.", payload.new_settlement_limit_mm), Vec::<String>::new());
    }
    if base.settlement_limit_mm == payload.new_settlement_limit_mm {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Settlement limit [mm] is already {}.", payload.new_settlement_limit_mm));
    }
    protocol::MutationOutcome::new(En1997Diff { settlement_limit_mm: Some(payload.new_settlement_limit_mm.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
