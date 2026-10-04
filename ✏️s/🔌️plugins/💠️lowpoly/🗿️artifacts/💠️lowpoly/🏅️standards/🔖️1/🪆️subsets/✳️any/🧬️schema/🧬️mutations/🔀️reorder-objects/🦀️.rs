//! 🔀️ `reorder-objects` — repositions an object within the document's ordered object list (display
//! order in the outliner; never spatial — see `move-object`/`rotate-object`/`scale-object` for that).

use crate::{LowpolyMutation, LowpolySnapshot};
#[cfg(test)]
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(Serialize, Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ReorderObjects {
    pub id: String,
    pub to_index: usize,
}

impl protocol::MutationKind<LowpolySnapshot, LowpolyMutation> for ReorderObjects {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "reorder", entity: "object", kind: "reorder-objects", record: "ReorderedObjects" };

    fn diff(&self, base: &LowpolySnapshot) -> protocol::MutationOutcome<<LowpolyMutation as protocol::Mutation<LowpolySnapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &LowpolySnapshot) -> Result<Vec<LowpolyMutation>, semio_framework_value::ValueError> {
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
