//! 🔺️ Sparse diff builder for `CreateSection`.
use super::mutation::CreateSection;
use crate::artifacts::fem3d::diff::{Fem3dDiff, Fem3dSectionsDelta};
use crate::artifacts::fem3d::Fem3dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &CreateSection, base: &Fem3dSnapshot) -> protocol::MutationOutcome<Fem3dDiff> {
    if base.sections.iter().any(|section| section.id == payload.section.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A section with id \"{}\" already exists.", payload.section.id), [payload.section.id.clone()]);
    }
    protocol::MutationOutcome::new(Fem3dDiff { sections: Some(Fem3dSectionsDelta { added: vec![payload.section.clone()], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
