//! ↩️ Inverse of `RemoveFamilyParameter`: the concrete `SetFamilyParameter` carrying the kind and the formula of the removed parameter, none when it was absent.

use crate::standards::v1::subsets::any::schema::mutations::set_family_parameter::SetFamilyParameter;
use crate::standards::v1::subsets::any::schema::mutations::remove_family_parameter::RemoveFamilyParameter;
use crate::standards::v1::subsets::any::schema::authored::formula;
use crate::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;

pub fn inverse<Op>(payload: &RemoveFamilyParameter, base: &ModelSnapshot) -> Vec<Op>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::set_family_parameter::SetFamilyParameter>,
{
    match base.family_parameters.get(&formula::parameter_id(&payload.family, &payload.name)) {
        Some(record) => vec![Op::from(SetFamilyParameter { family: payload.family.clone(), name: payload.name.clone(), kind: Some(record.kind), value: Some(record.value.clone()) })],
        None => Vec::new(),
    }
}
