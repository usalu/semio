//! 🧭️ The `setField` / `insertItem` / `removeItem` / `applyRemedy` vocabulary of the Din18599 editor: which value-tree path raises which concrete kind.

use semio_s_plugin_norm_app_surface::{IdRemoval, NormEdit, NormEditRules, SelectorKey, SetFieldRule};
use crate::Din18599Mutation;
use semio_s_artifact_norm_din18599_model::Din18599Snapshot;

/// 📚 Every path a Din18599 editor edit resolves, one rule per kind that sets, inserts or removes through it.
pub const EDIT_RULES: NormEditRules = NormEditRules {
    set_field_arguments: &[],
    move_item: &[],
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

/// 🆔️ Every collection of this family removed by row id.
pub const ID_REMOVALS: &[IdRemoval] = &[

];

/// 🎯️ Resolves an editor edit into concrete kind mutations: row-id removals first, every other path through [`EDIT_RULES`].
pub fn resolve_edit(document: &Din18599Snapshot, edit: &NormEdit) -> Result<Vec<Din18599Mutation>, String> {
    EDIT_RULES.resolve_with_id_removals(ID_REMOVALS, document, edit)
}
