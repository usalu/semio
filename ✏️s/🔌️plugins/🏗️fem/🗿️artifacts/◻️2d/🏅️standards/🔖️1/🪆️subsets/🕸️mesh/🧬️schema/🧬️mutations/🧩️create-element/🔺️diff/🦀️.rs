//! 🔺️ Sparse diff builder for `CreateElement`.
//!
//! Guards, in the order they run: `mutation.duplicate-id` (Fatal), then the shared
//! `guards::element_references` resolution of all four foreign keys — `start`, `end`,
//! `material_id`, `section_id` (`mutation.target-missing`, Error). `replace-element` calls the
//! SAME guard, so the twins cannot drift apart.
use super::CreateElement;
use crate::artifacts::fem2d::diff::{Fem2dDiff, Fem2dElementsDelta};
use crate::artifacts::fem2d::mutations::guards;
use crate::artifacts::fem2d::{element_id, Fem2dSnapshot};

//#region 🔖️Diff
pub fn diff(payload: &CreateElement, base: &Fem2dSnapshot) -> protocol::MutationOutcome<Fem2dDiff> {
    let new_id = element_id(&payload.element);
    if base.elements.iter().any(|element| element_id(element) == new_id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("An element with id \"{}\" already exists.", new_id), [new_id.to_string()]);
    }
    if let Some(rejection) = guards::element_references(base, &payload.element) {
        return rejection;
    }
    protocol::MutationOutcome::new(Fem2dDiff { elements: Some(Fem2dElementsDelta { added: vec![(*payload.element).clone()], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
