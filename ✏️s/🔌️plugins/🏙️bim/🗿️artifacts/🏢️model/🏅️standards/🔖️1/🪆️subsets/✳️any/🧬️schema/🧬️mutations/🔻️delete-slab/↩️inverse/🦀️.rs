//! ↩️ Inverse of `DeleteSlab`: one concrete create per removed record and one setter per removed property or classification, in storage
//! order (dependants first, the target last), so the store, which replays the vector reversed, recreates the target before anything on it.

use super::super::cascade;
use super::DeleteSlab;
use crate::{ModelMutation, ModelSnapshot};

pub fn inverse(payload: &DeleteSlab, base: &ModelSnapshot) -> Vec<ModelMutation> {
    cascade::inverse(base, std::slice::from_ref(&payload.id))
}
