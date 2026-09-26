//! 🏷️ EN 1990 NormFieldMeta lookup — SI display units + en/de labels (longest-prefix + `[]` wildcards).

use crate::app_surface::{lookup_norm_field_meta, NormFieldChoice, NormFieldMeta};

const fn choice(value: &'static str, label_en: &'static str, label_de: &'static str) -> NormFieldChoice {
    NormFieldChoice { value, label_en, label_de }
}

const ANNEX: &[NormFieldChoice] = &[choice("En", "EN (CEN)", "EN (CEN)"), choice("De", "Germany (DIN)", "Deutschland (DIN)")];
const CC: &[NormFieldChoice] = &[choice("1", "CC1", "CC1"), choice("2", "CC2", "CC2"), choice("3", "CC3", "CC3")];
const RC: &[NormFieldChoice] = &[choice("1", "RC1", "RC1"), choice("2", "RC2", "RC2"), choice("3", "RC3", "RC3")];
const STRUCTURE_KIND: &[NormFieldChoice] = &[
    choice("building", "Building", "Gebäude"),
    choice("road_bridge", "Road bridge", "Straßenbrücke"),
    choice("footbridge", "Footbridge", "Fußgängerbrücke"),
    choice("rail_bridge", "Railway bridge", "Eisenbahnbrücke"),
];
const DWL_CATEGORY: &[NormFieldChoice] = &[
    choice("1", "Category 1 — temporary structures (10 a)", "Kategorie 1 — temporäre Bauwerke (10 a)"),
    choice("2", "Category 2 — replaceable structural parts (10–25 a)", "Kategorie 2 — austauschbare Tragwerksteile (10–25 a)"),
    choice("3", "Category 3 — agricultural / temporary buildings (15–30 a)", "Kategorie 3 — landwirtschaftliche / behelfsmäßige Gebäude (15–30 a)"),
    choice("4", "Category 4 — building structures / other (50 a)", "Kategorie 4 — Gebäude und sonstige Tragwerke (50 a)"),
    choice("5", "Category 5 — monumental / bridges (100 a)", "Kategorie 5 — monumentale Bauwerke / Brücken (100 a)"),
];
const DSL: &[NormFieldChoice] = &[
    choice("DSL1", "DSL1 — reduced supervision", "DSL1 — verminderte Überwachung"),
    choice("DSL2", "DSL2 — normal supervision", "DSL2 — normale Überwachung"),
    choice("DSL3", "DSL3 — extended supervision", "DSL3 — erweiterte Überwachung"),
];
const IL: &[NormFieldChoice] = &[
    choice("IL1", "IL1 — reduced inspection", "IL1 — verminderte Inspektion"),
    choice("IL2", "IL2 — normal inspection", "IL2 — normale Inspektion"),
    choice("IL3", "IL3 — extended inspection", "IL3 — erweiterte Inspektion"),
];
const PERMANENT_KINDS: &[NormFieldChoice] = &[
    choice("g_sup", "G_sup (unfavourable)", "G_sup (ungünstig)"),
    choice("g_inf", "G_inf (favourable)", "G_inf (günstig)"),
    choice("prestress", "Prestress", "Vorspannung"),
];
const IMPORTANCE: &[NormFieldChoice] = &[
    choice("I", "Importance class I (γ_I = 0.8)", "Bedeutungskategorie I (γ_I = 0,8)"),
    choice("II", "Importance class II (γ_I = 1.0)", "Bedeutungskategorie II (γ_I = 1,0)"),
    choice("III", "Importance class III (γ_I = 1.2)", "Bedeutungskategorie III (γ_I = 1,2)"),
    choice("IV", "Importance class IV (γ_I = 1.4)", "Bedeutungskategorie IV (γ_I = 1,4)"),
];

const VARIABLE_CATEGORIES: &[NormFieldChoice] = &[
    choice("residential", "Residential (A)", "Wohnen (A)"),
    choice("office", "Office (B)", "Büro (B)"),
    choice("congregation", "Congregation (C)", "Versammlung (C)"),
    choice("retail", "Retail (D)", "Verkaufsflächen (D)"),
    choice("storage", "Storage (E)", "Lager (E)"),
    choice("traffic_light", "Traffic — vehicles ≤30 kN (F)", "Verkehr ≤30 kN (F)"),
    choice("traffic_heavy", "Traffic — vehicles >30 kN (G)", "Verkehr >30 kN (G)"),
    choice("roof", "Roofs (H)", "Dächer (H)"),
    choice("snow", "Snow", "Schnee"),
    choice("snow_high", "Snow (high altitude)", "Schnee (hohe Lage)"),
    choice("wind", "Wind", "Wind"),
    choice("temperature", "Temperature", "Temperatur"),
    choice("road_traffic", "Road traffic (A2)", "Straßenverkehr (A2)"),
    choice("footbridge_crowd", "Footbridge crowd (A2)", "Fußgängermenge (A2)"),
    choice("rail_traffic", "Rail traffic (A2)", "Bahnverkehr (A2)"),
    choice("other", "Other", "Sonstige"),
];

const TABLE: &[(&str, NormFieldMeta)] = &[
    ("annex", NormFieldMeta { label_en: "National annex", label_de: "Nationaler Anhang", unit: None, choices: Some(ANNEX) }),
    ("projectId", NormFieldMeta { label_en: "Project id", label_de: "Projekt-Id", unit: None, choices: None }),
    ("structureKind", NormFieldMeta { label_en: "Structure kind", label_de: "Tragwerksart", unit: None, choices: Some(STRUCTURE_KIND) }),
    ("altitudeM", NormFieldMeta { label_en: "Site altitude", label_de: "Standort-Höhe über NN", unit: Some("m"), choices: None }),
    ("consequenceClass", NormFieldMeta { label_en: "Consequence class CC", label_de: "Schadensfolgeklasse CC", unit: None, choices: Some(CC) }),
    ("reliabilityClass", NormFieldMeta { label_en: "Reliability class RC", label_de: "Zuverlässigkeitsklasse RC", unit: None, choices: Some(RC) }),
    ("designWorkingLifeCategory", NormFieldMeta { label_en: "Design working life category", label_de: "Nutzungskategorie", unit: None, choices: Some(DWL_CATEGORY) }),
    ("designWorkingLifeYears", NormFieldMeta { label_en: "Design working life", label_de: "Nutzungsdauer", unit: Some("a"), choices: None }),
    ("referencePeriodYears", NormFieldMeta { label_en: "Reference period T", label_de: "Bezugszeitraum T", unit: Some("a"), choices: None }),
    ("supervisionLevel", NormFieldMeta { label_en: "Supervision level", label_de: "Überwachungsstufe", unit: None, choices: Some(DSL) }),
    ("inspectionLevel", NormFieldMeta { label_en: "Inspection level", label_de: "Inspektionsstufe", unit: None, choices: Some(IL) }),
    ("kFiDeclared", NormFieldMeta { label_en: "Declared K_FI", label_de: "Deklariertes K_FI", unit: None, choices: None }),
    ("betaComputed", NormFieldMeta { label_en: "Reliability index β", label_de: "Zuverlässigkeitsindex β", unit: None, choices: None }),
    ("permanents", NormFieldMeta { label_en: "Permanent actions", label_de: "Ständige Einwirkungen", unit: None, choices: None }),
    ("permanents[].id", NormFieldMeta { label_en: "Permanent action id", label_de: "Id ständige Einwirkung", unit: None, choices: None }),
    ("permanents[].kind", NormFieldMeta { label_en: "Permanent action kind", label_de: "Art ständige Einwirkung", unit: None, choices: Some(PERMANENT_KINDS) }),
    ("permanents[].gk", NormFieldMeta { label_en: "G_k", label_de: "G_k", unit: Some("N"), choices: None }),
    ("variables", NormFieldMeta { label_en: "Variable actions", label_de: "Veränderliche Einwirkungen", unit: None, choices: None }),
    ("variables[].id", NormFieldMeta { label_en: "Variable action id", label_de: "Id veränderliche Einwirkung", unit: None, choices: None }),
    ("variables[].category", NormFieldMeta { label_en: "Category", label_de: "Kategorie", unit: None, choices: Some(VARIABLE_CATEGORIES) }),
    ("variables[].qk", NormFieldMeta { label_en: "Q_k", label_de: "Q_k", unit: Some("N"), choices: None }),
    ("accidentals", NormFieldMeta { label_en: "Accidental actions", label_de: "Außergewöhnliche Einwirkungen", unit: None, choices: None }),
    ("accidentals[].id", NormFieldMeta { label_en: "Accidental action id", label_de: "Id außergewöhnliche Einwirkung", unit: None, choices: None }),
    ("accidentals[].ad", NormFieldMeta { label_en: "A_d (design)", label_de: "A_d (Bemessungswert)", unit: Some("N"), choices: None }),
    ("seismics", NormFieldMeta { label_en: "Seismic actions", label_de: "Seismische Einwirkungen", unit: None, choices: None }),
    ("seismics[].id", NormFieldMeta { label_en: "Seismic action id", label_de: "Id seismische Einwirkung", unit: None, choices: None }),
    ("seismics[].aEk", NormFieldMeta { label_en: "A_Ek (characteristic)", label_de: "A_Ek (charakteristisch)", unit: Some("N"), choices: None }),
    ("seismics[].importanceClass", NormFieldMeta { label_en: "Importance class", label_de: "Bedeutungskategorie", unit: None, choices: Some(IMPORTANCE) }),
    ("members", NormFieldMeta { label_en: "Members", label_de: "Bauteile", unit: None, choices: None }),
    ("members[].id", NormFieldMeta { label_en: "Member id", label_de: "Bauteil-Id", unit: None, choices: None }),
    ("members[].labelEn", NormFieldMeta { label_en: "Label (en)", label_de: "Bezeichnung (en)", unit: None, choices: None }),
    ("members[].labelDe", NormFieldMeta { label_en: "Label (de)", label_de: "Bezeichnung (de)", unit: None, choices: None }),
    ("members[].rdStr", NormFieldMeta { label_en: "R_d STR", label_de: "R_d STR", unit: Some("N"), choices: None }),
    ("members[].rdGeo", NormFieldMeta { label_en: "R_d GEO", label_de: "R_d GEO", unit: Some("N"), choices: None }),
    ("members[].rdEquStab", NormFieldMeta { label_en: "R_d EQU stabilizing", label_de: "R_d EQU stabilisierend", unit: Some("N"), choices: None }),
    ("members[].rdEquDestab", NormFieldMeta { label_en: "R_d EQU destabilizing", label_de: "R_d EQU destabilisierend", unit: Some("N"), choices: None }),
    ("members[].rdFat", NormFieldMeta { label_en: "R_d FAT", label_de: "R_d FAT", unit: Some("N"), choices: None }),
    ("members[].span", NormFieldMeta { label_en: "Span", label_de: "Spannweite", unit: Some("m"), choices: None }),
    ("members[].deflectionW", NormFieldMeta { label_en: "Deflection w", label_de: "Durchbiegung w", unit: Some("m"), choices: None }),
    ("members[].deflectionLimitRatio", NormFieldMeta { label_en: "Deflection limit L/…", label_de: "Durchbiegungsgrenze L/…", unit: None, choices: None }),
    ("members[].vibrationFrequency", NormFieldMeta { label_en: "Natural frequency", label_de: "Eigenfrequenz", unit: Some("Hz"), choices: None }),
    ("members[].vibrationFrequencyMin", NormFieldMeta { label_en: "Min. frequency", label_de: "Mindestfrequenz", unit: Some("Hz"), choices: None }),
    ("bridgeSls", NormFieldMeta { label_en: "Bridge SLS criteria", label_de: "Brücken-GZG-Kriterien", unit: None, choices: None }),
    ("bridgeSls[].id", NormFieldMeta { label_en: "Bridge SLS id", label_de: "Brücken-GZG-Id", unit: None, choices: None }),
    ("bridgeSls[].memberId", NormFieldMeta { label_en: "Member id", label_de: "Bauteil-Id", unit: None, choices: None }),
    ("bridgeSls[].deckAcceleration", NormFieldMeta { label_en: "Deck acceleration", label_de: "Deckbeschleunigung", unit: Some("m/s²"), choices: None }),
    ("bridgeSls[].deckAccelerationLimit", NormFieldMeta { label_en: "Deck acceleration limit", label_de: "Grenzbeschleunigung Deck", unit: Some("m/s²"), choices: None }),
    ("bridgeSls[].deckTwist", NormFieldMeta { label_en: "Deck twist", label_de: "Deckverwindung", unit: Some("rad"), choices: None }),
    ("bridgeSls[].deckTwistLimit", NormFieldMeta { label_en: "Deck twist limit", label_de: "Grenzverwindung Deck", unit: Some("rad"), choices: None }),
    ("bridgeSls[].bridgeDeflection", NormFieldMeta { label_en: "Bridge vertical deflection", label_de: "Vertikale Brückendurchbiegung", unit: Some("m"), choices: None }),
    ("bridgeSls[].bridgeDeflectionLimit", NormFieldMeta { label_en: "Bridge deflection limit", label_de: "Grenzdurchbiegung Brücke", unit: Some("m"), choices: None }),
    ("effects", NormFieldMeta { label_en: "Action influences", label_de: "Einflussbeiwerte", unit: None, choices: None }),
    ("effects[].memberId", NormFieldMeta { label_en: "Member id", label_de: "Bauteil-Id", unit: None, choices: None }),
    ("effects[].actionId", NormFieldMeta { label_en: "Action id", label_de: "Einwirkungs-Id", unit: None, choices: None }),
    ("effects[].influence", NormFieldMeta { label_en: "Influence coefficient", label_de: "Einflussbeiwert", unit: None, choices: None }),
];

/// 🏷️ Exact / `[]`-wildcard / longest-prefix metadata for the EN 1990 basis-of-design subject.
pub fn en1990_field_meta(path: &str) -> Option<NormFieldMeta> {
    lookup_norm_field_meta(TABLE, path)
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
