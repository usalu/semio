//! ↩️ Concrete create-workset inverse mutations.
use super::CreateWorkset;
use crate::*;
pub fn inverse(payload: &CreateWorkset, base: &ModelSnapshot) -> Vec<ModelMutation> {
    vec![ModelMutation::DeleteWorkset(super::super::delete_workset::DeleteWorkset { id: payload.id.clone() })]
}
