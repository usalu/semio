//! 🏷️ DIN EN 16798 FieldSpec lookup — SI display units + en/de labels for the structured inputs editor.

use semio_s_plugin_norm_app_surface::{NormFieldChoice, FieldSpec, UnitId};

/// 🏷️ B2 hook: exact path, else leaf segment / known suffixes.
fn describe_din16798_field(path: &str) -> Option<FieldSpec> {
    if let Some(meta) = exact(path) {
        return Some(meta);
    }
    let leaf = path.rsplit(['.', '[']).next().unwrap_or(path).trim_end_matches(']');
    leaf_meta(leaf)
}

fn exact(path: &str) -> Option<FieldSpec> {
    match path {
        "annex" => Some(FieldSpec::choice("National annex", "Nationaler Anhang", &[NormFieldChoice { value: "de", label_en: "Germany (DE)", label_de: "Deutschland (DE)" }, NormFieldChoice { value: "en", label_en: "Europe (EN)", label_de: "Europa (EN)" }])),
        "thetaRmC" => Some(FieldSpec::quantity("Running-mean outdoor temperature", "Gleitendes Außenluftmittel", UnitId::Celsius)),
        "outdoorCo2Ppm" => Some(FieldSpec::quantity("Outdoor CO₂ concentration", "Außenluft-CO₂-Konzentration", UnitId::Ppm)),
        "zones" => Some(FieldSpec::new("Zones", "Zonen")),
        "ventSystems" => Some(FieldSpec::new("Ventilation systems", "Lüftungsanlagen")),
        "envelopeN50HInv" => Some(FieldSpec::quantity("Envelope airtightness n₅₀", "Gebäude-Luftdichtheit n₅₀", UnitId::PerHour)),
        "envelopeVolumeM3" => Some(FieldSpec::quantity("Envelope internal volume", "Gebäudeinnenvolumen", UnitId::CubicMetre)),
        "cellarAreaM2" => Some(FieldSpec::quantity("Cellar floor area", "Kellerfläche", UnitId::SquareMetre)),
        "cellarVentilationM3H" => Some(FieldSpec::quantity("Cellar ventilation rate", "Kellerlüftungsvolumenstrom", UnitId::CubicMetrePerHour)),
        "nightSetbackK" => Some(FieldSpec::quantity("Night setback", "Nachtabsenkung", UnitId::Kelvin)),
        _ => None,
    }
}

fn leaf_meta(leaf: &str) -> Option<FieldSpec> {
    match leaf {
        "id" => Some(FieldSpec::new("Identifier", "Kennung")),
        "name" => Some(FieldSpec::new("Display name", "Anzeigename")),
        "usageType" => Some(FieldSpec::choice("Usage type", "Nutzungsart", &[
                NormFieldChoice { value: "office", label_en: "Office", label_de: "Büro" },
                NormFieldChoice { value: "residential", label_en: "Residential", label_de: "Wohnen" },
                NormFieldChoice { value: "classroom", label_en: "Classroom", label_de: "Unterrichtsraum" },
                NormFieldChoice { value: "meeting", label_en: "Meeting room", label_de: "Besprechungsraum" },
                NormFieldChoice { value: "retail", label_en: "Retail", label_de: "Handel" },
            ])),
        "floorAreaM2" => Some(FieldSpec::quantity("Floor area", "Fußbodenfläche", UnitId::SquareMetre)),
        "occupants" => Some(FieldSpec::new("Number of occupants", "Personenanzahl")),
        "comfortCategory" => Some(FieldSpec::choice("Comfort category", "Komfortkategorie", &[
                NormFieldChoice { value: "I", label_en: "I – high expectation", label_de: "I – hohe Erwartung" },
                NormFieldChoice { value: "II", label_en: "II – normal expectation", label_de: "II – normale Erwartung" },
                NormFieldChoice { value: "III", label_en: "III – moderate expectation", label_de: "III – mäßige Erwartung" },
                NormFieldChoice { value: "IV", label_en: "IV – low expectation", label_de: "IV – geringe Erwartung" },
            ])),
        "pollutionClass" => Some(FieldSpec::choice("Building pollution class", "Gebäude-Schadstoffklasse", &[
                NormFieldChoice { value: "very_low", label_en: "Very low pollution", label_de: "Sehr niedrige Schadstoffbelastung" },
                NormFieldChoice { value: "low", label_en: "Low pollution", label_de: "Niedrige Schadstoffbelastung" },
                NormFieldChoice { value: "non_low", label_en: "Non-low pollution", label_de: "Nicht niedrige Schadstoffbelastung" },
            ])),
        "comfortModel" => Some(FieldSpec::choice("Comfort evaluation model", "Komfortbewertungsmodell", &[
                NormFieldChoice { value: "fixed_hvac", label_en: "Fixed HVAC (PMV/PPD)", label_de: "Feste HVAC-Grenzen (PMV/PPD)" },
                NormFieldChoice { value: "adaptive", label_en: "Adaptive (Annex B.2)", label_de: "Adaptiv (Anhang B.2)" },
            ])),
        "tOpWinterC" => Some(FieldSpec::quantity("Winter operative temperature", "Operative Wintertemperatur", UnitId::Celsius)),
        "tOpSummerC" => Some(FieldSpec::quantity("Summer operative temperature", "Operative Sommertemperatur", UnitId::Celsius)),
        "airSpeedMS" => Some(FieldSpec::quantity("Air speed", "Luftgeschwindigkeit", UnitId::MetrePerSecond)),
        "clothingClo" => Some(FieldSpec::quantity("Clothing insulation (summer design)", "Bekleidungsisolation (Sommerauslegung)", UnitId::Clo)),
        "metabolicRateMet" => Some(FieldSpec::quantity("Metabolic rate", "Stoffwechselrate", UnitId::Met)),
        "rhPercent" => Some(FieldSpec::quantity("Relative humidity", "Relative Feuchte", UnitId::Percent)),
        "outdoorAirSuppliedM3H" => Some(FieldSpec::quantity("Outdoor air supplied", "Zugeführter Außenluftvolumenstrom", UnitId::CubicMetrePerHour)),
        "co2Ppm" => Some(FieldSpec::quantity("Indoor CO₂ concentration", "Raumluft-CO₂-Konzentration", UnitId::Ppm)),
        "illuminanceLx" => Some(FieldSpec::quantity("Illuminance", "Beleuchtungsstärke", UnitId::Lux)),
        "noiseDb" => Some(FieldSpec::quantity("Indoor noise level", "Raumschallpegel", UnitId::Decibel)),
        "turbulenceIntensityPercent" => Some(FieldSpec::quantity("Turbulence intensity", "Turbulenzintensität", UnitId::Percent)),
        "ventMethod" => Some(FieldSpec::choice("Ventilation design method (EN 16798-1 §6.3)", "Lüftungsauslegungsmethode (EN 16798-1 §6.3)", &[
                NormFieldChoice { value: "method_1_perceived_air_quality", label_en: "Method 1 – perceived air quality (Tables B.6/B.7)", label_de: "Methode 1 – empfundene Luftqualität (Tabellen B.6/B.7)" },
                NormFieldChoice { value: "method_2_limit_concentration", label_en: "Method 2 – limit substance concentration", label_de: "Methode 2 – Grenzkonzentration (Stoff)" },
                NormFieldChoice { value: "method_3_predefined_rates", label_en: "Method 3 – predefined outdoor-air rates", label_de: "Methode 3 – vorgegebene Außenluftraten" },
            ])),
        "ventSystemId" => Some(FieldSpec::choice("Linked ventilation system", "Verknüpfte Lüftungsanlage", &[
                NormFieldChoice { value: "vent-central", label_en: "Central AHU (vent-central)", label_de: "Zentrale RLT (vent-central)" },
                NormFieldChoice { value: "vent-weak", label_en: "Weak AHU (vent-weak)", label_de: "Schwache RLT (vent-weak)" },
                NormFieldChoice { value: "vent-res", label_en: "Residential unit (vent-res)", label_de: "Wohnungslüftung (vent-res)" },
                NormFieldChoice { value: "vent-other", label_en: "Secondary AHU (vent-other)", label_de: "Zweite RLT (vent-other)" },
            ])),
        "systemType" => Some(FieldSpec::choice("System type", "Anlagentyp", &[
                NormFieldChoice { value: "central_mech", label_en: "Central mechanical", label_de: "Zentrale mechanische Lüftung" },
                NormFieldChoice { value: "decentral_mech", label_en: "Decentral mechanical", label_de: "Dezentrale mechanische Lüftung" },
                NormFieldChoice { value: "natural", label_en: "Natural ventilation", label_de: "Natürliche Lüftung" },
            ])),
        "sfpWM3S" => Some(FieldSpec::quantity("Specific fan power", "Spezifische Ventilatorleistung", UnitId::WattPerCubicMetrePerSecond)),
        "sfpRequiredClass" => Some(FieldSpec::choice("Required SFP class", "Geforderte SFP-Klasse", &[
                NormFieldChoice { value: "0", label_en: "SFP 0 (≤ 300 W/(m³/s))", label_de: "SFP 0 (≤ 300 W/(m³/s))" },
                NormFieldChoice { value: "1", label_en: "SFP 1 (≤ 500 W/(m³/s))", label_de: "SFP 1 (≤ 500 W/(m³/s))" },
                NormFieldChoice { value: "2", label_en: "SFP 2 (≤ 750 W/(m³/s))", label_de: "SFP 2 (≤ 750 W/(m³/s))" },
                NormFieldChoice { value: "3", label_en: "SFP 3 (≤ 1250 W/(m³/s))", label_de: "SFP 3 (≤ 1250 W/(m³/s))" },
                NormFieldChoice { value: "4", label_en: "SFP 4 (≤ 2000 W/(m³/s))", label_de: "SFP 4 (≤ 2000 W/(m³/s))" },
                NormFieldChoice { value: "5", label_en: "SFP 5 (≤ 3000 W/(m³/s))", label_de: "SFP 5 (≤ 3000 W/(m³/s))" },
                NormFieldChoice { value: "6", label_en: "SFP 6 (≤ 4500 W/(m³/s))", label_de: "SFP 6 (≤ 4500 W/(m³/s))" },
                NormFieldChoice { value: "7", label_en: "SFP 7 (≤ 6500 W/(m³/s))", label_de: "SFP 7 (≤ 6500 W/(m³/s))" },
            ])),
        "heatRecoveryEta" => Some(FieldSpec::quantity("Heat recovery temperature efficiency", "Temperaturübertragungsgrad der Wärmerückgewinnung", UnitId::One)),
        "odaClass" => Some(FieldSpec::choice("Outdoor air (ODA) class", "Außenluftklasse (ODA)", &[
                NormFieldChoice { value: "ODA1", label_en: "ODA 1 – clean outdoor air", label_de: "ODA 1 – reine Außenluft" },
                NormFieldChoice { value: "ODA2", label_en: "ODA 2 – dusty outdoor air", label_de: "ODA 2 – staubhaltige Außenluft" },
                NormFieldChoice { value: "ODA3", label_en: "ODA 3 – very high concentrations", label_de: "ODA 3 – sehr hohe Konzentrationen" },
                NormFieldChoice { value: "ODA4", label_en: "ODA 4 – extremely high concentrations (ISO 16890-1)", label_de: "ODA 4 – extrem hohe Konzentrationen (ISO 16890-1)" },
            ])),
        "filterSupClass" => Some(FieldSpec::choice("Supply air filter class (ISO 16890)", "Zuluftfilterklasse (ISO 16890)", &[
                NormFieldChoice { value: "ePM1_80_G", label_en: "ePM1 ≥ 80 % + gas filter", label_de: "ePM1 ≥ 80 % + Gasfilter" },
                NormFieldChoice { value: "ePM1_80", label_en: "ePM1 ≥ 80 %", label_de: "ePM1 ≥ 80 %" },
                NormFieldChoice { value: "ePM1_55", label_en: "ePM1 ≥ 55 %", label_de: "ePM1 ≥ 55 %" },
                NormFieldChoice { value: "ePM2_5_65", label_en: "ePM2.5 ≥ 65 %", label_de: "ePM2,5 ≥ 65 %" },
                NormFieldChoice { value: "ePM10_50", label_en: "ePM10 ≥ 50 %", label_de: "ePM10 ≥ 50 %" },
                NormFieldChoice { value: "coarse", label_en: "Coarse (ISO 16890)", label_de: "Grobstaub (ISO 16890)" },
            ])),
        "yearsSinceInspection" => Some(FieldSpec::quantity("Years since inspection", "Jahre seit der Inspektion", UnitId::Year)),
        "humidificationRequiredKgH" => Some(FieldSpec::quantity("Humidification required", "Erforderliche Befeuchtungsleistung", UnitId::KilogramPerHour)),
        "humidificationProvidedKgH" => Some(FieldSpec::quantity("Humidification provided", "Bereitgestellte Befeuchtungsleistung", UnitId::KilogramPerHour)),
        "fanQVM3S" => Some(FieldSpec::quantity("Fan volume flow", "Ventilator-Volumenstrom", UnitId::CubicMetrePerSecond)),
        "fanTRunH" => Some(FieldSpec::quantity("Fan annual run hours", "Jährliche Ventilatorlaufzeit", UnitId::Hour)),
        "ductClass" => Some(FieldSpec::choice("Duct airtightness class", "Kanal-Dichtheitsklasse", &[
                NormFieldChoice { value: "A", label_en: "Class A", label_de: "Klasse A" },
                NormFieldChoice { value: "B", label_en: "Class B", label_de: "Klasse B" },
                NormFieldChoice { value: "C", label_en: "Class C", label_de: "Klasse C" },
                NormFieldChoice { value: "D", label_en: "Class D", label_de: "Klasse D" },
            ])),
        "ductTestPressurePa" => Some(FieldSpec::quantity("Duct test pressure", "Kanal-Prüfdruck", UnitId::Pascal)),
        "ductLeakageM3SM2" => Some(FieldSpec::quantity("Duct leakage factor", "Kanalleckagefaktor", UnitId::CubicMetrePerSecondSquareMetre)),
        "designAirflowM3H" => Some(FieldSpec::quantity("Design outdoor-air capacity", "Auslegungs-Außenluftkapazität", UnitId::CubicMetrePerHour)),
        _ => None,
    }
}

/// 🛡️ Descriptive field metadata admitted by the family's actual native setter vocabulary.
pub fn din16798_field_meta(path: &str) -> Option<FieldSpec> {
    semio_s_plugin_norm_app_surface::admit_list_meta(
        semio_s_plugin_norm_app_surface::admit_field_meta(
        describe_din16798_field(path),
        semio_s_artifact_norm_din16798_mutations::EDIT_RULES.can_set_field(path),
        ),
        semio_s_artifact_norm_din16798_mutations::EDIT_RULES.can_insert_item(path),
        semio_s_artifact_norm_din16798_mutations::EDIT_RULES.can_remove_item(path, ["zones", "ventSystems"]),
    )
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
