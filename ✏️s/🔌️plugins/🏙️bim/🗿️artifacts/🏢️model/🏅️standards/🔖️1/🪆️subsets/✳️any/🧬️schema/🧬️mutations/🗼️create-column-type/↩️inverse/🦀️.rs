//! ↩️ Inverse of `CreateColumnType`: the concrete `DeleteColumnType` of the id it created, none when the id was already taken.

use crate::standards::v1::subsets::any::schema::mutations::delete_column_type::DeleteColumnType;
use crate::standards::v1::subsets::any::schema::mutations::create_column_type::CreateColumnType;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;

pub fn inverse<Op>(payload: &CreateColumnType, base: &ModelSnapshot) -> Vec<Op>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::delete_column_type::DeleteColumnType>,
{
    if base.column_types.contains_key(&payload.id) {
        return Vec::new();
    }
    vec![Op::from(DeleteColumnType { id: payload.id.clone() })]
}
