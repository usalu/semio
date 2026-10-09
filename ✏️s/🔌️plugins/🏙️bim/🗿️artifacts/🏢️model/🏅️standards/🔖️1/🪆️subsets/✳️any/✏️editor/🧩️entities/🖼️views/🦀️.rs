//! 🖼️ The view rows of the entity table: how an authored view reads off the snapshot, how an edited value becomes a `set-view` mutation, and what a new view is. A view is created as the plan of
//! a storey here; the other kinds are created by the `createView` command, which knows how to place a section, an elevation or a camera.

use super::{container, number, parse_count, parse_number, parse_optional_number, parse_point, parse_text, partial, point, variant, Created, FieldRow};
use crate::{Assigned, DetailLevel, ModelMutation, ModelSnapshot, Phase, Point2, View, ViewCamera, ViewCategory, ViewCrop, ViewKind, ViewPlane};
use semio_framework_plugin::plugin_app_close_prelude::InputKind;

const CATEGORIES: [ViewCategory; 11] = [
    ViewCategory::Walls,
    ViewCategory::CurtainWalls,
    ViewCategory::Columns,
    ViewCategory::Beams,
    ViewCategory::Slabs,
    ViewCategory::Roofs,
    ViewCategory::Openings,
    ViewCategory::Stairs,
    ViewCategory::Railings,
    ViewCategory::Spaces,
    ViewCategory::Grids,
];

/// ▭️ The text of a plane or a crop rectangle: `x, y → x, y`.
pub fn pair_text(first: Point2, second: Point2) -> String {
    format!("{} → {}", point(first.x, first.y), point(second.x, second.y))
}

fn parse_pair(text: &str) -> Option<(Point2, Point2)> {
    let (first, second) = text.split_once('→')?;
    Some((parse_point(first)?, parse_point(second)?))
}

fn parse_plane(text: &str) -> Option<ViewPlane> {
    parse_pair(text).map(|(start, end)| ViewPlane { start, end })
}

pub fn parse_crop(text: &str) -> Option<Option<ViewCrop>> {
    if text.trim().is_empty() {
        return Some(None);
    }
    parse_pair(text).map(|(min, max)| Some(ViewCrop { min, max }))
}

fn camera_text(camera: &ViewCamera) -> String {
    [camera.target.x, camera.target.y, camera.target_height, camera.azimuth, camera.pitch, camera.distance].map(number).join(", ")
}

fn parse_camera(text: &str) -> Option<ViewCamera> {
    let numbers: Vec<f64> = text.split(',').map(|part| part.trim().parse().ok()).collect::<Option<Vec<f64>>>()?;
    let [x, y, height, azimuth, pitch, distance] = numbers.try_into().ok()?;
    Some(ViewCamera { target: Point2 { x, y }, target_height: height, azimuth, pitch, distance })
}

fn parse_detail(text: &str) -> Option<DetailLevel> {
    variant(text, &[DetailLevel::Coarse, DetailLevel::Medium, DetailLevel::Fine])
}

fn parse_phase(text: &str) -> Option<Option<Phase>> {
    if text.trim().is_empty() {
        return Some(None);
    }
    variant(text, &[Phase::Existing, Phase::New, Phase::Demolished, Phase::Temporary]).map(Some)
}

fn hidden_text(hidden: &[ViewCategory]) -> String {
    hidden.iter().map(|category| format!("{category:?}")).collect::<Vec<_>>().join(", ")
}

fn parse_hidden(text: &str) -> Option<Vec<ViewCategory>> {
    let mut rows: Vec<ViewCategory> = text.split(',').map(str::trim).filter(|part| !part.is_empty()).map(|part| variant(part, &CATEGORIES)).collect::<Option<Vec<_>>>()?;
    rows.sort_by_key(|category| *category as u8);
    rows.dedup();
    Some(rows)
}

/// 🧾️ The authored parameters of a view; the rows a kind does not own read as nothing and are not shown.
pub static VIEW_FIELDS: &[FieldRow] = &[
    field!("name", field_name, Text, |s, id| s.views.get(id).map(|row| row.name.clone()), parse_text => set_view::SetView),
    field!("building", field_building, Text, |s, id| s.views.get(id).map(|row| row.building.clone())),
    field!("kind", field_kind, Text, |s, id| s.views.get(id).map(|row| format!("{:?}", row.kind))),
    field!("storey", field_storey, Text, |s, id| s.views.get(id).and_then(|row| row.storey.clone()), parse_text => set_view::SetView),
    field!("plane", field_plane, Text, |s, id| s.views.get(id).and_then(|row| row.plane).map(|plane| pair_text(plane.start, plane.end)), parse_plane => set_view::SetView),
    field!("camera", field_camera, Text, |s, id| s.views.get(id).and_then(|row| row.camera).map(|camera| camera_text(&camera)), parse_camera => set_view::SetView),
    field!("cut_height", field_cut_height, Number, |s, id| s.views.get(id).filter(|row| row.kind.is_plan()).map(|row| row.cut_height.map_or_else(String::new, number)), |_, id, value| set!(set_view::SetView, id, "cut_height", &Assigned::new(parse_optional_number(value)?))),
    field!("depth", field_view_depth, Number, |s, id| s.views.get(id).map(|row| number(row.depth)), parse_number => set_view::SetView),
    field!("crop", field_crop, Text, |s, id| s.views.get(id).filter(|row| !row.kind.is_camera()).map(|row| row.crop.map_or_else(String::new, |crop| pair_text(crop.min, crop.max))), |_, id, value| set!(set_view::SetView, id, "crop", &Assigned::new(parse_crop(value)?))),
    field!("hidden", field_hidden, Text, |s, id| s.views.get(id).map(|row| hidden_text(&row.hidden)), parse_hidden => set_view::SetView),
    field!("phase", field_phase, Text, |s, id| s.views.get(id).map(|row| row.phase.map_or_else(String::new, |phase| format!("{phase:?}"))), |_, id, value| set!(set_view::SetView, id, "phase", &Assigned::new(parse_phase(value)?))),
    field!("scale", field_scale, Number, |s, id| s.views.get(id).map(|row| row.scale.to_string()), parse_count => set_view::SetView),
    field!("detail", field_detail, Text, |s, id| s.views.get(id).map(|row| format!("{:?}", row.detail)), parse_detail => set_view::SetView),
];

/// 🔖️ A name no other view of the building has: `name`, else `name (2)`, `name (3)` and so on.
pub fn unique_name(snapshot: &ModelSnapshot, building: &str, name: &str) -> String {
    let taken = |candidate: &str| snapshot.views.values().any(|row| row.building == building && row.name == candidate);
    (1..).map(|count| if count == 1 { name.to_string() } else { format!("{name} ({count})") }).find(|candidate| !taken(candidate)).unwrap_or_else(|| name.to_string())
}

/// 🏗️ A new plan view of the storey `parent` (or the first storey): the `createEntity` of kind view.
pub fn create_view(snapshot: &ModelSnapshot, id: &str, parent: &str, name: &str) -> Created {
    let storey = container(&snapshot.storeys, parent, "bim.create.storey-missing")?;
    let building = snapshot.storeys[&storey].building.clone();
    let view = View::of_storey(&building, &unique_name(snapshot, &building, name), ViewKind::Plan, &storey);
    Ok(ModelMutation::CreateView(crate::mutations::create_view::CreateView { id: id.into(), view }))
}
