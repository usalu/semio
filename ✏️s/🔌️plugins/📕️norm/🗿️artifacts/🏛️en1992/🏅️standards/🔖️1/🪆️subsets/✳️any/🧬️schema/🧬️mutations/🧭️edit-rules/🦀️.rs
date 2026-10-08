//! 🧭️ The `setField` / `insertItem` / `removeItem` / `applyRemedy` vocabulary of the En1992 editor: which value-tree path raises which concrete kind.

use crate::app_surface::{InsertItemRule, NormEdit, NormEditRules, RemoveItemRule, SelectorKey, SetFieldRule};
use super::En1992Mutation;
use crate::En1992Snapshot;

/// 📚 Every path a En1992 editor edit resolves, one rule per kind that sets, inserts or removes through it.
pub const EDIT_RULES: NormEditRules = NormEditRules {
    set_field: &[
        SetFieldRule { path: "members[].longitudinal[].count", kind: "change-bar-layer-count", selectors: &[SelectorKey::Id("memberId"), SelectorKey::Id("layerId")], value: "newCount" },
        SetFieldRule { path: "members[].width", kind: "change-member-width", selectors: &[SelectorKey::Id("memberId")], value: "newValue" },
        SetFieldRule { path: "members[].height", kind: "change-member-height", selectors: &[SelectorKey::Id("memberId")], value: "newValue" },
        SetFieldRule { path: "members[].actions[].vK", kind: "change-action-vk", selectors: &[SelectorKey::Id("memberId"), SelectorKey::Id("actionId")], value: "newValue" },
        SetFieldRule { path: "members[].actions[].mK", kind: "change-action-mk", selectors: &[SelectorKey::Id("memberId"), SelectorKey::Id("actionId")], value: "newValue" },
        SetFieldRule { path: "members[].longitudinal[].diameter", kind: "change-bar-layer-diameter", selectors: &[SelectorKey::Id("memberId"), SelectorKey::Id("layerId")], value: "newDiameter" },
        SetFieldRule { path: "members[].span", kind: "change-member-span", selectors: &[SelectorKey::Id("memberId")], value: "newValue" },
        SetFieldRule { path: "annex", kind: "change-annex", selectors: &[], value: "newAnnex" },
        SetFieldRule { path: "members[].exposure", kind: "change-member-exposure", selectors: &[SelectorKey::Id("memberId")], value: "newExposure" },
        SetFieldRule { path: "members[].actions[].nK", kind: "change-action-nk", selectors: &[SelectorKey::Id("memberId"), SelectorKey::Id("actionId")], value: "newValue" },
        SetFieldRule { path: "title", kind: "change-title", selectors: &[], value: "newTitle" },
        SetFieldRule { path: "designWorkingLifeYears", kind: "change-design-working-life", selectors: &[], value: "newYears" },
        SetFieldRule { path: "anchors[].hEf", kind: "change-anchor-h-ef", selectors: &[SelectorKey::Id("anchorId")], value: "newValue" },
        SetFieldRule { path: "deltaCDev", kind: "change-delta-c-dev", selectors: &[], value: "newDeltaCDev" },
        SetFieldRule { path: "members[].effectiveDepth", kind: "change-member-effective-depth", selectors: &[SelectorKey::Id("memberId")], value: "newValue" },
        SetFieldRule { path: "members[].fire.axisDistance", kind: "change-member-axis-distance", selectors: &[SelectorKey::Id("memberId")], value: "newAxisDistance" },
        SetFieldRule { path: "members[].fire.rating", kind: "change-member-fire-rating", selectors: &[SelectorKey::Id("memberId")], value: "newRating" },
        SetFieldRule { path: "reinforcementGrades[].fYk", kind: "change-reinforcement-f-yk", selectors: &[SelectorKey::Id("gradeId")], value: "newFYk" },
        SetFieldRule { path: "members[].cover", kind: "change-member-cover", selectors: &[SelectorKey::Id("memberId")], value: "newValue" },
        SetFieldRule { path: "cementType", kind: "change-cement-type", selectors: &[], value: "newCementType" },
        SetFieldRule { path: "concreteGrades[].fCk", kind: "change-concrete-f-ck", selectors: &[SelectorKey::Id("gradeId")], value: "newFCk" },
        SetFieldRule { path: "anchors[].aS", kind: "change-anchor-as", selectors: &[SelectorKey::Id("anchorId")], value: "newValue" },
        SetFieldRule { path: "members[].stirrups.spacing", kind: "change-member-stirrup-spacing", selectors: &[SelectorKey::Id("memberId")], value: "newSpacing" },
    ],
    insert_item: &[
        InsertItemRule { path: "anchors", kind: "insert-anchor", selectors: &[], index: "index", item: "anchor" },
        InsertItemRule { path: "members", kind: "insert-member", selectors: &[], index: "index", item: "member" },
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
        IdRemoval { path: "members", kind: "remove-member", id: "memberId" },
        IdRemoval { path: "anchors", kind: "remove-anchor", id: "anchorId" },
];

/// 🎯️ Resolves an editor edit into concrete kind mutations: row-id removals first, every other path through [`EDIT_RULES`].
pub fn resolve_edit(document: &En1992Snapshot, edit: &NormEdit) -> Result<Vec<En1992Mutation>, String> {
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
            return <En1992Mutation as protocol::Mutation<En1992Snapshot>>::from_payload_value(rule.kind, semio_framework_value::DslValue::object([(rule.id.to_string(), id)])).map(|mutation| vec![mutation]).map_err(|error| error.to_string());
        }
    }
    EDIT_RULES.resolve(document, edit)
}
