//! 🔺️ Sparse diff builder for `DeleteObject` — re-materializes the pane's composed model child.
use super::DeleteObject;
use crate::diff::CadDiff;
use crate::CadSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &DeleteObject, base: &CadSnapshot) -> protocol::MutationOutcome<CadDiff> {
    let Some(scene) = crate::cad_pane_local_scene(base, payload.pane) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Object \"{}\" does not exist.", payload.object_id), [payload.object_id.clone()]);
    };
    let mut objects = crate::cad_scene_pane_objects(&scene, payload.pane).to_vec();
    let Some(index) = objects.iter().position(|object| object.id == payload.object_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Object \"{}\" does not exist.", payload.object_id), [payload.object_id.clone()]);
    };
    objects.remove(index);
    let mut diff = CadDiff::default();
    crate::cad_pane_child_diff_slot(&mut diff, payload.pane, crate::cad_pane_rematerialized_child(&scene, payload.pane, objects));
    protocol::MutationOutcome::new(diff)
}
//#endregion 🔖️Diff
