//! 📍️ EN 1998 placements mutation text codec: one print and one parse function per mutation kind, routed by keyword from the `mutations` dispatcher.

#![allow(unused_imports)]

use semio_s_artifact_norm_en1998_mutations_text_kit::{dec_json, enc_json, opt_index, required, Args};
use semio_s_artifact_norm_en1998_mut_placements::change_placement_x::ChangePlacementX;
use semio_s_artifact_norm_en1998_mut_placements::change_placement_y::ChangePlacementY;
use semio_s_artifact_norm_en1998_mut_placements::change_placement_z::ChangePlacementZ;
use semio_s_artifact_norm_en1998_mut_placements::change_placement_yaw::ChangePlacementYaw;
use semio_s_artifact_norm_en1998_mut_placements::insert_placement::InsertPlacement;
use semio_s_artifact_norm_en1998_mut_placements::remove_placement::RemovePlacement;

/// ✍️ Prints `change-placement-x` as an op-text line.
pub fn print_change_placement_x(p: &ChangePlacementX) -> String {
    format!("change-placement-x id={} new-x={}", enc_json(&p.id), enc_json(&p.new_x))
}

/// 📖 Parses the arguments of `change-placement-x`.
pub fn parse_change_placement_x(args: &Args) -> Result<ChangePlacementX, String> {
    let arg = |k: &str| required(args, "change-placement-x", k);
    Ok(ChangePlacementX { id: dec_json(&arg("id")?)?, new_x: dec_json(&arg("new-x")?)? })
}

/// ✍️ Prints `change-placement-y` as an op-text line.
pub fn print_change_placement_y(p: &ChangePlacementY) -> String {
    format!("change-placement-y id={} new-y={}", enc_json(&p.id), enc_json(&p.new_y))
}

/// 📖 Parses the arguments of `change-placement-y`.
pub fn parse_change_placement_y(args: &Args) -> Result<ChangePlacementY, String> {
    let arg = |k: &str| required(args, "change-placement-y", k);
    Ok(ChangePlacementY { id: dec_json(&arg("id")?)?, new_y: dec_json(&arg("new-y")?)? })
}

/// ✍️ Prints `change-placement-z` as an op-text line.
pub fn print_change_placement_z(p: &ChangePlacementZ) -> String {
    format!("change-placement-z id={} new-z={}", enc_json(&p.id), enc_json(&p.new_z))
}

/// 📖 Parses the arguments of `change-placement-z`.
pub fn parse_change_placement_z(args: &Args) -> Result<ChangePlacementZ, String> {
    let arg = |k: &str| required(args, "change-placement-z", k);
    Ok(ChangePlacementZ { id: dec_json(&arg("id")?)?, new_z: dec_json(&arg("new-z")?)? })
}

/// ✍️ Prints `change-placement-yaw` as an op-text line.
pub fn print_change_placement_yaw(p: &ChangePlacementYaw) -> String {
    format!("change-placement-yaw id={} new-yaw={}", enc_json(&p.id), enc_json(&p.new_yaw))
}

/// 📖 Parses the arguments of `change-placement-yaw`.
pub fn parse_change_placement_yaw(args: &Args) -> Result<ChangePlacementYaw, String> {
    let arg = |k: &str| required(args, "change-placement-yaw", k);
    Ok(ChangePlacementYaw { id: dec_json(&arg("id")?)?, new_yaw: dec_json(&arg("new-yaw")?)? })
}

/// ✍️ Prints `insert-placement` as an op-text line.
pub fn print_insert_placement(p: &InsertPlacement) -> String {
    format!("insert-placement index={} item={}", enc_json(&p.index), enc_json(&p.item))
}

/// 📖 Parses the arguments of `insert-placement`.
pub fn parse_insert_placement(args: &Args) -> Result<InsertPlacement, String> {
    let arg = |k: &str| required(args, "insert-placement", k);
    Ok(InsertPlacement { index: opt_index(args)?, item: dec_json(&arg("item")?)? })
}

/// ✍️ Prints `remove-placement` as an op-text line.
pub fn print_remove_placement(p: &RemovePlacement) -> String {
    format!("remove-placement id={}", enc_json(&p.id))
}

/// 📖 Parses the arguments of `remove-placement`.
pub fn parse_remove_placement(args: &Args) -> Result<RemovePlacement, String> {
    let arg = |k: &str| required(args, "remove-placement", k);
    Ok(RemovePlacement { id: dec_json(&arg("id")?)? })
}
