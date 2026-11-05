//! 🧭️ The `setField` / `insertItem` / `removeItem` / `applyRemedy` vocabulary of the Vdi3805 editor: which value-tree path raises which concrete kind.

use semio_s_plugin_norm_app_surface::{IdRemoval, InsertItemRule, NormEdit, NormEditRules, SelectorKey, SetFieldRule};
use crate::Vdi3805Mutation;
use semio_s_artifact_norm_vdi3805_model::Vdi3805Snapshot;

/// 📚 Every path a Vdi3805 editor edit resolves, one rule per kind that sets, inserts or removes through it.
pub const EDIT_RULES: NormEditRules = NormEditRules {
    set_field_arguments: &[],
    move_item: &[],
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

/// 🆔️ Every collection of this family removed by row id.
pub const ID_REMOVALS: &[IdRemoval] = &[
        IdRemoval { path: "catalog.products", kind: "remove-product", selectors: &[], id: "id" },
];

/// 🎯️ Resolves an editor edit into concrete kind mutations: row-id removals first, every other path through [`EDIT_RULES`].
pub fn resolve_edit(document: &Vdi3805Snapshot, edit: &NormEdit) -> Result<Vec<Vdi3805Mutation>, String> {
    EDIT_RULES.resolve_with_id_removals(ID_REMOVALS, document, edit)
}
