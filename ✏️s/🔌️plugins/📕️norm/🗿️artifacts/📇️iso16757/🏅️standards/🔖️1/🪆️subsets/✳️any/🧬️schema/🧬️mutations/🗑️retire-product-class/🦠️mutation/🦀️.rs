//! ➖️ `retire-product-class` — remove from catalogue.product_classes.

use semio_s_artifact_norm_iso16757_model::Iso16757Snapshot;

#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct RetireProductClass {
    pub id: String,
}

impl<Op> protocol::MutationKind<Iso16757Snapshot, Op> for RetireProductClass where Op: protocol::Mutation<Iso16757Snapshot, Diff = semio_s_artifact_norm_iso16757_diff::Iso16757Diff> + From<crate::introduce_product_class::mutation::IntroduceProductClass> {
    const SEMANTICS: protocol::SemanticDescriptor =
        protocol::SemanticDescriptor { verb: "remove", entity: "productClass", kind: "retire-product-class", record: "RetiredProductClass" };

    fn diff(&self, base: &Iso16757Snapshot) -> protocol::MutationOutcome<<Op as protocol::Mutation<Iso16757Snapshot>>::Diff> {
        crate::retire_product_class::diff::diff(self, base)
    }
    fn inverse(&self, base: &Iso16757Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        crate::retire_product_class::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(
            &format!("Retire product class \"{}\"", self.id),
            &format!("Produktklasse \"{}\" zurückziehen", self.id),
        )
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
