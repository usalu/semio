//! 🔤️ Field codecs of the BIM entity table: the text and parse helpers of every field row kind. See [`super`].

use semio_s_artifact_bim_model_schema::standards::v1::subsets::any::schema::snapshot::values::{Axis, Baluster, DoorLeaves, EndJoin, Infill, LocationLine, MaterialCategory, OpeningKind, Point2, Profile, RiserKind, RoofShape, Slope, StairFlight, StairStringer, StringerKind, Swing, TopConstraint, Vertex};
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
use semio_framework_plugin::DslValue;
use semio_framework_value::{FromValue, ToValue};
use std::collections::BTreeMap;

//#region 🔖️Helpers
pub fn number(value: f64) -> String {
    format!("{value}")
}

pub fn point(x: f64, y: f64) -> String {
    format!("{}, {}", number(x), number(y))
}

pub fn axis_text(axis: &Axis) -> String {
    match axis {
        Axis::Line { start, end } => format!("{} → {}", point(start.x, start.y), point(end.x, end.y)),
        Axis::Arc { start, end, bulge } => format!("{} → {} ⌒ {}", point(start.x, start.y), point(end.x, end.y), number(*bulge)),
    }
}

pub fn top_text(top: &TopConstraint) -> String {
    match top {
        TopConstraint::Unconnected { height } => format!("{}", number(*height)),
        TopConstraint::StoreyTop { offset } => format!("storey top {}", number(*offset)),
        TopConstraint::Storey { storey, offset } => format!("{storey} {}", number(*offset)),
        TopConstraint::Roof { roof, offset } => format!("roof {roof} {}", number(*offset)),
        TopConstraint::Slab { slab, offset } => format!("slab {slab} {}", number(*offset)),
        TopConstraint::Ceiling { ceiling, offset } => format!("ceiling {ceiling} {}", number(*offset)),
    }
}

pub fn opening_text(kind: &OpeningKind) -> String {
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

pub fn keys<T>(map: &BTreeMap<String, T>) -> Vec<String> {
    map.keys().cloned().collect()
}

pub fn first<T>(map: &BTreeMap<String, T>) -> Option<String> {
    map.keys().next().cloned()
}

pub fn parse_number(text: &str) -> Option<f64> {
    text.trim().parse().ok()
}

pub fn parse_optional_number(text: &str) -> Option<Option<f64>> {
    let text = text.trim();
    if text.is_empty() {
        return Some(None);
    }
    text.parse().ok().map(Some)
}

pub fn type_sill(snapshot: &ModelSnapshot, kind: &OpeningKind) -> f64 {
    match kind {
        OpeningKind::Window { window_type } => snapshot.window_types.get(window_type).map_or(0.0, |row| row.sill),
        _ => 0.0,
    }
}

pub fn parse_int(text: &str) -> Option<i32> {
    text.trim().parse().ok()
}

pub fn parse_count(text: &str) -> Option<u32> {
    text.trim().parse().ok()
}

pub fn parse_text(text: &str) -> Option<String> {
    Some(text.trim().to_string())
}

pub fn parse_flag(text: &str) -> Option<bool> {
    text.trim().parse().ok()
}

pub fn parse_point(text: &str) -> Option<Point2> {
    let (x, y) = text.split_once(',')?;
    Some(Point2 { x: parse_number(x)?, y: parse_number(y)? })
}

pub fn parse_axis(text: &str) -> Option<Axis> {
    let (ends, bulge) = match text.split_once('⌒') {
        Some((ends, bulge)) => (ends, Some(parse_number(bulge)?)),
        None => (text, None),
    };
    let (start, end) = ends.split_once('→')?;
    let (start, end) = (parse_point(start)?, parse_point(end)?);
    Some(bulge.map_or(Axis::Line { start, end }, |bulge| Axis::Arc { start, end, bulge }))
}

pub fn parse_top(text: &str) -> Option<TopConstraint> {
    let text = text.trim();
    if let Some(height) = parse_number(text) {
        return Some(TopConstraint::Unconnected { height });
    }
    if let Some(offset) = text.strip_prefix("storey top") {
        return Some(TopConstraint::StoreyTop { offset: parse_number(offset)? });
    }
    if let Some(attached) = parse_attach(text) {
        return Some(attached);
    }
    let (storey, offset) = text.rsplit_once(' ')?;
    Some(TopConstraint::Storey { storey: storey.trim().to_string(), offset: parse_number(offset)? })
}

fn parse_attach(text: &str) -> Option<TopConstraint> {
    let (keyword, rest) = text.split_once(' ')?;
    let (target, offset) = rest.trim().rsplit_once(' ')?;
    let (target, offset) = (target.trim().to_string(), parse_number(offset)?);
    match keyword {
        "roof" => Some(TopConstraint::Roof { roof: target, offset }),
        "slab" => Some(TopConstraint::Slab { slab: target, offset }),
        "ceiling" => Some(TopConstraint::Ceiling { ceiling: target, offset }),
        _ => None,
    }
}

pub fn variant<T: Clone + std::fmt::Debug>(text: &str, all: &[T]) -> Option<T> {
    all.iter().find(|candidate| format!("{candidate:?}").eq_ignore_ascii_case(text.trim())).cloned()
}

pub fn profile_text(profile: &Profile) -> String {
    match profile {
        Profile::Rectangle { width, depth } => format!("rect {} x {}", number(*width), number(*depth)),
        Profile::Circle { diameter } => format!("circle {}", number(*diameter)),
        Profile::IShape { width, depth, web, flange } => format!("i {} x {} web {} flange {}", number(*width), number(*depth), number(*web), number(*flange)),
        Profile::Custom { outline } => format!("custom {}", loop_text(outline)),
        Profile::Family { family } => format!("family {family}"),
    }
}

pub fn size_pair(text: &str) -> Option<(f64, f64)> {
    let (first, second) = text.split_once(['x', '×'])?;
    Some((parse_number(first)?, parse_number(second)?))
}

pub fn parse_profile(text: &str) -> Option<Profile> {
    let text = text.trim();
    let (name, rest) = text.split_once(' ').map_or((text, ""), |(name, rest)| (name, rest.trim()));
    match name.to_ascii_lowercase().as_str() {
        "rect" | "rectangle" => size_pair(rest).map(|(width, depth)| Profile::Rectangle { width, depth }),
        "circle" => parse_number(rest).map(|diameter| Profile::Circle { diameter }),
        "i" => {
            let (size, tail) = rest.split_once("web")?;
            let (web, flange) = tail.split_once("flange")?;
            let (width, depth) = size_pair(size)?;
            Some(Profile::IShape { width, depth, web: parse_number(web)?, flange: parse_number(flange)? })
        }
        "custom" => parse_loop(rest).map(|outline| Profile::Custom { outline }),
        "family" => Some(rest).filter(|family| !family.is_empty()).map(|family| Profile::Family { family: family.to_string() }),
        _ => None,
    }
}

pub fn stringer_text(stringer: &StairStringer) -> String {
    format!("{:?} {} × {}", stringer.kind, number(stringer.width), number(stringer.depth))
}

pub fn parse_stringer(text: &str) -> Option<StairStringer> {
    let (kind, size) = text.trim().split_once(' ').unwrap_or((text.trim(), "0 × 0"));
    let (width, depth) = size.split_once('×')?;
    let kind = variant(kind, &[StringerKind::None, StringerKind::Closed, StringerKind::Open, StringerKind::Mono])?;
    Some(StairStringer { kind, width: parse_number(width)?, depth: parse_number(depth)? })
}

pub fn parse_riser(text: &str) -> Option<RiserKind> {
    variant(text, &[RiserKind::Open, RiserKind::Closed])
}

pub fn infill_text(infill: &Infill) -> String {
    match infill {
        Infill::None => "none".to_string(),
        Infill::Glass { thickness } => format!("glass {}", number(*thickness)),
        Infill::Panel { thickness } => format!("panel {}", number(*thickness)),
    }
}

pub fn parse_infill(text: &str) -> Option<Infill> {
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

pub fn baluster_text(baluster: Option<&Baluster>) -> String {
    baluster.map_or_else(|| "none".to_string(), |row| format!("{} @ {}", profile_text(&row.profile), number(row.spacing)))
}

pub fn parse_baluster(text: &str) -> Option<Option<Baluster>> {
    let text = text.trim();
    if text.eq_ignore_ascii_case("none") {
        return Some(None);
    }
    let (profile, spacing) = text.split_once('@')?;
    Some(Some(Baluster { profile: parse_profile(profile)?, spacing: parse_number(spacing)? }))
}

/// 🔗️ The text of the authored join preference of a wall end: `auto` when the geometry decides, else `miter`, `butt` or `none`.
pub fn join_text(join: Option<EndJoin>) -> String {
    join.map_or("auto".to_string(), |join| format!("{join:?}").to_ascii_lowercase())
}

/// 🔗️ The join preference a text names: `auto` (the geometry decides), `miter`, `butt` or `none`.
pub fn parse_join(text: &str) -> Option<Option<EndJoin>> {
    let text = text.trim().to_ascii_lowercase();
    match text.as_str() {
        "auto" => Some(None),
        "miter" => Some(Some(EndJoin::Miter)),
        "butt" => Some(Some(EndJoin::Butt)),
        "none" => Some(Some(EndJoin::None)),
        _ => None,
    }
}

pub fn parse_location(text: &str) -> Option<LocationLine> {
    variant(text, &[LocationLine::Center, LocationLine::Interior, LocationLine::Exterior, LocationLine::CoreCenter])
}

pub fn parse_category(text: &str) -> Option<MaterialCategory> {
    use semio_s_artifact_bim_model_schema::standards::v1::subsets::any::schema::snapshot::values::MaterialCategory::*;
    variant(text, &[Concrete, Masonry, Wood, Metal, Glass, Insulation, Finish, Membrane, Other])
}

pub fn parse_leaves(text: &str) -> Option<DoorLeaves> {
    variant(text, &[DoorLeaves::Single, DoorLeaves::Double])
}

pub fn parse_swing(text: &str) -> Option<Swing> {
    variant(text, &[Swing::Left, Swing::Right])
}

/// 🔷️ The text of a vertex loop: `x, y` per vertex, `⌒ bulge` after a bulging one, joined by `; `.
pub fn loop_text(vertices: &[Vertex]) -> String {
    vertices.iter().map(|vertex| if vertex.bulge == 0.0 { point(vertex.point.x, vertex.point.y) } else { format!("{} ⌒ {}", point(vertex.point.x, vertex.point.y), number(vertex.bulge)) }).collect::<Vec<_>>().join("; ")
}

/// 🔷️ The vertex loop a text names: at least three `x, y` or `x, y ⌒ bulge` vertices joined by `;`.
pub fn parse_loop(text: &str) -> Option<Vec<Vertex>> {
    let vertices = text
        .split(';')
        .map(str::trim)
        .filter(|part| !part.is_empty())
        .map(|part| {
            let (at, bulge) = match part.split_once('⌒') {
                Some((at, bulge)) => (at, parse_number(bulge)?),
                None => (part, 0.0),
            };
            Some(Vertex { point: parse_point(at)?, bulge })
        })
        .collect::<Option<Vec<_>>>()?;
    (vertices.len() >= 3).then_some(vertices)
}

/// 🕳️ The text of the hole loops of a slab, joined by ` | `.
pub fn holes_text(holes: &[Vec<Vertex>]) -> String {
    holes.iter().map(|hole| loop_text(hole)).collect::<Vec<_>>().join(" | ")
}

/// 🕳️ The hole loops a text names; empty text is no hole.
pub fn parse_holes(text: &str) -> Option<Vec<Vec<Vertex>>> {
    text.split('|').map(str::trim).filter(|part| !part.is_empty()).map(parse_loop).collect()
}

/// 🏠️ The text of a roof shape: its name and its angles in radians and lengths in metres.
pub fn roof_shape_text(shape: &RoofShape) -> String {
    match shape {
        RoofShape::Flat => "flat".to_string(),
        RoofShape::Shed { pitch, direction } => format!("shed {} {}", number(*pitch), number(*direction)),
        RoofShape::Gable { pitch, ridge_direction } => format!("gable {} {}", number(*pitch), number(*ridge_direction)),
        RoofShape::Hip { pitch } => format!("hip {}", number(*pitch)),
        RoofShape::Mansard { lower_pitch, upper_pitch, break_height } => format!("mansard {} {} {}", number(*lower_pitch), number(*upper_pitch), number(*break_height)),
    }
}

fn words_of(text: &str) -> Option<(String, Vec<String>)> {
    let mut words = text.split_whitespace().map(str::to_lowercase);
    Some((words.next()?, words.collect()))
}

fn numbers_of(words: &[String]) -> Option<Vec<f64>> {
    words.iter().map(|word| word.parse::<f64>().ok().filter(|value| value.is_finite())).collect()
}

/// 🏠️ The roof shape a text names: `flat`, `shed pitch direction`, `gable pitch ridge-direction`, `hip pitch` or `mansard lower upper break`.
pub fn parse_roof_shape(text: &str) -> Option<RoofShape> {
    let (name, words) = words_of(text)?;
    match (name.as_str(), numbers_of(&words)?.as_slice()) {
        ("flat", []) => Some(RoofShape::Flat),
        ("shed", [pitch, direction]) => Some(RoofShape::Shed { pitch: *pitch, direction: *direction }),
        ("gable", [pitch, ridge_direction]) => Some(RoofShape::Gable { pitch: *pitch, ridge_direction: *ridge_direction }),
        ("hip", [pitch]) => Some(RoofShape::Hip { pitch: *pitch }),
        ("mansard", [lower_pitch, upper_pitch, break_height]) => Some(RoofShape::Mansard { lower_pitch: *lower_pitch, upper_pitch: *upper_pitch, break_height: *break_height }),
        _ => None,
    }
}

/// 🪜️ The text of a stair flight: its name and its split, gap, radius and sweep.
pub fn flight_text(flight: &StairFlight) -> String {
    match flight {
        StairFlight::Straight => "straight".to_string(),
        StairFlight::LTurn { split, turn } => format!("l-turn {} {}", number(*split), if matches!(turn, semio_s_artifact_bim_model_schema::standards::v1::subsets::any::schema::snapshot::values::Turn::Left) { "left" } else { "right" }),
        StairFlight::UTurn { gap } => format!("u-turn {}", number(*gap)),
        StairFlight::Spiral { radius, sweep } => format!("spiral {} {}", number(*radius), number(*sweep)),
    }
}

/// 🪜️ The stair flight a text names: `straight`, `l-turn split left|right`, `u-turn gap` or `spiral radius sweep`.
pub fn parse_flight(text: &str) -> Option<StairFlight> {
    let (name, words) = words_of(text)?;
    match (name.as_str(), words.as_slice()) {
        ("straight", []) => Some(StairFlight::Straight),
        ("l-turn", [split, turn]) => Some(StairFlight::LTurn { split: split.parse().ok().filter(|value: &f64| value.is_finite())?, turn: match turn.as_str() { "left" => semio_s_artifact_bim_model_schema::standards::v1::subsets::any::schema::snapshot::values::Turn::Left, "right" => semio_s_artifact_bim_model_schema::standards::v1::subsets::any::schema::snapshot::values::Turn::Right, _ => return None } }),
        ("u-turn", [gap]) => Some(StairFlight::UTurn { gap: gap.parse().ok().filter(|value: &f64| value.is_finite())? }),
        ("spiral", [radius, sweep]) => Some(StairFlight::Spiral { radius: radius.parse().ok().filter(|value: &f64| value.is_finite())?, sweep: sweep.parse().ok().filter(|value: &f64| value.is_finite())? }),
        _ => None,
    }
}

/// 🧩️ Builds the leaf of a `set-*` mutation that names exactly one parameter: the leaf's own value decoding fills every other slot with "unchanged".
pub fn partial<L: FromValue, V: ToValue + ?Sized>(id: &str, field: &str, value: &V) -> Option<L> {
    L::from_value(DslValue::object([("id".to_string(), DslValue::String(id.into())), (field.to_string(), value.to_value())])).ok()
}

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
//#endregion 🔖️Helpers
