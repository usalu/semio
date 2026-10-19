//! ↩️ Inverse of `CreateMepElement`: the concrete `DeleteMepElement` of the id it created, none when the id was already taken.

use semio_s_artifact_bim_model_mut_any_123::standards::v1::subsets::any::schema::mutations::delete_mep_element::DeleteMepElement;
use crate::standards::v1::subsets::any::schema::mutations::create_mep_element::CreateMepElement;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;

pub fn inverse<Op>(payload: &CreateMepElement, base: &ModelSnapshot) -> Vec<Op>
where
    Op: From<semio_s_artifact_bim_model_mut_any_123::standards::v1::subsets::any::schema::mutations::delete_mep_element::DeleteMepElement>,
{
    if base.mep_elements.contains_key(&payload.id) {
        return Vec::new();
    }
    vec![Op::from(DeleteMepElement { id: payload.id.clone() })]
}
