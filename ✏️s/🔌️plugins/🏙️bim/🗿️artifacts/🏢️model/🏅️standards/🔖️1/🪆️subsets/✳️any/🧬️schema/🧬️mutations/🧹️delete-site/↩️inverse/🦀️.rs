//! ↩️ Inverse of `DeleteSite`: one concrete create per removed record and one setter per removed property or classification, in storage order
//! (dependants first, the target last), so the store, which replays the vector reversed, recreates the target before everything on it.

use super::super::cascade;
use super::DeleteSite;
use crate::standards::v1::subsets::any::schema::{mutations::ModelMutation, snapshot::ModelSnapshot};

pub fn inverse(payload: &DeleteSite, base: &ModelSnapshot) -> Vec<ModelMutation> {
    cascade::inverse(base, std::slice::from_ref(&payload.id))
}
