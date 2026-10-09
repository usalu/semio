//! ↩️ Inverse of `SetView`: an absolute `SetView` restoring the base value of exactly the fields the forward really changes, none when the view is absent or nothing changes.

use super::SetView;
use crate::{ModelMutation, ModelSnapshot, Patch};

pub fn inverse(payload: &SetView, base: &ModelSnapshot) -> Vec<ModelMutation> {
    let Some(record) = base.views.get(&payload.id) else {
        return Vec::new();
    };
    let restore = payload.patch().minimal(record).restoring(record);
    if restore.is_empty() {
        return Vec::new();
    }
    vec![ModelMutation::SetView(SetView::from_patch(payload.id.clone(), restore))]
}
