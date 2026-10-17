//! ↩️ Inverse of `DeleteTextNote`: the concrete `CreateTextNote` carrying the full removed record, in storage order through the shared cascade.

use super::super::cascade;
use super::DeleteTextNote;
use crate::standards::v1::subsets::any::schema::{snapshot::ModelSnapshot};

pub fn inverse<Op>(payload: &DeleteTextNote, base: &ModelSnapshot) -> Vec<Op>
where
    Op: crate::standards::v1::subsets::any::schema::mutations::cascade::RemovalInverse,
{
    cascade::inverse(base, std::slice::from_ref(&payload.id))
}
