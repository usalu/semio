//! 🔺️ Sparse diff builder for `DeleteElement`.
use super::mutation::DeleteElement;
use crate::artifacts::fem3d::diff::{Fem3dDiff, Fem3dElementsDelta};
use crate::artifacts::fem3d::{element_id, Fem3dSnapshot};

//#region 🔖️Diff
pub async fn diff(payload: &DeleteElement, base: &Fem3dSnapshot) -> protocol::MutationOutcome<Fem3dDiff> {
    if !base.elements.iter().any(|element| element_id(element) == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Element \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    protocol::MutationOutcome::new(Fem3dDiff { elements: Some(Fem3dElementsDelta { removed: vec![payload.id.clone()], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
