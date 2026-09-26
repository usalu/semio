//! 🏷️ EN 1994 NormFieldMeta — SI units + en/de labels (`[]` wildcards for list leaves).

use crate::app_surface::{lookup_norm_field_meta, NormFieldChoice, NormFieldMeta};

const fn choice(value: &'static str, label_en: &'static str, label_de: &'static str) -> NormFieldChoice {
    NormFieldChoice { value, label_en, label_de }
}

const ANNEX: &[NormFieldChoice] = &[
    choice("en", "EN (CEN recommended)", "EN (CEN-Empfehlung)"),
    choice("de", "Germany (DIN NA)", "Deutschland (DIN NA)"),
];
const STRUCTURE: &[NormFieldChoice] = &[
    choice("building", "Building", "Hochbau"),
    choice("bridge", "Bridge", "Brückenbau"),
];
const FIRE: &[NormFieldChoice] = &[
    choice("r30", "R30 (30 min)", "R30 (30 min)"),
    choice("r60", "R60 (60 min)", "R60 (60 min)"),
    choice("r90", "R90 (90 min)", "R90 (90 min)"),
    choice("r120", "R120 (120 min)", "R120 (120 min)"),
];
const FATIGUE: &[NormFieldChoice] = &[
    choice("stud_welded", "Welded headed stud", "Geschweißter Kopfbolzen"),
    choice("shear_connector", "Shear connector", "Schubdübel"),
    choice("reinforcement", "Reinforcement detail", "Bewehrungsdetail"),
    choice("flange_butt_weld", "Flange butt weld", "Flansch-Stumpfnaht"),
];
const CONSTRUCTION: &[NormFieldChoice] = &[
    choice("propped", "Propped", "mit Hilfsstützen"),
    choice("unpropped", "Unpropped", "ohne Hilfsstützen"),
];
const SUPPORT: &[NormFieldChoice] = &[
    choice("simply_supported", "Simply supported", "Einfeldträger"),
    choice("continuous_2_span", "Continuous (2 equal spans)", "Durchlaufträger (2 gleiche Felder)"),
];
const ACTION_KIND: &[NormFieldChoice] = &[
    choice("permanent", "Permanent (G)", "Ständig (G)"),
    choice("imposed", "Imposed (Q)", "Nutzlast (Q)"),
    choice("snow", "Snow (S)", "Schnee (S)"),
    choice("wind", "Wind (W)", "Wind (W)"),
    choice("construction", "Construction load", "Belastung im Bauzustand"),
    choice("fatigue", "Fatigue", "Ermüdung"),
];
const ACTION_STAGE: &[NormFieldChoice] = &[
    choice("construction", "Construction stage", "Herstellungszustand"),
    choice("composite", "Composite stage", "Verbundzustand"),
];
const PROFILE: &[NormFieldChoice] = &[
    choice("trapezoidal", "Trapezoidal", "Trapezprofil"),
    choice("re-entrant", "Re-entrant", "Schwalbenschwanzprofil"),
];
const BOOL: &[NormFieldChoice] = &[choice("true", "Yes", "Ja"), choice("false", "No", "Nein")];
const COLUMN_KIND: &[NormFieldChoice] = &[
    choice("encased", "Fully encased", "Vollständig einbetoniert"),
    choice("concrete_filled", "Concrete-filled", "Betongefüllt"),
    choice("partially_encased", "Partially encased", "Teilweise einbetoniert"),
];
const BUCKLING: &[NormFieldChoice] = &[
    choice("a", "Curve a", "Knicklinie a"),
    choice("b", "Curve b", "Knicklinie b"),
    choice("c", "Curve c", "Knicklinie c"),
    choice("d", "Curve d", "Knicklinie d"),
];
const STEEL_DESIG: &[NormFieldChoice] = &[
    choice("HEB300", "HEB 300", "HEB 300"),
    choice("HEB320", "HEB 320", "HEB 320"),
    choice("HEB340", "HEB 340", "HEB 340"),
    choice("HEB360", "HEB 360", "HEB 360"),
    choice("HEB400", "HEB 400", "HEB 400"),
];

const fn m(en: &'static str, de: &'static str) -> NormFieldMeta {
    NormFieldMeta { label_en: en, label_de: de, unit: None, choices: None }
}
const fn mu(en: &'static str, de: &'static str, unit: &'static str) -> NormFieldMeta {
    NormFieldMeta { label_en: en, label_de: de, unit: Some(unit), choices: None }
}
const fn mc(en: &'static str, de: &'static str, choices: &'static [NormFieldChoice]) -> NormFieldMeta {
    NormFieldMeta { label_en: en, label_de: de, unit: None, choices: Some(choices) }
}

const TABLE: &[(&str, NormFieldMeta)] = &[
    ("annex", mc("National annex", "Nationaler Anhang", ANNEX)),
    ("structureKind", mc("Structure kind", "Tragwerkstyp", STRUCTURE)),
    ("steelFYPa", mu("Steel yield strength f_y", "Stahlstreckgrenze f_y", "Pa")),
    ("beams", m("Composite beams", "Verbundträger")),
    ("beams[].id", m("Beam id", "Träger-Id")),
    ("beams[].spanM", mu("Span", "Spannweite", "m")),
    ("beams[].spacingM", mu("Beam spacing", "Trägerabstand", "m")),
    ("beams[].construction", mc("Construction", "Herstellung", CONSTRUCTION)),
    ("beams[].steel", m("Steel section", "Stahlquerschnitt")),
    ("beams[].steel.designation", mc("Section designation", "Querschnittsbezeichnung", STEEL_DESIG)),
    ("beams[].steel.heightM", mu("Section height h", "Querschnittshöhe h", "m")),
    ("beams[].steel.widthM", mu("Section width b", "Querschnittsbreite b", "m")),
    ("beams[].steel.twM", mu("Web thickness t_w", "Stegdicke t_w", "m")),
    ("beams[].steel.tfM", mu("Flange thickness t_f", "Flanschdicke t_f", "m")),
    ("beams[].steel.aM2", mu("Area A", "Querschnittsfläche A", "m²")),
    ("beams[].steel.wPlYM3", mu("Plastic modulus W_pl,y", "Plastisches Widerstandsmoment W_pl,y", "m³")),
    ("beams[].steel.iYM4", mu("Second moment I_y", "Trägheitsmoment I_y", "m⁴")),
    ("beams[].steel.aVM2", mu("Shear area A_v", "Schubfläche A_v", "m²")),
    ("beams[].slabThicknessM", mu("Slab thickness", "Plattendicke", "m")),
    ("beams[].concreteFCkPa", mu("Concrete f_ck", "Beton f_ck", "Pa")),
    ("beams[].concreteECmPa", mu("Concrete E_cm", "Beton E_cm", "Pa")),
    ("beams[].sheeting", m("Profiled sheeting", "Profilblech")),
    ("beams[].sheeting.profile", mc("Sheeting profile", "Profilblechform", PROFILE)),
    ("beams[].sheeting.heightM", mu("Sheeting height", "Profilblechhöhe", "m")),
    ("beams[].sheeting.ribWidthM", mu("Rib width b_0", "Rippenbreite b_0", "m")),
    ("beams[].sheeting.thicknessM", mu("Sheeting thickness", "Profilblechdicke", "m")),
    ("beams[].sheeting.ribsParallelToBeam", mc("Ribs parallel to beam", "Rippen parallel zum Träger", BOOL)),
    ("beams[].studs", m("Headed studs", "Kopfbolzen")),
    ("beams[].studs.diameterM", mu("Stud diameter", "Bolzendurchmesser", "m")),
    ("beams[].studs.heightM", mu("Stud height", "Bolzenhöhe", "m")),
    ("beams[].studs.fUPa", mu("Stud ultimate strength f_u", "Bolzenzugfestigkeit f_u", "Pa")),
    ("beams[].studs.countPerRib", m("Studs per rib", "Bolzen je Rippe")),
    ("beams[].studs.spacingM", mu("Stud spacing", "Bolzenabstand", "m")),
    ("beams[].studs.totalCount", m("Total stud count", "Bolzenanzahl gesamt")),
    ("beams[].transverseAsM2PerM", mu("Transverse reinforcement A_sf/s_f", "Querbewehrung A_sf/s_f", "m²/m")),
    ("beams[].ltbLengthM", mu("LTB length L_cr", "Biegedrillknicklänge L_cr", "m")),
    ("beams[].support", mc("Support type", "Lagerung", SUPPORT)),
    ("beams[].asHoggingM2PerM", mu("Hogging reinforcement A_s", "Stützbewehrung A_s", "m²/m")),
    ("beams[].barSpacingM", mu("Bar spacing s", "Stababstand s", "m")),
    ("beams[].wkLimitM", mu("Crack-width limit w_k", "Rissbreitengrenze w_k", "m")),
    ("beams[].nCycles", m("Fatigue cycles N", "Lastwechsel N")),
    ("beams[].actions", m("Characteristic actions", "Charakteristische Einwirkungen")),
    ("beams[].actions[].id", m("Action id", "Einwirkungs-Id")),
    ("beams[].actions[].kind", mc("Action kind", "Einwirkungsart", ACTION_KIND)),
    ("beams[].actions[].category", m("Action category", "Einwirkungskategorie")),
    ("beams[].actions[].stage", mc("Action stage", "Einwirkungsstadium", ACTION_STAGE)),
    ("beams[].actions[].qAreaPa", mu("Characteristic area load q_k", "Charakteristische Flächenlast q_k", "Pa")),
    ("beams[].actions[].fKN", mu("Characteristic point force F_k", "Charakteristische Einzellast F_k", "N")),
    ("beams[].actions[].deltaSigmaKPa", mu("Characteristic stress range Δσ_k", "Charakteristische Spannungsschwingbreite Δσ_k", "Pa")),
    ("beams[].actions[].deltaTauKPa", mu("Characteristic stud Δτ_k", "Charakteristische Bolzen-Δτ_k", "Pa")),
    ("columns", m("Composite columns", "Verbundstützen")),
    ("columns[].id", m("Column id", "Stützen-Id")),
    ("columns[].kind", mc("Column kind", "Stützenart", COLUMN_KIND)),
    ("columns[].lengthM", mu("Length", "Länge", "m")),
    ("columns[].outerSizeM", mu("Outer size D or b", "Außenmaß D bzw. b", "m")),
    ("columns[].wallThicknessM", mu("Wall thickness t", "Wanddicke t", "m")),
    ("columns[].steelAM2", mu("Steel area A_a", "Stahlfläche A_a", "m²")),
    ("columns[].steelFYPa", mu("Steel yield strength f_y", "Stahlstreckgrenze f_y", "Pa")),
    ("columns[].concreteAM2", mu("Concrete area A_c", "Betonfläche A_c", "m²")),
    ("columns[].concreteFCkPa", mu("Concrete f_ck", "Beton f_ck", "Pa")),
    ("columns[].reinforcementAsM2", mu("Reinforcement A_s", "Bewehrung A_s", "m²")),
    ("columns[].reinforcementFYkPa", mu("Reinforcement f_yk", "Bewehrung f_yk", "Pa")),
    ("columns[].iM4", mu("Second moment I", "Trägheitsmoment I", "m⁴")),
    ("columns[].bucklingCurve", mc("Buckling curve", "Knicklinie", BUCKLING)),
    ("columns[].actions", m("Characteristic actions", "Charakteristische Einwirkungen")),
    ("columns[].actions[].id", m("Action id", "Einwirkungs-Id")),
    ("columns[].actions[].kind", mc("Action kind", "Einwirkungsart", ACTION_KIND)),
    ("columns[].actions[].category", m("Action category", "Einwirkungskategorie")),
    ("columns[].actions[].stage", mc("Action stage", "Einwirkungsstadium", ACTION_STAGE)),
    ("columns[].actions[].nKN", mu("Characteristic axial N_k", "Charakteristische Normalkraft N_k", "N")),
    ("columns[].actions[].mKNm", mu("Characteristic moment M_k", "Charakteristisches Moment M_k", "N·m")),
    ("slabs", m("Composite slabs", "Verbunddecken")),
    ("slabs[].id", m("Slab id", "Decken-Id")),
    ("slabs[].spanM", mu("Span", "Spannweite", "m")),
    ("slabs[].support", mc("Support type", "Lagerung", SUPPORT)),
    ("slabs[].sheeting", m("Profiled sheeting", "Profilblech")),
    ("slabs[].sheeting.profile", mc("Sheeting profile", "Profilblechform", PROFILE)),
    ("slabs[].sheeting.heightM", mu("Sheeting height", "Profilblechhöhe", "m")),
    ("slabs[].sheeting.ribWidthM", mu("Rib width b_0", "Rippenbreite b_0", "m")),
    ("slabs[].sheeting.thicknessM", mu("Sheeting thickness", "Profilblechdicke", "m")),
    ("slabs[].sheeting.ribsParallelToBeam", mc("Ribs parallel to beam", "Rippen parallel zum Träger", BOOL)),
    ("slabs[].concreteThicknessM", mu("Concrete thickness", "Betondicke", "m")),
    ("slabs[].fCkPa", mu("Concrete f_ck", "Beton f_ck", "Pa")),
    ("slabs[].actions", m("Characteristic actions", "Charakteristische Einwirkungen")),
    ("slabs[].actions[].id", m("Action id", "Einwirkungs-Id")),
    ("slabs[].actions[].kind", mc("Action kind", "Einwirkungsart", ACTION_KIND)),
    ("slabs[].actions[].category", m("Action category", "Einwirkungskategorie")),
    ("slabs[].actions[].stage", mc("Action stage", "Einwirkungsstadium", ACTION_STAGE)),
    ("slabs[].actions[].qAreaPa", mu("Characteristic area load q_k", "Charakteristische Flächenlast q_k", "Pa")),
    ("slabs[].actions[].fKN", mu("Characteristic point force F_k", "Charakteristische Einzellast F_k", "N")),
    ("slabs[].actions[].deltaSigmaKPa", mu("Characteristic stress range Δσ_k", "Charakteristische Spannungsschwingbreite Δσ_k", "Pa")),
    ("slabs[].actions[].deltaTauKPa", mu("Characteristic stud Δτ_k", "Charakteristische Bolzen-Δτ_k", "Pa")),
    ("slabs[].mFactor", m("m-factor (m-k)", "m-Faktor (m-k)")),
    ("slabs[].kFactor", m("k-factor (m-k)", "k-Faktor (m-k)")),
    ("slabs[].asM2PerM", mu("Reinforcement A_s", "Bewehrung A_s", "m²/m")),
    ("fireRating", mc("Fire rating", "Feuerwiderstandsklasse", FIRE)),
    ("insulationThicknessM", mu("Fire insulation thickness", "Brandschutzdämmstärke", "m")),
    ("fatigueDetail", mc("Fatigue detail", "Ermüdungsdetail", FATIGUE)),
];

/// 🏷️ Exact / `[]`-wildcard metadata for the EN 1994 composite subject.
pub fn en1994_field_meta(path: &str) -> Option<NormFieldMeta> {
    lookup_norm_field_meta(TABLE, path)
}
