//! ↩️ Inverse of `CreateSheetRevision`: the concrete `DeleteSheetRevision` of the id it created, none when the id was already taken.

use super::super::delete_sheet_revision::DeleteSheetRevision;
use super::CreateSheetRevision;
use crate::{ModelMutation, ModelSnapshot};

pub fn inverse(payload: &CreateSheetRevision, base: &ModelSnapshot) -> Vec<ModelMutation> {
    if base.sheet_revisions.contains_key(&payload.id) {
        return Vec::new();
    }
    vec![ModelMutation::DeleteSheetRevision(DeleteSheetRevision { id: payload.id.clone() })]
}
