//! 🏷️ DIN V 18599 NormFieldMeta lookup — SI display units + en/de labels (longest-prefix + `[]` wildcards).

use crate::app_surface::{lookup_norm_field_meta, NormFieldChoice, NormFieldMeta};

const fn choice(value: &'static str, label_en: &'static str, label_de: &'static str) -> NormFieldChoice {
    NormFieldChoice { value, label_en, label_de }
}

const BUILDING_CATEGORY: &[NormFieldChoice] = &[
    choice("Residential", "Residential", "Wohngebäude"),
    choice("NonResidential", "Non-residential", "Nichtwohngebäude"),
];
const ATTACHMENT: &[NormFieldChoice] = &[
    choice("Detached", "Detached", "Freistehend"),
    choice("SemiDetached", "Semi-detached", "Doppelhaushälfte"),
    choice("EndTerrace", "End terrace", "Reihenendhaus"),
    choice("MidTerrace", "Mid terrace", "Reihenmittelhaus"),
];
const USE_CLASS: &[NormFieldChoice] = &[
    choice("Residential", "Residential", "Wohnen"),
    choice("Office", "Office", "Büro"),
    choice("School", "School", "Schule"),
];
const METHOD: &[NormFieldChoice] = &[
    choice("DetailedMonthly", "Detailed monthly", "Detailliert monatlich"),
    choice("Tabular", "Tabular", "Tabellarisch"),
];
const AUTOMATION: &[NormFieldChoice] = &[
    choice("A", "Class A", "Klasse A"),
    choice("B", "Class B", "Klasse B"),
    choice("C", "Class C", "Klasse C"),
    choice("D", "Class D", "Klasse D"),
];
const ELEMENT_KIND: &[NormFieldChoice] = &[
    choice("Wall", "Wall", "Wand"),
    choice("Roof", "Roof", "Dach"),
    choice("Floor", "Floor", "Boden"),
    choice("Door", "Door", "Tür"),
    choice("Window", "Window", "Fenster"),
];
const ADJACENCY: &[NormFieldChoice] = &[
    choice("Outdoor", "Outdoor", "Außenluft"),
    choice("Ground", "Ground", "Erdreich"),
    choice("Unheated", "Unheated", "Unbeheizt"),
    choice("Heated", "Heated", "Beheizt"),
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
    choice("WFH", "Residential dwelling (WFH)", "Wohnen (WFH)"),
    choice("Office", "Office", "Büro"),
    choice("School", "School / education", "Schule / Unterricht"),
];
const BOOL_YES_NO: &[NormFieldChoice] = &[choice("true", "Yes", "Ja"), choice("false", "No", "Nein")];

const TABLE: &[(&str, NormFieldMeta)] = &[
    ("buildingCategory", NormFieldMeta { label_en: "Building category", label_de: "Gebäudekategorie", unit: None, choices: Some(BUILDING_CATEGORY) }),
    ("attachment", NormFieldMeta { label_en: "Attachment", label_de: "Anbauart", unit: None, choices: Some(ATTACHMENT) }),
    ("useClass", NormFieldMeta { label_en: "Use class", label_de: "Nutzungsklasse", unit: None, choices: Some(USE_CLASS) }),
    ("method", NormFieldMeta { label_en: "Calculation method", label_de: "Berechnungsverfahren", unit: None, choices: Some(METHOD) }),
    ("netFloorAreaM2", NormFieldMeta { label_en: "Net floor area", label_de: "Nettogrundfläche", unit: Some("m²"), choices: None }),
    ("heatedVolumeM3", NormFieldMeta { label_en: "Heated volume", label_de: "Beheiztes Volumen", unit: Some("m³"), choices: None }),
    ("gegQpFactor", NormFieldMeta { label_en: "GEG QP factor", label_de: "GEG-QP-Faktor", unit: None, choices: None }),
    ("deltaUWbWM2k", NormFieldMeta { label_en: "Thermal-bridge surcharge ΔU_WB", label_de: "Wärmebrückenzuschlag ΔU_WB", unit: Some("W/(m²·K)"), choices: None }),
    ("automationClass", NormFieldMeta { label_en: "Automation class", label_de: "Automationsklasse", unit: None, choices: Some(AUTOMATION) }),
    ("heating", NormFieldMeta { label_en: "Heating system", label_de: "Heizungsanlage", unit: None, choices: None }),
    ("heating.generationEfficiency", NormFieldMeta { label_en: "Heating generation efficiency", label_de: "Heizung Erzeugerwirkungsgrad", unit: None, choices: None }),
    ("heating.distributionEfficiency", NormFieldMeta { label_en: "Heating distribution efficiency", label_de: "Heizung Verteilwirkungsgrad", unit: None, choices: None }),
    ("heating.storageEfficiency", NormFieldMeta { label_en: "Heating storage efficiency", label_de: "Heizung Speicherwirkungsgrad", unit: None, choices: None }),
    ("heating.transferEfficiency", NormFieldMeta { label_en: "Heating transfer efficiency", label_de: "Heizung Übergabewirkungsgrad", unit: None, choices: None }),
    ("heating.energyCarrier", NormFieldMeta { label_en: "Heating energy carrier", label_de: "Heizung Energieträger", unit: None, choices: Some(ENERGY_CARRIER) }),
    ("dhw", NormFieldMeta { label_en: "Domestic hot water", label_de: "Trinkwarmwasser", unit: None, choices: None }),
    ("dhw.specificDemandKwhPersonA", NormFieldMeta { label_en: "DHW specific demand", label_de: "TWW-spezifischer Bedarf", unit: Some("kWh/(Person·a)"), choices: None }),
    ("dhw.storageLossKwhA", NormFieldMeta { label_en: "DHW storage loss", label_de: "TWW-Speicherverlust", unit: Some("kWh/a"), choices: None }),
    ("dhw.distributionLossKwhA", NormFieldMeta { label_en: "DHW distribution loss", label_de: "TWW-Verteilverlust", unit: Some("kWh/a"), choices: None }),
    ("dhw.energyCarrier", NormFieldMeta { label_en: "DHW energy carrier", label_de: "TWW-Energieträger", unit: None, choices: Some(ENERGY_CARRIER) }),
    ("ventilation", NormFieldMeta { label_en: "Ventilation", label_de: "Lüftung", unit: None, choices: None }),
    ("ventilation.airflowM3H", NormFieldMeta { label_en: "Ventilation airflow", label_de: "Lüftungsvolumenstrom", unit: Some("m³/h"), choices: None }),
    ("ventilation.heatRecoveryEta", NormFieldMeta { label_en: "Heat recovery efficiency", label_de: "Wärmerückgewinnungsgrad", unit: None, choices: None }),
    ("ventilation.fanPowerW", NormFieldMeta { label_en: "Fan power", label_de: "Ventilatorleistung", unit: Some("W"), choices: None }),
    ("cooling", NormFieldMeta { label_en: "Cooling", label_de: "Kühlung", unit: None, choices: None }),
    ("cooling.plant", NormFieldMeta { label_en: "Cooling plant", label_de: "Kühlanlage", unit: None, choices: None }),
    ("cooling.plant.eer", NormFieldMeta { label_en: "Cooling EER", label_de: "Kühlung EER", unit: None, choices: None }),
    ("cooling.plant.energyCarrier", NormFieldMeta { label_en: "Cooling energy carrier", label_de: "Kühlung Energieträger", unit: None, choices: Some(ENERGY_CARRIER) }),
    ("lighting", NormFieldMeta { label_en: "Lighting", label_de: "Beleuchtung", unit: None, choices: None }),
    ("lighting.controlFactor", NormFieldMeta { label_en: "Lighting control factor", label_de: "Beleuchtungssteuerungsfaktor", unit: None, choices: None }),
    ("renewables", NormFieldMeta { label_en: "Renewables", label_de: "Erneuerbare", unit: None, choices: None }),
    ("renewables.pvAreaM2", NormFieldMeta { label_en: "PV area", label_de: "PV-Fläche", unit: Some("m²"), choices: None }),
    ("renewables.pvEfficiency", NormFieldMeta { label_en: "PV efficiency", label_de: "PV-Wirkungsgrad", unit: None, choices: None }),
    ("renewables.solarThermalKwhA", NormFieldMeta { label_en: "Solar thermal yield", label_de: "Solarthermie-Ertrag", unit: Some("kWh/a"), choices: None }),
    ("climate", NormFieldMeta { label_en: "Climate", label_de: "Klima", unit: None, choices: None }),
    ("climate.childId", NormFieldMeta { label_en: "Climate child id", label_de: "Klima-Child-ID", unit: None, choices: None }),
    ("climate.target", NormFieldMeta { label_en: "Climate target", label_de: "Klima-Ziel", unit: None, choices: None }),
    ("climate.target.artifactId", NormFieldMeta { label_en: "Climate artifact id", label_de: "Klima-Artifact-ID", unit: None, choices: None }),
    ("climate.target.dialect", NormFieldMeta { label_en: "Climate dialect", label_de: "Klima-Dialekt", unit: None, choices: None }),
    ("climate.target.dialect.artifactKind", NormFieldMeta { label_en: "Climate artifact kind", label_de: "Klima-Artifact-Art", unit: None, choices: None }),
    ("climate.target.dialect.standard", NormFieldMeta { label_en: "Climate standard", label_de: "Klima-Standard", unit: None, choices: None }),
    ("climate.target.dialect.subset", NormFieldMeta { label_en: "Climate subset", label_de: "Klima-Teilmenge", unit: None, choices: None }),
    ("elements", NormFieldMeta { label_en: "Envelope elements", label_de: "Hüllflächenbauteile", unit: None, choices: None }),
    ("elements[].id", NormFieldMeta { label_en: "Element id", label_de: "Bauteil-ID", unit: None, choices: None }),
    ("elements[].kind", NormFieldMeta { label_en: "Element kind", label_de: "Bauteilart", unit: None, choices: Some(ELEMENT_KIND) }),
    ("elements[].zoneId", NormFieldMeta { label_en: "Zone id", label_de: "Zonen-ID", unit: None, choices: None }),
    ("elements[].areaM2", NormFieldMeta { label_en: "Area", label_de: "Fläche", unit: Some("m²"), choices: None }),
    ("elements[].uValueWM2k", NormFieldMeta { label_en: "U-value", label_de: "U-Wert", unit: Some("W/(m²·K)"), choices: None }),
    ("elements[].orientationDeg", NormFieldMeta { label_en: "Orientation", label_de: "Orientierung", unit: Some("°"), choices: None }),
    ("elements[].tiltDeg", NormFieldMeta { label_en: "Tilt", label_de: "Neigung", unit: Some("°"), choices: None }),
    ("elements[].gValue", NormFieldMeta { label_en: "g-value", label_de: "g-Wert", unit: None, choices: None }),
    ("elements[].fc", NormFieldMeta { label_en: "Shading factor Fc", label_de: "Abminderungsfaktor Fc", unit: None, choices: None }),
    ("elements[].adjacency", NormFieldMeta { label_en: "Adjacency", label_de: "Angrenzung", unit: None, choices: Some(ADJACENCY) }),
    ("elements[].labelEn", NormFieldMeta { label_en: "Label (EN)", label_de: "Bezeichnung (EN)", unit: None, choices: None }),
    ("elements[].labelDe", NormFieldMeta { label_en: "Label (DE)", label_de: "Bezeichnung (DE)", unit: None, choices: None }),
    ("zones", NormFieldMeta { label_en: "Zones", label_de: "Zonen", unit: None, choices: None }),
    ("zones[].id", NormFieldMeta { label_en: "Zone id", label_de: "Zonen-ID", unit: None, choices: None }),
    ("zones[].usageProfile", NormFieldMeta { label_en: "Usage profile", label_de: "Nutzungsprofil", unit: None, choices: Some(USAGE_PROFILE) }),
    ("zones[].areaM2", NormFieldMeta { label_en: "Zone area", label_de: "Zonenfläche", unit: Some("m²"), choices: None }),
    ("zones[].volumeM3", NormFieldMeta { label_en: "Zone volume", label_de: "Zonenvolumen", unit: Some("m³"), choices: None }),
    ("zones[].thetaIHeatC", NormFieldMeta { label_en: "Indoor heating setpoint", label_de: "Heizsolltemperatur", unit: Some("°C"), choices: None }),
    ("zones[].thetaICoolC", NormFieldMeta { label_en: "Indoor cooling setpoint", label_de: "Kühlsolltemperatur", unit: Some("°C"), choices: None }),
    ("zones[].occupants", NormFieldMeta { label_en: "Occupants", label_de: "Personen", unit: None, choices: None }),
    ("zones[].internalGainsWM2", NormFieldMeta { label_en: "Internal gains", label_de: "Interne Gewinne", unit: Some("W/m²"), choices: None }),
    ("zones[].lightingPowerWM2", NormFieldMeta { label_en: "Zone lighting power", label_de: "Zonenbeleuchtungsleistung", unit: Some("W/m²"), choices: None }),
    ("zones[].labelEn", NormFieldMeta { label_en: "Label (EN)", label_de: "Bezeichnung (EN)", unit: None, choices: None }),
    ("zones[].labelDe", NormFieldMeta { label_en: "Label (DE)", label_de: "Bezeichnung (DE)", unit: None, choices: None }),
];

/// 🏷️ Exact-path / wildcard metadata for the din18599 document editor.
pub fn field_meta(path: &str) -> Option<NormFieldMeta> {
    lookup_norm_field_meta(TABLE, path)
}

#[cfg(test)]
#[path = "🧪️tests/🔬️field-meta-coverage/🦀️.rs"]
mod field_meta_coverage;
