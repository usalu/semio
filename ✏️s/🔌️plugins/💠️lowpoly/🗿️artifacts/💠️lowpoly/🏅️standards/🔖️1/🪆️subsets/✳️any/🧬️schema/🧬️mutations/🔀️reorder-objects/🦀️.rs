//! 🔀️ `reorder-objects` — repositions an object within the document's ordered object list (display
//! order in the outliner; never spatial — see `move-object`/`rotate-object`/`scale-object` for that).

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
pub struct ReorderObjects {
    pub id: String,
    pub to_index: usize,
}

impl<Op> protocol::MutationKind<LowpolySnapshot, Op> for ReorderObjects
where
    Op: protocol::Mutation<LowpolySnapshot, Diff = semio_s_artifact_lowpoly_lowpoly_model_diff::LowpolyDiff> + From<crate::reorder_objects::ReorderObjects>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "reorder", entity: "object", kind: "reorder-objects", record: "ReorderedObjects" };

    fn diff(&self, base: &LowpolySnapshot) -> protocol::MutationOutcome<<Op as protocol::Mutation<LowpolySnapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &LowpolySnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Reorder object \"{}\" to {}", self.id, self.to_index), &format!("Objekt \"{}\" an Position {} verschieben", self.id, self.to_index))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Payload
