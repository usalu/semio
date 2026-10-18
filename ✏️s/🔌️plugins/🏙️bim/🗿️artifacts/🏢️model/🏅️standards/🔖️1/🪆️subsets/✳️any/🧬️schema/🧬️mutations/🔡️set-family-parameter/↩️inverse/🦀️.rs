//! ↩️ Inverse of `SetFamilyParameter`: for an existing parameter an absolute `SetFamilyParameter` restoring the base value of exactly the fields the forward really changes, for a created one the
//! concrete `RemoveFamilyParameter`; none when the family is absent or nothing changes.

use crate::standards::v1::subsets::any::schema::mutations::remove_family_parameter::RemoveFamilyParameter;
use crate::standards::v1::subsets::any::schema::mutations::set_family_parameter::SetFamilyParameter;
use crate::standards::v1::subsets::any::schema::authored::formula;
use crate::standards::v1::subsets::any::schema::{snapshot::ModelSnapshot, diff::Patch};

pub fn inverse<Op>(payload: &SetFamilyParameter, base: &ModelSnapshot) -> Vec<Op>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::remove_family_parameter::RemoveFamilyParameter> + From<crate::standards::v1::subsets::any::schema::mutations::set_family_parameter::SetFamilyParameter>,
{
    if !base.families.contains_key(&payload.family) {
        return Vec::new();
    }
    match base.family_parameters.get(&formula::parameter_id(&payload.family, &payload.name)) {
        None => vec![Op::from(RemoveFamilyParameter { family: payload.family.clone(), name: payload.name.clone() })],
        Some(record) => {
            let restore = payload.patch().minimal(record).restoring(record);
            if restore.is_empty() {
                return Vec::new();
            }
            vec![Op::from(SetFamilyParameter::from_patch(payload.family.clone(), payload.name.clone(), restore))]
        }
    }
}
