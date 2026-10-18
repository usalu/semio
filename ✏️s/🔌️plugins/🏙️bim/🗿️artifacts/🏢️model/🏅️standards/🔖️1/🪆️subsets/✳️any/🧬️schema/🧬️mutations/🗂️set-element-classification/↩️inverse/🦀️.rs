//! ↩️ Inverse of `SetElementClassification`: the absolute `SetElementClassification` back to the base code of the system, or a `RemoveElementClassification` of the system when the holder had none there; none when the holder
//! is absent.

use crate::standards::v1::subsets::any::schema::mutations::elements;
use crate::standards::v1::subsets::any::schema::mutations::remove_element_classification::RemoveElementClassification;
use crate::standards::v1::subsets::any::schema::mutations::set_element_classification::SetElementClassification;
use crate::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;

pub fn inverse<Op>(payload: &SetElementClassification, base: &ModelSnapshot) -> Vec<Op>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::remove_element_classification::RemoveElementClassification> + From<crate::standards::v1::subsets::any::schema::mutations::set_element_classification::SetElementClassification>,
{
    if !elements::holds_data(base, &payload.id) {
        return Vec::new();
    }
    match base.classifications.get(&payload.id).and_then(|set| set.get(&payload.system)) {
        Some(code) => vec![Op::from(SetElementClassification { id: payload.id.clone(), system: payload.system.clone(), code: code.clone() })],
        None => vec![Op::from(RemoveElementClassification { id: payload.id.clone(), system: payload.system.clone() })],
    }
}
