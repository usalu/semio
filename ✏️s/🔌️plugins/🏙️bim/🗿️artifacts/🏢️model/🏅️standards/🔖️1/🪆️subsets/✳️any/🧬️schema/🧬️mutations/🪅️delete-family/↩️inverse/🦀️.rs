//! ↩️ Inverse of `DeleteFamily`: the setters of the properties and classifications of the family, then one concrete create per removed record. The store replays the vector reversed, so it holds them first,
//! then the solids, the parameters against their dependency order and the family last: the family is recreated first, then every parameter after the ones its formula uses, then the solids that use the
//! parameters, then the data.

use crate::standards::v1::subsets::any::schema::mutations::cascade;
use crate::standards::v1::subsets::any::schema::mutations::create_family::CreateFamily;
use crate::standards::v1::subsets::any::schema::mutations::create_family_solid::CreateFamilySolid;
use crate::standards::v1::subsets::any::schema::mutations::family_rules;
use crate::standards::v1::subsets::any::schema::mutations::set_family_parameter::SetFamilyParameter;
use crate::standards::v1::subsets::any::schema::mutations::delete_family::DeleteFamily;
use crate::standards::v1::subsets::any::schema::{snapshot::ModelSnapshot};

pub fn inverse<Op>(payload: &DeleteFamily, base: &ModelSnapshot) -> Vec<Op>
where
    Op: crate::standards::v1::subsets::any::schema::mutations::cascade::RemovalInverse + From<crate::standards::v1::subsets::any::schema::mutations::create_family::CreateFamily> + From<crate::standards::v1::subsets::any::schema::mutations::create_family_solid::CreateFamilySolid> + From<crate::standards::v1::subsets::any::schema::mutations::set_family_parameter::SetFamilyParameter>,
{
    let Some(family) = base.families.get(&payload.id) else {
        return Vec::new();
    };
    let solids = base.family_solids.iter().filter(|(_, row)| row.family == payload.id).map(|(id, row)| Op::from(CreateFamilySolid { id: id.clone(), solid: row.clone() }));
    let parameters = family_rules::parameters_in_order(base, &payload.id).into_iter().rev().map(|row| Op::from(SetFamilyParameter { family: row.family.clone(), name: row.name.clone(), kind: Some(row.kind), value: Some(row.value.clone()) }));
    cascade::data_rows(base, &payload.id).into_iter().chain(solids).chain(parameters).chain(std::iter::once(Op::from(CreateFamily { id: payload.id.clone(), family: family.clone() }))).collect()
}
