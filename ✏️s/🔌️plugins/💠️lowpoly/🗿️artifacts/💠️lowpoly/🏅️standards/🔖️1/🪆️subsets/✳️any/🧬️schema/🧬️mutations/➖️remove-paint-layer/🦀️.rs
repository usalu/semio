//! ➖️ `remove-paint-layer` — takes a paint layer out of an object's ordered (compositing-order)
//! layer list at a BASE-state index. Reuses this directory's pre-existing path (glue.rs still
//! `#[path]`-wires it) — same kebab slug survives the semantic-mutations rewrite unchanged.

use crate::{LowpolyMutation, LowpolySnapshot};
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

impl protocol::MutationKind<LowpolySnapshot, LowpolyMutation> for RemovePaintLayer {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "paint-layer", kind: "remove-paint-layer", record: "RemovedPaintLayer" };

    fn diff(&self, base: &LowpolySnapshot) -> protocol::MutationOutcome<<LowpolyMutation as protocol::Mutation<LowpolySnapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &LowpolySnapshot) -> Result<Vec<LowpolyMutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Remove paint layer {} from object \"{}\"", self.index, self.object_id), &format!("Malebene {} aus Objekt \"{}\" entfernen", self.index, self.object_id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.object_id.clone()]
    }
}
//#endregion 🔖️Payload
