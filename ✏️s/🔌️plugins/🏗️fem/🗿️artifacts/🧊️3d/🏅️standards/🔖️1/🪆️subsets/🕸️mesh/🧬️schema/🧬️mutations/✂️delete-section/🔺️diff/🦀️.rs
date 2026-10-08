//! 🔺️ Sparse diff builder for `DeleteSection`.
use super::DeleteSection;
use crate::standards::v1::subsets::any::schema::diff::{Fem3dDiff, Fem3dSectionRemoval, Fem3dSectionsDelta};
use crate::standards::v1::subsets::any::schema::mutations::{section_referrers,target_referenced};

use crate::Fem3dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &DeleteSection, base: &Fem3dSnapshot) -> protocol::MutationOutcome<Fem3dDiff> {
    let Some(at) = base.sections.iter().position(|section| section.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Section \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    let referrers = section_referrers(base, &payload.id);
    if !referrers.is_empty() {
        return target_referenced("Section", &payload.id, referrers);
    }
    protocol::MutationOutcome::new(Fem3dDiff { sections: Some(Fem3dSectionsDelta { removed: vec![Fem3dSectionRemoval { id: payload.id.clone(), index: at }], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
