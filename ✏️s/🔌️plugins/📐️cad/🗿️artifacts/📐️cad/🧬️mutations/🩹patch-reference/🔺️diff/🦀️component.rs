//! 🔺️ Diff fragment yielded by `PatchReference`.
use crate::artifacts::cad::diff::CadDiff;
use crate::artifacts::cad::mutations::CadMutation;
use crate::artifacts::cad::CadProjection;
use protocol::MutationDiff;
use serde::{Deserialize, Serialize};

//#region 🔖️Diff
/// @emoji 🔺️ Diff produced by one `PatchReference` mutation.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct PatchReferenceDiff {
    pub mutation: Option<CadMutation>,
}

impl PatchReferenceDiff {
    pub fn from_mutation(mutation: CadMutation) -> Self {
        Self { mutation: Some(mutation) }
    }
}

impl MutationDiff<CadProjection> for PatchReferenceDiff {
    fn apply(&self, projection: &CadProjection) -> CadProjection {
        match &self.mutation {
            Some(mutation) => {
                let diff = <CadMutation as protocol::Mutation<CadProjection>>::diff(mutation, projection);
                <CadDiff as MutationDiff<CadProjection>>::apply(&diff, projection)
            }
            None => projection.clone(),
        }
    }

    fn absorb(&mut self, other: Self) {
        if other.mutation.is_some() {
            *self = other;
        }
    }
}
//#endregion 🔖️Diff
