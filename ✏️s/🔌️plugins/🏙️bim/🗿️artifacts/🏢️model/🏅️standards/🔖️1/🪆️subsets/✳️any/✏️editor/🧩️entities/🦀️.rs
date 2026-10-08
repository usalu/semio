//! 🧩️ The BIM entity table: every entity kind of the snapshot with its tree placement, its authored parameter rows and the mutations that create, rename
//! and delete it. The outliner, the properties panel, the library, the id mint and the delete/rename/set/create commands all read this table, so wiring a new
//! mutation kind is one row (or one `write` on a field row) here and nowhere else.

use crate::editor::bim::terminology::BimLabels;
use crate::{Axis, ModelInference, ModelMutation, ModelSnapshot, OpeningKind, TopConstraint};
use semio_framework_plugin::plugin_app_close_prelude::InputKind;
use semio_framework_ui_locale::LabelText;

//#region 🔖️Types
pub type LabelOf = fn(&BimLabels) -> LabelText;

/// 🧾️ One authored parameter of an entity: how it reads off the snapshot and, when a `set-*` mutation exists for it, how an edited value becomes that mutation.
pub struct FieldRow {
    pub key: &'static str,
    pub label: LabelOf,
    pub input: InputKind,
    pub read: fn(&ModelSnapshot, &str) -> Option<String>,
    pub write: Option<fn(&ModelSnapshot, &str, &str) -> Option<ModelMutation>>,
}

/// 💡️ One inferred, read-only value of an entity.
pub struct InferredRow {
    pub key: &'static str,
    pub label: LabelOf,
    pub read: fn(&ModelSnapshot, &ModelInference, &str) -> Option<String>,
}

/// 🧱️ One entity kind of the snapshot.
pub struct EntityKind {
    pub kind: &'static str,
    pub icon: &'static str,
    pub library: bool,
    pub label: LabelOf,
    pub group: LabelOf,
    pub ids: fn(&ModelSnapshot) -> Vec<String>,
    pub name: fn(&ModelSnapshot, &str) -> Option<String>,
    pub parent: fn(&ModelSnapshot, &str) -> Option<String>,
    pub delete: Option<fn(&str) -> ModelMutation>,
    pub rename: Option<fn(&ModelSnapshot, &str, &str) -> Option<ModelMutation>>,
    pub create: Option<fn(&ModelSnapshot, &str, &str, &str) -> Result<ModelMutation, &'static str>>,
    pub fields: &'static [FieldRow],
    pub inferred: &'static [InferredRow],
}
//#endregion 🔖️Types

//#region 🔖️Helpers
use crate::{Assigned, Baluster, DoorLeaves, Infill, Layer, LayerFunction, LocationLine, MaterialCategory, Phase, Point2, Profile, Rgb, RiserKind, RoofShape, SpaceBoundary, StairFlight, StairStringer, StringerKind, Swing, Vertex};
use semio_framework_plugin::DslValue;
use semio_framework_value::{FromValue, ToValue};
use std::collections::BTreeMap;

fn number(value: f64) -> String {
    format!("{value}")
}

fn point(x: f64, y: f64) -> String {
    format!("{}, {}", number(x), number(y))
}

fn axis_text(axis: &Axis) -> String {
    match axis {
        Axis::Line { start, end } => format!("{} → {}", point(start.x, start.y), point(end.x, end.y)),
        Axis::Arc { start, end, bulge } => format!("{} → {} ⌒ {}", point(start.x, start.y), point(end.x, end.y), number(*bulge)),
    }
}

fn top_text(top: &TopConstraint) -> String {
    match top {
        TopConstraint::Unconnected { height } => format!("{}", number(*height)),
        TopConstraint::StoreyTop { offset } => format!("storey top {}", number(*offset)),
        TopConstraint::Storey { storey, offset } => format!("{storey} {}", number(*offset)),
    }
}

fn opening_text(kind: &OpeningKind) -> String {
    match kind {
        OpeningKind::Window { window_type } => window_type.clone(),
        OpeningKind::Door { door_type } => door_type.clone(),
        OpeningKind::Void { width, height } => format!("{} × {}", number(*width), number(*height)),
    }
}

/// 🪜️ The storeys of one building ordered by level.
pub fn ordered_storeys(snapshot: &ModelSnapshot, building: &str) -> Vec<String> {
    let mut rows: Vec<(i32, &String)> = snapshot.storeys.iter().filter(|(_, storey)| storey.building == building).map(|(id, storey)| (storey.level, id)).collect();
    rows.sort();
    rows.into_iter().map(|(_, id)| id.clone()).collect()
}

fn keys<T>(map: &BTreeMap<String, T>) -> Vec<String> {
    map.keys().cloned().collect()
}

fn first<T>(map: &BTreeMap<String, T>) -> Option<String> {
    map.keys().next().cloned()
}

fn parse_number(text: &str) -> Option<f64> {
    text.trim().parse().ok()
}

fn parse_optional_number(text: &str) -> Option<Option<f64>> {
    let text = text.trim();
    if text.is_empty() {
        return Some(None);
    }
    text.parse().ok().map(Some)
}

fn type_sill(snapshot: &ModelSnapshot, kind: &OpeningKind) -> f64 {
    match kind {
        OpeningKind::Window { window_type } => snapshot.window_types.get(window_type).map_or(0.0, |row| row.sill),
        _ => 0.0,
    }
}

fn parse_int(text: &str) -> Option<i32> {
    text.trim().parse().ok()
}

fn parse_count(text: &str) -> Option<u32> {
    text.trim().parse().ok()
}

fn parse_text(text: &str) -> Option<String> {
    Some(text.trim().to_string())
}

fn parse_flag(text: &str) -> Option<bool> {
    text.trim().parse().ok()
}

fn parse_point(text: &str) -> Option<Point2> {
    let (x, y) = text.split_once(',')?;
    Some(Point2 { x: parse_number(x)?, y: parse_number(y)? })
}

fn parse_axis(text: &str) -> Option<Axis> {
    let (ends, bulge) = match text.split_once('⌒') {
        Some((ends, bulge)) => (ends, Some(parse_number(bulge)?)),
        None => (text, None),
    };
    let (start, end) = ends.split_once('→')?;
    let (start, end) = (parse_point(start)?, parse_point(end)?);
    Some(bulge.map_or(Axis::Line { start, end }, |bulge| Axis::Arc { start, end, bulge }))
}

fn parse_top(text: &str) -> Option<TopConstraint> {
    let text = text.trim();
    if let Some(height) = parse_number(text) {
        return Some(TopConstraint::Unconnected { height });
    }
    if let Some(offset) = text.strip_prefix("storey top") {
        return Some(TopConstraint::StoreyTop { offset: parse_number(offset)? });
    }
    let (storey, offset) = text.rsplit_once(' ')?;
    Some(TopConstraint::Storey { storey: storey.trim().to_string(), offset: parse_number(offset)? })
}

fn variant<T: Clone + std::fmt::Debug>(text: &str, all: &[T]) -> Option<T> {
    all.iter().find(|candidate| format!("{candidate:?}").eq_ignore_ascii_case(text.trim())).cloned()
}

fn profile_text(profile: &Profile) -> String {
    match profile {
        Profile::Rectangle { width, depth } => format!("rectangle {} × {}", number(*width), number(*depth)),
        Profile::Circle { diameter } => format!("circle {}", number(*diameter)),
        Profile::IShape { width, depth, web, flange } => format!("i {} × {} web {} flange {}", number(*width), number(*depth), number(*web), number(*flange)),
        Profile::Custom { outline } => format!("custom {} vertices", outline.len()),
    }
}

fn parse_profile(text: &str) -> Option<Profile> {
    let text = text.trim();
    if let Some(rest) = text.strip_prefix("rectangle") {
        let (width, depth) = rest.split_once('×')?;
        return Some(Profile::Rectangle { width: parse_number(width)?, depth: parse_number(depth)? });
    }
    text.strip_prefix("circle").and_then(parse_number).map(|diameter| Profile::Circle { diameter })
}

fn stringer_text(stringer: &StairStringer) -> String {
    format!("{:?} {} × {}", stringer.kind, number(stringer.width), number(stringer.depth))
}

fn parse_stringer(text: &str) -> Option<StairStringer> {
    let (kind, size) = text.trim().split_once(' ').unwrap_or((text.trim(), "0 × 0"));
    let (width, depth) = size.split_once('×')?;
    let kind = variant(kind, &[StringerKind::None, StringerKind::Closed, StringerKind::Open, StringerKind::Mono])?;
    Some(StairStringer { kind, width: parse_number(width)?, depth: parse_number(depth)? })
}

fn parse_riser(text: &str) -> Option<RiserKind> {
    variant(text, &[RiserKind::Open, RiserKind::Closed])
}

fn infill_text(infill: &Infill) -> String {
    match infill {
        Infill::None => "none".to_string(),
        Infill::Glass { thickness } => format!("glass {}", number(*thickness)),
        Infill::Panel { thickness } => format!("panel {}", number(*thickness)),
    }
}

fn parse_infill(text: &str) -> Option<Infill> {
    let text = text.trim();
    if text.eq_ignore_ascii_case("none") {
        return Some(Infill::None);
    }
    let (kind, thickness) = text.split_once(' ')?;
    let thickness = parse_number(thickness)?;
    match kind.to_ascii_lowercase().as_str() {
        "glass" => Some(Infill::Glass { thickness }),
        "panel" => Some(Infill::Panel { thickness }),
        _ => None,
    }
}

fn baluster_text(baluster: Option<&Baluster>) -> String {
    baluster.map_or_else(|| "none".to_string(), |row| format!("{} @ {}", profile_text(&row.profile), number(row.spacing)))
}

fn parse_baluster(text: &str) -> Option<Option<Baluster>> {
    let text = text.trim();
    if text.eq_ignore_ascii_case("none") {
        return Some(None);
    }
    let (profile, spacing) = text.split_once('@')?;
    Some(Some(Baluster { profile: parse_profile(profile)?, spacing: parse_number(spacing)? }))
}

fn parse_location(text: &str) -> Option<LocationLine> {
    variant(text, &[LocationLine::Center, LocationLine::Interior, LocationLine::Exterior, LocationLine::CoreCenter])
}

fn parse_category(text: &str) -> Option<MaterialCategory> {
    use MaterialCategory::*;
    variant(text, &[Concrete, Masonry, Wood, Metal, Glass, Insulation, Finish, Membrane, Other])
}

fn parse_leaves(text: &str) -> Option<DoorLeaves> {
    variant(text, &[DoorLeaves::Single, DoorLeaves::Double])
}

fn parse_swing(text: &str) -> Option<Swing> {
    variant(text, &[Swing::Left, Swing::Right])
}

/// 🧩️ Builds the leaf of a `set-*` mutation that names exactly one parameter: the leaf's own value decoding fills every other slot with "unchanged".
fn partial<L: FromValue, V: ToValue + ?Sized>(id: &str, field: &str, value: &V) -> Option<L> {
    L::from_value(DslValue::object([("id".to_string(), DslValue::String(id.into())), (field.to_string(), value.to_value())])).ok()
}

macro_rules! set {
    ($module:ident :: $leaf:ident, $id:expr, $field:literal, $value:expr) => {
        partial::<crate::mutations::$module::$leaf, _>($id, $field, $value).map(ModelMutation::$leaf)
    };
}

macro_rules! delete {
    ($module:ident :: $leaf:ident) => {
        Some(|id| ModelMutation::$leaf(crate::mutations::$module::$leaf { id: id.into() }))
    };
}

macro_rules! renaming {
    ($module:ident :: $leaf:ident) => {
        Some(|_, id, name| set!($module::$leaf, id, "name", &name.to_string()))
    };
}

fn rename_element(_: &ModelSnapshot, id: &str, name: &str) -> Option<ModelMutation> {
    Some(ModelMutation::RenameElement(crate::mutations::rename_element::RenameElement { id: id.into(), name: name.into() }))
}

macro_rules! field {
    ($key:literal, $label:ident, $input:ident, |$s:pat_param, $id:pat_param| $read:expr) => {
        FieldRow { key: $key, label: |labels| labels.$label, input: InputKind::$input, read: |$s, $id| $read, write: None }
    };
    ($key:literal, $label:ident, $input:ident, |$s:pat_param, $id:pat_param| $read:expr, rename) => {
        FieldRow { key: $key, label: |labels| labels.$label, input: InputKind::$input, read: |$s, $id| $read, write: Some(rename_element) }
    };
    ($key:literal, $label:ident, $input:ident, |$s:pat_param, $id:pat_param| $read:expr, $parse:ident => $module:ident :: $leaf:ident) => {
        FieldRow { key: $key, label: |labels| labels.$label, input: InputKind::$input, read: |$s, $id| $read, write: Some(|_, id, value| set!($module::$leaf, id, $key, &$parse(value)?)) }
    };
    ($key:literal, $label:ident, $input:ident, |$s:pat_param, $id:pat_param| $read:expr, |$ws:pat_param, $wid:pat_param, $value:pat_param| $write:expr) => {
        FieldRow { key: $key, label: |labels| labels.$label, input: InputKind::$input, read: |$s, $id| $read, write: Some(|$ws, $wid, $value| $write) }
    };
}

macro_rules! inferred {
    ($key:literal, $label:ident, |$s:pat_param, $inference:pat_param, $id:pat_param| $read:expr) => {
        InferredRow { key: $key, label: |labels| labels.$label, read: |$s, $inference, $id| $read }
    };
}

macro_rules! kind {
    ($kind:literal, $icon:literal, $library:literal, $label:ident, $group:ident, $coll:ident . $name:ident, parent: |$ps:pat_param, $pid:pat_param| $parent:expr, $($rest:tt)*) => {
        kind!(@build $kind, $icon, $library, $label, $group, $coll . $name, |$ps, $pid| $parent, $($rest)*)
    };
    (@build $kind:literal, $icon:literal, $library:literal, $label:ident, $group:ident, $coll:ident . $name:ident, |$ps:pat_param, $pid:pat_param| $parent:expr, delete: $delete:expr, rename: $rename:expr, create: $create:expr, fields: $fields:expr, inferred: $inferred:expr $(,)?) => {
        EntityKind {
            kind: $kind,
            icon: $icon,
            library: $library,
            label: |labels| labels.$label,
            group: |labels| labels.$group,
            ids: |snapshot| keys(&snapshot.$coll),
            name: |snapshot, id| snapshot.$coll.get(id).map(|row| row.$name.clone()),
            parent: |$ps, $pid| $parent,
            delete: $delete,
            rename: $rename,
            create: $create,
            fields: $fields,
            inferred: $inferred,
        }
    };
}
//#endregion 🔖️Helpers

//#region 🔖️Fields
static SITE_FIELDS: &[FieldRow] = &[
    field!("name", field_name, Text, |s, id| s.sites.get(id).map(|row| row.name.clone()), rename),
    field!("latitude", field_latitude, Number, |s, id| s.sites.get(id).map(|row| number(row.latitude)), parse_number => set_site::SetSite),
    field!("longitude", field_longitude, Number, |s, id| s.sites.get(id).map(|row| number(row.longitude)), parse_number => set_site::SetSite),
    field!("elevation", field_elevation, Number, |s, id| s.sites.get(id).map(|row| number(row.elevation)), parse_number => set_site::SetSite),
    field!("true_north", field_true_north, Number, |s, id| s.sites.get(id).map(|row| number(row.true_north)), parse_number => set_site::SetSite),
];

static BUILDING_FIELDS: &[FieldRow] = &[
    field!("name", field_name, Text, |s, id| s.buildings.get(id).map(|row| row.name.clone()), rename),
    field!("site", field_site, Text, |s, id| s.buildings.get(id).map(|row| row.site.clone())),
    field!("origin", field_origin, Text, |s, id| s.buildings.get(id).map(|row| point(row.origin.x, row.origin.y)), parse_point => set_building::SetBuilding),
    field!("rotation", field_rotation, Number, |s, id| s.buildings.get(id).map(|row| number(row.rotation)), parse_number => set_building::SetBuilding),
    field!("elevation", field_elevation, Number, |s, id| s.buildings.get(id).map(|row| number(row.elevation)), parse_number => set_building::SetBuilding),
];

static STOREY_FIELDS: &[FieldRow] = &[
    field!("name", field_name, Text, |s, id| s.storeys.get(id).map(|row| row.name.clone()), rename),
    field!("building", field_building, Text, |s, id| s.storeys.get(id).map(|row| row.building.clone())),
    field!("level", field_level, Number, |s, id| s.storeys.get(id).map(|row| row.level.to_string()), parse_int => set_storey_level::SetStoreyLevel),
    field!("height", field_height, Number, |s, id| s.storeys.get(id).map(|row| number(row.height)), parse_number => set_storey_height::SetStoreyHeight),
    field!("cut_height", field_cut_height, Number, |s, id| s.storeys.get(id).map(|row| number(row.cut_height.unwrap_or(crate::DEFAULT_CUT_HEIGHT))), |_, id, value| set!(set_storey_cut_height::SetStoreyCutHeight, id, "cut_height", &Assigned::new(parse_optional_number(value)?))),
];

static GRID_FIELDS: &[FieldRow] = &[
    field!("label", field_label, Text, |s, id| s.grids.get(id).map(|row| row.label.clone()), rename),
    field!("building", field_building, Text, |s, id| s.grids.get(id).map(|row| row.building.clone())),
    field!("start", field_start, Text, |s, id| s.grids.get(id).map(|row| point(row.start.x, row.start.y)), parse_point => set_grid_line::SetGridLine),
    field!("end", field_end, Text, |s, id| s.grids.get(id).map(|row| point(row.end.x, row.end.y)), parse_point => set_grid_line::SetGridLine),
];

static WALL_FIELDS: &[FieldRow] = &[
    field!("name", field_name, Text, |s, id| s.walls.get(id).map(|row| row.name.clone()), rename),
    field!("storey", field_storey, Text, |s, id| s.walls.get(id).map(|row| row.storey.clone())),
    field!("wall_type", field_wall_type, Text, |s, id| s.walls.get(id).map(|row| row.wall_type.clone()), parse_text => set_wall_type_of::SetWallTypeOf),
    field!("axis", field_axis, Text, |s, id| s.walls.get(id).map(|row| axis_text(&row.axis)), parse_axis => set_wall_axis::SetWallAxis),
    field!("location", field_location, Text, |s, id| s.walls.get(id).map(|row| format!("{:?}", row.location)), parse_location => set_wall_location::SetWallLocation),
    field!("base_offset", field_base_offset, Number, |s, id| s.walls.get(id).map(|row| number(row.base_offset)), parse_number => set_wall_base_offset::SetWallBaseOffset),
    field!("top", field_top, Text, |s, id| s.walls.get(id).map(|row| top_text(&row.top)), parse_top => set_wall_top::SetWallTop),
    field!("phase", field_phase, Text, |s, id| s.walls.get(id).map(|row| format!("{:?}", row.phase))),
];

static CURTAIN_WALL_FIELDS: &[FieldRow] = &[
    field!("name", field_name, Text, |s, id| s.curtain_walls.get(id).map(|row| row.name.clone()), rename),
    field!("storey", field_storey, Text, |s, id| s.curtain_walls.get(id).map(|row| row.storey.clone())),
    field!("axis", field_axis, Text, |s, id| s.curtain_walls.get(id).map(|row| axis_text(&row.axis)), parse_axis => set_curtain_wall::SetCurtainWall),
    field!("base_offset", field_base_offset, Number, |s, id| s.curtain_walls.get(id).map(|row| number(row.base_offset)), parse_number => set_curtain_wall::SetCurtainWall),
    field!("top", field_top, Text, |s, id| s.curtain_walls.get(id).map(|row| top_text(&row.top)), parse_top => set_curtain_wall::SetCurtainWall),
    field!("u_spacing", field_width, Number, |s, id| s.curtain_walls.get(id).map(|row| number(row.u_spacing)), parse_number => set_curtain_wall::SetCurtainWall),
    field!("v_spacing", field_height, Number, |s, id| s.curtain_walls.get(id).map(|row| number(row.v_spacing)), parse_number => set_curtain_wall::SetCurtainWall),
    field!("panel_material", field_panel_material, Text, |s, id| s.curtain_walls.get(id).map(|row| row.panel_material.clone()), parse_text => set_curtain_wall::SetCurtainWall),
    field!("mullion_material", field_mullion_material, Text, |s, id| s.curtain_walls.get(id).map(|row| row.mullion_material.clone()), parse_text => set_curtain_wall::SetCurtainWall),
];

static COLUMN_FIELDS: &[FieldRow] = &[
    field!("name", field_name, Text, |s, id| s.columns.get(id).map(|row| row.name.clone()), rename),
    field!("storey", field_storey, Text, |s, id| s.columns.get(id).map(|row| row.storey.clone())),
    field!("column_type", field_column_type, Text, |s, id| s.columns.get(id).map(|row| row.column_type.clone()), parse_text => set_column::SetColumn),
    field!("position", field_position, Text, |s, id| s.columns.get(id).map(|row| point(row.position.x, row.position.y)), parse_point => set_column::SetColumn),
    field!("rotation", field_rotation, Number, |s, id| s.columns.get(id).map(|row| number(row.rotation)), parse_number => set_column::SetColumn),
    field!("base_offset", field_base_offset, Number, |s, id| s.columns.get(id).map(|row| number(row.base_offset)), parse_number => set_column::SetColumn),
    field!("top", field_top, Text, |s, id| s.columns.get(id).map(|row| top_text(&row.top)), parse_top => set_column::SetColumn),
];

static BEAM_FIELDS: &[FieldRow] = &[
    field!("name", field_name, Text, |s, id| s.beams.get(id).map(|row| row.name.clone()), rename),
    field!("storey", field_storey, Text, |s, id| s.beams.get(id).map(|row| row.storey.clone())),
    field!("beam_type", field_beam_type, Text, |s, id| s.beams.get(id).map(|row| row.beam_type.clone()), parse_text => set_beam::SetBeam),
    field!("start", field_start, Text, |s, id| s.beams.get(id).map(|row| point(row.start.x, row.start.y)), parse_point => set_beam::SetBeam),
    field!("end", field_end, Text, |s, id| s.beams.get(id).map(|row| point(row.end.x, row.end.y)), parse_point => set_beam::SetBeam),
    field!("top_offset", field_offset, Number, |s, id| s.beams.get(id).map(|row| number(row.top_offset)), parse_number => set_beam::SetBeam),
];

static SLAB_FIELDS: &[FieldRow] = &[
    field!("name", field_name, Text, |s, id| s.slabs.get(id).map(|row| row.name.clone()), rename),
    field!("storey", field_storey, Text, |s, id| s.slabs.get(id).map(|row| row.storey.clone())),
    field!("slab_type", field_slab_type, Text, |s, id| s.slabs.get(id).map(|row| row.slab_type.clone()), parse_text => set_slab::SetSlab),
    field!("offset", field_offset, Number, |s, id| s.slabs.get(id).map(|row| number(row.offset)), parse_number => set_slab::SetSlab),
];

static ROOF_FIELDS: &[FieldRow] = &[
    field!("name", field_name, Text, |s, id| s.roofs.get(id).map(|row| row.name.clone()), rename),
    field!("storey", field_storey, Text, |s, id| s.roofs.get(id).map(|row| row.storey.clone())),
    field!("roof_type", field_roof_type, Text, |s, id| s.roofs.get(id).map(|row| row.roof_type.clone())),
    field!("shape", field_shape, Text, |s, id| s.roofs.get(id).map(|row| format!("{:?}", row.shape))),
    field!("overhang", field_overhang, Number, |s, id| s.roofs.get(id).map(|row| number(row.overhang)), parse_number => set_roof_shape::SetRoofShape),
    field!("base_offset", field_base_offset, Number, |s, id| s.roofs.get(id).map(|row| number(row.base_offset)), parse_number => set_roof_shape::SetRoofShape),
];

fn opening_kind(snapshot: &ModelSnapshot, id: &str, value: &str) -> Option<ModelMutation> {
    let kind = match &snapshot.openings.get(id)?.kind {
        OpeningKind::Window { .. } => OpeningKind::Window { window_type: value.trim().to_string() },
        OpeningKind::Door { .. } => OpeningKind::Door { door_type: value.trim().to_string() },
        OpeningKind::Void { .. } => {
            let (width, height) = value.split_once('×')?;
            OpeningKind::Void { width: parse_number(width)?, height: parse_number(height)? }
        }
    };
    set!(set_opening::SetOpening, id, "kind", &kind)
}

static OPENING_FIELDS: &[FieldRow] = &[
    field!("name", field_name, Text, |s, id| s.openings.get(id).map(|row| row.name.clone()), rename),
    field!("host", field_host, Text, |s, id| s.openings.get(id).map(|row| row.host.clone()), |s, id, value| Some(ModelMutation::MoveOpening(crate::mutations::move_opening::MoveOpening { id: id.into(), offset: s.openings.get(id)?.offset, host: Some(value.trim().into()) }))),
    field!("kind", field_kind, Text, |s, id| s.openings.get(id).map(|row| opening_text(&row.kind)), |s, id, value| opening_kind(s, id, value)),
    field!("offset", field_offset, Number, |s, id| s.openings.get(id).map(|row| number(row.offset)), parse_number => move_opening::MoveOpening),
    field!("sill_override", field_sill, Number, |s, id| s.openings.get(id).map(|row| number(row.sill_override.unwrap_or_else(|| type_sill(s, &row.kind)))), |_, id, value| set!(set_opening::SetOpening, id, "sill_override", &Assigned::new(parse_optional_number(value)?))),
    field!("width", field_width, Number, |s, id| s.openings.get(id).and_then(|row| row.width).map(number), |_, id, value| set!(set_opening::SetOpening, id, "width", &Assigned::new(Some(parse_number(value)?)))),
    field!("height", field_height, Number, |s, id| s.openings.get(id).and_then(|row| row.height).map(number), |_, id, value| set!(set_opening::SetOpening, id, "height", &Assigned::new(Some(parse_number(value)?)))),
    field!("flip_hand", field_flip_hand, Text, |s, id| s.openings.get(id).map(|row| row.flip_hand.to_string()), parse_flag => set_opening::SetOpening),
    field!("flip_facing", field_flip_facing, Text, |s, id| s.openings.get(id).map(|row| row.flip_facing.to_string()), parse_flag => set_opening::SetOpening),
];

static STAIR_FIELDS: &[FieldRow] = &[
    field!("name", field_name, Text, |s, id| s.stairs.get(id).map(|row| row.name.clone()), rename),
    field!("storey", field_storey, Text, |s, id| s.stairs.get(id).map(|row| row.storey.clone())),
    field!("start", field_start, Text, |s, id| s.stairs.get(id).map(|row| point(row.start.x, row.start.y)), parse_point => set_stair::SetStair),
    field!("direction", field_rotation, Number, |s, id| s.stairs.get(id).map(|row| number(row.direction)), parse_number => set_stair::SetStair),
    field!("width", field_width, Number, |s, id| s.stairs.get(id).map(|row| number(row.width)), parse_number => set_stair::SetStair),
    field!("flight", field_flight, Text, |s, id| s.stairs.get(id).map(|row| format!("{:?}", row.flight))),
    field!("top", field_top, Text, |s, id| s.stairs.get(id).map(|row| top_text(&row.top)), parse_top => set_stair::SetStair),
    field!("max_riser", field_max_riser, Number, |s, id| s.stairs.get(id).map(|row| number(row.max_riser)), parse_number => set_stair::SetStair),
    field!("min_tread", field_min_tread, Number, |s, id| s.stairs.get(id).map(|row| number(row.min_tread)), parse_number => set_stair::SetStair),
    field!("stringer", field_stringer, Text, |s, id| s.stairs.get(id).map(|row| stringer_text(&row.stringer)), parse_stringer => set_stair::SetStair),
    field!("nosing", field_nosing, Number, |s, id| s.stairs.get(id).map(|row| number(row.nosing)), parse_number => set_stair::SetStair),
    field!("tread_thickness", field_tread_thickness, Number, |s, id| s.stairs.get(id).map(|row| number(row.tread_thickness)), parse_number => set_stair::SetStair),
    field!("riser", field_riser, Text, |s, id| s.stairs.get(id).map(|row| format!("{:?}", row.riser)), parse_riser => set_stair::SetStair),
    field!("landing_depth", field_landing_depth, Number, |s, id| s.stairs.get(id).map(|row| number(row.landing_depth)), parse_number => set_stair::SetStair),
];

static RAILING_FIELDS: &[FieldRow] = &[
    field!("name", field_name, Text, |s, id| s.railings.get(id).map(|row| row.name.clone()), rename),
    field!("storey", field_storey, Text, |s, id| s.railings.get(id).map(|row| row.storey.clone())),
    field!("height", field_height, Number, |s, id| s.railings.get(id).map(|row| number(row.height)), parse_number => set_railing::SetRailing),
    field!("post_spacing", field_post_spacing, Number, |s, id| s.railings.get(id).map(|row| number(row.post_spacing)), parse_number => set_railing::SetRailing),
    field!("profile", field_rail_profile, Text, |s, id| s.railings.get(id).map(|row| profile_text(&row.profile)), parse_profile => set_railing::SetRailing),
    field!("post_profile", field_post_profile, Text, |s, id| s.railings.get(id).map(|row| profile_text(&row.post_profile)), parse_profile => set_railing::SetRailing),
    field!("baluster", field_baluster, Text, |s, id| s.railings.get(id).map(|row| baluster_text(row.baluster.as_ref())), |_, id, value| set!(set_railing::SetRailing, id, "baluster", &Assigned::new(parse_baluster(value)?))),
    field!("infill", field_infill, Text, |s, id| s.railings.get(id).map(|row| infill_text(&row.infill)), parse_infill => set_railing::SetRailing),
    field!("material", field_material, Text, |s, id| s.railings.get(id).map(|row| row.material.clone()), parse_text => set_railing::SetRailing),
    field!("base_offset", field_base_offset, Number, |s, id| s.railings.get(id).map(|row| number(row.base_offset)), parse_number => set_railing::SetRailing),
];

static SPACE_FIELDS: &[FieldRow] = &[
    field!("name", field_name, Text, |s, id| s.spaces.get(id).map(|row| row.name.clone()), rename),
    field!("storey", field_storey, Text, |s, id| s.spaces.get(id).map(|row| row.storey.clone())),
    field!("number", field_number, Text, |s, id| s.spaces.get(id).map(|row| row.number.clone()), parse_text => set_space::SetSpace),
    field!("usage", field_usage, Text, |s, id| s.spaces.get(id).map(|row| row.usage.clone()), parse_text => set_space::SetSpace),
];

static MATERIAL_FIELDS: &[FieldRow] = &[
    field!("name", field_name, Text, |s, id| s.materials.get(id).map(|row| row.name.clone()), parse_text => set_material::SetMaterial),
    field!("category", field_category, Text, |s, id| s.materials.get(id).map(|row| format!("{:?}", row.category)), parse_category => set_material::SetMaterial),
    field!("density", field_density, Number, |s, id| s.materials.get(id).map(|row| number(row.density)), parse_number => set_material::SetMaterial),
    field!("conductivity", field_conductivity, Number, |s, id| s.materials.get(id).map(|row| number(row.conductivity)), parse_number => set_material::SetMaterial),
    field!("specific_heat", field_specific_heat, Number, |s, id| s.materials.get(id).map(|row| number(row.specific_heat)), parse_number => set_material::SetMaterial),
];

static WALL_TYPE_FIELDS: &[FieldRow] = &[
    field!("name", field_name, Text, |s, id| s.wall_types.get(id).map(|row| row.name.clone()), parse_text => set_wall_type::SetWallType),
    field!("layers", field_layers, Text, |s, id| s.wall_types.get(id).map(|row| row.layers.len().to_string())),
    field!("thickness", field_thickness, Number, |s, id| s.wall_types.get(id).map(|row| number(row.layers.iter().map(|layer| layer.thickness).sum()))),
];

static SLAB_TYPE_FIELDS: &[FieldRow] = &[
    field!("name", field_name, Text, |s, id| s.slab_types.get(id).map(|row| row.name.clone()), parse_text => set_slab_type::SetSlabType),
    field!("layers", field_layers, Text, |s, id| s.slab_types.get(id).map(|row| row.layers.len().to_string())),
    field!("thickness", field_thickness, Number, |s, id| s.slab_types.get(id).map(|row| number(row.layers.iter().map(|layer| layer.thickness).sum()))),
];

static ROOF_TYPE_FIELDS: &[FieldRow] = &[
    field!("name", field_name, Text, |s, id| s.roof_types.get(id).map(|row| row.name.clone()), parse_text => set_roof_type::SetRoofType),
    field!("layers", field_layers, Text, |s, id| s.roof_types.get(id).map(|row| row.layers.len().to_string())),
    field!("thickness", field_thickness, Number, |s, id| s.roof_types.get(id).map(|row| number(row.layers.iter().map(|layer| layer.thickness).sum()))),
];

static COLUMN_TYPE_FIELDS: &[FieldRow] = &[
    field!("name", field_name, Text, |s, id| s.column_types.get(id).map(|row| row.name.clone()), parse_text => set_column_type::SetColumnType),
    field!("material", field_material, Text, |s, id| s.column_types.get(id).map(|row| row.material.clone()), parse_text => set_column_type::SetColumnType),
];

static BEAM_TYPE_FIELDS: &[FieldRow] = &[
    field!("name", field_name, Text, |s, id| s.beam_types.get(id).map(|row| row.name.clone()), parse_text => set_beam_type::SetBeamType),
    field!("material", field_material, Text, |s, id| s.beam_types.get(id).map(|row| row.material.clone()), parse_text => set_beam_type::SetBeamType),
];

static WINDOW_TYPE_FIELDS: &[FieldRow] = &[
    field!("name", field_name, Text, |s, id| s.window_types.get(id).map(|row| row.name.clone()), parse_text => set_window_type::SetWindowType),
    field!("width", field_width, Number, |s, id| s.window_types.get(id).map(|row| number(row.width)), parse_number => set_window_type::SetWindowType),
    field!("height", field_height, Number, |s, id| s.window_types.get(id).map(|row| number(row.height)), parse_number => set_window_type::SetWindowType),
    field!("sill", field_sill, Number, |s, id| s.window_types.get(id).map(|row| number(row.sill)), parse_number => set_window_type::SetWindowType),
    field!("frame_width", field_frame_width, Number, |s, id| s.window_types.get(id).map(|row| number(row.frame_width)), parse_number => set_window_type::SetWindowType),
    field!("frame_depth", field_frame_depth, Number, |s, id| s.window_types.get(id).map(|row| number(row.frame_depth)), parse_number => set_window_type::SetWindowType),
    field!("panes", field_panes, Number, |s, id| s.window_types.get(id).map(|row| row.panes.to_string()), parse_count => set_window_type::SetWindowType),
    field!("material", field_material, Text, |s, id| s.window_types.get(id).map(|row| row.material.clone()), parse_text => set_window_type::SetWindowType),
];

static DOOR_TYPE_FIELDS: &[FieldRow] = &[
    field!("name", field_name, Text, |s, id| s.door_types.get(id).map(|row| row.name.clone()), parse_text => set_door_type::SetDoorType),
    field!("width", field_width, Number, |s, id| s.door_types.get(id).map(|row| number(row.width)), parse_number => set_door_type::SetDoorType),
    field!("height", field_height, Number, |s, id| s.door_types.get(id).map(|row| number(row.height)), parse_number => set_door_type::SetDoorType),
    field!("frame_width", field_frame_width, Number, |s, id| s.door_types.get(id).map(|row| number(row.frame_width)), parse_number => set_door_type::SetDoorType),
    field!("frame_depth", field_frame_depth, Number, |s, id| s.door_types.get(id).map(|row| number(row.frame_depth)), parse_number => set_door_type::SetDoorType),
    field!("leaves", field_leaves, Text, |s, id| s.door_types.get(id).map(|row| format!("{:?}", row.leaves)), parse_leaves => set_door_type::SetDoorType),
    field!("swing", field_swing, Text, |s, id| s.door_types.get(id).map(|row| format!("{:?}", row.swing)), parse_swing => set_door_type::SetDoorType),
    field!("material", field_material, Text, |s, id| s.door_types.get(id).map(|row| row.material.clone()), parse_text => set_door_type::SetDoorType),
];
//#endregion 🔖️Fields

//#region 🔖️Inferred
static STOREY_INFERRED: &[InferredRow] = &[
    inferred!("elevation", field_elevation, |_, inference, id| inference.storey_levels.get(id).map(|row| number(row.elevation))),
    inferred!("top_elevation", field_top_elevation, |_, inference, id| inference.storey_levels.get(id).map(|row| number(row.top_elevation))),
];

/// 💡️ The quantities of a placed element, read off the `quantities` inference; a measure that does not apply to the kind is 0 and shows no row.
static QUANTITY_INFERRED: &[InferredRow] = &[
    inferred!("length", field_length, |_, inference, id| inference.quantities.elements.get(id).filter(|row| row.length > 0.0).map(|row| number(row.length))),
    inferred!("width", field_width, |_, inference, id| inference.quantities.elements.get(id).filter(|row| row.width > 0.0).map(|row| number(row.width))),
    inferred!("height", field_height, |_, inference, id| inference.quantities.elements.get(id).filter(|row| row.height > 0.0).map(|row| number(row.height))),
    inferred!("area", field_area, |_, inference, id| inference.quantities.elements.get(id).map(|row| row.area()).filter(|area| *area > 0.0).map(number)),
    inferred!("volume", field_volume, |_, inference, id| inference.quantities.elements.get(id).filter(|row| row.net_volume > 0.0).map(|row| number(row.net_volume))),
    inferred!("mass", field_mass, |_, inference, id| inference.quantities.elements.get(id).filter(|row| row.mass > 0.0).map(|row| number(row.mass))),
];
//#endregion 🔖️Inferred

//#region 🔖️Create
type Created = Result<ModelMutation, &'static str>;

fn container(map: &BTreeMap<String, impl Sized>, parent: &str, missing: &'static str) -> Result<String, &'static str> {
    map.contains_key(parent).then(|| parent.to_string()).ok_or(missing)
}

fn rectangle(width: f64, depth: f64) -> Vec<Vertex> {
    [(0.0, 0.0), (width, 0.0), (width, depth), (0.0, depth)].into_iter().map(|(x, y)| Vertex { point: Point2 { x, y }, bulge: 0.0 }).collect()
}

fn along(row: f64) -> (Point2, Point2) {
    (Point2 { x: 0.0, y: row }, Point2 { x: 4.0, y: row })
}

fn create_site(_: &ModelSnapshot, id: &str, _parent: &str, name: &str) -> Created {
    Ok(ModelMutation::CreateSite(crate::mutations::create_site::CreateSite { id: id.into(), site: crate::Site { name: name.into(), latitude: 0.0, longitude: 0.0, elevation: 0.0, true_north: 0.0, boundary: Vec::new() } }))
}

fn create_building(snapshot: &ModelSnapshot, id: &str, parent: &str, name: &str) -> Created {
    let site = snapshot.sites.contains_key(parent).then(|| parent.to_string()).or_else(|| first(&snapshot.sites)).ok_or("bim.create.site-missing")?;
    Ok(ModelMutation::CreateBuilding(crate::mutations::create_building::CreateBuilding { id: id.into(), building: crate::Building { site, name: name.into(), origin: Point2 { x: 0.0, y: 0.0 }, rotation: 0.0, elevation: 0.0 } }))
}

fn create_storey(snapshot: &ModelSnapshot, id: &str, parent: &str, name: &str) -> Created {
    let building = snapshot.buildings.contains_key(parent).then(|| parent.to_string()).or_else(|| first(&snapshot.buildings)).ok_or("bim.create.building-missing")?;
    let levels: Vec<&crate::Storey> = snapshot.storeys.values().filter(|storey| storey.building == building).collect();
    let level = levels.iter().map(|storey| storey.level).max().map_or(0, |highest| highest + 1);
    let height = levels.iter().max_by_key(|storey| storey.level).map_or(3.0, |top| top.height);
    Ok(ModelMutation::CreateStorey(crate::mutations::create_storey::CreateStorey { id: id.into(), storey: crate::Storey { building, name: name.into(), level, height, cut_height: None } }))
}

fn create_grid(snapshot: &ModelSnapshot, id: &str, parent: &str, name: &str) -> Created {
    let building = container(&snapshot.buildings, parent, "bim.create.building-missing")?;
    let column = snapshot.grids.values().filter(|grid| grid.building == building).count() as f64;
    let (start, end) = (Point2 { x: column * 3.0, y: 0.0 }, Point2 { x: column * 3.0, y: 10.0 });
    Ok(ModelMutation::CreateGridLine(crate::mutations::create_grid_line::CreateGridLine { id: id.into(), grid_line: crate::GridLine { building, label: name.into(), start, end } }))
}

fn create_wall(snapshot: &ModelSnapshot, id: &str, parent: &str, name: &str) -> Created {
    let storey = container(&snapshot.storeys, parent, "bim.create.storey-missing")?;
    let wall_type = first(&snapshot.wall_types).ok_or("bim.create.wall-type-missing")?;
    let row = snapshot.walls.values().filter(|wall| wall.storey == storey).count() as f64;
    let (start, end) = along(row);
    Ok(ModelMutation::CreateWall(crate::mutations::create_wall::CreateWall {
        id: id.into(),
        wall: crate::Wall { storey, wall_type, axis: Axis::Line { start, end }, location: LocationLine::Center, base_offset: 0.0, top: TopConstraint::StoreyTop { offset: 0.0 }, phase: Phase::New, name: name.into() },
    }))
}

fn create_curtain_wall(snapshot: &ModelSnapshot, id: &str, parent: &str, name: &str) -> Created {
    let storey = container(&snapshot.storeys, parent, "bim.create.storey-missing")?;
    let material = first(&snapshot.materials).ok_or("bim.create.material-missing")?;
    let (start, end) = along(snapshot.curtain_walls.values().filter(|wall| wall.storey == storey).count() as f64);
    Ok(ModelMutation::CreateCurtainWall(crate::mutations::create_curtain_wall::CreateCurtainWall {
        id: id.into(),
        curtain_wall: crate::CurtainWall {
            storey,
            axis: Axis::Line { start, end },
            base_offset: 0.0,
            top: TopConstraint::StoreyTop { offset: 0.0 },
            u_spacing: 1.5,
            v_spacing: 1.5,
            mullion: Profile::Rectangle { width: 0.05, depth: 0.1 },
            panel_material: material.clone(),
            mullion_material: material,
            name: name.into(),
        },
    }))
}

fn create_column(snapshot: &ModelSnapshot, id: &str, parent: &str, name: &str) -> Created {
    let storey = container(&snapshot.storeys, parent, "bim.create.storey-missing")?;
    let column_type = first(&snapshot.column_types).ok_or("bim.create.column-type-missing")?;
    let position = Point2 { x: snapshot.columns.values().filter(|column| column.storey == storey).count() as f64 * 3.0, y: 0.0 };
    Ok(ModelMutation::CreateColumn(crate::mutations::create_column::CreateColumn {
        id: id.into(),
        column: crate::Column { storey, column_type, position, rotation: 0.0, base_offset: 0.0, top: TopConstraint::StoreyTop { offset: 0.0 }, name: name.into() },
    }))
}

fn create_beam(snapshot: &ModelSnapshot, id: &str, parent: &str, name: &str) -> Created {
    let storey = container(&snapshot.storeys, parent, "bim.create.storey-missing")?;
    let beam_type = first(&snapshot.beam_types).ok_or("bim.create.beam-type-missing")?;
    let (start, end) = along(snapshot.beams.values().filter(|beam| beam.storey == storey).count() as f64);
    Ok(ModelMutation::CreateBeam(crate::mutations::create_beam::CreateBeam { id: id.into(), beam: crate::Beam { storey, beam_type, start, end, top_offset: 0.0, name: name.into() } }))
}

fn create_slab(snapshot: &ModelSnapshot, id: &str, parent: &str, name: &str) -> Created {
    let storey = container(&snapshot.storeys, parent, "bim.create.storey-missing")?;
    let slab_type = first(&snapshot.slab_types).ok_or("bim.create.slab-type-missing")?;
    Ok(ModelMutation::CreateSlab(crate::mutations::create_slab::CreateSlab {
        id: id.into(),
        slab: crate::Slab { storey, slab_type, boundary: rectangle(4.0, 4.0), holes: Vec::new(), offset: 0.0, slope: None, name: name.into() },
    }))
}

fn create_roof(snapshot: &ModelSnapshot, id: &str, parent: &str, name: &str) -> Created {
    let storey = container(&snapshot.storeys, parent, "bim.create.storey-missing")?;
    let roof_type = first(&snapshot.roof_types).ok_or("bim.create.roof-type-missing")?;
    Ok(ModelMutation::CreateRoof(crate::mutations::create_roof::CreateRoof {
        id: id.into(),
        roof: crate::Roof { storey, roof_type, footprint: rectangle(4.0, 4.0), shape: RoofShape::Flat, overhang: 0.3, base_offset: 0.0, name: name.into() },
    }))
}

fn create_opening(snapshot: &ModelSnapshot, id: &str, parent: &str, name: &str) -> Created {
    let host = snapshot.walls.contains_key(parent).then(|| parent.to_string()).or_else(|| snapshot.curtain_walls.contains_key(parent).then(|| parent.to_string())).ok_or("bim.create.wall-missing")?;
    let kind = match (first(&snapshot.window_types), first(&snapshot.door_types)) {
        (Some(window_type), _) => OpeningKind::Window { window_type },
        (None, Some(door_type)) => OpeningKind::Door { door_type },
        (None, None) => OpeningKind::Void { width: 0.9, height: 2.1 },
    };
    Ok(ModelMutation::CreateOpening(crate::mutations::create_opening::CreateOpening {
        id: id.into(),
        opening: crate::Opening { host, kind, offset: 1.0, sill_override: None, width: None, height: None, flip_hand: false, flip_facing: false, name: name.into() },
    }))
}

fn create_stair(snapshot: &ModelSnapshot, id: &str, parent: &str, name: &str) -> Created {
    let storey = container(&snapshot.storeys, parent, "bim.create.storey-missing")?;
    Ok(ModelMutation::CreateStair(crate::mutations::create_stair::CreateStair {
        id: id.into(),
        stair: crate::Stair {
            storey,
            start: Point2 { x: 0.0, y: 0.0 },
            direction: 0.0,
            width: 1.2,
            flight: StairFlight::Straight,
            top: TopConstraint::StoreyTop { offset: 0.0 },
            max_riser: 0.18,
            min_tread: 0.27,
            stringer: crate::STANDARD_STRINGER,
            nosing: 0.0,
            tread_thickness: crate::STANDARD_TREAD_THICKNESS,
            riser: crate::STANDARD_RISER,
            landing_depth: 1.2,
            name: name.into(),
        },
    }))
}

fn create_railing(snapshot: &ModelSnapshot, id: &str, parent: &str, name: &str) -> Created {
    let storey = container(&snapshot.storeys, parent, "bim.create.storey-missing")?;
    let material = first(&snapshot.materials).ok_or("bim.create.material-missing")?;
    let (start, end) = along(snapshot.railings.values().filter(|railing| railing.storey == storey).count() as f64);
    Ok(ModelMutation::CreateRailing(crate::mutations::create_railing::CreateRailing {
        id: id.into(),
        railing: crate::Railing { storey, path: vec![start, end], height: 1.0, post_spacing: 1.2, profile: crate::standard_rail_profile(), post_profile: crate::standard_post_profile(), baluster: None, infill: crate::STANDARD_INFILL, material, base_offset: 0.0, name: name.into() },
    }))
}

fn create_space(snapshot: &ModelSnapshot, id: &str, parent: &str, name: &str) -> Created {
    let storey = container(&snapshot.storeys, parent, "bim.create.storey-missing")?;
    let number = (snapshot.spaces.values().filter(|space| space.storey == storey).count() + 1).to_string();
    Ok(ModelMutation::CreateSpace(crate::mutations::create_space::CreateSpace {
        id: id.into(),
        space: crate::Space { storey, number, name: name.into(), boundary: SpaceBoundary::Bounded { seed: Point2 { x: 1.0, y: 1.0 } }, usage: String::new() },
    }))
}

fn create_material(_: &ModelSnapshot, id: &str, _parent: &str, name: &str) -> Created {
    Ok(ModelMutation::CreateMaterial(crate::mutations::create_material::CreateMaterial {
        id: id.into(),
        material: crate::Material { name: name.into(), category: MaterialCategory::Other, color: Rgb { r: 0.7, g: 0.7, b: 0.7 }, density: 1000.0, conductivity: 1.0, specific_heat: 1000.0 },
    }))
}

fn layers(snapshot: &ModelSnapshot) -> Result<Vec<Layer>, &'static str> {
    let material = first(&snapshot.materials).ok_or("bim.create.material-missing")?;
    Ok(vec![Layer { material, thickness: 0.2, function: LayerFunction::Structure }])
}

fn create_wall_type(snapshot: &ModelSnapshot, id: &str, _parent: &str, name: &str) -> Created {
    Ok(ModelMutation::CreateWallType(crate::mutations::create_wall_type::CreateWallType { id: id.into(), wall_type: crate::WallType { name: name.into(), layers: layers(snapshot)? } }))
}

fn create_slab_type(snapshot: &ModelSnapshot, id: &str, _parent: &str, name: &str) -> Created {
    Ok(ModelMutation::CreateSlabType(crate::mutations::create_slab_type::CreateSlabType { id: id.into(), slab_type: crate::SlabType { name: name.into(), layers: layers(snapshot)? } }))
}

fn create_roof_type(snapshot: &ModelSnapshot, id: &str, _parent: &str, name: &str) -> Created {
    Ok(ModelMutation::CreateRoofType(crate::mutations::create_roof_type::CreateRoofType { id: id.into(), roof_type: crate::RoofType { name: name.into(), layers: layers(snapshot)? } }))
}

fn create_column_type(snapshot: &ModelSnapshot, id: &str, _parent: &str, name: &str) -> Created {
    let material = first(&snapshot.materials).ok_or("bim.create.material-missing")?;
    Ok(ModelMutation::CreateColumnType(crate::mutations::create_column_type::CreateColumnType {
        id: id.into(),
        column_type: crate::ColumnType { name: name.into(), profile: Profile::Rectangle { width: 0.3, depth: 0.3 }, material },
    }))
}

fn create_beam_type(snapshot: &ModelSnapshot, id: &str, _parent: &str, name: &str) -> Created {
    let material = first(&snapshot.materials).ok_or("bim.create.material-missing")?;
    Ok(ModelMutation::CreateBeamType(crate::mutations::create_beam_type::CreateBeamType {
        id: id.into(),
        beam_type: crate::BeamType { name: name.into(), profile: Profile::Rectangle { width: 0.2, depth: 0.4 }, material },
    }))
}

fn create_window_type(snapshot: &ModelSnapshot, id: &str, _parent: &str, name: &str) -> Created {
    let material = first(&snapshot.materials).ok_or("bim.create.material-missing")?;
    Ok(ModelMutation::CreateWindowType(crate::mutations::create_window_type::CreateWindowType {
        id: id.into(),
        window_type: crate::WindowType { name: name.into(), width: 1.2, height: 1.2, sill: 0.9, frame_width: 0.06, frame_depth: 0.08, panes: 2, material },
    }))
}

fn create_door_type(snapshot: &ModelSnapshot, id: &str, _parent: &str, name: &str) -> Created {
    let material = first(&snapshot.materials).ok_or("bim.create.material-missing")?;
    Ok(ModelMutation::CreateDoorType(crate::mutations::create_door_type::CreateDoorType {
        id: id.into(),
        door_type: crate::DoorType { name: name.into(), width: 0.9, height: 2.1, frame_width: 0.06, frame_depth: 0.08, leaves: DoorLeaves::Single, swing: Swing::Left, material },
    }))
}
//#endregion 🔖️Create

//#region 🔖️Table
/// 🧩️ Every entity kind, structure first (site to space), then the library (materials and types). A new mutation kind is one row.
pub static ENTITIES: &[EntityKind] = &[
    kind!("site", "map", false, kind_site, group_sites, sites . name, parent: |_, _| None,
        delete: delete!(delete_site::DeleteSite), rename: Some(rename_element), create: Some(create_site), fields: SITE_FIELDS, inferred: &[]),
    kind!("building", "building", false, kind_building, group_buildings, buildings . name, parent: |s, id| s.buildings.get(id).map(|row| row.site.clone()),
        delete: delete!(delete_building::DeleteBuilding), rename: Some(rename_element), create: Some(create_building), fields: BUILDING_FIELDS, inferred: &[]),
    kind!("storey", "layers", false, kind_storey, group_storeys, storeys . name, parent: |s, id| s.storeys.get(id).map(|row| row.building.clone()),
        delete: delete!(delete_storey::DeleteStorey), rename: Some(rename_element), create: Some(create_storey), fields: STOREY_FIELDS, inferred: STOREY_INFERRED),
    kind!("grid", "grid", false, kind_grid, group_grids, grids . label, parent: |s, id| s.grids.get(id).map(|row| row.building.clone()),
        delete: delete!(delete_grid_line::DeleteGridLine), rename: Some(rename_element), create: Some(create_grid), fields: GRID_FIELDS, inferred: &[]),
    kind!("wall", "square", false, kind_wall, group_walls, walls . name, parent: |s, id| s.walls.get(id).map(|row| row.storey.clone()),
        delete: delete!(delete_wall::DeleteWall), rename: Some(rename_element), create: Some(create_wall), fields: WALL_FIELDS, inferred: QUANTITY_INFERRED),
    kind!("curtain-wall", "panels-top-left", false, kind_curtain_wall, group_curtain_walls, curtain_walls . name, parent: |s, id| s.curtain_walls.get(id).map(|row| row.storey.clone()),
        delete: delete!(delete_curtain_wall::DeleteCurtainWall), rename: Some(rename_element), create: Some(create_curtain_wall), fields: CURTAIN_WALL_FIELDS, inferred: QUANTITY_INFERRED),
    kind!("column", "columns", false, kind_column, group_columns, columns . name, parent: |s, id| s.columns.get(id).map(|row| row.storey.clone()),
        delete: delete!(delete_column::DeleteColumn), rename: Some(rename_element), create: Some(create_column), fields: COLUMN_FIELDS, inferred: QUANTITY_INFERRED),
    kind!("beam", "minus", false, kind_beam, group_beams, beams . name, parent: |s, id| s.beams.get(id).map(|row| row.storey.clone()),
        delete: delete!(delete_beam::DeleteBeam), rename: Some(rename_element), create: Some(create_beam), fields: BEAM_FIELDS, inferred: QUANTITY_INFERRED),
    kind!("slab", "layout-panel-top", false, kind_slab, group_slabs, slabs . name, parent: |s, id| s.slabs.get(id).map(|row| row.storey.clone()),
        delete: delete!(delete_slab::DeleteSlab), rename: Some(rename_element), create: Some(create_slab), fields: SLAB_FIELDS, inferred: QUANTITY_INFERRED),
    kind!("roof", "house", false, kind_roof, group_roofs, roofs . name, parent: |s, id| s.roofs.get(id).map(|row| row.storey.clone()),
        delete: delete!(delete_roof::DeleteRoof), rename: Some(rename_element), create: Some(create_roof), fields: ROOF_FIELDS, inferred: QUANTITY_INFERRED),
    kind!("opening", "door-open", false, kind_opening, group_openings, openings . name, parent: |s, id| s.openings.get(id).map(|row| row.host.clone()),
        delete: delete!(delete_opening::DeleteOpening), rename: Some(rename_element), create: Some(create_opening), fields: OPENING_FIELDS, inferred: QUANTITY_INFERRED),
    kind!("stair", "footprints", false, kind_stair, group_stairs, stairs . name, parent: |s, id| s.stairs.get(id).map(|row| row.storey.clone()),
        delete: delete!(delete_stair::DeleteStair), rename: Some(rename_element), create: Some(create_stair), fields: STAIR_FIELDS, inferred: QUANTITY_INFERRED),
    kind!("railing", "fence", false, kind_railing, group_railings, railings . name, parent: |s, id| s.railings.get(id).map(|row| row.storey.clone()),
        delete: delete!(delete_railing::DeleteRailing), rename: Some(rename_element), create: Some(create_railing), fields: RAILING_FIELDS, inferred: QUANTITY_INFERRED),
    kind!("space", "square-dashed", false, kind_space, group_spaces, spaces . name, parent: |s, id| s.spaces.get(id).map(|row| row.storey.clone()),
        delete: delete!(delete_space::DeleteSpace), rename: Some(rename_element), create: Some(create_space), fields: SPACE_FIELDS, inferred: QUANTITY_INFERRED),
    kind!("material", "palette", true, kind_material, group_materials, materials . name, parent: |_, _| None,
        delete: delete!(delete_material::DeleteMaterial), rename: renaming!(set_material::SetMaterial), create: Some(create_material), fields: MATERIAL_FIELDS, inferred: &[]),
    kind!("wall-type", "brick-wall", true, kind_wall_type, group_wall_types, wall_types . name, parent: |_, _| None,
        delete: delete!(delete_wall_type::DeleteWallType), rename: renaming!(set_wall_type::SetWallType), create: Some(create_wall_type), fields: WALL_TYPE_FIELDS, inferred: &[]),
    kind!("slab-type", "layout-panel-top", true, kind_slab_type, group_slab_types, slab_types . name, parent: |_, _| None,
        delete: delete!(delete_slab_type::DeleteSlabType), rename: renaming!(set_slab_type::SetSlabType), create: Some(create_slab_type), fields: SLAB_TYPE_FIELDS, inferred: &[]),
    kind!("roof-type", "house", true, kind_roof_type, group_roof_types, roof_types . name, parent: |_, _| None,
        delete: delete!(delete_roof_type::DeleteRoofType), rename: renaming!(set_roof_type::SetRoofType), create: Some(create_roof_type), fields: ROOF_TYPE_FIELDS, inferred: &[]),
    kind!("column-type", "columns", true, kind_column_type, group_column_types, column_types . name, parent: |_, _| None,
        delete: delete!(delete_column_type::DeleteColumnType), rename: renaming!(set_column_type::SetColumnType), create: Some(create_column_type), fields: COLUMN_TYPE_FIELDS, inferred: &[]),
    kind!("beam-type", "minus", true, kind_beam_type, group_beam_types, beam_types . name, parent: |_, _| None,
        delete: delete!(delete_beam_type::DeleteBeamType), rename: renaming!(set_beam_type::SetBeamType), create: Some(create_beam_type), fields: BEAM_TYPE_FIELDS, inferred: &[]),
    kind!("window-type", "app-window", true, kind_window_type, group_window_types, window_types . name, parent: |_, _| None,
        delete: delete!(delete_window_type::DeleteWindowType), rename: renaming!(set_window_type::SetWindowType), create: Some(create_window_type), fields: WINDOW_TYPE_FIELDS, inferred: &[]),
    kind!("door-type", "door-closed", true, kind_door_type, group_door_types, door_types . name, parent: |_, _| None,
        delete: delete!(delete_door_type::DeleteDoorType), rename: renaming!(set_door_type::SetDoorType), create: Some(create_door_type), fields: DOOR_TYPE_FIELDS, inferred: &[]),
];
//#endregion 🔖️Table

//#region 🔖️Lookup
/// 🔎️ The kind row of a kind id.
pub fn kind_of(kind: &str) -> Option<&'static EntityKind> {
    ENTITIES.iter().find(|row| row.kind == kind)
}

/// 🔎️ The kind row that holds `id`, searched in table order.
pub fn kind_holding(snapshot: &ModelSnapshot, id: &str) -> Option<&'static EntityKind> {
    ENTITIES.iter().find(|row| (row.name)(snapshot, id).is_some())
}

/// 🪪️ Whether any collection of the snapshot already holds `id`.
pub fn id_taken(snapshot: &ModelSnapshot, id: &str) -> bool {
    kind_holding(snapshot, id).is_some()
}

/// 🪜️ The storey an entity is drawn on: its own, or its host's for an opening.
pub fn storey_of(snapshot: &ModelSnapshot, id: &str) -> Option<String> {
    let row = kind_holding(snapshot, id)?;
    match row.kind {
        "storey" => Some(id.to_string()),
        "opening" => (row.parent)(snapshot, id).and_then(|host| storey_of(snapshot, &host)),
        "site" | "building" | "grid" => None,
        _ if row.library => None,
        _ => (row.parent)(snapshot, id),
    }
}
//#endregion 🔖️Lookup

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
