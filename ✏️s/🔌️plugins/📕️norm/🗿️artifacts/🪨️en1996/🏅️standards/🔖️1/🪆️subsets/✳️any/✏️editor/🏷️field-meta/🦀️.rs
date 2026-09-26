//! 🏷️ EN 1996 NormFieldMeta lookup — SI display units + en/de labels (wildcards for lists).

use crate::app_surface::{lookup_norm_field_meta, NormFieldChoice, NormFieldMeta};

const fn choice(value: &'static str, label_en: &'static str, label_de: &'static str) -> NormFieldChoice {
    NormFieldChoice { value, label_en, label_de }
}

const ANNEX: &[NormFieldChoice] = &[choice("en", "EN (CEN)", "EN (CEN)"), choice("de", "Germany (DIN)", "Deutschland (DIN)")];
const MASONRY_CLASS: &[NormFieldChoice] = &[
    choice("class1", "Cat. I + execution A (γ_M=1.5 DE / 1.5 EN)", "Kat. I + Ausführung A (γ_M=1,5 DE / 1,5 EN)"),
    choice("class2", "Cat. I + execution B (γ_M=1.7 DE / 1.7 EN)", "Kat. I + Ausführung B (γ_M=1,7 DE / 1,7 EN)"),
    choice("class3", "Cat. II + execution A (γ_M=1.7 DE / 2.0 EN)", "Kat. II + Ausführung A (γ_M=1,7 DE / 2,0 EN)"),
    choice("class4", "Cat. II + execution B (γ_M=2.0 DE / 2.2 EN)", "Kat. II + Ausführung B (γ_M=2,0 DE / 2,2 EN)"),
    choice("class5", "Cat. II + execution B conservative (γ_M=2.0 DE / 2.5 EN)", "Kat. II + Ausführung B konservativ (γ_M=2,0 DE / 2,5 EN)"),
];
const DESIGN_SITUATION: &[NormFieldChoice] = &[
    choice("persistent", "Persistent", "Ständig"),
    choice("transient", "Transient", "Vorübergehend"),
    choice("accidental", "Accidental", "Außergewöhnlich"),
    choice("seismic", "Seismic", "Erdbeben"),
];
const WALL_TYPES: &[NormFieldChoice] = &[
    choice("loadBearing", "Load-bearing", "Tragend"),
    choice("shear", "Shear / stiffening", "Aussteifend"),
    choice("nonLoadBearing", "Non-load-bearing", "Nichttragend"),
];
const UNIT_MATERIALS: &[NormFieldChoice] = &[
    choice("clay", "Clay", "Ziegel"),
    choice("calciumSilicate", "Calcium silicate", "Kalksandstein"),
    choice("aerated", "Aerated concrete", "Porenbeton"),
    choice("concrete", "Concrete", "Beton"),
];
const UNIT_GROUPS: &[NormFieldChoice] = &[
    choice("group1", "Group 1", "Gruppe 1"),
    choice("group2", "Group 2", "Gruppe 2"),
    choice("group3", "Group 3", "Gruppe 3"),
    choice("group4", "Group 4", "Gruppe 4"),
];
const MORTAR_TYPES: &[NormFieldChoice] = &[
    choice("generalPurpose", "General purpose", "Normalmörtel"),
    choice("thinLayer", "Thin layer", "Dünnbettmörtel"),
    choice("lightweight", "Lightweight", "Leichtmörtel"),
];
const MORTAR_CLASSES: &[NormFieldChoice] = &[
    choice("M1", "Mortar class M1 (f_m = 1 MPa)", "Mörtelklasse M1 (f_m = 1 MPa)"),
    choice("M2_5", "Mortar class M2,5 (f_m = 2,5 MPa)", "Mörtelklasse M2,5 (f_m = 2,5 MPa)"),
    choice("M5", "Mortar class M5 (f_m = 5 MPa)", "Mörtelklasse M5 (f_m = 5 MPa)"),
    choice("M10", "Mortar class M10 (f_m = 10 MPa)", "Mörtelklasse M10 (f_m = 10 MPa)"),
    choice("M15", "Mortar class M15 (f_m = 15 MPa)", "Mörtelklasse M15 (f_m = 15 MPa)"),
    choice("M20", "Mortar class M20 (f_m = 20 MPa)", "Mörtelklasse M20 (f_m = 20 MPa)"),
];
const EXPOSURES: &[NormFieldChoice] = &[
    choice("Mx1", "MX1 (dry)", "MX1 (trocken)"),
    choice("Mx2", "MX2 (exposed)", "MX2 (bewittert)"),
    choice("Mx3", "MX3 (severe)", "MX3 (stark)"),
    choice("Mx4", "MX4 (salt)", "MX4 (salzhaltig)"),
    choice("Mx5", "MX5 (aggressive)", "MX5 (aggressiv)"),
];
const SUPPORT: &[NormFieldChoice] = &[
    choice("2", "2-sided", "2-seitig gehalten"),
    choice("3", "3-sided", "3-seitig gehalten"),
    choice("4", "4-sided", "4-seitig gehalten"),
];
const BOOL: &[NormFieldChoice] = &[choice("true", "Yes", "Ja"), choice("false", "No", "Nein")];
const IMPOSED_CAT: &[NormFieldChoice] = &[
    choice("A", "Category A — residential", "Kategorie A — Wohnen"),
    choice("B", "Category B — office", "Kategorie B — Büro"),
    choice("C", "Category C — congregation", "Kategorie C — Versammlung"),
    choice("D", "Category D — shopping", "Kategorie D — Verkauf"),
    choice("E", "Category E — storage", "Kategorie E — Lager"),
    choice("H", "Category H — roofs", "Kategorie H — Dächer"),
];

const TABLE: &[(&str, NormFieldMeta)] = &[
    ("annex", NormFieldMeta { label_en: "National annex", label_de: "Nationaler Anhang", unit: None, choices: Some(ANNEX) }),
    ("masonryClass", NormFieldMeta { label_en: "Manufacturing / execution category", label_de: "Herstellungs-/Ausführungskategorie", unit: None, choices: Some(MASONRY_CLASS) }),
    ("designSituation", NormFieldMeta { label_en: "Design situation", label_de: "Bemessungssituation", unit: None, choices: Some(DESIGN_SITUATION) }),
    ("storeys", NormFieldMeta { label_en: "Number of storeys", label_de: "Geschossanzahl", unit: None, choices: None }),
    ("walls", NormFieldMeta { label_en: "Walls", label_de: "Wände", unit: None, choices: None }),
    ("walls[].id", NormFieldMeta { label_en: "Wall id", label_de: "Wand-Id", unit: None, choices: None }),
    ("walls[].labelEn", NormFieldMeta { label_en: "Label (EN)", label_de: "Bezeichnung (EN)", unit: None, choices: None }),
    ("walls[].labelDe", NormFieldMeta { label_en: "Label (DE)", label_de: "Bezeichnung (DE)", unit: None, choices: None }),
    ("walls[].wallType", NormFieldMeta { label_en: "Wall type", label_de: "Wandart", unit: None, choices: Some(WALL_TYPES) }),
    ("walls[].thicknessM", NormFieldMeta { label_en: "Thickness", label_de: "Dicke", unit: Some("m"), choices: None }),
    ("walls[].heightM", NormFieldMeta { label_en: "Clear height", label_de: "Lichte Höhe", unit: Some("m"), choices: None }),
    ("walls[].lengthM", NormFieldMeta { label_en: "Length", label_de: "Länge", unit: Some("m"), choices: None }),
    ("walls[].supportSides", NormFieldMeta { label_en: "Supported sides", label_de: "Gehaltene Seiten", unit: None, choices: Some(SUPPORT) }),
    ("walls[].openings", NormFieldMeta { label_en: "Openings", label_de: "Öffnungen", unit: None, choices: None }),
    ("walls[].openings[].id", NormFieldMeta { label_en: "Opening id", label_de: "Öffnungs-Id", unit: None, choices: None }),
    ("walls[].openings[].widthM", NormFieldMeta { label_en: "Opening width", label_de: "Öffnungsbreite", unit: Some("m"), choices: None }),
    ("walls[].openings[].heightM", NormFieldMeta { label_en: "Opening height", label_de: "Öffnungshöhe", unit: Some("m"), choices: None }),
    ("walls[].openings[].sillHeightM", NormFieldMeta { label_en: "Sill height", label_de: "Brüstungshöhe", unit: Some("m"), choices: None }),
    ("walls[].slabBearingDepthM", NormFieldMeta { label_en: "Slab bearing depth", label_de: "Deckenauflagerungstiefe", unit: Some("m"), choices: None }),
    ("walls[].eccentricityTopM", NormFieldMeta { label_en: "Eccentricity at top", label_de: "Exzentrizität oben", unit: Some("m"), choices: None }),
    ("walls[].eccentricityBottomM", NormFieldMeta { label_en: "Eccentricity at bottom", label_de: "Exzentrizität unten", unit: Some("m"), choices: None }),
    ("walls[].unitGroup", NormFieldMeta { label_en: "Unit group", label_de: "Steingruppe", unit: None, choices: Some(UNIT_GROUPS) }),
    ("walls[].unitMaterial", NormFieldMeta { label_en: "Unit material", label_de: "Steinmaterial", unit: None, choices: Some(UNIT_MATERIALS) }),
    ("walls[].fBPa", NormFieldMeta { label_en: "Normalized unit strength f_b", label_de: "Normierte Steindruckfestigkeit f_b", unit: Some("Pa"), choices: None }),
    ("walls[].unitLengthM", NormFieldMeta { label_en: "Unit length", label_de: "Steinlänge", unit: Some("m"), choices: None }),
    ("walls[].unitWidthM", NormFieldMeta { label_en: "Unit width", label_de: "Steinbreite", unit: Some("m"), choices: None }),
    ("walls[].unitHeightM", NormFieldMeta { label_en: "Unit height", label_de: "Steinhöhe", unit: Some("m"), choices: None }),
    ("walls[].mortarType", NormFieldMeta { label_en: "Mortar type", label_de: "Mörtelart", unit: None, choices: Some(MORTAR_TYPES) }),
    ("walls[].mortarClass", NormFieldMeta { label_en: "Mortar class", label_de: "Mörtelklasse", unit: None, choices: Some(MORTAR_CLASSES) }),
    ("walls[].mortarStrengthPa", NormFieldMeta { label_en: "Mortar strength f_m", label_de: "Mörtelfestigkeit f_m", unit: Some("Pa"), choices: None }),
    ("walls[].bedJointThicknessM", NormFieldMeta { label_en: "Bed-joint thickness", label_de: "Lagerfugendicke", unit: Some("m"), choices: None }),
    ("walls[].reinforced", NormFieldMeta { label_en: "Reinforced", label_de: "Bewehrt", unit: None, choices: Some(BOOL) }),
    ("walls[].asVerticalM2", NormFieldMeta { label_en: "Vertical reinforcement As", label_de: "Vertikale Bewehrung As", unit: Some("m2"), choices: None }),
    ("walls[].asHorizontalM2", NormFieldMeta { label_en: "Horizontal / bed-joint reinforcement As", label_de: "Horizontale / Lagerfugenbewehrung As", unit: Some("m2"), choices: None }),
    ("walls[].fYdPa", NormFieldMeta { label_en: "Reinforcement design yield f_yd", label_de: "Bemessungsstreckgrenze f_yd", unit: Some("Pa"), choices: None }),
    ("walls[].fireReiMin", NormFieldMeta { label_en: "Required fire resistance REI", label_de: "Erforderlicher Feuerwiderstand REI", unit: Some("min"), choices: None }),
    ("walls[].exposure", NormFieldMeta { label_en: "Exposure class", label_de: "Expositionsklasse", unit: None, choices: Some(EXPOSURES) }),
    ("walls[].mu", NormFieldMeta { label_en: "Friction coefficient μ", label_de: "Reibungsbeiwert μ", unit: None, choices: None }),
    ("walls[].densityKgM3", NormFieldMeta { label_en: "Masonry density", label_de: "Rohdichte Mauerwerk", unit: Some("kg/m3"), choices: None }),
    ("walls[].phiInfinity", NormFieldMeta { label_en: "Final creep coefficient φ_∞", label_de: "Endkriechzahl φ_∞", unit: None, choices: None }),
    ("walls[].isBasement", NormFieldMeta { label_en: "Basement wall", label_de: "Kellerwand", unit: None, choices: Some(BOOL) }),
    ("walls[].loadCases", NormFieldMeta { label_en: "Load cases", label_de: "Lastfälle", unit: None, choices: None }),
    ("walls[].loadCases[].id", NormFieldMeta { label_en: "Load case id", label_de: "Lastfall-Id", unit: None, choices: None }),
    ("walls[].loadCases[].designSituation", NormFieldMeta { label_en: "Load-case design situation", label_de: "Bemessungssituation (Lastfall)", unit: None, choices: Some(DESIGN_SITUATION) }),
    ("walls[].loadCases[].imposedCategory", NormFieldMeta { label_en: "Imposed load category", label_de: "Nutzlastkategorie", unit: None, choices: Some(IMPOSED_CAT) }),
    ("walls[].loadCases[].gKSlabN", NormFieldMeta { label_en: "Permanent slab load G_k", label_de: "Ständige Deckenlast G_k", unit: Some("N"), choices: None }),
    ("walls[].loadCases[].qKImposedPa", NormFieldMeta { label_en: "Imposed floor load q_k", label_de: "Nutzlast q_k", unit: Some("Pa"), choices: None }),
    ("walls[].loadCases[].tributaryAreaM2", NormFieldMeta { label_en: "Tributary area", label_de: "Einzugsfläche", unit: Some("m2"), choices: None }),
    ("walls[].loadCases[].slabSpanM", NormFieldMeta { label_en: "Slab clear span", label_de: "Deckenstützweite", unit: Some("m"), choices: None }),
    ("walls[].loadCases[].qKSnowPa", NormFieldMeta { label_en: "Snow load q_k", label_de: "Schneelast q_k", unit: Some("Pa"), choices: None }),
    ("walls[].loadCases[].qPWindPa", NormFieldMeta { label_en: "Peak velocity pressure q_p", label_de: "Böengeschwindigkeitsdruck q_p", unit: Some("Pa"), choices: None }),
    ("walls[].loadCases[].cPe", NormFieldMeta { label_en: "External pressure coefficient c_pe", label_de: "Außendruckbeiwert c_pe", unit: None, choices: None }),
    ("walls[].loadCases[].hKEarthN", NormFieldMeta { label_en: "Earth pressure resultant H_k", label_de: "Erddruckresultierende H_k", unit: Some("N"), choices: None }),
    ("walls[].loadCases[].concentrated", NormFieldMeta { label_en: "Concentrated actions", label_de: "Einzellasten", unit: None, choices: None }),
    ("walls[].loadCases[].concentrated[].id", NormFieldMeta { label_en: "Concentrated action id", label_de: "Einzellast-Id", unit: None, choices: None }),
    ("walls[].loadCases[].concentrated[].forceN", NormFieldMeta { label_en: "Characteristic force F_k", label_de: "Charakteristische Kraft F_k", unit: Some("N"), choices: None }),
    ("walls[].loadCases[].concentrated[].bearingAreaM2", NormFieldMeta { label_en: "Bearing area", label_de: "Aufstandsfläche", unit: Some("m2"), choices: None }),
    ("walls[].loadCases[].concentrated[].bearingLengthM", NormFieldMeta { label_en: "Bearing length", label_de: "Auflagerlänge", unit: Some("m"), choices: None }),
];

/// 🏷️ Exact / `[]`-wildcard / longest-prefix metadata for the EN 1996 masonry subject.
pub fn en1996_field_meta(path: &str) -> Option<NormFieldMeta> {
    lookup_norm_field_meta(TABLE, path)
}
