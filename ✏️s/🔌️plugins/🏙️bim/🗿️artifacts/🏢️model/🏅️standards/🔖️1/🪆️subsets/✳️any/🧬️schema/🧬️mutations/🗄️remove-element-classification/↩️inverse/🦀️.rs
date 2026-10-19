//! ↩️ Inverse of `RemoveElementClassification`: the concrete `SetElementClassification` carrying the removed code of the system, none when the holder had none there.

use crate::standards::v1::subsets::any::schema::mutations::set_element_classification::SetElementClassification;
use crate::standards::v1::subsets::any::schema::mutations::remove_element_classification::RemoveElementClassification;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;

pub fn inverse<Op>(payload: &RemoveElementClassification, base: &ModelSnapshot) -> Vec<Op>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::set_element_classification::SetElementClassification>,
{
    match base.classifications.get(&payload.id).and_then(|set| set.get(&payload.system)) {
        Some(code) => vec![Op::from(SetElementClassification { id: payload.id.clone(), system: payload.system.clone(), code: code.clone() })],
        None => Vec::new(),
    }
}
