//! ↩️ Inverse of `SetCurtainWallTypeOf`: an absolute `SetCurtainWallTypeOf` back to the base curtain wall type, none when the curtain wall is absent.

use crate::standards::v1::subsets::any::schema::mutations::set_curtain_wall_type_of::SetCurtainWallTypeOf;
use crate::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;

pub fn inverse<Op>(payload: &SetCurtainWallTypeOf, base: &ModelSnapshot) -> Vec<Op>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::set_curtain_wall_type_of::SetCurtainWallTypeOf>,
{
    match base.curtain_walls.get(&payload.id) {
        Some(wall) => vec![Op::from(SetCurtainWallTypeOf { id: payload.id.clone(), curtain_wall_type: wall.curtain_wall_type.clone() })],
        None => Vec::new(),
    }
}
