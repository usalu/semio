//! 🔺️ Sparse diff builder for `DeleteSection`.
use super::mutation::DeleteSection;
use crate::artifacts::fem2d::diff::{Fem2dDiff, Fem2dSectionsDelta};
use crate::artifacts::fem2d::Fem2dSnapshot;

//#region 🔖️Diff
pub async fn diff(payload: &DeleteSection, base: &Fem2dSnapshot) -> protocol::MutationOutcome<Fem2dDiff> {
    if !base.sections.iter().any(|section| section.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Section \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    protocol::MutationOutcome::new(Fem2dDiff { sections: Some(Fem2dSectionsDelta { removed: vec![payload.id.clone()], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
