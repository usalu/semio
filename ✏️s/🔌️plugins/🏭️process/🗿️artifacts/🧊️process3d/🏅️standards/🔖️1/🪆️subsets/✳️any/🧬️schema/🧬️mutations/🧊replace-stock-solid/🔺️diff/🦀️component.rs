//! 🔺️ `replace-stock-solid` sparse diff construction — a single `stock_solid` handle swap, never
//! a snapshot clone. The document has exactly one stock (no target to be missing); Warning `no-op`
//! when the handle is unchanged.

use crate::artifacts::process3d::diff::Process3dDiff;
use crate::artifacts::process3d::mutations::replace_stock_solid::mutation::ReplaceStockSolid;
use crate::artifacts::process3d::Process3dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &ReplaceStockSolid, base: &Process3dSnapshot) -> protocol::MutationOutcome<Process3dDiff> {
    if base.stock_solid == payload.new_solid {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Stock solid handle is unchanged.".to_string());
    }
    protocol::MutationOutcome::new(Process3dDiff { stock_solid: Some(payload.new_solid.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
