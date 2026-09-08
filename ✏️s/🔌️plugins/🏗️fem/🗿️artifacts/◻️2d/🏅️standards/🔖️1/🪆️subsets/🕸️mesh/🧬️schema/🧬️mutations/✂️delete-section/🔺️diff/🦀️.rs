//! 🔺️ Sparse diff builder for `DeleteSection`.
//!
//! Guards, in the order they run: `mutation.target-missing` (Error), then
//! `mutation.target-referenced` (Error) while any element still names this cross-section.
use super::DeleteSection;
use crate::diff::{Fem2dDiff, Fem2dSectionsDelta};
use crate::mutations::guards;
use crate::Fem2dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &DeleteSection, base: &Fem2dSnapshot) -> protocol::MutationOutcome<Fem2dDiff> {
    if !base.sections.iter().any(|section| section.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Section \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    if let Some(rejection) = guards::referenced("Section", "element", &payload.id, guards::section_referrers(base, &payload.id)) {
        return rejection;
    }
    protocol::MutationOutcome::new(Fem2dDiff { sections: Some(Fem2dSectionsDelta { removed: vec![payload.id.clone()], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
