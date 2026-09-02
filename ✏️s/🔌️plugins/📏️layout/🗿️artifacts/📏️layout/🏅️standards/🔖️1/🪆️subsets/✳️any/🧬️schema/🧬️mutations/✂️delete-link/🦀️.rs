//! 🗑️ `delete-link` — removes an {@link ImageLink} by id; inverse recreates it via `create-link`.


use crate::artifacts::layout::{LayoutDiff, LayoutSnapshot};
use crate::artifacts::layout::mutations::{LayoutMutation, create_link};
use crate::artifacts::layout::schema::diff::LayoutLinksDelta;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🗑️DeleteLink
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, ToValue, FromValue)]
#[cfg_attr(test, derive(Serialize, Deserialize))]
#[mutation_leaf(contract = ::protocol)]
pub struct DeleteLink {
    pub id: String,
}

impl MutationKind<LayoutSnapshot, LayoutMutation> for DeleteLink {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "delete", entity: "link", kind: "delete-link", record: "DeletedLink" };
    async fn diff(&self, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
        diff_delete_link(self, base)
    }
    async fn inverse(&self, base: &LayoutSnapshot) -> Vec<LayoutMutation> {
        inverse_delete_link(self, base)
    }
    async fn label(&self) -> String {
        format!("Delete link \"{}\"", self.id)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🗑️DeleteLink


//#region 🗑️DeleteLink
pub async fn diff_delete_link(payload: &DeleteLink, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
    if !base.links.iter().any(|link| link.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Link \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    protocol::MutationOutcome::new(LayoutDiff { links: Some(LayoutLinksDelta { removed: vec![payload.id.clone()], ..Default::default() }), ..Default::default() })
}
//#endregion 🗑️DeleteLink


//#region 🗑️DeleteLink
pub async fn inverse_delete_link(payload: &DeleteLink, base: &LayoutSnapshot) -> Vec<LayoutMutation> {
    match base.links.iter().position(|link| link.id == payload.id) {
        Some(index) => vec![LayoutMutation::CreateLink(create_link::CreateLink { link: base.links[index].clone(), index: Some(index) })],
        None => Vec::new(),
    }
}
//#endregion 🗑️DeleteLink
