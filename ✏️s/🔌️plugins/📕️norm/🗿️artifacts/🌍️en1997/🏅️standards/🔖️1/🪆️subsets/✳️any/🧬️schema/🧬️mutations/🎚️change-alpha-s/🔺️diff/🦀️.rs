//! 🔺️ `change-alpha-s` sparse diff construction — writes only `En1997Diff.alpha_s` from the payload.

use crate::diff::En1997Diff;
use crate::mutations::change_alpha_s::ChangeAlphaS;
use crate::En1997Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeAlphaS, base: &En1997Snapshot) -> protocol::MutationOutcome<En1997Diff> {
    if !payload.new_alpha_s.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Shaft resistance factor alpha_s must be a finite number, got {}.", payload.new_alpha_s), Vec::<String>::new());
    }
    if base.alpha_s == payload.new_alpha_s {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Shaft resistance factor alpha_s is already {}.", payload.new_alpha_s));
    }
    protocol::MutationOutcome::new(En1997Diff { alpha_s: Some(payload.new_alpha_s), ..Default::default() })
}
//#endregion 🔖️Diff
