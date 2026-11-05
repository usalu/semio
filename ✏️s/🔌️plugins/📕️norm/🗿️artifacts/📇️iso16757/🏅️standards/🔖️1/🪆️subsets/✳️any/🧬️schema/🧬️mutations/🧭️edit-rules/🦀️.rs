//! 🧭️ The `setField` / `insertItem` / `removeItem` / `applyRemedy` vocabulary of the Iso16757 editor: which value-tree path raises which concrete kind.

use semio_s_plugin_norm_app_surface::{IdRemoval, InsertItemRule, NormEdit, NormEditRules, RemoveItemRule, SelectorKey, SetFieldRule};
use crate::Iso16757Mutation;
use semio_s_artifact_norm_iso16757_model::Iso16757Snapshot;

/// 📚 Every path a Iso16757 editor edit resolves, one rule per kind that sets, inserts or removes through it.
pub const EDIT_RULES: NormEditRules = NormEditRules {
    set_field_arguments: &[],
    move_item: &[],
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

/// 🆔️ Every collection of this family removed by row id.
pub const ID_REMOVALS: &[IdRemoval] = &[
        IdRemoval { path: "dictionary.subjects", kind: "retire-subject", selectors: &[], id: "id" },
        IdRemoval { path: "catalogue.productClasses", kind: "retire-product-class", selectors: &[], id: "id" },
        IdRemoval { path: "catalogue.productIndexes", kind: "retire-product-index", selectors: &[], id: "id" },
        IdRemoval { path: "catalogue.productSeries", kind: "retire-product-series", selectors: &[], id: "id" },
        IdRemoval { path: "catalogue.products", kind: "retire-product", selectors: &[], id: "id" },
        IdRemoval { path: "catalogue.productGroups", kind: "retire-product-group", selectors: &[], id: "id" },
        IdRemoval { path: "catalogue.propertyDefinitions", kind: "retire-property-definition", selectors: &[], id: "id" },
];

/// 🎯️ Resolves an editor edit into concrete kind mutations: row-id removals first, every other path through [`EDIT_RULES`].
pub fn resolve_edit(document: &Iso16757Snapshot, edit: &NormEdit) -> Result<Vec<Iso16757Mutation>, String> {
    EDIT_RULES.resolve_with_id_removals(ID_REMOVALS, document, edit)
}
