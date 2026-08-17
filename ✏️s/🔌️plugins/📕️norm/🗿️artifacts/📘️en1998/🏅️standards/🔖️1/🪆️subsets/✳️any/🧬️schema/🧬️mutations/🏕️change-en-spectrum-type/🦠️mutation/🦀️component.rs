//! 🏕️ `change-en-spectrum-type` payload — changes the En1998 document's `en_spectrum_type` (EN spectrum type).

use crate::artifacts::en1998::diff::En1998Diff;
use crate::artifacts::en1998::mutations::En1998Mutation;
use crate::artifacts::en1998::En1998Snapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeEnSpectrumType
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeEnSpectrumType {
    pub new_en_spectrum_type: String,
}

impl protocol::MutationKind<En1998Snapshot, En1998Mutation> for ChangeEnSpectrumType {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "en-spectrum-type", kind: "change-en-spectrum-type", record: "ChangedEnSpectrumType" };

    fn diff(&self, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
        crate::artifacts::en1998::mutations::change_en_spectrum_type::diff::diff(self, base)
    }

    fn inverse(&self, base: &En1998Snapshot) -> Vec<En1998Mutation> {
        crate::artifacts::en1998::mutations::change_en_spectrum_type::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change EN spectrum type to \"{}\"", self.new_en_spectrum_type)
    }
}
//#endregion 🔖️ChangeEnSpectrumType
