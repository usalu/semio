//! 🔺️ Sparse diff builder for `DeleteSection`.
use super::mutation::DeleteSection;
use crate::artifacts::fem3d::diff::{Fem3dDiff, Fem3dSectionsDelta};
use crate::artifacts::fem3d::Fem3dSnapshot;

//#region 🔖️Diff
pub async fn diff(payload: &DeleteSection, base: &Fem3dSnapshot) -> protocol::MutationOutcome<Fem3dDiff> {
    if !base.sections.iter().any(|section| section.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Section \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    protocol::MutationOutcome::new(Fem3dDiff { sections: Some(Fem3dSectionsDelta { removed: vec![payload.id.clone()], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
