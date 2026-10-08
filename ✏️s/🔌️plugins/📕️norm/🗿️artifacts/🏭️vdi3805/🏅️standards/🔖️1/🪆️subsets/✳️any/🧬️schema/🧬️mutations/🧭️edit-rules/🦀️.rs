//! 🧭️ The `setField` / `insertItem` / `removeItem` / `applyRemedy` vocabulary of the Vdi3805 editor: which value-tree path raises which concrete kind.

use crate::app_surface::{InsertItemRule, NormEdit, NormEditRules, RemoveItemRule, SelectorKey, SetFieldRule};
use super::Vdi3805Mutation;
use crate::Vdi3805Snapshot;

/// 📚 Every path a Vdi3805 editor edit resolves, one rule per kind that sets, inserts or removes through it.
pub const EDIT_RULES: NormEditRules = NormEditRules {
    set_field: &[
        SetFieldRule { path: "catalog.products[].configuration", kind: "change-product-configuration", selectors: &[SelectorKey::Id("id")], value: "newConfiguration" },
        SetFieldRule { path: "catalog.file", kind: "change-manufacturer-file", selectors: &[], value: "newManufacturerFile" },
        SetFieldRule { path: "catalog.products[].title", kind: "rename-product", selectors: &[SelectorKey::Id("id")], value: "newTitle" },
        SetFieldRule { path: "correctionAsOf", kind: "change-correction-as-of", selectors: &[], value: "newCorrectionAsOf" },
        SetFieldRule { path: "strictMode", kind: "change-strict-mode", selectors: &[], value: "newStrictMode" },
        SetFieldRule { path: "limits", kind: "change-limits", selectors: &[], value: "newLimits" },
    ],
    insert_item: &[
        InsertItemRule { path: "catalog.products", kind: "add-product", selectors: &[], index: "index", item: "product" },
    ],
    remove_item: &[

    ],
};

/// 🆔️ A collection whose remove kind addresses the row by its id instead of its position.
#[derive(Clone, Copy, Debug)]
pub struct IdRemoval {
    pub path: &'static str,
    pub kind: &'static str,
    pub id: &'static str,
}

/// 🆔️ Every collection of this family removed by row id.
pub const ID_REMOVALS: &[IdRemoval] = &[
        IdRemoval { path: "catalog.products", kind: "remove-product", id: "id" },
];

/// 🎯️ Resolves an editor edit into concrete kind mutations: row-id removals first, every other path through [`EDIT_RULES`].
pub fn resolve_edit(document: &Vdi3805Snapshot, edit: &NormEdit) -> Result<Vec<Vdi3805Mutation>, String> {
    if let NormEdit::RemoveItem { path, index } = edit {
        if let Some(rule) = ID_REMOVALS.iter().find(|rule| rule.path == path) {
            let tree = semio_framework_value::ToValue::to_value(document);
            let mut cursor = &tree;
            for segment in path.split('.') {
                let semio_framework_value::DslValue::Object(entries) = cursor else { return Err(format!("expected object before '{segment}' in path '{path}'")) };
                cursor = &entries.iter().find(|(key, _)| key == segment).ok_or_else(|| format!("missing field '{segment}' in path '{path}'"))?.1;
            }
            let semio_framework_value::DslValue::Array(items) = cursor else { return Err(format!("'{path}' is not a list")) };
            let semio_framework_value::DslValue::Object(row) = items.get(*index).ok_or_else(|| format!("'{path}' has no row {index}"))? else { return Err(format!("row {index} of '{path}' is not a record")) };
            let id = row.iter().find(|(key, _)| key == "id").map(|(_, value)| value.clone()).ok_or_else(|| format!("row {index} of '{path}' has no id"))?;
            return <Vdi3805Mutation as protocol::Mutation<Vdi3805Snapshot>>::from_payload_value(rule.kind, semio_framework_value::DslValue::object([(rule.id.to_string(), id)])).map(|mutation| vec![mutation]).map_err(|error| error.to_string());
        }
    }
    EDIT_RULES.resolve(document, edit)
}
