//! 📇️ DIN 4108-3 period-method inputs and prescriptive construction cases.

use super::{AnnexADryingExposure, annex_a_elevation_in_scope};
use super::material_limits::CondensateMaterials;
use super::construction_cases::{
    AtticCeiling, attic_ceiling_limits, interior_wall_limit, internal_slab_sd_recommendation,
    perimeter_slab_limit, unventilated_roof_limit, unventilated_roof_minimum_sd,
};
use semio_s_plugin_norm_compliance::design_case::{CaseCatalogue, CaseEntry, CaseValues, DesignCase, InputChoice, InputSpec};
use semio_s_plugin_norm_compliance::{AnnexChoice, CheckResult, CheckStatus, ClauseId, LocalizedCopy, SubjectRef, UnitId};

const STANDARD: &str = "DIN 4108-3:2024-03";
pub const SITE_ENTRY: &str = "DINA-01";
pub const ROOF_ENTRY: &str = "DINA-02";
const MATERIAL_ENTRY: &str = "DINA-08";
pub const SLAB_ENTRY: &str = "DINA-03";
pub const ATTIC_ENTRY: &str = "DINA-04";
const PERIMETER_ENTRY: &str = "DINA-05";
const WALL_ENTRY: &str = "DINA-06";
const UNVENTILATED_ROOF_ENTRY: &str = "DINA-07";

const ENTRIES: &[CaseEntry] = &[
    CaseEntry {
        id: SITE_ENTRY, part: STANDARD, clause: "5.2.1",
        en: "Period-method site and room conditions", de: "Standort und Raumzustand für das Periodenverfahren",
        inputs: &["site_elevation", "period_conditions"], geometry: None,
    },
    CaseEntry {
        id: ROOF_ENTRY, part: STANDARD, clause: "A.2.2",
        en: "Roof exposure for the drying period", de: "Dachrandbedingung für die Verdunstungsperiode",
        inputs: &["roof_element", "roof_exposure"], geometry: None,
    },
    CaseEntry {
        id: SLAB_ENTRY, part: STANDARD, clause: "5.3.3.3",
        en: "Ground slab with room-side insulation", de: "Bodenplatte mit raumseitiger Dämmung",
        inputs: &["slab_build_up", "slab_room_side_sd"], geometry: None,
    },
    CaseEntry {
        id: ATTIC_ENTRY, part: STANDARD, clause: "5.3.5 / Figure 17",
        en: "Ceiling below a ventilated attic", de: "Oberste Geschossdecke unter belüftetem Dachraum",
        inputs: &["attic_build_up", "attic_area", "attic_openings", "attic_upper_sd", "attic_insulation_sd", "attic_air_seal_sd", "attic_inboard_r_fraction"], geometry: None,
    },
    CaseEntry {
        id: PERIMETER_ENTRY, part: STANDARD, clause: "5.3.3.2",
        en: "Ground slab with perimeter insulation", de: "Bodenplatte mit Perimeterdämmung",
        inputs: &["perimeter_build_up", "perimeter_inboard_r_fraction"], geometry: None,
    },
    CaseEntry {
        id: WALL_ENTRY, part: STANDARD, clause: "5.3.2.2",
        en: "External wall with room-side insulation", de: "Außenwand mit Innendämmung",
        inputs: &["wall_build_up", "wall_insulation_r", "wall_inboard_sd"], geometry: None,
    },
    CaseEntry {
        id: UNVENTILATED_ROOF_ENTRY, part: STANDARD, clause: "5.3.4.2.2 / Table 3",
        en: "Unventilated roof insulation under Figures 4 and 5", de: "Nicht belüftete Dachdämmung nach Bild 4 und Bild 5",
        inputs: &["table3_roof_build_up", "table3_roof_outboard_sd", "table3_roof_inboard_sd", "table3_roof_inboard_r_fraction"], geometry: None,
    },
    CaseEntry {
        id: MATERIAL_ENTRY, part: STANDARD, clause: "5.2.2",
        en: "Materials at condensation interfaces", de: "Baustoffe an Tauwasserebenen",
        inputs: &["condensate_element", "condensate_capillarity", "condensate_damage", "condensate_timber_increase", "condensate_wood_product_increase"], geometry: None,
    },
];

const INPUTS: &[InputSpec] = &[
    InputSpec::number("site_elevation", "Site elevation above sea level", "Standorthöhe über NN", UnitId::Metre)
        .range(-500.0, 9000.0).example(500.0).scope(SITE_ENTRY),
    InputSpec::choice("period_conditions", "Room conditions under §5.2.1", "Raumzustand nach 5.2.1", &[
        InputChoice::new("confirmed", "Residential or similar and eligible", "Wohn- oder wohnähnliche Nutzung im Anwendungsbereich"),
        InputChoice::new("excluded", "Room conditions exclude the period method", "Raumzustand schließt das Periodenverfahren aus"),
        InputChoice::new("unconfirmed", "Room conditions unconfirmed", "Raumzustand unbestätigt"),
    ]).example_text("confirmed").scope(SITE_ENTRY),
    InputSpec::text("roof_element", "Roof element ID", "Dachbauteil-ID").example_text("roof").scope(ROOF_ENTRY),
    InputSpec::choice("roof_exposure", "Roof cover and shading", "Dachdeckung und Verschattung", &[
        InputChoice::new("shaded-or-light", "Shaded or light-coloured roof", "Verschattetes oder helles Dach"),
        InputChoice::new("unshaded-dark", "Unshaded dark roof", "Unverschattetes dunkles Dach"),
        InputChoice::new("unconfirmed", "Roof exposure unconfirmed", "Dachrandbedingung unbestätigt"),
    ]).example_text("shaded-or-light").scope(ROOF_ENTRY),
    InputSpec::choice("slab_build_up", "Ground slab construction evidence", "Nachweis des Bodenplattenaufbaus", &[
        InputChoice::new("confirmed", "Heated occupied room, ground contact, specified waterproofing, screed and insulation", "Beheizter Aufenthaltsraum, Erdreichkontakt, vorgesehene Abdichtung, Estrich und Dämmung"),
        InputChoice::new("unconfirmed", "Slab construction unconfirmed", "Bodenplattenaufbau unbestätigt"),
    ]).example_text("confirmed").scope(SLAB_ENTRY),
    InputSpec::number("slab_room_side_sd", "Total s_d above the insulation", "Gesamt-s_d raumseitig der Dämmung", UnitId::Metre)
        .range(0.0, 1000000.0).example(24.0).scope(SLAB_ENTRY),
    InputSpec::choice("attic_build_up", "Attic ceiling construction evidence", "Nachweis des Dachdeckenaufbaus", &[
        InputChoice::new("confirmed", "Figure 17 assembly, outdoor openings and §5.3.1 confirmed", "Aufbau nach Bild 17, Außenöffnungen und 5.3.1 bestätigt"),
        InputChoice::new("unconfirmed", "Ceiling construction unconfirmed", "Dachdeckenaufbau unbestätigt"),
    ]).example_text("confirmed").scope(ATTIC_ENTRY),
    InputSpec::number("attic_area", "Attic plan area", "Grundfläche des Dachraums", UnitId::SquareMetre)
        .range(0.0, 1000000.0).example(90.0).scope(ATTIC_ENTRY),
    InputSpec::number("attic_openings", "Total free ventilation opening area", "Gesamtfläche freier Lüftungsöffnungen", UnitId::SquareMetre)
        .range(0.0, 1000000.0).example(0.20).scope(ATTIC_ENTRY),
    InputSpec::number("attic_upper_sd", "Total s_d of covering and decking", "Gesamt-s_d von Bodenbelag und Schalung", UnitId::Metre)
        .range(0.0, 1000000.0).example(1.0).scope(ATTIC_ENTRY),
    InputSpec::number("attic_insulation_sd", "Insulation s_d", "s_d der Dämmung", UnitId::Metre)
        .range(0.0, 1000000.0).example(5.0).scope(ATTIC_ENTRY),
    InputSpec::number("attic_air_seal_sd", "Room-side airtight layer s_d", "s_d der raumseitigen Luftdichtheitsschicht", UnitId::Metre)
        .range(0.0, 1000000.0).example(15.0).scope(ATTIC_ENTRY),
    InputSpec::number("attic_inboard_r_fraction", "Room-side share of thermal resistance", "Raumseitiger Anteil des Wärmedurchlasswiderstands", UnitId::One)
        .range(0.0, 1.0).example(0.15).scope(ATTIC_ENTRY),
    InputSpec::choice("perimeter_build_up", "Perimeter-insulated slab evidence", "Nachweis der Perimeterdämmung", &[
        InputChoice::new("confirmed", "Ground contact, perimeter insulation, DIN 18533 waterproofing and §5.3.1 confirmed", "Erdkontakt, Perimeterdämmung, Abdichtung nach DIN 18533 und 5.3.1 bestätigt"),
        InputChoice::new("unconfirmed", "Construction prerequisites unconfirmed", "Voraussetzungen des Aufbaus unbestätigt"),
    ]).example_text("confirmed").scope(PERIMETER_ENTRY),
    InputSpec::number("perimeter_inboard_r_fraction", "Share of layer resistance inward of waterproofing", "Widerstandsanteil raumseitig der Abdichtung", UnitId::One)
        .range(0.0, 1.0).example(0.20).scope(PERIMETER_ENTRY),
    InputSpec::choice("wall_build_up", "Room-side wall insulation evidence", "Nachweis der Innendämmung", &[
        InputChoice::new("confirmed", "§5.3.1, rain protection and prevention of room airflow behind insulation confirmed", "5.3.1, Schlagregenschutz und Verhinderung der Hinterströmung bestätigt"),
        InputChoice::new("unconfirmed", "Construction prerequisites unconfirmed", "Voraussetzungen des Aufbaus unbestätigt"),
    ]).example_text("confirmed").scope(WALL_ENTRY),
    InputSpec::number("wall_insulation_r", "Thermal resistance of internal insulation", "Wärmedurchlasswiderstand der Innendämmung", UnitId::SquareMetreKelvinPerWatt)
        .range(0.0, 100.0).example(0.75).scope(WALL_ENTRY),
    InputSpec::number("wall_inboard_sd", "Total s_d of insulation and room-side lining", "Gesamt-s_d der Dämmung einschließlich raumseitiger Bekleidung", UnitId::Metre)
        .range(0.0, 1000000.0).example(0.5).scope(WALL_ENTRY),
    InputSpec::choice("table3_roof_build_up", "Figure 4 or Figure 5 roof evidence", "Nachweis des Dachaufbaus nach Bild 4 oder 5", &[
        InputChoice::new("confirmed", "Figure 4 or 5 assembly, required materials, weather protection and §5.3.1 confirmed", "Aufbau nach Bild 4 oder 5, vorgesehene Baustoffe, Witterungsschutz und 5.3.1 bestätigt"),
        InputChoice::new("unconfirmed", "Construction prerequisites unconfirmed", "Voraussetzungen des Aufbaus unbestätigt"),
    ]).example_text("confirmed").scope(UNVENTILATED_ROOF_ENTRY),
    InputSpec::number("table3_roof_outboard_sd", "Total s_d above insulation up to the ventilated layer", "Gesamt-s_d oberhalb der Dämmung bis zur belüfteten Luftschicht", UnitId::Metre)
        .range(0.0, 1000000.0).example(0.3).scope(UNVENTILATED_ROOF_ENTRY),
    InputSpec::number("table3_roof_inboard_sd", "Total s_d below insulation", "Gesamt-s_d unterhalb der Dämmung", UnitId::Metre)
        .range(0.0, 1000000.0).example(2.0).scope(UNVENTILATED_ROOF_ENTRY),
    InputSpec::number("table3_roof_inboard_r_fraction", "Room-side thermal resistance share in the least insulated path", "Raumseitiger Widerstandsanteil im Gefach mit dem geringsten Widerstand", UnitId::One)
        .range(0.0, 1.0).example(0.20).scope(UNVENTILATED_ROOF_ENTRY),
    InputSpec::text("condensate_element", "Envelope element ID", "Hüllbauteil-ID").example_text("wall").scope(MATERIAL_ENTRY),
    InputSpec::choice("condensate_capillarity", "Capillary absorption at wet interfaces", "Kapillare Wasseraufnahme an Tauwasserebenen", &[
        InputChoice::new("absorbing", "All materials adjoining wet interfaces have Ww ≥ 0.1 kg/(m²√h)", "Alle angrenzenden Baustoffe haben Ww ≥ 0,1 kg/(m²√h)"),
        InputChoice::new("limited", "A material adjoining a wet interface has Ww < 0.1 kg/(m²√h)", "Ein angrenzender Baustoff hat Ww < 0,1 kg/(m²√h)"),
        InputChoice::new("unconfirmed", "Capillary absorption unconfirmed", "Kapillare Wasseraufnahme unbestätigt"),
    ]).example_text("absorbing").scope(MATERIAL_ENTRY),
    InputSpec::choice("condensate_damage", "Durability under the assessed condensate", "Beständigkeit gegenüber dem beurteilten Tauwasser", &[
        InputChoice::new("resistant", "Material durability and function verified", "Beständigkeit und Funktionsfähigkeit der Baustoffe nachgewiesen"),
        InputChoice::new("damage", "Material damage or loss of function identified", "Baustoffschaden oder Funktionsverlust festgestellt"),
        InputChoice::new("unconfirmed", "Material durability unconfirmed", "Beständigkeit der Baustoffe unbestätigt"),
    ]).example_text("resistant").scope(MATERIAL_ENTRY),
    InputSpec::number("condensate_timber_increase", "Maximum timber moisture increase, 0 if none exposed", "Höchste Feuchtezunahme bei Holz, 0 ohne betroffenes Holz", UnitId::Percent)
        .range(0.0, 1000.0).example(0.0).scope(MATERIAL_ENTRY),
    InputSpec::number("condensate_wood_product_increase", "Maximum non-exempt wood-product moisture increase, 0 if none exposed", "Höchste Feuchtezunahme bei nicht ausgenommenen Holzwerkstoffen, 0 ohne solche", UnitId::Percent)
        .range(0.0, 1000.0).example(0.0).scope(MATERIAL_ENTRY),
];

/// 📚️ DIN 4108 design cases for determining the scope and summer boundary conditions of Annex A.
pub struct Din4108PeriodCases;

impl Din4108PeriodCases {
    /// 📋️ Whether the period and construction catalogue owns this entry.
    pub(crate) fn contains(id: &str) -> bool {
        ENTRIES.iter().any(|entry| entry.id == id)
    }
}

impl CaseCatalogue for Din4108PeriodCases {
    fn entries() -> Vec<CaseEntry> {
        ENTRIES.to_vec()
    }

    fn vocabulary() -> &'static [InputSpec] {
        INPUTS
    }

    fn new_document_case(entry: &CaseEntry, id: &str) -> DesignCase {
        let key = match entry.id {
            SITE_ENTRY => "period_conditions",
            SLAB_ENTRY => "slab_build_up",
            ATTIC_ENTRY => "attic_build_up",
            PERIMETER_ENTRY => "perimeter_build_up",
            WALL_ENTRY => "wall_build_up",
            UNVENTILATED_ROOF_ENTRY => "table3_roof_build_up",
            MATERIAL_ENTRY => "condensate_capillarity",
            _ => "roof_exposure",
        };
        let values = CaseValues::default().with_text(key, "unconfirmed");
        DesignCase { id: id.into(), label_en: entry.en.into(), label_de: entry.de.into(), entry_id: entry.id.into(), values }
    }

    fn evaluate(entry: &CaseEntry, annex: AnnexChoice, case: &DesignCase) -> Vec<CheckResult> {
        let check = CheckResult::assess(
            format!("din4108-3.annex-a.basis.{}", case.id),
            STANDARD, ClauseId::new("DIN 4108-3", if entry.id == ROOF_ENTRY { "Annex A" } else { "5" }, entry.clause),
            SubjectRef::new(&case.id, format!("cases[id={}]", case.id), LocalizedCopy::new(entry.en, entry.de)),
            LocalizedCopy::new(entry.en, entry.de),
        ).annex(annex);
        if entry.id == MATERIAL_ENTRY {
            let identified = case.values.texts.get("condensate_element").is_some_and(|id| !id.trim().is_empty());
            let status = if identified && material_scope_value(&case.values).is_some() { CheckStatus::Info } else { CheckStatus::UnverifiedBasis };
            return vec![check.status(status).explanation(LocalizedCopy::new(
                "Link the material evidence to the envelope element ID. Enter the maximum mass-percent moisture increases. Exclude the wood-wool lightweight and multilayer lightweight boards covered by EN 13168 from the 3% check.",
                "Baustoffnachweise mit der Hüllbauteil-ID verknüpfen. Maximale Feuchtezunahmen in Masseprozent eingeben. Holzwolle-Leichtbauplatten und Mehrschicht-Leichtbauplatten nach EN 13168 von der 3-%-Prüfung ausnehmen.",
            )).build()];
        }
        if !matches!(entry.id, SITE_ENTRY | ROOF_ENTRY) {
            let (status, explanation) = construction_case(entry.id, &case.values);
            return vec![check.status(status).explanation(explanation).build()];
        }
        let assessment = if entry.id == SITE_ENTRY {
            match site_scope_value(&case.values) {
                Some(true) => check.status(CheckStatus::Info).explanation(LocalizedCopy::new(
                    "The recorded elevation and room conditions are within the selected scope.",
                    "Die erfasste Standorthöhe und der Raumzustand liegen im gewählten Anwendungsbereich.",
                )),
                Some(false) => check.status(CheckStatus::UnverifiedBasis).explanation(LocalizedCopy::new(
                    "The period method is excluded. Use the assessment under Annex D.",
                    "Das Periodenverfahren ist ausgeschlossen. Nachweis nach Anhang D führen.",
                )),
                None => check.status(CheckStatus::UnverifiedBasis).explanation(LocalizedCopy::new(
                    "Enter the elevation and confirm the room conditions under §5.2.1.",
                    "Standorthöhe eingeben und Raumzustand nach 5.2.1 bestätigen.",
                )),
            }
        } else {
            match roof_exposure_value(&case.values) {
                Some(exposure) if case.values.texts.get("roof_element").is_some_and(|id| !id.trim().is_empty()) => {
                    check.status(CheckStatus::Info).explanation(LocalizedCopy::new(
                        &format!("Evaporation-period interface saturation pressure {} Pa.", exposure.pressure_pa()),
                        &format!("Sättigungsdampfdruck in der Verdunstungsperiode {} Pa.", exposure.pressure_pa()),
                    ))
                }
                _ => check.status(CheckStatus::UnverifiedBasis).explanation(LocalizedCopy::new(
                    "Enter the roof element ID and select its cover and shading.",
                    "Dachbauteil-ID eingeben und Deckung sowie Verschattung auswählen.",
                )),
            }
        };
        vec![assessment.build()]
    }
}

fn construction_case(entry: &str, values: &CaseValues) -> (CheckStatus, LocalizedCopy) {
    let number = |key: &str| values.numbers.get(key).copied().unwrap_or(f64::NAN);
    let confirmed = |key: &str| values.texts.get(key).is_some_and(|value| value == "confirmed");
    if entry == SLAB_ENTRY {
        let sd_m = number("slab_room_side_sd");
        return match (confirmed("slab_build_up"), internal_slab_sd_recommendation(sd_m)) {
            (true, Some(true)) => (CheckStatus::Info, LocalizedCopy::new(
                &format!("Room-side s_d of {sd_m:.2} m meets the 20 m recommendation."),
                &format!("Der raumseitige s_d-Wert von {sd_m:.2} m erfüllt die Empfehlung von 20 m."),
            )),
            (true, Some(false)) => (CheckStatus::Info, LocalizedCopy::new(
                &format!("Room-side s_d of {sd_m:.2} m is below the 20 m recommendation."),
                &format!("Der raumseitige s_d-Wert von {sd_m:.2} m liegt unter der Empfehlung von 20 m."),
            )),
            _ => (CheckStatus::UnverifiedBasis, LocalizedCopy::new(
                "Confirm the construction under §5.3.3.3 and enter the total room-side s_d.",
                "Aufbau nach 5.3.3.3 bestätigen und den gesamten raumseitigen s_d-Wert eingeben.",
            )),
        };
    }
    let checked = |basis, limit| match (basis, limit) {
        (true, Some(true)) => CheckStatus::Pass,
        (true, Some(false)) => CheckStatus::Fail,
        _ => CheckStatus::UnverifiedBasis,
    };
    match entry {
        PERIMETER_ENTRY => {
            let result = perimeter_slab_limit(number("perimeter_inboard_r_fraction"));
            return (checked(confirmed("perimeter_build_up"), result), LocalizedCopy::new(
                "Confirm the slab assembly. The resistance share inward of the waterproofing must be at most 20%.",
                "Bodenplattenaufbau bestätigen. Der Widerstandsanteil raumseitig der Abdichtung darf höchstens 20 % betragen.",
            ));
        }
        WALL_ENTRY => {
            let result = interior_wall_limit(number("wall_insulation_r"), number("wall_inboard_sd"));
            return (checked(confirmed("wall_build_up"), result), LocalizedCopy::new(
                "Confirm the wall assembly. The insulation resistance must be at most 1.0 m²K/W. Above 0.5 m²K/W the combined room-side s_d must be at least 0.5 m.",
                "Wandaufbau bestätigen. Der Dämmwiderstand darf höchstens 1,0 m²K/W betragen. Über 0,5 m²K/W muss der raumseitige Gesamt-s_d-Wert mindestens 0,5 m betragen.",
            ));
        }
        UNVENTILATED_ROOF_ENTRY => {
            let outboard_sd = number("table3_roof_outboard_sd");
            let result = unventilated_roof_limit(outboard_sd, number("table3_roof_inboard_sd"), number("table3_roof_inboard_r_fraction"));
            let explanation = match unventilated_roof_minimum_sd(outboard_sd) {
                Some(minimum) => LocalizedCopy::new(
                    &format!("Confirm the Figure 4 or 5 assembly. Table 3 requires room-side s_d ≥ {minimum:.3} m and the room-side resistance share must be at most 20%."),
                    &format!("Aufbau nach Bild 4 oder 5 bestätigen. Tabelle 3 fordert raumseitig s_d ≥ {minimum:.3} m. Der raumseitige Widerstandsanteil darf höchstens 20 % betragen."),
                ),
                None => LocalizedCopy::new(
                    "Confirm the Figure 4 or 5 assembly and enter the exterior and interior s_d. Table 3 covers exterior s_d up to 2 m.",
                    "Aufbau nach Bild 4 oder 5 bestätigen und den äußeren und inneren s_d-Wert eingeben. Tabelle 3 erfasst äußere s_d-Werte bis 2 m.",
                ),
            };
            return (checked(confirmed("table3_roof_build_up"), result), explanation);
        }
        _ => {}
    }
    let ceiling = AtticCeiling {
        attic_area_m2: number("attic_area"),
        ventilation_openings_m2: number("attic_openings"),
        upper_sd_m: number("attic_upper_sd"),
        insulation_sd_m: number("attic_insulation_sd"),
        airtight_layer_sd_m: number("attic_air_seal_sd"),
        room_side_resistance_fraction: number("attic_inboard_r_fraction"),
    };
    match (confirmed("attic_build_up"), attic_ceiling_limits(ceiling)) {
        (true, Some(limits)) => (if limits.meets_limits { CheckStatus::Pass } else { CheckStatus::Fail }, LocalizedCopy::new(
            &format!("Figure 17 requires openings ≥ {:.3} m², upper s_d ≤ {:.1} m, airtight layer s_d ≥ 10 m and room-side resistance share ≤ 20%.", limits.minimum_openings_m2, limits.maximum_upper_sd_m),
            &format!("Bild 17 fordert Öffnungen ≥ {:.3} m², oberen s_d-Wert ≤ {:.1} m, s_d-Wert der Luftdichtheitsschicht ≥ 10 m und raumseitigen Widerstandsanteil ≤ 20%.", limits.minimum_openings_m2, limits.maximum_upper_sd_m),
        )),
        _ => (CheckStatus::UnverifiedBasis, LocalizedCopy::new(
            "Confirm the Figure 17 construction and enter the opening area and layer values.",
            "Aufbau nach Bild 17 bestätigen und Öffnungsfläche sowie Schichtwerte eingeben.",
        )),
    }
}

fn site_scope_value(values: &CaseValues) -> Option<bool> {
    let elevation = values.numbers.get("site_elevation").copied().and_then(annex_a_elevation_in_scope);
    let known = values.texts.get("period_conditions").map(String::as_str);
    match (elevation, known) {
        (Some(false), _) | (_, Some("excluded")) => Some(false),
        (Some(true), Some("confirmed")) => Some(true),
        _ => None,
    }
}

fn roof_exposure_value(values: &CaseValues) -> Option<AnnexADryingExposure> {
    match values.texts.get("roof_exposure").map(String::as_str) {
        Some("shaded-or-light") => Some(AnnexADryingExposure::ShadedOrLightRoof),
        Some("unshaded-dark") => Some(AnnexADryingExposure::UnshadedDarkRoof),
        _ => None,
    }
}

fn material_scope_value(values: &CaseValues) -> Option<CondensateMaterials> {
    let limited_capillarity = match values.texts.get("condensate_capillarity")?.as_str() {
        "absorbing" => false,
        "limited" => true,
        _ => return None,
    };
    let materials_resist_damage = match values.texts.get("condensate_damage")?.as_str() {
        "resistant" => true,
        "damage" => false,
        _ => return None,
    };
    let timber_increase_pp = *values.numbers.get("condensate_timber_increase")?;
    let wood_product_increase_pp = *values.numbers.get("condensate_wood_product_increase")?;
    if ![timber_increase_pp, wood_product_increase_pp].into_iter().all(|value| value.is_finite() && value >= 0.0) {
        return None;
    }
    Some(CondensateMaterials { limited_capillarity, materials_resist_damage, timber_increase_pp, wood_product_increase_pp })
}

/// 🌦️ Scope and exposure for an envelope check, resolved from authored design cases.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PeriodBasis {
    pub site_scope: Option<bool>,
    pub roof_exposure: Option<AnnexADryingExposure>,
    pub materials: Option<CondensateMaterials>,
}

/// 🗺️ The site case and the selected element's roof case each have one authoritative row.
pub fn period_basis(cases: impl IntoIterator<Item = DesignCase>, element_id: &str) -> PeriodBasis {
    let mut site_cases = 0;
    let mut roof_cases = 0;
    let mut material_cases = 0;
    let mut basis = PeriodBasis::default();
    for case in cases {
        match case.entry_id.as_str() {
            SITE_ENTRY => {
                site_cases += 1;
                basis.site_scope = site_scope_value(&case.values);
            }
            ROOF_ENTRY if case.values.texts.get("roof_element").is_some_and(|id| id == element_id) => {
                roof_cases += 1;
                basis.roof_exposure = roof_exposure_value(&case.values);
            }
            MATERIAL_ENTRY if case.values.texts.get("condensate_element").is_some_and(|id| id == element_id) => {
                material_cases += 1;
                basis.materials = material_scope_value(&case.values);
            }
            _ => {}
        }
    }
    if site_cases != 1 { basis.site_scope = None; }
    if roof_cases != 1 { basis.roof_exposure = None; }
    if material_cases != 1 { basis.materials = None; }
    basis
}
