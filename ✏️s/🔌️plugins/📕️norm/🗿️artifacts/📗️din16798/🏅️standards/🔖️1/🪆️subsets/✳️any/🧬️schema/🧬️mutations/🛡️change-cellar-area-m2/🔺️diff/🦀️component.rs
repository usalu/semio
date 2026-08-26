//! 🔺️ `change-cellar-area-m2` sparse diff construction — writes only `Din16798Diff.cellar_area_m2` from the payload.

use crate::artifacts::din16798::diff::Din16798Diff;
use crate::artifacts::din16798::mutations::change_cellar_area_m2::mutation::ChangeCellarAreaM2;
use crate::artifacts::din16798::Din16798Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeCellarAreaM2, base: &Din16798Snapshot) -> protocol::MutationOutcome<Din16798Diff> {
    if !payload.new_cellar_area_m2.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Cellar area must be a finite number, got {}.", payload.new_cellar_area_m2), Vec::<String>::new());
    }
    if base.cellar_area_m2 == payload.new_cellar_area_m2 {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Cellar area is already {}.", payload.new_cellar_area_m2));
    }
    protocol::MutationOutcome::new(Din16798Diff { cellar_area_m2: Some(payload.new_cellar_area_m2.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
