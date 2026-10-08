//! 🧭️ The `setField` / `insertItem` / `removeItem` / `applyRemedy` vocabulary of the En1994 editor: which value-tree path raises which concrete kind.

use crate::app_surface::{InsertItemRule, NormEdit, NormEditRules, RemoveItemRule, SelectorKey, SetFieldRule};
use super::En1994Mutation;
use crate::En1994Snapshot;

/// 📚 Every path a En1994 editor edit resolves, one rule per kind that sets, inserts or removes through it.
pub const EDIT_RULES: NormEditRules = NormEditRules {
    set_field: &[
        SetFieldRule { path: "beams[].studs.totalCount", kind: "change-beam-stud-count", selectors: &[SelectorKey::Index("index")], value: "newTotalCount" },
        SetFieldRule { path: "beams[].transverseAsM2PerM", kind: "change-beam-transverse-as", selectors: &[SelectorKey::Index("index")], value: "newTransverseAsM2PerM" },
        SetFieldRule { path: "columns[].kind", kind: "change-column-kind", selectors: &[SelectorKey::Index("index")], value: "newKind" },
        SetFieldRule { path: "beams[].studs.spacingM", kind: "change-beam-stud-spacing-m", selectors: &[SelectorKey::Index("index")], value: "newSpacingM" },
        SetFieldRule { path: "columns[].actions[].nKN", kind: "change-column-action-force-n", selectors: &[SelectorKey::Index("index"), SelectorKey::Index("actionIndex")], value: "newNKN" },
        SetFieldRule { path: "beams[].studs.diameterM", kind: "change-beam-stud-diameter-m", selectors: &[SelectorKey::Index("index")], value: "newDiameterM" },
        SetFieldRule { path: "beams[].actions[].qAreaPa", kind: "change-beam-action-q-area-pa", selectors: &[SelectorKey::Index("index"), SelectorKey::Index("actionIndex")], value: "newQAreaPa" },
        SetFieldRule { path: "annex", kind: "change-annex", selectors: &[], value: "newAnnex" },
        SetFieldRule { path: "steelFYPa", kind: "change-steel-fy-pa", selectors: &[], value: "newSteelFYPa" },
        SetFieldRule { path: "structureKind", kind: "change-structure-kind", selectors: &[], value: "newStructureKind" },
        SetFieldRule { path: "beams[].studs.fUPa", kind: "change-beam-stud-fu-pa", selectors: &[SelectorKey::Index("index")], value: "newFUPa" },
        SetFieldRule { path: "slabs[].concreteThicknessM", kind: "change-slab-thickness-m", selectors: &[SelectorKey::Index("index")], value: "newConcreteThicknessM" },
        SetFieldRule { path: "beams[].spanM", kind: "change-beam-span-m", selectors: &[SelectorKey::Index("index")], value: "newSpanM" },
        SetFieldRule { path: "slabs[].actions[].qAreaPa", kind: "change-slab-action-q-area-pa", selectors: &[SelectorKey::Index("index"), SelectorKey::Index("actionIndex")], value: "newQAreaPa" },
        SetFieldRule { path: "fatigueDetail", kind: "change-fatigue-detail", selectors: &[], value: "newFatigueDetail" },
        SetFieldRule { path: "fireRating", kind: "change-fire-rating", selectors: &[], value: "newFireRating" },
        SetFieldRule { path: "beams[].construction", kind: "change-beam-construction", selectors: &[SelectorKey::Index("index")], value: "newConstruction" },
        SetFieldRule { path: "insulationThicknessM", kind: "change-insulation-thickness-m", selectors: &[], value: "newInsulationThicknessM" },
        SetFieldRule { path: "beams[].slabThicknessM", kind: "change-beam-slab-thickness-m", selectors: &[SelectorKey::Index("index")], value: "newSlabThicknessM" },
    ],
    insert_item: &[
        InsertItemRule { path: "slabs", kind: "insert-slab", selectors: &[], index: "index", item: "slab" },
        InsertItemRule { path: "beams", kind: "insert-beam", selectors: &[], index: "index", item: "beam" },
        InsertItemRule { path: "columns", kind: "insert-column", selectors: &[], index: "index", item: "column" },
    ],
    remove_item: &[
        RemoveItemRule { path: "columns", kind: "remove-column", selectors: &[], index: "index" },
        RemoveItemRule { path: "slabs", kind: "remove-slab", selectors: &[], index: "index" },
        RemoveItemRule { path: "beams", kind: "remove-beam", selectors: &[], index: "index" },
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
pub fn resolve_edit(document: &En1994Snapshot, edit: &NormEdit) -> Result<Vec<En1994Mutation>, String> {
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
            return <En1994Mutation as protocol::Mutation<En1994Snapshot>>::from_payload_value(rule.kind, semio_framework_value::DslValue::object([(rule.id.to_string(), id)])).map(|mutation| vec![mutation]).map_err(|error| error.to_string());
        }
    }
    EDIT_RULES.resolve(document, edit)
}
