//! 🧭️ The `setField` / `insertItem` / `removeItem` / `applyRemedy` vocabulary of the En1999 editor: which value-tree path raises which concrete kind.

use crate::app_surface::{InsertItemRule, NormEdit, NormEditRules, RemoveItemRule, SelectorKey, SetFieldRule};
use super::En1999Mutation;
use crate::En1999Snapshot;

/// 📚 Every path a En1999 editor edit resolves, one rule per kind that sets, inserts or removes through it.
pub const EDIT_RULES: NormEditRules = NormEditRules {
    set_field: &[
        SetFieldRule { path: "materials[].designation", kind: "change-material-designation", selectors: &[SelectorKey::Id("materialId")], value: "newDesignation" },
        SetFieldRule { path: "coldFormed", kind: "change-cold-formed", selectors: &[], value: "coldFormed" },
        SetFieldRule { path: "members[].actions[].mYK", kind: "change-member-my-ed", selectors: &[SelectorKey::Id("memberId"), SelectorKey::Id("actionId")], value: "newMYK" },
        SetFieldRule { path: "annex", kind: "change-annex", selectors: &[], value: "newAnnex" },
        SetFieldRule { path: "members[].actions[].nK", kind: "change-member-n-ed", selectors: &[SelectorKey::Id("memberId"), SelectorKey::Id("actionId")], value: "newNK" },
        SetFieldRule { path: "members", kind: "change-members", selectors: &[], value: "members" },
        SetFieldRule { path: "members[].bucklingLengthZ", kind: "change-member-buckling-length", selectors: &[SelectorKey::Id("memberId")], value: "newLength" },
        SetFieldRule { path: "sections", kind: "change-sections", selectors: &[], value: "sections" },
        SetFieldRule { path: "fatigueDetails", kind: "change-fatigue-details", selectors: &[], value: "fatigueDetails" },
        SetFieldRule { path: "connections", kind: "change-connections", selectors: &[], value: "connections" },
        SetFieldRule { path: "fireScenarios", kind: "change-fire-scenarios", selectors: &[], value: "fireScenarios" },
        SetFieldRule { path: "connections[].welds.throat", kind: "change-weld-throat", selectors: &[SelectorKey::Id("connectionId")], value: "newThroat" },
        SetFieldRule { path: "connections[].bolts.rows", kind: "change-bolt-count", selectors: &[SelectorKey::Id("connectionId")], value: "newRows" },
        SetFieldRule { path: "materials", kind: "change-materials", selectors: &[], value: "materials" },
        SetFieldRule { path: "sections[].elements[].thickness", kind: "change-plate-thickness", selectors: &[SelectorKey::Id("sectionId"), SelectorKey::Id("elementId")], value: "newThickness" },
        SetFieldRule { path: "shells", kind: "change-shells", selectors: &[], value: "shells" },
    ],
    insert_item: &[
        InsertItemRule { path: "members", kind: "add-member", selectors: &[], index: "index", item: "member" },
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
        IdRemoval { path: "members", kind: "remove-member", id: "id" },
];

/// 🎯️ Resolves an editor edit into concrete kind mutations: row-id removals first, every other path through [`EDIT_RULES`].
pub fn resolve_edit(document: &En1999Snapshot, edit: &NormEdit) -> Result<Vec<En1999Mutation>, String> {
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
            return <En1999Mutation as protocol::Mutation<En1999Snapshot>>::from_payload_value(rule.kind, semio_framework_value::DslValue::object([(rule.id.to_string(), id)])).map(|mutation| vec![mutation]).map_err(|error| error.to_string());
        }
    }
    EDIT_RULES.resolve(document, edit)
}
