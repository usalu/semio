//! 🔺️ Sparse diff builder for `DeleteSection`.
use super::DeleteSection;
use crate::diff::{Fem3dDiff, Fem3dSectionsDelta};
use crate::mutations::{section_referrers, target_referenced};
use crate::Fem3dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &DeleteSection, base: &Fem3dSnapshot) -> protocol::MutationOutcome<Fem3dDiff> {
    if !base.sections.iter().any(|section| section.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Section \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    let referrers = section_referrers(base, &payload.id);
    if !referrers.is_empty() {
        return target_referenced("Section", &payload.id, referrers);
    }
    protocol::MutationOutcome::new(Fem3dDiff { sections: Some(Fem3dSectionsDelta { removed: vec![payload.id.clone()], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
