//! 🇪🇺 `change-annex` payload — changes the EN 1990 document's `annex` (national annex).
//! Repurposes the pre-migration `📄set-snapshot/` triad directory in place: `🦀️.rs`
//! path-includes this exact directory outside this facet's writable boundary, so the directory
//! name stays `📄set-snapshot` while its content becomes `ChangeAnnex` — see the migration
//! report's `sharedFileRequests` for the rename once a later pass can touch `🦀️.rs`.


use crate::artifacts::en1990::{En1990Diff, En1990Snapshot};
use crate::artifacts::en1990::mutations::En1990Mutation;
use crate::artifacts::en1990::mutations::set_snapshot::ChangeAnnex;
use crate::document::AnnexChoice;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeAnnex
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase")]
pub struct ChangeAnnex {
    pub new_annex: AnnexChoice,
}

impl protocol::MutationKind<En1990Snapshot, En1990Mutation> for ChangeAnnex {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "annex", kind: "change-annex", record: "ChangedAnnex" };

    fn diff(&self, base: &En1990Snapshot) -> protocol::MutationOutcome<<En1990Mutation as protocol::Mutation<En1990Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &En1990Snapshot) -> Vec<En1990Mutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change national annex to {}", self.new_annex.label())
    }
}
//#endregion 🔖️ChangeAnnex
