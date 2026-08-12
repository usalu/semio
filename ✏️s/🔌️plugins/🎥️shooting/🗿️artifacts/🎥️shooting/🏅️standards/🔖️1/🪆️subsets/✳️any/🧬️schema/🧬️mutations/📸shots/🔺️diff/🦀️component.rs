//! 🔺 Diff constructors for the `shots` collection's mutation kinds — each builds [`ShootingDiff`]
//! sparsely and directly from its payload.

use super::mutation::{ChangeShotFormat, ChangeShotHeight, ChangeShotShape, ChangeShotWidth, CreateShot, DeleteShot, RenameShot, ReorderShots};
use crate::artifacts::shooting::diff::{ShootingDiff, ShootingShotPatchEntry, ShootingShotsDelta};
use crate::artifacts::shooting::{ShootingShotPatch, ShootingSnapshot};

//#region 🌱️CreateShot
pub fn diff_create_shot(payload: &CreateShot, _base: &ShootingSnapshot) -> ShootingDiff {
    ShootingDiff { shots: Some(ShootingShotsDelta { added: vec![payload.shot.clone()], ..Default::default() }), ..Default::default() }
}
//#endregion 🌱️CreateShot

//#region 🗑️DeleteShot
pub fn diff_delete_shot(payload: &DeleteShot, _base: &ShootingSnapshot) -> ShootingDiff {
    ShootingDiff { shots: Some(ShootingShotsDelta { removed: vec![payload.id.clone()], ..Default::default() }), ..Default::default() }
}
//#endregion 🗑️DeleteShot

//#region ✏️RenameShot
pub fn diff_rename_shot(payload: &RenameShot, _base: &ShootingSnapshot) -> ShootingDiff {
    ShootingDiff {
        shots: Some(ShootingShotsDelta { patched: vec![ShootingShotPatchEntry { id: payload.id.clone(), patch: ShootingShotPatch { label: Some(payload.new_label.clone()), ..Default::default() } }], ..Default::default() }),
        ..Default::default()
    }
}
//#endregion ✏️RenameShot

//#region 📐️ChangeShotWidth
pub fn diff_change_shot_width(payload: &ChangeShotWidth, _base: &ShootingSnapshot) -> ShootingDiff {
    ShootingDiff {
        shots: Some(ShootingShotsDelta { patched: vec![ShootingShotPatchEntry { id: payload.id.clone(), patch: ShootingShotPatch { width: Some(payload.new_width), ..Default::default() } }], ..Default::default() }),
        ..Default::default()
    }
}
//#endregion 📐️ChangeShotWidth

//#region 📐️ChangeShotHeight
pub fn diff_change_shot_height(payload: &ChangeShotHeight, _base: &ShootingSnapshot) -> ShootingDiff {
    ShootingDiff {
        shots: Some(ShootingShotsDelta { patched: vec![ShootingShotPatchEntry { id: payload.id.clone(), patch: ShootingShotPatch { height: Some(payload.new_height), ..Default::default() } }], ..Default::default() }),
        ..Default::default()
    }
}
//#endregion 📐️ChangeShotHeight

//#region 🖼️ChangeShotFormat
pub fn diff_change_shot_format(payload: &ChangeShotFormat, _base: &ShootingSnapshot) -> ShootingDiff {
    ShootingDiff {
        shots: Some(ShootingShotsDelta { patched: vec![ShootingShotPatchEntry { id: payload.id.clone(), patch: ShootingShotPatch { format: Some(payload.new_format.clone()), ..Default::default() } }], ..Default::default() }),
        ..Default::default()
    }
}
//#endregion 🖼️ChangeShotFormat

//#region ✂️ChangeShotShape
pub fn diff_change_shot_shape(payload: &ChangeShotShape, _base: &ShootingSnapshot) -> ShootingDiff {
    ShootingDiff {
        shots: Some(ShootingShotsDelta { patched: vec![ShootingShotPatchEntry { id: payload.id.clone(), patch: ShootingShotPatch { shape: Some(payload.new_shape.clone()), ..Default::default() } }], ..Default::default() }),
        ..Default::default()
    }
}
//#endregion ✂️ChangeShotShape

//#region 🔀️ReorderShots
pub fn diff_reorder_shots(payload: &ReorderShots, base: &ShootingSnapshot) -> ShootingDiff {
    let mut ids: Vec<String> = base.shots.iter().map(|shot| shot.id.clone()).collect();
    if let Some(from) = ids.iter().position(|id| id == &payload.id) {
        let item = ids.remove(from);
        let to = payload.to_index.min(ids.len());
        ids.insert(to, item);
    }
    ShootingDiff { shots: Some(ShootingShotsDelta { reordered: Some(ids), ..Default::default() }), ..Default::default() }
}
//#endregion 🔀️ReorderShots
