//! 🧾 `change-data-fields` — whole-field replace for `LayoutSnapshot::data_fields`. Semantic
//! replacement for the retired `SetDataFields` generic variant; the `fields:in` workflow port's
//! real, undoable write (see `crate::editor::layout::LayoutPlayApp::import_media`).

use crate::mutations::LayoutMutation;
use crate::{LayoutDiff, LayoutSnapshot};
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🧾ChangeDataFields
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, ToValue, FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct ChangeDataFields {
    pub new_fields: Option<crate::FormDictionary>,
}


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
    use crate::standards::v1::subsets::any::schema::diff::{data_entry_rows, DataFieldsPresence, LayoutDataEntriesDelta, LayoutDataEntryInsertion, LayoutDataFieldsDelta};
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
pub fn inverse_change_data_fields(_payload: &ChangeDataFields, base: &LayoutSnapshot) -> Result<Vec<LayoutMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![LayoutMutation::ChangeDataFields(ChangeDataFields { new_fields: base.data_fields.clone() })]

    })())
}
//#endregion 🧾ChangeDataFields
