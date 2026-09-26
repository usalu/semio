//! 🏷️ EN 1999 NormFieldMeta lookup — SI display units + en/de labels (longest-prefix + `[]` wildcards).

use crate::app_surface::{lookup_norm_field_meta, NormFieldChoice, NormFieldMeta};

const fn choice(value: &'static str, label_en: &'static str, label_de: &'static str) -> NormFieldChoice {
    NormFieldChoice { value, label_en, label_de }
}

const ANNEX: &[NormFieldChoice] = &[
    choice("en", "EN (CEN recommended)", "EN (CEN-Empfehlung)"),
    choice("de", "Germany (DIN EN / NA)", "Deutschland (DIN EN / NA)"),
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
    choice("self", "Self-weight", "Eigengewicht"),
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

const SECTION_KINDS: &[NormFieldChoice] = &[
    choice("extrudedI", "Extruded I-section", "Stranggepresstes I-Profil"),
    choice("channel", "Channel section", "U-Profil"),
    choice("angle", "Angle section", "Winkelprofil"),
    choice("rhs", "Rectangular hollow section", "Rechteck-Hohlprofil"),
    choice("box", "Box section", "Kastenprofil"),
    choice("tube", "Circular hollow section (CHS)", "Kreis-Hohlprofil (CHS)"),
    choice("chs", "Circular hollow section (CHS)", "Kreis-Hohlprofil (CHS)"),
];

const CONN_KINDS: &[NormFieldChoice] = &[
    choice("bolted", "Bolted connection", "Schraubenverbindung"),
    choice("welded", "Welded connection", "Schweißverbindung"),
    choice("combined", "Combined bolted and welded", "Kombiniert geschraubt und geschweißt"),
];

const fn m(label_en: &'static str, label_de: &'static str, unit: Option<&'static str>, choices: Option<&'static [NormFieldChoice]>) -> NormFieldMeta {
    NormFieldMeta { label_en, label_de, unit, choices }
}

const TABLE: &[(&str, NormFieldMeta)] = &[
    ("annex", m("National annex", "Nationaler Anhang", None, Some(ANNEX))),
    ("materials[].id", m("Material id", "Material-Id", None, None)),
    ("materials[].designation", m("Alloy / temper", "Legierung / Zustand", None, Some(ALLOYS))),
    ("sections[].id", m("Section id", "Querschnitts-Id", None, None)),
    ("sections[].kind", m("Section kind", "Querschnittsart", None, Some(SECTION_KINDS))),
    ("sections[].height", m("Section height h", "Querschnittshöhe h", Some("m"), None)),
    ("sections[].width", m("Section width b", "Querschnittsbreite b", Some("m"), None)),
    ("sections[].flangeThickness", m("Flange thickness t_f", "Flanschdicke t_f", Some("m"), None)),
    ("sections[].webThickness", m("Web thickness t_w", "Stegdicke t_w", Some("m"), None)),
    ("sections[].outerDiameter", m("Outer diameter D", "Außendurchmesser D", Some("m"), None)),
    ("sections[].elements[].id", m("Plate element id", "Blechelement-Id", None, None)),
    ("sections[].elements[].width", m("Plate width b", "Blechbreite b", Some("m"), None)),
    ("sections[].elements[].thickness", m("Plate thickness t", "Blechdicke t", Some("m"), None)),
    ("sections[].elements[].outstand", m("Outstand plate", "Auskragendes Blech", None, None)),
    ("sections[].elements[].welded", m("Welded plate", "Geschweißtes Blech", None, None)),
    ("sections[].elements[].weldPosition", m("Weld position", "Schweißnahtlage", Some("m"), None)),
    ("members[].id", m("Member id", "Bauteil-Id", None, None)),
    ("members[].sectionId", m("Section reference", "Querschnittsverweis", None, None)),
    ("members[].materialId", m("Material reference", "Materialverweis", None, None)),
    ("members[].length", m("Member length L", "Bauteillänge L", Some("m"), None)),
    ("members[].bucklingLengthY", m("Buckling length L_cr,y", "Knicklänge L_cr,y", Some("m"), None)),
    ("members[].bucklingLengthZ", m("Buckling length L_cr,z", "Knicklänge L_cr,z", Some("m"), None)),
    ("members[].bucklingLengthT", m("Torsional buckling length L_cr,T", "Drillknicklänge L_cr,T", Some("m"), None)),
    ("members[].ltbLength", m("LTB length L_LT", "Biegedrillknicklänge L_LT", Some("m"), None)),
    ("members[].c1", m("LTB factor C_1", "Biegedrillknickfaktor C_1", None, None)),
    ("members[].restrainedLtb", m("LTB fully restrained", "Biegedrillknicken gehalten", None, None)),
    ("members[].actions[].id", m("Action / load-case id", "Einwirkung / Lastfall-Id", None, None)),
    ("members[].actions[].nK", m("Characteristic axial force N_k", "Charakteristische Normalkraft N_k", Some("N"), None)),
    ("members[].actions[].mYK", m("Characteristic moment M_y,k", "Charakteristisches Moment M_y,k", Some("N·m"), None)),
    ("connections[].id", m("Connection id", "Anschluss-Id", None, None)),
    ("connections[].memberId", m("Connected member", "Angeschlossenes Bauteil", None, None)),
    ("connections[].materialId", m("Connection material", "Anschluss-Material", None, None)),
    ("connections[].kind", m("Connection kind", "Anschlussart", None, Some(CONN_KINDS))),
    ("connections[].actions[].nK", m("Characteristic connection axial N_k", "Charakteristische Anschluss-Normalkraft N_k", Some("N"), None)),
    ("connections[].bolts.material", m("Bolt grade", "Schraubenfestigkeit", None, None)),
    ("connections[].bolts.diameter", m("Bolt diameter d", "Schraubendurchmesser d", Some("m"), None)),
    ("connections[].bolts.rows", m("Bolt rows", "Schraubenreihen", None, None)),
    ("connections[].bolts.boltsPerRow", m("Bolts per row", "Schrauben je Reihe", None, None)),
    ("connections[].bolts.edgeDistance", m("Edge distance e_1", "Randabstand e_1", Some("m"), None)),
    ("connections[].bolts.pitch", m("Bolt pitch p_1", "Schraubenabstand p_1", Some("m"), None)),
    ("connections[].bolts.gauge", m("Bolt gauge p_2", "Schraubenabstand p_2", Some("m"), None)),
    ("connections[].bolts.plateThickness", m("Connection plate thickness", "Anschlussblechdicke", Some("m"), None)),
    ("connections[].welds.fillerAlloy", m("Weld filler alloy", "Schweißzusatz", None, Some(FILLER))),
    ("connections[].welds.throat", m("Weld throat a", "Kehlnahtdicke a", Some("m"), None)),
    ("connections[].welds.length", m("Weld length ℓ", "Nahtlänge ℓ", Some("m"), None)),
    ("connections[].welds.betaW", m("Correlation factor β_w", "Korrelationsfaktor β_w", None, None)),
    ("connections[].welds.hazExtent", m("HAZ extent", "WEZ-Ausdehnung", Some("m"), None)),
    ("fireScenarios[].id", m("Fire scenario id", "Brandszenario-Id", None, None)),
    ("fireScenarios[].memberId", m("Fire member reference", "Brand-Bauteilverweis", None, None)),
    ("fireScenarios[].thetaA", m("Aluminium temperature θ_a", "Aluminiumtemperatur θ_a", Some("°C"), None)),
    ("fireScenarios[].durationS", m("Fire duration", "Branddauer", Some("s"), None)),
    ("fatigueDetails[].id", m("Fatigue detail id", "Ermüdungsdetail-Id", None, None)),
    ("fatigueDetails[].memberId", m("Fatigue member reference", "Ermüdungs-Bauteilverweis", None, None)),
    ("fatigueDetails[].deltaSigmaC", m("Reference fatigue strength Δσ_C", "Bezugsschwingfestigkeit Δσ_C", Some("Pa"), None)),
    ("fatigueDetails[].deltaSigmaEd", m("Stress range Δσ_Ed", "Spannungsschwingbreite Δσ_Ed", Some("Pa"), None)),
    ("fatigueDetails[].m1", m("S–N slope m1", "S–N-Neigung m1", None, None)),
    ("fatigueDetails[].m2", m("S–N slope m2", "S–N-Neigung m2", None, None)),
    ("fatigueDetails[].detailCategory", m("Annex J detail category", "Annex-J-Kerbfall", None, None)),
    ("fatigueDetails[].nCycles", m("Number of cycles N", "Lastwechselzahl N", None, None)),
    ("coldFormed[].id", m("Cold-formed sheet id", "Kaltprofil-Id", None, None)),
    ("coldFormed[].materialId", m("Sheet material reference", "Blech-Materialverweis", None, None)),
    ("coldFormed[].thickness", m("Sheet thickness t", "Blechdicke t", Some("m"), None)),
    ("coldFormed[].width", m("Flat width b", "Flachbreite b", Some("m"), None)),
    ("coldFormed[].span", m("Sheet span L", "Blechstützweite L", Some("m"), None)),
    ("coldFormed[].mEd", m("Sheet moment M_Ed", "Blechmoment M_Ed", Some("N·m"), None)),
    ("coldFormed[].nEd", m("Sheet axial force N_Ed", "Blech-Normalkraft N_Ed", Some("N"), None)),
    ("coldFormed[].welded", m("Sheet welded", "Blech geschweißt", None, None)),
    ("shells[].id", m("Shell id", "Schalen-Id", None, None)),
    ("shells[].materialId", m("Shell material reference", "Schalen-Materialverweis", None, None)),
    ("shells[].radius", m("Shell mid-surface radius r", "Schalenradius r", Some("m"), None)),
    ("shells[].thickness", m("Shell thickness t", "Schalendicke t", Some("m"), None)),
    ("shells[].length", m("Shell meridian length L", "Schalenmeridianlänge L", Some("m"), None)),
    ("shells[].sigmaXEd", m("Meridional stress σ_x,Ed", "Meridianspannung σ_x,Ed", Some("Pa"), None)),
    ("shells[].sigmaThetaEd", m("Circumferential stress σ_θ,Ed", "Umfangsspannung σ_θ,Ed", Some("Pa"), None)),
    ("members[].support", m("Support condition", "Lagerungsart", None, Some(SUPPORT))),
    ("members[].actions[].kind", m("Action kind", "Einwirkungsart", None, Some(ACTION_KIND))),
    ("members[].actions[].category", m("EN 1990 action category", "EN-1990-Einwirkungskategorie", None, Some(ACTION_CAT))),
    ("members[].actions[].source", m("Action source", "Einwirkungsquelle", None, Some(ACTION_SRC))),
    ("members[].actions[].gKLine", m("Characteristic permanent line load g_k", "Charakteristische ständige Streckenlast g_k", Some("N/m"), None)),
    ("members[].actions[].qKLine", m("Characteristic variable line load q_k", "Charakteristische veränderliche Streckenlast q_k", Some("N/m"), None)),
    ("members[].actions[].vYK", m("Characteristic shear V_y,k", "Charakteristische Querkraft V_y,k", Some("N"), None)),
    ("members[].actions[].vZK", m("Characteristic shear V_z,k", "Charakteristische Querkraft V_z,k", Some("N"), None)),
    ("members[].actions[].mZK", m("Characteristic moment M_z,k", "Charakteristisches Moment M_z,k", Some("N·m"), None)),
    ("connections[].actions[].id", m("Connection load-case id", "Anschluss-Lastfall-Id", None, None)),
    ("connections[].actions[].kind", m("Connection action kind", "Anschluss-Einwirkungsart", None, Some(ACTION_KIND))),
    ("connections[].actions[].category", m("Connection action category", "Anschluss-Einwirkungskategorie", None, Some(ACTION_CAT))),
    ("connections[].actions[].source", m("Connection action source", "Anschluss-Einwirkungsquelle", None, Some(ACTION_SRC))),
    ("connections[].actions[].gKLine", m("Characteristic permanent line load g_k", "Charakteristische ständige Streckenlast g_k", Some("N/m"), None)),
    ("connections[].actions[].qKLine", m("Characteristic variable line load q_k", "Charakteristische veränderliche Streckenlast q_k", Some("N/m"), None)),
    ("connections[].actions[].vYK", m("Characteristic shear V_y,k", "Charakteristische Querkraft V_y,k", Some("N"), None)),
    ("connections[].actions[].vZK", m("Characteristic shear V_z,k", "Charakteristische Querkraft V_z,k", Some("N"), None)),
    ("connections[].actions[].mYK", m("Characteristic moment M_y,k", "Charakteristisches Moment M_y,k", Some("N·m"), None)),
    ("connections[].actions[].mZK", m("Characteristic moment M_z,k", "Charakteristisches Moment M_z,k", Some("N·m"), None)),
];

/// 🏷️ Longest-prefix / `[]` wildcard lookup for the EN 1999 structured inputs editor.
pub fn en1999_field_meta(path: &str) -> Option<NormFieldMeta> {
    lookup_norm_field_meta(TABLE, path)
}
