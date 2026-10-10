//! ↩️ Inverse of `CreateIssue`: the concrete `DeleteIssue` of the id it created, none when the id was already taken.

use super::super::delete_issue::DeleteIssue;
use super::CreateIssue;
use crate::{ModelMutation, ModelSnapshot};

pub fn inverse(payload: &CreateIssue, base: &ModelSnapshot) -> Vec<ModelMutation> {
    if base.issues.contains_key(&payload.id) {
        return Vec::new();
    }
    vec![ModelMutation::DeleteIssue(DeleteIssue { id: payload.id.clone() })]
}
