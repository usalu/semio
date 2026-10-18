//! ↩️ Inverse of `CreateWindowType`: the concrete `DeleteWindowType` of the id it created, none when the id was already taken.

use crate::standards::v1::subsets::any::schema::mutations::delete_window_type::DeleteWindowType;
use crate::standards::v1::subsets::any::schema::mutations::create_window_type::CreateWindowType;
use crate::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;

pub fn inverse<Op>(payload: &CreateWindowType, base: &ModelSnapshot) -> Vec<Op>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::delete_window_type::DeleteWindowType>,
{
    if base.window_types.contains_key(&payload.id) {
        return Vec::new();
    }
    vec![Op::from(DeleteWindowType { id: payload.id.clone() })]
}
