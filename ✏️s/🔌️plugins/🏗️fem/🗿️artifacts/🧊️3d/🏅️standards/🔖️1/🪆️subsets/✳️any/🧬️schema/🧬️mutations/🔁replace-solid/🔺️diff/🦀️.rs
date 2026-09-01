//! 🔺️ Sparse diff builder for `ReplaceSolid`.
use super::ReplaceSolid;
use crate::artifacts::fem3d::diff::{Fem3dDiff, Fem3dSolidsDelta, Fem3dSolidsPatchEntry};
use crate::artifacts::fem3d::Fem3dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &ReplaceSolid, base: &Fem3dSnapshot) -> protocol::MutationOutcome<Fem3dDiff> {
    let Some(existing) = base.solids.iter().find(|solid| solid.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Solid \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if existing == &payload.new_solid {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Solid \"{}\" already has that value.", payload.id));
    }
    protocol::MutationOutcome::new(Fem3dDiff { solids: Some(Fem3dSolidsDelta { patched: vec![Fem3dSolidsPatchEntry { id: payload.id.clone(), item: payload.new_solid.clone() }], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
