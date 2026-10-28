//! 🧾 `change-data-fields` — whole-field replace for `LayoutSnapshot::data_fields`. Semantic
//! replacement for the retired `SetDataFields` generic variant; the `fields:in` workflow port's
//! real, undoable write (see `crate::editor::layout::LayoutPlayApp::import_media`).

use semio_s_artifact_layout_layout_diff::LayoutDiff;
use semio_s_artifact_layout_layout_model::snapshot::LayoutSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🧾ChangeDataFields
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, ToValue, FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct ChangeDataFields {
    pub new_fields: Option<semio_framework_value_dictionary::QuestionDictionary>,
}

impl<Op> MutationKind<LayoutSnapshot, Op> for ChangeDataFields where Op: protocol::Mutation<LayoutSnapshot, Diff = semio_s_artifact_layout_layout_diff::LayoutDiff> + From<crate::change_data_fields::ChangeDataFields> {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "change", entity: "data-fields", kind: "change-data-fields", record: "ChangedDataFields" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Change data fields", "Datenfelder ändern")
    }
}

impl<Op> protocol::ColdMutationKind<LayoutSnapshot, Op> for ChangeDataFields where Op: protocol::ColdMutation<LayoutSnapshot, ReferenceDiff = semio_s_artifact_layout_layout_diff::LayoutDiff> + From<crate::change_data_fields::ChangeDataFields> {
    fn diff(&self, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
        diff_change_data_fields(self, base)
    }
    fn inverse(&self, base: &LayoutSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        inverse_change_data_fields(self, base)?
    
    })
}
}
//#endregion 🧾ChangeDataFields

//#region 🧾ChangeDataFields
pub fn diff_change_data_fields(payload: &ChangeDataFields, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
    if base.data_fields == payload.new_fields {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Data fields are already set to that value.");
    }
    use semio_s_artifact_layout_layout_diff::{data_entry_rows, DataFieldsPresence, LayoutDataEntriesDelta, LayoutDataEntryInsertion, LayoutDataFieldsDelta};
    let delta = match (&base.data_fields, &payload.new_fields) {
        (None, Some(next)) => LayoutDataFieldsDelta { presence: Some(DataFieldsPresence::Created), entries: Some(LayoutDataEntriesDelta { inserted: next.entries.iter().cloned().enumerate().map(|(index, row)| LayoutDataEntryInsertion { index, row }).collect(), ..Default::default() }) },
        (Some(_), None) => LayoutDataFieldsDelta { presence: Some(DataFieldsPresence::Deleted), entries: None },
        (Some(held), Some(next)) => LayoutDataFieldsDelta { presence: None, entries: Some(data_entry_rows(&held.entries, &next.entries)) },
        (None, None) => LayoutDataFieldsDelta::default(),
    };
    protocol::MutationOutcome::new(LayoutDiff { data_fields: Some(delta), ..Default::default() })
}
//#endregion 🧾ChangeDataFields

//#region 🧾ChangeDataFields
pub fn inverse_change_data_fields<Op: From<crate::change_data_fields::ChangeDataFields>>(_payload: &ChangeDataFields, base: &LayoutSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![Op::from(ChangeDataFields { new_fields: base.data_fields.clone() })]

    })())
}
//#endregion 🧾ChangeDataFields

//#region 🔮️SerdeOracle
/// 🔮️ Test-only independent Serde projection for typed data-field changes.
#[cfg(any(test, feature = "testkit"))]
impl serde::Serialize for ChangeDataFields {fn serialize<S:serde::Serializer>(&self,serializer:S)->Result<S::Ok,S::Error>{serde::Serialize::serialize(&serde_json::Value::from(semio_framework_value::ToValue::to_value(self)),serializer)}}
#[cfg(any(test, feature = "testkit"))]
impl<'de>serde::Deserialize<'de>for ChangeDataFields{fn deserialize<D:serde::Deserializer<'de>>(deserializer:D)->Result<Self,D::Error>{let value=<serde_json::Value as serde::Deserialize>::deserialize(deserializer)?;<Self as semio_framework_value::FromValue>::from_value(semio_framework_value::DslValue::from(&value)).map_err(serde::de::Error::custom)}}
//#endregion 🔮️SerdeOracle
