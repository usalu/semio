//! 🏗️ The frame rows of the entity table: how a leaning column, an arc or inclined beam, a curtain wall, a curtain wall type and a curtain panel override read off the snapshot, how an edited value
//! becomes a `set-beam-axis`, `set-column-tilt`, `set-curtain-wall-grid`, `set-curtain-wall-type-of`, `set-curtain-wall-type` or `set-curtain-panel-override` mutation, what a new curtain wall type
//! or panel override is, and the pickers (curtain wall types, materials, door and window types) of the reference rows. The grid and the panels are text: `spacing 1.5` or `lines 1, 2.5`, and
//! `glass`, `empty`, `solid material`, `door type` or `window type`.

use super::ceilings::{parse_slope, slope_text};
use super::{first, number, parse_number, parse_optional_number, parse_profile, parse_text, partial, profile_text, Created, EntityKind, FieldRow, InferredRow};
use crate::editor::bim::terminology::BimLabels;
use crate::{Assigned, CurtainGrid, CurtainPanel, CurtainPanelOverride, CurtainWallType, ModelMutation, ModelSnapshot, Profile};
use semio_framework_plugin::plugin_app_close_prelude::InputKind;

//#region 🔖️Text
/// 🕸️ The text of a grid rule: `spacing s` or `lines a, b, c`.
pub fn grid_text(grid: &CurtainGrid) -> String {
    match grid {
        CurtainGrid::Spacing { spacing } => format!("spacing {}", number(*spacing)),
        CurtainGrid::Lines { positions } => format!("lines {}", positions.iter().map(|position| number(*position)).collect::<Vec<_>>().join(", ")),
    }
}

/// 🕸️ The grid rule a text names: `spacing s` or `lines a, b, c` (the lines may be empty); anything else is no rule.
pub fn parse_grid(text: &str) -> Option<CurtainGrid> {
    let text = text.trim();
    if let Some(rest) = text.strip_prefix("spacing") {
        return parse_number(rest).map(|spacing| CurtainGrid::Spacing { spacing });
    }
    let rest = text.strip_prefix("lines")?;
    let positions = rest.split(',').map(str::trim).filter(|part| !part.is_empty()).map(parse_number).collect::<Option<Vec<_>>>()?;
    Some(CurtainGrid::Lines { positions })
}

/// 🕸️ The text of the grid override of a curtain wall: the rule it carries, `type` while it follows its type.
pub fn override_text(grid: Option<&CurtainGrid>) -> String {
    grid.map_or_else(|| "type".to_string(), grid_text)
}

/// 🕸️ The grid override a text names: a rule, or `type` (or nothing) to follow the curtain wall type again.
pub fn parse_override(text: &str) -> Option<Option<CurtainGrid>> {
    let text = text.trim();
    if text.is_empty() || text.eq_ignore_ascii_case("type") {
        return Some(None);
    }
    parse_grid(text).map(Some)
}

/// 🪟️ The text of a panel: `glass`, `empty`, `solid material`, `door type` or `window type`.
pub fn panel_text(panel: &CurtainPanel) -> String {
    match panel {
        CurtainPanel::Glass => "glass".to_string(),
        CurtainPanel::Empty => "empty".to_string(),
        CurtainPanel::Solid { material } => format!("solid {material}"),
        CurtainPanel::Door { door_type } => format!("door {door_type}"),
        CurtainPanel::Window { window_type } => format!("window {window_type}"),
    }
}

/// 🪟️ The panel a text names (see [`panel_text`]).
pub fn parse_panel(text: &str) -> Option<CurtainPanel> {
    let text = text.trim();
    let (keyword, rest) = text.split_once(' ').map_or((text, ""), |(keyword, rest)| (keyword, rest.trim()));
    match (keyword.to_ascii_lowercase().as_str(), rest) {
        ("glass", "") => Some(CurtainPanel::Glass),
        ("empty", "") => Some(CurtainPanel::Empty),
        ("solid", material) if !material.is_empty() => Some(CurtainPanel::Solid { material: material.to_string() }),
        ("door", door_type) if !door_type.is_empty() => Some(CurtainPanel::Door { door_type: door_type.to_string() }),
        ("window", window_type) if !window_type.is_empty() => Some(CurtainPanel::Window { window_type: window_type.to_string() }),
        _ => None,
    }
}
//#endregion 🔖️Text

//#region 🔖️Choices
/// 🏬️ The curtain wall types a curtain wall can take: every type of the library by its id and name.
pub fn type_choices(snapshot: &ModelSnapshot, _: &BimLabels) -> Vec<(String, String)> {
    snapshot.curtain_wall_types.iter().map(|(id, kind)| (id.clone(), kind.name.clone())).collect()
}

/// 🎨️ The materials a curtain wall type can name: every material of the model by its id and name.
pub fn material_choices(snapshot: &ModelSnapshot, _: &BimLabels) -> Vec<(String, String)> {
    snapshot.materials.iter().map(|(id, material)| (id.clone(), material.name.clone())).collect()
}
//#endregion 🔖️Choices

//#region 🔖️Writes
/// 📐️ Sets the tilt of a column: `direction, angle` in radians, empty for a plumb column.
pub fn write_tilt(_: &ModelSnapshot, id: &str, value: &str) -> Option<ModelMutation> {
    Some(ModelMutation::SetColumnTilt(crate::mutations::set_column_tilt::SetColumnTilt { id: id.into(), tilt: parse_slope(value)? }))
}

/// 📐️ Sets the axis of a beam: `x, y → x, y` for a line, `x, y → x, y ⌒ bulge` for an arc.
pub fn write_beam_axis(_: &ModelSnapshot, id: &str, value: &str) -> Option<ModelMutation> {
    Some(ModelMutation::SetBeamAxis(crate::mutations::set_beam_axis::SetBeamAxis { id: id.into(), axis: super::parse_axis(value)? }))
}

/// 📐️ Sets the top offset of the end of a beam: empty makes the beam level again.
pub fn write_end_offset(_: &ModelSnapshot, id: &str, value: &str) -> Option<ModelMutation> {
    partial::<crate::mutations::set_beam::SetBeam, _>(id, "end_top_offset", &Assigned::new(parse_optional_number(value)?)).map(ModelMutation::SetBeam)
}

/// 🏬️ Builds a curtain wall as another curtain wall type.
pub fn write_curtain_wall_type(_: &ModelSnapshot, id: &str, value: &str) -> Option<ModelMutation> {
    Some(ModelMutation::SetCurtainWallTypeOf(crate::mutations::set_curtain_wall_type_of::SetCurtainWallTypeOf { id: id.into(), curtain_wall_type: value.trim().to_string() }))
}

/// 🕸️ Sets the grid override along the wall; `type` or nothing follows the type again.
pub fn write_u_grid(_: &ModelSnapshot, id: &str, value: &str) -> Option<ModelMutation> {
    partial::<crate::mutations::set_curtain_wall_grid::SetCurtainWallGrid, _>(id, "u_grid", &Assigned::new(parse_override(value)?)).map(ModelMutation::SetCurtainWallGrid)
}

/// 🕸️ Sets the grid override up the wall; `type` or nothing follows the type again.
pub fn write_v_grid(_: &ModelSnapshot, id: &str, value: &str) -> Option<ModelMutation> {
    partial::<crate::mutations::set_curtain_wall_grid::SetCurtainWallGrid, _>(id, "v_grid", &Assigned::new(parse_override(value)?)).map(ModelMutation::SetCurtainWallGrid)
}

fn type_field<T: semio_framework_value::ToValue + ?Sized>(id: &str, field: &str, value: &T) -> Option<ModelMutation> {
    partial::<crate::mutations::set_curtain_wall_type::SetCurtainWallType, _>(id, field, value).map(ModelMutation::SetCurtainWallType)
}

fn write_type_name(_: &ModelSnapshot, id: &str, value: &str) -> Option<ModelMutation> {
    type_field(id, "name", &parse_text(value)?)
}

fn write_type_u_grid(_: &ModelSnapshot, id: &str, value: &str) -> Option<ModelMutation> {
    type_field(id, "u_grid", &parse_grid(value)?)
}

fn write_type_v_grid(_: &ModelSnapshot, id: &str, value: &str) -> Option<ModelMutation> {
    type_field(id, "v_grid", &parse_grid(value)?)
}

fn write_type_interior(_: &ModelSnapshot, id: &str, value: &str) -> Option<ModelMutation> {
    type_field(id, "interior_mullion", &parse_profile(value)?)
}

fn write_type_border(_: &ModelSnapshot, id: &str, value: &str) -> Option<ModelMutation> {
    type_field(id, "border_mullion", &parse_profile(value)?)
}

fn write_type_panel(_: &ModelSnapshot, id: &str, value: &str) -> Option<ModelMutation> {
    type_field(id, "panel", &parse_panel(value)?)
}

fn write_type_glass(_: &ModelSnapshot, id: &str, value: &str) -> Option<ModelMutation> {
    type_field(id, "panel_material", &parse_text(value)?)
}

fn write_type_mullion_material(_: &ModelSnapshot, id: &str, value: &str) -> Option<ModelMutation> {
    type_field(id, "mullion_material", &parse_text(value)?)
}

fn write_override_panel(_: &ModelSnapshot, id: &str, value: &str) -> Option<ModelMutation> {
    Some(ModelMutation::SetCurtainPanelOverride(crate::mutations::set_curtain_panel_override::SetCurtainPanelOverride { id: id.into(), panel: parse_panel(value)? }))
}
//#endregion 🔖️Writes

//#region 🔖️Fields
/// 🧾️ The authored parameters of a curtain wall type.
pub static CURTAIN_WALL_TYPE_FIELDS: &[FieldRow] = &[
    FieldRow { key: "name", label: |labels| labels.field_name, input: InputKind::Text, read: |s, id| s.curtain_wall_types.get(id).map(|row| row.name.clone()), write: Some(write_type_name), choices: None },
    FieldRow { key: "u_grid", label: |labels| labels.field_u_grid, input: InputKind::Text, read: |s, id| s.curtain_wall_types.get(id).map(|row| grid_text(&row.u_grid)), write: Some(write_type_u_grid), choices: None },
    FieldRow { key: "v_grid", label: |labels| labels.field_v_grid, input: InputKind::Text, read: |s, id| s.curtain_wall_types.get(id).map(|row| grid_text(&row.v_grid)), write: Some(write_type_v_grid), choices: None },
    FieldRow { key: "interior_mullion", label: |labels| labels.field_interior_mullion, input: InputKind::Text, read: |s, id| s.curtain_wall_types.get(id).map(|row| profile_text(&row.interior_mullion)), write: Some(write_type_interior), choices: None },
    FieldRow { key: "border_mullion", label: |labels| labels.field_border_mullion, input: InputKind::Text, read: |s, id| s.curtain_wall_types.get(id).map(|row| profile_text(&row.border_mullion)), write: Some(write_type_border), choices: None },
    FieldRow { key: "panel", label: |labels| labels.field_panel, input: InputKind::Text, read: |s, id| s.curtain_wall_types.get(id).map(|row| panel_text(&row.panel)), write: Some(write_type_panel), choices: None },
    FieldRow { key: "panel_material", label: |labels| labels.field_panel_material, input: InputKind::Text, read: |s, id| s.curtain_wall_types.get(id).map(|row| row.panel_material.clone()), write: Some(write_type_glass), choices: Some(material_choices) },
    FieldRow { key: "mullion_material", label: |labels| labels.field_mullion_material, input: InputKind::Text, read: |s, id| s.curtain_wall_types.get(id).map(|row| row.mullion_material.clone()), write: Some(write_type_mullion_material), choices: Some(material_choices) },
    FieldRow { key: "u_value", label: |labels| labels.field_u_value, input: InputKind::Number, read: |s, id| s.curtain_wall_types.get(id).map(|row| row.u_value.map_or_else(String::new, number)), write: Some(|_, id, value| partial::<crate::mutations::set_type_thermal_data::SetTypeThermalData, _>(id, "u_value", &Assigned::new(parse_optional_number(value)?)).map(ModelMutation::SetTypeThermalData)), choices: None },
    FieldRow { key: "g_value", label: |labels| labels.field_g_value, input: InputKind::Number, read: |s, id| s.curtain_wall_types.get(id).map(|row| row.g_value.map_or_else(String::new, number)), write: Some(|_, id, value| partial::<crate::mutations::set_type_thermal_data::SetTypeThermalData, _>(id, "g_value", &Assigned::new(parse_optional_number(value)?)).map(ModelMutation::SetTypeThermalData)), choices: None },
    FieldRow { key: "frame_fraction", label: |labels| labels.field_frame_fraction, input: InputKind::Number, read: |s, id| s.curtain_wall_types.get(id).map(|row| row.frame_fraction.map_or_else(String::new, number)), write: Some(|_, id, value| partial::<crate::mutations::set_type_thermal_data::SetTypeThermalData, _>(id, "frame_fraction", &Assigned::new(parse_optional_number(value)?)).map(ModelMutation::SetTypeThermalData)), choices: None },
];

/// 🧾️ The authored parameters of a curtain panel override: the cell it addresses is fixed, the panel is not.
pub static CURTAIN_PANEL_OVERRIDE_FIELDS: &[FieldRow] = &[
    FieldRow { key: "curtain", label: |labels| labels.field_curtain, input: InputKind::Text, read: |s, id| s.curtain_panel_overrides.get(id).map(|row| row.curtain.clone()), write: None, choices: None },
    FieldRow { key: "u", label: |labels| labels.field_cell_u, input: InputKind::Number, read: |s, id| s.curtain_panel_overrides.get(id).map(|row| row.u.to_string()), write: None, choices: None },
    FieldRow { key: "v", label: |labels| labels.field_cell_v, input: InputKind::Number, read: |s, id| s.curtain_panel_overrides.get(id).map(|row| row.v.to_string()), write: None, choices: None },
    FieldRow { key: "panel", label: |labels| labels.field_panel, input: InputKind::Text, read: |s, id| s.curtain_panel_overrides.get(id).map(|row| panel_text(&row.panel)), write: Some(write_override_panel), choices: None },
];

/// 🪟️ What a curtain wall type shows now: how many curtain walls use it.
pub static CURTAIN_WALL_TYPE_INFERRED: &[InferredRow] = &[InferredRow { key: "used_by", label: |labels| labels.field_used_by, read: |s, _, id| Some(s.curtain_walls.values().filter(|row| row.curtain_wall_type == id).count().to_string()) }];
//#endregion 🔖️Fields

//#region 🔖️Create
/// 🏬️ A new curtain wall type: equal cells of 1.5 m, 5 cm by 10 cm interior and 8 cm by 10 cm border mullions, glass in every cell, in the first material of the model.
pub fn create_curtain_wall_type(snapshot: &ModelSnapshot, id: &str, _parent: &str, name: &str) -> Created {
    let material = first(&snapshot.materials).ok_or("bim.create.material-missing")?;
    Ok(ModelMutation::CreateCurtainWallType(crate::mutations::create_curtain_wall_type::CreateCurtainWallType {
        id: id.into(),
        curtain_wall_type: CurtainWallType {
            name: name.into(),
            u_grid: CurtainGrid::Spacing { spacing: 1.5 },
            v_grid: CurtainGrid::Spacing { spacing: 1.5 },
            interior_mullion: Profile::Rectangle { width: 0.05, depth: 0.1 },
            border_mullion: Profile::Rectangle { width: 0.08, depth: 0.1 },
            panel: CurtainPanel::Glass,
            panel_material: material.clone(),
            mullion_material: material,
            u_value: None,
            g_value: None,
            frame_fraction: None,
        },
    }))
}

/// 🎯️ A new panel override on the curtain wall `parent`: the first free cell of its base row, opaque in the first material of the model.
pub fn create_curtain_panel_override(snapshot: &ModelSnapshot, id: &str, parent: &str, _name: &str) -> Created {
    let curtain = snapshot.curtain_walls.contains_key(parent).then(|| parent.to_string()).ok_or("bim.create.curtain-wall-missing")?;
    let material = first(&snapshot.materials).ok_or("bim.create.material-missing")?;
    let u = (0..).find(|u| !snapshot.curtain_panel_overrides.values().any(|row| row.curtain == curtain && row.u == *u && row.v == 0)).unwrap_or(0);
    Ok(ModelMutation::CreateCurtainPanelOverride(crate::mutations::create_curtain_panel_override::CreateCurtainPanelOverride {
        id: id.into(),
        curtain_panel_override: CurtainPanelOverride { curtain, u, v: 0, panel: CurtainPanel::Solid { material } },
    }))
}

/// 🏷️ The name an override shows in the outliner: its curtain wall and cell.
pub fn override_name(row: &CurtainPanelOverride) -> String {
    format!("{} ({}, {})", row.curtain, row.u, row.v)
}

//#endregion 🔖️Create

//#region 🔖️Kinds
/// 🏬️ The entity row of curtain wall types (a library entry).
pub const CURTAIN_WALL_TYPE: EntityKind = EntityKind {
    kind: "curtain-wall-type",
    icon: "panels-top-left",
    library: true,
    label: |labels| labels.kind_curtain_wall_type,
    group: |labels| labels.group_curtain_wall_types,
    ids: |snapshot| snapshot.curtain_wall_types.keys().cloned().collect(),
    name: |snapshot, id| snapshot.curtain_wall_types.get(id).map(|row| row.name.clone()),
    parent: |_, _| None,
    delete: Some(|id| ModelMutation::DeleteCurtainWallType(crate::mutations::delete_curtain_wall_type::DeleteCurtainWallType { id: id.into() })),
    rename: Some(|_, id, name| type_field(id, "name", &name.to_string())),
    create: Some(create_curtain_wall_type),
    fields: CURTAIN_WALL_TYPE_FIELDS,
    inferred: CURTAIN_WALL_TYPE_INFERRED,
};

/// 🎯️ The entity row of curtain panel overrides: named by their cell, standing under their curtain wall.
pub const CURTAIN_PANEL_OVERRIDE: EntityKind = EntityKind {
    kind: "curtain-panel-override",
    icon: "square-dashed",
    library: false,
    label: |labels| labels.kind_curtain_panel_override,
    group: |labels| labels.group_curtain_panel_overrides,
    ids: |snapshot| snapshot.curtain_panel_overrides.keys().cloned().collect(),
    name: |snapshot, id| snapshot.curtain_panel_overrides.get(id).map(override_name),
    parent: |snapshot, id| snapshot.curtain_panel_overrides.get(id).map(|row| row.curtain.clone()),
    delete: Some(|id| ModelMutation::DeleteCurtainPanelOverride(crate::mutations::delete_curtain_panel_override::DeleteCurtainPanelOverride { id: id.into() })),
    rename: None,
    create: Some(create_curtain_panel_override),
    fields: CURTAIN_PANEL_OVERRIDE_FIELDS,
    inferred: &[],
};
//#endregion 🔖️Kinds

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
