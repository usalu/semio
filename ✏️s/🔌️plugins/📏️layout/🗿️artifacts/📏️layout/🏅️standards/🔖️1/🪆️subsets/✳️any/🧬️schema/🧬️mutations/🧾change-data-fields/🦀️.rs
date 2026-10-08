//! 🧾 `change-data-fields` — whole-field replace for `LayoutSnapshot::data_fields`. Semantic
//! replacement for the retired `SetDataFields` generic variant; the `fields:in` workflow port's
//! real, undoable write (see `crate::editor::layout::LayoutPlayApp::import_media`).

use crate::mutations::LayoutMutation;
use crate::{LayoutDiff, LayoutSnapshot};
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🧾ChangeDataFields
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, ToValue, FromValue)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct ChangeDataFields {
    pub new_fields: Option<crate::FormDictionary>,
}
#[cfg(test)]
impl serde::Serialize for ChangeDataFields {fn serialize<S:serde::Serializer>(&self,serializer:S)->Result<S::Ok,S::Error>{serde::Serialize::serialize(&serde_json::Value::from(semio_framework_value::ToValue::to_value(self)),serializer)}}
#[cfg(test)]
impl<'de>serde::Deserialize<'de>for ChangeDataFields{fn deserialize<D:serde::Deserializer<'de>>(deserializer:D)->Result<Self,D::Error>{let value=<serde_json::Value as serde::Deserialize>::deserialize(deserializer)?;<Self as semio_framework_value::FromValue>::from_value(semio_framework_value::DslValue::from(&value)).map_err(serde::de::Error::custom)}}

impl MutationKind<LayoutSnapshot, LayoutMutation> for ChangeDataFields {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "change", entity: "data-fields", kind: "change-data-fields", record: "ChangedDataFields" };
    fn diff(&self, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
        diff_change_data_fields(self, base)
    }
    fn inverse(&self, base: &LayoutSnapshot) -> Result<Vec<LayoutMutation>, semio_framework_value::ValueError> {
    Ok({
        inverse_change_data_fields(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Change data fields", "Datenfelder ändern")
    }
}
//#endregion 🧾ChangeDataFields

//#region 🧾ChangeDataFields
pub fn diff_change_data_fields(payload: &ChangeDataFields, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
    if base.data_fields == payload.new_fields {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Data fields are already set to that value.");
    }
    use crate::standards::v1::subsets::any::schema::diff::{DataFieldsPresence, LayoutDataEntriesDelta, LayoutDataEntryPatchEntry, LayoutDataFieldsDelta};
    let delta = match (&base.data_fields, &payload.new_fields) {
        (None, Some(next)) => LayoutDataFieldsDelta { presence: Some(DataFieldsPresence::Created), entries: Some(LayoutDataEntriesDelta { added: next.entries.clone(), ..Default::default() }) },
        (Some(_), None) => LayoutDataFieldsDelta { presence: Some(DataFieldsPresence::Deleted), entries: None },
        (Some(held), Some(next)) => {
            let removed: Vec<String> = held.entries.iter().filter(|entry| next.entries.iter().all(|candidate| candidate.question_id != entry.question_id)).map(|entry| entry.question_id.clone()).collect();
            let added: Vec<_> = next.entries.iter().filter(|entry| held.entries.iter().all(|candidate| candidate.question_id != entry.question_id)).cloned().collect();
            let patched: Vec<LayoutDataEntryPatchEntry> = next.entries.iter().filter(|entry| held.entries.iter().any(|candidate| candidate.question_id == entry.question_id && candidate != *entry)).map(|entry| LayoutDataEntryPatchEntry { id: entry.question_id.clone(), item: entry.clone() }).collect();
            let natural: Vec<&str> = held.entries.iter().filter(|entry| !removed.contains(&entry.question_id)).map(|entry| entry.question_id.as_str()).chain(added.iter().map(|entry| entry.question_id.as_str())).collect();
            let target: Vec<&str> = next.entries.iter().map(|entry| entry.question_id.as_str()).collect();
            let reordered = (natural != target).then(|| target.into_iter().map(str::to_string).collect());
            LayoutDataFieldsDelta { presence: None, entries: Some(LayoutDataEntriesDelta { added, removed, patched, reordered }) }
        }
        (None, None) => LayoutDataFieldsDelta::default(),
    };
    protocol::MutationOutcome::new(LayoutDiff { data_fields: Some(delta), ..Default::default() })
}
//#endregion 🧾ChangeDataFields

//#region 🧾ChangeDataFields
pub fn inverse_change_data_fields(_payload: &ChangeDataFields, base: &LayoutSnapshot) -> Result<Vec<LayoutMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![LayoutMutation::ChangeDataFields(ChangeDataFields { new_fields: base.data_fields.clone() })]

    })())
}
//#endregion 🧾ChangeDataFields
