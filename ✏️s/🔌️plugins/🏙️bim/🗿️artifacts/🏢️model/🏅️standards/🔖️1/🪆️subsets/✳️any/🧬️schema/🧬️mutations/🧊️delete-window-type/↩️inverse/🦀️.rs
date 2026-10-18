//! ↩️ Inverse of `DeleteWindowType`: the setters of the properties and classifications of the type, then the concrete `CreateWindowType` carrying the full removed record, none when the window type was absent.

use crate::standards::v1::subsets::any::schema::mutations::create_window_type::CreateWindowType;
use crate::standards::v1::subsets::any::schema::mutations::cascade;
use crate::standards::v1::subsets::any::schema::mutations::delete_window_type::DeleteWindowType;
use crate::standards::v1::subsets::any::schema::{snapshot::ModelSnapshot};

pub fn inverse<Op>(payload: &DeleteWindowType, base: &ModelSnapshot) -> Vec<Op>
where
    Op: crate::standards::v1::subsets::any::schema::mutations::cascade::RemovalInverse + From<crate::standards::v1::subsets::any::schema::mutations::create_window_type::CreateWindowType>,
{
    match base.window_types.get(&payload.id) {
        Some(record) => cascade::data_rows(base, &payload.id).into_iter().chain(std::iter::once(Op::from(CreateWindowType { id: payload.id.clone(), window_type: record.clone() }))).collect(),
        None => Vec::new(),
    }
}
