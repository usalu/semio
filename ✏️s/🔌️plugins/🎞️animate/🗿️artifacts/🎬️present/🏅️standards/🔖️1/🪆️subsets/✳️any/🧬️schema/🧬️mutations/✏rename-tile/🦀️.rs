//! ✏️ `rename-tile` mutation payload — sets a figure tile crop's display name.

use crate::artifacts::present::PresentSnapshot;
use crate::artifacts::present::diff::PresentDiff;
use crate::artifacts::present::mutations::PresentMutation;
use protocol::{MutationKind, SemanticDescriptor};

//#region 🔹Payload
/// ✏️ Sets the `tiles` entry addressed by `id`'s `name` to `new_name`. Diff/inverse delegate to
/// the sibling `🔺️diff`/`↩️inverse` leaves.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "rename-tile")]
pub struct RenameTile {
    pub id: String,
    pub new_name: String,
}

impl MutationKind<PresentSnapshot, PresentMutation> for RenameTile {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "rename", entity: "tile", kind: "rename-tile", record: "RenamedTile" };

    fn diff(&self, base: &PresentSnapshot) -> protocol::MutationOutcome<PresentDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &PresentSnapshot) -> Vec<PresentMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Rename tile to \"{}\"", self.new_name)
    }

    fn target(&self) -> Vec<String> {
        vec!["tiles".into(), self.id.clone(), "name".into()]
    }
}
//#endregion 🔹Payload
