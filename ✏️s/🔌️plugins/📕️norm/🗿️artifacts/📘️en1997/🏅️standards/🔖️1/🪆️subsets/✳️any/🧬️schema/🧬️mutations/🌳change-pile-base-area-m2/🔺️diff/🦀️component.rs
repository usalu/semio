//! 🔺️ `change-pile-base-area-m2` sparse diff construction — writes only `En1997Diff.pile_base_area_m2` from the payload.

use crate::artifacts::en1997::diff::En1997Diff;
use crate::artifacts::en1997::mutations::change_pile_base_area_m2::mutation::ChangePileBaseAreaM2;
use crate::artifacts::en1997::En1997Snapshot;

//#region 🔖️Diff
pub async fn diff(payload: &ChangePileBaseAreaM2, base: &En1997Snapshot) -> protocol::MutationOutcome<En1997Diff> {
    if !payload.new_pile_base_area_m2.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Pile base area [m2] must be a finite number, got {}.", payload.new_pile_base_area_m2), Vec::<String>::new());
    }
    if base.pile_base_area_m2 == payload.new_pile_base_area_m2 {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Pile base area [m2] is already {}.", payload.new_pile_base_area_m2));
    }
    protocol::MutationOutcome::new(En1997Diff { pile_base_area_m2: Some(payload.new_pile_base_area_m2.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
