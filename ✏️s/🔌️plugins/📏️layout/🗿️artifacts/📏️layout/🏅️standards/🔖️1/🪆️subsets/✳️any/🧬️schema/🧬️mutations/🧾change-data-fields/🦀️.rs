//! 🧾 `change-data-fields` — whole-field replace for `LayoutSnapshot::data_fields_json`. Semantic
//! replacement for the retired `SetDataFields` generic variant; the `fields:in` workflow port's
//! real, undoable write (see `crate::editor::layout::LayoutPlayApp::import_media`).


use crate::artifacts::layout::{LayoutDiff, LayoutSnapshot};
use crate::artifacts::layout::mutations::LayoutMutation;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🧾ChangeDataFields
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, ToValue, FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
pub struct ChangeDataFields {
    pub new_json: Option<String>,
}

impl MutationKind<LayoutSnapshot, LayoutMutation> for ChangeDataFields {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "change", entity: "data-fields", kind: "change-data-fields", record: "ChangedDataFields" };
    async fn diff(&self, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
        diff_change_data_fields(self, base)
    }
    async fn inverse(&self, base: &LayoutSnapshot) -> Vec<LayoutMutation> {
        inverse_change_data_fields(self, base)
    }
    async fn label(&self) -> String {
        "Change data fields".into()
    }
}
//#endregion 🧾ChangeDataFields


//#region 🧾ChangeDataFields
pub async fn diff_change_data_fields(payload: &ChangeDataFields, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
    if base.data_fields_json == payload.new_json {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Data fields are already set to that value.");
    }
    protocol::MutationOutcome::new(LayoutDiff { data_fields_json: Some(payload.new_json.clone()), ..Default::default() })
}
//#endregion 🧾ChangeDataFields


//#region 🧾ChangeDataFields
pub async fn inverse_change_data_fields(_payload: &ChangeDataFields, base: &LayoutSnapshot) -> Vec<LayoutMutation> {
    vec![LayoutMutation::ChangeDataFields(ChangeDataFields { new_json: base.data_fields_json.clone() })]
}
//#endregion 🧾ChangeDataFields
