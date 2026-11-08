//! 🧭️ The `setField` / `insertItem` / `removeItem` / `applyRemedy` vocabulary of the Din18599 editor: which value-tree path raises which concrete kind.

use semio_framework_value::{FromValue, ToValue};
use semio_s_plugin_norm_app_surface::{parse_path, IdRemoval, InsertItemRule, NormEdit, NormEditRules, PathSegment, SelectorKey, SetFieldRule};
use crate::Din18599Mutation;
use semio_s_artifact_norm_din18599_model::{Din18599Snapshot, EnvelopeElement, ThermalZone};

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
        SetFieldRule { path: "dhw", kind: "specify-dhw-system", selectors: &[], value: "newDhw" },
        SetFieldRule { path: "method", kind: "change-method", selectors: &[], value: "newMethod" },
        SetFieldRule { path: "attachment", kind: "change-attachment", selectors: &[], value: "newAttachment" },
        SetFieldRule { path: "climate", kind: "update-climate", selectors: &[], value: "newClimate" },
        SetFieldRule { path: "zones[].labelEn", kind: "change-zone-label-en", selectors: &[SelectorKey::Id("zoneId")], value: "newLabelEn" },
        SetFieldRule { path: "zones[].labelDe", kind: "change-zone-label-de", selectors: &[SelectorKey::Id("zoneId")], value: "newLabelDe" },
        SetFieldRule { path: "zones[].usageProfile", kind: "change-zone-usage-profile", selectors: &[SelectorKey::Id("zoneId")], value: "newUsageProfile" },
        SetFieldRule { path: "zones[].occupants", kind: "change-zone-occupants", selectors: &[SelectorKey::Id("zoneId")], value: "newOccupants" },
        SetFieldRule { path: "zones[].areaM2", kind: "change-zone-area-m2", selectors: &[SelectorKey::Id("zoneId")], value: "newAreaM2" },
        SetFieldRule { path: "zones[].volumeM3", kind: "change-zone-volume-m3", selectors: &[SelectorKey::Id("zoneId")], value: "newVolumeM3" },
        SetFieldRule { path: "zones[].thetaIHeatC", kind: "change-zone-theta-i-heat-c", selectors: &[SelectorKey::Id("zoneId")], value: "newThetaIHeatC" },
        SetFieldRule { path: "zones[].thetaICoolC", kind: "change-zone-theta-i-cool-c", selectors: &[SelectorKey::Id("zoneId")], value: "newThetaICoolC" },
        SetFieldRule { path: "zones[].internalGainsWM2", kind: "change-zone-internal-gains-w-m2", selectors: &[SelectorKey::Id("zoneId")], value: "newInternalGainsWM2" },
        SetFieldRule { path: "zones[].lightingPowerWM2", kind: "change-zone-lighting-power-w-m2", selectors: &[SelectorKey::Id("zoneId")], value: "newLightingPowerWM2" },
        SetFieldRule { path: "elements[].labelEn", kind: "change-element-label-en", selectors: &[SelectorKey::Id("elementId")], value: "newLabelEn" },
        SetFieldRule { path: "elements[].labelDe", kind: "change-element-label-de", selectors: &[SelectorKey::Id("elementId")], value: "newLabelDe" },
        SetFieldRule { path: "elements[].kind", kind: "change-element-kind", selectors: &[SelectorKey::Id("elementId")], value: "newKind" },
        SetFieldRule { path: "elements[].zoneId", kind: "change-element-zone-id", selectors: &[SelectorKey::Id("elementId")], value: "newZoneId" },
        SetFieldRule { path: "elements[].areaM2", kind: "change-element-area-m2", selectors: &[SelectorKey::Id("elementId")], value: "newAreaM2" },
        SetFieldRule { path: "elements[].orientationDeg", kind: "change-element-orientation-deg", selectors: &[SelectorKey::Id("elementId")], value: "newOrientationDeg" },
        SetFieldRule { path: "elements[].tiltDeg", kind: "change-element-tilt-deg", selectors: &[SelectorKey::Id("elementId")], value: "newTiltDeg" },
        SetFieldRule { path: "elements[].gValue", kind: "change-element-g-value", selectors: &[SelectorKey::Id("elementId")], value: "newGValue" },
        SetFieldRule { path: "elements[].fc", kind: "change-element-fc", selectors: &[SelectorKey::Id("elementId")], value: "newFc" },
        SetFieldRule { path: "elements[].adjacency", kind: "change-element-adjacency", selectors: &[SelectorKey::Id("elementId")], value: "newAdjacency" },
    ],
    insert_item: &[
        InsertItemRule { path: "zones", kind: "insert-zone", selectors: &[], index: "index", item: "zone" },
        InsertItemRule { path: "elements", kind: "insert-element", selectors: &[], index: "index", item: "element" },
    ],
    remove_item: &[],
};

/// 🆔️ Every collection of this family removed by row id.
pub const ID_REMOVALS: &[IdRemoval] = &[
    IdRemoval { path: "zones", kind: "remove-zone", selectors: &[], id: "id" },
    IdRemoval { path: "elements", kind: "remove-element", selectors: &[], id: "id" },
];

fn fresh_id(prefix: &str, taken: &[&str]) -> String {
    (1..).map(|n| format!("{prefix}-{n}")).find(|id| !taken.contains(&id.as_str())).unwrap_or_default()
}

fn new_zone(document: &Din18599Snapshot, given: Option<&semio_framework_value::DslValue>) -> Result<ThermalZone, String> {
    let mut zone = match given {
        Some(value) => ThermalZone::from_value(value.clone()).map_err(|error| format!("{error:?}"))?,
        None => document.zones.last().cloned().ok_or("the document has no zone to copy")?,
    };
    if zone.id.is_empty() || document.zones.iter().any(|z| z.id == zone.id) {
        zone.id = fresh_id("zone", &document.zones.iter().map(|z| z.id.as_str()).collect::<Vec<_>>());
        zone.label_en = format!("Zone {}", zone.id);
        zone.label_de = format!("Zone {}", zone.id);
    }
    Ok(zone)
}

fn new_element(document: &Din18599Snapshot, given: Option<&semio_framework_value::DslValue>) -> Result<EnvelopeElement, String> {
    let mut element = match given {
        Some(value) => EnvelopeElement::from_value(value.clone()).map_err(|error| format!("{error:?}"))?,
        None => document.elements.last().cloned().ok_or("the document has no envelope element to copy")?,
    };
    if element.id.is_empty() || document.elements.iter().any(|e| e.id == element.id) {
        element.id = fresh_id("element", &document.elements.iter().map(|e| e.id.as_str()).collect::<Vec<_>>());
        element.label_en = format!("Element {}", element.id);
        element.label_de = format!("Bauteil {}", element.id);
    }
    if !document.zones.iter().any(|z| z.id == element.zone_id) {
        element.zone_id = document.zones.first().map(|z| z.id.clone()).ok_or("the document has no zone to attach the element to")?;
    }
    Ok(element)
}

fn selected_id(segments: &[PathSegment], ids: &[&str], index: usize) -> Result<String, String> {
    match segments.last() {
        Some(PathSegment::Id(id)) => Ok(id.clone()),
        Some(PathSegment::Index(at)) => ids.get(*at).map(|id| (*id).to_string()).ok_or_else(|| format!("no row at {at}")),
        _ => ids.get(index).map(|id| (*id).to_string()).ok_or_else(|| format!("no row at {index}")),
    }
}

fn removal(document: &Din18599Snapshot, collection: &str, id: &str) -> Result<Vec<Din18599Mutation>, String> {
    EDIT_RULES.resolve_with_id_removals(ID_REMOVALS, document, &NormEdit::RemoveItem { path: format!("{collection}[id={id}]"), index: 0 })
}

/// ➕➖ Row edits that need more than the declared rule: an insert without a row copies the last row under a fresh id (the clamp keeps the position inside the list), and removing a zone first removes the envelope elements standing on it, as separate events of the same transaction.
fn collection_edit(document: &Din18599Snapshot, edit: &NormEdit) -> Option<Result<Vec<Din18599Mutation>, String>> {
    let path = match edit {
        NormEdit::InsertItem { path, .. } | NormEdit::RemoveItem { path, .. } => path,
        _ => return None,
    };
    let segments = parse_path(path).ok()?;
    let Some(PathSegment::Field(collection)) = segments.first() else { return None };
    match (edit, collection.as_str(), segments.len()) {
        (NormEdit::InsertItem { index, value, .. }, "zones", 1) => Some(new_zone(document, value.as_ref()).and_then(|zone| EDIT_RULES.resolve_with_id_removals(ID_REMOVALS, document, &NormEdit::InsertItem { path: "zones".into(), index: (*index).min(document.zones.len()), value: Some(zone.to_value()) }))),
        (NormEdit::InsertItem { index, value, .. }, "elements", 1) => Some(new_element(document, value.as_ref()).and_then(|element| EDIT_RULES.resolve_with_id_removals(ID_REMOVALS, document, &NormEdit::InsertItem { path: "elements".into(), index: (*index).min(document.elements.len()), value: Some(element.to_value()) }))),
        (NormEdit::RemoveItem { index, .. }, "zones", 1 | 2) => Some(selected_id(&segments, &document.zones.iter().map(|z| z.id.as_str()).collect::<Vec<_>>(), *index).and_then(|zone| {
            let mut out = Vec::new();
            for element in document.elements.iter().filter(|e| e.zone_id == zone) {
                out.extend(removal(document, "elements", &element.id)?);
            }
            out.extend(removal(document, "zones", &zone)?);
            Ok(out)
        })),
        _ => None,
    }
}

/// 🎯️ Resolves an editor edit into concrete kind mutations: row edits through [`collection_edit`], every other path through [`EDIT_RULES`].
pub fn resolve_edit(document: &Din18599Snapshot, edit: &NormEdit) -> Result<Vec<Din18599Mutation>, String> {
    collection_edit(document, edit).unwrap_or_else(|| EDIT_RULES.resolve_with_id_removals(ID_REMOVALS, document, edit))
}
