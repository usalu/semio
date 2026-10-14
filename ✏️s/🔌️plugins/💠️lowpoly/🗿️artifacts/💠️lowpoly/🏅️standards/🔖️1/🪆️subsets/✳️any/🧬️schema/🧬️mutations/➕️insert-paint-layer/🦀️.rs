//! ➕️ `insert-paint-layer` — places a new paint layer into an object's ordered (compositing-order)
//! layer list at a FINAL-state index; layers have no stable id, only position.

use semio_s_artifact_lowpoly_lowpoly_model::{snapshot::LowpolyPaintLayer, snapshot::LowpolySnapshot};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct InsertPaintLayer {
    pub object_id: String,
    pub index: usize,
    pub layer: LowpolyPaintLayer,
}

impl<Op> protocol::MutationKind<LowpolySnapshot, Op> for InsertPaintLayer
where
    Op: protocol::Mutation<LowpolySnapshot, Diff = semio_s_artifact_lowpoly_lowpoly_model_diff::LowpolyDiff> + From<crate::remove_paint_layer::RemovePaintLayer>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "insert", entity: "paint-layer", kind: "insert-paint-layer", record: "InsertedPaintLayer" };

    fn diff(&self, base: &LowpolySnapshot) -> protocol::MutationOutcome<<Op as protocol::Mutation<LowpolySnapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &LowpolySnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Insert paint layer \"{}\"", self.layer.name), &format!("Malebene \"{}\" einfügen", self.layer.name))
    }
    fn target(&self) -> Vec<String> {
        vec![self.object_id.clone()]
    }
}
//#endregion 🔖️Payload
