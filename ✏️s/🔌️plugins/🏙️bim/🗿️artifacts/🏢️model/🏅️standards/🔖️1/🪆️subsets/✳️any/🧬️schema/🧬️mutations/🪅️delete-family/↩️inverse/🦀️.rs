//! ↩️ Inverse of `DeleteFamily`: the setters of the properties and classifications of the family, then one concrete create per removed record. The store replays the vector reversed, so it holds them first,
//! then the solids, the parameters against their dependency order and the family last: the family is recreated first, then every parameter after the ones its formula uses, then the solids that use the
//! parameters, then the data.

use super::super::cascade;
use super::super::create_family::CreateFamily;
use super::super::create_family_solid::CreateFamilySolid;
use super::super::family_rules;
use super::super::set_family_parameter::SetFamilyParameter;
use super::DeleteFamily;
use crate::{ModelMutation, ModelSnapshot};

pub fn inverse(payload: &DeleteFamily, base: &ModelSnapshot) -> Vec<ModelMutation> {
    let Some(family) = base.families.get(&payload.id) else {
        return Vec::new();
    };
    let solids = base.family_solids.iter().filter(|(_, row)| row.family == payload.id).map(|(id, row)| ModelMutation::CreateFamilySolid(CreateFamilySolid { id: id.clone(), solid: row.clone() }));
    let parameters = family_rules::parameters_in_order(base, &payload.id).into_iter().rev().map(|row| ModelMutation::SetFamilyParameter(SetFamilyParameter { family: row.family.clone(), name: row.name.clone(), kind: Some(row.kind), value: Some(row.value.clone()) }));
    cascade::data_rows(base, &payload.id).into_iter().chain(solids).chain(parameters).chain(std::iter::once(ModelMutation::CreateFamily(CreateFamily { id: payload.id.clone(), family: family.clone() }))).collect()
}
