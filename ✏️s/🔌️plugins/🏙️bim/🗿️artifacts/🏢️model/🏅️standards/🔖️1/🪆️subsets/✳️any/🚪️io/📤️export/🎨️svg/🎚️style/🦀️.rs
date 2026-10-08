//! 🎚️ The pen of a plan drawing: one stroke class per line style (heavy cut, medium projection, dashed hidden, fine annotation), poché fills and the text sizes, all in paper millimetres.
//! Classes carry the meaning, the style sheet carries the look, so a CAD or a browser restyles the plan without touching the geometry.
//! 📎 https://www.w3.org/TR/SVG11/styling.html

use crate::standards::v1::subsets::any::schema::inferences::plan_linework::{PlanKind, PlanStyle};

/// 🖊️ The class of a line style.
pub fn style_class(style: PlanStyle) -> &'static str {
    match style {
        PlanStyle::Cut => "cut",
        PlanStyle::Projection => "projection",
        PlanStyle::Hidden => "hidden",
        PlanStyle::Annotation => "annotation",
    }
}

/// 🖊️ Every style class, in paint order: hidden under projection under cut under annotation.
pub const STYLE_CLASSES: [&str; 4] = ["hidden", "projection", "cut", "annotation"];

/// 🎨️ The paint rank of a line style (lower is drawn first).
pub fn paint_rank(style: PlanStyle) -> u8 {
    match style {
        PlanStyle::Hidden => 0,
        PlanStyle::Projection => 1,
        PlanStyle::Cut => 2,
        PlanStyle::Annotation => 3,
    }
}

/// 🏷️ The class of what a primitive depicts.
pub fn kind_class(kind: PlanKind) -> &'static str {
    match kind {
        PlanKind::WallCut => "wall-cut",
        PlanKind::WallLayer => "wall-layer",
        PlanKind::WallOutline => "wall-outline",
        PlanKind::CurtainAxis => "curtain-axis",
        PlanKind::CurtainMullion => "curtain-mullion",
        PlanKind::WindowFrame => "window-frame",
        PlanKind::WindowGlazing => "window-glazing",
        PlanKind::WindowSill => "window-sill",
        PlanKind::DoorLeaf => "door-leaf",
        PlanKind::DoorSwing => "door-swing",
        PlanKind::ColumnCut => "column-cut",
        PlanKind::ColumnOutline => "column-outline",
        PlanKind::BeamOutline => "beam-outline",
        PlanKind::SlabEdge => "slab-edge",
        PlanKind::SlabHole => "slab-hole",
        PlanKind::RoofOutline => "roof-outline",
        PlanKind::StairOutline => "stair-outline",
        PlanKind::StairRiser => "stair-riser",
        PlanKind::StairCutLine => "stair-cut-line",
        PlanKind::StairArrow => "stair-arrow",
        PlanKind::StairLanding => "stair-landing",
        PlanKind::RailingPath => "railing-path",
        PlanKind::SpaceOutline => "space-outline",
        PlanKind::SpaceTag => "space-tag",
        PlanKind::GridLine => "grid-line",
        PlanKind::GridBubble => "grid-bubble",
        PlanKind::GridLabel => "grid-label",
    }
}

/// 📏️ Stroke widths in paper millimetres (ISO 128 line groups 0.5, 0.25, 0.18 and 0.13).
pub const STROKE_WIDTHS: [(&str, f64); 4] = [("cut", 0.5), ("projection", 0.25), ("hidden", 0.18), ("annotation", 0.13)];

/// 🔤️ Text sizes in paper millimetres: sheet titles, space tags and grid labels.
pub const TITLE_SIZE: f64 = 5.0;
pub const TAG_SIZE: f64 = 3.2;
pub const LABEL_SIZE: f64 = 3.5;

/// 📜️ The embedded style sheet.
pub fn sheet() -> String {
    let strokes: String = STROKE_WIDTHS.iter().map(|(class, width)| format!("      .{class}{{stroke-width:{width}}}\n")).collect();
    format!(
        "\n      .plan{{font-family:sans-serif}}\n      path{{stroke:#000;stroke-linecap:round;stroke-linejoin:round;fill:none}}\n{strokes}      .hidden{{stroke-dasharray:1.5 0.75}}\n      path.region{{fill-rule:evenodd}}\n      path.region.cut{{fill:#1a1a1a}}\n      path.region.projection{{fill:#e6e6e6}}\n      text{{fill:#000;stroke:none;font-size:{TAG_SIZE}px}}\n      text.title{{font-size:{TITLE_SIZE}px;font-weight:bold}}\n      text.space-tag{{text-anchor:middle}}\n      text.grid-label{{font-size:{LABEL_SIZE}px;text-anchor:middle;dominant-baseline:central}}\n    "
    )
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
