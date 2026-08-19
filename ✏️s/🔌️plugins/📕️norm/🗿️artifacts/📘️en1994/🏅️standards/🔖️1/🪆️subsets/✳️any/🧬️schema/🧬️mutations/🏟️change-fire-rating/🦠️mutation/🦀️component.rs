//! 🚒 `change-fire-rating` — sets the En 1994 fire resistance rating, e.g. r60 scalar.

use crate::artifacts::en1994::{En1994Mutation, En1994Snapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChangeFireRating {
    pub new_fire_rating: String,
}

impl protocol::MutationKind<En1994Snapshot, En1994Mutation> for ChangeFireRating {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "fire-rating", kind: "change-fire-rating", record: "ChangedFireRating" };

    async fn diff(&self, base: &En1994Snapshot) -> protocol::MutationOutcome<<En1994Mutation as protocol::Mutation<En1994Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &En1994Snapshot) -> Vec<En1994Mutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Change fire rating to \"{}\"", self.new_fire_rating)
    }
}
//#endregion 🔖️Payload
