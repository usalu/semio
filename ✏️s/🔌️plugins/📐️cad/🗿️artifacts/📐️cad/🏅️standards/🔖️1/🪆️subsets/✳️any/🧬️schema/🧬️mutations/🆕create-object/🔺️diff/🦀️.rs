//! 🔺️ Sparse diff builder for `CreateObject` — re-materializes the pane's composed model child.
use super::CreateObject;
use crate::diff::CadDiff;
use crate::mutations::cad_object_from_spec;
use crate::{CadSnapshot, CadWorkingScene};

//#region 🔖️Diff
pub fn diff(payload: &CreateObject, base: &CadSnapshot) -> protocol::MutationOutcome<CadDiff> {
    if payload.object.id.is_empty() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "An object needs a non-empty id.".to_string(), [payload.object.id.clone()]);
    }
    // 🪆️ An EMPTY pane admits its first object from an empty working scene. A pane whose slot is
    // occupied but NOT materialized (a wire-decoded handle, whose `local_owner` no codec carries)
    // must refuse: re-minting from an empty scene would silently discard whatever that child holds.
    let scene = match crate::cad_pane_local_scene(base, payload.pane) {
        Some(scene) => scene,
        None if crate::cad_pane_model(base, payload.pane).is_none() => std::sync::Arc::new(CadWorkingScene::default()),
        None => return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Pane {:?} holds an unresolved model child; creating into it would discard its content.", payload.pane)),
    };
    let mut objects = crate::cad_scene_pane_objects(&scene, payload.pane).to_vec();
    if objects.iter().any(|object| object.id == payload.object.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("An object with id \"{}\" already exists in this pane.", payload.object.id), [payload.object.id.clone()]);
    }
    let index = (payload.index as usize).min(objects.len());
    objects.insert(index, cad_object_from_spec(&payload.object, &payload.primitives));
    let mut diff = CadDiff::default();
    crate::cad_pane_child_diff_slot(&mut diff, payload.pane, crate::cad_pane_rematerialized_child(&scene, payload.pane, objects));
    protocol::MutationOutcome::new(diff)
}
//#endregion 🔖️Diff
