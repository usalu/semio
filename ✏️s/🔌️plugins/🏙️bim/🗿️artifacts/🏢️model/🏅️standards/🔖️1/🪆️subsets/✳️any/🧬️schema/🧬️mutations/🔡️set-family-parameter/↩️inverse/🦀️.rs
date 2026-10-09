//! ↩️ Inverse of `SetFamilyParameter`: for an existing parameter an absolute `SetFamilyParameter` restoring the base value of exactly the fields the forward really changes, for a created one the
//! concrete `RemoveFamilyParameter`; none when the family is absent or nothing changes.

use super::super::remove_family_parameter::RemoveFamilyParameter;
use super::SetFamilyParameter;
use crate::standards::v1::subsets::any::schema::inferences::families::formula;
use crate::{ModelMutation, ModelSnapshot, Patch};

pub fn inverse(payload: &SetFamilyParameter, base: &ModelSnapshot) -> Vec<ModelMutation> {
    if !base.families.contains_key(&payload.family) {
        return Vec::new();
    }
    match base.family_parameters.get(&formula::parameter_id(&payload.family, &payload.name)) {
        None => vec![ModelMutation::RemoveFamilyParameter(RemoveFamilyParameter { family: payload.family.clone(), name: payload.name.clone() })],
        Some(record) => {
            let restore = payload.patch().minimal(record).restoring(record);
            if restore.is_empty() {
                return Vec::new();
            }
            vec![ModelMutation::SetFamilyParameter(SetFamilyParameter::from_patch(payload.family.clone(), payload.name.clone(), restore))]
        }
    }
}
