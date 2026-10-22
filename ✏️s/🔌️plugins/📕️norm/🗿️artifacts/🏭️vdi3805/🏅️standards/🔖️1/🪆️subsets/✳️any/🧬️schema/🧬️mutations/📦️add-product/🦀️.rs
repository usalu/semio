//! 📦️ `add-product` — brings a new id-keyed catalogue product into existence, addressed by its
//! article number (`identity.article_number`, the format's native product key).

use semio_s_artifact_norm_vdi3805_model::{CatalogueProduct, Vdi3805Snapshot};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct AddProduct {
    pub product: CatalogueProduct,
    pub index: Option<usize>,
}

impl<Op> protocol::MutationKind<Vdi3805Snapshot, Op> for AddProduct where Op: protocol::Mutation<Vdi3805Snapshot, Diff = semio_s_artifact_norm_vdi3805_diff::Vdi3805Diff> + From<crate::remove_product::RemoveProduct> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "add", entity: "product", kind: "add-product", record: "AddedProduct" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Create product \"{}\"", self.product.identity.article_number), &format!("Produkt \"{}\" erstellen", self.product.identity.article_number))
    }
    fn target(&self) -> Vec<String> {
        vec![self.product.identity.article_number.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<Vdi3805Snapshot, Op> for AddProduct where Op: protocol::ColdMutation<Vdi3805Snapshot, ReferenceDiff = semio_s_artifact_norm_vdi3805_diff::Vdi3805Diff> + From<crate::remove_product::RemoveProduct> {
    fn diff(&self, base: &Vdi3805Snapshot) -> protocol::MutationOutcome<<Op as protocol::ColdMutation<Vdi3805Snapshot>>::ReferenceDiff> {
        crate::add_product::diff::diff(self, base)
    }
    fn inverse(&self, base: &Vdi3805Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        crate::add_product::inverse::inverse(self, base)?
    
    })
}
}
//#endregion 🔖️Payload

//#region 🪢️Mounts
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
//#endregion 🪢️Mounts
