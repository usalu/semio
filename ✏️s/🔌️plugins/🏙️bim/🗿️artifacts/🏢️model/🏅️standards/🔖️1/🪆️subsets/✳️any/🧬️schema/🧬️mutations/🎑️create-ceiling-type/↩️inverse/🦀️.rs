//! ↩️ Inverse of `CreateCeilingType`: the concrete `DeleteCeilingType` of the id it created, none when the id was already taken.

use crate::standards::v1::subsets::any::schema::mutations::delete_ceiling_type::DeleteCeilingType;
use crate::standards::v1::subsets::any::schema::mutations::create_ceiling_type::CreateCeilingType;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;

pub fn inverse<Op>(payload: &CreateCeilingType, base: &ModelSnapshot) -> Vec<Op>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::delete_ceiling_type::DeleteCeilingType>,
{
    if base.ceiling_types.contains_key(&payload.id) {
        return Vec::new();
    }
    vec![Op::from(DeleteCeilingType { id: payload.id.clone() })]
}
