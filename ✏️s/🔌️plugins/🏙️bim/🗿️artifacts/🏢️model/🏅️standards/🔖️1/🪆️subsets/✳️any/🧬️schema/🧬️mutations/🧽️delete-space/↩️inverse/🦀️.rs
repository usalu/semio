//! ↩️ Inverse of `DeleteSpace`: one concrete create per removed record and one setter per removed property or classification, in storage
//! order (dependants first, the target last), so the store, which replays the vector reversed, recreates the target before anything on it.

use super::super::cascade;
use super::DeleteSpace;
use crate::standards::v1::subsets::any::schema::{snapshot::ModelSnapshot};

pub fn inverse<Op>(payload: &DeleteSpace, base: &ModelSnapshot) -> Vec<Op>
where
    Op: crate::standards::v1::subsets::any::schema::mutations::cascade::RemovalInverse,
{
    cascade::inverse(base, std::slice::from_ref(&payload.id))
}
