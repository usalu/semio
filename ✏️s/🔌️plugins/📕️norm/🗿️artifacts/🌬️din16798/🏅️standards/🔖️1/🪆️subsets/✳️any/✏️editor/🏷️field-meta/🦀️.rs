//! 🏷️ DIN EN 16798 NormFieldMeta lookup — SI display units + en/de labels for the structured inputs editor.

use crate::app_surface::{NormFieldChoice, NormFieldMeta};

/// 🏷️ B2 hook: exact path, else leaf segment / known suffixes.
pub fn din16798_field_meta(path: &str) -> Option<NormFieldMeta> {
    if let Some(meta) = exact(path) {
        return Some(meta);
    }
    let leaf = path.rsplit(['.', '[']).next().unwrap_or(path).trim_end_matches(']');
    leaf_meta(leaf)
}

fn exact(path: &str) -> Option<NormFieldMeta> {
    match path {
        "annex" => Some(NormFieldMeta {
            label_en: "National annex",
            label_de: "Nationaler Anhang",
            unit: None,
            choices: Some(&[NormFieldChoice { value: "de", label_en: "Germany (DE)", label_de: "Deutschland (DE)" }, NormFieldChoice { value: "en", label_en: "Europe (EN)", label_de: "Europa (EN)" }]),
        }),
        "thetaRmC" => Some(NormFieldMeta { label_en: "Running-mean outdoor temperature", label_de: "Gleitendes Außenluftmittel", unit: Some("°C"), choices: None }),
        "outdoorCo2Ppm" => Some(NormFieldMeta { label_en: "Outdoor CO₂ concentration", label_de: "Außenluft-CO₂-Konzentration", unit: Some("ppm"), choices: None }),
        "zones" => Some(NormFieldMeta { label_en: "Zones", label_de: "Zonen", unit: None, choices: None }),
        "ventSystems" => Some(NormFieldMeta { label_en: "Ventilation systems", label_de: "Lüftungsanlagen", unit: None, choices: None }),
        "envelopeN50HInv" => Some(NormFieldMeta { label_en: "Envelope airtightness n₅₀", label_de: "Gebäude-Luftdichtheit n₅₀", unit: Some("1/h"), choices: None }),
        "envelopeVolumeM3" => Some(NormFieldMeta { label_en: "Envelope internal volume", label_de: "Gebäudeinnenvolumen", unit: Some("m³"), choices: None }),
        "cellarAreaM2" => Some(NormFieldMeta { label_en: "Cellar floor area", label_de: "Kellerfläche", unit: Some("m²"), choices: None }),
        "cellarVentilationM3H" => Some(NormFieldMeta { label_en: "Cellar ventilation rate", label_de: "Kellerlüftungsvolumenstrom", unit: Some("m³/h"), choices: None }),
        "nightSetbackK" => Some(NormFieldMeta { label_en: "Night setback", label_de: "Nachtabsenkung", unit: Some("K"), choices: None }),
        _ => None,
    }
}

fn leaf_meta(leaf: &str) -> Option<NormFieldMeta> {
    match leaf {
        "id" => Some(NormFieldMeta { label_en: "Identifier", label_de: "Kennung", unit: None, choices: None }),
        "name" => Some(NormFieldMeta { label_en: "Display name", label_de: "Anzeigename", unit: None, choices: None }),
        "usageType" => Some(NormFieldMeta {
            label_en: "Usage type",
            label_de: "Nutzungsart",
            unit: None,
            choices: Some(&[
                NormFieldChoice { value: "office", label_en: "Office", label_de: "Büro" },
                NormFieldChoice { value: "residential", label_en: "Residential", label_de: "Wohnen" },
                NormFieldChoice { value: "classroom", label_en: "Classroom", label_de: "Unterrichtsraum" },
                NormFieldChoice { value: "meeting", label_en: "Meeting room", label_de: "Besprechungsraum" },
                NormFieldChoice { value: "retail", label_en: "Retail", label_de: "Handel" },
            ]),
        }),
        "floorAreaM2" => Some(NormFieldMeta { label_en: "Floor area", label_de: "Fußbodenfläche", unit: Some("m²"), choices: None }),
        "occupants" => Some(NormFieldMeta { label_en: "Number of occupants", label_de: "Personenanzahl", unit: None, choices: None }),
        "comfortCategory" => Some(NormFieldMeta {
            label_en: "Comfort category",
            label_de: "Komfortkategorie",
            unit: None,
            choices: Some(&[
                NormFieldChoice { value: "I", label_en: "I – high expectation", label_de: "I – hohe Erwartung" },
                NormFieldChoice { value: "II", label_en: "II – normal expectation", label_de: "II – normale Erwartung" },
                NormFieldChoice { value: "III", label_en: "III – moderate expectation", label_de: "III – mäßige Erwartung" },
                NormFieldChoice { value: "IV", label_en: "IV – low expectation", label_de: "IV – geringe Erwartung" },
            ]),
        }),
        "pollutionClass" => Some(NormFieldMeta {
            label_en: "Building pollution class",
            label_de: "Gebäude-Schadstoffklasse",
            unit: None,
            choices: Some(&[
                NormFieldChoice { value: "very_low", label_en: "Very low pollution", label_de: "Sehr niedrige Schadstoffbelastung" },
                NormFieldChoice { value: "low", label_en: "Low pollution", label_de: "Niedrige Schadstoffbelastung" },
                NormFieldChoice { value: "non_low", label_en: "Non-low pollution", label_de: "Nicht niedrige Schadstoffbelastung" },
            ]),
        }),
        "comfortModel" => Some(NormFieldMeta {
            label_en: "Comfort evaluation model",
            label_de: "Komfortbewertungsmodell",
            unit: None,
            choices: Some(&[
                NormFieldChoice { value: "fixed_hvac", label_en: "Fixed HVAC (PMV/PPD)", label_de: "Feste HVAC-Grenzen (PMV/PPD)" },
                NormFieldChoice { value: "adaptive", label_en: "Adaptive (Annex B.2)", label_de: "Adaptiv (Anhang B.2)" },
            ]),
        }),
        "tOpWinterC" => Some(NormFieldMeta { label_en: "Winter operative temperature", label_de: "Operative Wintertemperatur", unit: Some("°C"), choices: None }),
        "tOpSummerC" => Some(NormFieldMeta { label_en: "Summer operative temperature", label_de: "Operative Sommertemperatur", unit: Some("°C"), choices: None }),
        "airSpeedMS" => Some(NormFieldMeta { label_en: "Air speed", label_de: "Luftgeschwindigkeit", unit: Some("m/s"), choices: None }),
        "clothingClo" => Some(NormFieldMeta { label_en: "Clothing insulation (summer design)", label_de: "Bekleidungsisolation (Sommerauslegung)", unit: Some("clo"), choices: None }),
        "metabolicRateMet" => Some(NormFieldMeta { label_en: "Metabolic rate", label_de: "Stoffwechselrate", unit: Some("met"), choices: None }),
        "rhPercent" => Some(NormFieldMeta { label_en: "Relative humidity", label_de: "Relative Feuchte", unit: Some("%"), choices: None }),
        "outdoorAirSuppliedM3H" => Some(NormFieldMeta { label_en: "Outdoor air supplied", label_de: "Zugeführter Außenluftvolumenstrom", unit: Some("m³/h"), choices: None }),
        "co2Ppm" => Some(NormFieldMeta { label_en: "Indoor CO₂ concentration", label_de: "Raumluft-CO₂-Konzentration", unit: Some("ppm"), choices: None }),
        "illuminanceLx" => Some(NormFieldMeta { label_en: "Illuminance", label_de: "Beleuchtungsstärke", unit: Some("lx"), choices: None }),
        "noiseDb" => Some(NormFieldMeta { label_en: "Indoor noise level", label_de: "Raumschallpegel", unit: Some("dB"), choices: None }),
        "turbulenceIntensityPercent" => Some(NormFieldMeta { label_en: "Turbulence intensity", label_de: "Turbulenzintensität", unit: Some("%"), choices: None }),
        "ventMethod" => Some(NormFieldMeta {
            label_en: "Ventilation design method (EN 16798-1 §6.3)",
            label_de: "Lüftungsauslegungsmethode (EN 16798-1 §6.3)",
            unit: None,
            choices: Some(&[
                NormFieldChoice { value: "method_1_perceived_air_quality", label_en: "Method 1 – perceived air quality (Tables B.6/B.7)", label_de: "Methode 1 – empfundene Luftqualität (Tabellen B.6/B.7)" },
                NormFieldChoice { value: "method_2_limit_concentration", label_en: "Method 2 – limit substance concentration", label_de: "Methode 2 – Grenzkonzentration (Stoff)" },
                NormFieldChoice { value: "method_3_predefined_rates", label_en: "Method 3 – predefined outdoor-air rates", label_de: "Methode 3 – vorgegebene Außenluftraten" },
            ]),
        }),
        "ventSystemId" => Some(NormFieldMeta {
            label_en: "Linked ventilation system",
            label_de: "Verknüpfte Lüftungsanlage",
            unit: None,
            choices: Some(&[
                NormFieldChoice { value: "vent-central", label_en: "Central AHU (vent-central)", label_de: "Zentrale RLT (vent-central)" },
                NormFieldChoice { value: "vent-weak", label_en: "Weak AHU (vent-weak)", label_de: "Schwache RLT (vent-weak)" },
                NormFieldChoice { value: "vent-res", label_en: "Residential unit (vent-res)", label_de: "Wohnungslüftung (vent-res)" },
                NormFieldChoice { value: "vent-other", label_en: "Secondary AHU (vent-other)", label_de: "Zweite RLT (vent-other)" },
            ]),
        }),
        "systemType" => Some(NormFieldMeta {
            label_en: "System type",
            label_de: "Anlagentyp",
            unit: None,
            choices: Some(&[
                NormFieldChoice { value: "central_mech", label_en: "Central mechanical", label_de: "Zentrale mechanische Lüftung" },
                NormFieldChoice { value: "decentral_mech", label_en: "Decentral mechanical", label_de: "Dezentrale mechanische Lüftung" },
                NormFieldChoice { value: "natural", label_en: "Natural ventilation", label_de: "Natürliche Lüftung" },
            ]),
        }),
        "sfpWM3S" => Some(NormFieldMeta { label_en: "Specific fan power", label_de: "Spezifische Ventilatorleistung", unit: Some("W/(m³/s)"), choices: None }),
        "sfpRequiredClass" => Some(NormFieldMeta {
            label_en: "Required SFP class",
            label_de: "Geforderte SFP-Klasse",
            unit: None,
            choices: Some(&[
                NormFieldChoice { value: "0", label_en: "SFP 0 (≤ 300 W/(m³/s))", label_de: "SFP 0 (≤ 300 W/(m³/s))" },
                NormFieldChoice { value: "1", label_en: "SFP 1 (≤ 500 W/(m³/s))", label_de: "SFP 1 (≤ 500 W/(m³/s))" },
                NormFieldChoice { value: "2", label_en: "SFP 2 (≤ 750 W/(m³/s))", label_de: "SFP 2 (≤ 750 W/(m³/s))" },
                NormFieldChoice { value: "3", label_en: "SFP 3 (≤ 1250 W/(m³/s))", label_de: "SFP 3 (≤ 1250 W/(m³/s))" },
                NormFieldChoice { value: "4", label_en: "SFP 4 (≤ 2000 W/(m³/s))", label_de: "SFP 4 (≤ 2000 W/(m³/s))" },
                NormFieldChoice { value: "5", label_en: "SFP 5 (≤ 3000 W/(m³/s))", label_de: "SFP 5 (≤ 3000 W/(m³/s))" },
                NormFieldChoice { value: "6", label_en: "SFP 6 (≤ 4500 W/(m³/s))", label_de: "SFP 6 (≤ 4500 W/(m³/s))" },
                NormFieldChoice { value: "7", label_en: "SFP 7 (≤ 6500 W/(m³/s))", label_de: "SFP 7 (≤ 6500 W/(m³/s))" },
            ]),
        }),
        "heatRecoveryEta" => Some(NormFieldMeta { label_en: "Heat recovery temperature efficiency", label_de: "Temperaturübertragungsgrad der Wärmerückgewinnung", unit: Some("1"), choices: None }),
        "odaClass" => Some(NormFieldMeta {
            label_en: "Outdoor air (ODA) class",
            label_de: "Außenluftklasse (ODA)",
            unit: None,
            choices: Some(&[
                NormFieldChoice { value: "ODA1", label_en: "ODA 1 – clean outdoor air", label_de: "ODA 1 – reine Außenluft" },
                NormFieldChoice { value: "ODA2", label_en: "ODA 2 – dusty outdoor air", label_de: "ODA 2 – staubhaltige Außenluft" },
                NormFieldChoice { value: "ODA3", label_en: "ODA 3 – very high concentrations", label_de: "ODA 3 – sehr hohe Konzentrationen" },
                NormFieldChoice { value: "ODA4", label_en: "ODA 4 – extremely high concentrations (ISO 16890-1)", label_de: "ODA 4 – extrem hohe Konzentrationen (ISO 16890-1)" },
            ]),
        }),
        "filterSupClass" => Some(NormFieldMeta {
            label_en: "Supply air filter class (ISO 16890)",
            label_de: "Zuluftfilterklasse (ISO 16890)",
            unit: None,
            choices: Some(&[
                NormFieldChoice { value: "ePM1_80_G", label_en: "ePM1 ≥ 80 % + gas filter", label_de: "ePM1 ≥ 80 % + Gasfilter" },
                NormFieldChoice { value: "ePM1_80", label_en: "ePM1 ≥ 80 %", label_de: "ePM1 ≥ 80 %" },
                NormFieldChoice { value: "ePM1_55", label_en: "ePM1 ≥ 55 %", label_de: "ePM1 ≥ 55 %" },
                NormFieldChoice { value: "ePM2_5_65", label_en: "ePM2.5 ≥ 65 %", label_de: "ePM2,5 ≥ 65 %" },
                NormFieldChoice { value: "ePM10_50", label_en: "ePM10 ≥ 50 %", label_de: "ePM10 ≥ 50 %" },
                NormFieldChoice { value: "coarse", label_en: "Coarse (ISO 16890)", label_de: "Grobstaub (ISO 16890)" },
            ]),
        }),
        "yearsSinceInspection" => Some(NormFieldMeta { label_en: "Years since inspection", label_de: "Jahre seit der Inspektion", unit: Some("a"), choices: None }),
        "humidificationRequiredKgH" => Some(NormFieldMeta { label_en: "Humidification required", label_de: "Erforderliche Befeuchtungsleistung", unit: Some("kg/h"), choices: None }),
        "humidificationProvidedKgH" => Some(NormFieldMeta { label_en: "Humidification provided", label_de: "Bereitgestellte Befeuchtungsleistung", unit: Some("kg/h"), choices: None }),
        "fanQVM3S" => Some(NormFieldMeta { label_en: "Fan volume flow", label_de: "Ventilator-Volumenstrom", unit: Some("m³/s"), choices: None }),
        "fanTRunH" => Some(NormFieldMeta { label_en: "Fan annual run hours", label_de: "Jährliche Ventilatorlaufzeit", unit: Some("h"), choices: None }),
        "ductClass" => Some(NormFieldMeta {
            label_en: "Duct airtightness class",
            label_de: "Kanal-Dichtheitsklasse",
            unit: None,
            choices: Some(&[
                NormFieldChoice { value: "A", label_en: "Class A", label_de: "Klasse A" },
                NormFieldChoice { value: "B", label_en: "Class B", label_de: "Klasse B" },
                NormFieldChoice { value: "C", label_en: "Class C", label_de: "Klasse C" },
                NormFieldChoice { value: "D", label_en: "Class D", label_de: "Klasse D" },
            ]),
        }),
        "ductTestPressurePa" => Some(NormFieldMeta { label_en: "Duct test pressure", label_de: "Kanal-Prüfdruck", unit: Some("Pa"), choices: None }),
        "ductLeakageM3SM2" => Some(NormFieldMeta { label_en: "Duct leakage factor", label_de: "Kanalleckagefaktor", unit: Some("m³/(s·m²)"), choices: None }),
        "designAirflowM3H" => Some(NormFieldMeta { label_en: "Design outdoor-air capacity", label_de: "Auslegungs-Außenluftkapazität", unit: Some("m³/h"), choices: None }),
        _ => None,
    }
}
