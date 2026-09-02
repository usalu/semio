//! 🔗 `change-link-path` — sets an {@link ImageLink}'s file `path`.


use crate::artifacts::layout::{ImageLinkPatch, LayoutDiff, LayoutSnapshot};
use crate::artifacts::layout::mutations::LayoutMutation;
use crate::artifacts::layout::schema::diff::{LayoutLinkPatchEntry, LayoutLinksDelta};
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔗ChangeLinkPath
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, ToValue, FromValue)]
#[cfg_attr(test, derive(Serialize, Deserialize))]
#[mutation_leaf(contract = ::protocol)]
pub struct ChangeLinkPath {
    pub id: String,
    pub new_path: String,
}

impl MutationKind<LayoutSnapshot, LayoutMutation> for ChangeLinkPath {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "change", entity: "link-path", kind: "change-link-path", record: "ChangedLinkPath" };
    async fn diff(&self, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
        diff_change_link_path(self, base)
    }
    async fn inverse(&self, base: &LayoutSnapshot) -> Vec<LayoutMutation> {
        inverse_change_link_path(self, base)
    }
    async fn label(&self) -> String {
        format!("Change link \"{}\" path", self.id)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔗ChangeLinkPath


//#region 🔗ChangeLinkPath
pub async fn diff_change_link_path(payload: &ChangeLinkPath, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
    let Some(link) = base.links.iter().find(|link| link.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Link \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if link.path == payload.new_path {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Link \"{}\" already has path \"{}\".", payload.id, payload.new_path));
    }
    protocol::MutationOutcome::new(LayoutDiff {
        links: Some(LayoutLinksDelta { patched: vec![LayoutLinkPatchEntry { id: payload.id.clone(), patch: ImageLinkPatch { path: Some(payload.new_path.clone()) } }], ..Default::default() }),
        ..Default::default()
    })
}
//#endregion 🔗ChangeLinkPath


//#region 🔗ChangeLinkPath
pub async fn inverse_change_link_path(payload: &ChangeLinkPath, base: &LayoutSnapshot) -> Vec<LayoutMutation> {
    match base.links.iter().find(|link| link.id == payload.id) {
        Some(link) => vec![LayoutMutation::ChangeLinkPath(ChangeLinkPath { id: payload.id.clone(), new_path: link.path.clone() })],
        None => Vec::new(),
    }
}
//#endregion 🔗ChangeLinkPath
