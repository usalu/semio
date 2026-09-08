//! 🔺️ Sparse diff builder for `ReplaceSection`.
//!
//! Guards, in the order they run: `mutation.target-missing` (Error) on the selected id,
//! `mutation.id-mismatch` (Fatal) when the replacement renames it, the SAME positivity bounds
//! `create-section` runs (`mutation.invariant`, Fatal), and finally `mutation.no-op`.
use super::ReplaceSection;
use crate::diff::{Fem2dDiff, Fem2dSectionsDelta, Fem2dSectionsPatchEntry};
use crate::mutations::guards;
use crate::Fem2dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &ReplaceSection, base: &Fem2dSnapshot) -> protocol::MutationOutcome<Fem2dDiff> {
    let Some(existing) = base.sections.iter().find(|section| section.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Section \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if let Some(rejection) = guards::identity_matches("section", &payload.id, &payload.new_section.id) {
        return rejection;
    }
    if let Some(rejection) = guards::section_plausibility(&payload.new_section) {
        return rejection;
    }
    if *existing == payload.new_section {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Section \"{}\" is already equal to the replacement value.", payload.id));
    }
    protocol::MutationOutcome::new(Fem2dDiff { sections: Some(Fem2dSectionsDelta { patched: vec![Fem2dSectionsPatchEntry { id: payload.id.clone(), item: payload.new_section.clone() }], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
