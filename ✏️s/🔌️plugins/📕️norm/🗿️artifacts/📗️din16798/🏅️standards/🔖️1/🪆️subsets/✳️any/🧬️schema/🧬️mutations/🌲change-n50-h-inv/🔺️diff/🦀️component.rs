//! 🔺️ `change-n50-h-inv` sparse diff construction — writes only `Din16798Diff.n50_h_inv` from the payload.

use crate::artifacts::din16798::diff::Din16798Diff;
use crate::artifacts::din16798::mutations::change_n50_h_inv::mutation::ChangeN50HInv;
use crate::artifacts::din16798::Din16798Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeN50HInv, base: &Din16798Snapshot) -> protocol::MutationOutcome<Din16798Diff> {
    if !payload.new_n50_h_inv.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("N50 air change rate must be a finite number, got {}.", payload.new_n50_h_inv), Vec::<String>::new());
    }
    if base.n50_h_inv == payload.new_n50_h_inv {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("N50 air change rate is already {}.", payload.new_n50_h_inv));
    }
    protocol::MutationOutcome::new(Din16798Diff { n50_h_inv: Some(payload.new_n50_h_inv.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
