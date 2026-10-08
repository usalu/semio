//! 🧭️ The `setField` / `insertItem` / `removeItem` / `applyRemedy` vocabulary of the Iso16757 editor: which value-tree path raises which concrete kind.

use crate::app_surface::{InsertItemRule, NormEdit, NormEditRules, RemoveItemRule, SelectorKey, SetFieldRule};
use super::Iso16757Mutation;
use crate::Iso16757Snapshot;

/// 📚 Every path a Iso16757 editor edit resolves, one rule per kind that sets, inserts or removes through it.
pub const EDIT_RULES: NormEditRules = NormEditRules {
    set_field: &[
        SetFieldRule { path: "selection.classId", kind: "change-selection-class", selectors: &[], value: "newClassId" },
        SetFieldRule { path: "catalogue.manufacturer.names.preferred.text", kind: "rename-manufacturer", selectors: &[], value: "newName" },
        SetFieldRule { path: "catalogue.products[].names.preferred.text", kind: "rename-product", selectors: &[SelectorKey::Id("id")], value: "newName" },
        SetFieldRule { path: "catalogue.metadata.names.preferred.text", kind: "rename-catalogue", selectors: &[], value: "newName" },
        SetFieldRule { path: "exchangeProcess", kind: "change-exchange-process", selectors: &[], value: "newExchangeProcess" },
        SetFieldRule { path: "catalogue.productGroups[].names.preferred.text", kind: "rename-product-group", selectors: &[SelectorKey::Id("id")], value: "newName" },
        SetFieldRule { path: "partNumberRule", kind: "replace-part-number-rule", selectors: &[], value: "newRule" },
        SetFieldRule { path: "selection.seriesId", kind: "change-selection-series", selectors: &[], value: "newSeriesId" },
    ],
    insert_item: &[
        InsertItemRule { path: "dictionary.subjects", kind: "introduce-subject", selectors: &[], index: "index", item: "subject" },
        InsertItemRule { path: "catalogue.productClasses", kind: "introduce-product-class", selectors: &[], index: "index", item: "productClass" },
        InsertItemRule { path: "catalogue.propertyDefinitions", kind: "introduce-property-definition", selectors: &[], index: "index", item: "propertyDefinition" },
        InsertItemRule { path: "catalogue.productSeries", kind: "introduce-product-series", selectors: &[], index: "index", item: "productSeries" },
        InsertItemRule { path: "catalogue.products", kind: "introduce-product", selectors: &[], index: "index", item: "product" },
        InsertItemRule { path: "catalogue.productIndexes", kind: "introduce-product-index", selectors: &[], index: "index", item: "productIndex" },
        InsertItemRule { path: "selection.constraints", kind: "add-selection-constraint", selectors: &[], index: "index", item: "constraint" },
        InsertItemRule { path: "catalogue.productGroups", kind: "introduce-product-group", selectors: &[], index: "index", item: "productGroup" },
    ],
    remove_item: &[
        RemoveItemRule { path: "selection.constraints", kind: "remove-selection-constraint", selectors: &[], index: "index" },
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
        IdRemoval { path: "dictionary.subjects", kind: "retire-subject", id: "id" },
        IdRemoval { path: "catalogue.productClasses", kind: "retire-product-class", id: "id" },
        IdRemoval { path: "catalogue.productIndexes", kind: "retire-product-index", id: "id" },
        IdRemoval { path: "catalogue.productSeries", kind: "retire-product-series", id: "id" },
        IdRemoval { path: "catalogue.products", kind: "retire-product", id: "id" },
        IdRemoval { path: "catalogue.productGroups", kind: "retire-product-group", id: "id" },
        IdRemoval { path: "catalogue.propertyDefinitions", kind: "retire-property-definition", id: "id" },
];

/// 🎯️ Resolves an editor edit into concrete kind mutations: row-id removals first, every other path through [`EDIT_RULES`].
pub fn resolve_edit(document: &Iso16757Snapshot, edit: &NormEdit) -> Result<Vec<Iso16757Mutation>, String> {
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
            return <Iso16757Mutation as protocol::Mutation<Iso16757Snapshot>>::from_payload_value(rule.kind, semio_framework_value::DslValue::object([(rule.id.to_string(), id)])).map(|mutation| vec![mutation]).map_err(|error| error.to_string());
        }
    }
    EDIT_RULES.resolve(document, edit)
}
