//! 🗑️ `delete-link` — removes an {@link ImageLink} by id; inverse recreates it via `create-link`.

use crate::create_link;
use semio_s_artifact_layout_layout_diff::LayoutLinksDelta;
use semio_s_artifact_layout_layout_diff::LayoutLinkRemoval;
use semio_s_artifact_layout_layout_diff::LayoutDiff;
use semio_s_artifact_layout_layout_model::snapshot::LayoutSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🗑️DeleteLink
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, ToValue, FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "testkit"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "testkit"), serde(rename_all = "camelCase"))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct DeleteLink {
    pub id: String,
}

impl<Op> MutationKind<LayoutSnapshot, Op> for DeleteLink where Op: protocol::Mutation<LayoutSnapshot, Diff = semio_s_artifact_layout_layout_diff::LayoutDiff> + From<crate::create_link::CreateLink> {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "delete", entity: "link", kind: "delete-link", record: "DeletedLink" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Delete link \"{}\"", self.id), &format!("Verknüpfung \"{}\" löschen", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<LayoutSnapshot, Op> for DeleteLink where Op: protocol::ColdMutation<LayoutSnapshot, ReferenceDiff = semio_s_artifact_layout_layout_diff::LayoutDiff> + From<crate::create_link::CreateLink> {
    fn diff(&self, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
        diff_delete_link(self, base)
    }
    fn inverse(&self, base: &LayoutSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        inverse_delete_link(self, base)?
    
    })
}
}
//#endregion 🗑️DeleteLink

//#region 🗑️DeleteLink
pub fn diff_delete_link(payload: &DeleteLink, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
    let Some(at) = base.links.iter().position(|link| link.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Link \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    protocol::MutationOutcome::new(LayoutDiff { links: Some(LayoutLinksDelta { removed: vec![LayoutLinkRemoval { id: payload.id.clone(), index: at }], ..Default::default() }), ..Default::default() })
}
//#endregion 🗑️DeleteLink

//#region 🗑️DeleteLink
pub fn inverse_delete_link<Op: From<crate::create_link::CreateLink>>(payload: &DeleteLink, base: &LayoutSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    match base.links.iter().position(|link| link.id == payload.id) {
        Some(index) => vec![Op::from(create_link::CreateLink { link: base.links[index].clone(), index: Some(index) })],
        None => Vec::new(),
    }

    })())
}
//#endregion 🗑️DeleteLink

