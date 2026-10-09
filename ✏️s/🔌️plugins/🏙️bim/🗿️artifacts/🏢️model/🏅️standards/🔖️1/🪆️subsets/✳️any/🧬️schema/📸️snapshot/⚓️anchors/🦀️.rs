//! ⚓️ The vocabulary of what an annotation is attached to: which element an anchor names, the element a tag reads, and the free points an annotation carries.
//! Pure and base-free: whether the named elements exist is decided by the leaves and by the diagnostics, never here.

use super::{AnnotationAnchor, Dimension, Leader, Point2, TextNote};

impl AnnotationAnchor {
    /// 🔗️ The id of the element the anchor follows, `None` for a free point.
    pub fn element(&self) -> Option<&str> {
        match self {
            Self::Point { .. } => None,
            Self::WallFace { wall, .. } | Self::WallAxis { wall } | Self::WallEnd { wall, .. } => Some(wall),
            Self::OpeningCentre { opening } => Some(opening),
            Self::Grid { grid } => Some(grid),
            Self::ColumnCentre { column } => Some(column),
        }
    }

    /// 📍️ The free point of the anchor, `None` when it follows an element.
    pub fn free_point(&self) -> Option<Point2> {
        match self {
            Self::Point { point } => Some(*point),
            _ => None,
        }
    }
}

impl Dimension {
    /// 🔗️ The ids of the elements the anchors of the dimension follow, in anchor order.
    pub fn elements(&self) -> impl Iterator<Item = &str> {
        self.anchors.iter().filter_map(AnnotationAnchor::element)
    }
}

impl Leader {
    /// 🔗️ The id of the element the leader points at, `None` for a free point.
    pub fn element(&self) -> Option<&str> {
        self.anchor.element()
    }
}

/// 🔢️ Whether every number of a point is finite.
pub fn is_finite_point(point: Point2) -> bool {
    point.x.is_finite() && point.y.is_finite()
}

/// 🔢️ Every number an annotation carries, to prove it finite.
pub fn dimension_numbers(dimension: &Dimension) -> Vec<f64> {
    dimension.anchors.iter().filter_map(AnnotationAnchor::free_point).flat_map(|point| [point.x, point.y]).chain([dimension.angle, dimension.offset]).chain(dimension.lock).collect()
}

/// 🔢️ Every number of a text note.
pub fn note_numbers(note: &TextNote) -> Vec<f64> {
    vec![note.position.x, note.position.y, note.rotation]
}
