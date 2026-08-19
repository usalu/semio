//! 🔺️ `move-stock` sparse diff construction — a whole-`Stock` value with only `pose` replaced from
//! `base`, never a snapshot clone. The document has exactly one stock (no target to be missing);
//! Warning `no-op` when the pose is unchanged, Fatal `invariant` when the pose is non-finite.

use crate::artifacts::process3d::diff::Process3dDiff;
use crate::artifacts::process3d::mutations::move_stock::mutation::MoveStock;
use crate::artifacts::process3d::Process3dSnapshot;

//#region 🔖️Diff
pub async fn diff(payload: &MoveStock, base: &Process3dSnapshot) -> protocol::MutationOutcome<Process3dDiff> {
    let pose = &payload.new_pose;
    let finite = pose.position.iter().chain(pose.axis.iter()).all(|value| value.is_finite()) && pose.angle.is_finite();
    if !finite {
        return protocol::MutationOutcome::fatal("mutation.invariant", "Stock pose must be finite.".to_string(), Vec::<String>::new());
    }
    if base.stock_pose == payload.new_pose {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Stock is already at that pose.".to_string());
    }
    protocol::MutationOutcome::new(Process3dDiff { stock_pose: Some(payload.new_pose.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
