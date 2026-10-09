//! 🔎️ What a view lets through: the category of a drawn primitive or solid, the phase of an element, the primitives each detail level leaves out, and the crop of a plan. These are the
//! answers to "is this drawn in this view?"; the view's authored fields only parameterise them, nothing here is stored.

use crate::standards::v1::subsets::any::schema::inferences::element_solids::SolidFamily;
use crate::standards::v1::subsets::any::schema::inferences::plan_linework::{PlanKind, PlanLinework, PlanPolyline, PlanRegion, PlanStyle, PlanText, PlanVertex};
use super::clip::Rect;
use crate::{DetailLevel, ModelSnapshot, Phase, View, ViewCategory};

/// 🧮️ The category a plan primitive of `kind` belongs to, none for the primitives of a vertical view that take the category of their element.
pub fn category_of_kind(kind: PlanKind) -> Option<ViewCategory> {
    use PlanKind::*;
    match kind {
        WallCut | WallLayer | WallOutline => Some(ViewCategory::Walls),
        CurtainAxis | CurtainMullion => Some(ViewCategory::CurtainWalls),
        WindowFrame | WindowGlazing | WindowSill | DoorLeaf | DoorSwing => Some(ViewCategory::Openings),
        ColumnCut | ColumnOutline => Some(ViewCategory::Columns),
        BeamOutline => Some(ViewCategory::Beams),
        SlabEdge | SlabHole => Some(ViewCategory::Slabs),
        RoofOutline => Some(ViewCategory::Roofs),
        StairOutline | StairRiser | StairCutLine | StairArrow | StairLanding => Some(ViewCategory::Stairs),
        RailingPath => Some(ViewCategory::Railings),
        SpaceOutline | SpaceTag => Some(ViewCategory::Spaces),
        GridLine | GridBubble | GridLabel => Some(ViewCategory::Grids),
        SectionCut | Silhouette | Edge | Datum | DatumLabel => None,
        #[allow(unreachable_patterns)]
        _ => None,
    }
}

/// 🧮️ The category of the solids of a family, none for a family no category switches off yet.
pub fn category_of_family(family: SolidFamily) -> Option<ViewCategory> {
    match family {
        SolidFamily::Wall => Some(ViewCategory::Walls),
        SolidFamily::CurtainWall => Some(ViewCategory::CurtainWalls),
        SolidFamily::Window | SolidFamily::Door => Some(ViewCategory::Openings),
        SolidFamily::Column => Some(ViewCategory::Columns),
        SolidFamily::Beam => Some(ViewCategory::Beams),
        SolidFamily::Slab => Some(ViewCategory::Slabs),
        SolidFamily::Roof => Some(ViewCategory::Roofs),
        SolidFamily::Stair => Some(ViewCategory::Stairs),
        SolidFamily::Railing => Some(ViewCategory::Railings),
        #[allow(unreachable_patterns)]
        _ => None,
    }
}

/// 🕰️ The phase an element has, none for an id that is no element: the one rule of the `phase-visibility` inference (an element's own phase, its host's for an opening, new construction for a kind without a phase).
pub fn phase_of(snapshot: &ModelSnapshot, element: &str) -> Option<Phase> {
    crate::standards::v1::subsets::any::schema::inferences::phase_visibility::phase_of(snapshot, element)
}

/// 🕰️ The phases of every element on the storeys of a building, the one authored input the phase filter reads besides the view.
pub fn phases_of(snapshot: &ModelSnapshot, building: &str) -> std::collections::BTreeMap<String, Phase> {
    crate::standards::v1::subsets::any::schema::inferences::phase_visibility::phases_in(snapshot, building)
}

/// 🔎️ Whether a plan primitive of an element survives the view's category, phase and detail filters.
pub fn lets_through(snapshot: &ModelSnapshot, view: &View, element: &str, kind: PlanKind, style: PlanStyle) -> bool {
    if category_of_kind(kind).is_some_and(|category| view.hides(category)) {
        return false;
    }
    if view.phase.is_some() && phase_of(snapshot, element).is_some_and(|phase| !view.shows_phase(phase)) {
        return false;
    }
    match view.detail {
        DetailLevel::Coarse => !(matches!(kind, PlanKind::WallLayer | PlanKind::WindowGlazing | PlanKind::WindowSill | PlanKind::DoorSwing | PlanKind::StairRiser | PlanKind::StairArrow | PlanKind::GridBubble) || style == PlanStyle::Hidden),
        DetailLevel::Medium | DetailLevel::Fine => true,
    }
}

fn spans(vertices: &[PlanVertex]) -> Option<(f64, f64, f64, f64)> {
    vertices.iter().fold(None, |bounds, vertex| match bounds {
        None => Some((vertex.x, vertex.y, vertex.x, vertex.y)),
        Some((x0, y0, x1, y1)) => Some((x0.min(vertex.x), y0.min(vertex.y), x1.max(vertex.x), y1.max(vertex.y))),
    })
}

/// ✂️ The plan primitives that touch the crop rectangle; a primitive is kept whole or dropped, never cut at the rectangle.
pub fn crop_plan(lines: PlanLinework, rect: &Rect) -> PlanLinework {
    let touches = |vertices: &[PlanVertex]| spans(vertices).is_some_and(|(x0, y0, x1, y1)| rect.meets(x0, y0, x1, y1));
    PlanLinework {
        regions: lines.regions.into_iter().filter(|region: &PlanRegion| touches(&region.outer)).collect(),
        polylines: lines.polylines.into_iter().filter(|line: &PlanPolyline| touches(&line.vertices)).collect(),
        texts: lines.texts.into_iter().filter(|text: &PlanText| rect.contains([text.x, text.y])).collect(),
        bounds: crate::standards::v1::subsets::any::schema::inferences::plan_linework::PlanBounds { min_x: rect.x0, min_y: rect.y0, max_x: rect.x1, max_y: rect.y1 },
        ..lines
    }
}
