//! 🔺️ Diff fragment yielded by `TranslateAssets`.
use crate::artifacts::shooting::mutations::ShootingMutation;
use crate::artifacts::shooting::ShootingFixture;
use protocol::MutationDiff;
use serde::{Deserialize, Serialize};

//#region 🔖️Diff
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct TranslateAssetsDiff {
    pub mutation: Option<ShootingMutation>,
}

impl TranslateAssetsDiff {
    pub fn from_mutation(mutation: ShootingMutation) -> Self {
        Self { mutation: Some(mutation) }
    }
}

impl MutationDiff<ShootingFixture> for TranslateAssetsDiff {
    fn apply(&self, projection: &ShootingFixture) -> ShootingFixture {
        match &self.mutation {
            Some(m) => {
                let diff = <ShootingMutation as protocol::Mutation<ShootingFixture>>::diff(m, projection);
                MutationDiff::apply(&diff, projection)
            }
            None => projection.clone(),
        }
    }
    fn absorb(&mut self, other: Self) {
        if other.mutation.is_some() { *self = other; }
    }
}
//#endregion 🔖️Diff
