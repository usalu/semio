//! ✏️ `rename-tile` mutation payload — sets a figure tile crop's display name.
use crate::artifacts::present::diff::PresentDiff;
use crate::artifacts::present::mutations::PresentMutation;
use crate::artifacts::present::PresentSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔹Payload
/// ✏️ Sets the `tiles` entry addressed by `id`'s `name` to `new_name`. Diff/inverse delegate to
/// the sibling `🔺️diff`/`↩️inverse` leaves.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
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
