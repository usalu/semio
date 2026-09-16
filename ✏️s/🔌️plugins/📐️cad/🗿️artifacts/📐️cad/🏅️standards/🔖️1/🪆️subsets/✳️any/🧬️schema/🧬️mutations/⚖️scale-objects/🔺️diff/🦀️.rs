//! 🔺️ Sparse diff builder for `ScaleObjects` — re-materializes the pane's composed model child.
use super::ScaleObjects;
use crate::diff::CadDiff;
use crate::CadSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &ScaleObjects, base: &CadSnapshot) -> protocol::MutationOutcome<CadDiff> {
    if payload.placements.iter().any(|placement| placement.new_scale.iter().any(|component| !component.is_finite() || *component == 0.0)) {
        return protocol::MutationOutcome::fatal("mutation.invariant", "Object scales must be finite and nonzero.".to_string(), payload.placements.iter().map(|placement| placement.object_id.clone()));
    }
    let Some(scene) = crate::cad_pane_local_scene(base, payload.pane) else {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Pane {:?} has no materialized model child to scale objects in.", payload.pane));
    };
    let mut objects = crate::cad_scene_pane_objects(&scene, payload.pane).to_vec();
    let mut scaled = 0usize;
    for placement in &payload.placements {
        if let Some(object) = objects.iter_mut().find(|object| object.id == placement.object_id) {
            if object.scale != Some(placement.new_scale) {
                object.scale = Some(placement.new_scale);
                scaled += 1;
            }
        }
    }
    if scaled == 0 {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "No addressed object scaled.".to_string());
    }
    let mut diff = CadDiff::default();
    crate::cad_pane_child_diff_slot(&mut diff, payload.pane, crate::cad_pane_rematerialized_child(&scene, payload.pane, objects));
    protocol::MutationOutcome::new(diff)
}
//#endregion 🔖️Diff
