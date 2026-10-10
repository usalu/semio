//! ↩️ Inverse of `RemoveFamilyParameter`: the concrete `SetFamilyParameter` carrying the kind and the formula of the removed parameter, none when it was absent.

use super::super::set_family_parameter::SetFamilyParameter;
use super::RemoveFamilyParameter;
use crate::standards::v1::subsets::any::schema::authored::formula;
use crate::{ModelMutation, ModelSnapshot};

pub fn inverse(payload: &RemoveFamilyParameter, base: &ModelSnapshot) -> Vec<ModelMutation> {
    match base.family_parameters.get(&formula::parameter_id(&payload.family, &payload.name)) {
        Some(record) => vec![ModelMutation::SetFamilyParameter(SetFamilyParameter { family: payload.family.clone(), name: payload.name.clone(), kind: Some(record.kind), value: Some(record.value.clone()) })],
        None => Vec::new(),
    }
}
