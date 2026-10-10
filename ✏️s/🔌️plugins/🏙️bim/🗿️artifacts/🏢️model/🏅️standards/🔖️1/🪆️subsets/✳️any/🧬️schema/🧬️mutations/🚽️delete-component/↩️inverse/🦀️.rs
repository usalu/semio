//! ↩️ Inverse of `DeleteComponent`: the concrete `CreateComponent` of the removed component, one `SetComponentOverride` per removed override and one setter per removed property or classification, in storage
//! order (dependants first, the component last), so the store, which replays the vector reversed, recreates the component before anything that belongs to it.

use super::super::cascade;
use super::DeleteComponent;
use crate::{ModelMutation, ModelSnapshot};

pub fn inverse(payload: &DeleteComponent, base: &ModelSnapshot) -> Vec<ModelMutation> {
    cascade::inverse(base, std::slice::from_ref(&payload.id))
}
