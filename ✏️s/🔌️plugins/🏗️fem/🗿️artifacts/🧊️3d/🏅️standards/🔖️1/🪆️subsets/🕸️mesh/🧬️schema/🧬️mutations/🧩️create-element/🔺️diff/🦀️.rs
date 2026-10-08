//! 🔺️ Sparse diff builder for `CreateElement`.
use super::CreateElement;
use crate::standards::v1::subsets::any::schema::diff::{Fem3dDiff, Fem3dElementInsertion, Fem3dElementsDelta};
use crate::standards::v1::subsets::any::schema::mutations::resolve_element;
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
    if payload.index.is_some_and(|at| at > base.elements.len()) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Insert index {} is past the end of {} rows.", payload.index.unwrap_or_default(), base.elements.len()), [new_id.to_string()]);
    }
    protocol::MutationOutcome::new(Fem3dDiff { elements: Some(Fem3dElementsDelta { inserted: vec![Fem3dElementInsertion { index: payload.index.unwrap_or(base.elements.len()), row: (*payload.element).clone() }], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
