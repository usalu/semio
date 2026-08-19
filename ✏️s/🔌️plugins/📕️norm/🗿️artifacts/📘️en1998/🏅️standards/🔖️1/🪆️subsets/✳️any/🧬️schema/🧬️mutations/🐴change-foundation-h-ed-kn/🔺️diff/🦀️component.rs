//! 🔺️ `change-foundation-h-ed-kn` sparse diff construction — writes only `En1998Diff.foundation_h_ed_kn` from the payload.

use crate::artifacts::en1998::diff::En1998Diff;
use crate::artifacts::en1998::mutations::change_foundation_h_ed_kn::mutation::ChangeFoundationHEdKn;
use crate::artifacts::en1998::En1998Snapshot;

//#region 🔖️Diff
pub async fn diff(payload: &ChangeFoundationHEdKn, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    if !payload.new_foundation_h_ed_kn.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Foundation design horizontal force H_Ed [kN] must be a finite number, got {}.", payload.new_foundation_h_ed_kn), Vec::<String>::new());
    }
    if base.foundation_h_ed_kn == payload.new_foundation_h_ed_kn {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Foundation design horizontal force H_Ed [kN] is already {}.", payload.new_foundation_h_ed_kn));
    }
    protocol::MutationOutcome::new(En1998Diff { foundation_h_ed_kn: Some(payload.new_foundation_h_ed_kn.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
