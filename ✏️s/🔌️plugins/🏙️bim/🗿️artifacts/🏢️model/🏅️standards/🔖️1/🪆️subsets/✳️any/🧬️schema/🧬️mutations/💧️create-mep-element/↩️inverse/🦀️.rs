//! ↩️ Inverse of `CreateMepElement`: the concrete `DeleteMepElement` of the id it created, none when the id was already taken.

use crate::standards::v1::subsets::any::schema::mutations::delete_mep_element::DeleteMepElement;
use crate::standards::v1::subsets::any::schema::mutations::create_mep_element::CreateMepElement;
use crate::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;

pub fn inverse<Op>(payload: &CreateMepElement, base: &ModelSnapshot) -> Vec<Op>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::delete_mep_element::DeleteMepElement>,
{
    if base.mep_elements.contains_key(&payload.id) {
        return Vec::new();
    }
    vec![Op::from(DeleteMepElement { id: payload.id.clone() })]
}
