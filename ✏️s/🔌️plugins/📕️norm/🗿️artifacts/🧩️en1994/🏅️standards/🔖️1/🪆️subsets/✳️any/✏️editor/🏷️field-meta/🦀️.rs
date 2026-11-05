//! 🏷️ EN 1994 FieldSpec — SI units + en/de labels (`[]` wildcards for list leaves).

use semio_s_plugin_norm_app_surface::{case_input_field_meta, lookup_norm_field_meta, NormFieldChoice, FieldSpec, UnitId};

const fn choice(value: &'static str, label_en: &'static str, label_de: &'static str) -> NormFieldChoice {
    NormFieldChoice { value, label_en, label_de }
}

const ANNEX: &[NormFieldChoice] = &[
    choice("En", "Recommended CEN values (EN)", "Empfohlene CEN-Werte (EN)"),
    choice("De", "German national annex (DIN EN)", "Deutscher Nationaler Anhang (DIN EN)"),
];
const RC: &[NormFieldChoice] = &[
    choice("RC1", "RC1 (K_FI 0.9)", "RC1 (K_FI 0,9)"),
    choice("RC2", "RC2 (K_FI 1.0)", "RC2 (K_FI 1,0)"),
    choice("RC3", "RC3 (K_FI 1.1)", "RC3 (K_FI 1,1)"),
];
const STRUCTURE: &[NormFieldChoice] = &[
    choice("building", "Building", "Hochbau"),
    choice("bridge", "Bridge", "Brückenbau"),
];
const FIRE: &[NormFieldChoice] = &[
    choice("unrated", "Unrated", "Ohne Brandschutz"),
    choice("r30", "R30 (30 min)", "R30 (30 min)"),
    choice("r60", "R60 (60 min)", "R60 (60 min)"),
    choice("r90", "R90 (90 min)", "R90 (90 min)"),
    choice("r120", "R120 (120 min)", "R120 (120 min)"),
];
const FATIGUE: &[NormFieldChoice] = &[
    choice("studWelded", "Welded headed stud", "Geschweißter Kopfbolzen"),
    choice("shearConnector", "Shear connector", "Schubdübel"),
    choice("reinforcement", "Reinforcement detail", "Bewehrungsdetail"),
    choice("flangeButtWeld", "Flange butt weld", "Flansch-Stumpfnaht"),
];
const CONSTRUCTION: &[NormFieldChoice] = &[
    choice("propped", "Propped", "mit Hilfsstützen"),
    choice("unpropped", "Unpropped", "ohne Hilfsstützen"),
];
const SUPPORT: &[NormFieldChoice] = &[
    choice("simplySupported", "Simply supported", "Einfeldträger"),
    choice("continuous2Span", "Continuous (2 equal spans)", "Durchlaufträger (2 gleiche Felder)"),
    choice("endSpan", "End span of a continuous beam", "Endfeld eines Durchlaufträgers"),
    choice("interiorSpan", "Interior span of a continuous beam", "Innenfeld eines Durchlaufträgers"),
    choice("cantilever", "Cantilever", "Kragarm"),
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
    choice("any", "Every stage", "Jeder Zustand"),
];
const PROFILE: &[NormFieldChoice] = &[
    choice("trapezoidal", "Trapezoidal", "Trapezprofil"),
    choice("reEntrant", "Re-entrant", "Schwalbenschwanzprofil"),
];
const REBAR_PRODUCT: &[NormFieldChoice] = &[
    choice("hotRolled", "Hot rolled", "Warmgewalzt"),
    choice("coldWorked", "Cold worked", "Kaltverformt"),
];
const BOOL: &[NormFieldChoice] = &[choice("true", "Yes", "Ja"), choice("false", "No", "Nein")];
const COLUMN_KIND: &[NormFieldChoice] = &[
    choice("encased", "Fully encased", "Vollständig einbetoniert"),
    choice("concreteFilled", "Concrete-filled", "Betongefüllt"),
    choice("partiallyEncased", "Partially encased", "Teilweise einbetoniert"),
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
    choice("HEB450", "HEB 450", "HEB 450"),
    choice("HEB500", "HEB 500", "HEB 500"),
    choice("HEB550", "HEB 550", "HEB 550"),
    choice("HEB600", "HEB 600", "HEB 600"),
    choice("HEB650", "HEB 650", "HEB 650"),
    choice("HEB700", "HEB 700", "HEB 700"),
];

const fn m(en: &'static str, de: &'static str) -> FieldSpec {
    FieldSpec::new(en, de)
}
const fn mu(en: &'static str, de: &'static str, unit: UnitId) -> FieldSpec {
    FieldSpec::quantity(en, de, unit)
}
const fn mc(en: &'static str, de: &'static str, choices: &'static [NormFieldChoice]) -> FieldSpec {
    FieldSpec::choice(en, de, choices)
}

const TABLE: &[(&str, FieldSpec)] = &[
    ("annex", mc("National annex", "Nationaler Anhang", ANNEX)),
    ("reliabilityClass", mc("Reliability class RC", "Zuverlässigkeitsklasse RC", RC)),
    ("structureKind", mc("Structure kind", "Tragwerkstyp", STRUCTURE)),
    ("steelFYPa", mu("Steel yield strength f_y", "Stahlstreckgrenze f_y", UnitId::Pascal)),
    ("beams", m("Composite beams", "Verbundträger")),
    ("beams[].id", m("Beam id", "Träger-Id")),
    ("beams[].spanM", mu("Span", "Spannweite", UnitId::Metre)),
    ("beams[].spacingM", mu("Beam spacing", "Trägerabstand", UnitId::Metre)),
    ("beams[].construction", mc("Construction", "Herstellung", CONSTRUCTION)),
    ("beams[].steel", m("Steel section", "Stahlquerschnitt")),
    ("beams[].steel.designation", mc("Section designation", "Querschnittsbezeichnung", STEEL_DESIG)),
    ("beams[].steel.heightM", mu("Section height h", "Querschnittshöhe h", UnitId::Metre)),
    ("beams[].steel.widthM", mu("Section width b", "Querschnittsbreite b", UnitId::Metre)),
    ("beams[].steel.twM", mu("Web thickness t_w", "Stegdicke t_w", UnitId::Metre)),
    ("beams[].steel.tfM", mu("Flange thickness t_f", "Flanschdicke t_f", UnitId::Metre)),
    ("beams[].steel.aM2", mu("Area A", "Querschnittsfläche A", UnitId::SquareMetre)),
    ("beams[].steel.wPlYM3", mu("Plastic modulus W_pl,y", "Plastisches Widerstandsmoment W_pl,y", UnitId::CubicMetre)),
    ("beams[].steel.iYM4", mu("Second moment I_y", "Trägheitsmoment I_y", UnitId::MetreFourth)),
    ("beams[].steel.aVM2", mu("Shear area A_v", "Schubfläche A_v", UnitId::SquareMetre)),
    ("beams[].steel.rM", mu("Fillet radius r", "Ausrundungsradius r", UnitId::Metre)),
    ("beams[].slabThicknessM", mu("Slab thickness", "Plattendicke", UnitId::Metre)),
    ("beams[].concreteFCkPa", mu("Concrete f_ck", "Beton f_ck", UnitId::Pascal)),
    ("beams[].concreteECmPa", mu("Concrete E_cm", "Beton E_cm", UnitId::Pascal)),
    ("beams[].sheeting", m("Profiled sheeting", "Profilblech")),
    ("beams[].sheeting.profile", mc("Sheeting profile", "Profilblechform", PROFILE)),
    ("beams[].sheeting.heightM", mu("Sheeting height", "Profilblechhöhe", UnitId::Metre)),
    ("beams[].sheeting.ribWidthM", mu("Rib width b_0", "Rippenbreite b_0", UnitId::Metre)),
    ("beams[].sheeting.thicknessM", mu("Sheeting thickness", "Profilblechdicke", UnitId::Metre)),
    ("beams[].sheeting.ribsParallelToBeam", mc("Ribs parallel to beam", "Rippen parallel zum Träger", BOOL)),
    ("beams[].transverseAsM2PerM", mu("Transverse reinforcement A_sf/s_f", "Querbewehrung A_sf/s_f", UnitId::SquareMetrePerMetre)),
    ("beams[].ltbLengthM", mu("LTB length L_cr", "Biegedrillknicklänge L_cr", UnitId::Metre)),
    ("beams[].support", mc("Support type", "Lagerung", SUPPORT)),
    ("beams[].asHoggingM2PerM", mu("Hogging reinforcement A_s", "Stützbewehrung A_s", UnitId::SquareMetrePerMetre)),
    ("beams[].barSpacingM", mu("Bar spacing s", "Stababstand s", UnitId::Metre)),
    ("beams[].wkLimitM", mu("Crack-width limit w_k", "Rissbreitengrenze w_k", UnitId::Metre)),
    ("beams[].nCycles", m("Fatigue cycles N", "Lastwechsel N")),
    ("beams[].adjacentSpanM", mu("Adjacent span L_2 (0 = equal span)", "Nachbarfeld L_2 (0 = gleiche Stützweite)", UnitId::Metre)),
    ("beams[].patchBearingLengthM", mu("Patch load bearing length s_s", "Lasteinleitungslänge s_s (Einzellast)", UnitId::Metre)),
    ("beams[].bridgeLanes", m("Notional lanes on the girder (0 = from spacing)", "Rechnerische Fahrstreifen am Träger (0 = aus Trägerabstand)")),
    ("beams[].crackedLengthFraction", m("Cracked length fraction at internal supports (0 = uncracked analysis)", "Gerissener Längenanteil an Innenstützen (0 = ungerissene Berechnung)")),
    ("beams[].studTransverseSpacingM", mu("Stud transverse spacing (0 = not declared)", "Bolzenquerabstand (0 = nicht angegeben)", UnitId::Metre)),
    ("beams[].studEdgeDistanceM", mu("Stud edge distance (0 = not declared)", "Bolzenrandabstand (0 = nicht angegeben)", UnitId::Metre)),
    ("beams[].transverseAsTopM2PerM", mu("Top transverse reinforcement per metre (bridge deck)", "Obere Querbewehrung je Meter (Brückenplatte)", UnitId::SquareMetrePerMetre)),
    ("beams[].endPlateWidthM", mu("End plate width (0 = no end plate)", "Endplattenbreite (0 = keine Endplatte)", UnitId::Metre)),
    ("beams[].endPlateHeightM", mu("End plate height", "Endplattenhöhe", UnitId::Metre)),
    ("beams[].endZoneLengthM", mu("End zone length of the stud group", "Endbereichslänge der Bolzengruppe", UnitId::Metre)),
    ("beams[].reinforcementProduct", mc("Reinforcement product", "Bewehrungsprodukt", REBAR_PRODUCT)),
    ("beams[].actions", m("Characteristic actions", "Charakteristische Einwirkungen")),
    ("beams[].actions[].id", m("Action id", "Einwirkungs-Id")),
    ("beams[].actions[].kind", mc("Action kind", "Einwirkungsart", ACTION_KIND)),
    ("beams[].actions[].category", m("Action category", "Einwirkungskategorie")),
    ("beams[].actions[].stage", mc("Action stage", "Einwirkungsstadium", ACTION_STAGE)),
    ("beams[].actions[].qAreaPa", mu("Characteristic area load q_k", "Charakteristische Flächenlast q_k", UnitId::Pascal)),
    ("beams[].actions[].fKN", mu("Characteristic point force F_k", "Charakteristische Einzellast F_k", UnitId::Newton)),
    ("beams[].actions[].deltaSigmaKPa", mu("Characteristic stress range Δσ_k", "Charakteristische Spannungsschwingbreite Δσ_k", UnitId::Pascal)),
    ("beams[].actions[].deltaTauKPa", mu("Characteristic stud Δτ_k", "Charakteristische Bolzen-Δτ_k", UnitId::Pascal)),
    ("beams[].actions[].xM", mu("Point force position x", "Position der Einzellast x", UnitId::Metre)),
    ("columns", m("Composite columns", "Verbundstützen")),
    ("columns[].id", m("Column id", "Stützen-Id")),
    ("columns[].kind", mc("Column kind", "Stützenart", COLUMN_KIND)),
    ("columns[].lengthM", mu("Length", "Länge", UnitId::Metre)),
    ("columns[].outerSizeM", mu("Outer size D or b", "Außenmaß D bzw. b", UnitId::Metre)),
    ("columns[].wallThicknessM", mu("Wall thickness t", "Wanddicke t", UnitId::Metre)),
    ("columns[].steelAM2", mu("Steel area A_a", "Stahlfläche A_a", UnitId::SquareMetre)),
    ("columns[].steelFYPa", mu("Steel yield strength f_y", "Stahlstreckgrenze f_y", UnitId::Pascal)),
    ("columns[].concreteAM2", mu("Concrete area A_c", "Betonfläche A_c", UnitId::SquareMetre)),
    ("columns[].concreteFCkPa", mu("Concrete f_ck", "Beton f_ck", UnitId::Pascal)),
    ("columns[].reinforcementAsM2", mu("Reinforcement A_s", "Bewehrung A_s", UnitId::SquareMetre)),
    ("columns[].reinforcementFYkPa", mu("Reinforcement f_yk", "Bewehrung f_yk", UnitId::Pascal)),
    ("columns[].iM4", mu("Second moment I", "Trägheitsmoment I", UnitId::MetreFourth)),
    ("columns[].bucklingCurve", mc("Buckling curve", "Knicklinie", BUCKLING)),
    ("columns[].reinforcementProduct", mc("Reinforcement product", "Bewehrungsprodukt", REBAR_PRODUCT)),
    ("columns[].introducedForceN", mu("Introduced force N_intro (0 = from actions)", "Eingeleitete Kraft N_intro (0 = aus den Einwirkungen)", UnitId::Newton)),
    ("columns[].sectionHeightM", mu("Encased I-section height (0 = equivalent section)", "Höhe des einbetonierten I-Profils (0 = Ersatzquerschnitt)", UnitId::Metre)),
    ("columns[].sectionWidthM", mu("Encased I-section flange width", "Flanschbreite des einbetonierten I-Profils", UnitId::Metre)),
    ("columns[].sectionTwM", mu("Encased I-section web thickness", "Stegdicke des einbetonierten I-Profils", UnitId::Metre)),
    ("columns[].sectionTfM", mu("Encased I-section flange thickness", "Flanschdicke des einbetonierten I-Profils", UnitId::Metre)),
    ("columns[].rebarCoverM", mu("Axis distance of the longitudinal bars from the surface", "Achsabstand der Längsbewehrung von der Oberfläche", UnitId::Metre)),
    ("columns[].actions", m("Characteristic actions", "Charakteristische Einwirkungen")),
    ("columns[].actions[].id", m("Action id", "Einwirkungs-Id")),
    ("columns[].actions[].kind", mc("Action kind", "Einwirkungsart", ACTION_KIND)),
    ("columns[].actions[].category", m("Action category", "Einwirkungskategorie")),
    ("columns[].actions[].stage", mc("Action stage", "Einwirkungsstadium", ACTION_STAGE)),
    ("columns[].actions[].nKN", mu("Characteristic axial N_k", "Charakteristische Normalkraft N_k", UnitId::Newton)),
    ("columns[].actions[].mKNm", mu("Characteristic moment M_k", "Charakteristisches Moment M_k", UnitId::NewtonMetre)),
    ("slabs", m("Composite slabs", "Verbunddecken")),
    ("slabs[].id", m("Slab id", "Decken-Id")),
    ("slabs[].spanM", mu("Span", "Spannweite", UnitId::Metre)),
    ("slabs[].support", mc("Support type", "Lagerung", SUPPORT)),
    ("slabs[].sheeting", m("Profiled sheeting", "Profilblech")),
    ("slabs[].sheeting.profile", mc("Sheeting profile", "Profilblechform", PROFILE)),
    ("slabs[].sheeting.heightM", mu("Sheeting height", "Profilblechhöhe", UnitId::Metre)),
    ("slabs[].sheeting.ribWidthM", mu("Rib width b_0", "Rippenbreite b_0", UnitId::Metre)),
    ("slabs[].sheeting.thicknessM", mu("Sheeting thickness", "Profilblechdicke", UnitId::Metre)),
    ("slabs[].sheeting.ribsParallelToBeam", mc("Ribs parallel to beam", "Rippen parallel zum Träger", BOOL)),
    ("slabs[].concreteThicknessM", mu("Concrete thickness", "Betondicke", UnitId::Metre)),
    ("slabs[].fCkPa", mu("Concrete f_ck", "Beton f_ck", UnitId::Pascal)),
    ("slabs[].actions", m("Characteristic actions", "Charakteristische Einwirkungen")),
    ("slabs[].actions[].id", m("Action id", "Einwirkungs-Id")),
    ("slabs[].actions[].kind", mc("Action kind", "Einwirkungsart", ACTION_KIND)),
    ("slabs[].actions[].category", m("Action category", "Einwirkungskategorie")),
    ("slabs[].actions[].stage", mc("Action stage", "Einwirkungsstadium", ACTION_STAGE)),
    ("slabs[].actions[].qAreaPa", mu("Characteristic area load q_k", "Charakteristische Flächenlast q_k", UnitId::Pascal)),
    ("slabs[].actions[].fKN", mu("Characteristic point force F_k", "Charakteristische Einzellast F_k", UnitId::Newton)),
    ("slabs[].actions[].deltaSigmaKPa", mu("Characteristic stress range Δσ_k", "Charakteristische Spannungsschwingbreite Δσ_k", UnitId::Pascal)),
    ("slabs[].actions[].deltaTauKPa", mu("Characteristic stud Δτ_k", "Charakteristische Bolzen-Δτ_k", UnitId::Pascal)),
    ("slabs[].actions[].xM", mu("Point force position x along the strip", "Position der Einzellast x entlang des Streifens", UnitId::Metre)),
    ("slabs[].actions[].yM", mu("Point force position y across the strip", "Position der Einzellast y quer zum Streifen", UnitId::Metre)),
    ("slabs[].mFactor", m("m-factor (m-k)", "m-Faktor (m-k)")),
    ("slabs[].kFactor", m("k-factor (m-k)", "k-Faktor (m-k)")),
    ("slabs[].asM2PerM", mu("Reinforcement A_s", "Bewehrung A_s", UnitId::SquareMetrePerMetre)),
    ("placements", m("Placements", "Platzierungen")),
    ("placements[].id", m("Placed entity id", "Platzierte Element-Id")),
    ("placements[].x", mu("Placement x", "Platzierung x", UnitId::Metre)),
    ("placements[].y", mu("Placement y", "Platzierung y", UnitId::Metre)),
    ("placements[].z", mu("Placement z", "Platzierung z", UnitId::Metre)),
    ("placements[].yaw", mu("Placement yaw about Z", "Platzierung Gierwinkel um Z", UnitId::Radian)),
    ("studs", m("Shear studs", "Kopfbolzendübel")),
    ("studs[].id", m("Stud ID", "Dübel-ID")),
    ("studs[].beamId", m("Beam", "Träger")),
    ("studs[].xM", mu("Position x", "Position x", UnitId::Metre)),
    ("studs[].diameterM", mu("Shank diameter", "Schaftdurchmesser", UnitId::Metre)),
    ("studs[].heightM", mu("Overall height", "Gesamthöhe", UnitId::Metre)),
    ("studs[].fUPa", mu("Ultimate strength", "Zugfestigkeit", UnitId::Pascal)),
    ("fireRating", mc("Fire rating", "Feuerwiderstandsklasse", FIRE)),
    ("insulationThicknessM", mu("Fire insulation thickness", "Brandschutzdämmstärke", UnitId::Metre)),
    ("fatigueDetail", mc("Fatigue detail", "Ermüdungsdetail", FATIGUE)),
    ("cases", m("Design cases", "Bemessungsfälle")),
    ("cases[].id", m("Case id", "Fall-Id")),
    ("cases[].labelEn", m("Case label (English)", "Fallbezeichnung (Englisch)")),
    ("cases[].labelDe", m("Case label (German)", "Fallbezeichnung (Deutsch)")),
    ("cases[].entryId", m("Catalogue entry", "Katalogeintrag")),
    ("cases[].numbers", m("Case numbers", "Fallzahlen")),
    ("cases[].numbers[].id", m("Number id", "Zahlen-Id")),
    ("cases[].numbers[].value", m("Number value", "Zahlenwert")),
    ("cases[].texts", m("Case texts", "Falltexte")),
    ("cases[].texts[].id", m("Text id", "Text-Id")),
    ("cases[].texts[].value", m("Text value", "Textwert")),
];

/// 🏷️ Exact / `[]`-wildcard metadata for the EN 1994 composite subject; the inputs of a design case (`cases[id=…].numbers[id=KEY].value`) take their unit, label, bounds and choices from the typed case-input vocabulary.
fn describe_en1994_field(path: &str) -> Option<FieldSpec> {
    case_input_field_meta(semio_s_artifact_norm_en1994_checks::case_catalogue::case_inputs::VOCABULARY, path).or_else(|| lookup_norm_field_meta(TABLE, path))
}

/// 🛡️ Descriptive field metadata admitted by the family's actual native setter vocabulary.
pub fn en1994_field_meta(path: &str) -> Option<FieldSpec> {
    semio_s_plugin_norm_app_surface::admit_list_meta(
        semio_s_plugin_norm_app_surface::admit_field_meta(
        describe_en1994_field(path),
        semio_s_artifact_norm_en1994_mutations::EDIT_RULES.can_set_field(path),
        ),
        semio_s_artifact_norm_en1994_mutations::EDIT_RULES.can_insert_item(path),
        semio_s_artifact_norm_en1994_mutations::EDIT_RULES.can_remove_item(path, semio_s_artifact_norm_en1994_mutations::ID_REMOVALS.iter().map(|rule| rule.path)),
    )
}
