//! ↩️ Inverse of `CreateView`: the concrete `DeleteView` of the id it created, none when the id was already taken.

use super::super::delete_view::DeleteView;
use super::CreateView;
use crate::{ModelMutation, ModelSnapshot};

pub fn inverse(payload: &CreateView, base: &ModelSnapshot) -> Vec<ModelMutation> {
    if base.views.contains_key(&payload.id) {
        return Vec::new();
    }
    vec![ModelMutation::DeleteView(DeleteView { id: payload.id.clone() })]
}
