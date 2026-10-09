//! 🎚️ The pen of a view drawing (plan, ceiling plan, section, elevation): one stroke class per line style (heavy cut, medium projection, dashed hidden, fine annotation), poché fills and the text sizes, all in paper millimetres.
//! Classes carry the meaning, the style sheet carries the look, so a CAD or a browser restyles the drawing without touching the geometry.
//! 📎 https://www.w3.org/TR/SVG11/styling.html

use crate::standards::v1::subsets::any::schema::inferences::plan_linework::{PlanKind, PlanStyle};
use crate::ViewKind;

/// 🖊️ The class of a line style.
pub fn style_class(style: PlanStyle) -> &'static str {
    match style {
        PlanStyle::Cut => "cut",
        PlanStyle::Projection => "projection",
        PlanStyle::Hidden => "hidden",
        PlanStyle::Annotation => "annotation",
    }
}

/// 🖼️ The class of a view kind.
pub fn view_class(kind: ViewKind) -> &'static str {
    match kind {
        ViewKind::Plan => "plan",
        ViewKind::CeilingPlan => "ceiling-plan",
        ViewKind::Section => "section",
        ViewKind::Elevation => "elevation",
        ViewKind::Orthographic => "orthographic",
        ViewKind::Perspective => "perspective",
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
        PlanKind::RampOutline => "ramp-outline",
        PlanKind::RampLanding => "ramp-landing",
        PlanKind::RampArrow => "ramp-arrow",
        PlanKind::RampTag => "ramp-tag",
        PlanKind::RailingPath => "railing-path",
        PlanKind::SpaceOutline => "space-outline",
        PlanKind::SpaceTag => "space-tag",
        PlanKind::GridLine => "grid-line",
        PlanKind::GridBubble => "grid-bubble",
        PlanKind::GridLabel => "grid-label",
        PlanKind::DimensionLine => "dimension-line",
        PlanKind::DimensionExtension => "dimension-extension",
        PlanKind::DimensionMark => "dimension-mark",
        PlanKind::DimensionText => "dimension-text",
        PlanKind::TagText => "tag-text",
        PlanKind::NoteText => "note-text",
        PlanKind::LeaderLine => "leader-line",
        PlanKind::LeaderMark => "leader-mark",
        PlanKind::LeaderText => "leader-text",
        PlanKind::SectionCut => "section-cut",
        PlanKind::Silhouette => "silhouette",
        PlanKind::Edge => "edge",
        PlanKind::Datum => "datum",
        PlanKind::DatumLabel => "datum-label",
        PlanKind::CeilingEdge => "ceiling-edge",
        PlanKind::CeilingHole => "ceiling-hole",
    }
}

/// 🪧️ Whether a primitive belongs to the annotation layer: a dimension, tag, text note or leader.
pub fn annotated(kind: PlanKind) -> bool {
    kind.is_notation()
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
