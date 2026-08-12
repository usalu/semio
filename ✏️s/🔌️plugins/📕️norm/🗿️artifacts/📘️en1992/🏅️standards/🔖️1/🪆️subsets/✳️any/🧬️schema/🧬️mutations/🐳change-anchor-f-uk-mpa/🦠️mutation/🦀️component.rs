//! 🔧 `change-anchor-f-uk-mpa` payload — changes the En1992 document's `anchor_f_uk_mpa` (EN 1992 input).

use crate::artifacts::en1992::diff::En1992Diff;
use crate::artifacts::en1992::mutations::En1992Mutation;
use crate::artifacts::en1992::En1992Snapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeAnchorFUkMpa
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeAnchorFUkMpa {
    pub new_anchor_f_uk_mpa: f64,
}

impl protocol::MutationKind<En1992Snapshot, En1992Mutation> for ChangeAnchorFUkMpa {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "anchor-f-uk-mpa", kind: "change-anchor-f-uk-mpa", record: "ChangedAnchorFUkMpa" };

    fn diff(&self, base: &En1992Snapshot) -> En1992Diff {
        crate::artifacts::en1992::mutations::change_anchor_f_uk_mpa::diff::diff(self, base)
    }

    fn inverse(&self, base: &En1992Snapshot) -> Vec<En1992Mutation> {
        crate::artifacts::en1992::mutations::change_anchor_f_uk_mpa::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change anchor f uk mpa to {:?}", self.new_anchor_f_uk_mpa)
    }
}
//#endregion 🔖️ChangeAnchorFUkMpa
