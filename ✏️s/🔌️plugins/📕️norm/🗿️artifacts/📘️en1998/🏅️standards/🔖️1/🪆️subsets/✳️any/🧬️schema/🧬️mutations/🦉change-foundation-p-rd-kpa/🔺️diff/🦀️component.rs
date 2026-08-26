//! 🔺️ `change-foundation-p-rd-kpa` sparse diff construction — writes only `En1998Diff.foundation_p_rd_kpa` from the payload.

use crate::artifacts::en1998::diff::En1998Diff;
use crate::artifacts::en1998::mutations::change_foundation_p_rd_kpa::mutation::ChangeFoundationPRdKpa;
use crate::artifacts::en1998::En1998Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeFoundationPRdKpa, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    if !payload.new_foundation_p_rd_kpa.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Foundation bearing resistance p_Rd [kPa] must be a finite number, got {}.", payload.new_foundation_p_rd_kpa), Vec::<String>::new());
    }
    if base.foundation_p_rd_kpa == payload.new_foundation_p_rd_kpa {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Foundation bearing resistance p_Rd [kPa] is already {}.", payload.new_foundation_p_rd_kpa));
    }
    protocol::MutationOutcome::new(En1998Diff { foundation_p_rd_kpa: Some(payload.new_foundation_p_rd_kpa.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
