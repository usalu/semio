//! ↩️ Inverse of `SetCurtainWallType`: an absolute `SetCurtainWallType` restoring the base value of exactly the fields the forward really changes, none when the curtain wall type is absent or nothing changes.

use crate::standards::v1::subsets::any::schema::mutations::set_curtain_wall_type::SetCurtainWallType;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
use semio_s_artifact_bim_model_authored::standards::v1::subsets::any::schema::diff::algebra::Patch;

pub fn inverse<Op>(payload: &SetCurtainWallType, base: &ModelSnapshot) -> Vec<Op>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::set_curtain_wall_type::SetCurtainWallType>,
{
    let Some(record) = base.curtain_wall_types.get(&payload.id) else {
        return Vec::new();
    };
    let restore = payload.patch().minimal(record).restoring(record);
    if restore.is_empty() {
        return Vec::new();
    }
    vec![Op::from(SetCurtainWallType::from_patch(payload.id.clone(), restore))]
}
