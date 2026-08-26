//! 🔺️ `change-alpha-s` sparse diff construction — writes only `En1997Diff.alpha_s` from the payload.

use crate::artifacts::en1997::diff::En1997Diff;
use crate::artifacts::en1997::mutations::change_alpha_s::mutation::ChangeAlphaS;
use crate::artifacts::en1997::En1997Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeAlphaS, base: &En1997Snapshot) -> protocol::MutationOutcome<En1997Diff> {
    if !payload.new_alpha_s.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Shaft resistance factor alpha_s must be a finite number, got {}.", payload.new_alpha_s), Vec::<String>::new());
    }
    if base.alpha_s == payload.new_alpha_s {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Shaft resistance factor alpha_s is already {}.", payload.new_alpha_s));
    }
    protocol::MutationOutcome::new(En1997Diff { alpha_s: Some(payload.new_alpha_s.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
