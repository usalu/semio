//! ➖️ `remove-paint-layer` — takes a paint layer out of an object's ordered (compositing-order)
//! layer list at a BASE-state index. Reuses this directory's pre-existing path (glue.rs still
//! `#[path]`-wires it) — same kebab slug survives the semantic-mutations rewrite unchanged.

use semio_s_artifact_lowpoly_lowpoly_model::snapshot::LowpolySnapshot;
#[cfg(test)]
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(Serialize, Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct RemovePaintLayer {
    pub object_id: String,
    pub index: usize,
}

impl<Op> protocol::MutationKind<LowpolySnapshot, Op> for RemovePaintLayer
where
    Op: protocol::Mutation<LowpolySnapshot, Diff = semio_s_artifact_lowpoly_lowpoly_model_diff::LowpolyDiff> + From<crate::insert_paint_layer::InsertPaintLayer>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "paint-layer", kind: "remove-paint-layer", record: "RemovedPaintLayer" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Remove paint layer {} from object \"{}\"", self.index, self.object_id), &format!("Malebene {} aus Objekt \"{}\" entfernen", self.index, self.object_id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.object_id.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<LowpolySnapshot, Op> for RemovePaintLayer
where
    Op: protocol::ColdMutation<LowpolySnapshot, ReferenceDiff = semio_s_artifact_lowpoly_lowpoly_model_diff::LowpolyDiff> + From<crate::insert_paint_layer::InsertPaintLayer>,
{
    fn diff(&self, base: &LowpolySnapshot) -> protocol::MutationOutcome<<Op as protocol::ColdMutation<LowpolySnapshot>>::ReferenceDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &LowpolySnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
}
//#endregion 🔖️Payload
