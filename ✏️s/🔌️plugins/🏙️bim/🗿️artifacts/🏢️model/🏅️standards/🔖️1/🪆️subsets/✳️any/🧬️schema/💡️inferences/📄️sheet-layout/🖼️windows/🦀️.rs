//! 🖼️ The paper frame of a sheet and the window of a viewport: where the drawing area of the paper lies, and how large the window of a viewport is on the paper (the crop of the viewport, else the bounds of the drawing plus a little air, at the scale of the viewport)
//! together with the map from view metres to paper millimetres. All in millimetres from the top left corner of the sheet, x to the right and y downward.
//! 📎 https://en.wikipedia.org/wiki/Technical_drawing#Sheet_sizes_and_frame

use super::{PaperRect, PlacedViewport, ViewMap};
use crate::standards::v1::subsets::any::schema::inferences::view_linework::ViewLinework;
use crate::{View, Viewport};

/// 📏️ The margin of the frame at the binding edge (the left side), as ISO 5457 asks for.
pub const BINDING_MARGIN: f64 = 20.0;

/// 📏️ The margin of the frame on the other three sides.
pub const MARGIN: f64 = 10.0;

/// 📏️ The air around an uncropped drawing, so the labels at the edge of the drawing stay inside the window.
pub const WINDOW_PADDING: f64 = 5.0;

/// 📏️ The smallest side of a window, so an empty or tiny drawing stays selectable on the paper.
pub const MIN_WINDOW: f64 = 10.0;

/// 📏️ Paper millimetres per model metre at the drawing scale `1:scale`.
pub fn mm_per_metre(scale: u32) -> f64 {
    1000.0 / f64::from(scale.max(1))
}

/// 🖼️ The drawing frame of a sheet of the given size: the paper without its margins.
pub fn frame_of(width: f64, height: f64) -> PaperRect {
    PaperRect { x: BINDING_MARGIN, y: MARGIN, width: (width - BINDING_MARGIN - MARGIN).max(0.0), height: (height - 2.0 * MARGIN).max(0.0) }
}

/// 🪟️ The window of `viewport` showing `view` whose linework is `drawing` (none when it has not been inferred): its top left corner lies at the position of the viewport, its size is the crop (else the bounds of the drawing with
/// [`WINDOW_PADDING`] around it) at the scale of the viewport, at least [`MIN_WINDOW`] on each side.
pub fn place(id: &str, viewport: &Viewport, view: &View, drawing: Option<&ViewLinework>) -> PlacedViewport {
    let mm = mm_per_metre(viewport.scale);
    let (x0, y0, x1, y1, cropped) = match (&viewport.crop, drawing.map(|drawing| drawing.lines.bounds)) {
        (Some(crop), _) => (crop.min.x, crop.min.y, crop.max.x, crop.max.y, true),
        (None, Some(bounds)) => {
            let air = WINDOW_PADDING / mm;
            (bounds.min_x - air, bounds.min_y - air, bounds.max_x + air, bounds.max_y + air, false)
        }
        (None, None) => (0.0, 0.0, MIN_WINDOW / mm, MIN_WINDOW / mm, false),
    };
    let window = PaperRect { x: viewport.position.x, y: viewport.position.y, width: ((x1 - x0) * mm).max(MIN_WINDOW), height: ((y1 - y0) * mm).max(MIN_WINDOW) };
    let empty = drawing.is_none_or(|drawing| drawing.lines.regions.is_empty() && drawing.lines.polylines.is_empty() && drawing.lines.texts.is_empty());
    PlacedViewport {
        viewport: id.to_string(),
        view: viewport.view.clone(),
        label: viewport.label.clone().unwrap_or_else(|| view.name.clone()),
        kind: view.kind,
        scale: viewport.scale,
        window,
        map: ViewMap { min_x: x0, max_y: y1, mm },
        empty,
        cropped,
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
