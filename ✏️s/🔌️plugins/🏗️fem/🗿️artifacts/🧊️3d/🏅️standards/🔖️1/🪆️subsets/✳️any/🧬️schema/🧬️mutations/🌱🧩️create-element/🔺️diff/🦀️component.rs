//! 🔺️ Sparse diff builder for `CreateElement`.
use super::mutation::CreateElement;
use crate::artifacts::fem3d::diff::{Fem3dDiff, Fem3dElementsDelta};
use crate::artifacts::fem3d::{element_id, Fem3dSnapshot, FemElement};

//#region 🔖️Diff
pub fn diff(payload: &CreateElement, base: &Fem3dSnapshot) -> protocol::MutationOutcome<Fem3dDiff> {
    let id = element_id(&payload.element);
    if base.elements.iter().any(|element| element_id(element) == id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("An element with id \"{}\" already exists.", id), [id.to_string()]);
    }
    let (start, end, material_id, section_id) = match payload.element.as_ref() {
        FemElement::Bar { start, end, material_id, section_id, .. } => (start, end, material_id, section_id),
        FemElement::Frame { start, end, material_id, section_id, .. } => (start, end, material_id, section_id),
    };
    if !base.nodes.iter().any(|node| &node.id == start) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Node \"{}\" does not exist.", start), [start.clone()]);
    }
    if !base.nodes.iter().any(|node| &node.id == end) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Node \"{}\" does not exist.", end), [end.clone()]);
    }
    if !base.materials.iter().any(|material| &material.id == material_id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Material \"{}\" does not exist.", material_id), [material_id.clone()]);
    }
    if !base.sections.iter().any(|section| &section.id == section_id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Section \"{}\" does not exist.", section_id), [section_id.clone()]);
    }
    protocol::MutationOutcome::new(Fem3dDiff { elements: Some(Fem3dElementsDelta { added: vec![(*payload.element).clone()], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
