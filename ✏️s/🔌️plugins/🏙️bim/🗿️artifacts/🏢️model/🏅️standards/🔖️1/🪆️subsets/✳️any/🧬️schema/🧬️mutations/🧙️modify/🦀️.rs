//! 🧙️ The modify kernel shared by the copy, mirror, array, align, offset, trim and split leaves and by the editor tools that propose them:
//! the geometric maps and their image of every placed element (`map`), the copies and in-place images built from them (`copies`) and the
//! pure curve geometry of trimming, offsetting and cutting (`cut`). Everything here is a pure function of authored parameters: the
//! ids of copies are minted from a prefix, never from a clock or a counter, so every device derives the same diff from the same mutation.

use crate::Point2;

#[path = "👯️copies/🦀️.rs"]
pub mod copies;
#[path = "✂️cut/🦀️.rs"]
pub mod cut;
#[path = "🗺️map/🦀️.rs"]
pub mod map;

pub use copies::{duplicate, in_place, mint, sources, Built, Placed, Sources, MAX_CREATED};
pub use map::{Image, Map};

//#region 🔖️Vocabulary
/// 🔚️ One end of a wall axis.
#[derive(semio_framework_value::RetireOwned, Clone, Copy, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
pub enum WallEnd {
    Start,
    End,
}

/// 🧭️ The coordinate of the plan an alignment sets: the `x` or the `y` of the extent edge it names.
#[derive(semio_framework_value::RetireOwned, Clone, Copy, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
pub enum AlignAxis {
    X,
    Y,
}

/// 📐️ The edge of the extent of an element that an alignment sets: the lower edge, the middle or the upper edge along the coordinate.
#[derive(semio_framework_value::RetireOwned, Clone, Copy, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
pub enum AlignEdge {
    Min,
    Center,
    Max,
}

/// 🔁️ How an array repeats its sources: `count` further copies, each one the previous translated by `spacing` (a linear array), or each
/// one turned by a further `step` radians about `center` (a radial array).
#[derive(semio_framework_value::RetireOwned, Clone, Copy, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
pub enum ArrayPattern {
    Linear { count: u32, spacing: Point2 },
    Radial { count: u32, center: Point2, step: f64 },
}

impl ArrayPattern {
    /// 🔢️ How many copies the pattern makes.
    pub fn count(&self) -> u32 {
        match self {
            ArrayPattern::Linear { count, .. } | ArrayPattern::Radial { count, .. } => *count,
        }
    }

    /// 🗺️ The map of copy `copy` (1 is the first copy): the k-th multiple of the spacing, or the k-th multiple of the step, always computed
    /// from the source so that no rounding accumulates from copy to copy.
    pub fn map_of(&self, copy: u32) -> Map {
        let k = f64::from(copy);
        match self {
            ArrayPattern::Linear { spacing, .. } => Map::Translate(Point2 { x: spacing.x * k, y: spacing.y * k }),
            ArrayPattern::Radial { center, step, .. } => Map::Rotate { pivot: *center, angle: step * k },
        }
    }

    /// 🔎️ Why the pattern cannot repeat anything: a count of zero or above [`MAX_COPIES`], a non-finite spacing, centre or step, a spacing of
    /// no length or a step of no turn.
    pub fn flaw(&self) -> Option<(&'static str, &'static str)> {
        if self.count() == 0 || self.count() > MAX_COPIES {
            return Some(("pattern", "An array makes between one and 1024 copies."));
        }
        match self {
            ArrayPattern::Linear { spacing, .. } if !(spacing.x.is_finite() && spacing.y.is_finite()) => Some(("pattern", "An array spacing must be a finite vector.")),
            ArrayPattern::Linear { spacing, .. } if spacing.x == 0.0 && spacing.y == 0.0 => Some(("pattern", "An array spacing must have a length.")),
            ArrayPattern::Radial { center, step, .. } if !(center.x.is_finite() && center.y.is_finite() && step.is_finite()) => Some(("pattern", "A radial array needs a finite centre and step.")),
            ArrayPattern::Radial { step, .. } if *step == 0.0 => Some(("pattern", "A radial array step must turn.")),
            _ => None,
        }
    }
}

/// 🔢️ The most copies one array makes.
pub const MAX_COPIES: u32 = 1024;
//#endregion 🔖️Vocabulary

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
