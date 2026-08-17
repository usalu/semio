//! 🔺️ `change-fatigue-m` sparse diff construction — writes only `En1999Diff.fatigue_m` from the payload.

use crate::artifacts::en1999::diff::En1999Diff;
use crate::artifacts::en1999::mutations::change_fatigue_m::mutation::ChangeFatigueM;
use crate::artifacts::en1999::En1999Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeFatigueM, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
    if !payload.new_fatigue_m.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Fatigue S-N slope m must be a finite number, got {}.", payload.new_fatigue_m), Vec::<String>::new());
    }
    if base.fatigue_m == payload.new_fatigue_m {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Fatigue S-N slope m is already {}.", payload.new_fatigue_m));
    }
    protocol::MutationOutcome::new(En1999Diff { fatigue_m: Some(payload.new_fatigue_m.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
