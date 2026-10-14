//! `introduce-product-index` — insert into catalogue.product_indexes.

use semio_s_artifact_norm_iso16757_model::part_1::ProductIndex;
use semio_s_artifact_norm_iso16757_model::Iso16757Snapshot;

#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct IntroduceProductIndex {
    pub product_index: ProductIndex,
    pub index: Option<usize>,
}

impl<Op> protocol::MutationKind<Iso16757Snapshot, Op> for IntroduceProductIndex where Op: protocol::Mutation<Iso16757Snapshot, Diff = semio_s_artifact_norm_iso16757_diff::Iso16757Diff> + From<crate::retire_product_index::mutation::RetireProductIndex> {
    const SEMANTICS: protocol::SemanticDescriptor =
        protocol::SemanticDescriptor { verb: "insert", entity: "productIndex", kind: "introduce-product-index", record: "IntroducedProductIndex" };

    fn diff(&self, base: &Iso16757Snapshot) -> protocol::MutationOutcome<<Op as protocol::Mutation<Iso16757Snapshot>>::Diff> {
        crate::introduce_product_index::diff::diff(self, base)
    }
    fn inverse(&self, base: &Iso16757Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        crate::introduce_product_index::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(
            &format!("Introduce product index \"{}\"", self.product_index.id),
            &format!("Produktindex \"{}\" einführen", self.product_index.id),
        )
    }
    fn target(&self) -> Vec<String> {
        vec![self.product_index.id.clone()]
    }
}
