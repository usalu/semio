//! 🧮️ The methods of the value types of the `views` kit; they live with the value types so the entity types and the model root stay above them.

use crate::standards::v1::subsets::any::schema::snapshot::values::ViewKind;

impl ViewKind {
    /// 🗺️ Whether the kind cuts a storey horizontally.
    pub fn is_plan(self) -> bool {
        matches!(self, Self::Plan | Self::CeilingPlan)
    }

    /// 📐️ Whether the kind looks through a vertical plane.
    pub fn is_vertical(self) -> bool {
        matches!(self, Self::Section | Self::Elevation)
    }

    /// 🎥️ Whether the kind is a camera.
    pub fn is_camera(self) -> bool {
        matches!(self, Self::Orthographic | Self::Perspective)
    }
}
