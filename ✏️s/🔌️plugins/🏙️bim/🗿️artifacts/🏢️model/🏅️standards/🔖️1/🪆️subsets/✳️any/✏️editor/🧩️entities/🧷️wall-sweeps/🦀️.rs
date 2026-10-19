//! 🧷️ The wall depth rows of the entity table: the wall sweep kind (a profile run along one face of a wall) with its pickers and its create default, the base slab and attached top rows of a wall (the pickers of the
//! slab its base follows and of the roof, slab or ceiling its top follows) and the reveal rows of an opening. Every write is a sparse `set-wall-sweep`, `set-wall-base-slab`, `set-wall-top` or `set-opening` mutation.

use semio_s_artifact_bim_model_inferences::standards::v1::subsets::any::schema::inferences::ModelInference;
use semio_s_artifact_bim_model_authored::editor::bim::entities::field_codecs::{first, number, parse_number, parse_optional_number, parse_profile, parse_text, partial, profile_text, variant};
use crate::editor::bim::entities::{ramps, rename_element, zoning};
use semio_s_artifact_bim_model_viewer::editor::bim::entities::create_rows::Created;
use semio_s_artifact_bim_model_viewer::editor::bim::entities::row_types::FieldRow;
use semio_s_artifact_bim_model_tests::editor::bim::terminology::BimLabels;
use semio_s_artifact_bim_model_mut_any_186::standards::v1::subsets::any::schema::mutations::create_wall_sweep::CreateWallSweep;
use semio_s_artifact_bim_model_mut_any_188::standards::v1::subsets::any::schema::mutations::set_wall_base_slab::SetWallBaseSlab;
use semio_s_artifact_bim_model_mut_any_174::standards::v1::subsets::any::schema::mutations::set_wall_top::SetWallTop;
use semio_s_artifact_bim_model_authored::standards::v1::subsets::any::schema::authored::references::top_target;
use semio_s_artifact_bim_model_authored::standards::v1::subsets::any::schema::diff::algebra::Assigned;
use semio_s_artifact_bim_model_mut_any_190::standards::v1::subsets::any::schema::mutations::ModelMutation;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::{ModelSnapshot, WallSweep};
use semio_s_artifact_bim_model_schema::standards::v1::subsets::any::schema::snapshot::values::{Profile, TopConstraint, WallSide};
use semio_framework_plugin::plugin_app_close_prelude::InputKind;
use semio_s_artifact_bim_model_mut_any_176::standards::v1::subsets::any::schema::mutations::set_opening;
use semio_s_artifact_bim_model_mut_any_187::standards::v1::subsets::any::schema::mutations::set_wall_sweep;

/// 📏️ How far a new sweep runs out of the wall face, in metres: a baseboard two centimetres deep.
pub const BASEBOARD_OUT: f64 = 0.02;
/// 📏️ How high a new sweep is, in metres: a baseboard ten centimetres high.
pub const BASEBOARD_UP: f64 = 0.1;

/// 🧷️ The profile of a new sweep: a baseboard section, `out of the wall` by `up`.
pub fn baseboard() -> Profile {
    Profile::Rectangle { width: BASEBOARD_OUT, depth: BASEBOARD_UP }
}

//#region 🔖️Choices
/// 🧱️ The walls a sweep can run along: every wall of the model by its id and name.
pub fn wall_choices(snapshot: &ModelSnapshot, _: &BimLabels) -> Vec<(String, String)> {
    snapshot.walls.iter().map(|(id, wall)| (id.clone(), wall.name.clone())).collect()
}

/// 🔽️ The slabs the base of a wall can follow: every slab of the model by its id and name (the properties panel adds the choice that frees the base).
pub fn slab_choices(snapshot: &ModelSnapshot, _: &BimLabels) -> Vec<(String, String)> {
    snapshot.slabs.iter().map(|(id, slab)| (id.clone(), slab.name.clone())).collect()
}

/// 🔝️ The surfaces the top of a wall can follow: every roof, slab and ceiling of the model by its id, its kind and its name (the properties panel adds the choice that frees the top).
pub fn surface_choices(snapshot: &ModelSnapshot, labels: &BimLabels) -> Vec<(String, String)> {
    let named = |kind: &str, id: &String, name: &String| (id.clone(), format!("{kind}: {name}"));
    let roofs = snapshot.roofs.iter().map(|(id, roof)| named(labels.kind_roof.as_str(), id, &roof.name));
    let slabs = snapshot.slabs.iter().map(|(id, slab)| named(labels.kind_slab.as_str(), id, &slab.name));
    let ceilings = snapshot.ceilings.iter().map(|(id, ceiling)| named(labels.kind_ceiling.as_str(), id, &ceiling.name));
    roofs.chain(slabs).chain(ceilings).collect()
}
//#endregion 🔖️Choices

//#region 🔖️Wall
/// 🔽️ The slab the base of a wall follows; empty when the base is free.
pub fn base_slab_text(snapshot: &ModelSnapshot, id: &str) -> Option<String> {
    snapshot.walls.get(id).map(|wall| wall.base_slab.clone().unwrap_or_default())
}

/// 🔝️ The roof, slab or ceiling the top of a wall follows; empty when the top is not attached.
pub fn top_attach_text(snapshot: &ModelSnapshot, id: &str) -> Option<String> {
    snapshot.walls.get(id).map(|wall| top_target(&wall.top).map(|(target, _)| target.to_string()).unwrap_or_default())
}

/// 🔽️ The mutation that makes the base of a wall follow the slab a text names (empty frees the base).
pub fn write_base_slab(snapshot: &ModelSnapshot, id: &str, value: &str) -> Option<ModelMutation> {
    let slab = value.trim();
    (slab.is_empty() || snapshot.slabs.contains_key(slab)).then(|| ModelMutation::SetWallBaseSlab(SetWallBaseSlab { id: id.into(), slab: (!slab.is_empty()).then(|| slab.to_string()) }))
}

/// 🔝️ The top that follows the roof, slab or ceiling `target` names, keeping the offset the wall already has over such a surface.
pub fn attached_top(snapshot: &ModelSnapshot, wall: &semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::Wall, target: &str) -> Option<TopConstraint> {
    let offset = top_target(&wall.top).map_or(0.0, |(_, offset)| offset);
    let target = target.trim().to_string();
    if snapshot.roofs.contains_key(&target) {
        Some(TopConstraint::Roof { roof: target, offset })
    } else if snapshot.slabs.contains_key(&target) {
        Some(TopConstraint::Slab { slab: target, offset })
    } else if snapshot.ceilings.contains_key(&target) {
        Some(TopConstraint::Ceiling { ceiling: target, offset })
    } else {
        None
    }
}

/// 🔝️ The mutation that attaches the top of a wall to the roof, slab or ceiling a text names, keeping the offset (empty frees the top: it returns to the top of its storey).
pub fn write_top_attach(snapshot: &ModelSnapshot, id: &str, value: &str) -> Option<ModelMutation> {
    let wall = snapshot.walls.get(id)?;
    let top = if value.trim().is_empty() { TopConstraint::StoreyTop { offset: 0.0 } } else { attached_top(snapshot, wall, value)? };
    Some(ModelMutation::SetWallTop(SetWallTop { id: id.into(), top }))
}
//#endregion 🔖️Wall

//#region 🔖️Reveal
/// 🪟️ The depth of the authored reveal of an opening; empty when the frame is centred in the wall.
pub fn reveal_depth_text(snapshot: &ModelSnapshot, id: &str) -> Option<String> {
    snapshot.openings.get(id).map(|opening| opening.reveal_depth.map(number).unwrap_or_default())
}

/// 🪟️ The material of the authored reveal of an opening; empty when the reveal has no material of its own.
pub fn reveal_material_text(snapshot: &ModelSnapshot, id: &str) -> Option<String> {
    snapshot.openings.get(id).map(|opening| opening.reveal_material.clone().unwrap_or_default())
}

/// 🪟️ The mutation that sets the reveal depth of an opening (empty clears the reveal).
pub fn write_reveal_depth(_: &ModelSnapshot, id: &str, value: &str) -> Option<ModelMutation> {
    set!(set_opening::SetOpening, id, "reveal_depth", &Assigned::new(parse_optional_number(value)?))
}

/// 🪟️ The mutation that sets the material of the reveal of an opening (empty clears it).
pub fn write_reveal_material(_: &ModelSnapshot, id: &str, value: &str) -> Option<ModelMutation> {
    let material = value.trim();
    set!(set_opening::SetOpening, id, "reveal_material", &Assigned::new((!material.is_empty()).then(|| material.to_string())))
}
//#endregion 🔖️Reveal

//#region 🔖️Sweep
fn parse_side(text: &str) -> Option<WallSide> {
    variant(text, &[WallSide::Left, WallSide::Right])
}

/// 🧾️ The authored parameters of a wall sweep.
pub static SWEEP_FIELDS: &[FieldRow] = &[
    field!("name", field_name, Text, |s, id| s.wall_sweeps.get(id).map(|row| row.name.clone()), rename),
    field!("host", field_host, Text, |s, id| s.wall_sweeps.get(id).map(|row| row.host.clone()), choices: wall_choices, parse_text => set_wall_sweep::SetWallSweep),
    field!("side", field_side, Text, |s, id| s.wall_sweeps.get(id).map(|row| format!("{:?}", row.side)), choices: ramps::side_choices, parse_side => set_wall_sweep::SetWallSweep),
    field!("profile", field_profile, Text, |s, id| s.wall_sweeps.get(id).map(|row| profile_text(&row.profile)), parse_profile => set_wall_sweep::SetWallSweep),
    field!("height", field_sweep_height, Number, |s, id| s.wall_sweeps.get(id).map(|row| number(row.height)), parse_number => set_wall_sweep::SetWallSweep),
    field!("inset", field_inset, Number, |s, id| s.wall_sweeps.get(id).map(|row| number(row.inset)), parse_number => set_wall_sweep::SetWallSweep),
    field!("material", field_material, Text, |s, id| s.wall_sweeps.get(id).map(|row| row.material.clone()), choices: zoning::material_choices, parse_text => set_wall_sweep::SetWallSweep),
];

/// 🧷️ A new wall sweep: a baseboard along the left face of the wall it is created in (else the first wall of the model), standing on the wall base, in the first material of the project.
pub fn create_wall_sweep(snapshot: &ModelSnapshot, id: &str, parent: &str, name: &str) -> Created {
    let host = snapshot.walls.contains_key(parent).then(|| parent.to_string()).or_else(|| first(&snapshot.walls)).ok_or("bim.create.wall-missing")?;
    let material = first(&snapshot.materials).ok_or("bim.create.material-missing")?;
    Ok(ModelMutation::CreateWallSweep(CreateWallSweep { id: id.into(), wall_sweep: WallSweep { host, side: WallSide::Left, profile: baseboard(), height: 0.0, inset: 0.0, material, name: name.into() } }))
}
//#endregion 🔖️Sweep


