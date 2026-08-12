//! 🌱️ `create-page` — brings a new {@link Page} into existence in the id-keyed `pages` collection.

use crate::artifacts::layout::mutations::LayoutMutation;
use crate::artifacts::layout::{LayoutDiff, LayoutSnapshot, Page};
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🌱️CreatePage
/// 🌱️ `index` is descriptive of authoring intent (the append-only `LayoutPagesDelta` apply always
/// pushes at the end, matching the pre-migration `CollectionMutation::Add` behavior).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreatePage {
    pub page: Page,
    pub index: Option<usize>,
}

impl MutationKind<LayoutSnapshot, LayoutMutation> for CreatePage {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "page", kind: "create-page", record: "CreatedPage" };
    fn diff(&self, base: &LayoutSnapshot) -> LayoutDiff {
        super::diff::diff_create_page(self, base)
    }
    fn inverse(&self, base: &LayoutSnapshot) -> Vec<LayoutMutation> {
        super::inverse::inverse_create_page(self, base)
    }
    fn label(&self) -> String {
        format!("Create page \"{}\"", self.page.name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.page.id.clone()]
    }
}
//#endregion 🌱️CreatePage
