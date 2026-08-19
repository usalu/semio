//! 🌍️ `change-annex` — sets the annex scalar.

use crate::artifacts::en1993::{En1993Mutation, En1993Snapshot};
use crate::document::AnnexChoice;
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChangeAnnex {
    pub new_annex: AnnexChoice,
}

impl protocol::MutationKind<En1993Snapshot, En1993Mutation> for ChangeAnnex {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "annex", kind: "change-annex", record: "ChangedAnnex" };

    async fn diff(&self, base: &En1993Snapshot) -> protocol::MutationOutcome<<En1993Mutation as protocol::Mutation<En1993Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &En1993Snapshot) -> Vec<En1993Mutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Change national annex to {:?}", self.new_annex)
    }
}
//#endregion 🔖️Payload
