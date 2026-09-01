//! 🔺️ Sparse diff builder for `ReplaceSection`.
use super::ReplaceSection;
use crate::artifacts::fem2d::diff::{Fem2dDiff, Fem2dSectionsDelta, Fem2dSectionsPatchEntry};
use crate::artifacts::fem2d::Fem2dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &ReplaceSection, base: &Fem2dSnapshot) -> protocol::MutationOutcome<Fem2dDiff> {
    let Some(existing) = base.sections.iter().find(|section| section.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Section \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if *existing == payload.new_section {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Section \"{}\" is already equal to the replacement value.", payload.id));
    }
    protocol::MutationOutcome::new(Fem2dDiff { sections: Some(Fem2dSectionsDelta { patched: vec![Fem2dSectionsPatchEntry { id: payload.id.clone(), item: payload.new_section.clone() }], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
