//! 🧭️ The `setField` / `insertItem` / `removeItem` / `applyRemedy` vocabulary of the DIN EN 16798 editor: which value-tree path raises which concrete kind.

use crate::app_surface::{InsertItemRule, NormEdit, NormEditRules, RemoveItemRule, SelectorKey, SetFieldRule};
use crate::mutations::{remove_vent_system::RemoveVentSystem, remove_zone::RemoveZone};
use crate::{Din16798Mutation, Din16798Snapshot};

/// 📚 Every path an DIN EN 16798 editor edit resolves, one rule per kind that sets, inserts or removes through it.
pub const EDIT_RULES: NormEditRules = NormEditRules {
    set_field: &[
        SetFieldRule { path: "zones[].tOpSummerC", kind: "change-zone-t-op-summer", selectors: &[SelectorKey::Id("zoneId")], value: "newTOpSummerC" },
        SetFieldRule { path: "ventSystems[].heatRecoveryEta", kind: "change-vent-heat-recovery", selectors: &[SelectorKey::Id("ventId")], value: "newHeatRecoveryEta" },
        SetFieldRule { path: "ventSystems[].systemType", kind: "change-vent-system-type", selectors: &[SelectorKey::Id("ventId")], value: "newSystemType" },
        SetFieldRule { path: "zones[].tOpWinterC", kind: "change-zone-t-op-winter", selectors: &[SelectorKey::Id("zoneId")], value: "newTOpWinterC" },
        SetFieldRule { path: "cellarVentilationM3H", kind: "change-cellar-ventilation", selectors: &[], value: "newCellarVentilationM3H" },
        SetFieldRule { path: "ventSystems[].sfpWM3S", kind: "change-vent-sfp", selectors: &[SelectorKey::Id("ventId")], value: "newSfpWM3S" },
        SetFieldRule { path: "annex", kind: "change-annex", selectors: &[], value: "newAnnex" },
        SetFieldRule { path: "nightSetbackK", kind: "change-night-setback", selectors: &[], value: "newNightSetbackK" },
        SetFieldRule { path: "outdoorCo2Ppm", kind: "change-outdoor-co2", selectors: &[], value: "newOutdoorCo2Ppm" },
        SetFieldRule { path: "ventSystems[].designAirflowM3H", kind: "change-vent-design-airflow", selectors: &[SelectorKey::Id("ventId")], value: "newDesignAirflowM3H" },
        SetFieldRule { path: "ventSystems[].sfpRequiredClass", kind: "change-vent-sfp-class", selectors: &[SelectorKey::Id("ventId")], value: "newSfpRequiredClass" },
        SetFieldRule { path: "zones[].metabolicRateMet", kind: "change-zone-metabolic-rate", selectors: &[SelectorKey::Id("zoneId")], value: "newMetabolicRateMet" },
        SetFieldRule { path: "cellarAreaM2", kind: "change-cellar-area", selectors: &[], value: "newCellarAreaM2" },
        SetFieldRule { path: "ventSystems[].odaClass", kind: "change-vent-oda-class", selectors: &[SelectorKey::Id("ventId")], value: "newOdaClass" },
        SetFieldRule { path: "envelopeN50HInv", kind: "change-envelope-n50", selectors: &[], value: "newEnvelopeN50HInv" },
        SetFieldRule { path: "zones[].usageType", kind: "change-zone-usage-type", selectors: &[SelectorKey::Id("zoneId")], value: "newUsageType" },
        SetFieldRule { path: "zones[].pollutionClass", kind: "change-zone-pollution-class", selectors: &[SelectorKey::Id("zoneId")], value: "newPollutionClass" },
        SetFieldRule { path: "zones[].clothingClo", kind: "change-zone-clothing", selectors: &[SelectorKey::Id("zoneId")], value: "newClothingClo" },
        SetFieldRule { path: "zones[].occupants", kind: "change-zone-occupants", selectors: &[SelectorKey::Id("zoneId")], value: "newOccupants" },
        SetFieldRule { path: "zones[].illuminanceLx", kind: "change-zone-illuminance", selectors: &[SelectorKey::Id("zoneId")], value: "newIlluminanceLx" },
        SetFieldRule { path: "zones[].rhPercent", kind: "change-zone-rh", selectors: &[SelectorKey::Id("zoneId")], value: "newRhPercent" },
        SetFieldRule { path: "zones[].airSpeedMS", kind: "change-zone-air-speed", selectors: &[SelectorKey::Id("zoneId")], value: "newAirSpeedMS" },
        SetFieldRule { path: "zones[].turbulenceIntensityPercent", kind: "change-zone-turbulence", selectors: &[SelectorKey::Id("zoneId")], value: "newTurbulenceIntensityPercent" },
        SetFieldRule { path: "zones[].outdoorAirSuppliedM3H", kind: "change-zone-outdoor-air", selectors: &[SelectorKey::Id("zoneId")], value: "newOutdoorAirSuppliedM3H" },
        SetFieldRule { path: "ventSystems[].yearsSinceInspection", kind: "change-vent-inspection", selectors: &[SelectorKey::Id("ventId")], value: "newYearsSinceInspection" },
        SetFieldRule { path: "zones[].floorAreaM2", kind: "change-zone-floor-area", selectors: &[SelectorKey::Id("zoneId")], value: "newFloorAreaM2" },
        SetFieldRule { path: "zones[].ventMethod", kind: "change-zone-vent-method", selectors: &[SelectorKey::Id("zoneId")], value: "newVentMethod" },
        SetFieldRule { path: "envelopeVolumeM3", kind: "change-envelope-volume", selectors: &[], value: "newEnvelopeVolumeM3" },
        SetFieldRule { path: "thetaRmC", kind: "change-theta-rm", selectors: &[], value: "newThetaRmC" },
        SetFieldRule { path: "zones[].noiseDb", kind: "change-zone-noise", selectors: &[SelectorKey::Id("zoneId")], value: "newNoiseDb" },
        SetFieldRule { path: "zones[].ventSystemId", kind: "change-zone-vent-system-id", selectors: &[SelectorKey::Id("zoneId")], value: "newVentSystemId" },
        SetFieldRule { path: "ventSystems[].ductLeakageM3SM2", kind: "change-vent-duct-leakage", selectors: &[SelectorKey::Id("ventId")], value: "newDuctLeakageM3SM2" },
        SetFieldRule { path: "zones[].comfortCategory", kind: "change-zone-comfort-category", selectors: &[SelectorKey::Id("zoneId")], value: "newComfortCategory" },
        SetFieldRule { path: "zones[].comfortModel", kind: "change-zone-comfort-model", selectors: &[SelectorKey::Id("zoneId")], value: "newComfortModel" },
        SetFieldRule { path: "ventSystems[].ductClass", kind: "change-vent-duct-class", selectors: &[SelectorKey::Id("ventId")], value: "newDuctClass" },
        SetFieldRule { path: "ventSystems[].filterSupClass", kind: "change-vent-filter-sup", selectors: &[SelectorKey::Id("ventId")], value: "newFilterSupClass" },
        SetFieldRule { path: "zones[].co2Ppm", kind: "change-zone-co2", selectors: &[SelectorKey::Id("zoneId")], value: "newCo2Ppm" },
    ],
    insert_item: &[
        InsertItemRule { path: "zones", kind: "insert-zone", selectors: &[], index: "index", item: "zone" },
        InsertItemRule { path: "ventSystems", kind: "insert-vent-system", selectors: &[], index: "index", item: "vent" },
    ],
    remove_item: &[

    ],
};

fn row_position(path: &str, collection: &str, index: usize, ids: &[&str]) -> Result<Option<String>, String> {
    let Some(selector) = path.strip_prefix(collection) else { return Ok(None) };
    let position = match selector.strip_prefix('[').and_then(|rest| rest.strip_suffix(']')) {
        None if selector.is_empty() => index,
        Some(inner) => match inner.strip_prefix("id=") {
            Some(id) => ids.iter().position(|existing| *existing == id).ok_or_else(|| format!("no row with id '{id}' in {collection}"))?,
            None => inner.parse().map_err(|_| format!("'{inner}' is not a row position in {path}"))?,
        },
        None => return Ok(None),
    };
    ids.get(position).map(|id| Some((*id).to_string())).ok_or_else(|| format!("{collection} has no row at {position}"))
}

/// 🧭️ Resolves one value-tree edit into concrete kinds: the vocabulary of [`EDIT_RULES`], plus removal of a zone or vent system, which addresses its row by id — a spelling a rule's index selector cannot express.
pub fn resolve_edit(document: &Din16798Snapshot, edit: &NormEdit) -> Result<Vec<Din16798Mutation>, String> {
    if let NormEdit::RemoveItem { path, index } = edit {
        let zones: Vec<&str> = document.zones.iter().map(|zone| zone.id.as_str()).collect();
        if let Some(zone_id) = row_position(path, "zones", *index, &zones)? {
            return Ok(vec![Din16798Mutation::RemoveZone(RemoveZone { zone_id })]);
        }
        let vents: Vec<&str> = document.vent_systems.iter().map(|vent| vent.id.as_str()).collect();
        if let Some(vent_id) = row_position(path, "ventSystems", *index, &vents)? {
            return Ok(vec![Din16798Mutation::RemoveVentSystem(RemoveVentSystem { vent_id })]);
        }
    }
    EDIT_RULES.resolve(document, edit)
}
