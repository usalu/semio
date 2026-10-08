//! 🔺️ Sparse diff builder for `CreateSection`.
use super::CreateSection;
use crate::standards::v1::subsets::any::schema::diff::{Fem3dDiff, Fem3dSectionInsertion, Fem3dSectionsDelta};
use crate::standards::v1::subsets::any::schema::mutations::{invariant,section_breach};

use crate::Fem3dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &CreateSection, base: &Fem3dSnapshot) -> protocol::MutationOutcome<Fem3dDiff> {
    if base.sections.iter().any(|section| section.id == payload.section.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A section with id \"{}\" already exists.", payload.section.id), [payload.section.id.clone()]);
    }
    if let Some(breach) = section_breach(&payload.section) {
        return invariant(breach, vec![payload.section.id.clone()]);
    }
    if payload.index.is_some_and(|at| at > base.sections.len()) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Insert index {} is past the end of {} rows.", payload.index.unwrap_or_default(), base.sections.len()), [&payload.section.id.to_string()]);
    }
    protocol::MutationOutcome::new(Fem3dDiff { sections: Some(Fem3dSectionsDelta { inserted: vec![Fem3dSectionInsertion { index: payload.index.unwrap_or(base.sections.len()), row: payload.section.clone() }], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
