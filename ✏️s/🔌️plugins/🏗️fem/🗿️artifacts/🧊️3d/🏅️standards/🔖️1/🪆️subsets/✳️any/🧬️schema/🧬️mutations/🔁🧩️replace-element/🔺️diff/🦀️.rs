//! 🔺️ Sparse diff builder for `ReplaceElement`.
use super::ReplaceElement;
use crate::artifacts::fem3d::diff::{Fem3dDiff, Fem3dElementsDelta, Fem3dElementsPatchEntry};
use crate::artifacts::fem3d::{element_id, Fem3dSnapshot};

//#region 🔖️Diff
pub fn diff(payload: &ReplaceElement, base: &Fem3dSnapshot) -> protocol::MutationOutcome<Fem3dDiff> {
    let Some(existing) = base.elements.iter().find(|element| element_id(element) == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Element \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if existing == payload.new_element.as_ref() {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Element \"{}\" already has that value.", payload.id));
    }
    protocol::MutationOutcome::new(Fem3dDiff { elements: Some(Fem3dElementsDelta { patched: vec![Fem3dElementsPatchEntry { id: payload.id.clone(), item: (*payload.new_element).clone() }], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
