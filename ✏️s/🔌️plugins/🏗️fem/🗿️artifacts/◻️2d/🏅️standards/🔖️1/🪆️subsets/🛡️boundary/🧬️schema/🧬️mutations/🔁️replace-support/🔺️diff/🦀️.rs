//! 🔺️ Sparse diff builder for `ReplaceSupport`.
//!
//! Guards, in the order they run: `mutation.target-missing` (Error) on the selected id,
//! `mutation.id-mismatch` (Fatal) when the replacement renames it, the SAME `node_id` resolution
//! `create-support` runs (`mutation.target-missing`, Error), and finally `mutation.no-op`.
use super::ReplaceSupport;
use crate::standards::v1::subsets::any::schema::diff::{Fem2dDiff, Fem2dSupportsDelta, Fem2dSupportsPatchEntry};
use crate::standards::v1::subsets::any::schema::mutations::guards;
use crate::Fem2dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &ReplaceSupport, base: &Fem2dSnapshot) -> protocol::MutationOutcome<Fem2dDiff> {
    let Some(existing) = base.supports.iter().find(|support| support.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Support \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if let Some(rejection) = guards::identity_matches("support", &payload.id, &payload.new_support.id) {
        return rejection;
    }
    if let Some(rejection) = guards::node_reference(base, &payload.new_support.node_id) {
        return rejection;
    }
    if *existing == payload.new_support {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Support \"{}\" is already equal to the replacement value.", payload.id));
    }
    protocol::MutationOutcome::new(Fem2dDiff { supports: Some(Fem2dSupportsDelta { patched: vec![Fem2dSupportsPatchEntry { id: payload.id.clone(), item: payload.new_support.clone() }], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
