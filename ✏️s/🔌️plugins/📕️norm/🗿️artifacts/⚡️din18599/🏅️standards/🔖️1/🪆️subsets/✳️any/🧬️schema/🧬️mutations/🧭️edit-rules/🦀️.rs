//! 🧭️ The `setField` / `insertItem` / `removeItem` / `applyRemedy` vocabulary of the Din18599 editor: which value-tree path raises which concrete kind.

use crate::app_surface::{InsertItemRule, NormEdit, NormEditRules, RemoveItemRule, SelectorKey, SetFieldRule};
use super::Din18599Mutation;
use crate::Din18599Snapshot;

/// 📚 Every path a Din18599 editor edit resolves, one rule per kind that sets, inserts or removes through it.
pub const EDIT_RULES: NormEditRules = NormEditRules {
    set_field: &[
        SetFieldRule { path: "renewables", kind: "update-renewables", selectors: &[], value: "newRenewables" },
        SetFieldRule { path: "gegQpFactor", kind: "change-geg-qp-factor", selectors: &[], value: "newGegQpFactor" },
        SetFieldRule { path: "cooling", kind: "update-cooling", selectors: &[], value: "newCooling" },
        SetFieldRule { path: "deltaUWbWM2k", kind: "change-delta-u-wb", selectors: &[], value: "newDeltaUWbWM2k" },
        SetFieldRule { path: "elements[].uValueWM2k", kind: "change-element-u", selectors: &[SelectorKey::Id("elementId")], value: "newUValueWM2k" },
        SetFieldRule { path: "ventilation", kind: "update-ventilation", selectors: &[], value: "newVentilation" },
        SetFieldRule { path: "automationClass", kind: "change-automation-class", selectors: &[], value: "newAutomationClass" },
        SetFieldRule { path: "buildingCategory", kind: "change-building-category", selectors: &[], value: "newBuildingCategory" },
        SetFieldRule { path: "useClass", kind: "change-use-class", selectors: &[], value: "newUseClass" },
        SetFieldRule { path: "lighting", kind: "update-lighting", selectors: &[], value: "newLighting" },
        SetFieldRule { path: "netFloorAreaM2", kind: "change-net-floor-area-m2", selectors: &[], value: "newNetFloorAreaM2" },
        SetFieldRule { path: "heatedVolumeM3", kind: "change-heated-volume-m3", selectors: &[], value: "newHeatedVolumeM3" },
        SetFieldRule { path: "heating", kind: "specify-heating-system", selectors: &[], value: "newHeating" },
        SetFieldRule { path: "zones", kind: "replace-zones", selectors: &[], value: "newZones" },
        SetFieldRule { path: "dhw", kind: "specify-dhw-system", selectors: &[], value: "newDhw" },
        SetFieldRule { path: "elements", kind: "replace-elements", selectors: &[], value: "newElements" },
        SetFieldRule { path: "method", kind: "change-method", selectors: &[], value: "newMethod" },
        SetFieldRule { path: "attachment", kind: "change-attachment", selectors: &[], value: "newAttachment" },
        SetFieldRule { path: "climate", kind: "update-climate", selectors: &[], value: "newClimate" },
    ],
    insert_item: &[

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

];

/// 🎯️ Resolves an editor edit into concrete kind mutations: row-id removals first, every other path through [`EDIT_RULES`].
pub fn resolve_edit(document: &Din18599Snapshot, edit: &NormEdit) -> Result<Vec<Din18599Mutation>, String> {
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
            return <Din18599Mutation as protocol::Mutation<Din18599Snapshot>>::from_payload_value(rule.kind, semio_framework_value::DslValue::object([(rule.id.to_string(), id)])).map(|mutation| vec![mutation]).map_err(|error| error.to_string());
        }
    }
    EDIT_RULES.resolve(document, edit)
}
