//! 🔲️ The ceiling rows of the entity table: how a ceiling and a ceiling type read off the snapshot, how an edited value becomes a `set-ceiling`, `set-ceiling-boundary` or `set-ceiling-type` mutation,
//! what a new ceiling or ceiling type is, and the picker of the ceiling type row.

use super::{container, first, holes_text, layers, loop_text, number, parse_holes, parse_loop, parse_number, parse_text, partial, rectangle, rename_element, Created, FieldRow};
use crate::editor::bim::terminology::BimLabels;
use crate::{Assigned, ModelMutation, ModelSnapshot, Slope};
use semio_framework_plugin::plugin_app_close_prelude::InputKind;

/// 📏️ How far a drawn ceiling hangs below the storey top, in metres.
pub const DEFAULT_DROP: f64 = 0.3;

//#region 🔖️Choices
/// 🔲️ The ceiling types a ceiling can take: every type of the library by its id and name.
pub fn type_choices(snapshot: &ModelSnapshot, _: &BimLabels) -> Vec<(String, String)> {
    snapshot.ceiling_types.iter().map(|(id, kind)| (id.clone(), kind.name.clone())).collect()
}
//#endregion 🔖️Choices

//#region 🔖️Writes
/// 📐️ The text of a slope: its fall direction and its angle in radians, empty when the ceiling is level.
pub fn slope_text(slope: Option<Slope>) -> String {
    slope.map_or_else(String::new, |slope| format!("{}, {}", number(slope.direction), number(slope.angle)))
}

/// 📐️ The slope a text names: `direction, angle` in radians, empty for none; text that is neither is no slope at all.
pub fn parse_slope(text: &str) -> Option<Option<Slope>> {
    let text = text.trim();
    if text.is_empty() {
        return Some(None);
    }
    let (direction, angle) = text.split_once(',')?;
    Some(Some(Slope { direction: parse_number(direction)?, angle: parse_number(angle)? }))
}

fn write_slope(_: &ModelSnapshot, id: &str, value: &str) -> Option<ModelMutation> {
    partial::<crate::mutations::set_ceiling::SetCeiling, _>(id, "slope", &Assigned::new(parse_slope(value)?)).map(ModelMutation::SetCeiling)
}

fn write_boundary(snapshot: &ModelSnapshot, id: &str, value: &str) -> Option<ModelMutation> {
    Some(ModelMutation::SetCeilingBoundary(crate::mutations::set_ceiling_boundary::SetCeilingBoundary { id: id.into(), boundary: parse_loop(value)?, holes: snapshot.ceilings.get(id)?.holes.clone() }))
}

fn write_holes(snapshot: &ModelSnapshot, id: &str, value: &str) -> Option<ModelMutation> {
    Some(ModelMutation::SetCeilingBoundary(crate::mutations::set_ceiling_boundary::SetCeilingBoundary { id: id.into(), boundary: snapshot.ceilings.get(id)?.boundary.clone(), holes: parse_holes(value)? }))
}
//#endregion 🔖️Writes

//#region 🔖️Fields
/// 🧾️ The authored parameters of a ceiling.
pub static CEILING_FIELDS: &[FieldRow] = &[
    field!("name", field_name, Text, |s, id| s.ceilings.get(id).map(|row| row.name.clone()), rename),
    field!("storey", field_storey, Text, |s, id| s.ceilings.get(id).map(|row| row.storey.clone())),
    field!("ceiling_type", field_ceiling_type, Text, |s, id| s.ceilings.get(id).map(|row| row.ceiling_type.clone()), choices: type_choices, parse_text => set_ceiling::SetCeiling),
    field!("offset", field_drop, Number, |s, id| s.ceilings.get(id).map(|row| number(row.offset)), parse_number => set_ceiling::SetCeiling),
    field!("slope", field_slope, Text, |s, id| s.ceilings.get(id).map(|row| slope_text(row.slope)), |s, id, value| write_slope(s, id, value)),
    field!("boundary", field_boundary, Text, |s, id| s.ceilings.get(id).map(|row| loop_text(&row.boundary)), |s, id, value| write_boundary(s, id, value)),
    field!("holes", field_holes, Text, |s, id| s.ceilings.get(id).map(|row| holes_text(&row.holes)), |s, id, value| write_holes(s, id, value)),
];

/// 🧾️ The authored parameters of a ceiling type.
pub static CEILING_TYPE_FIELDS: &[FieldRow] = &[
    field!("name", field_name, Text, |s, id| s.ceiling_types.get(id).map(|row| row.name.clone()), parse_text => set_ceiling_type::SetCeilingType),
    field!("layers", field_layers, Text, |s, id| s.ceiling_types.get(id).map(|row| row.layers.len().to_string())),
    field!("thickness", field_thickness, Number, |s, id| s.ceiling_types.get(id).map(|row| number(row.layers.iter().map(|layer| layer.thickness).sum()))),
];
//#endregion 🔖️Fields

//#region 🔖️Create
/// 🔲️ A new ceiling: a 4 m square hung below the storey top, of the first ceiling type of the library.
pub fn create_ceiling(snapshot: &ModelSnapshot, id: &str, parent: &str, name: &str) -> Created {
    let storey = container(&snapshot.storeys, parent, "bim.create.storey-missing")?;
    let ceiling_type = first(&snapshot.ceiling_types).ok_or("bim.create.ceiling-type-missing")?;
    Ok(ModelMutation::CreateCeiling(crate::mutations::create_ceiling::CreateCeiling {
        id: id.into(),
        ceiling: crate::Ceiling { storey, ceiling_type, boundary: rectangle(4.0, 4.0), holes: Vec::new(), offset: DEFAULT_DROP, slope: None, name: name.into() },
    }))
}

/// 🔲️ A new ceiling type: one 12.5 mm layer of the first material of the project.
pub fn create_ceiling_type(snapshot: &ModelSnapshot, id: &str, _parent: &str, name: &str) -> Created {
    let mut stack = layers(snapshot)?;
    stack.iter_mut().for_each(|layer| layer.thickness = 0.0125);
    Ok(ModelMutation::CreateCeilingType(crate::mutations::create_ceiling_type::CreateCeilingType { id: id.into(), ceiling_type: crate::CeilingType { name: name.into(), layers: stack } }))
}
//#endregion 🔖️Create
