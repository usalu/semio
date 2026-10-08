//! 📐️ BIM section window: a vertical cut through the model on a `Canvas2d` surface. The cut is the plane section of every `element-solids` mesh by the vertical plane
//! through the configured section line (the geometry API's `section_plane` and `chain`), painted as poché, with the storey levels as dashed datum lines. The section line is
//! given in plan coordinates; the section is seen from its right-hand side, `u` running to the right and the model's up axis running up.

#[path = "🎚️config/🦀️.rs"]
pub mod config;

use self::config::BimSectionWindowConfig;
use crate::editor::bim::interaction::BIM_ELEMENT_DOMAIN;
use crate::editor::bim::kit::{canvas_surface, meta_record, path, path_record, text_record, Paint, Rgba};
use crate::editor::bim::terminology::BimLabels;
use crate::standards::v1::subsets::any::schema::inferences::element_solids::ElementSolid;
use crate::{ModelInference, ModelSnapshot};
use semio_framework_geometry::section::{chain, section_plane};
use semio_framework_plugin::DslValue;
use semio_framework_plugin::InteractionRef;
use semio_framework_plugin::SurfaceKind;
use semio_framework_plugin::WindowKindDefinition;
use semio_framework_plugin::WindowOptions;
use semio_framework_ui_locale::LocalizedLabel;

//#region 🔖️Constants
pub const WINDOW_KIND_ID: &str = "bim-edit-section";
pub const BODY_KEY: &str = "bim.edit.section";
const SURFACE_ID: &str = "bim.edit.section2d/section";
/// 📏️ The classification band, in metres, of the plane section.
const SECTION_EPSILON: f64 = 1e-6;
const TEXT_SIZE: f64 = 0.22;
const CUT_FILL: Rgba = [0.46, 0.48, 0.52, 1.0];
const CUT_STROKE: Rgba = [0.08, 0.08, 0.1, 1.0];
const LEVEL_STROKE: Rgba = [0.24, 0.45, 0.78, 0.9];
const TEXT_COLOUR: Rgba = [0.22, 0.24, 0.28, 1.0];
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the app manifest by `crate::editor::bim::create_bim_app`.
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition {
        initial_utility_id: crate::editor::bim::utilities::initial(),
        id: WINDOW_KIND_ID.into(),
        label: LocalizedLabel::native(BimLabels::NATIVE_EN.window_section.as_str(), BimLabels::NATIVE_DE.window_section.as_str()),
        body_key: BODY_KEY.into(),
        surface_kind: SurfaceKind::Canvas2d,
        icon_id: "scissors".into(),
        options: WindowOptions::default(),
        actions: Vec::new(),
        utilities: crate::editor::bim::utilities::for_window(WINDOW_KIND_ID),
        interactions: vec![InteractionRef::new(BIM_ELEMENT_DOMAIN)],
        params_schema: None,
        artifact_snapshot_schema: None,
        input_event_schema: None,
        output_schema: None,
        capabilities: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Cut
/// ✂️ The length of the section line in metres.
pub fn line_length(config: &BimSectionWindowConfig) -> f64 {
    (config.end_x - config.start_x).hypot(config.end_y - config.start_y)
}

/// ✂️ One chained cut of an element: its points in section space `(u, v)` and whether it closes.
pub struct Cut {
    pub element: String,
    pub points: Vec<[f64; 2]>,
    pub closed: bool,
}

/// ✂️ The cuts of every solid by the vertical plane through the section line, in element id order; a degenerate line cuts nothing.
pub fn cuts(inference: &ModelInference, config: &BimSectionWindowConfig) -> Vec<Cut> {
    let length = line_length(config);
    if length < 1e-9 {
        return Vec::new();
    }
    let (dx, dy) = ((config.end_x - config.start_x) / length, (config.end_y - config.start_y) / length);
    let (origin, normal, along) = ([config.start_x, config.start_y, 0.0], [dy, -dx, 0.0], [dx, dy, 0.0]);
    let crosses = |solid: &ElementSolid| {
        let (min, max) = (solid.bounds.min, solid.bounds.max);
        let side = |x: f64, y: f64| (x - origin[0]) * normal[0] + (y - origin[1]) * normal[1];
        let corners = [side(min.x, min.y), side(min.x, max.y), side(max.x, min.y), side(max.x, max.y)];
        corners.iter().any(|d| *d <= SECTION_EPSILON) && corners.iter().any(|d| *d >= -SECTION_EPSILON)
    };
    let mut cuts = Vec::new();
    for (element, solid) in inference.element_solids.iter().filter(|(_, solid)| crosses(solid)) {
        let segments = section_plane(&solid.mesh(), origin, normal, along, SECTION_EPSILON);
        for polyline in chain(&segments, SECTION_EPSILON) {
            cuts.push(Cut { element: element.clone(), points: polyline.points.iter().map(|p| [p.x, p.y]).collect(), closed: polyline.closed });
        }
    }
    cuts
}
//#endregion 🔖️Cut

//#region 🔖️Render
/// 🎨️ The records of the section: level datums, then the cut poché, then the label.
pub fn records(snapshot: &ModelSnapshot, inference: &ModelInference, config: &BimSectionWindowConfig, utility: &str) -> Vec<DslValue> {
    let length = line_length(config);
    let mut records = vec![meta_record(utility)];
    for (storey, level) in &inference.storey_levels {
        let y = -level.elevation;
        let name = snapshot.storeys.get(storey).map_or(storey.as_str(), |row| row.name.as_str());
        records.push(path_record(&format!("level:{storey}"), "node", path(&[[-0.5, y], [length + 0.5, y]], false), Paint { fill: None, stroke: Some((LEVEL_STROKE, 0.008)), dash: Some([0.2, 0.1]) }));
        records.push(text_record(&format!("level-label:{storey}"), (length + 0.6, y), &format!("{name} {:+.2}", level.elevation), TEXT_SIZE, TEXT_COLOUR));
    }
    for (index, cut) in cuts(inference, config).iter().enumerate() {
        let points: Vec<[f64; 2]> = cut.points.iter().map(|[u, v]| [*u, -*v]).collect();
        let paint = Paint { fill: cut.closed.then_some(CUT_FILL), stroke: Some((CUT_STROKE, 0.03)), dash: None };
        records.push(path_record(&format!("cut:{}:{index}", cut.element), "node", path(&points, cut.closed), paint));
    }
    records
}

/// 📐️ The framing rectangle: the line's length wide, the storeys' full height tall.
fn framing_bounds(inference: &ModelInference, config: &BimSectionWindowConfig) -> Option<[f64; 4]> {
    let low = inference.storey_levels.values().map(|level| level.elevation).fold(f64::INFINITY, f64::min);
    let high = inference.storey_levels.values().map(|level| level.top_elevation).fold(f64::NEG_INFINITY, f64::max);
    (low.is_finite() && high.is_finite()).then(|| [-0.5, -high - 0.5, line_length(config) + 0.5, -low + 0.5])
}

/// 📐️ Renders the section window.
pub fn render(snapshot: &ModelSnapshot, inference: &ModelInference, config: &BimSectionWindowConfig, utility: &str, revision: u32) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    render_over(snapshot, inference, config, utility, revision, &[])
}

/// 📐️ [`render`] with the records of the authoring overlay (storey top handles, the gesture preview) painted last, over everything.
pub fn render_over(snapshot: &ModelSnapshot, inference: &ModelInference, config: &BimSectionWindowConfig, utility: &str, revision: u32, overlay: &[DslValue]) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let framing = (!config.framed).then(|| framing_bounds(inference, config).map(|bounds| semio_framework_plugin::Canvas2dFraming { revision, bounds, padding: 48.0 })).flatten();
    let mut records = records(snapshot, inference, config, utility);
    records.extend(overlay.iter().cloned());
    canvas_surface(SURFACE_ID, config.viewport, framing, &records)
}
//#endregion 🔖️Render

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
