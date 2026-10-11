//! 🆕️ `introduce-product-group` — brings a new id-keyed catalogue product group into existence.

use crate::{part_1::ProductGroup, Iso16757Mutation, Iso16757Snapshot};

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

impl protocol::MutationKind<Iso16757Snapshot, Iso16757Mutation> for IntroduceProductGroup {
    const SEMANTICS: protocol::SemanticDescriptor =

        protocol::SemanticDescriptor { verb: "insert", entity: "productGroup", kind: "introduce-product-group", record: "IntroducedProductGroup" };

    fn diff(&self, base: &Iso16757Snapshot) -> protocol::MutationOutcome<<Iso16757Mutation as protocol::Mutation<Iso16757Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Iso16757Snapshot) -> Result<Vec<Iso16757Mutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Create product group \"{}\"", self.product_group.names.preferred.text), &format!("Produktgruppe \"{}\" erstellen", self.product_group.names.preferred.text))
    }
    fn target(&self) -> Vec<String> {
        vec![self.product_group.id.clone()]
    }
}
//#endregion 🔖️Payload
