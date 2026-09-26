//! 🏷️ EN 1992 NormFieldMeta — SI units + en/de labels (`[]` wildcards for list leaves).

use crate::app_surface::{NormFieldChoice, NormFieldMeta};

const fn choice(value: &'static str, label_en: &'static str, label_de: &'static str) -> NormFieldChoice {
    NormFieldChoice { value, label_en, label_de }
}

const ANNEX: &[NormFieldChoice] = &[
    choice("en", "EN (CEN recommended)", "EN (CEN-Empfehlung)"),
    choice("de", "Germany (DIN NA)", "Deutschland (DIN NA)"),
];

const EXPOSURE: &[NormFieldChoice] = &[
    choice("x0", "X0 — no risk", "X0 — kein Risiko"),
    choice("xc1", "XC1 — dry / permanently wet", "XC1 — trocken / ständig nass"),
    choice("xc2", "XC2 — wet, rarely dry", "XC2 — nass, selten trocken"),
    choice("xc3", "XC3 — moderate humidity", "XC3 — mäßige Feuchte"),
    choice("xc4", "XC4 — cyclic wet/dry", "XC4 — wechselnd nass/trocken"),
    choice("xd1", "XD1 — moderate chloride", "XD1 — mäßige Chloride"),
    choice("xd2", "XD2 — wet chloride", "XD2 — nass Chloride"),
    choice("xd3", "XD3 — cyclic chloride", "XD3 — wechselnd Chloride"),
    choice("xs1", "XS1 — airborne salt", "XS1 — luftgetragenes Salz"),
    choice("xs2", "XS2 — submerged", "XS2 — unter Wasser"),
    choice("xs3", "XS3 — tidal / splash", "XS3 — Gezeiten / Spritzwasser"),
    choice("xf1", "XF1 — moderate freeze", "XF1 — mäßiger Frost"),
    choice("xf2", "XF2 — freeze + de-icing", "XF2 — Frost + Tausalz"),
    choice("xf3", "XF3 — high water saturation", "XF3 — hohe Wassersättigung"),
    choice("xf4", "XF4 — freeze + salt spray", "XF4 — Frost + Salzsprühnebel"),
    choice("xa1", "XA1 — slightly aggressive", "XA1 — schwach angreifend"),
    choice("xa2", "XA2 — moderately aggressive", "XA2 — mäßig angreifend"),
    choice("xa3", "XA3 — highly aggressive", "XA3 — stark angreifend"),
];

const MEMBER_KIND: &[NormFieldChoice] = &[
    choice("beam", "Beam", "Balken"),
    choice("slab", "Slab", "Platte"),
    choice("column", "Column", "Stütze"),
    choice("wall", "Wall", "Wand"),
    choice("flat-slab", "Flat slab (punching)", "Flachdecke (Durchstanzen)"),
    choice("ribbed-slab", "Ribbed slab", "Rippendecke"),
    choice("tension-member", "Tension member", "Zugglied"),
    choice("bridge", "Bridge member", "Brückenbauteil"),
    choice("liquid-retaining", "Liquid-retaining", "Flüssigkeitsbehälter"),
];

const SUPPORT: &[NormFieldChoice] = &[
    choice("simply-supported", "Simply supported", "Einfach gelagert"),
    choice("continuous", "Continuous", "Durchlaufend"),
    choice("cantilever", "Cantilever", "Kragarm"),
    choice("fixed", "Fixed", "Eingespannt"),
];

const FIRE: &[NormFieldChoice] = &[
    choice("r30", "R30 (30 min)", "R30 (30 Min.)"),
    choice("r60", "R60 (60 min)", "R60 (60 Min.)"),
    choice("r90", "R90 (90 min)", "R90 (90 Min.)"),
    choice("r120", "R120 (120 min)", "R120 (120 Min.)"),
];

const TIGHTNESS: &[NormFieldChoice] = &[
    choice("tc0", "TC0 — some leakage allowed", "TC0 — geringe Leckage erlaubt"),
    choice("tc1", "TC1 — leakage minimized", "TC1 — Leckage minimiert"),
    choice("tc2", "TC2 — no leakage", "TC2 — keine Leckage"),
];

const BOOL: &[NormFieldChoice] = &[choice("true", "Yes", "Ja"), choice("false", "No", "Nein")];

const BAR_POS: &[NormFieldChoice] = &[
    choice("bottom", "Bottom / tension", "Unten / Zug"),
    choice("top", "Top / compression", "Oben / Druck"),
    choice("tension", "Tension", "Zug"),
    choice("compression", "Compression", "Druck"),
];

const ACTION_KIND: &[NormFieldChoice] = &[
    choice("permanent", "Permanent (G)", "Ständig (G)"),
    choice("imposed", "Imposed (Q)", "Nutzlast (Q)"),
    choice("snow", "Snow (S)", "Schnee (S)"),
    choice("wind", "Wind (W)", "Wind (W)"),
    choice("accidental", "Accidental (A)", "Außergewöhnlich (A)"),
    choice("prestress", "Prestress (P)", "Vorspannung (P)"),
];
const ACTION_CATEGORY: &[NormFieldChoice] = &[
    choice("self", "Self-weight", "Eigengewicht"),
    choice("office", "Office", "Büro"),
    choice("residential", "Residential", "Wohnen"),
    choice("shopping", "Shopping", "Verkauf"),
    choice("storage", "Storage", "Lager"),
    choice("snow", "Snow", "Schnee"),
    choice("wind", "Wind", "Wind"),
    choice("accidental", "Accidental", "Außergewöhnlich"),
];
const ACTION_SOURCE: &[NormFieldChoice] = &[
    choice("udl", "Derive from line load (span/support)", "Aus Streckenlast (Spannweite/Lagerung)"),
    choice("point", "Derive from midspan point load", "Aus Einzellast in Feldmitte"),
    choice("external", "External characteristic internal forces", "Äußere charakteristische Schnittgrößen"),
];
const COLUMN_POS: &[NormFieldChoice] = &[
    choice("interior", "Interior column", "Innenstütze"),
    choice("edge", "Edge column", "Randstütze"),
    choice("corner", "Corner column", "Eckstütze"),
];

const DUCTILITY: &[NormFieldChoice] = &[
    choice("a", "Class A (low ductility)", "Klasse A (geringe Duktilität)"),
    choice("b", "Class B (medium ductility)", "Klasse B (mittlere Duktilität)"),
    choice("c", "Class C (high ductility)", "Klasse C (hohe Duktilität)"),
];
const COLUMN_METHOD: &[NormFieldChoice] = &[
    choice("A", "Table 5.2a method A", "Tabelle 5.2a Verfahren A"),
    choice("B", "Table 5.2b method B", "Tabelle 5.2b Verfahren B"),
];
const SLAB_SYSTEM: &[NormFieldChoice] = &[
    choice("one-way", "One-way slab (Table 5.8)", "Einachsig (Tabelle 5.8)"),
    choice("two-way", "Two-way slab (Table 5.9)", "Zweiachsig (Tabelle 5.9)"),
    choice("flat", "Flat slab (Table 5.10)", "Flachdecke (Tabelle 5.10)"),
    choice("ribbed", "Ribbed slab (Table 5.11)", "Rippendecke (Tabelle 5.11)"),
];

const BOND: &[NormFieldChoice] = &[
    choice("good", "Good bond conditions", "Gute Verbundbedingungen"),
    choice("poor", "Poor bond conditions", "Schlechte Verbundbedingungen"),
];

const fn m(label_en: &'static str, label_de: &'static str, unit: Option<&'static str>, choices: Option<&'static [NormFieldChoice]>) -> NormFieldMeta {
    NormFieldMeta { label_en, label_de, unit, choices }
}

const TABLE: &[(&str, NormFieldMeta)] = &[
    ("annex", m("National annex", "Nationaler Anhang", None, Some(ANNEX))),
    ("title", m("Title", "Titel", None, None)),
    ("designWorkingLifeYears", m("Design working life", "Geplante Nutzungsdauer", Some("a"), None)),
    ("deltaCDev", m("Cover allowance Δc_dev", "Vorhaltemaß Δc_dev", Some("m"), None)),
    ("cementType", m("Cement type class", "Zementart-Klasse", None, None)),
    ("concreteGrades", m("Concrete grades", "Betonfestigkeitsklassen", None, None)),
    ("concreteGrades[].id", m("Grade id", "Klassen-Id", None, None)),
    ("concreteGrades[].name", m("Grade name", "Klassenname", None, None)),
    ("concreteGrades[].fCk", m("Characteristic strength f_ck", "Charakteristische Festigkeit f_ck", Some("Pa"), None)),
    ("reinforcementGrades", m("Reinforcing steel grades", "Betonstahlgüten", None, None)),
    ("reinforcementGrades[].id", m("Grade id", "Güte-Id", None, None)),
    ("reinforcementGrades[].name", m("Grade name", "Gütename", None, None)),
    ("reinforcementGrades[].fYk", m("Yield strength f_yk", "Streckgrenze f_yk", Some("Pa"), None)),
    ("reinforcementGrades[].eS", m("Modulus E_s", "Elastizitätsmodul E_s", Some("Pa"), None)),
    ("reinforcementGrades[].ductility", m("Ductility class", "Duktilitätsklasse", None, Some(DUCTILITY))),
    ("reinforcementGrades[].k", m("Ductility ratio k=f_t/f_yk", "Duktilitätsverhältnis k=f_t/f_yk", None, None)),
    ("reinforcementGrades[].epsUk", m("Ultimate strain ε_uk", "Bruchdehnung ε_uk", None, None)),
    ("prestressSteels", m("Prestressing steels", "Spannstähle", None, None)),
    ("prestressSteels[].id", m("Steel id", "Spannstahl-Id", None, None)),
    ("prestressSteels[].name", m("Steel name", "Spannstahlname", None, None)),
    ("prestressSteels[].fPk", m("Tensile strength f_pk", "Zugfestigkeit f_pk", Some("Pa"), None)),
    ("prestressSteels[].fP01k", m("0.1% proof stress f_p0,1k", "0,1%-Dehngrenze f_p0,1k", Some("Pa"), None)),
    ("members", m("Members", "Bauteile", None, None)),
    ("members[].id", m("Member id", "Bauteil-Id", None, None)),
    ("members[].labelEn", m("Label (EN)", "Bezeichnung (EN)", None, None)),
    ("members[].labelDe", m("Label (DE)", "Bezeichnung (DE)", None, None)),
    ("members[].kind", m("Member kind", "Bauteilart", None, Some(MEMBER_KIND))),
    ("members[].concreteGradeId", m("Concrete grade", "Betonklasse", None, None)),
    ("members[].reinforcementGradeId", m("Reinforcement grade", "Betonstahlgüte", None, None)),
    ("members[].prestressSteelId", m("Prestressing steel", "Spannstahl", None, None)),
    ("members[].exposure", m("Exposure class", "Expositionsklasse", None, Some(EXPOSURE))),
    ("members[].width", m("Width b", "Breite b", Some("m"), None)),
    ("members[].height", m("Height h", "Höhe h", Some("m"), None)),
    ("members[].effectiveDepth", m("Effective depth d", "Statische Nutzhöhe d", Some("m"), None)),
    ("members[].cover", m("Concrete cover c", "Betondeckung c", Some("m"), None)),
    ("members[].span", m("Span l", "Spannweite l", Some("m"), None)),
    ("members[].support", m("Support condition", "Lagerungsbedingung", None, Some(SUPPORT))),
    ("members[].bucklingLength", m("Buckling length l_0", "Knicklänge l_0", Some("m"), None)),
    ("members[].longitudinal", m("Longitudinal bars", "Längsbewehrung", None, None)),
    ("members[].longitudinal[].id", m("Layer id", "Lage-Id", None, None)),
    ("members[].longitudinal[].diameter", m("Bar diameter Ø", "Stabdurchmesser Ø", Some("m"), None)),
    ("members[].longitudinal[].count", m("Bar count", "Stabanzahl", None, None)),
    ("members[].longitudinal[].position", m("Layer position", "Lage", None, Some(BAR_POS))),
    ("members[].longitudinal[].anchorageLength", m("Anchorage length l_bd", "Verankerungslänge l_bd", Some("m"), None)),
    ("members[].longitudinal[].lapLength", m("Lap length l_0", "Übergreifungslänge l_0", Some("m"), None)),
    ("members[].longitudinal[].bondCondition", m("Bond condition", "Verbundbedingung", None, Some(BOND))),
    ("members[].longitudinal[].aggregateSize", m("Max. aggregate size", "Größtkorndurchmesser", Some("m"), None)),
    ("members[].stirrups", m("Stirrups", "Bügel", None, None)),
    ("members[].stirrups.diameter", m("Stirrup diameter", "Bügeldurchmesser", Some("m"), None)),
    ("members[].stirrups.spacing", m("Stirrup spacing s", "Bügelabstand s", Some("m"), None)),
    ("members[].stirrups.legs", m("Stirrup legs", "Bügelschenkel", None, None)),
    ("members[].punching", m("Punching", "Durchstanzen", None, None)),
    ("members[].punching.columnWidth", m("Column width c_1", "Stützenbreite c_1", Some("m"), None)),
    ("members[].punching.columnDepth", m("Column depth c_2", "Stützentiefe c_2", Some("m"), None)),
    ("members[].punching.columnPosition", m("Column position", "Stützenlage", None, Some(COLUMN_POS))),
    ("members[].punching.asw", m("Punching reinforcement A_sw", "Durchstanzbewehrung A_sw", Some("m²/m"), None)),
    ("members[].prestress", m("Prestress", "Vorspannung", None, None)),
    ("members[].prestress.force", m("Prestress force P_m0", "Vorspannkraft P_m0", Some("N"), None)),
    ("members[].prestress.area", m("Tendon area A_p", "Spannstahlfläche A_p", Some("m²"), None)),
    ("members[].prestress.eccentricity", m("Eccentricity e", "Exzentrizität e", Some("m"), None)),
    ("members[].prestress.lossRatio", m("Long-term loss ratio", "Langzeitverlustanteil", None, None)),
    ("members[].fire", m("Fire exposure", "Brandbeanspruchung", None, None)),
    ("members[].fire.rating", m("Fire resistance rating", "Feuerwiderstandsklasse", None, Some(FIRE))),
    ("members[].fire.axisDistance", m("Axis distance a", "Achsmaß a", Some("m"), None)),
    ("members[].fire.columnMethod", m("Column fire table method", "Stützen-Brandschutzverfahren", None, Some(COLUMN_METHOD))),
    ("members[].fire.slabSystem", m("Slab fire table system", "Platten-Brandschutzsystem", None, Some(SLAB_SYSTEM))),
    ("members[].actions", m("Characteristic actions", "Charakteristische Einwirkungen", None, None)),
    ("members[].actions[].id", m("Load-case id", "Lastfall-Id", None, None)),
    ("members[].actions[].kind", m("Action kind", "Einwirkungsart", None, Some(ACTION_KIND))),
    ("members[].actions[].category", m("ψ category", "ψ-Kategorie", None, Some(ACTION_CATEGORY))),
    ("members[].actions[].source", m("Effect source", "Schnittgrößenquelle", None, Some(ACTION_SOURCE))),
    ("members[].actions[].gKLine", m("Permanent line load g_k", "Ständige Streckenlast g_k", Some("N/m"), None)),
    ("members[].actions[].qKLine", m("Variable line load q_k", "Veränderliche Streckenlast q_k", Some("N/m"), None)),
    ("members[].actions[].pointForce", m("Point force (midspan)", "Einzellast (Feldmitte)", Some("N"), None)),
    ("members[].actions[].mK", m("Characteristic moment M_k", "Charakteristisches Moment M_k", Some("N·m"), None)),
    ("members[].actions[].nK", m("Characteristic axial N_k", "Charakteristische Normalkraft N_k", Some("N"), None)),
    ("members[].actions[].vK", m("Characteristic shear V_k", "Charakteristische Querkraft V_k", Some("N"), None)),
    ("members[].actions[].tK", m("Characteristic torsion T_k", "Charakteristische Torsion T_k", Some("N·m"), None)),
    ("members[].actions[].vKPunch", m("Characteristic punching V_k", "Charakteristische Durchstanzkraft V_k", Some("N"), None)),
    ("members[].useFem", m("Use external FEM effects", "Äußere FEM-Schnittgrößen nutzen", None, Some(BOOL))),
    ("members[].udl", m("Extra permanent UDL (added to G)", "Zusätzliche ständige UDL (zu G)", Some("N/m"), None)),
    ("members[].deflectionSensitive", m("Deflection-sensitive partitions", "Durchbiegungsempfindliche Trennwände", None, Some(BOOL))),
    ("members[].tightness", m("Tightness class", "Dichtigkeitsklasse", None, Some(TIGHTNESS))),
    ("members[].hdOverH", m("h_D / h (liquid)", "h_D / h (Flüssigkeit)", None, None)),
    ("members[].liquidSigmaS", m("Steel stress σ_s (liquid)", "Stahlspannung σ_s (Flüssigkeit)", Some("Pa"), None)),
    ("members[].liquidRhoPEff", m("ρ_p,eff (liquid)", "ρ_p,eff (Flüssigkeit)", None, None)),
    ("members[].liquidFCtEff", m("f_ct,eff (liquid)", "f_ct,eff (Flüssigkeit)", Some("Pa"), None)),
    ("members[].liquidSRMax", m("s_r,max (liquid)", "s_r,max (Flüssigkeit)", Some("m"), None)),
    ("members[].bridgeSigmaC", m("Bridge concrete stress", "Brücken-Betonspannung", Some("Pa"), None)),
    ("members[].bridgeDeltaSigmaS", m("Bridge steel stress range", "Brücken-Stahlschwingbreite", Some("Pa"), None)),
    ("anchors", m("Anchors", "Dübel", None, None)),
    ("anchors[].id", m("Anchor id", "Dübel-Id", None, None)),
    ("anchors[].hEf", m("Effective embedment h_ef", "Wirksame Verankerungstiefe h_ef", Some("m"), None)),
    ("anchors[].cracked", m("Cracked concrete", "Gerissener Beton", None, Some(BOOL))),
    ("anchors[].fUk", m("Ultimate steel strength f_uk", "Zugfestigkeit Stahl f_uk", Some("Pa"), None)),
    ("anchors[].fYk", m("Yield strength f_yk", "Streckgrenze f_yk", Some("Pa"), None)),
    ("anchors[].aS", m("Steel area A_s", "Stahlquerschnitt A_s", Some("m²"), None)),
    ("anchors[].d", m("Anchor diameter d", "Dübeldurchmesser d", Some("m"), None)),
    ("anchors[].c1", m("Edge distance c_1", "Randabstand c_1", Some("m"), None)),
    ("anchors[].fCk", m("Concrete f_ck", "Beton f_ck", Some("Pa"), None)),
    ("anchors[].actions", m("Characteristic actions", "Charakteristische Einwirkungen", None, None)),
    ("anchors[].actions[].id", m("Load-case id", "Lastfall-Id", None, None)),
    ("anchors[].actions[].kind", m("Action kind", "Einwirkungsart", None, Some(ACTION_KIND))),
    ("anchors[].actions[].category", m("ψ category", "ψ-Kategorie", None, Some(ACTION_CATEGORY))),
    ("anchors[].actions[].source", m("Effect source", "Schnittgrößenquelle", None, Some(ACTION_SOURCE))),
    ("anchors[].actions[].gKLine", m("Permanent line load g_k", "Ständige Streckenlast g_k", Some("N/m"), None)),
    ("anchors[].actions[].qKLine", m("Variable line load q_k", "Veränderliche Streckenlast q_k", Some("N/m"), None)),
    ("anchors[].actions[].pointForce", m("Point force", "Einzellast", Some("N"), None)),
    ("anchors[].actions[].mK", m("Characteristic moment M_k", "Charakteristisches Moment M_k", Some("N·m"), None)),
    ("anchors[].actions[].nK", m("Characteristic tension N_k", "Charakteristische Zugkraft N_k", Some("N"), None)),
    ("anchors[].actions[].vK", m("Characteristic shear V_k", "Charakteristische Querkraft V_k", Some("N"), None)),
    ("anchors[].actions[].tK", m("Characteristic torsion T_k", "Charakteristische Torsion T_k", Some("N·m"), None)),
    ("anchors[].actions[].vKPunch", m("Characteristic punching V_k", "Charakteristische Durchstanzkraft V_k", Some("N"), None)),
];

/// 🏷️ Exact / `[]`-wildcard lookup only — no longest-prefix fallback (every leaf has an explicit label).
pub fn en1992_field_meta(path: &str) -> Option<NormFieldMeta> {
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
pub fn en1992_field_meta_keys() -> &'static [(&'static str, NormFieldMeta)] {
    TABLE
}
