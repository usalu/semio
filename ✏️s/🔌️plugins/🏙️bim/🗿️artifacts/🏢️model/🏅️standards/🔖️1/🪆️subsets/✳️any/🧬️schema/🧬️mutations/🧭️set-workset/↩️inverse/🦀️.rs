//! ↩️ Concrete set-workset inverse mutations.
use super::SetWorkset;
use crate::*;
pub fn inverse(payload: &SetWorkset, base: &ModelSnapshot) -> Vec<ModelMutation> {
    let Some(record) = base.worksets.get(&payload.id) else { return Vec::new(); };
    let patch = payload.patch().minimal(record).restoring(record);
    if patch.is_empty() { return Vec::new(); }
    vec![ModelMutation::SetWorkset(SetWorkset::from_patch(payload.id.clone(), patch))]
}
