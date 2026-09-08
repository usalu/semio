//! 🖇️ `create-link` — brings a new {@link ImageLink} into existence in the id-keyed `links`
//! collection.


use crate::{ImageLink, LayoutDiff, LayoutSnapshot};
use crate::mutations::{LayoutMutation, delete_link};
use crate::standards::v1::subsets::any::schema::diff::LayoutLinksDelta;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🖇️CreateLink
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, ToValue, FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
pub struct CreateLink {
    pub link: ImageLink,
    pub index: Option<usize>,
}

impl MutationKind<LayoutSnapshot, LayoutMutation> for CreateLink {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "link", kind: "create-link", record: "CreatedLink" };
    fn diff(&self, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
        diff_create_link(self, base)
    }
    fn inverse(&self, base: &LayoutSnapshot) -> Vec<LayoutMutation> {
        inverse_create_link(self, base)
    }
    fn label(&self) -> String {
        format!("Create link \"{}\"", self.link.path)
    }
    fn target(&self) -> Vec<String> {
        vec![self.link.id.clone()]
    }
}
//#endregion 🖇️CreateLink


//#region 🖇️CreateLink
pub fn diff_create_link(payload: &CreateLink, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
    if base.links.iter().any(|link| link.id == payload.link.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A link with id \"{}\" already exists.", payload.link.id), [payload.link.id.clone()]);
    }
    protocol::MutationOutcome::new(LayoutDiff { links: Some(LayoutLinksDelta { added: vec![payload.link.clone()], ..Default::default() }), ..Default::default() })
}
//#endregion 🖇️CreateLink


//#region 🖇️CreateLink
pub fn inverse_create_link(payload: &CreateLink, _base: &LayoutSnapshot) -> Vec<LayoutMutation> {
    vec![LayoutMutation::DeleteLink(delete_link::DeleteLink { id: payload.link.id.clone() })]
}
//#endregion 🖇️CreateLink
