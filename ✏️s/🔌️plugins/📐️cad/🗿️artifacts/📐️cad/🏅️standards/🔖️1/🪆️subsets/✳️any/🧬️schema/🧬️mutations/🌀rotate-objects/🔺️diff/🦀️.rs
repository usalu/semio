//! 🔺️ Sparse diff builder for `RotateObjects` — re-materializes the pane's composed model child.
use super::RotateObjects;
use crate::diff::CadDiff;
use crate::CadSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &RotateObjects, base: &CadSnapshot) -> protocol::MutationOutcome<CadDiff> {
    if payload.placements.iter().any(|placement| placement.new_orientation.iter().any(|component| !component.is_finite())) {
        return protocol::MutationOutcome::fatal("mutation.invariant", "Object orientations must be finite.".to_string(), payload.placements.iter().map(|placement| placement.object_id.clone()));
    }
    let Some(scene) = crate::cad_pane_local_scene(base, payload.pane) else {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Pane {:?} has no materialized model child to rotate objects in.", payload.pane));
    };
    let mut objects = crate::cad_scene_pane_objects(&scene, payload.pane).to_vec();
    let mut rotated = 0usize;
    for placement in &payload.placements {
        if let Some(object) = objects.iter_mut().find(|object| object.id == placement.object_id) {
            if object.orientation != Some(placement.new_orientation) {
                object.orientation = Some(placement.new_orientation);
                rotated += 1;
            }
        }
    }
    if rotated == 0 {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "No addressed object rotated.".to_string());
    }
    let mut diff = CadDiff::default();
    crate::cad_pane_child_diff_slot(&mut diff, payload.pane, crate::cad_pane_rematerialized_child(&scene, payload.pane, objects));
    protocol::MutationOutcome::new(diff)
}
//#endregion 🔖️Diff
