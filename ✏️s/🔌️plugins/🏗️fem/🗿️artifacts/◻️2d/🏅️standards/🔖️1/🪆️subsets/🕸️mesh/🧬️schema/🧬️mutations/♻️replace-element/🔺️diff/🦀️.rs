//! 🔺️ Sparse diff builder for `ReplaceElement`.
//!
//! Guards, in the order they run: `mutation.target-missing` (Error) on the selected id,
//! `mutation.id-mismatch` (Fatal) when the replacement renames it, the SAME four foreign-key
//! resolutions `create-element` runs (`mutation.target-missing`, Error), and finally the
//! `mutation.no-op` warning when the replacement equals what is already there.
use super::ReplaceElement;
use crate::artifacts::fem2d::diff::{Fem2dDiff, Fem2dElementsDelta, Fem2dElementsPatchEntry};
use crate::artifacts::fem2d::mutations::guards;
use crate::artifacts::fem2d::{element_id, Fem2dSnapshot};

//#region 🔖️Diff
pub fn diff(payload: &ReplaceElement, base: &Fem2dSnapshot) -> protocol::MutationOutcome<Fem2dDiff> {
    let Some(existing) = base.elements.iter().find(|element| element_id(element) == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Element \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if let Some(rejection) = guards::identity_matches("element", &payload.id, element_id(&payload.new_element)) {
        return rejection;
    }
    if let Some(rejection) = guards::element_references(base, &payload.new_element) {
        return rejection;
    }
    if existing == payload.new_element.as_ref() {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Element \"{}\" is already equal to the replacement value.", payload.id));
    }
    protocol::MutationOutcome::new(Fem2dDiff { elements: Some(Fem2dElementsDelta { patched: vec![Fem2dElementsPatchEntry { id: payload.id.clone(), item: (*payload.new_element).clone() }], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
