//! 🫧️ The overlay of the authoring tools: a window transient's preview turned into canvas records painted over the plan or section. Marks arrive in model metres (`x`, `y` or
//! section `u`, `v`) and leave in the window's mirrored logical space; sizes that must stay legible at any zoom (markers, handles, labels) are scaled by the metres one pixel spans.

use super::plane::P;
use super::select;
use super::session::{Preview, Shape, Style};
use crate::{ModelInference, ModelSnapshot};
use crate::editor::bim::kit::{path, path_record, text_record, Paint, Rgba};
use semio_framework_plugin::DslValue;
use std::f64::consts::TAU;

const ACCENT: Rgba = [0.1, 0.45, 0.95, 1.0];
const WARNING: Rgba = [0.9, 0.2, 0.2, 1.0];
const SNAP: Rgba = [0.95, 0.5, 0.1, 1.0];
const GUIDE: Rgba = [0.3, 0.32, 0.38, 0.75];
const WHITE: Rgba = [1.0, 1.0, 1.0, 1.0];
const TEXT: Rgba = [0.12, 0.14, 0.2, 1.0];
/// 🔘️ The radius of a handle dot and the half-size of a snap marker, in pixels.
const MARKER_PIXELS: f64 = 5.0;
/// 🔤️ The height of a label, in pixels.
const LABEL_PIXELS: f64 = 12.0;
/// 🖊️ The stroke of a mark, in pixels.
const STROKE_PIXELS: f64 = 1.5;

fn flip(p: P) -> [f64; 2] {
    [p[0], -p[1]]
}

fn tinted(rgba: Rgba, alpha: f64) -> Rgba {
    [rgba[0], rgba[1], rgba[2], alpha]
}

fn paint(style: Style, closed: bool, per_pixel: f64) -> Paint {
    let width = STROKE_PIXELS * per_pixel;
    match style {
        Style::Ghost => Paint { fill: closed.then(|| tinted(ACCENT, 0.14)), stroke: Some((ACCENT, width)), dash: None },
        Style::Guide => Paint { fill: None, stroke: Some((GUIDE, width * 0.7)), dash: Some([6.0 * per_pixel, 4.0 * per_pixel]) },
        Style::Snap => Paint { fill: None, stroke: Some((SNAP, width)), dash: None },
        Style::Handle => Paint { fill: Some(WHITE), stroke: Some((ACCENT, width)), dash: None },
        Style::Warning => Paint { fill: closed.then(|| tinted(WARNING, 0.2)), stroke: Some((WARNING, width)), dash: None },
        Style::Selection => Paint { fill: closed.then(|| tinted(ACCENT, 0.08)), stroke: Some((ACCENT, width * 0.8)), dash: Some([5.0 * per_pixel, 3.0 * per_pixel]) },
    }
}

fn disc(centre: P, radius: f64) -> Vec<P> {
    (0..12).map(|step| [centre[0] + radius * (TAU * f64::from(step) / 12.0).cos(), centre[1] + radius * (TAU * f64::from(step) / 12.0).sin()]).collect()
}

fn marker(kind: &str, at: P, size: f64) -> Vec<DslValue> {
    let [x, y] = at;
    match kind {
        "endpoint" => path(&[[x - size, y - size], [x + size, y - size], [x + size, y + size], [x - size, y + size]], true),
        "midpoint" => path(&[[x - size, y + size], [x + size, y + size], [x, y - size]], true),
        "intersection" => [path(&[[x - size, y - size], [x + size, y + size]], false), path(&[[x - size, y + size], [x + size, y - size]], false)].concat(),
        "grid" | "edge" => path(&[[x, y - size], [x + size, y], [x, y + size], [x - size, y]], true),
        "orthogonal" => [path(&[[x - size, y], [x + size, y]], false), path(&[[x, y - size], [x, y + size]], false)].concat(),
        _ => Vec::new(),
    }
}

/// 🫧️ The canvas records of a preview, `per_pixel` being the metres one pixel spans at the window's zoom.
pub fn records(preview: &Preview, per_pixel: f64) -> Vec<DslValue> {
    let mut records = Vec::new();
    for (index, mark) in preview.marks.iter().enumerate() {
        let id = format!("preview:{index}");
        let corners: Vec<P> = mark.corners().into_iter().map(flip).collect();
        let size = MARKER_PIXELS * per_pixel;
        match (mark.shape, corners.first()) {
            (Shape::Path, Some(_)) if corners.len() >= 2 => records.push(path_record(&id, "overlay", path(&corners, mark.closed), paint(mark.style, mark.closed, per_pixel))),
            (Shape::Dot, Some(centre)) => records.push(path_record(&id, "overlay", path(&disc(*centre, size), true), paint(mark.style, true, per_pixel))),
            (Shape::Label, Some(at)) => records.push(text_record(&id, (at[0], at[1]), &mark.text, LABEL_PIXELS * per_pixel, TEXT)),
            (Shape::Snap, Some(at)) => {
                let segments = marker(&mark.text, *at, size);
                if !segments.is_empty() {
                    records.push(path_record(&id, "overlay", segments, paint(Style::Snap, false, per_pixel)));
                }
            }
            _ => {}
        }
    }
    records
}

/// 🗺️ The overlay of a plan window: the live handles of the one selected wall while the select utility is armed, then the preview of the gesture in progress.
pub fn plan_records(snapshot: &ModelSnapshot, selected: &[String], utility: &str, preview: &Preview, per_pixel: f64) -> Vec<DslValue> {
    let handles = if utility == crate::editor::bim::utilities::DEFAULT_UTILITY { select::plan_marks(snapshot, selected) } else { Vec::new() };
    records(&Preview::of(handles.into_iter().chain(preview.marks.iter().cloned()).collect()), per_pixel)
}

/// 📐️ The overlay of a section window: the storey top handles while the select utility is armed, then the preview of the gesture in progress.
pub fn section_records(inference: &ModelInference, start: P, end: P, utility: &str, preview: &Preview, per_pixel: f64) -> Vec<DslValue> {
    let handles = if utility == crate::editor::bim::utilities::DEFAULT_UTILITY { select::section_marks(inference, start, end) } else { Vec::new() };
    records(&Preview::of(handles.into_iter().chain(preview.marks.iter().cloned()).collect()), per_pixel)
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
