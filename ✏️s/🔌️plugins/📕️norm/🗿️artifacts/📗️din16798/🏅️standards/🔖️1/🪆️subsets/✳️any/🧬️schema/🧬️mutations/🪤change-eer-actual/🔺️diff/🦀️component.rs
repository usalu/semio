//! 🔺️ `change-eer-actual` sparse diff construction — writes only `Din16798Diff.eer_actual` from the payload.

use crate::artifacts::din16798::diff::Din16798Diff;
use crate::artifacts::din16798::mutations::change_eer_actual::mutation::ChangeEerActual;
use crate::artifacts::din16798::Din16798Snapshot;

//#region 🔖️Diff
pub async fn diff(payload: &ChangeEerActual, base: &Din16798Snapshot) -> protocol::MutationOutcome<Din16798Diff> {
    if !payload.new_eer_actual.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Actual EER must be a finite number, got {}.", payload.new_eer_actual), Vec::<String>::new());
    }
    if base.eer_actual == payload.new_eer_actual {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Actual EER is already {}.", payload.new_eer_actual));
    }
    protocol::MutationOutcome::new(Din16798Diff { eer_actual: Some(payload.new_eer_actual.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
