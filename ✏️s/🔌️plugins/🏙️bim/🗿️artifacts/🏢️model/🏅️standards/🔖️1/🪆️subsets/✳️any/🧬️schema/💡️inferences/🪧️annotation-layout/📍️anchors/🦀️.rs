//! 📍️ Resolves what an annotation is anchored to: an [`AnnotationAnchor`] becomes a point or a finite segment in building coordinates (metres), read from the current geometry of the
//! element it names (the join-trimmed faces of a wall layout, the authored axis of a wall, the centre of an opening on its host axis, a grid line, a column position). An anchor that
//! names nothing, or an element without that geometry now, resolves to a [`Reason`], never to a stale number. Every element also has a reference point, the origin tags and leaders
//! measure their offset from.

use super::Inputs;
use crate::standards::v1::subsets::any::schema::inferences::element_solids::plan_kit::{mark, seg};
use crate::{AnchorEnd, AnnotationAnchor, Axis, ModelSnapshot, Point2, Space, SpaceBoundary, WallSide};

/// 📏️ Lengths at or below this many metres count as zero.
pub const EPS: f64 = 1e-9;

/// 🚫️ Why an anchor has no geometry now.
#[derive(semio_framework_value::RetireOwned, Clone, Copy, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue)]
pub enum Reason {
    Missing,
    Curved,
    Degenerate,
    Parallel,
}

/// 📍️ The geometry of a resolved anchor: a point, or a finite straight segment (a face, an axis or a grid line).
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Reference {
    Point(Point2),
    Segment(Point2, Point2),
}

impl Reference {
    /// 👆️ The point a leader or a dimension picks the reference by: the point itself, or the middle of the segment.
    pub fn pick(&self) -> Point2 {
        match self {
            Self::Point(point) => *point,
            Self::Segment(start, end) => Point2 { x: (start.x + end.x) / 2.0, y: (start.y + end.y) / 2.0 },
        }
    }
}

fn straight(start: Point2, end: Point2, bulge: f64) -> Result<Reference, Reason> {
    if bulge != 0.0 {
        return Err(Reason::Curved);
    }
    if (end.x - start.x).hypot(end.y - start.y) <= EPS {
        return Err(Reason::Degenerate);
    }
    Ok(Reference::Segment(start, end))
}

fn ends(axis: &Axis) -> (Point2, Point2) {
    match axis {
        Axis::Line { start, end } | Axis::Arc { start, end, .. } => (*start, *end),
    }
}

fn bulge(axis: &Axis) -> f64 {
    match axis {
        Axis::Line { .. } => 0.0,
        Axis::Arc { bulge, .. } => *bulge,
    }
}

/// 🏗️ The axis of the wall or curtain wall that hosts an opening.
fn host_axis<'a>(snapshot: &'a ModelSnapshot, host: &str) -> Option<&'a Axis> {
    snapshot.walls.get(host).map(|wall| &wall.axis).or_else(|| snapshot.curtain_walls.get(host).map(|curtain| &curtain.axis))
}

/// 📍️ The point on the host axis at the arc length of an opening: its centre in plan.
fn opening_centre(snapshot: &ModelSnapshot, opening: &str) -> Result<Point2, Reason> {
    let row = snapshot.openings.get(opening).ok_or(Reason::Missing)?;
    let axis = host_axis(snapshot, &row.host).ok_or(Reason::Missing)?;
    let line = seg(axis);
    if line.length() <= EPS {
        return Err(Reason::Degenerate);
    }
    Ok(mark(line.point_at_length(row.offset)))
}

/// ⚓️ The geometry of an anchor in the current model: `layouts` carries the join-trimmed faces of the walls (the `WallLayout` values the annotation node is computed from).
pub fn resolve(snapshot: &ModelSnapshot, inputs: &Inputs<'_>, anchor: &AnnotationAnchor) -> Result<Reference, Reason> {
    match anchor {
        AnnotationAnchor::Point { point } => Ok(Reference::Point(*point)),
        AnnotationAnchor::WallFace { wall, side } => {
            if !snapshot.walls.contains_key(wall) {
                return Err(Reason::Missing);
            }
            let layout = inputs.layouts.get(wall.as_str()).filter(|layout| !layout.footprint.is_empty()).ok_or(Reason::Degenerate)?;
            let face = match side {
                WallSide::Left => layout.left_face,
                WallSide::Right => layout.right_face,
            };
            straight(face.start, face.end, face.bulge)
        }
        AnnotationAnchor::WallAxis { wall } => {
            let axis = &snapshot.walls.get(wall).ok_or(Reason::Missing)?.axis;
            let (start, end) = ends(axis);
            straight(start, end, bulge(axis))
        }
        AnnotationAnchor::WallEnd { wall, end } => {
            let (start, last) = ends(&snapshot.walls.get(wall).ok_or(Reason::Missing)?.axis);
            Ok(Reference::Point(match end {
                AnchorEnd::Start => start,
                AnchorEnd::End => last,
            }))
        }
        AnnotationAnchor::OpeningCentre { opening } => opening_centre(snapshot, opening).map(Reference::Point),
        AnnotationAnchor::Grid { grid } => {
            let row = snapshot.grids.get(grid).ok_or(Reason::Missing)?;
            straight(row.start, row.end, 0.0)
        }
        AnnotationAnchor::ColumnCentre { column } => snapshot.columns.get(column).map(|row| Reference::Point(row.position)).ok_or(Reason::Missing),
    }
}

fn mean(points: impl IntoIterator<Item = Point2>) -> Option<Point2> {
    let (count, sum) = points.into_iter().fold((0.0, (0.0, 0.0)), |(count, sum), point| (count + 1.0, (sum.0 + point.x, sum.1 + point.y)));
    (count > 0.0).then(|| Point2 { x: sum.0 / count, y: sum.1 / count })
}

fn space_point(space: &Space) -> Option<Point2> {
    match &space.boundary {
        SpaceBoundary::Bounded { seed } => Some(*seed),
        SpaceBoundary::Explicit { outline } => mean(outline.iter().map(|vertex| vertex.point)),
    }
}

fn middle(axis: &Axis) -> Point2 {
    let line = seg(axis);
    mark(line.point_at_length(line.length() / 2.0))
}

/// 📍️ The reference point of an element, the origin of the offset of a tag or leader: the middle of a wall, curtain wall, beam, grid line or railing path, the position of a column or the start of a stair,
/// the centre of an opening on its host axis, the seed or outline centre of a space and the vertex mean of a slab or roof. `None` for an id that is no taggable element.
pub fn reference_point(snapshot: &ModelSnapshot, element: &str) -> Option<Point2> {
    if let Some(wall) = snapshot.walls.get(element) {
        return Some(middle(&wall.axis));
    }
    if let Some(curtain) = snapshot.curtain_walls.get(element) {
        return Some(middle(&curtain.axis));
    }
    if let Some(column) = snapshot.columns.get(element) {
        return Some(column.position);
    }
    if let Some(beam) = snapshot.beams.get(element) {
        return Some(middle(&beam.axis));
    }
    if let Some(slab) = snapshot.slabs.get(element) {
        return mean(slab.boundary.iter().map(|vertex| vertex.point));
    }
    if let Some(roof) = snapshot.roofs.get(element) {
        return mean(roof.footprint.iter().map(|vertex| vertex.point));
    }
    if snapshot.openings.contains_key(element) {
        return opening_centre(snapshot, element).ok();
    }
    if let Some(stair) = snapshot.stairs.get(element) {
        return Some(stair.start);
    }
    if let Some(railing) = snapshot.railings.get(element) {
        return mean(railing.path.iter().copied());
    }
    if let Some(space) = snapshot.spaces.get(element) {
        return space_point(space);
    }
    snapshot.grids.get(element).map(|grid| Point2 { x: (grid.start.x + grid.end.x) / 2.0, y: (grid.start.y + grid.end.y) / 2.0 })
}
