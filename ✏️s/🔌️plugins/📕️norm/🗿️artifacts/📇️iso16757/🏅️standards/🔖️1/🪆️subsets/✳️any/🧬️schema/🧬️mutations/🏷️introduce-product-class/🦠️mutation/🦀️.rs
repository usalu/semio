//! ➕️ `introduce-product-class` — insert into catalogue.product_classes.

use semio_s_artifact_norm_iso16757_model::part_1::ProductClass;
use semio_s_artifact_norm_iso16757_model::Iso16757Snapshot;

#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct IntroduceProductClass {
    pub product_class: ProductClass,
    pub index: Option<usize>,
}

impl<Op> protocol::MutationKind<Iso16757Snapshot, Op> for IntroduceProductClass where Op: protocol::Mutation<Iso16757Snapshot, Diff = semio_s_artifact_norm_iso16757_diff::Iso16757Diff> + From<crate::retire_product_class::mutation::RetireProductClass> {
    const SEMANTICS: protocol::SemanticDescriptor =
        protocol::SemanticDescriptor { verb: "insert", entity: "productClass", kind: "introduce-product-class", record: "IntroducedProductClass" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(
            &format!("Introduce product class \"{}\"", self.product_class.id),
            &format!("Produktklasse \"{}\" einführen", self.product_class.id),
        )
    }
    fn target(&self) -> Vec<String> {
        vec![self.product_class.id.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<Iso16757Snapshot, Op> for IntroduceProductClass where Op: protocol::ColdMutation<Iso16757Snapshot, ReferenceDiff = semio_s_artifact_norm_iso16757_diff::Iso16757Diff> + From<crate::retire_product_class::mutation::RetireProductClass> {
    fn diff(&self, base: &Iso16757Snapshot) -> protocol::MutationOutcome<<Op as protocol::ColdMutation<Iso16757Snapshot>>::ReferenceDiff> {
        crate::introduce_product_class::diff::diff(self, base)
    }
    fn inverse(&self, base: &Iso16757Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        crate::introduce_product_class::inverse::inverse(self, base)?
    
    })
}
}
