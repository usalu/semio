//! 🏷️ `rename-page` — changes a page's identity `name` field.


use crate::artifacts::layout::{LayoutDiff, LayoutSnapshot, PagePatch};
use crate::artifacts::layout::mutations::LayoutMutation;
use crate::artifacts::layout::schema::diff::{LayoutPagePatchEntry, LayoutPagesDelta};
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🏷️RenamePage
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, ToValue, FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
pub struct RenamePage {
    pub id: String,
    pub new_name: String,
}

impl MutationKind<LayoutSnapshot, LayoutMutation> for RenamePage {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "rename", entity: "page", kind: "rename-page", record: "RenamedPage" };
    async fn diff(&self, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
        diff_rename_page(self, base)
    }
    async fn inverse(&self, base: &LayoutSnapshot) -> Vec<LayoutMutation> {
        inverse_rename_page(self, base)
    }
    async fn label(&self) -> String {
        format!("Rename page to \"{}\"", self.new_name)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🏷️RenamePage


//#region 🏷️RenamePage
pub async fn diff_rename_page(payload: &RenamePage, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
    let Some(page) = base.pages.iter().find(|page| page.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Page \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if page.name == payload.new_name {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Page \"{}\" already has that name.", payload.id));
    }
    protocol::MutationOutcome::new(LayoutDiff {
        pages: Some(LayoutPagesDelta { patched: vec![LayoutPagePatchEntry { id: payload.id.clone(), patch: PagePatch { name: Some(payload.new_name.clone()), ..Default::default() } }], ..Default::default() }),
        ..Default::default()
    })
}
//#endregion 🏷️RenamePage


//#region 🏷️RenamePage
pub async fn inverse_rename_page(payload: &RenamePage, base: &LayoutSnapshot) -> Vec<LayoutMutation> {
    match base.pages.iter().find(|page| page.id == payload.id) {
        Some(page) => vec![LayoutMutation::RenamePage(RenamePage { id: payload.id.clone(), new_name: page.name.clone() })],
        None => Vec::new(),
    }
}
//#endregion 🏷️RenamePage
