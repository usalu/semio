//! 🔺️ Sparse diff builder for `CreateElement`.
use super::CreateElement;
use crate::diff::{Fem3dDiff, Fem3dElementsDelta};
use crate::mutations::resolve_element;
use crate::{element_id, Fem3dSnapshot};

//#region 🔖️Diff
pub fn diff(payload: &CreateElement, base: &Fem3dSnapshot) -> protocol::MutationOutcome<Fem3dDiff> {
    let id = element_id(&payload.element);
    if base.elements.iter().any(|element| element_id(element) == id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("An element with id \"{id}\" already exists."), [id.to_string()]);
    }
    if let Some(refusal) = resolve_element(base, &payload.element) {
        return refusal;
    }
    protocol::MutationOutcome::new(Fem3dDiff { elements: Some(Fem3dElementsDelta { added: vec![(*payload.element).clone()], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
