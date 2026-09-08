//! 🔺️ Sparse diff builder for `DeleteElement`.
//!
//! Guards, in the order they run: `mutation.target-missing` (Error), then
//! `mutation.target-referenced` (Error) while any load case still carries a member UDL naming this
//! element — deleting it would leave that UDL pointing at nothing.
use super::DeleteElement;
use crate::diff::{Fem2dDiff, Fem2dElementsDelta};
use crate::mutations::guards;
use crate::{element_id, Fem2dSnapshot};

//#region 🔖️Diff
pub fn diff(payload: &DeleteElement, base: &Fem2dSnapshot) -> protocol::MutationOutcome<Fem2dDiff> {
    if !base.elements.iter().any(|element| element_id(element) == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Element \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    if let Some(rejection) = guards::referenced("Element", "member UDL", &payload.id, guards::element_referrers(base, &payload.id)) {
        return rejection;
    }
    protocol::MutationOutcome::new(Fem2dDiff { elements: Some(Fem2dElementsDelta { removed: vec![payload.id.clone()], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
