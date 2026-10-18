//! ↩️ Inverse of `CreateStorey`: the concrete `DeleteStorey` of the id it created, none when the id was already taken.

use crate::standards::v1::subsets::any::schema::mutations::delete_storey::DeleteStorey;
use crate::standards::v1::subsets::any::schema::mutations::create_storey::CreateStorey;
use crate::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;

pub fn inverse<Op>(payload: &CreateStorey, base: &ModelSnapshot) -> Vec<Op>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::delete_storey::DeleteStorey>,
{
    if base.storeys.contains_key(&payload.id) {
        return Vec::new();
    }
    vec![Op::from(DeleteStorey { id: payload.id.clone() })]
}
