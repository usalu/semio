//! 📏️ The annotations of a storey in its plan: the dimension lines with their extension lines, end marks and texts, the tags and text notes, and the leaders with their end marks and texts. Everything is
//! drawn from the [`StoreyAnnotations`] value the `Annotation(storey)` node derived, never from the snapshot, so a plan shows the printed number of the current geometry. A text is placed by the middle
//! of its baseline ([`TextAnchor`]) and carries its height, which the drawing of the plan scales with the paper (model metres of the style), so a plan at 1:50 and at 1:100 each keep their sheet text size.

use super::{PlanKind, PlanStyle, PlanText, PlanVertex, Sheet};
use crate::standards::v1::subsets::any::schema::inferences::annotation_layout::{AnchorLayout, DimensionLayout, LeaderLayout, StoreyAnnotations, StyleMarks, TextAnchor};
use crate::{Point2, Terminator};
use semio_framework_geometry::Point;

fn point(p: Point2) -> Point {
    Point::new(p.x, p.y)
}

fn put_text(sheet: &mut Sheet, element: &str, kind: PlanKind, at: &TextAnchor, label: &str, height: f64) {
    if label.is_empty() {
        return;
    }
    let id = sheet.id(element, kind);
    sheet.texts.push(PlanText { id, element: element.to_string(), kind, style: PlanStyle::Annotation, x: at.at.x, y: at.at.y, rotation: at.rotation, label: label.to_string(), detail: String::new(), measure: None, height });
}

fn along(from: Point2, to: Point2) -> Option<(f64, f64)> {
    let (dx, dy) = ((to.x - from.x), (to.y - from.y));
    let length = dx.hypot(dy);
    (length > 1e-9).then(|| (dx / length, dy / length))
}

fn offset(p: Point2, direction: (f64, f64), distance: f64) -> Point {
    Point::new(p.x + direction.0 * distance, p.y + direction.1 * distance)
}

fn dot_mark(sheet: &mut Sheet, element: &str, kind: PlanKind, at: Point2, size: f64) {
    let radius = size / 2.0;
    sheet.polyline(element, kind, PlanStyle::Annotation, true, vec![PlanVertex { x: at.x + radius, y: at.y, bulge: 1.0 }, PlanVertex { x: at.x - radius, y: at.y, bulge: 1.0 }]);
}

fn arrow_mark(sheet: &mut Sheet, element: &str, kind: PlanKind, tip: Point2, inward: (f64, f64), size: f64) {
    let side = (-inward.1, inward.0);
    let base = offset(tip, inward, size);
    let wing = size * 0.17;
    sheet.path(element, kind, PlanStyle::Annotation, &[Point::new(base.x + side.0 * wing, base.y + side.1 * wing), point(tip), Point::new(base.x - side.0 * wing, base.y - side.1 * wing)]);
}

fn tick_mark(sheet: &mut Sheet, element: &str, kind: PlanKind, at: Point2, direction: (f64, f64), size: f64) {
    let slant = ((direction.0 - direction.1) * std::f64::consts::FRAC_1_SQRT_2, (direction.0 + direction.1) * std::f64::consts::FRAC_1_SQRT_2);
    sheet.path(element, kind, PlanStyle::Annotation, &[offset(at, slant, -size / 2.0), offset(at, slant, size / 2.0)]);
}

fn draw_dimension(sheet: &mut Sheet, id: &str, layout: &DimensionLayout) {
    if !layout.complete {
        return;
    }
    let StyleMarks { text_height, terminator, mark_size, .. } = layout.style;
    let by_position = |a: &&AnchorLayout, b: &&AnchorLayout| a.position.total_cmp(&b.position);
    let (low, high) = (layout.anchors.iter().min_by(by_position), layout.anchors.iter().max_by(by_position));
    if let (Some(low), Some(high)) = (low, high) {
        sheet.path(id, PlanKind::DimensionLine, PlanStyle::Annotation, &[point(low.mark), point(high.mark)]);
    }
    for anchor in &layout.anchors {
        if let Some(extension) = anchor.extension {
            sheet.path(id, PlanKind::DimensionExtension, PlanStyle::Annotation, &[point(extension.start), point(extension.end)]);
        }
    }
    let direction = (layout.angle.cos(), layout.angle.sin());
    match terminator {
        Terminator::Tick => layout.anchors.iter().for_each(|anchor| tick_mark(sheet, id, PlanKind::DimensionMark, anchor.mark, direction, mark_size)),
        Terminator::Dot => layout.anchors.iter().for_each(|anchor| dot_mark(sheet, id, PlanKind::DimensionMark, anchor.mark, mark_size)),
        Terminator::Arrow => {
            for segment in &layout.segments {
                if let Some(inward) = along(segment.from, segment.to) {
                    arrow_mark(sheet, id, PlanKind::DimensionMark, segment.from, inward, mark_size);
                    arrow_mark(sheet, id, PlanKind::DimensionMark, segment.to, (-inward.0, -inward.1), mark_size);
                }
            }
        }
    }
    for segment in &layout.segments {
        put_text(sheet, id, PlanKind::DimensionText, &segment.text_at, &segment.text, text_height);
    }
}

fn draw_leader(sheet: &mut Sheet, id: &str, layout: &LeaderLayout) {
    if layout.reason.is_some() {
        return;
    }
    let tip = layout.tip;
    sheet.path(id, PlanKind::LeaderLine, PlanStyle::Annotation, &[point(tip), point(layout.at.at)]);
    if let Some(inward) = along(tip, layout.at.at) {
        match layout.style.terminator {
            Terminator::Tick => tick_mark(sheet, id, PlanKind::LeaderMark, tip, inward, layout.style.mark_size),
            Terminator::Dot => dot_mark(sheet, id, PlanKind::LeaderMark, tip, layout.style.mark_size),
            Terminator::Arrow => arrow_mark(sheet, id, PlanKind::LeaderMark, tip, inward, layout.style.mark_size),
        }
    }
    put_text(sheet, id, PlanKind::LeaderText, &layout.at, &layout.text, layout.style.text_height);
}

/// 📏️ Draws the annotations of a storey, if it has any, on top of everything else.
pub fn draw(sheet: &mut Sheet, annotations: Option<&StoreyAnnotations>) {
    let Some(annotations) = annotations else { return };
    for (id, layout) in &annotations.dimensions {
        draw_dimension(sheet, id, layout);
    }
    for (id, layout) in annotations.tags.iter().filter(|(_, layout)| layout.complete) {
        put_text(sheet, id, PlanKind::TagText, &layout.at, &layout.text, layout.style.text_height);
    }
    for (id, layout) in &annotations.notes {
        put_text(sheet, id, PlanKind::NoteText, &layout.at, &layout.text, layout.style.text_height);
    }
    for (id, layout) in &annotations.leaders {
        draw_leader(sheet, id, layout);
    }
}
