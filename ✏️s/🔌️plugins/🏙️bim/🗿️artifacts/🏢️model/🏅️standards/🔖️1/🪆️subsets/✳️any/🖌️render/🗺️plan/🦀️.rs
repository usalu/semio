//! 🗺️ Shared Canvas2d scene of the BIM plan windows: one layer record per `plan-linework` primitive of a storey. Canvas coordinates are the
//! plan coordinates in metres with the y axis mirrored (`canvas = (x, -y)`), so north points up on screen; `canvas_point` and `plan_point` are
//! the only conversions and the editor picks through them. Bulged segments are flattened within `FLATTEN_TOLERANCE`. Stroke widths are world
//! units (metres) because the canvas host scales them with the camera.

use crate::standards::v1::subsets::any::schema::inferences::plan_linework::{PlanKind, PlanLinework, PlanPolyline, PlanRegion, PlanStyle, PlanText, PlanVertex};
use semio_framework_2d::PathSegment;
use semio_framework_geometry::bulge::BulgeSeg;
use semio_framework_geometry::Point;
use semio_framework_plugin::{Canvas2dFraming, Canvas2dScene};
use semio_framework_value::{DslValue, ToValue};

//#region 🔖️Constants
/// 📏️ Sagitta of a flattened arc in metres.
pub const FLATTEN_TOLERANCE: f64 = 5e-4;
/// 🔤️ Text height in metres.
pub const TEXT_SIZE: f64 = 0.22;
const ACCENT: [f64; 4] = [0.95, 0.5, 0.1, 1.0];
const FRAMING_PADDING: f64 = 48.0;
//#endregion 🔖️Constants

//#region 🔖️Coordinates
/// 🧭️ Plan coordinates (x east, y north) to canvas coordinates (y down).
pub fn canvas_point(x: f64, y: f64) -> [f64; 2] {
    [x, -y]
}

/// 🧭️ Canvas coordinates back to plan coordinates.
pub fn plan_point(x: f64, y: f64) -> [f64; 2] {
    [x, -y]
}

/// 📦️ The bounds of a plan as canvas `[x0, y0, x1, y1]`, or `None` for a plan with no primitive.
pub fn canvas_bounds(plan: &PlanLinework) -> Option<[f64; 4]> {
    let empty = plan.regions.is_empty() && plan.polylines.is_empty() && plan.texts.is_empty();
    (!empty).then(|| [plan.bounds.min_x, -plan.bounds.max_y, plan.bounds.max_x, -plan.bounds.min_y])
}
//#endregion 🔖️Coordinates

//#region 🔖️Paths
fn ring(vertices: &[PlanVertex], closed: bool) -> Vec<PathSegment> {
    let Some(first) = vertices.first() else { return Vec::new() };
    let mut points = vec![Point::new(first.x, first.y)];
    let spans = if closed { vertices.len() } else { vertices.len() - 1 };
    for index in 0..spans {
        let (a, b) = (vertices[index], vertices[(index + 1) % vertices.len()]);
        BulgeSeg::new(Point::new(a.x, a.y), Point::new(b.x, b.y), a.bulge).flatten_into(FLATTEN_TOLERANCE, &mut points);
    }
    if closed && points.len() > 1 && (points[0].x, points[0].y) == (points[points.len() - 1].x, points[points.len() - 1].y) {
        points.pop();
    }
    let mut segments: Vec<PathSegment> = points.iter().enumerate().map(|(index, point)| if index == 0 { PathSegment::Move { to: canvas_point(point.x, point.y) } } else { PathSegment::Line { to: canvas_point(point.x, point.y) } }).collect();
    if closed {
        segments.push(PathSegment::Close);
    }
    segments
}
//#endregion 🔖️Paths

//#region 🔖️Paint
struct Paint {
    fill: Option<[f64; 4]>,
    stroke: [f64; 4],
    width: f64,
    dash: Vec<f64>,
}

fn paint(kind: PlanKind, style: PlanStyle, filled: bool, selected: bool) -> Paint {
    let base = match style {
        PlanStyle::Cut => Paint { fill: filled.then_some([0.12, 0.12, 0.14, 1.0]), stroke: [0.1, 0.1, 0.12, 1.0], width: 0.025, dash: Vec::new() },
        PlanStyle::Projection => Paint { fill: filled.then_some([0.78, 0.78, 0.8, 1.0]), stroke: [0.3, 0.3, 0.34, 1.0], width: 0.012, dash: Vec::new() },
        PlanStyle::Hidden => Paint { fill: filled.then_some([0.0, 0.0, 0.0, 0.05]), stroke: [0.5, 0.5, 0.55, 1.0], width: 0.01, dash: vec![0.12, 0.08] },
        PlanStyle::Annotation => Paint { fill: filled.then_some([0.2, 0.4, 0.7, 0.1]), stroke: [0.2, 0.4, 0.7, 1.0], width: 0.008, dash: Vec::new() },
    };
    let base = if matches!(kind, PlanKind::WindowGlazing) { Paint { stroke: [0.3, 0.55, 0.8, 1.0], ..base } } else { base };
    if selected {
        Paint { fill: base.fill.map(|_| [ACCENT[0], ACCENT[1], ACCENT[2], 0.35]), stroke: ACCENT, width: base.width + 0.012, dash: base.dash }
    } else {
        base
    }
}
//#endregion 🔖️Paint

//#region 🔖️Records
fn record(id: &str, role: &str, segments: Vec<PathSegment>, paint: &Paint) -> DslValue {
    let color = |values: [f64; 4]| values.to_vec().to_value();
    let stroke = [
        ("color".to_string(), color(paint.stroke)),
        ("width".to_string(), DslValue::float(paint.width)),
        ("cap".to_string(), DslValue::String("round".to_string())),
        ("join".to_string(), DslValue::String("round".to_string())),
    ]
    .into_iter()
    .chain((!paint.dash.is_empty()).then(|| ("dash".to_string(), paint.dash.to_value())));
    DslValue::object([
        ("id".to_string(), DslValue::String(id.to_string())),
        ("role".to_string(), DslValue::String(role.to_string())),
        ("transform".to_string(), [1.0, 0.0, 0.0, 1.0, 0.0, 0.0_f64].to_value()),
        ("segments".to_string(), segments.to_value()),
        ("fill".to_string(), paint.fill.map_or(DslValue::Null, |fill| DslValue::object([("kind".to_string(), DslValue::String("solid".to_string())), ("color".to_string(), color(fill))]))),
        ("stroke".to_string(), DslValue::object(stroke)),
        ("opacity".to_string(), DslValue::float(1.0)),
        ("blendMode".to_string(), DslValue::String("normal".to_string())),
        ("visible".to_string(), DslValue::Bool(true)),
        ("fillRule".to_string(), DslValue::String("evenodd".to_string())),
    ])
}

fn region_record(region: &PlanRegion, selected: bool) -> DslValue {
    let segments = std::iter::once(&region.outer).chain(region.holes.iter()).flat_map(|loop_| ring(loop_, true)).collect();
    record(&region.id, "node", segments, &paint(region.kind, region.style, true, selected))
}

fn polyline_record(line: &PlanPolyline, selected: bool) -> DslValue {
    record(&line.id, "node", ring(&line.vertices, line.closed), &paint(line.kind, line.style, false, selected))
}

/// 🪧️ Where an annotation text starts and how it turns on the canvas. The plan places it by the middle of its baseline; the canvas draws from the left of the baseline, so the start moves back by half the
/// estimated advance along the text direction, and the transform rotates the text about that start (the canvas y axis is mirrored, so a counter-clockwise plan angle is a clockwise canvas angle).
fn notation_placement(text: &PlanText, size: f64) -> ([f64; 2], [f64; 6]) {
    let half = crate::standards::v1::subsets::any::schema::inferences::annotation_layout::text_width(&text.label, size) / 2.0;
    let (sin, cos) = text.rotation.sin_cos();
    let start = [text.x - half * cos, text.y - half * sin];
    if text.rotation == 0.0 {
        return (start, [1.0, 0.0, 0.0, 1.0, 0.0, 0.0]);
    }
    let [px, py] = canvas_point(start[0], start[1]);
    (start, [cos, -sin, sin, cos, px - cos * px - sin * py, py + sin * px - cos * py])
}

fn text_record(text: &PlanText, selected: bool) -> DslValue {
    let content = [Some(text.label.clone()), (!text.detail.is_empty()).then(|| text.detail.clone()), text.measure.map(|area| format!("{area:.1} m\u{b2}"))].into_iter().flatten().collect::<Vec<String>>().join("\n");
    let size = if text.kind.is_notation() && text.height > 0.0 { text.height } else { TEXT_SIZE };
    let (start, transform) = if text.kind.is_notation() { notation_placement(text, size) } else { ([text.x, text.y], [1.0, 0.0, 0.0, 1.0, 0.0, 0.0_f64]) };
    let [x, y] = canvas_point(start[0], start[1]);
    let color = if selected { ACCENT } else { paint(text.kind, text.style, true, false).stroke };
    DslValue::object([
        ("id".to_string(), DslValue::String(text.id.clone())),
        ("role".to_string(), DslValue::String("node".to_string())),
        ("transform".to_string(), transform.to_value()),
        ("segments".to_string(), Vec::<PathSegment>::new().to_value()),
        ("x".to_string(), DslValue::float(x)),
        ("y".to_string(), DslValue::float(y - size)),
        ("fill".to_string(), DslValue::object([("kind".to_string(), DslValue::String("solid".to_string())), ("color".to_string(), color.to_vec().to_value())])),
        ("text".to_string(), DslValue::object([("content".to_string(), DslValue::String(content)), ("size".to_string(), DslValue::float(size))])),
        ("opacity".to_string(), DslValue::float(1.0)),
        ("blendMode".to_string(), DslValue::String("normal".to_string())),
        ("visible".to_string(), DslValue::Bool(true)),
    ])
}

fn style_rank(style: PlanStyle) -> u8 {
    match style {
        PlanStyle::Hidden => 0,
        PlanStyle::Projection => 1,
        PlanStyle::Cut => 2,
        PlanStyle::Annotation => 3,
    }
}

/// 🗺️ The layer records of one storey in paint order: filled regions (poche) first, then paths from hidden to annotation, then text. An element in
/// `selected` is drawn in the accent colour.
pub fn layers(plan: &PlanLinework, selected: &[String]) -> Vec<DslValue> {
    let marked = |element: &str| selected.iter().any(|id| id == element);
    let mut regions: Vec<&PlanRegion> = plan.regions.iter().collect();
    regions.sort_by_key(|region| style_rank(region.style));
    let mut lines: Vec<&PlanPolyline> = plan.polylines.iter().collect();
    lines.sort_by_key(|line| style_rank(line.style));
    regions
        .into_iter()
        .map(|region| region_record(region, marked(&region.element)))
        .chain(lines.into_iter().map(|line| polyline_record(line, marked(&line.element))))
        .chain(plan.texts.iter().map(|text| text_record(text, marked(&text.element))))
        .collect()
}
//#endregion 🔖️Records

//#region 🔖️Scene
/// 🔢️ A framing revision that changes with the storey, so switching the storey refits the window once.
pub fn framing_revision(storey: &str) -> u32 {
    storey.bytes().fold(0x811c_9dc5_u32, |hash, byte| (hash ^ u32::from(byte)).wrapping_mul(0x0100_0193))
}

/// 🗺️ The Canvas2d scene of a storey plan: its layers, the stored viewport and, while the window is not framed, the fit to the plan bounds.
pub fn scene(plan: Option<&PlanLinework>, viewport: &store::Viewport2d, framed: bool, selected: &[String]) -> Canvas2dScene {
    let layers = plan.map(|plan| layers(plan, selected)).unwrap_or_default();
    let mut scene = Canvas2dScene::base(viewport.x, viewport.y, viewport.zoom, semio_framework_pack_json::to_json_string(&DslValue::Array(layers)));
    scene.framing = plan.filter(|_| !framed).and_then(|plan| canvas_bounds(plan).map(|bounds| Canvas2dFraming { revision: framing_revision(&plan.storey), bounds, padding: FRAMING_PADDING }));
    scene
}
//#endregion 🔖️Scene

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
