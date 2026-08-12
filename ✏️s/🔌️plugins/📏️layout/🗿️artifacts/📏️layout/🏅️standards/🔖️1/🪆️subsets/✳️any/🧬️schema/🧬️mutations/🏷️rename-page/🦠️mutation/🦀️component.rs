//! 🏷️ `rename-page` — changes a page's identity `name` field.

use crate::artifacts::layout::mutations::LayoutMutation;
use crate::artifacts::layout::{LayoutDiff, LayoutSnapshot};
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🏷️RenamePage
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RenamePage {
    pub id: String,
    pub new_name: String,
}

impl MutationKind<LayoutSnapshot, LayoutMutation> for RenamePage {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "rename", entity: "page", kind: "rename-page", record: "RenamedPage" };
    fn diff(&self, base: &LayoutSnapshot) -> LayoutDiff {
        super::diff::diff_rename_page(self, base)
    }
    fn inverse(&self, base: &LayoutSnapshot) -> Vec<LayoutMutation> {
        super::inverse::inverse_rename_page(self, base)
    }
    fn label(&self) -> String {
        format!("Rename page to \"{}\"", self.new_name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🏷️RenamePage
