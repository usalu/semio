//! ↩️ Inverse of `SetCurtainWall`: an absolute `SetCurtainWall` restoring the base value of exactly the fields the forward really changes, none when the curtain wall is absent or nothing changes.

use crate::standards::v1::subsets::any::schema::mutations::set_curtain_wall::SetCurtainWall;
use crate::standards::v1::subsets::any::schema::{snapshot::ModelSnapshot, diff::Patch};

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
