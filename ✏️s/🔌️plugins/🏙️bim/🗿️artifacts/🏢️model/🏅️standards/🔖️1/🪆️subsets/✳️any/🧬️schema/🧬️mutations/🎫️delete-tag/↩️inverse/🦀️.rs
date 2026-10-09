//! ↩️ Inverse of `DeleteTag`: the concrete `CreateTag` carrying the full removed record, in storage order through the shared cascade.

use super::super::cascade;
use super::DeleteTag;
use crate::{ModelMutation, ModelSnapshot};

pub fn inverse(payload: &DeleteTag, base: &ModelSnapshot) -> Vec<ModelMutation> {
    cascade::inverse(base, std::slice::from_ref(&payload.id))
}
