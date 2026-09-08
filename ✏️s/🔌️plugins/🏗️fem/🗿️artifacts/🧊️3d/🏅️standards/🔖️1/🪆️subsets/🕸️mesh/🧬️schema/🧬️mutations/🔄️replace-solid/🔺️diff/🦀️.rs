//! 🔺️ Sparse diff builder for `ReplaceSolid`.
use super::ReplaceSolid;
use crate::diff::{Fem3dDiff, Fem3dSolidsDelta, Fem3dSolidsPatchEntry};
use crate::mutations::{id_mismatch, invariant, solid_breach};
use crate::Fem3dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &ReplaceSolid, base: &Fem3dSnapshot) -> protocol::MutationOutcome<Fem3dDiff> {
    let Some(existing) = base.solids.iter().find(|solid| solid.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Solid \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if payload.new_solid.id != payload.id {
        return id_mismatch("Solid", &payload.id, &payload.new_solid.id);
    }
    if existing == &payload.new_solid {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Solid \"{}\" already has that value.", payload.id));
    }
    if !base.materials.iter().any(|material| material.id == payload.new_solid.material_id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Material \"{}\" does not exist.", payload.new_solid.material_id), [payload.new_solid.material_id.clone()]);
    }
    if let Some(breach) = solid_breach(&payload.new_solid) {
        return invariant(breach, vec![payload.id.clone()]);
    }
    protocol::MutationOutcome::new(Fem3dDiff { solids: Some(Fem3dSolidsDelta { patched: vec![Fem3dSolidsPatchEntry { id: payload.id.clone(), item: payload.new_solid.clone() }], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
