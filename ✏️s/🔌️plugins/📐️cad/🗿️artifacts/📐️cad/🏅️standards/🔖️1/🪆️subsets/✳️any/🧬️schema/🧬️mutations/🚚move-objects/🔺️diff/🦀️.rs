//! 🔺️ Sparse diff builder for `MoveObjects` — re-materializes the pane's composed model child.
use super::MoveObjects;
use crate::diff::CadDiff;
use crate::CadSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &MoveObjects, base: &CadSnapshot) -> protocol::MutationOutcome<CadDiff> {
    if payload.placements.iter().any(|placement| placement.new_origin.iter().any(|component| !component.is_finite())) {
        return protocol::MutationOutcome::fatal("mutation.invariant", "Object origins must be finite.".to_string(), payload.placements.iter().map(|placement| placement.object_id.clone()));
    }
    let Some(scene) = crate::cad_pane_local_scene(base, payload.pane) else {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Pane {:?} has no materialized model child to move objects in.", payload.pane));
    };
    let mut objects = crate::cad_scene_pane_objects(&scene, payload.pane).to_vec();
    let mut moved = 0usize;
    for placement in &payload.placements {
        if let Some(object) = objects.iter_mut().find(|object| object.id == placement.object_id) {
            if object.origin != placement.new_origin {
                object.origin = placement.new_origin;
                moved += 1;
            }
        }
    }
    if moved == 0 {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "No addressed object moved.".to_string());
    }
    let mut diff = CadDiff::default();
    crate::cad_pane_child_diff_slot(&mut diff, payload.pane, crate::cad_pane_rematerialized_child(&scene, payload.pane, objects));
    protocol::MutationOutcome::new(diff)
}
//#endregion 🔖️Diff
