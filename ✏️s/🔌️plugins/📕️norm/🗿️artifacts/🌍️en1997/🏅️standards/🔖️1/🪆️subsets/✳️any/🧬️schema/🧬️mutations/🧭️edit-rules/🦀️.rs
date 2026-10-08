//! 🧭️ The `setField` / `insertItem` / `removeItem` / `applyRemedy` vocabulary of the En1997 editor: which value-tree path raises which concrete kind.

use crate::app_surface::{InsertItemRule, NormEdit, NormEditRules, RemoveItemRule, SelectorKey, SetFieldRule};
use super::En1997Mutation;
use crate::En1997Snapshot;

/// 📚 Every path a En1997 editor edit resolves, one rule per kind that sets, inserts or removes through it.
pub const EDIT_RULES: NormEditRules = NormEditRules {
    set_field: &[
        SetFieldRule { path: "footings[].width", kind: "change-footing-width", selectors: &[SelectorKey::Id("id")], value: "newWidth" },
        SetFieldRule { path: "slopes[].angleDeg", kind: "change-slope-angle", selectors: &[SelectorKey::Id("id")], value: "newAngleDeg" },
        SetFieldRule { path: "footings[].embedment", kind: "change-footing-embedment", selectors: &[SelectorKey::Id("id")], value: "newEmbedment" },
        SetFieldRule { path: "layers[].oedometricModulus", kind: "change-layer-oedometric-modulus", selectors: &[SelectorKey::Id("id")], value: "newOedometricModulus" },
        SetFieldRule { path: "annex", kind: "change-annex", selectors: &[], value: "newAnnex" },
        SetFieldRule { path: "groundwaterLevel", kind: "change-groundwater-level", selectors: &[], value: "newGroundwaterLevel" },
        SetFieldRule { path: "designSituation", kind: "change-design-situation", selectors: &[], value: "newDesignSituation" },
        SetFieldRule { path: "piles[].length", kind: "change-pile-length", selectors: &[SelectorKey::Id("id")], value: "newLength" },
        SetFieldRule { path: "layers[].phiPrimeDeg", kind: "change-layer-phi-prime", selectors: &[SelectorKey::Id("id")], value: "newPhiPrimeDeg" },
        SetFieldRule { path: "investigationDepth", kind: "change-investigation-depth", selectors: &[], value: "newInvestigationDepth" },
        SetFieldRule { path: "piles[].count", kind: "change-pile-count", selectors: &[SelectorKey::Id("id")], value: "newCount" },
        SetFieldRule { path: "geotechnicalCategory", kind: "change-geotechnical-category", selectors: &[], value: "newGeotechnicalCategory" },
        SetFieldRule { path: "designApproach", kind: "change-design-approach", selectors: &[], value: "newDesignApproach" },
        SetFieldRule { path: "retainingWalls[].baseWidth", kind: "change-wall-base-width", selectors: &[SelectorKey::Id("id")], value: "newBaseWidth" },
    ],
    insert_item: &[
        InsertItemRule { path: "footings", kind: "insert-footing", selectors: &[], index: "index", item: "footing" },
        InsertItemRule { path: "layers", kind: "insert-layer", selectors: &[], index: "index", item: "layer" },
        InsertItemRule { path: "piles", kind: "insert-pile", selectors: &[], index: "index", item: "pile" },
    ],
    remove_item: &[
        RemoveItemRule { path: "footings", kind: "remove-footing", selectors: &[], index: "index" },
        RemoveItemRule { path: "layers", kind: "remove-layer", selectors: &[], index: "index" },
        RemoveItemRule { path: "piles", kind: "remove-pile", selectors: &[], index: "index" },
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

];

/// 🎯️ Resolves an editor edit into concrete kind mutations: row-id removals first, every other path through [`EDIT_RULES`].
pub fn resolve_edit(document: &En1997Snapshot, edit: &NormEdit) -> Result<Vec<En1997Mutation>, String> {
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
            return <En1997Mutation as protocol::Mutation<En1997Snapshot>>::from_payload_value(rule.kind, semio_framework_value::DslValue::object([(rule.id.to_string(), id)])).map(|mutation| vec![mutation]).map_err(|error| error.to_string());
        }
    }
    EDIT_RULES.resolve(document, edit)
}
