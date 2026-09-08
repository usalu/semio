//! 🔺️ Sparse diff builder for `ReplaceSection`.
use super::ReplaceSection;
use crate::standards::v1::subsets::any::schema::diff::{Fem3dDiff, Fem3dSectionsDelta, Fem3dSectionsPatchEntry};
use crate::standards::v1::subsets::any::schema::mutations::{id_mismatch, invariant, section_breach};
use crate::Fem3dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &ReplaceSection, base: &Fem3dSnapshot) -> protocol::MutationOutcome<Fem3dDiff> {
    let Some(existing) = base.sections.iter().find(|section| section.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Section \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if payload.new_section.id != payload.id {
        return id_mismatch("Section", &payload.id, &payload.new_section.id);
    }
    if existing == &payload.new_section {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Section \"{}\" already has that value.", payload.id));
    }
    if let Some(breach) = section_breach(&payload.new_section) {
        return invariant(breach, vec![payload.id.clone()]);
    }
    protocol::MutationOutcome::new(Fem3dDiff { sections: Some(Fem3dSectionsDelta { patched: vec![Fem3dSectionsPatchEntry { id: payload.id.clone(), item: payload.new_section.clone() }], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
