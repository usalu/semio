//! 📏️ The geometry and the text of a dimension, from the resolved geometry of its anchors. The dimension line is the line `dot(p, n) = c` with `n` the left normal of the measuring direction `d`
//! and `c` the position of the pick point of the first anchor along `n` plus the authored offset. A point anchor is measured where its perpendicular meets that line; a line anchor (a face, an
//! axis, a grid line) where the line itself crosses it, so two parallel faces measured across give the clear distance between them. The measured distance of each consecutive pair is the
//! difference of the positions along `d`, printed in the unit and precision of the style. Nothing here is stored: moving an anchored wall moves the line and changes the printed number.
//! 📎 https://www.iso.org/standard/53699.html (ISO 129-1 presentation of dimensions)

use super::anchors::{resolve, Reason, Reference, EPS};
use super::{marks_of, print, text_width, AnchorLayout, DimensionLayout, DimensionSegment, Inputs, Line, TextAnchor};
use crate::{Dimension, ModelSnapshot, Point2};

fn dot(a: (f64, f64), b: (f64, f64)) -> f64 {
    a.0 * b.0 + a.1 * b.1
}

fn point(x: f64, y: f64) -> Point2 {
    Point2 { x, y }
}

fn vector(point: Point2) -> (f64, f64) {
    (point.x, point.y)
}

/// 🔤️ The angle text is set at: the measuring direction turned by half a turn when it would read upside down, so it always reads left to right or bottom to top.
pub fn readable(angle: f64) -> f64 {
    let (cos, sin) = (angle.cos(), angle.sin());
    if cos < -EPS || (cos.abs() <= EPS && sin < 0.0) {
        angle + std::f64::consts::PI
    } else {
        angle
    }
}

fn crossing(reference: &Reference, direction: (f64, f64), normal: (f64, f64), line: f64) -> Result<(Point2, Point2), Reason> {
    match reference {
        Reference::Point(at) => {
            let along = dot(vector(*at), direction);
            Ok((*at, point(along * direction.0 + line * normal.0, along * direction.1 + line * normal.1)))
        }
        Reference::Segment(start, end) => {
            let span = (end.x - start.x, end.y - start.y);
            let across = dot(span, normal);
            if across.abs() <= EPS * span.0.hypot(span.1).max(1.0) {
                return Err(Reason::Parallel);
            }
            let t = (line - dot(vector(*start), normal)) / across;
            let foot = t.clamp(0.0, 1.0);
            Ok((point(start.x + foot * span.0, start.y + foot * span.1), point(start.x + t * span.0, start.y + t * span.1)))
        }
    }
}

fn extension(foot: Point2, mark: Point2, gap: f64, overshoot: f64) -> Option<Line> {
    let (dx, dy) = (mark.x - foot.x, mark.y - foot.y);
    let length = dx.hypot(dy);
    (length > gap.max(EPS)).then(|| Line { start: point(foot.x + dx / length * gap, foot.y + dy / length * gap), end: point(mark.x + dx / length * overshoot, mark.y + dy / length * overshoot) })
}

/// 📏️ The layout of one dimension. Anchors that cannot be resolved leave the layout incomplete: it then carries no segment and no total, and the reasons say why.
pub fn layout_of(snapshot: &ModelSnapshot, inputs: &Inputs<'_>, dimension: &Dimension) -> DimensionLayout {
    let style = marks_of(snapshot, &dimension.style);
    let resolved: Vec<Result<Reference, Reason>> = dimension.anchors.iter().map(|anchor| resolve(snapshot, inputs, anchor)).collect();
    let mut layout = DimensionLayout { style, angle: dimension.angle, anchors: Vec::new(), segments: Vec::new(), total: 0.0, total_text: String::new(), lock_difference: None, complete: false };
    let direction = (dimension.angle.cos(), dimension.angle.sin());
    let normal = (-direction.1, direction.0);
    let base = resolved.iter().find_map(|row| row.as_ref().ok()).map(|reference| reference.pick());
    let line = base.map_or(0.0, |base| dot(vector(base), normal) + dimension.offset);
    for row in &resolved {
        let placed = row.as_ref().map_err(|reason| *reason).and_then(|reference| crossing(reference, direction, normal, line));
        layout.anchors.push(match placed {
            Ok((foot, mark)) => AnchorLayout { reason: None, foot, mark, position: dot(vector(mark), direction), extension: extension(foot, mark, layout.style.gap, layout.style.overshoot) },
            Err(reason) => AnchorLayout { reason: Some(reason), foot: point(0.0, 0.0), mark: point(0.0, 0.0), position: 0.0, extension: None },
        });
    }
    layout.complete = dimension.anchors.len() >= 2 && layout.anchors.iter().all(|anchor| anchor.reason.is_none());
    if !layout.complete {
        return layout;
    }
    let reading = readable(dimension.angle);
    let lift = (-reading.sin(), reading.cos());
    for pair in layout.anchors.windows(2) {
        let length = (pair[1].position - pair[0].position).abs();
        let middle = point((pair[0].mark.x + pair[1].mark.x) / 2.0, (pair[0].mark.y + pair[1].mark.y) / 2.0);
        let raised = point(middle.x + lift.0 * layout.style.text_height * 0.6, middle.y + lift.1 * layout.style.text_height * 0.6);
        let text = print(length, layout.style.unit, layout.style.precision);
        layout.segments.push(DimensionSegment { from: pair[0].mark, to: pair[1].mark, length, text: text.clone(), text_at: TextAnchor { at: raised, rotation: reading, width: text_width(&text, layout.style.text_height) } });
    }
    let (first, last) = (&layout.anchors[0], &layout.anchors[layout.anchors.len() - 1]);
    layout.total = (last.position - first.position).abs();
    layout.total_text = print(layout.total, layout.style.unit, layout.style.precision);
    layout.lock_difference = dimension.lock.map(|lock| layout.total - lock);
    layout
}
