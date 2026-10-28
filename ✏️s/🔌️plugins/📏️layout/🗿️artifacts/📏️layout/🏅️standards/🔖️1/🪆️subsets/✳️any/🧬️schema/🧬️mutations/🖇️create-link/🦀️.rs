//! 🖇️ `create-link` — brings a new {@link ImageLink} into existence in the id-keyed `links`
//! collection.

use crate::delete_link;
use semio_s_artifact_layout_layout_diff::LayoutLinksDelta;
use semio_s_artifact_layout_layout_diff::LayoutLinkInsertion;
use semio_s_artifact_layout_layout_model::ImageLink;
use semio_s_artifact_layout_layout_diff::LayoutDiff;
use semio_s_artifact_layout_layout_model::snapshot::LayoutSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🖇️CreateLink
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, ToValue, FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "testkit"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "testkit"), serde(rename_all = "camelCase"))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct CreateLink {
    pub link: ImageLink,
    pub index: Option<usize>,
}

impl<Op> MutationKind<LayoutSnapshot, Op> for CreateLink where Op: protocol::Mutation<LayoutSnapshot, Diff = semio_s_artifact_layout_layout_diff::LayoutDiff> + From<crate::delete_link::DeleteLink> {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "link", kind: "create-link", record: "CreatedLink" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Create link \"{}\"", self.link.path), &format!("Verknüpfung \"{}\" erstellen", self.link.path))
    }
    fn target(&self) -> Vec<String> {
        vec![self.link.id.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<LayoutSnapshot, Op> for CreateLink where Op: protocol::ColdMutation<LayoutSnapshot, ReferenceDiff = semio_s_artifact_layout_layout_diff::LayoutDiff> + From<crate::delete_link::DeleteLink> {
    fn diff(&self, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
        diff_create_link(self, base)
    }
    fn inverse(&self, base: &LayoutSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        inverse_create_link(self, base)?
    
    })
}
}
//#endregion 🖇️CreateLink

//#region 🖇️CreateLink
pub fn diff_create_link(payload: &CreateLink, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
    if let Err(message) = semio_s_artifact_layout_layout_model::validation::validate_id(&payload.link.id) {
        return protocol::MutationOutcome::fatal("mutation.invariant", message, [payload.link.id.clone()]);
    }
    if base.links.iter().any(|link| link.id == payload.link.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A link with id \"{}\" already exists.", payload.link.id), [payload.link.id.clone()]);
    }
    if payload.index.is_some_and(|at| at > base.links.len()) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Insert index {} is past the end of {} rows.", payload.index.unwrap_or_default(), base.links.len()), [&payload.link.id.to_string()]);
    }
    protocol::MutationOutcome::new(LayoutDiff { links: Some(LayoutLinksDelta { inserted: vec![LayoutLinkInsertion { index: payload.index.unwrap_or(base.links.len()), row: payload.link.clone() }], ..Default::default() }), ..Default::default() })
}
//#endregion 🖇️CreateLink

//#region 🖇️CreateLink
pub fn inverse_create_link<Op: From<crate::delete_link::DeleteLink>>(payload: &CreateLink, base: &LayoutSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    if diff_create_link(payload, base).diff() == &LayoutDiff::default() { return Ok(Vec::new()); }
    Ok((|| {
    vec![Op::from(delete_link::DeleteLink { id: payload.link.id.clone() })]

    })())
}
//#endregion 🖇️CreateLink

