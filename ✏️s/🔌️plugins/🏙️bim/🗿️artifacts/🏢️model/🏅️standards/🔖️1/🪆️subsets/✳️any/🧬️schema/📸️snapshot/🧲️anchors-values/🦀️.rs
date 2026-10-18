//! 🧮️ The methods of the value types of the `anchors` kit; they live with the value types so the entity types and the model root stay above them.

use crate::standards::v1::subsets::any::schema::snapshot::values::{AnnotationAnchor, Point2};

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
