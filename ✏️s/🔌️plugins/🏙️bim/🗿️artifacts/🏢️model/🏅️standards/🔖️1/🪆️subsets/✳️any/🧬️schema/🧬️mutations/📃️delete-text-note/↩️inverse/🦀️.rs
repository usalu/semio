//! ↩️ Inverse of `DeleteTextNote`: the concrete `CreateTextNote` carrying the full removed record, in storage order through the shared cascade.

use super::super::cascade;
use super::DeleteTextNote;
use crate::{ModelMutation, ModelSnapshot};

pub fn inverse(payload: &DeleteTextNote, base: &ModelSnapshot) -> Vec<ModelMutation> {
    cascade::inverse(base, std::slice::from_ref(&payload.id))
}
