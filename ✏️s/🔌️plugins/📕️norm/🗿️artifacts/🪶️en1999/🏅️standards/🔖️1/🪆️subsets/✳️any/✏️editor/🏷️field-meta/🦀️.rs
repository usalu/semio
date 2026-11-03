//! 🏷️ EN 1999 FieldSpec lookup — SI display units + en/de labels (longest-prefix + `[]` wildcards).

use semio_s_plugin_norm_app_surface::{lookup_norm_field_meta, NormFieldChoice, FieldSpec, UnitId};

const fn choice(value: &'static str, label_en: &'static str, label_de: &'static str) -> NormFieldChoice {
    NormFieldChoice { value, label_en, label_de }
}

const ANNEX: &[NormFieldChoice] = &[
    choice("En", "EN (CEN recommended)", "EN (CEN-Empfehlung)"),
    choice("De", "Germany (DIN EN / NA)", "Deutschland (DIN EN / NA)"),
];

const RC: &[NormFieldChoice] = &[
    choice("RC1", "RC1 (K_FI 0.9)", "RC1 (K_FI 0,9)"),
    choice("RC2", "RC2 (K_FI 1.0)", "RC2 (K_FI 1,0)"),
    choice("RC3", "RC3 (K_FI 1.1)", "RC3 (K_FI 1,1)"),
];
const SUPPORT: &[NormFieldChoice] = &[
    choice("simplySupported", "Simply supported", "Einfach gelagert"),
    choice("continuous", "Continuous", "Durchlaufend"),
    choice("cantilever", "Cantilever", "Kragträger"),
];
const ACTION_KIND: &[NormFieldChoice] = &[
    choice("permanent", "Permanent (G)", "Ständig (G)"),
    choice("imposed", "Imposed (Q)", "Nutzlast (Q)"),
    choice("snow", "Snow (S)", "Schnee (S)"),
    choice("wind", "Wind (W)", "Wind (W)"),
    choice("temperature", "Temperature (T)", "Temperatur (T)"),
    choice("fire", "Fire", "Brand"),
];
const ACTION_CAT: &[NormFieldChoice] = &[
    choice("selfWeight", "Self-weight", "Eigengewicht"),
    choice("office", "Office (B)", "Büro (B)"),
    choice("snow", "Snow", "Schnee"),
    choice("wind", "Wind", "Wind"),
];
const ACTION_SRC: &[NormFieldChoice] = &[
    choice("udl", "Uniformly distributed line load", "Gleichmäßig verteilte Streckenlast"),
    choice("external", "Externally analysed characteristic effects", "Äußerlich ermittelte charakteristische Schnittgrößen"),
];
const FILLER: &[NormFieldChoice] = &[
    choice("4043", "4043 AlSi5 filler", "4043 AlSi5-Zusatz"),
    choice("5356", "5356 AlMg5 filler", "5356 AlMg5-Zusatz"),
    choice("5183", "5183 AlMg4.5Mn filler", "5183 AlMg4.5Mn-Zusatz"),
];
const ALLOYS: &[NormFieldChoice] = &[
    choice("aw6060-t6", "EN AW-6060-T6", "EN AW-6060-T6"),
    choice("aw6061-t6", "EN AW-6061-T6", "EN AW-6061-T6"),
    choice("aw6063-t6", "EN AW-6063-T6", "EN AW-6063-T6"),
    choice("aw6082-t6", "EN AW-6082-T6", "EN AW-6082-T6"),
    choice("aw5083-o", "EN AW-5083-O", "EN AW-5083-O"),
    choice("aw5083-h111", "EN AW-5083-H111", "EN AW-5083-H111"),
];

const WELD_PROCESS: &[NormFieldChoice] = &[
    choice("mig", "MIG welding", "MIG-Schweißen"),
    choice("tig", "TIG welding", "WIG-Schweißen"),
];
const WELD_DIRECTION: &[NormFieldChoice] = &[
    choice("longitudinal", "Longitudinal weld (HAZ strips)", "Längsnaht (WEZ-Streifen)"),
    choice("transverse", "Transverse weld (HAZ across the width)", "Quernaht (WEZ über die Breite)"),
];
const DETAIL_TYPES: &[NormFieldChoice] = &[
    choice("plain", "Plain material", "Grundwerkstoff"),
    choice("buttWeld", "Butt weld", "Stumpfnaht"),
    choice("filletWeld", "Fillet weld", "Kehlnaht"),
    choice("bolted", "Bolted connection", "Schraubverbindung"),
    choice("riveted", "Riveted connection", "Nietverbindung"),
    choice("other", "Other detail", "Sonstiges Detail"),
];
const PATCH_POSITIONS: &[NormFieldChoice] = &[
    choice("end", "End (near support)", "Ende (nahe Auflager)"),
    choice("interior", "Interior", "Innen"),
];
const STIFFENER_SIDES: &[NormFieldChoice] = &[
    choice("one", "One-sided", "Einseitig"),
    choice("two", "Two-sided", "Zweiseitig"),
];
const FABRICATION_CLASS: &[NormFieldChoice] = &[
    choice("a", "Class A (excellent)", "Klasse A (ausgezeichnet)"),
    choice("b", "Class B (high)", "Klasse B (hoch)"),
    choice("c", "Class C (normal)", "Klasse C (normal)"),
];

const SECTION_KINDS: &[NormFieldChoice] = &[
    choice("extrudedI", "Extruded I-section", "Stranggepresstes I-Profil"),
    choice("channel", "Channel section", "U-Profil"),
    choice("angle", "Angle section", "Winkelprofil"),
    choice("rhs", "Rectangular hollow section", "Rechteck-Hohlprofil"),
    choice("box", "Box section", "Kastenprofil"),
    choice("chs", "Circular hollow section (CHS)", "Kreis-Hohlprofil (CHS)"),
    choice("free", "Free outline (polygon with holes)", "Freier Umriss (Polygon mit Löchern)"),
];

const PRODUCT_FORMS: &[NormFieldChoice] = &[
    choice("sheet", "Sheet, strip, plate", "Blech, Band, Platte"),
    choice("extrudedProfile", "Extruded profile (EP)", "Strangpressprofil (EP)"),
    choice("extrudedTube", "Extruded tube (ET)", "Strangpressrohr (ET)"),
    choice("extrudedBar", "Extruded rod/bar (ER/B)", "Strangpressstange (ER/B)"),
    choice("drawnTube", "Drawn tube (DT)", "Gezogenes Rohr (DT)"),
    choice("forging", "Forging (FO)", "Schmiedestück (FO)"),
];

const CONN_KINDS: &[NormFieldChoice] = &[
    choice("bolted", "Bolted connection", "Schraubenverbindung"),
    choice("welded", "Welded connection", "Schweißverbindung"),
    choice("combined", "Combined bolted and welded", "Kombiniert geschraubt und geschweißt"),
    choice("pinned", "Pin connection (double shear)", "Bolzenverbindung (zweischnittig)"),
];

const OUTLINE_RING: &[NormFieldChoice] = &[
    choice("outer", "Outer boundary", "Außenkontur"),
    choice("hole", "Hole", "Loch"),
];
const FASTENER_FORCE_METHOD: &[NormFieldChoice] = &[
    choice("elastic", "Elastic method", "Elastisches Verfahren"),
    choice("instantaneousCentre", "Instantaneous-centre method", "Momentanpolverfahren"),
];

const fn m(label_en: &'static str, label_de: &'static str, unit: Option<UnitId>, choices: Option<&'static [NormFieldChoice]>) -> FieldSpec {
    match (unit, choices) {
        (Some(unit), _) => FieldSpec::quantity(label_en, label_de, unit),
        (None, Some(choices)) => FieldSpec::choice(label_en, label_de, choices),
        (None, None) => FieldSpec::new(label_en, label_de),
    }
}

const TABLE: &[(&str, FieldSpec)] = &[
    ("annex", m("National annex", "Nationaler Anhang", None, Some(ANNEX))),
    ("reliabilityClass", m("Reliability class RC", "Zuverlässigkeitsklasse RC", None, Some(RC))),
    ("materials[].id", m("Material id", "Material-Id", None, None)),
    ("materials[].designation", m("Alloy / temper", "Legierung / Zustand", None, Some(ALLOYS))),
    ("materials[].productForm", m("Product form (Table 3.2)", "Erzeugnisform (Tabelle 3.2)", None, Some(PRODUCT_FORMS))),
    ("sections[].id", m("Section id", "Querschnitts-Id", None, None)),
    ("sections[].kind", m("Section kind", "Querschnittsart", None, Some(SECTION_KINDS))),
    ("sections[].height", m("Section height h", "Querschnittshöhe h", Some(UnitId::Metre), None)),
    ("sections[].width", m("Section width b", "Querschnittsbreite b", Some(UnitId::Metre), None)),
    ("sections[].flangeThickness", m("Flange thickness t_f", "Flanschdicke t_f", Some(UnitId::Metre), None)),
    ("sections[].webThickness", m("Web thickness t_w", "Stegdicke t_w", Some(UnitId::Metre), None)),
    ("sections[].outerDiameter", m("Outer diameter D", "Außendurchmesser D", Some(UnitId::Metre), None)),
    ("sections[].elements[].id", m("Plate element id", "Blechelement-Id", None, None)),
    ("sections[].elements[].width", m("Plate width b", "Blechbreite b", Some(UnitId::Metre), None)),
    ("sections[].elements[].thickness", m("Plate thickness t", "Blechdicke t", Some(UnitId::Metre), None)),
    ("sections[].elements[].outstand", m("Outstand plate", "Auskragendes Blech", None, None)),
    ("sections[].elements[].welded", m("Welded plate", "Geschweißtes Blech", None, None)),
    ("sections[].elements[].weldPosition", m("Weld position from the element root", "Schweißnahtlage ab Elementanschluss", Some(UnitId::Metre), None)),
    ("members[].id", m("Member id", "Bauteil-Id", None, None)),
    ("members[].sectionId", m("Section reference", "Querschnittsverweis", None, None)),
    ("members[].materialId", m("Material reference", "Materialverweis", None, None)),
    ("members[].length", m("Member length L", "Bauteillänge L", Some(UnitId::Metre), None)),
    ("members[].bucklingLengthY", m("Buckling length L_cr,y", "Knicklänge L_cr,y", Some(UnitId::Metre), None)),
    ("members[].bucklingLengthZ", m("Buckling length L_cr,z", "Knicklänge L_cr,z", Some(UnitId::Metre), None)),
    ("members[].bucklingLengthT", m("Torsional buckling length L_cr,T", "Drillknicklänge L_cr,T", Some(UnitId::Metre), None)),
    ("members[].ltbLength", m("LTB length L_LT", "Biegedrillknicklänge L_LT", Some(UnitId::Metre), None)),
    ("members[].c1", m("LTB factor C_1", "Biegedrillknickfaktor C_1", None, None)),
    ("members[].restrainedLtb", m("LTB fully restrained", "Biegedrillknicken gehalten", None, None)),
    ("sections[].elements[].weldProcess", m("Welding process", "Schweißverfahren", None, Some(WELD_PROCESS))),
    ("sections[].elements[].weldTJunction", m("Weld in a T-junction", "Naht im T-Stoß", None, None)),
    ("sections[].elements[].weldDirection", m("Weld direction", "Schweißrichtung", None, Some(WELD_DIRECTION))),
    ("sections[].elements[].edgeStiffenerDepth", m("Edge stiffener depth", "Randsteifentiefe", Some(UnitId::Metre), None)),
    ("sections[].elements[].edgeStiffenerThickness", m("Edge stiffener thickness", "Randsteifendicke", Some(UnitId::Metre), None)),
    ("members[].bearingLength", m("Stiff bearing length s_s", "Lasteinleitungslänge s_s", Some(UnitId::Metre), None)),
    ("members[].transverseWeld", m("Localized transverse weld", "Örtliche Querschweißnaht", None, None)),
    ("members[].transverseWeldOffset", m("Transverse weld distance x_s from the end", "Quernahtabstand x_s vom Ende", Some(UnitId::Metre), None)),
    ("members[].actions[].torsionK", m("Characteristic torque T_k", "Charakteristisches Torsionsmoment T_k", Some(UnitId::NewtonMetre), None)),
    ("members[].actions[].localLoadK", m("Characteristic patch load F_k", "Charakteristische Einzellast F_k", Some(UnitId::Newton), None)),
    ("connections[].actions[].torsionK", m("Characteristic torque T_k", "Charakteristisches Torsionsmoment T_k", Some(UnitId::NewtonMetre), None)),
    ("connections[].actions[].localLoadK", m("Characteristic patch load F_k", "Charakteristische Einzellast F_k", Some(UnitId::Newton), None)),
    ("coldFormed[].actions[].torsionK", m("Sheet characteristic torque T_k", "Blech charakteristisches Torsionsmoment T_k", Some(UnitId::NewtonMetre), None)),
    ("coldFormed[].actions[].localLoadK", m("Sheet characteristic patch load F_k", "Blech charakteristische Einzellast F_k", Some(UnitId::Newton), None)),
    ("shells[].actions[].torsionK", m("Shell characteristic T_k (unused for membrane)", "Schale charakteristisches T_k (für Membran ungenutzt)", Some(UnitId::NewtonMetre), None)),
    ("shells[].actions[].localLoadK", m("Shell characteristic F_k (unused for membrane)", "Schale charakteristisches F_k (für Membran ungenutzt)", Some(UnitId::Newton), None)),
    ("shells[].welded", m("Circumferential weld in the wall", "Rundnaht in der Wand", None, None)),
    ("shells[].fabricationClass", m("Fabrication quality class", "Ausführungsqualitätsklasse", None, Some(FABRICATION_CLASS))),
    ("placements[].id", m("Placed entity id", "Platziertes Element", None, None)),
    ("placements[].x", m("Placement x", "Platzierung x", Some(UnitId::Metre), None)),
    ("placements[].y", m("Placement y", "Platzierung y", Some(UnitId::Metre), None)),
    ("placements[].z", m("Placement z", "Platzierung z", Some(UnitId::Metre), None)),
    ("placements[].yaw", m("Placement yaw about Z", "Platzierung Gierwinkel um Z", Some(UnitId::Radian), None)),
    ("weldRuns[].id", m("Welded run", "Schweißnaht", None, None)),
    ("weldRuns[].memberId", m("Welded member", "Geschweißtes Bauteil", None, None)),
    ("weldRuns[].elementId", m("Welded plate element", "Geschweißtes Blechelement", None, None)),
    ("weldRuns[].start", m("Run start / position x_s", "Nahtanfang / Lage x_s", Some(UnitId::Metre), None)),
    ("weldRuns[].length", m("Run length", "Nahtlänge", Some(UnitId::Metre), None)),
    ("weldRuns[].offset", m("Weld position across the element", "Nahtlage quer zum Element", Some(UnitId::Metre), None)),
    ("weldRuns[].throat", m("Weld throat a", "Kehlnahtdicke a", Some(UnitId::Metre), None)),
    ("weldRuns[].process", m("Welding process", "Schweißverfahren", None, Some(WELD_PROCESS))),
    ("weldRuns[].tJunction", m("T-junction weld", "Schweißnaht im T-Stoß", None, None)),
    ("weldRuns[].direction", m("Weld direction", "Schweißrichtung", None, Some(WELD_DIRECTION))),
    ("weldRuns[].hazExtent", m("HAZ extent b_haz override (0 = table)", "WEZ-Breite b_haz abweichend (0 = Tabelle)", Some(UnitId::Metre), None)),
    ("outlineVertices[].id", m("Outline vertex", "Umrissknoten", None, None)),
    ("outlineVertices[].sectionId", m("Section", "Querschnitt", None, None)),
    ("outlineVertices[].ring", m("Ring (outer or hole name)", "Ring (outer oder Lochname)", None, Some(OUTLINE_RING))),
    ("outlineVertices[].elementId", m("Plate element of the edge", "Blechelement der Kante", None, None)),
    ("outlineVertices[].y", m("Vertex y", "Knoten y", Some(UnitId::Metre), None)),
    ("outlineVertices[].z", m("Vertex z", "Knoten z", Some(UnitId::Metre), None)),
    ("fatigueCategories[].id", m("Fatigue detail site", "Ermüdungsdetail-Stelle", None, None)),
    ("fatigueCategories[].memberId", m("Member", "Bauteil", None, None)),
    ("fatigueCategories[].elementId", m("Plate element at the detail", "Blechelement am Detail", None, None)),
    ("fatigueCategories[].detailCategory", m("Detail category", "Kerbfallklasse", None, None)),
    ("fatigueCategories[].detailType", m("Detail type", "Detailtyp", None, Some(DETAIL_TYPES))),
    ("fatigueCategories[].locationX", m("Location x along the member", "Lage x am Bauteil", Some(UnitId::Metre), None)),
    ("fatigueCategories[].m1", m("S-N slope m1 (0 = from first block)", "Wöhler-Neigung m1 (0 = vom ersten Block)", None, None)),
    ("fatigueCategories[].m2", m("S-N slope m2 (0 = m1 + 2)", "Wöhler-Neigung m2 (0 = m1 + 2)", None, None)),
    ("patchLoads[].id", m("Web patch load", "Stegeinzellast", None, None)),
    ("patchLoads[].subjectId", m("Member or sheet", "Bauteil oder Blech", None, None)),
    ("patchLoads[].position", m("Load position", "Lastlage", None, Some(PATCH_POSITIONS))),
    ("patchLoads[].x", m("Position x", "Position x", Some(UnitId::Metre), None)),
    ("patchLoads[].bearingLength", m("Bearing length s_s", "Lasteinleitungslänge s_s", Some(UnitId::Metre), None)),
    ("patchLoads[].webHeight", m("Web height h_w (0 = section)", "Steghöhe h_w (0 = Querschnitt)", Some(UnitId::Metre), None)),
    ("patchLoads[].webAngle", m("Web slope φ (0 = 90°)", "Stegneigung φ (0 = 90°)", Some(UnitId::Radian), None)),
    ("patchLoads[].cornerRadius", m("Corner radius r", "Eckradius r", Some(UnitId::Metre), None)),
    ("patchLoads[].load", m("Transverse force F_Ed", "Querkraft F_Ed", Some(UnitId::Newton), None)),
    ("ringStiffeners[].id", m("Ring stiffener", "Ringsteife", None, None)),
    ("ringStiffeners[].shellId", m("Shell", "Schale", None, None)),
    ("ringStiffeners[].x", m("Axial position x", "Axiale Lage x", Some(UnitId::Metre), None)),
    ("ringStiffeners[].depth", m("Ring depth", "Ringhöhe", Some(UnitId::Metre), None)),
    ("ringStiffeners[].thickness", m("Ring thickness", "Ringdicke", Some(UnitId::Metre), None)),
    ("ringStiffeners[].internal", m("Internal ring", "Innenliegender Ring", None, None)),
    ("transverseStiffeners[].id", m("Transverse stiffener", "Quersteife", None, None)),
    ("transverseStiffeners[].girderId", m("Plate girder", "Blechträger", None, None)),
    ("transverseStiffeners[].position", m("Position along the girder", "Lage entlang des Trägers", Some(UnitId::Metre), None)),
    ("transverseStiffeners[].width", m("Stiffener width b_st", "Steifenbreite b_st", Some(UnitId::Metre), None)),
    ("transverseStiffeners[].thickness", m("Stiffener thickness t_st", "Steifendicke t_st", Some(UnitId::Metre), None)),
    ("transverseStiffeners[].sides", m("Sides of the web", "Stegseiten", None, Some(STIFFENER_SIDES))),
    ("members[].actions[].id", m("Action / load-case id", "Einwirkung / Lastfall-Id", None, None)),
    ("members[].actions[].nK", m("Characteristic axial force N_k", "Charakteristische Normalkraft N_k", Some(UnitId::Newton), None)),
    ("members[].actions[].mYK", m("Characteristic moment M_y,k", "Charakteristisches Moment M_y,k", Some(UnitId::NewtonMetre), None)),
    ("connections[].id", m("Connection id", "Anschluss-Id", None, None)),
    ("connections[].memberId", m("Connected member", "Angeschlossenes Bauteil", None, None)),
    ("connections[].materialId", m("Connection material", "Anschluss-Material", None, None)),
    ("connections[].kind", m("Connection kind", "Anschlussart", None, Some(CONN_KINDS))),
    ("connections[].actions[].nK", m("Characteristic connection axial N_k", "Charakteristische Anschluss-Normalkraft N_k", Some(UnitId::Newton), None)),
    ("connections[].bolts.material", m("Bolt grade", "Schraubenfestigkeit", None, None)),
    ("connections[].bolts.diameter", m("Bolt diameter d", "Schraubendurchmesser d", Some(UnitId::Metre), None)),
    ("connections[].bolts.rows", m("Bolt rows", "Schraubenreihen", None, None)),
    ("connections[].bolts.boltsPerRow", m("Bolts per row", "Schrauben je Reihe", None, None)),
    ("connections[].bolts.edgeDistance", m("Edge distance e_1", "Randabstand e_1", Some(UnitId::Metre), None)),
    ("connections[].bolts.pitch", m("Bolt pitch p_1", "Schraubenabstand p_1", Some(UnitId::Metre), None)),
    ("connections[].bolts.gauge", m("Bolt gauge p_2", "Schraubenabstand p_2", Some(UnitId::Metre), None)),
    ("connections[].bolts.plateThickness", m("Connection plate thickness", "Anschlussblechdicke", Some(UnitId::Metre), None)),
    ("connections[].welds.fillerAlloy", m("Weld filler alloy", "Schweißzusatz", None, Some(FILLER))),
    ("connections[].welds.throat", m("Weld throat a", "Kehlnahtdicke a", Some(UnitId::Metre), None)),
    ("connections[].welds.length", m("Weld length ℓ", "Nahtlänge ℓ", Some(UnitId::Metre), None)),
    ("connections[].welds.betaW", m("Correlation factor β_w", "Korrelationsfaktor β_w", None, None)),
    ("connections[].welds.hazExtent", m("HAZ width b_haz override (0 = table)", "WEZ-Breite b_haz abweichend (0 = Tabelle)", Some(UnitId::Metre), None)),
    ("fireScenarios[].id", m("Fire scenario id", "Brandszenario-Id", None, None)),
    ("fireScenarios[].memberId", m("Fire member reference", "Brand-Bauteilverweis", None, None)),
    ("fireScenarios[].thetaA", m("Start temperature of the member θ_a", "Starttemperatur des Bauteils θ_a", Some(UnitId::Celsius), None)),
    ("fireScenarios[].durationS", m("Fire duration", "Branddauer", Some(UnitId::Second), None)),
    ("fatigueDetails[].id", m("Fatigue detail id", "Ermüdungsdetail-Id", None, None)),
    ("fatigueDetails[].memberId", m("Fatigue member reference", "Ermüdungs-Bauteilverweis", None, None)),
    ("fatigueDetails[].deltaSigmaC", m("Reference fatigue strength Δσ_C", "Bezugsschwingfestigkeit Δσ_C", Some(UnitId::Pascal), None)),
    ("fatigueDetails[].deltaSigmaEd", m("Stress range Δσ_Ed", "Spannungsschwingbreite Δσ_Ed", Some(UnitId::Pascal), None)),
    ("fatigueDetails[].m1", m("S–N slope m1", "S–N-Neigung m1", None, None)),
    ("fatigueDetails[].m2", m("S–N slope m2", "S–N-Neigung m2", None, None)),
    ("fatigueDetails[].detailCategory", m("Annex J detail category", "Annex-J-Kerbfall", None, None)),
    ("fatigueDetails[].nCycles", m("Number of cycles N", "Lastwechselzahl N", None, None)),
    ("coldFormed[].id", m("Cold-formed sheet id", "Kaltprofil-Id", None, None)),
    ("coldFormed[].materialId", m("Sheet material reference", "Blech-Materialverweis", None, None)),
    ("coldFormed[].thickness", m("Sheet thickness t", "Blechdicke t", Some(UnitId::Metre), None)),
    ("coldFormed[].width", m("Flat width b", "Flachbreite b", Some(UnitId::Metre), None)),
    ("coldFormed[].span", m("Sheet span L", "Blechstützweite L", Some(UnitId::Metre), None)),
    ("coldFormed[].welded", m("Sheet welded", "Blech geschweißt", None, None)),
    ("coldFormed[].actions[].id", m("Sheet action id", "Blech-Einwirkungs-Id", None, None)),
    ("coldFormed[].actions[].kind", m("Sheet action kind", "Blech-Einwirkungsart", None, Some(ACTION_KIND))),
    ("coldFormed[].actions[].category", m("Sheet action category", "Blech-Einwirkungskategorie", None, Some(ACTION_CAT))),
    ("coldFormed[].actions[].source", m("Sheet action source", "Blech-Einwirkungsquelle", None, Some(ACTION_SRC))),
    ("coldFormed[].actions[].gKLine", m("Sheet g_k line load", "Blech g_k Streckenlast", Some(UnitId::NewtonPerMetre), None)),
    ("coldFormed[].actions[].qKLine", m("Sheet q_k line load", "Blech q_k Streckenlast", Some(UnitId::NewtonPerMetre), None)),
    ("coldFormed[].actions[].nK", m("Sheet characteristic axial N_k", "Blech charakteristische Normalkraft N_k", Some(UnitId::Newton), None)),
    ("coldFormed[].actions[].vYK", m("Sheet characteristic shear V_y,k", "Blech charakteristische Querkraft V_y,k", Some(UnitId::Newton), None)),
    ("coldFormed[].actions[].vZK", m("Sheet characteristic shear V_z,k", "Blech charakteristische Querkraft V_z,k", Some(UnitId::Newton), None)),
    ("coldFormed[].actions[].mYK", m("Sheet characteristic moment M_y,k", "Blech charakteristisches Moment M_y,k", Some(UnitId::NewtonMetre), None)),
    ("coldFormed[].actions[].mZK", m("Sheet characteristic moment M_z,k", "Blech charakteristisches Moment M_z,k", Some(UnitId::NewtonMetre), None)),
    ("shells[].id", m("Shell id", "Schalen-Id", None, None)),
    ("shells[].materialId", m("Shell material reference", "Schalen-Materialverweis", None, None)),
    ("shells[].radius", m("Shell mid-surface radius r", "Schalenradius r", Some(UnitId::Metre), None)),
    ("shells[].thickness", m("Shell thickness t", "Schalendicke t", Some(UnitId::Metre), None)),
    ("shells[].length", m("Shell meridian length L", "Schalenmeridianlänge L", Some(UnitId::Metre), None)),
    ("shells[].actions[].id", m("Shell action id", "Schalen-Einwirkungs-Id", None, None)),
    ("shells[].actions[].kind", m("Shell action kind", "Schalen-Einwirkungsart", None, Some(ACTION_KIND))),
    ("shells[].actions[].category", m("Shell action category", "Schalen-Einwirkungskategorie", None, Some(ACTION_CAT))),
    ("shells[].actions[].source", m("Shell action source", "Schalen-Einwirkungsquelle", None, Some(ACTION_SRC))),
    ("shells[].actions[].gKLine", m("Shell g_k (unused for membrane)", "Schale g_k (für Membran ungenutzt)", Some(UnitId::NewtonPerMetre), None)),
    ("shells[].actions[].qKLine", m("Shell q_k (unused for membrane)", "Schale q_k (für Membran ungenutzt)", Some(UnitId::NewtonPerMetre), None)),
    ("shells[].actions[].nK", m("Characteristic meridional stress σ_x,k", "Charakteristische Meridianspannung σ_x,k", Some(UnitId::Pascal), None)),
    ("shells[].actions[].vYK", m("Shell characteristic V_y,k", "Schale charakteristische V_y,k", Some(UnitId::Newton), None)),
    ("shells[].actions[].vZK", m("Characteristic shear stress τ_k", "Charakteristische Schubspannung τ_k", Some(UnitId::Pascal), None)),
    ("shells[].actions[].mYK", m("Characteristic circumferential stress σ_θ,k", "Charakteristische Umfangsspannung σ_θ,k", Some(UnitId::Pascal), None)),
    ("shells[].actions[].mZK", m("Shell characteristic M_z,k", "Schale charakteristisches M_z,k", Some(UnitId::NewtonMetre), None)),
    ("members[].support", m("Support condition", "Lagerungsart", None, Some(SUPPORT))),
    ("members[].actions[].kind", m("Action kind", "Einwirkungsart", None, Some(ACTION_KIND))),
    ("members[].actions[].category", m("EN 1990 action category", "EN-1990-Einwirkungskategorie", None, Some(ACTION_CAT))),
    ("members[].actions[].source", m("Action source", "Einwirkungsquelle", None, Some(ACTION_SRC))),
    ("members[].actions[].gKLine", m("Characteristic permanent line load g_k", "Charakteristische ständige Streckenlast g_k", Some(UnitId::NewtonPerMetre), None)),
    ("members[].actions[].qKLine", m("Characteristic variable line load q_k", "Charakteristische veränderliche Streckenlast q_k", Some(UnitId::NewtonPerMetre), None)),
    ("members[].actions[].vYK", m("Characteristic shear V_y,k", "Charakteristische Querkraft V_y,k", Some(UnitId::Newton), None)),
    ("members[].actions[].vZK", m("Characteristic shear V_z,k", "Charakteristische Querkraft V_z,k", Some(UnitId::Newton), None)),
    ("members[].actions[].mZK", m("Characteristic moment M_z,k", "Charakteristisches Moment M_z,k", Some(UnitId::NewtonMetre), None)),
    ("connections[].actions[].id", m("Connection load-case id", "Anschluss-Lastfall-Id", None, None)),
    ("connections[].actions[].kind", m("Connection action kind", "Anschluss-Einwirkungsart", None, Some(ACTION_KIND))),
    ("connections[].actions[].category", m("Connection action category", "Anschluss-Einwirkungskategorie", None, Some(ACTION_CAT))),
    ("connections[].actions[].source", m("Connection action source", "Anschluss-Einwirkungsquelle", None, Some(ACTION_SRC))),
    ("connections[].actions[].gKLine", m("Characteristic permanent line load g_k", "Charakteristische ständige Streckenlast g_k", Some(UnitId::NewtonPerMetre), None)),
    ("connections[].actions[].qKLine", m("Characteristic variable line load q_k", "Charakteristische veränderliche Streckenlast q_k", Some(UnitId::NewtonPerMetre), None)),
    ("connections[].actions[].vYK", m("Characteristic shear V_y,k", "Charakteristische Querkraft V_y,k", Some(UnitId::Newton), None)),
    ("connections[].actions[].vZK", m("Characteristic shear V_z,k", "Charakteristische Querkraft V_z,k", Some(UnitId::Newton), None)),
    ("connections[].actions[].mYK", m("Characteristic moment M_y,k", "Charakteristisches Moment M_y,k", Some(UnitId::NewtonMetre), None)),
    ("connections[].actions[].mZK", m("Characteristic moment M_z,k", "Charakteristisches Moment M_z,k", Some(UnitId::NewtonMetre), None)),
    ("connections[].loadEccentricity", m("Load eccentricity to the fastener group centroid (0 = concentric)", "Lastexzentrizität zum Schwerpunkt der Verbindungsmittelgruppe (0 = zentrisch)", Some(UnitId::Metre), None)),
    ("connections[].holeStagger", m("Hole stagger s (0 = aligned hole lines)", "Lochversatz s (0 = fluchtende Lochlinien)", Some(UnitId::Metre), None)),
    ("connections[].forceMethod", m("Fastener force distribution method", "Verfahren der Kraftverteilung auf die Verbindungsmittel", None, Some(FASTENER_FORCE_METHOD))),
];

/// 🏷️ Longest-prefix / `[]` wildcard lookup for the EN 1999 structured inputs editor.
pub fn en1999_field_meta(path: &str) -> Option<FieldSpec> {
    lookup_norm_field_meta(TABLE, path)
}
