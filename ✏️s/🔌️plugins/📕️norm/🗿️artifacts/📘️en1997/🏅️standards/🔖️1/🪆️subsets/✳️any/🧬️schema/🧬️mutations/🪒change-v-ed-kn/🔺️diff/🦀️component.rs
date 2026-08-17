//! 🔺️ `change-v-ed-kn` sparse diff construction — writes only `En1997Diff.v_ed_kn` from the payload.

use crate::artifacts::en1997::diff::En1997Diff;
use crate::artifacts::en1997::mutations::change_v_ed_kn::mutation::ChangeVEdKn;
use crate::artifacts::en1997::En1997Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeVEdKn, base: &En1997Snapshot) -> protocol::MutationOutcome<En1997Diff> {
    if !payload.new_v_ed_kn.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Design vertical load V_Ed [kN] must be a finite number, got {}.", payload.new_v_ed_kn), Vec::<String>::new());
    }
    if base.v_ed_kn == payload.new_v_ed_kn {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Design vertical load V_Ed [kN] is already {}.", payload.new_v_ed_kn));
    }
    protocol::MutationOutcome::new(En1997Diff { v_ed_kn: Some(payload.new_v_ed_kn.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
