//! 🔺️ Sparse diff builder for `CreateSection`.
use super::mutation::CreateSection;
use crate::artifacts::fem2d::diff::{Fem2dDiff, Fem2dSectionsDelta};
use crate::artifacts::fem2d::Fem2dSnapshot;

//#region 🔖️Diff
pub async fn diff(payload: &CreateSection, base: &Fem2dSnapshot) -> protocol::MutationOutcome<Fem2dDiff> {
    if base.sections.iter().any(|section| section.id == payload.section.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A section with id \"{}\" already exists.", payload.section.id), [payload.section.id.clone()]);
    }
    protocol::MutationOutcome::new(Fem2dDiff { sections: Some(Fem2dSectionsDelta { added: vec![payload.section.clone()], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
