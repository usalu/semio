//! 🔺️ Sparse diff builder for `CreateSolid`.
use super::CreateSolid;
use crate::standards::v1::subsets::any::schema::diff::{Fem3dDiff, Fem3dSolidInsertion, Fem3dSolidsDelta};
use crate::standards::v1::subsets::any::schema::mutations::{invariant,solid_breach};

use crate::Fem3dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &CreateSolid, base: &Fem3dSnapshot) -> protocol::MutationOutcome<Fem3dDiff> {
    if base.solids.iter().any(|solid| solid.id == payload.solid.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A solid with id \"{}\" already exists.", payload.solid.id), [payload.solid.id.clone()]);
    }
    if !base.materials.iter().any(|material| material.id == payload.solid.material_id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Material \"{}\" does not exist.", payload.solid.material_id), [payload.solid.material_id.clone()]);
    }
    if let Some(breach) = solid_breach(&payload.solid) {
        return invariant(breach, vec![payload.solid.id.clone()]);
    }
    if payload.index.is_some_and(|at| at > base.solids.len()) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Insert index {} is past the end of {} rows.", payload.index.unwrap_or_default(), base.solids.len()), [&payload.solid.id.to_string()]);
    }
    protocol::MutationOutcome::new(Fem3dDiff { solids: Some(Fem3dSolidsDelta { inserted: vec![Fem3dSolidInsertion { index: payload.index.unwrap_or(base.solids.len()), row: payload.solid.clone() }], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
