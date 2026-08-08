//! 🔺️ Diff fragment yielded by `PaintStroke`.
use crate::artifacts::lowpoly::diff::LowpolyDiff;
use crate::artifacts::lowpoly::mutations::LowpolyMutation;
use crate::artifacts::lowpoly::LowpolyProjection;
use protocol::MutationDiff;
use serde::{Deserialize, Serialize};

//#region 🔖️Diff
/// @emoji 🔺️ Diff produced by one `PaintStroke` mutation.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct PaintStrokeDiff {
    pub mutation: Option<LowpolyMutation>,
}

impl PaintStrokeDiff {
    pub fn from_mutation(mutation: LowpolyMutation) -> Self {
        Self { mutation: Some(mutation) }
    }

    pub fn into_lowpoly_diff(self) -> LowpolyDiff {
        LowpolyDiff { mutations: self.mutation.into_iter().collect() }
    }
}

impl MutationDiff<LowpolyProjection> for PaintStrokeDiff {
    fn apply(&self, projection: &LowpolyProjection) -> LowpolyProjection {
        self.clone().into_lowpoly_diff().apply(projection)
    }

    fn absorb(&mut self, other: Self) {
        if other.mutation.is_some() {
            *self = other;
        }
    }
}
//#endregion 🔖️Diff
