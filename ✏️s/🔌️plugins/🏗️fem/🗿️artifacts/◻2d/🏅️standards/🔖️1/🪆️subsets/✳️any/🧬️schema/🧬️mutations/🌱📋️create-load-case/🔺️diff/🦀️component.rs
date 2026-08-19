//! 🔺️ Sparse diff builder for `CreateLoadCase`.
use super::mutation::CreateLoadCase;
use crate::artifacts::fem2d::diff::{Fem2dDiff, Fem2dLoadCasesDelta};
use crate::artifacts::fem2d::{element_id, Fem2dSnapshot, FemLoad};

//#region 🔖️Diff
pub async fn diff(payload: &CreateLoadCase, base: &Fem2dSnapshot) -> protocol::MutationOutcome<Fem2dDiff> {
    if base.load_cases.iter().any(|case| case.id == payload.load_case.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A load case with id \"{}\" already exists.", payload.load_case.id), [payload.load_case.id.clone()]);
    }
    for load in &payload.load_case.loads {
        match load {
            FemLoad::Nodal { node_id, .. } => {
                if !base.nodes.iter().any(|node| &node.id == node_id) {
                    return protocol::MutationOutcome::error("mutation.target-missing", format!("Node \"{}\" does not exist.", node_id), [node_id.clone()]);
                }
            }
            FemLoad::MemberUdl { element_id: referenced_element_id, .. } => {
                if !base.elements.iter().any(|element| element_id(element) == referenced_element_id) {
                    return protocol::MutationOutcome::error("mutation.target-missing", format!("Element \"{}\" does not exist.", referenced_element_id), [referenced_element_id.clone()]);
                }
            }
            FemLoad::Area { region_id, .. } => {
                if !base.regions.iter().any(|region| &region.id == region_id) {
                    return protocol::MutationOutcome::error("mutation.target-missing", format!("Region \"{}\" does not exist.", region_id), [region_id.clone()]);
                }
            }
        }
    }
    protocol::MutationOutcome::new(Fem2dDiff { load_cases: Some(Fem2dLoadCasesDelta { added: vec![payload.load_case.clone()], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
