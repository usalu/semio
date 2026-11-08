//! 🏷️ DIN V 18599 FieldSpec lookup — SI display units + en/de labels (longest-prefix + `[]` wildcards).

use semio_s_plugin_norm_app_surface::{lookup_norm_field_meta, NormFieldChoice, FieldSpec, UnitId};

const fn choice(value: &'static str, label_en: &'static str, label_de: &'static str) -> NormFieldChoice {
    NormFieldChoice { value, label_en, label_de }
}

const BUILDING_CATEGORY: &[NormFieldChoice] = &[
    choice("Residential", "Residential building", "Wohngebäude"),
    choice("NonResidential", "Non-residential building", "Nichtwohngebäude"),
];
const ATTACHMENT: &[NormFieldChoice] = &[
    choice("Detached", "Detached", "Freistehend"),
    choice("SemiDetached", "Semi-detached", "Doppelhaushälfte"),
    choice("EndTerrace", "End of terrace", "Reihenendhaus"),
    choice("MidTerrace", "Mid-terrace", "Reihenmittelhaus"),
];
const USE_CLASS: &[NormFieldChoice] = &[
    choice("Residential", "Residential", "Wohnen"),
    choice("Office", "Office", "Büro"),
    choice("School", "School", "Schule"),
];
const METHOD: &[NormFieldChoice] = &[
    choice("DetailedMonthly", "Detailed monthly balance", "Detailliertes Monatsbilanzverfahren"),
    choice("Tabular", "Tabular method", "Tabellenverfahren"),
];
const AUTOMATION: &[NormFieldChoice] = &[
    choice("A", "A — high energy performance", "A — hohe Energieeffizienz"),
    choice("B", "B — advanced", "B — weiterentwickelt"),
    choice("C", "C — standard", "C — Standard"),
    choice("D", "D — non energy efficient", "D — nicht energieeffizient"),
];
const ELEMENT_KIND: &[NormFieldChoice] = &[
    choice("Wall", "Wall", "Wand"),
    choice("Roof", "Roof", "Dach"),
    choice("Floor", "Floor", "Boden"),
    choice("Door", "Door", "Tür"),
    choice("Window", "Window", "Fenster"),
];
const ADJACENCY: &[NormFieldChoice] = &[
    choice("Outdoor", "Outdoor air", "Außenluft"),
    choice("Ground", "Ground", "Erdreich"),
    choice("Unheated", "Unheated space", "Unbeheizter Raum"),
    choice("Heated", "Heated space", "Beheizter Raum"),
];
/// ⛽ GEG Anlage 4 energy carriers (wire value = subject string; labels localized).
const ENERGY_CARRIER: &[NormFieldChoice] = &[
    choice("natural_gas", "Natural gas", "Erdgas"),
    choice("heating_oil", "Heating oil", "Heizöl"),
    choice("electricity", "Electricity (grid mix)", "Strom (Netzmix)"),
    choice("district_heating", "District heating", "Fernwärme"),
    choice("biomass", "Biomass", "Biomasse"),
];
/// 🏷️ DIN V 18599-10 Nutzungsprofile (wire codes used in zone.usageProfile).
const USAGE_PROFILE: &[NormFieldChoice] = &[
    choice("WFH", "Residential (home)", "Wohnen"),
    choice("Office", "Office", "Büro"),
    choice("School", "School", "Schule"),
];
const BOOL_YES_NO: &[NormFieldChoice] = &[choice("true", "Yes", "Ja"), choice("false", "No", "Nein")];

const TABLE: &[(&str, FieldSpec)] = &[
    ("buildingCategory", FieldSpec::choice("Building category", "Gebäudekategorie", BUILDING_CATEGORY)),
    ("attachment", FieldSpec::choice("Attachment", "Anbauart", ATTACHMENT)),
    ("useClass", FieldSpec::choice("Use class", "Nutzungsklasse", USE_CLASS)),
    ("method", FieldSpec::choice("Calculation method", "Berechnungsverfahren", METHOD)),
    ("netFloorAreaM2", FieldSpec::quantity("Net floor area", "Nettogrundfläche", UnitId::SquareMetre)),
    ("heatedVolumeM3", FieldSpec::quantity("Heated volume", "Beheiztes Volumen", UnitId::CubicMetre)),
    ("gegQpFactor", FieldSpec::new("GEG QP factor", "GEG-QP-Faktor")),
    ("deltaUWbWM2k", FieldSpec::quantity("Thermal-bridge surcharge ΔU_WB", "Wärmebrückenzuschlag ΔU_WB", UnitId::WattPerSquareMetreKelvin)),
    ("automationClass", FieldSpec::choice("Automation class", "Automationsklasse", AUTOMATION)),
    ("heating", FieldSpec::new("Heating system", "Heizungsanlage")),
    ("heating.generationEfficiency", FieldSpec::new("Heating generation efficiency", "Heizung Erzeugerwirkungsgrad")),
    ("heating.distributionEfficiency", FieldSpec::new("Heating distribution efficiency", "Heizung Verteilwirkungsgrad")),
    ("heating.storageEfficiency", FieldSpec::new("Heating storage efficiency", "Heizung Speicherwirkungsgrad")),
    ("heating.transferEfficiency", FieldSpec::new("Heating transfer efficiency", "Heizung Übergabewirkungsgrad")),
    ("heating.energyCarrier", FieldSpec::choice("Heating energy carrier", "Heizung Energieträger", ENERGY_CARRIER)),
    ("dhw", FieldSpec::new("Domestic hot water", "Trinkwarmwasser")),
    ("dhw.specificDemandKwhPersonA", FieldSpec::quantity("DHW specific demand", "TWW-spezifischer Bedarf", UnitId::KilowattHourPerPersonYear)),
    ("dhw.storageLossKwhA", FieldSpec::quantity("DHW storage loss", "TWW-Speicherverlust", UnitId::KilowattHourPerYear)),
    ("dhw.distributionLossKwhA", FieldSpec::quantity("DHW distribution loss", "TWW-Verteilverlust", UnitId::KilowattHourPerYear)),
    ("dhw.energyCarrier", FieldSpec::choice("DHW energy carrier", "TWW-Energieträger", ENERGY_CARRIER)),
    ("ventilation", FieldSpec::new("Ventilation", "Lüftung")),
    ("ventilation.airflowM3H", FieldSpec::quantity("Ventilation airflow", "Lüftungsvolumenstrom", UnitId::CubicMetrePerHour)),
    ("ventilation.heatRecoveryEta", FieldSpec::new("Heat recovery efficiency", "Wärmerückgewinnungsgrad")),
    ("ventilation.fanPowerW", FieldSpec::quantity("Fan power", "Ventilatorleistung", UnitId::Watt)),
    ("cooling", FieldSpec::new("Cooling", "Kühlung")),
    ("cooling.plant", FieldSpec::new("Cooling plant", "Kühlanlage")),
    ("cooling.plant.eer", FieldSpec::new("Cooling EER", "Kühlung EER")),
    ("cooling.plant.energyCarrier", FieldSpec::choice("Cooling energy carrier", "Kühlung Energieträger", ENERGY_CARRIER)),
    ("lighting", FieldSpec::new("Lighting", "Beleuchtung")),
    ("lighting.controlFactor", FieldSpec::new("Lighting control factor", "Beleuchtungssteuerungsfaktor")),
    ("renewables", FieldSpec::new("Renewables", "Erneuerbare")),
    ("renewables.pvAreaM2", FieldSpec::quantity("PV area", "PV-Fläche", UnitId::SquareMetre)),
    ("renewables.pvEfficiency", FieldSpec::new("PV efficiency", "PV-Wirkungsgrad")),
    ("renewables.solarThermalKwhA", FieldSpec::quantity("Solar thermal yield", "Solarthermie-Ertrag", UnitId::KilowattHourPerYear)),
    ("climate", FieldSpec::new("Monthly climate", "Monatsklima")),
    ("climate.thetaEC", FieldSpec::quantity("Monthly mean outdoor temperature θ_e", "Monatsmittelwert der Außentemperatur θ_e", UnitId::Celsius)),
    ("climate.gHWM2", FieldSpec::quantity("Monthly mean global irradiance on the horizontal", "Monatsmittelwert der Globalstrahlung auf die Horizontale", UnitId::WattPerSquareMetre)),
    ("climateTable", FieldSpec::new("Climate table", "Klimatabelle")),
    ("climateTable.childId", FieldSpec::new("Climate table id", "Klimatabellen-ID")),
    ("climateTable.target", FieldSpec::new("Climate table target", "Klimatabellen-Ziel")),
    ("climateTable.target.artifactId", FieldSpec::new("Climate table artifact id", "Klimatabellen-Artifact-ID")),
    ("climateTable.target.dialect", FieldSpec::new("Climate table dialect", "Klimatabellen-Dialekt")),
    ("climateTable.target.dialect.artifactKind", FieldSpec::new("Climate table artifact kind", "Klimatabellen-Artifact-Art")),
    ("climateTable.target.dialect.standard", FieldSpec::new("Climate table standard", "Klimatabellen-Standard")),
    ("climateTable.target.dialect.subset", FieldSpec::new("Climate table subset", "Klimatabellen-Teilmenge")),
    ("elements", FieldSpec::new("Envelope elements", "Hüllflächenbauteile")),
    ("elements[].id", FieldSpec::new("Element id", "Bauteil-ID")),
    ("elements[].kind", FieldSpec::choice("Element kind", "Bauteilart", ELEMENT_KIND)),
    ("elements[].zoneId", FieldSpec::new("Zone id", "Zonen-ID")),
    ("elements[].areaM2", FieldSpec::quantity("Area", "Fläche", UnitId::SquareMetre)),
    ("elements[].uValueWM2k", FieldSpec::quantity("U-value", "U-Wert", UnitId::WattPerSquareMetreKelvin)),
    ("elements[].orientationDeg", FieldSpec::quantity("Orientation", "Orientierung", UnitId::Degree)),
    ("elements[].tiltDeg", FieldSpec::quantity("Tilt", "Neigung", UnitId::Degree)),
    ("elements[].gValue", FieldSpec::new("g-value", "g-Wert")),
    ("elements[].fc", FieldSpec::new("Shading factor Fc", "Abminderungsfaktor Fc")),
    ("elements[].adjacency", FieldSpec::choice("Adjacency", "Angrenzung", ADJACENCY)),
    ("elements[].labelEn", FieldSpec::new("Label (EN)", "Bezeichnung (EN)")),
    ("elements[].labelDe", FieldSpec::new("Label (DE)", "Bezeichnung (DE)")),
    ("zones", FieldSpec::new("Zones", "Zonen")),
    ("zones[].id", FieldSpec::new("Zone id", "Zonen-ID")),
    ("zones[].usageProfile", FieldSpec::choice("Usage profile", "Nutzungsprofil", USAGE_PROFILE)),
    ("zones[].areaM2", FieldSpec::quantity("Zone area", "Zonenfläche", UnitId::SquareMetre)),
    ("zones[].volumeM3", FieldSpec::quantity("Zone volume", "Zonenvolumen", UnitId::CubicMetre)),
    ("zones[].thetaIHeatC", FieldSpec::quantity("Indoor heating setpoint", "Heizsolltemperatur", UnitId::Celsius)),
    ("zones[].thetaICoolC", FieldSpec::quantity("Indoor cooling setpoint", "Kühlsolltemperatur", UnitId::Celsius)),
    ("zones[].occupants", FieldSpec::new("Occupants", "Personen")),
    ("zones[].internalGainsWM2", FieldSpec::quantity("Internal gains", "Interne Gewinne", UnitId::WattPerSquareMetre)),
    ("zones[].lightingPowerWM2", FieldSpec::quantity("Zone lighting power", "Zonenbeleuchtungsleistung", UnitId::WattPerSquareMetre)),
    ("zones[].labelEn", FieldSpec::new("Label (EN)", "Bezeichnung (EN)")),
    ("zones[].labelDe", FieldSpec::new("Label (DE)", "Bezeichnung (DE)")),
];

/// 🏷️ Exact-path / wildcard metadata for the din18599 document editor.
fn describe_din18599_field(path: &str) -> Option<FieldSpec> {
    lookup_norm_field_meta(TABLE, path)
}

#[cfg(test)]
#[path = "🧪️tests/🔬️field-meta-coverage/🦀️.rs"]
mod field_meta_coverage;

/// 🛡️ Descriptive field metadata admitted by the family's actual native setter vocabulary.
pub fn din18599_field_meta(path: &str) -> Option<FieldSpec> {
    semio_s_plugin_norm_app_surface::admit_list_meta(
        semio_s_plugin_norm_app_surface::admit_field_meta(
        describe_din18599_field(path),
        semio_s_artifact_norm_din18599_mutations::EDIT_RULES.can_set_field(path),
        ),
        semio_s_artifact_norm_din18599_mutations::EDIT_RULES.can_insert_item(path),
        semio_s_artifact_norm_din18599_mutations::EDIT_RULES.can_remove_item(path, semio_s_artifact_norm_din18599_mutations::ID_REMOVALS.iter().map(|rule| rule.path)),
    )
}
