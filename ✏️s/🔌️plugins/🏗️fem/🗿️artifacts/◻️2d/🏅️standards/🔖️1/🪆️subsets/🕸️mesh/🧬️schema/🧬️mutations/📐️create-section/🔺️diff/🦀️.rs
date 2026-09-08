//! 🔺️ Sparse diff builder for `CreateSection`.
//!
//! Guards, in the order they run: `mutation.duplicate-id` (Fatal), then the shared
//! `guards::section_plausibility` positivity bounds (`mutation.invariant`, Fatal).
use super::CreateSection;
use crate::standards::v1::subsets::any::schema::diff::{Fem2dDiff, Fem2dSectionsDelta};
use crate::standards::v1::subsets::any::schema::mutations::guards;
use crate::Fem2dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &CreateSection, base: &Fem2dSnapshot) -> protocol::MutationOutcome<Fem2dDiff> {
    if base.sections.iter().any(|section| section.id == payload.section.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A section with id \"{}\" already exists.", payload.section.id), [payload.section.id.clone()]);
    }
    if let Some(rejection) = guards::section_plausibility(&payload.section) {
        return rejection;
    }
    protocol::MutationOutcome::new(Fem2dDiff { sections: Some(Fem2dSectionsDelta { added: vec![payload.section.clone()], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
