//! 🔗 `change-link-path` — sets an {@link ImageLink}'s file `path`.

use crate::artifacts::layout::mutations::LayoutMutation;
use crate::artifacts::layout::{LayoutDiff, LayoutSnapshot};
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔗ChangeLinkPath
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChangeLinkPath {
    pub id: String,
    pub new_path: String,
}

impl MutationKind<LayoutSnapshot, LayoutMutation> for ChangeLinkPath {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "change", entity: "link-path", kind: "change-link-path", record: "ChangedLinkPath" };
    async fn diff(&self, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
        super::diff::diff_change_link_path(self, base)
    }
    async fn inverse(&self, base: &LayoutSnapshot) -> Vec<LayoutMutation> {
        super::inverse::inverse_change_link_path(self, base)
    }
    async fn label(&self) -> String {
        format!("Change link \"{}\" path", self.id)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔗ChangeLinkPath
