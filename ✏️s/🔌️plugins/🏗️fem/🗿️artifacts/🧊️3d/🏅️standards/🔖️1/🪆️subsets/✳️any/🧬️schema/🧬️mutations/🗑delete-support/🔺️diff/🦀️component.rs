//! 🔺️ Sparse diff builder for `DeleteSupport`.
use super::mutation::DeleteSupport;
use crate::artifacts::fem3d::diff::{Fem3dDiff, Fem3dSupportsDelta};
use crate::artifacts::fem3d::Fem3dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &DeleteSupport, base: &Fem3dSnapshot) -> protocol::MutationOutcome<Fem3dDiff> {
    if !base.supports.iter().any(|support| support.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Support \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    protocol::MutationOutcome::new(Fem3dDiff { supports: Some(Fem3dSupportsDelta { removed: vec![payload.id.clone()], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
