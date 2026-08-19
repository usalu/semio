//! 🔺️ Sparse diff builder for `ReplaceSupport`.
use super::mutation::ReplaceSupport;
use crate::artifacts::fem3d::diff::{Fem3dDiff, Fem3dSupportsDelta, Fem3dSupportsPatchEntry};
use crate::artifacts::fem3d::Fem3dSnapshot;

//#region 🔖️Diff
pub async fn diff(payload: &ReplaceSupport, base: &Fem3dSnapshot) -> protocol::MutationOutcome<Fem3dDiff> {
    let Some(existing) = base.supports.iter().find(|support| support.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Support \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if existing == &payload.new_support {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Support \"{}\" already has that value.", payload.id));
    }
    protocol::MutationOutcome::new(Fem3dDiff { supports: Some(Fem3dSupportsDelta { patched: vec![Fem3dSupportsPatchEntry { id: payload.id.clone(), item: payload.new_support.clone() }], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
