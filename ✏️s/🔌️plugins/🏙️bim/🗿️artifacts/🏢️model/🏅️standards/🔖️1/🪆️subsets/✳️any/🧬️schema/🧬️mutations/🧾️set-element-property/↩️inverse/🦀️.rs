//! ↩️ Inverse of `SetElementProperty`: an absolute `SetElementProperty` back to the base value, or a `RemoveElementProperty` when the
//! property did not exist; none when the element is absent.

use crate::standards::v1::subsets::any::schema::mutations::remove_element_property::RemoveElementProperty;
use crate::standards::v1::subsets::any::schema::mutations::set_element_property::SetElementProperty;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;

pub fn inverse<Op>(payload: &SetElementProperty, base: &ModelSnapshot) -> Vec<Op>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::remove_element_property::RemoveElementProperty> + From<crate::standards::v1::subsets::any::schema::mutations::set_element_property::SetElementProperty>,
{
    if !semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::mutations::elements::holds_data(base, &payload.id) {
        return Vec::new();
    }
    let old = base.properties.get(&payload.id).and_then(|sets| sets.get(&payload.pset)).and_then(|properties| properties.get(&payload.property));
    match old {
        Some(value) => vec![Op::from(SetElementProperty { id: payload.id.clone(), pset: payload.pset.clone(), property: payload.property.clone(), value: value.clone() })],
        None => vec![Op::from(RemoveElementProperty { id: payload.id.clone(), pset: payload.pset.clone(), property: payload.property.clone() })],
    }
}
