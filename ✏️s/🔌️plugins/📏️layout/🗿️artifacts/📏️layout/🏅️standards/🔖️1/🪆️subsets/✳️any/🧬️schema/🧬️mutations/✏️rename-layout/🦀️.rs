//! ✏️ `rename-layout` — changes the document's identity `name` field.

use semio_s_artifact_layout_layout_diff::LayoutDiff;
use semio_s_artifact_layout_layout_model::snapshot::LayoutSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

//#region ✏️RenameLayout
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, ToValue, FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "testkit"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "testkit"), serde(rename_all = "camelCase"))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct RenameLayout {
    pub new_name: String,
}

impl<Op> MutationKind<LayoutSnapshot, Op> for RenameLayout where Op: protocol::Mutation<LayoutSnapshot, Diff = semio_s_artifact_layout_layout_diff::LayoutDiff> + From<crate::rename_layout::RenameLayout> {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "rename", entity: "layout", kind: "rename-layout", record: "RenamedLayout" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Rename document to \"{}\"", self.new_name), &format!("Dokument in \"{}\" umbenennen", self.new_name))
    }
}

impl<Op> protocol::ColdMutationKind<LayoutSnapshot, Op> for RenameLayout where Op: protocol::ColdMutation<LayoutSnapshot, ReferenceDiff = semio_s_artifact_layout_layout_diff::LayoutDiff> + From<crate::rename_layout::RenameLayout> {
    fn diff(&self, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
        diff_rename_layout(self, base)
    }
    fn inverse(&self, base: &LayoutSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        inverse_rename_layout(self, base)?
    
    })
}
}
//#endregion ✏️RenameLayout

//#region ✏️RenameLayout
pub fn diff_rename_layout(payload: &RenameLayout, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
    if base.name == payload.new_name {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Layout already has that name.");
    }
    protocol::MutationOutcome::new(LayoutDiff { name: Some(payload.new_name.clone()), ..Default::default() })
}
//#endregion ✏️RenameLayout

//#region ✏️RenameLayout
pub fn inverse_rename_layout<Op: From<crate::rename_layout::RenameLayout>>(_payload: &RenameLayout, base: &LayoutSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![Op::from(RenameLayout { new_name: base.name.clone() })]

    })())
}
//#endregion ✏️RenameLayout

