//! 🏷️ EN 1996 FieldSpec lookup — SI display units + en/de labels (wildcards for lists).

use semio_s_plugin_norm_app_surface::{case_input_field_meta, lookup_norm_field_meta, NormFieldChoice, FieldSpec, UnitId};

const fn choice(value: &'static str, label_en: &'static str, label_de: &'static str) -> NormFieldChoice {
    NormFieldChoice { value, label_en, label_de }
}

const ANNEX: &[NormFieldChoice] = &[
    choice("En", "EN (CEN)", "EN (CEN)"),
    choice("De", "Germany (DIN)", "Deutschland (DIN)"),
];
const MASONRY_CLASS: &[NormFieldChoice] = &[
    choice("Class1", "Cat. I + execution A (γ_M=1.5 DE / 1.5 EN)", "Kat. I + Ausführung A (γ_M=1,5 DE / 1,5 EN)"),
    choice("Class2", "Cat. I + execution B (γ_M=1.7 DE / 1.7 EN)", "Kat. I + Ausführung B (γ_M=1,7 DE / 1,7 EN)"),
    choice("Class3", "Cat. II + execution A (γ_M=1.7 DE / 2.0 EN)", "Kat. II + Ausführung A (γ_M=1,7 DE / 2,0 EN)"),
    choice("Class4", "Cat. II + execution B (γ_M=2.0 DE / 2.2 EN)", "Kat. II + Ausführung B (γ_M=2,0 DE / 2,2 EN)"),
    choice("Class5", "Cat. II + execution B conservative (γ_M=2.0 DE / 2.5 EN)", "Kat. II + Ausführung B konservativ (γ_M=2,0 DE / 2,5 EN)"),
];
const RC: &[NormFieldChoice] = &[
    choice("RC1", "RC1 (K_FI 0.9)", "RC1 (K_FI 0,9)"),
    choice("RC2", "RC2 (K_FI 1.0)", "RC2 (K_FI 1,0)"),
    choice("RC3", "RC3 (K_FI 1.1)", "RC3 (K_FI 1,1)"),
];
const DESIGN_SITUATION: &[NormFieldChoice] = &[
    choice("Persistent", "Persistent", "Ständig"),
    choice("Transient", "Transient", "Vorübergehend"),
    choice("Accidental", "Accidental", "Außergewöhnlich"),
    choice("Seismic", "Seismic", "Erdbeben"),
];
const LOAD_CASE_SITUATION: &[NormFieldChoice] = &[
    choice("persistent", "Persistent", "Ständig"),
    choice("transient", "Transient", "Vorübergehend"),
    choice("accidental", "Accidental", "Außergewöhnlich"),
    choice("seismic", "Seismic", "Erdbeben"),
];
const WALL_TYPES: &[NormFieldChoice] = &[
    choice("LoadBearing", "Load-bearing", "Tragend"),
    choice("Shear", "Shear / stiffening", "Aussteifend"),
    choice("NonLoadBearing", "Non-load-bearing", "Nichttragend"),
];
const UNIT_MATERIALS: &[NormFieldChoice] = &[
    choice("Clay", "Clay", "Ziegel"),
    choice("CalciumSilicate", "Calcium silicate", "Kalksandstein"),
    choice("Aerated", "Aerated concrete", "Porenbeton"),
    choice("Concrete", "Concrete", "Beton"),
];
const UNIT_GROUPS: &[NormFieldChoice] = &[
    choice("Group1", "Group 1", "Gruppe 1"),
    choice("Group2", "Group 2", "Gruppe 2"),
    choice("Group3", "Group 3", "Gruppe 3"),
    choice("Group4", "Group 4", "Gruppe 4"),
];
const MORTAR_TYPES: &[NormFieldChoice] = &[
    choice("GeneralPurpose", "General purpose", "Normalmörtel"),
    choice("ThinLayer", "Thin layer", "Dünnbettmörtel"),
    choice("Lightweight", "Lightweight", "Leichtmörtel"),
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
    choice("a", "Residential (A)", "Wohnen (A)"),
    choice("b", "Office (B)", "Büro (B)"),
    choice("c", "Congregation (C)", "Versammlung (C)"),
    choice("d", "Retail (D)", "Verkauf (D)"),
    choice("e", "Storage (E)", "Lager (E)"),
    choice("f", "Traffic (F)", "Verkehr (F)"),
    choice("g", "Vehicles (G)", "Fahrzeuge (G)"),
    choice("h", "Roofs (H)", "Dächer (H)"),
];

const TIE_KINDS: &[NormFieldChoice] = &[
    choice("WallTie", "Cavity wall tie", "Drahtanker"),
    choice("Anchor", "Wall-floor anchor", "Wand-Decken-Anker"),
    choice("RingTie", "Ring tie", "Ringanker"),
];
const BAR_KINDS: &[NormFieldChoice] = &[
    choice("Anchorage", "Anchorage bar", "Verankerungsstab"),
    choice("Shear", "Shear reinforcement", "Querkraftbewehrung"),
    choice("DeepBeam", "Deep-beam bar", "Wandträgerstab"),
];
const FROST_CLASSES: &[NormFieldChoice] = &[
    choice("Exempt", "No frost exposure", "Keine Frostbeanspruchung"),
    choice("Moderate", "Moderate frost exposure", "Mäßige Frostbeanspruchung"),
    choice("Severe", "Severe frost exposure", "Starke Frostbeanspruchung"),
];
const CHASE_KINDS: &[NormFieldChoice] = &[
    choice("Vertical", "Vertical chase", "Senkrechter Schlitz"),
    choice("Horizontal", "Horizontal chase", "Waagerechter Schlitz"),
    choice("Recess", "Recess", "Aussparung"),
];
const BRACING_SYSTEMS: &[NormFieldChoice] = &[
    choice("Unreinforced", "Unreinforced masonry", "Unbewehrtes Mauerwerk"),
    choice("Confined", "Confined masonry", "Eingefasstes Mauerwerk"),
    choice("Reinforced", "Reinforced masonry", "Bewehrtes Mauerwerk"),
];

const TABLE: &[(&str, FieldSpec)] = &[
    ("annex", FieldSpec::choice("National annex", "Nationaler Anhang", ANNEX)),
    ("reliabilityClass", FieldSpec::choice("Reliability class RC", "Zuverlässigkeitsklasse RC", RC)),
    ("masonryClass", FieldSpec::choice("Manufacturing / execution category", "Herstellungs-/Ausführungskategorie", MASONRY_CLASS)),
    ("designSituation", FieldSpec::choice("Design situation", "Bemessungssituation", DESIGN_SITUATION)),
    ("storeys", FieldSpec::new("Number of storeys", "Geschossanzahl")),
    ("walls", FieldSpec::new("Walls", "Wände")),
    ("walls[].id", FieldSpec::new("Wall id", "Wand-Id")),
    ("walls[].labelEn", FieldSpec::new("Label (EN)", "Bezeichnung (EN)")),
    ("walls[].labelDe", FieldSpec::new("Label (DE)", "Bezeichnung (DE)")),
    ("walls[].wallType", FieldSpec::choice("Wall type", "Wandart", WALL_TYPES)),
    ("walls[].thicknessM", FieldSpec::quantity("Thickness", "Dicke", UnitId::Metre)),
    ("walls[].heightM", FieldSpec::quantity("Clear height", "Lichte Höhe", UnitId::Metre)),
    ("walls[].lengthM", FieldSpec::quantity("Length", "Länge", UnitId::Metre)),
    ("walls[].supportSides", FieldSpec::choice("Supported sides", "Gehaltene Seiten", SUPPORT)),
    ("walls[].openings", FieldSpec::new("Openings", "Öffnungen")),
    ("walls[].openings[].id", FieldSpec::new("Opening id", "Öffnungs-Id")),
    ("walls[].openings[].widthM", FieldSpec::quantity("Opening width", "Öffnungsbreite", UnitId::Metre)),
    ("walls[].openings[].heightM", FieldSpec::quantity("Opening height", "Öffnungshöhe", UnitId::Metre)),
    ("walls[].openings[].sillHeightM", FieldSpec::quantity("Sill height", "Brüstungshöhe", UnitId::Metre)),
    ("walls[].slabBearingDepthM", FieldSpec::quantity("Slab bearing depth", "Deckenauflagerungstiefe", UnitId::Metre)),
    ("walls[].eccentricityTopM", FieldSpec::quantity("Eccentricity at top", "Exzentrizität oben", UnitId::Metre)),
    ("walls[].eccentricityBottomM", FieldSpec::quantity("Eccentricity at bottom", "Exzentrizität unten", UnitId::Metre)),
    ("walls[].unitGroup", FieldSpec::choice("Unit group", "Steingruppe", UNIT_GROUPS)),
    ("walls[].unitMaterial", FieldSpec::choice("Unit material", "Steinmaterial", UNIT_MATERIALS)),
    ("walls[].fBPa", FieldSpec::quantity("Normalized unit strength f_b", "Normierte Steindruckfestigkeit f_b", UnitId::Pascal)),
    ("walls[].unitLengthM", FieldSpec::quantity("Unit length", "Steinlänge", UnitId::Metre)),
    ("walls[].unitWidthM", FieldSpec::quantity("Unit width", "Steinbreite", UnitId::Metre)),
    ("walls[].unitHeightM", FieldSpec::quantity("Unit height", "Steinhöhe", UnitId::Metre)),
    ("walls[].mortarType", FieldSpec::choice("Mortar type", "Mörtelart", MORTAR_TYPES)),
    ("walls[].mortarClass", FieldSpec::choice("Mortar class", "Mörtelklasse", MORTAR_CLASSES)),
    ("walls[].mortarStrengthPa", FieldSpec::quantity("Mortar strength f_m", "Mörtelfestigkeit f_m", UnitId::Pascal)),
    ("walls[].bedJointThicknessM", FieldSpec::quantity("Bed-joint thickness", "Lagerfugendicke", UnitId::Metre)),
    ("walls[].reinforced", FieldSpec::choice("Reinforced", "Bewehrt", BOOL)),
    ("walls[].asVerticalM2", FieldSpec::quantity("Vertical reinforcement As", "Vertikale Bewehrung As", UnitId::SquareMetre)),
    ("walls[].asHorizontalM2", FieldSpec::quantity("Horizontal / bed-joint reinforcement As", "Horizontale / Lagerfugenbewehrung As", UnitId::SquareMetre)),
    ("walls[].fYdPa", FieldSpec::quantity("Reinforcement design yield f_yd", "Bemessungsstreckgrenze f_yd", UnitId::Pascal)),
    ("walls[].fireReiMin", FieldSpec::quantity("Required fire resistance REI", "Erforderlicher Feuerwiderstand REI", UnitId::Minute)),
    ("walls[].exposure", FieldSpec::choice("Exposure class", "Expositionsklasse", EXPOSURES)),
    ("walls[].mu", FieldSpec::new("Friction coefficient μ", "Reibungsbeiwert μ")),
    ("walls[].densityKgM3", FieldSpec::quantity("Masonry density", "Rohdichte Mauerwerk", UnitId::KilogramPerCubicMetre)),
    ("walls[].phiInfinity", FieldSpec::new("Final creep coefficient φ_∞", "Endkriechzahl φ_∞")),
    ("walls[].isBasement", FieldSpec::choice("Basement wall", "Kellerwand", BOOL)),
    ("walls[].loadCases", FieldSpec::new("Load cases", "Lastfälle")),
    ("walls[].loadCases[].id", FieldSpec::new("Load case id", "Lastfall-Id")),
    ("walls[].loadCases[].designSituation", FieldSpec::choice("Load-case design situation", "Bemessungssituation (Lastfall)", LOAD_CASE_SITUATION)),
    ("walls[].loadCases[].imposedCategory", FieldSpec::choice("Imposed load category", "Nutzlastkategorie", IMPOSED_CAT)),
    ("walls[].loadCases[].gKSlabN", FieldSpec::quantity("Permanent slab load G_k", "Ständige Deckenlast G_k", UnitId::Newton)),
    ("walls[].loadCases[].qKImposedPa", FieldSpec::quantity("Imposed floor load q_k", "Nutzlast q_k", UnitId::Pascal)),
    ("walls[].loadCases[].tributaryAreaM2", FieldSpec::quantity("Tributary area", "Einzugsfläche", UnitId::SquareMetre)),
    ("walls[].loadCases[].slabSpanM", FieldSpec::quantity("Slab clear span", "Deckenstützweite", UnitId::Metre)),
    ("walls[].loadCases[].qKSnowPa", FieldSpec::quantity("Snow load q_k", "Schneelast q_k", UnitId::Pascal)),
    ("walls[].loadCases[].qPWindPa", FieldSpec::quantity("Peak velocity pressure q_p", "Böengeschwindigkeitsdruck q_p", UnitId::Pascal)),
    ("walls[].loadCases[].cPe", FieldSpec::new("External pressure coefficient c_pe", "Außendruckbeiwert c_pe")),
    ("walls[].loadCases[].hKEarthN", FieldSpec::quantity("Earth pressure resultant H_k", "Erddruckresultierende H_k", UnitId::Newton)),
    ("walls[].loadCases[].concentrated", FieldSpec::new("Concentrated actions", "Einzellasten")),
    ("walls[].loadCases[].concentrated[].id", FieldSpec::new("Concentrated action id", "Einzellast-Id")),
    ("walls[].loadCases[].concentrated[].forceN", FieldSpec::quantity("Characteristic force F_k", "Charakteristische Kraft F_k", UnitId::Newton)),
    ("walls[].loadCases[].concentrated[].bearingAreaM2", FieldSpec::quantity("Bearing area", "Aufstandsfläche", UnitId::SquareMetre)),
    ("walls[].loadCases[].concentrated[].bearingLengthM", FieldSpec::quantity("Bearing length", "Auflagerlänge", UnitId::Metre)),
    ("walls[].openings[].positionM", FieldSpec::quantity("Opening position x", "Öffnungslage x", UnitId::Metre)),
    ("walls[].loadCases[].concentrated[].distanceA1M", FieldSpec::quantity("Distance a1 to wall end", "Abstand a1 vom Wandende", UnitId::Metre)),
    ("walls[].loadCases[].concentrated[].positionM", FieldSpec::quantity("Load position x", "Lastlage x", UnitId::Metre)),
    ("walls[].piers", FieldSpec::new("Piers", "Pfeiler")),
    ("walls[].piers[].id", FieldSpec::new("Pier id", "Pfeiler-Id")),
    ("walls[].piers[].positionM", FieldSpec::quantity("Pier position x", "Pfeilerlage x", UnitId::Metre)),
    ("walls[].piers[].widthM", FieldSpec::quantity("Pier width", "Pfeilerbreite", UnitId::Metre)),
    ("walls[].lintels", FieldSpec::new("Lintels", "Sturz")),
    ("walls[].lintels[].id", FieldSpec::new("Lintel id", "Sturz-Id")),
    ("walls[].lintels[].positionM", FieldSpec::quantity("Lintel position x", "Sturzlage x", UnitId::Metre)),
    ("walls[].lintels[].spanM", FieldSpec::quantity("Lintel clear span", "Sturz-Lichtweite", UnitId::Metre)),
    ("walls[].lintels[].depthM", FieldSpec::quantity("Lintel depth", "Sturzhöhe", UnitId::Metre)),
    ("walls[].lintels[].bearingLengthM", FieldSpec::quantity("Lintel bearing length", "Sturz-Auflagerlänge", UnitId::Metre)),
    ("walls[].ties", FieldSpec::new("Tie or anchors", "Anker")),
    ("walls[].ties[].id", FieldSpec::new("Tie or anchor id", "Anker-Id")),
    ("walls[].ties[].kind", FieldSpec::choice("Tie type", "Ankerart", TIE_KINDS)),
    ("walls[].ties[].positionM", FieldSpec::quantity("Tie position x", "Ankerlage x", UnitId::Metre)),
    ("walls[].ties[].levelM", FieldSpec::quantity("Tie level y", "Ankerhöhe y", UnitId::Metre)),
    ("walls[].ties[].capacityN", FieldSpec::quantity("Tie design resistance", "Bemessungswiderstand Anker", UnitId::Newton)),
    ("walls[].prestress", FieldSpec::new("Tendons", "Spannglied")),
    ("walls[].prestress[].id", FieldSpec::new("Tendon id", "Spannglied-Id")),
    ("walls[].prestress[].forceN", FieldSpec::quantity("Initial prestressing force P0", "Anfängliche Vorspannkraft P0", UnitId::Newton)),
    ("walls[].prestress[].areaM2", FieldSpec::quantity("Tendon area A_p", "Spannstahlfläche A_p", UnitId::SquareMetre)),
    ("walls[].prestress[].fPkPa", FieldSpec::quantity("Characteristic tendon strength f_pk", "Charakteristische Spannstahlfestigkeit f_pk", UnitId::Pascal)),
    ("walls[].prestress[].positionM", FieldSpec::quantity("Tendon position x", "Spanngliedlage x", UnitId::Metre)),
    ("walls[].prestress[].eccentricityM", FieldSpec::quantity("Tendon eccentricity", "Spanngliedausmitte", UnitId::Metre)),
    ("walls[].prestress[].lossRatio", FieldSpec::new("Prestress loss ratio", "Spannkraftverlust")),
    ("walls[].confined", FieldSpec::new("Confining elements", "Einfassungselement")),
    ("walls[].confined[].id", FieldSpec::new("Confining element id", "Einfassungselement-Id")),
    ("walls[].confined[].positionM", FieldSpec::quantity("Column position x", "Stützenlage x", UnitId::Metre)),
    ("walls[].confined[].widthM", FieldSpec::quantity("Column width", "Stützenbreite", UnitId::Metre)),
    ("walls[].confined[].depthM", FieldSpec::quantity("Column depth", "Stützentiefe", UnitId::Metre)),
    ("walls[].confined[].asM2", FieldSpec::quantity("Column reinforcement A_s", "Stützenbewehrung A_s", UnitId::SquareMetre)),
    ("walls[].flanges", FieldSpec::new("Flange walls", "Flanschwand")),
    ("walls[].flanges[].id", FieldSpec::new("Flange wall id", "Flanschwand-Id")),
    ("walls[].flanges[].lengthM", FieldSpec::quantity("Flange length", "Flanschlänge", UnitId::Metre)),
    ("walls[].flanges[].thicknessM", FieldSpec::quantity("Flange thickness", "Flanschdicke", UnitId::Metre)),
    ("walls[].flanges[].positionM", FieldSpec::quantity("Flange position x", "Flanschlage x", UnitId::Metre)),
    ("walls[].bars", FieldSpec::new("Reinforcing bars", "Bewehrungsstab")),
    ("walls[].bars[].id", FieldSpec::new("Reinforcing bar id", "Bewehrungsstab-Id")),
    ("walls[].bars[].kind", FieldSpec::choice("Bar role", "Stabfunktion", BAR_KINDS)),
    ("walls[].bars[].diameterM", FieldSpec::quantity("Bar diameter", "Stabdurchmesser", UnitId::Metre)),
    ("walls[].bars[].lengthM", FieldSpec::quantity("Provided bar length", "Vorhandene Stablänge", UnitId::Metre)),
    ("walls[].bars[].spacingM", FieldSpec::quantity("Bar spacing", "Stababstand", UnitId::Metre)),
    ("walls[].frostClass", FieldSpec::choice("Frost class", "Frostklasse", FROST_CLASSES)),
    ("walls[].coverM", FieldSpec::quantity("Cover to reinforcement", "Betondeckung bzw. Überdeckung der Bewehrung", UnitId::Metre)),
    ("walls[].deflectionLimit", FieldSpec::new("Deflection limit span/x", "Verformungsgrenze Stützweite/x")),
    ("walls[].soilHeightM", FieldSpec::quantity("Retained soil height", "Höhe der Anschüttung", UnitId::Metre)),
    ("walls[].soilDensityKgM3", FieldSpec::quantity("Soil density", "Wichte des Bodens", UnitId::KilogramPerCubicMetre)),
    ("walls[].isothermDepthM", FieldSpec::quantity("Isotherm depth a_θ", "Isothermentiefe a_θ", UnitId::Metre)),
    ("walls[].planXM", FieldSpec::quantity("Plan position x", "Grundrissposition x", UnitId::Metre)),
    ("walls[].planYM", FieldSpec::quantity("Plan position y", "Grundrissposition y", UnitId::Metre)),
    ("walls[].planAngleDeg", FieldSpec::quantity("Plan angle", "Grundrisswinkel", UnitId::Degree)),
    ("walls[].inPlaneShearN", FieldSpec::quantity("In-plane shear V_Ed", "Scheibenschub V_Ed", UnitId::Newton)),
    ("walls[].inPlaneMomentNm", FieldSpec::quantity("In-plane moment M_Ed", "Scheibenmoment M_Ed", UnitId::NewtonMetre)),
    ("walls[].bracingSystem", FieldSpec::choice("Bracing system", "Aussteifungssystem", BRACING_SYSTEMS)),
    ("walls[].diaphragmStiffnessNPerM", FieldSpec::quantity("Diaphragm in-plane stiffness", "Scheibensteifigkeit der Decke", UnitId::NewtonPerMetre)),
    ("walls[].webSpacingM", FieldSpec::quantity("Web spacing of the shear wall (0 = not given)", "Stegabstand der Schubwand (0 = nicht angegeben)", UnitId::Metre)),
    ("placements", FieldSpec::new("Placements", "Platzierungen")),
    ("placements[].id", FieldSpec::new("Placed entity id", "Platzierte Element-Id")),
    ("placements[].x", FieldSpec::quantity("Placement x", "Platzierung x", UnitId::Metre)),
    ("placements[].y", FieldSpec::quantity("Placement y", "Platzierung y", UnitId::Metre)),
    ("placements[].z", FieldSpec::quantity("Placement z", "Platzierung z", UnitId::Metre)),
    ("placements[].yaw", FieldSpec::quantity("Placement yaw about Z", "Platzierung Gierwinkel um Z", UnitId::Radian)),
    ("chases", FieldSpec::new("Chases and recesses", "Schlitze und Aussparungen")),
    ("chases[].id", FieldSpec::new("Chase id", "Schlitz-Id")),
    ("chases[].wallId", FieldSpec::new("Wall", "Wand")),
    ("chases[].kind", FieldSpec::choice("Chase kind", "Schlitzart", CHASE_KINDS)),
    ("chases[].xM", FieldSpec::quantity("Chase position x", "Schlitzlage x", UnitId::Metre)),
    ("chases[].yM", FieldSpec::quantity("Chase position y", "Schlitzlage y", UnitId::Metre)),
    ("chases[].widthM", FieldSpec::quantity("Chase width", "Schlitzbreite", UnitId::Metre)),
    ("chases[].lengthM", FieldSpec::quantity("Chase length", "Schlitzlänge", UnitId::Metre)),
    ("chases[].depthM", FieldSpec::quantity("Chase depth", "Schlitztiefe", UnitId::Metre)),
    ("movementJoints", FieldSpec::new("Movement joints", "Dehnfugen")),
    ("movementJoints[].id", FieldSpec::new("Movement joint id", "Dehnfugen-Id")),
    ("movementJoints[].wallId", FieldSpec::new("Wall", "Wand")),
    ("movementJoints[].positionM", FieldSpec::quantity("Joint position x", "Fugenlage x", UnitId::Metre)),
    ("cases", FieldSpec::new("Design cases", "Bemessungsfälle")),
    ("cases[].id", FieldSpec::new("Case id", "Fall-Id")),
    ("cases[].labelEn", FieldSpec::new("Case label (English)", "Fallbezeichnung (Englisch)")),
    ("cases[].labelDe", FieldSpec::new("Case label (German)", "Fallbezeichnung (Deutsch)")),
    ("cases[].entryId", FieldSpec::new("Catalogue entry", "Katalogeintrag")),
    ("cases[].numbers", FieldSpec::new("Case numbers", "Fallzahlen")),
    ("cases[].numbers[].id", FieldSpec::new("Number id", "Zahlen-Id")),
    ("cases[].numbers[].value", FieldSpec::new("Number value", "Zahlenwert")),
    ("cases[].texts", FieldSpec::new("Case texts", "Falltexte")),
    ("cases[].texts[].id", FieldSpec::new("Text id", "Text-Id")),
    ("cases[].texts[].value", FieldSpec::new("Text value", "Textwert")),
];

/// 🏷️ Exact / `[]`-wildcard / longest-prefix metadata for the EN 1996 masonry subject; the inputs of a design case (`cases[id=…].numbers[id=KEY].value`) take their unit, label, bounds and choices from the typed case-input vocabulary.
fn describe_en1996_field(path: &str) -> Option<FieldSpec> {
    case_input_field_meta(semio_s_artifact_norm_en1996_schema::checks::masonry::case_inputs::VOCABULARY, path).or_else(|| lookup_norm_field_meta(TABLE, path))
}

/// 🛡️ Descriptive field metadata admitted by the family's actual native setter vocabulary.
pub fn en1996_field_meta(path: &str) -> Option<FieldSpec> {
    semio_s_plugin_norm_app_surface::admit_list_meta(
        semio_s_plugin_norm_app_surface::admit_field_meta(
        describe_en1996_field(path),
        semio_s_artifact_norm_en1996_mutations::EDIT_RULES.can_set_field(path),
        ),
        semio_s_artifact_norm_en1996_mutations::EDIT_RULES.can_insert_item(path),
        semio_s_artifact_norm_en1996_mutations::EDIT_RULES.can_remove_item(path, semio_s_artifact_norm_en1996_mutations::ID_REMOVALS.iter().map(|rule| rule.path)),
    )
}
