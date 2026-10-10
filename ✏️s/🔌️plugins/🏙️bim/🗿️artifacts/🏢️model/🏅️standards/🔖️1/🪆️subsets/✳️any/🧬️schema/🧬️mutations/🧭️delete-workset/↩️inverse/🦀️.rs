//! ↩️ Concrete delete-workset inverse mutations.
use super::DeleteWorkset;
use crate::*;
pub fn inverse(payload: &DeleteWorkset, base: &ModelSnapshot) -> Vec<ModelMutation> {
    let Some(record) = base.worksets.get(&payload.id) else { return Vec::new(); };
    let mut result = Vec::new();
    for (id, member) in &base.element_worksets { if member.target == payload.id { result.push(ModelMutation::SetElementWorkset(super::super::set_element_workset::SetElementWorkset { id: id.clone(), workset: Some(member.target.clone()) })); } }
    result.push(ModelMutation::CreateWorkset(super::super::create_workset::CreateWorkset { id: payload.id.clone(), workset: record.clone() }));
    result
}
