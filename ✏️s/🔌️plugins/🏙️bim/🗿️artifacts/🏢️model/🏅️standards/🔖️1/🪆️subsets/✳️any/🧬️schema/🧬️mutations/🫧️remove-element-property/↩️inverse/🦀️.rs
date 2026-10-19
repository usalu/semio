//! ↩️ Inverse of `RemoveElementProperty`: the concrete `SetElementProperty` carrying the removed typed value, none when the property was absent.

use crate::standards::v1::subsets::any::schema::mutations::set_element_property::SetElementProperty;
use crate::standards::v1::subsets::any::schema::mutations::remove_element_property::RemoveElementProperty;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;

pub fn inverse<Op>(payload: &RemoveElementProperty, base: &ModelSnapshot) -> Vec<Op>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::set_element_property::SetElementProperty>,
{
    match base.properties.get(&payload.id).and_then(|sets| sets.get(&payload.pset)).and_then(|properties| properties.get(&payload.property)) {
        Some(value) => vec![Op::from(SetElementProperty { id: payload.id.clone(), pset: payload.pset.clone(), property: payload.property.clone(), value: value.clone() })],
        None => Vec::new(),
    }
}
