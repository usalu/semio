//! 🔺️ Sparse diff builder for `DeleteElement`.
use super::DeleteElement;
use crate::artifacts::fem2d::diff::{Fem2dDiff, Fem2dElementsDelta};
use crate::artifacts::fem2d::{element_id, Fem2dSnapshot};

//#region 🔖️Diff
pub fn diff(payload: &DeleteElement, base: &Fem2dSnapshot) -> protocol::MutationOutcome<Fem2dDiff> {
    if !base.elements.iter().any(|element| element_id(element) == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Element \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    protocol::MutationOutcome::new(Fem2dDiff { elements: Some(Fem2dElementsDelta { removed: vec![payload.id.clone()], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
