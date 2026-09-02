//! 🔧 `change-annex` payload — changes the Din16798 document's `annex` (national annex).
//! Repurposes the pre-migration `📄set-snapshot/` triad directory in place: `🦀️.rs`
//! path-includes this exact directory outside this facet's writable boundary, so the
//! directory name stays `📄set-snapshot` while its content becomes `ChangeAnnex` — see the
//! migration report's `sharedFileRequests` for the rename once a later pass can touch
//! `🦀️.rs`.


use crate::artifacts::din16798::Din16798Snapshot;
use crate::artifacts::din16798::diff::Din16798Diff;
use crate::artifacts::din16798::mutations::Din16798Mutation;
use crate::artifacts::din16798::mutations::change_annex::ChangeAnnex;
use crate::document::AnnexChoice;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeAnnex
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase")]
pub struct ChangeAnnex {
    pub new_annex: AnnexChoice,
}

impl protocol::MutationKind<Din16798Snapshot, Din16798Mutation> for ChangeAnnex {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "annex", kind: "change-annex", record: "ChangedAnnex" };

    fn diff(&self, base: &Din16798Snapshot) -> protocol::MutationOutcome<Din16798Diff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change national annex to {}", self.new_annex.label())
    }
}
//#endregion 🔖️ChangeAnnex
