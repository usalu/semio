//! ↩️ Inverse of `DeleteBeam`: one concrete create per removed record and one setter per removed property or classification, in storage
//! order (dependants first, the target last), so the store, which replays the vector reversed, recreates the target before anything on it.

use super::super::cascade;
use super::DeleteBeam;
use crate::{ModelMutation, ModelSnapshot};

pub fn inverse(payload: &DeleteBeam, base: &ModelSnapshot) -> Vec<ModelMutation> {
    cascade::inverse(base, std::slice::from_ref(&payload.id))
}
