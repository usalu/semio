//! 🔺️ `change-stock-label` sparse diff construction — a whole-`Stock` value with only `label`
//! replaced from `base`, never a snapshot clone. The document has exactly one stock (no target to
//! be missing); Warning `no-op` when the label is unchanged.

use crate::artifacts::process3d::diff::Process3dDiff;
use crate::artifacts::process3d::mutations::change_stock_label::mutation::ChangeStockLabel;
use crate::artifacts::process3d::Process3dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeStockLabel, base: &Process3dSnapshot) -> protocol::MutationOutcome<Process3dDiff> {
    if base.stock_label == payload.new_label {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Stock is already labeled \"{}\".", payload.new_label));
    }
    protocol::MutationOutcome::new(Process3dDiff { stock_label: Some(payload.new_label.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
