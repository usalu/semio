//! `introduce-product-series` — insert into catalogue.product_series.

use crate::{part_1::ProductSeries, Iso16757Mutation, Iso16757Snapshot};

#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
pub struct IntroduceProductSeries {
    pub product_series: ProductSeries,
    pub index: Option<usize>,
}

impl protocol::MutationKind<Iso16757Snapshot, Iso16757Mutation> for IntroduceProductSeries {
    const SEMANTICS: protocol::SemanticDescriptor =
        protocol::SemanticDescriptor { verb: "insert", entity: "productSeries", kind: "introduce-product-series", record: "IntroducedProductSeries" };

    fn diff(&self, base: &Iso16757Snapshot) -> protocol::MutationOutcome<<Iso16757Mutation as protocol::Mutation<Iso16757Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Iso16757Snapshot) -> Vec<Iso16757Mutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native(
            &format!("Introduce productSeries \"{}\"", self.product_series.id),
            &format!("productSeries \"{}\" erstellen", self.product_series.id),
        )
    }
    fn target(&self) -> Vec<String> {
        vec![self.product_series.id.clone()]
    }
}
