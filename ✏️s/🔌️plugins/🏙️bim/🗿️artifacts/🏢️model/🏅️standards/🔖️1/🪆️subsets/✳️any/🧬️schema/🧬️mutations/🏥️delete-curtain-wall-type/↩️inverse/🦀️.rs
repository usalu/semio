//! ↩️ Inverse of `DeleteCurtainWallType`: the concrete `CreateCurtainWallType` carrying the full removed record, none when the record was absent.

use crate::standards::v1::subsets::any::schema::mutations::create_curtain_wall_type::CreateCurtainWallType;
use crate::standards::v1::subsets::any::schema::mutations::delete_curtain_wall_type::DeleteCurtainWallType;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;

pub fn inverse<Op>(payload: &DeleteCurtainWallType, base: &ModelSnapshot) -> Vec<Op>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::create_curtain_wall_type::CreateCurtainWallType>,
{
    match base.curtain_wall_types.get(&payload.id) {
        Some(record) => vec![Op::from(CreateCurtainWallType { id: payload.id.clone(), curtain_wall_type: record.clone() })],
        None => Vec::new(),
    }
}
