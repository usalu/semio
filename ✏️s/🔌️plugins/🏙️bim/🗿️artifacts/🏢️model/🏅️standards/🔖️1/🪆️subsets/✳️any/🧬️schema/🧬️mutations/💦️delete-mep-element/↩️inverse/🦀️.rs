//! ↩️ Inverse of `DeleteMepElement`: the concrete `CreateMepElement` of the removed element and one setter per removed property or classification, in storage order (the element last), so the store,
//! which replays the vector reversed, recreates the element before its data.

use super::super::cascade;
use super::DeleteMepElement;
use crate::standards::v1::subsets::any::schema::{snapshot::ModelSnapshot};

pub fn inverse<Op>(payload: &DeleteMepElement, base: &ModelSnapshot) -> Vec<Op>
where
    Op: crate::standards::v1::subsets::any::schema::mutations::cascade::RemovalInverse,
{
    cascade::inverse(base, std::slice::from_ref(&payload.id))
}
