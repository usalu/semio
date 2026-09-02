//! 🔧 `change-annex` payload — changes the En1995 document's `annex` (national annex).
//! Repurposes the pre-migration `📄set-snapshot/` triad directory in place: `🦀️.rs`
//! path-includes this exact directory outside this facet's writable boundary, so the directory
//! name stays `📄set-snapshot` while its content becomes `ChangeAnnex` — see this ticket's wave2
//! report `sharedFileRequests` for the rename once a later pass can touch `🦀️.rs` (mirrors the
//! en1990/en1992 precedent).


use crate::artifacts::en1995::En1995Snapshot;
use crate::artifacts::en1995::diff::En1995Diff;
use crate::artifacts::en1995::mutations::En1995Mutation;
use crate::artifacts::en1995::mutations::set_snapshot::ChangeAnnex;
use crate::document::AnnexChoice;

//#region 🔖️ChangeAnnex
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangeAnnex {
    pub new_annex: AnnexChoice,
}

impl protocol::MutationKind<En1995Snapshot, En1995Mutation> for ChangeAnnex {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "annex", kind: "change-annex", record: "ChangedAnnex" };

    fn diff(&self, base: &En1995Snapshot) -> protocol::MutationOutcome<En1995Diff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &En1995Snapshot) -> Vec<En1995Mutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change national annex to {}", self.new_annex.label())
    }
}
//#endregion 🔖️ChangeAnnex
