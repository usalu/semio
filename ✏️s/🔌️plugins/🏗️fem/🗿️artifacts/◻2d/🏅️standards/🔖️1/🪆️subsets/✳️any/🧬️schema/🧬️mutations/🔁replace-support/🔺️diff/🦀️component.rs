//! 🔺️ Sparse diff builder for `ReplaceSupport`.
use super::mutation::ReplaceSupport;
use crate::artifacts::fem2d::diff::{Fem2dDiff, Fem2dSupportsDelta, Fem2dSupportsPatchEntry};
use crate::artifacts::fem2d::Fem2dSnapshot;

//#region 🔖️Diff
pub async fn diff(payload: &ReplaceSupport, base: &Fem2dSnapshot) -> protocol::MutationOutcome<Fem2dDiff> {
    let Some(existing) = base.supports.iter().find(|support| support.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Support \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if *existing == payload.new_support {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Support \"{}\" is already equal to the replacement value.", payload.id));
    }
    protocol::MutationOutcome::new(Fem2dDiff { supports: Some(Fem2dSupportsDelta { patched: vec![Fem2dSupportsPatchEntry { id: payload.id.clone(), item: payload.new_support.clone() }], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
