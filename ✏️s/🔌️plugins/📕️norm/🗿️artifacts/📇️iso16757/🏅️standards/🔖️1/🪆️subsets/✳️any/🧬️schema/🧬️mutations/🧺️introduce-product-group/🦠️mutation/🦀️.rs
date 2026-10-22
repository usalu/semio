//! 🆕️ `introduce-product-group` — brings a new id-keyed catalogue product group into existence.

use semio_s_artifact_norm_iso16757_model::part_1::ProductGroup;
use semio_s_artifact_norm_iso16757_model::Iso16757Snapshot;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct IntroduceProductGroup {
    pub product_group: ProductGroup,
    pub index: Option<usize>,
}

impl<Op> protocol::MutationKind<Iso16757Snapshot, Op> for IntroduceProductGroup where Op: protocol::Mutation<Iso16757Snapshot, Diff = semio_s_artifact_norm_iso16757_diff::Iso16757Diff> + From<crate::retire_product_group::mutation::RetireProductGroup> {
    const SEMANTICS: protocol::SemanticDescriptor =

        protocol::SemanticDescriptor { verb: "insert", entity: "productGroup", kind: "introduce-product-group", record: "IntroducedProductGroup" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Create product group \"{}\"", self.product_group.names.preferred.text), &format!("Produktgruppe \"{}\" erstellen", self.product_group.names.preferred.text))
    }
    fn target(&self) -> Vec<String> {
        vec![self.product_group.id.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<Iso16757Snapshot, Op> for IntroduceProductGroup where Op: protocol::ColdMutation<Iso16757Snapshot, ReferenceDiff = semio_s_artifact_norm_iso16757_diff::Iso16757Diff> + From<crate::retire_product_group::mutation::RetireProductGroup> {
    fn diff(&self, base: &Iso16757Snapshot) -> protocol::MutationOutcome<<Op as protocol::ColdMutation<Iso16757Snapshot>>::ReferenceDiff> {
        crate::introduce_product_group::diff::diff(self, base)
    }
    fn inverse(&self, base: &Iso16757Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        crate::introduce_product_group::inverse::inverse(self, base)?
    
    })
}
}
//#endregion 🔖️Payload
