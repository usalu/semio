//! 🔺️ Sparse diff builder for `CreateLoadCase`.
use super::mutation::CreateLoadCase;
use crate::artifacts::fem3d::diff::{Fem3dDiff, Fem3dLoadCasesDelta};
use crate::artifacts::fem3d::{element_id, Fem3dSnapshot, FemLoad};

//#region 🔖️Diff
pub fn diff(payload: &CreateLoadCase, base: &Fem3dSnapshot) -> protocol::MutationOutcome<Fem3dDiff> {
    if base.load_cases.iter().any(|case| case.id == payload.load_case.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A load case with id \"{}\" already exists.", payload.load_case.id), [payload.load_case.id.clone()]);
    }
    for load in &payload.load_case.loads {
        let missing = match load {
            FemLoad::Nodal { node_id, .. } => (!base.nodes.iter().any(|node| &node.id == node_id)).then(|| ("Node", node_id.clone())),
            FemLoad::MemberUdl { element_id: eid, .. } => (!base.elements.iter().any(|element| element_id(element) == eid)).then(|| ("Element", eid.clone())),
            FemLoad::Area { solid_id, .. } => (!base.solids.iter().any(|solid| &solid.id == solid_id)).then(|| ("Solid", solid_id.clone())),
        };
        if let Some((label, id)) = missing {
            return protocol::MutationOutcome::error("mutation.target-missing", format!("{} \"{}\" does not exist.", label, id), [id]);
        }
    }
    protocol::MutationOutcome::new(Fem3dDiff { load_cases: Some(Fem3dLoadCasesDelta { added: vec![payload.load_case.clone()], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
