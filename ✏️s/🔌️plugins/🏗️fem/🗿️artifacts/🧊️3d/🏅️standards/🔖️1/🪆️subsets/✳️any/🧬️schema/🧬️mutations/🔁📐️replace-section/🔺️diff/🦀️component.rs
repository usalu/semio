//! 🔺️ Sparse diff builder for `ReplaceSection`.
use super::mutation::ReplaceSection;
use crate::artifacts::fem3d::diff::{Fem3dDiff, Fem3dSectionsDelta, Fem3dSectionsPatchEntry};
use crate::artifacts::fem3d::Fem3dSnapshot;

//#region 🔖️Diff
pub async fn diff(payload: &ReplaceSection, base: &Fem3dSnapshot) -> protocol::MutationOutcome<Fem3dDiff> {
    let Some(existing) = base.sections.iter().find(|section| section.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Section \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if existing == &payload.new_section {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Section \"{}\" already has that value.", payload.id));
    }
    protocol::MutationOutcome::new(Fem3dDiff { sections: Some(Fem3dSectionsDelta { patched: vec![Fem3dSectionsPatchEntry { id: payload.id.clone(), item: payload.new_section.clone() }], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
