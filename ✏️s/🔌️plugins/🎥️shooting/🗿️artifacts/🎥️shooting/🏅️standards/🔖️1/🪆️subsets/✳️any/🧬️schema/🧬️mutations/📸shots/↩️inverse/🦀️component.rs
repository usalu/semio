//! ↩ Inverse constructors for the `shots` collection's mutation kinds — reconstructed from
//! captured BASE state. Missing target ⇒ `Vec::new()`.

use super::mutation::{ChangeShotFormat, ChangeShotHeight, ChangeShotShape, ChangeShotWidth, CreateShot, DeleteShot, RenameShot, ReorderShots};
use crate::artifacts::shooting::mutations::ShootingMutation;
use crate::artifacts::shooting::ShootingSnapshot;

//#region 🌱️CreateShot
pub fn inverse_create_shot(payload: &CreateShot, _base: &ShootingSnapshot) -> Vec<ShootingMutation> {
    vec![ShootingMutation::DeleteShot(DeleteShot { id: payload.shot.id.clone() })]
}
//#endregion 🌱️CreateShot

//#region 🗑️DeleteShot
pub fn inverse_delete_shot(payload: &DeleteShot, base: &ShootingSnapshot) -> Vec<ShootingMutation> {
    match base.shots.iter().position(|shot| shot.id == payload.id) {
        Some(index) => vec![ShootingMutation::CreateShot(CreateShot { shot: base.shots[index].clone(), index: Some(index) })],
        None => Vec::new(),
    }
}
//#endregion 🗑️DeleteShot

//#region ✏️RenameShot
pub fn inverse_rename_shot(payload: &RenameShot, base: &ShootingSnapshot) -> Vec<ShootingMutation> {
    match base.shots.iter().find(|shot| shot.id == payload.id) {
        Some(shot) => vec![ShootingMutation::RenameShot(RenameShot { id: payload.id.clone(), new_label: shot.label.clone() })],
        None => Vec::new(),
    }
}
//#endregion ✏️RenameShot

//#region 📐️ChangeShotWidth
pub fn inverse_change_shot_width(payload: &ChangeShotWidth, base: &ShootingSnapshot) -> Vec<ShootingMutation> {
    match base.shots.iter().find(|shot| shot.id == payload.id) {
        Some(shot) => vec![ShootingMutation::ChangeShotWidth(ChangeShotWidth { id: payload.id.clone(), new_width: shot.width })],
        None => Vec::new(),
    }
}
//#endregion 📐️ChangeShotWidth

//#region 📐️ChangeShotHeight
pub fn inverse_change_shot_height(payload: &ChangeShotHeight, base: &ShootingSnapshot) -> Vec<ShootingMutation> {
    match base.shots.iter().find(|shot| shot.id == payload.id) {
        Some(shot) => vec![ShootingMutation::ChangeShotHeight(ChangeShotHeight { id: payload.id.clone(), new_height: shot.height })],
        None => Vec::new(),
    }
}
//#endregion 📐️ChangeShotHeight

//#region 🖼️ChangeShotFormat
pub fn inverse_change_shot_format(payload: &ChangeShotFormat, base: &ShootingSnapshot) -> Vec<ShootingMutation> {
    match base.shots.iter().find(|shot| shot.id == payload.id) {
        Some(shot) => vec![ShootingMutation::ChangeShotFormat(ChangeShotFormat { id: payload.id.clone(), new_format: shot.format.clone() })],
        None => Vec::new(),
    }
}
//#endregion 🖼️ChangeShotFormat

//#region ✂️ChangeShotShape
pub fn inverse_change_shot_shape(payload: &ChangeShotShape, base: &ShootingSnapshot) -> Vec<ShootingMutation> {
    match base.shots.iter().find(|shot| shot.id == payload.id) {
        Some(shot) => vec![ShootingMutation::ChangeShotShape(ChangeShotShape { id: payload.id.clone(), new_shape: shot.shape.clone() })],
        None => Vec::new(),
    }
}
//#endregion ✂️ChangeShotShape

//#region 🔀️ReorderShots
pub fn inverse_reorder_shots(payload: &ReorderShots, base: &ShootingSnapshot) -> Vec<ShootingMutation> {
    match base.shots.iter().position(|shot| shot.id == payload.id) {
        Some(original_index) => vec![ShootingMutation::ReorderShots(ReorderShots { id: payload.id.clone(), to_index: original_index })],
        None => Vec::new(),
    }
}
//#endregion 🔀️ReorderShots
