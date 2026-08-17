//! 🔀 `reorder-pages` — repositions a page within the display-ordered `pages` list (document page
//! sequence, unlike `stories`/`links` which have no display order).

use crate::artifacts::layout::mutations::LayoutMutation;
use crate::artifacts::layout::{LayoutDiff, LayoutSnapshot};
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔀ReorderPages
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReorderPages {
    pub id: String,
    pub to_index: usize,
}

impl MutationKind<LayoutSnapshot, LayoutMutation> for ReorderPages {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "reorder", entity: "pages", kind: "reorder-pages", record: "ReorderedPages" };
    fn diff(&self, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
        super::diff::diff_reorder_pages(self, base)
    }
    fn inverse(&self, base: &LayoutSnapshot) -> Vec<LayoutMutation> {
        super::inverse::inverse_reorder_pages(self, base)
    }
    fn label(&self) -> String {
        format!("Reorder page \"{}\"", self.id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔀ReorderPages
