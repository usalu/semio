//! 🔧 `change-tightness-class` payload — changes the En1992 document's `tightness_class` (EN 1992 input).

use crate::artifacts::en1992::diff::En1992Diff;
use crate::artifacts::en1992::mutations::En1992Mutation;
use crate::artifacts::en1992::En1992Snapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeTightnessClass
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeTightnessClass {
    pub new_tightness_class: crate::artifacts::en1992::part_3::TightnessClass,
}

impl protocol::MutationKind<En1992Snapshot, En1992Mutation> for ChangeTightnessClass {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "tightness-class", kind: "change-tightness-class", record: "ChangedTightnessClass" };

    fn diff(&self, base: &En1992Snapshot) -> En1992Diff {
        crate::artifacts::en1992::mutations::change_tightness_class::diff::diff(self, base)
    }

    fn inverse(&self, base: &En1992Snapshot) -> Vec<En1992Mutation> {
        crate::artifacts::en1992::mutations::change_tightness_class::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change tightness class to {:?}", self.new_tightness_class)
    }
}
//#endregion 🔖️ChangeTightnessClass
