//! 🏷️ EN 1992 FieldSpec — SI units + en/de labels (`[]` wildcards for list leaves).

use semio_s_plugin_norm_app_surface::{case_input_field_meta, NormFieldChoice, FieldSpec, UnitId};

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

const EXPOSURE: &[NormFieldChoice] = &[
    choice("X0", "X0", "X0"),
    choice("Xc1", "XC1", "XC1"),
    choice("Xc2", "XC2", "XC2"),
    choice("Xc3", "XC3", "XC3"),
    choice("Xc4", "XC4", "XC4"),
    choice("Xd1", "XD1", "XD1"),
    choice("Xd2", "XD2", "XD2"),
    choice("Xd3", "XD3", "XD3"),
    choice("Xs1", "XS1", "XS1"),
    choice("Xs2", "XS2", "XS2"),
    choice("Xs3", "XS3", "XS3"),
    choice("Xf1", "XF1", "XF1"),
    choice("Xf2", "XF2", "XF2"),
    choice("Xf3", "XF3", "XF3"),
    choice("Xf4", "XF4", "XF4"),
    choice("Xa1", "XA1", "XA1"),
    choice("Xa2", "XA2", "XA2"),
    choice("Xa3", "XA3", "XA3"),
];

const MEMBER_KIND: &[NormFieldChoice] = &[
    choice("Beam", "Beam", "Balken"),
    choice("Slab", "Slab", "Platte"),
    choice("Column", "Column", "Stütze"),
    choice("Wall", "Wall", "Wand"),
    choice("FlatSlab", "Flat slab", "Flachdecke"),
    choice("RibbedSlab", "Ribbed slab", "Rippendecke"),
    choice("TensionMember", "Tension member", "Zugglied"),
    choice("Bridge", "Bridge", "Brücke"),
    choice("LiquidRetaining", "Liquid-retaining structure", "Flüssigkeitsbehälter"),
];

const SUPPORT: &[NormFieldChoice] = &[
    choice("SimplySupported", "Simply supported", "Einfeldträger"),
    choice("Continuous", "Continuous", "Durchlaufträger"),
    choice("Cantilever", "Cantilever", "Kragarm"),
    choice("Fixed", "Fixed at both ends", "Beidseitig eingespannt"),
];

const FIRE: &[NormFieldChoice] = &[
    choice("R30", "R30", "R30"),
    choice("R60", "R60", "R60"),
    choice("R90", "R90", "R90"),
    choice("R120", "R120", "R120"),
];

const TIGHTNESS: &[NormFieldChoice] = &[
    choice("Tc0", "Tightness class 0", "Dichtheitsklasse 0"),
    choice("Tc1", "Tightness class 1", "Dichtheitsklasse 1"),
    choice("Tc2", "Tightness class 2", "Dichtheitsklasse 2"),
];

const BOOL: &[NormFieldChoice] = &[choice("true", "Yes", "Ja"), choice("false", "No", "Nein")];

const BAR_POS: &[NormFieldChoice] = &[
    choice("Bottom", "Bottom / tension", "Unten / Zug"),
    choice("Top", "Top / compression", "Oben / Druck"),
    choice("Tension", "Tension", "Zug"),
    choice("Compression", "Compression", "Druck"),
];

const BAR_BEND: &[NormFieldChoice] = &[
    choice("Auto", "Automatic (hooks only where anchorage needs them)", "Automatisch (Haken nur bei Verankerungsbedarf)"),
    choice("Straight", "Straight", "Gerade"),
    choice("Hook", "Hook at one end", "Haken an einem Ende"),
    choice("UHook", "Hooks at both ends", "Haken an beiden Enden"),
];

const ACTION_KIND: &[NormFieldChoice] = &[
    choice("Permanent", "Permanent (G)", "Ständig (G)"),
    choice("Imposed", "Imposed (Q)", "Nutzlast (Q)"),
    choice("Snow", "Snow (S)", "Schnee (S)"),
    choice("Wind", "Wind (W)", "Wind (W)"),
    choice("Accidental", "Accidental (A)", "Außergewöhnlich (A)"),
    choice("Prestress", "Prestress (P)", "Vorspannung (P)"),
];
const ACTION_CATEGORY: &[NormFieldChoice] = &[
    choice("SelfWeight", "Self-weight", "Eigengewicht"),
    choice("Office", "Office", "Büro"),
    choice("Residential", "Residential", "Wohnen"),
    choice("Congregation", "Congregation", "Versammlung"),
    choice("Shopping", "Shopping", "Verkauf"),
    choice("Storage", "Storage", "Lager"),
    choice("Snow", "Snow (CEN member states up to 1000 m)", "Schnee (bis 1000 m)"),
    choice("SnowHigh", "Snow (above 1000 m)", "Schnee (über 1000 m)"),
    choice("Wind", "Wind", "Wind"),
    choice("Temperature", "Temperature", "Temperatur"),
    choice("Accidental", "Accidental", "Außergewöhnlich"),
];
const ACTION_SOURCE: &[NormFieldChoice] = &[
    choice("Udl", "Derive from line load (span/support)", "Aus Streckenlast (Spannweite/Lagerung)"),
    choice("Point", "Derive from midspan point load", "Aus Einzellast in Feldmitte"),
    choice("External", "External characteristic internal forces", "Äußere charakteristische Schnittgrößen"),
];
const COLUMN_POS: &[NormFieldChoice] = &[
    choice("Interior", "Interior column", "Innenstütze"),
    choice("Edge", "Edge column", "Randstütze"),
    choice("Corner", "Corner column", "Eckstütze"),
    choice("WallEnd", "Wall end", "Wandende"),
];

const DUCTILITY: &[NormFieldChoice] = &[
    choice("a", "Class A (normal ductility)", "Klasse A (normale Duktilität)"),
    choice("b", "Class B (high ductility)", "Klasse B (hohe Duktilität)"),
    choice("c", "Class C (very high ductility)", "Klasse C (sehr hohe Duktilität)"),
];
const SLAB_SYSTEM: &[NormFieldChoice] = &[
    choice("OneWay", "One-way slab (Table 5.8)", "Einachsig (Tabelle 5.8)"),
    choice("TwoWay", "Two-way slab (Table 5.9)", "Zweiachsig (Tabelle 5.9)"),
];

const BETA_METHOD: &[NormFieldChoice] = &[
    choice("Simplified", "Simplified (fixed β of 6.4.3(6))", "Vereinfacht (festes β nach 6.4.3(6))"),
    choice("UnbalancedMoments", "From unbalanced moments (6.4.3(3)-(5))", "Aus Stützenmomenten (6.4.3(3)-(5))"),
];
const BOND: &[NormFieldChoice] = &[
    choice("Good", "Good bond conditions", "Gute Verbundbedingungen"),
    choice("Poor", "Poor bond conditions", "Schlechte Verbundbedingungen"),
];

const CEMENT_CLASS: &[NormFieldChoice] = &[
    choice("S", "Class S (slow hardening)", "Klasse S (langsam erhärtend)"),
    choice("N", "Class N (normal hardening)", "Klasse N (normal erhärtend)"),
    choice("R", "Class R (rapid hardening)", "Klasse R (rasch erhärtend)"),
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
    ("title", m("Title", "Titel", None, None)),
    ("designWorkingLifeYears", m("Design working life", "Geplante Nutzungsdauer", Some(UnitId::Year), None)),
    ("deltaCDev", m("Cover allowance Δc_dev", "Vorhaltemaß Δc_dev", Some(UnitId::Metre), None)),
    ("cementType", m("Cement type class", "Zementart-Klasse", None, Some(CEMENT_CLASS))),
    ("concreteGrades", m("Concrete grades", "Betonfestigkeitsklassen", None, None)),
    ("concreteGrades[].id", m("Grade id", "Klassen-Id", None, None)),
    ("concreteGrades[].name", m("Grade name", "Klassenname", None, None)),
    ("concreteGrades[].fCk", m("Characteristic strength f_ck", "Charakteristische Festigkeit f_ck", Some(UnitId::Pascal), None)),
    ("reinforcementGrades", m("Reinforcing steel grades", "Betonstahlgüten", None, None)),
    ("reinforcementGrades[].id", m("Grade id", "Güte-Id", None, None)),
    ("reinforcementGrades[].name", m("Grade name", "Gütename", None, None)),
    ("reinforcementGrades[].fYk", m("Yield strength f_yk", "Streckgrenze f_yk", Some(UnitId::Pascal), None)),
    ("reinforcementGrades[].eS", m("Modulus E_s", "Elastizitätsmodul E_s", Some(UnitId::Pascal), None)),
    ("reinforcementGrades[].ductility", m("Ductility class", "Duktilitätsklasse", None, Some(DUCTILITY))),
    ("reinforcementGrades[].k", m("Ductility ratio k=f_t/f_yk", "Duktilitätsverhältnis k=f_t/f_yk", None, None)),
    ("reinforcementGrades[].epsUk", m("Ultimate strain ε_uk", "Bruchdehnung ε_uk", None, None)),
    ("prestressSteels", m("Prestressing steels", "Spannstähle", None, None)),
    ("prestressSteels[].id", m("Steel id", "Spannstahl-Id", None, None)),
    ("prestressSteels[].name", m("Steel name", "Spannstahlname", None, None)),
    ("prestressSteels[].fPk", m("Tensile strength f_pk", "Zugfestigkeit f_pk", Some(UnitId::Pascal), None)),
    ("prestressSteels[].fP01k", m("0.1% proof stress f_p0,1k", "0,1%-Dehngrenze f_p0,1k", Some(UnitId::Pascal), None)),
    ("members", m("Members", "Bauteile", None, None)),
    ("members[].id", m("Member id", "Bauteil-Id", None, None)),
    ("members[].labelEn", m("Label (EN)", "Bezeichnung (EN)", None, None)),
    ("members[].labelDe", m("Label (DE)", "Bezeichnung (DE)", None, None)),
    ("members[].kind", m("Member kind", "Bauteilart", None, Some(MEMBER_KIND))),
    ("members[].concreteGradeId", m("Concrete grade", "Betonklasse", None, None)),
    ("members[].reinforcementGradeId", m("Reinforcement grade", "Betonstahlgüte", None, None)),
    ("members[].prestressSteelId", m("Prestressing steel", "Spannstahl", None, None)),
    ("members[].exposure", m("Exposure class", "Expositionsklasse", None, Some(EXPOSURE))),
    ("members[].width", m("Width b", "Breite b", Some(UnitId::Metre), None)),
    ("members[].height", m("Height h", "Höhe h", Some(UnitId::Metre), None)),
    ("members[].effectiveDepth", m("Effective depth d", "Statische Nutzhöhe d", Some(UnitId::Metre), None)),
    ("members[].cover", m("Concrete cover c", "Betondeckung c", Some(UnitId::Metre), None)),
    ("members[].span", m("Span l", "Spannweite l", Some(UnitId::Metre), None)),
    ("members[].support", m("Support condition", "Lagerungsbedingung", None, Some(SUPPORT))),
    ("members[].bucklingLength", m("Buckling length l_0", "Knicklänge l_0", Some(UnitId::Metre), None)),
    ("members[].longitudinal", m("Longitudinal bars", "Längsbewehrung", None, None)),
    ("members[].longitudinal[].id", m("Layer id", "Lage-Id", None, None)),
    ("members[].longitudinal[].diameter", m("Bar diameter Ø", "Stabdurchmesser Ø", Some(UnitId::Metre), None)),
    ("members[].longitudinal[].count", m("Bar count", "Stabanzahl", None, None)),
    ("members[].longitudinal[].position", m("Layer position", "Lage", None, Some(BAR_POS))),
    ("members[].longitudinal[].anchorageLength", m("Anchorage length l_bd", "Verankerungslänge l_bd", Some(UnitId::Metre), None)),
    ("members[].longitudinal[].lapLength", m("Lap length l_0", "Übergreifungslänge l_0", Some(UnitId::Metre), None)),
    ("members[].longitudinal[].bondCondition", m("Bond condition", "Verbundbedingung", None, Some(BOND))),
    ("members[].longitudinal[].aggregateSize", m("Max. aggregate size", "Größtkorndurchmesser", Some(UnitId::Metre), None)),
    ("members[].longitudinal[].lapShare", m("Lapped share of bars", "Gestoßener Anteil der Stäbe", Some(UnitId::One), None)),
    ("members[].stirrups", m("Stirrups", "Bügel", None, None)),
    ("members[].stirrups.diameter", m("Stirrup diameter", "Bügeldurchmesser", Some(UnitId::Metre), None)),
    ("members[].stirrups.spacing", m("Stirrup spacing s", "Bügelabstand s", Some(UnitId::Metre), None)),
    ("members[].stirrups.legs", m("Stirrup legs", "Bügelschenkel", None, None)),
    ("members[].punching", m("Punching", "Durchstanzen", None, None)),
    ("members[].punching.columnWidth", m("Column width c_1", "Stützenbreite c_1", Some(UnitId::Metre), None)),
    ("members[].punching.columnDepth", m("Column depth c_2", "Stützentiefe c_2", Some(UnitId::Metre), None)),
    ("members[].punching.columnPosition", m("Column position", "Stützenlage", None, Some(COLUMN_POS))),
    ("members[].punching.asw", m("Punching reinforcement A_sw per perimeter", "Durchstanzbewehrung A_sw je Dübelkreis", Some(UnitId::SquareMetre), None)),
    ("members[].punching.betaMethod", m("Load increase factor β", "Lasterhöhungsfaktor β", None, Some(BETA_METHOD))),
    ("members[].punching.momentC1", m("Unbalanced column moment M_Ed,1 (along c_1)", "Stützenmoment M_Ed,1 (entlang c_1)", Some(UnitId::NewtonMetre), None)),
    ("members[].punching.momentC2", m("Unbalanced column moment M_Ed,2 (along c_2)", "Stützenmoment M_Ed,2 (entlang c_2)", Some(UnitId::NewtonMetre), None)),
    ("members[].prestress", m("Prestress", "Vorspannung", None, None)),
    ("members[].prestress.force", m("Prestress force P_m0", "Vorspannkraft P_m0", Some(UnitId::Newton), None)),
    ("members[].prestress.area", m("Tendon area A_p", "Spannstahlfläche A_p", Some(UnitId::SquareMetre), None)),
    ("members[].prestress.eccentricity", m("Eccentricity e", "Exzentrizität e", Some(UnitId::Metre), None)),
    ("members[].prestress.lossRatio", m("Long-term loss ratio", "Langzeitverlustanteil", None, None)),
    ("members[].fire", m("Fire exposure", "Brandbeanspruchung", None, None)),
    ("members[].fire.rating", m("Fire resistance rating", "Feuerwiderstandsklasse", None, Some(FIRE))),
    ("members[].fire.axisDistance", m("Axis distance a", "Achsmaß a", Some(UnitId::Metre), None)),
    ("members[].fire.slabSystem", m("Slab fire table system", "Platten-Brandschutzsystem", None, Some(SLAB_SYSTEM))),
    ("members[].actions", m("Characteristic actions", "Charakteristische Einwirkungen", None, None)),
    ("members[].actions[].id", m("Load-case id", "Lastfall-Id", None, None)),
    ("members[].actions[].kind", m("Action kind", "Einwirkungsart", None, Some(ACTION_KIND))),
    ("members[].actions[].category", m("ψ category", "ψ-Kategorie", None, Some(ACTION_CATEGORY))),
    ("members[].actions[].source", m("Effect source", "Schnittgrößenquelle", None, Some(ACTION_SOURCE))),
    ("members[].actions[].gKLine", m("Permanent line load g_k", "Ständige Streckenlast g_k", Some(UnitId::NewtonPerMetre), None)),
    ("members[].actions[].qKLine", m("Variable line load q_k", "Veränderliche Streckenlast q_k", Some(UnitId::NewtonPerMetre), None)),
    ("members[].actions[].pointForce", m("Point force (midspan)", "Einzellast (Feldmitte)", Some(UnitId::Newton), None)),
    ("members[].actions[].mK", m("Characteristic moment M_k", "Charakteristisches Moment M_k", Some(UnitId::NewtonMetre), None)),
    ("members[].actions[].nK", m("Characteristic axial N_k (tension positive)", "Charakteristische Normalkraft N_k (Zug positiv)", Some(UnitId::Newton), None)),
    ("members[].actions[].vK", m("Characteristic shear V_k", "Charakteristische Querkraft V_k", Some(UnitId::Newton), None)),
    ("members[].actions[].tK", m("Characteristic torsion T_k", "Charakteristische Torsion T_k", Some(UnitId::NewtonMetre), None)),
    ("members[].actions[].vKPunch", m("Characteristic punching V_k", "Charakteristische Durchstanzkraft V_k", Some(UnitId::Newton), None)),
    ("members[].useFem", m("Use external FEM effects", "Äußere FEM-Schnittgrößen nutzen", None, Some(BOOL))),
    ("members[].udl", m("Extra permanent UDL (added to G)", "Zusätzliche ständige UDL (zu G)", Some(UnitId::NewtonPerMetre), None)),
    ("members[].deflectionSensitive", m("Deflection-sensitive partitions", "Durchbiegungsempfindliche Trennwände", None, Some(BOOL))),
    ("members[].tightness", m("Tightness class", "Dichtigkeitsklasse", None, Some(TIGHTNESS))),
    ("members[].hdOverH", m("h_D / h (liquid)", "h_D / h (Flüssigkeit)", None, None)),
    ("members[].liquidSigmaS", m("Steel stress σ_s (liquid)", "Stahlspannung σ_s (Flüssigkeit)", Some(UnitId::Pascal), None)),
    ("members[].liquidRhoPEff", m("ρ_p,eff (liquid)", "ρ_p,eff (Flüssigkeit)", None, None)),
    ("members[].liquidFCtEff", m("f_ct,eff (liquid)", "f_ct,eff (Flüssigkeit)", Some(UnitId::Pascal), None)),
    ("members[].liquidSRMax", m("s_r,max (liquid)", "s_r,max (Flüssigkeit)", Some(UnitId::Metre), None)),
    ("members[].bridgeSigmaC", m("Bridge concrete stress", "Brücken-Betonspannung", Some(UnitId::Pascal), None)),
    ("members[].bridgeDeltaSigmaS", m("Bridge steel stress range", "Brücken-Stahlschwingbreite", Some(UnitId::Pascal), None)),
    ("members[].relativeHumidity", m("Relative humidity RH", "Relative Luftfeuchte RH", None, None)),
    ("members[].bearingWidth", m("Bearing plate width a (0 = not given)", "Auflagerplattenbreite a (0 = nicht angegeben)", Some(UnitId::Metre), None)),
    ("members[].sectionPoints", m("Section outline points", "Querschnittsumriss", None, None)),
    ("members[].sectionPoints[].id", m("Point id", "Punkt-Id", None, None)),
    ("members[].sectionPoints[].y", m("Outline y", "Umriss y", Some(UnitId::Metre), None)),
    ("members[].sectionPoints[].z", m("Outline z", "Umriss z", Some(UnitId::Metre), None)),
    ("members[].bars", m("Explicit bars", "Einzelstäbe", None, None)),
    ("members[].bars[].id", m("Bar id", "Stab-Id", None, None)),
    ("members[].bars[].layerId", m("Layer id", "Lage-Id", None, None)),
    ("members[].bars[].y", m("Bar position y", "Stabposition y", Some(UnitId::Metre), None)),
    ("members[].bars[].z", m("Bar position z", "Stabposition z", Some(UnitId::Metre), None)),
    ("members[].bars[].diameter", m("Bar diameter Ø", "Stabdurchmesser Ø", Some(UnitId::Metre), None)),
    ("members[].bars[].length", m("Bar length (0 = derived)", "Stablänge (0 = abgeleitet)", Some(UnitId::Metre), None)),
    ("members[].bars[].bend", m("Bar bend", "Stabbiegung", None, Some(BAR_BEND))),
    ("members[].tendonPoints", m("Tendon control points", "Spanngliedstützpunkte", None, None)),
    ("members[].tendonPoints[].id", m("Point id", "Punkt-Id", None, None)),
    ("members[].tendonPoints[].x", m("Tendon x along member", "Spannglied x entlang Bauteil", Some(UnitId::Metre), None)),
    ("members[].tendonPoints[].y", m("Tendon y", "Spannglied y", Some(UnitId::Metre), None)),
    ("members[].tendonPoints[].z", m("Tendon z (eccentricity)", "Spannglied z (Exzentrizität)", Some(UnitId::Metre), None)),
    ("members[].studRows", m("Shear stud rows (rails, 6.4.5)", "Dübelreihen (Schienen, 6.4.5)", None, None)),
    ("members[].studRows[].id", m("Row id", "Reihen-Id", None, None)),
    ("members[].studRows[].angle", m("Rail angle from the column x axis", "Schienenwinkel zur Stützenachse x", Some(UnitId::Angle), None)),
    ("members[].studRows[].count", m("Studs per row", "Dübel je Reihe", None, None)),
    ("members[].studRows[].spacingFirst", m("First stud distance s0 from the column face", "Abstand des ersten Dübels s0 vom Stützenrand", Some(UnitId::Metre), None)),
    ("members[].studRows[].spacingRadial", m("Radial stud spacing s_r", "Radialer Dübelabstand s_r", Some(UnitId::Metre), None)),
    ("members[].studRows[].diameter", m("Stud diameter", "Dübeldurchmesser", Some(UnitId::Metre), None)),
    ("members[].openings", m("Openings near the column (6.4.2(3))", "Öffnungen neben der Stütze (6.4.2(3))", None, None)),
    ("members[].openings[].id", m("Opening id", "Öffnungs-Id", None, None)),
    ("members[].openings[].x", m("Opening centre x (column frame)", "Öffnungsmitte x (Stützenachsen)", Some(UnitId::Metre), None)),
    ("members[].openings[].y", m("Opening centre y (column frame)", "Öffnungsmitte y (Stützenachsen)", Some(UnitId::Metre), None)),
    ("members[].openings[].width", m("Opening width (along x)", "Öffnungsbreite (in x)", Some(UnitId::Metre), None)),
    ("members[].openings[].depth", m("Opening depth (along y)", "Öffnungstiefe (in y)", Some(UnitId::Metre), None)),
    ("anchors", m("Anchors", "Dübel", None, None)),
    ("anchors[].id", m("Anchor id", "Dübel-Id", None, None)),
    ("anchors[].hEf", m("Effective embedment h_ef", "Wirksame Verankerungstiefe h_ef", Some(UnitId::Metre), None)),
    ("anchors[].cracked", m("Cracked concrete", "Gerissener Beton", None, Some(BOOL))),
    ("anchors[].fUk", m("Ultimate steel strength f_uk", "Zugfestigkeit Stahl f_uk", Some(UnitId::Pascal), None)),
    ("anchors[].fYk", m("Yield strength f_yk", "Streckgrenze f_yk", Some(UnitId::Pascal), None)),
    ("anchors[].aS", m("Steel area A_s", "Stahlquerschnitt A_s", Some(UnitId::SquareMetre), None)),
    ("anchors[].d", m("Anchor diameter d", "Dübeldurchmesser d", Some(UnitId::Metre), None)),
    ("anchors[].c1", m("Edge distance c_1", "Randabstand c_1", Some(UnitId::Metre), None)),
    ("anchors[].fCk", m("Concrete f_ck", "Beton f_ck", Some(UnitId::Pascal), None)),
    ("anchors[].headDiameter", m("Head diameter d_h (0 = 2 d)", "Kopfdurchmesser d_h (0 = 2 d)", Some(UnitId::Metre), None)),
    ("anchors[].actions", m("Characteristic actions", "Charakteristische Einwirkungen", None, None)),
    ("anchors[].actions[].id", m("Load-case id", "Lastfall-Id", None, None)),
    ("anchors[].actions[].kind", m("Action kind", "Einwirkungsart", None, Some(ACTION_KIND))),
    ("anchors[].actions[].category", m("ψ category", "ψ-Kategorie", None, Some(ACTION_CATEGORY))),
    ("anchors[].actions[].source", m("Effect source", "Schnittgrößenquelle", None, Some(ACTION_SOURCE))),
    ("anchors[].actions[].gKLine", m("Permanent line load g_k", "Ständige Streckenlast g_k", Some(UnitId::NewtonPerMetre), None)),
    ("anchors[].actions[].qKLine", m("Variable line load q_k", "Veränderliche Streckenlast q_k", Some(UnitId::NewtonPerMetre), None)),
    ("anchors[].actions[].pointForce", m("Point force", "Einzellast", Some(UnitId::Newton), None)),
    ("anchors[].actions[].mK", m("Characteristic moment M_k", "Charakteristisches Moment M_k", Some(UnitId::NewtonMetre), None)),
    ("anchors[].actions[].nK", m("Characteristic tension N_k", "Charakteristische Zugkraft N_k", Some(UnitId::Newton), None)),
    ("anchors[].actions[].vK", m("Characteristic shear V_k", "Charakteristische Querkraft V_k", Some(UnitId::Newton), None)),
    ("anchors[].actions[].tK", m("Characteristic torsion T_k", "Charakteristische Torsion T_k", Some(UnitId::NewtonMetre), None)),
    ("anchors[].actions[].vKPunch", m("Characteristic punching V_k", "Charakteristische Durchstanzkraft V_k", Some(UnitId::Newton), None)),
    ("placements", m("Placements", "Platzierungen", None, None)),
    ("placements[].id", m("Placed entity id", "Platzierte Element-Id", None, None)),
    ("placements[].x", m("Position x", "Position x", Some(UnitId::Metre), None)),
    ("placements[].y", m("Position y", "Position y", Some(UnitId::Metre), None)),
    ("placements[].z", m("Position z", "Position z", Some(UnitId::Metre), None)),
    ("placements[].yaw", m("Yaw angle about z", "Drehwinkel um z", Some(UnitId::Radian), None)),
    ("cases", m("Design cases", "Bemessungsfälle", None, None)),
    ("cases[].id", m("Case id", "Fall-Id", None, None)),
    ("cases[].labelEn", m("Case label (English)", "Fallbezeichnung (Englisch)", None, None)),
    ("cases[].labelDe", m("Case label (German)", "Fallbezeichnung (Deutsch)", None, None)),
    ("cases[].entryId", m("Catalogue entry", "Katalogeintrag", None, None)),
    ("cases[].numbers", m("Case numbers", "Fallzahlen", None, None)),
    ("cases[].numbers[].id", m("Number id", "Zahlen-Id", None, None)),
    ("cases[].numbers[].value", m("Number value", "Zahlenwert", None, None)),
    ("cases[].texts", m("Case texts", "Falltexte", None, None)),
    ("cases[].texts[].id", m("Text id", "Text-Id", None, None)),
    ("cases[].texts[].value", m("Text value", "Textwert", None, None)),
];

/// 🏷️ Exact / `[]`-wildcard lookup only — no longest-prefix fallback (every leaf has an explicit label); the inputs of a design case (`cases[id=…].numbers[id=KEY].value`) take their unit, label, bounds and choices from the typed case-input vocabulary.
pub fn en1992_field_meta(path: &str) -> Option<FieldSpec> {
    if let Some(meta) = case_input_field_meta(semio_s_artifact_norm_en1992_checks::catalogue_registry::case_inputs::VOCABULARY, path) {
        return Some(meta);
    }
    if let Some((_, meta)) = TABLE.iter().find(|(key, _)| *key == path) {
        return Some(*meta);
    }
    let templated = {
        let mut out = String::with_capacity(path.len());
        let mut chars = path.chars().peekable();
        while let Some(c) = chars.next() {
            if c == '[' {
                out.push_str("[]");
                for x in chars.by_ref() {
                    if x == ']' { break; }
                }
            } else {
                out.push(c);
            }
        }
        out
    };
    TABLE.iter().find(|(key, _)| *key == templated.as_str()).map(|(_, m)| *m)
}

/// 🧪 Table keys for parity tests.
pub fn en1992_field_meta_keys() -> &'static [(&'static str, FieldSpec)] {
    TABLE
}
