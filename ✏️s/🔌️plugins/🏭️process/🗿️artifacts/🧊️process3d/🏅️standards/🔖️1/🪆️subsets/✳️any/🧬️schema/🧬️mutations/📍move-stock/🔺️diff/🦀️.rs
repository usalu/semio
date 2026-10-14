//! 🔺️ `move-stock` sparse diff construction — a whole-`Stock` value with only `pose` replaced from
//! `base`, never a snapshot clone. The document has exactly one stock (no target to be missing);
//! Warning `no-op` when the pose is unchanged, Fatal `invariant` when the pose is non-finite.

use semio_s_artifact_process_process3d_model_diff::Process3dDiff;
use semio_s_artifact_process_process3d_model::snapshot::Process3dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::MoveStock, base: &Process3dSnapshot) -> protocol::MutationOutcome<Process3dDiff> {
    let pose = &payload.new_pose;
    let finite = pose.position.iter().chain(pose.axis.iter()).all(|value| value.is_finite()) && pose.angle.is_finite();
    if !finite {
        return protocol::MutationOutcome::fatal("mutation.invariant", "Stock pose must be finite.".to_string(), Vec::<String>::new());
    }
    if base.stock_pose == payload.new_pose {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Stock is already at that pose.".to_string());
    }
    protocol::MutationOutcome::new(Process3dDiff { stock_pose: Some(payload.new_pose.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
