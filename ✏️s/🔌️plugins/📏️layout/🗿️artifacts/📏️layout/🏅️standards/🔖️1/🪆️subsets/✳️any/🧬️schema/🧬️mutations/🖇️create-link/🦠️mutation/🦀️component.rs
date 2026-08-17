//! 🖇️ `create-link` — brings a new {@link ImageLink} into existence in the id-keyed `links`
//! collection.

use crate::artifacts::layout::mutations::LayoutMutation;
use crate::artifacts::layout::{ImageLink, LayoutDiff, LayoutSnapshot};
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🖇️CreateLink
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateLink {
    pub link: ImageLink,
    pub index: Option<usize>,
}

impl MutationKind<LayoutSnapshot, LayoutMutation> for CreateLink {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "link", kind: "create-link", record: "CreatedLink" };
    fn diff(&self, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
        super::diff::diff_create_link(self, base)
    }
    fn inverse(&self, base: &LayoutSnapshot) -> Vec<LayoutMutation> {
        super::inverse::inverse_create_link(self, base)
    }
    fn label(&self) -> String {
        format!("Create link \"{}\"", self.link.path)
    }
    fn target(&self) -> Vec<String> {
        vec![self.link.id.clone()]
    }
}
//#endregion 🖇️CreateLink
