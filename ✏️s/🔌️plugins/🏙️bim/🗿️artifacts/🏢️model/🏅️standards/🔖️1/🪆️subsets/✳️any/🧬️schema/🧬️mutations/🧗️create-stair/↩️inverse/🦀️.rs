//! ↩️ Inverse of `CreateStair`: the concrete `DeleteStair` of the id it created, none when the id was already taken.

use crate::standards::v1::subsets::any::schema::mutations::delete_stair::DeleteStair;
use crate::standards::v1::subsets::any::schema::mutations::create_stair::CreateStair;
use crate::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;

pub fn inverse<Op>(payload: &CreateStair, base: &ModelSnapshot) -> Vec<Op>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::delete_stair::DeleteStair>,
{
    if base.stairs.contains_key(&payload.id) {
        return Vec::new();
    }
    vec![Op::from(DeleteStair { id: payload.id.clone() })]
}
