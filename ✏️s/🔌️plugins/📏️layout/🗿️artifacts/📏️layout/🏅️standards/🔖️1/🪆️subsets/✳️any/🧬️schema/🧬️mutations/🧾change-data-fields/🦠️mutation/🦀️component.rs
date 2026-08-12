//! 🧾 `change-data-fields` — whole-field replace for `LayoutSnapshot::data_fields_json`. Semantic
//! replacement for the retired `SetDataFields` generic variant; the `fields:in` workflow port's
//! real, undoable write (see `crate::apps::layout::commands::author::import_media`).

use crate::artifacts::layout::mutations::LayoutMutation;
use crate::artifacts::layout::{LayoutDiff, LayoutSnapshot};
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🧾ChangeDataFields
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChangeDataFields {
    pub new_json: Option<String>,
}

impl MutationKind<LayoutSnapshot, LayoutMutation> for ChangeDataFields {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "change", entity: "data-fields", kind: "change-data-fields", record: "ChangedDataFields" };
    fn diff(&self, base: &LayoutSnapshot) -> LayoutDiff {
        super::diff::diff_change_data_fields(self, base)
    }
    fn inverse(&self, base: &LayoutSnapshot) -> Vec<LayoutMutation> {
        super::inverse::inverse_change_data_fields(self, base)
    }
    fn label(&self) -> String {
        "Change data fields".into()
    }
}
//#endregion 🧾ChangeDataFields
