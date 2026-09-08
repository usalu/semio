//! 🔺️ Sparse diff builder for `DeleteElement`.
use super::DeleteElement;
use crate::standards::v1::subsets::any::schema::diff::{Fem3dDiff, Fem3dElementsDelta};
use crate::standards::v1::subsets::any::schema::mutations::{element_referrers, target_referenced};
use crate::{element_id, Fem3dSnapshot};

//#region 🔖️Diff
pub fn diff(payload: &DeleteElement, base: &Fem3dSnapshot) -> protocol::MutationOutcome<Fem3dDiff> {
    if !base.elements.iter().any(|element| element_id(element) == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Element \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    let referrers = element_referrers(base, &payload.id);
    if !referrers.is_empty() {
        return target_referenced("Element", &payload.id, referrers);
    }
    protocol::MutationOutcome::new(Fem3dDiff { elements: Some(Fem3dElementsDelta { removed: vec![payload.id.clone()], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
