//! ↩️ Inverse of `SetCurtainWall`: an absolute `SetCurtainWall` restoring the base value of exactly the fields the forward really changes, none when the curtain wall is absent or nothing changes.

use crate::standards::v1::subsets::any::schema::mutations::set_curtain_wall::SetCurtainWall;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
use semio_s_artifact_bim_model_authored::standards::v1::subsets::any::schema::diff::algebra::Patch;

pub fn inverse<Op>(payload: &SetCurtainWall, base: &ModelSnapshot) -> Vec<Op>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::set_curtain_wall::SetCurtainWall>,
{
    let Some(record) = base.curtain_walls.get(&payload.id) else {
        return Vec::new();
    };
    let restore = payload.patch().minimal(record).restoring(record);
    if restore.is_empty() {
        return Vec::new();
    }
    vec![Op::from(SetCurtainWall::from_patch(payload.id.clone(), restore))]
}
