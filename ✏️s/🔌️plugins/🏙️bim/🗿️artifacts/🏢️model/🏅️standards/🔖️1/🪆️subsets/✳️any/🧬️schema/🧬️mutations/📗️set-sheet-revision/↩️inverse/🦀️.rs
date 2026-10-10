//! ↩️ Inverse of `SetSheetRevision`: an absolute `SetSheetRevision` restoring the base value of exactly the fields the forward really changes, none when the revision is absent or nothing changes.

use super::SetSheetRevision;
use crate::{ModelMutation, ModelSnapshot, Patch};

pub fn inverse(payload: &SetSheetRevision, base: &ModelSnapshot) -> Vec<ModelMutation> {
    let Some(record) = base.sheet_revisions.get(&payload.id) else {
        return Vec::new();
    };
    let restore = payload.patch().minimal(record).restoring(record);
    if restore.is_empty() {
        return Vec::new();
    }
    vec![ModelMutation::SetSheetRevision(SetSheetRevision::from_patch(payload.id.clone(), restore))]
}
