//! 🏷️ EN 1995 NormFieldMeta — SI units + en/de labels (`[]` wildcards for list leaves, nested action tables).

use crate::app_surface::{lookup_norm_field_meta, NormFieldChoice, NormFieldMeta};

const fn choice(value: &'static str, label_en: &'static str, label_de: &'static str) -> NormFieldChoice {
    NormFieldChoice { value, label_en, label_de }
}

const ANNEX: &[NormFieldChoice] = &[
    choice("en", "EN — recommended values (CEN)", "EN — Empfohlene Werte (CEN)"),
    choice("de", "DE — German national annex (DIN)", "DE — Deutscher Nationaler Anhang (DIN)"),
];
const STRENGTH: &[NormFieldChoice] = &[
    choice("C14", "C14 — softwood strength class", "C14 — Nadelholz-Festigkeitsklasse"),
    choice("C16", "C16 — softwood strength class", "C16 — Nadelholz-Festigkeitsklasse"),
    choice("C18", "C18 — softwood strength class", "C18 — Nadelholz-Festigkeitsklasse"),
    choice("C20", "C20 — softwood strength class", "C20 — Nadelholz-Festigkeitsklasse"),
    choice("C22", "C22 — softwood strength class", "C22 — Nadelholz-Festigkeitsklasse"),
    choice("C24", "C24 — softwood strength class", "C24 — Nadelholz-Festigkeitsklasse"),
    choice("C27", "C27 — softwood strength class", "C27 — Nadelholz-Festigkeitsklasse"),
    choice("C30", "C30 — softwood strength class", "C30 — Nadelholz-Festigkeitsklasse"),
    choice("C35", "C35 — softwood strength class", "C35 — Nadelholz-Festigkeitsklasse"),
    choice("C40", "C40 — softwood strength class", "C40 — Nadelholz-Festigkeitsklasse"),
    choice("C45", "C45 — softwood strength class", "C45 — Nadelholz-Festigkeitsklasse"),
    choice("C50", "C50 — softwood strength class", "C50 — Nadelholz-Festigkeitsklasse"),
    choice("GL20h", "GL20h — glulam homogeneous", "GL20h — Brettschichtholz homogen"),
    choice("GL22h", "GL22h — glulam homogeneous", "GL22h — Brettschichtholz homogen"),
    choice("GL24h", "GL24h — glulam homogeneous", "GL24h — Brettschichtholz homogen"),
    choice("GL24c", "GL24c — glulam combined", "GL24c — Brettschichtholz kombiniert"),
    choice("GL26h", "GL26h — glulam homogeneous", "GL26h — Brettschichtholz homogen"),
    choice("GL26c", "GL26c — glulam combined", "GL26c — Brettschichtholz kombiniert"),
    choice("GL28h", "GL28h — glulam homogeneous", "GL28h — Brettschichtholz homogen"),
    choice("GL28c", "GL28c — glulam combined", "GL28c — Brettschichtholz kombiniert"),
    choice("GL30h", "GL30h — glulam homogeneous", "GL30h — Brettschichtholz homogen"),
    choice("GL30c", "GL30c — glulam combined", "GL30c — Brettschichtholz kombiniert"),
    choice("GL32h", "GL32h — glulam homogeneous", "GL32h — Brettschichtholz homogen"),
    choice("GL32c", "GL32c — glulam combined", "GL32c — Brettschichtholz kombiniert"),
    choice("LVL32", "LVL 32 — laminated veneer lumber", "LVL 32 — Furnierschichtholz"),
    choice("CLT100", "CLT 100 — cross-laminated timber", "BSP 100 — Brettsperrholz"),
];
const SERVICE: &[NormFieldChoice] = &[
    choice("1", "Service class 1 — heated interior", "Nutzungsklasse 1 — beheizter Innenraum"),
    choice("2", "Service class 2 — sheltered exterior", "Nutzungsklasse 2 — überdachter Außenbereich"),
    choice("3", "Service class 3 — exposed to weather", "Nutzungsklasse 3 — der Witterung ausgesetzt"),
];
const ROLE: &[NormFieldChoice] = &[
    choice("beam", "Beam — bending member", "Träger — Biegebauteil"),
    choice("column", "Column — compression member", "Stütze — Druckbauteil"),
    choice("floor", "Floor — joist or panel with vibration check", "Decke — Balken oder Platte mit Schwingungsnachweis"),
    choice("bridge", "Bridge — EN 1995-2 girder", "Brücke — Träger nach EN 1995-2"),
];
const SUPPORT: &[NormFieldChoice] = &[
    choice("simplySupported", "Simply supported", "Einfeldträger"),
    choice("cantilever", "Cantilever", "Kragarm"),
    choice("continuousTwoSpan", "Continuous over two spans", "Durchlaufträger über zwei Felder"),
];
const ACTION_KIND: &[NormFieldChoice] = &[
    choice("permanent", "Permanent (G)", "Ständig (G)"),
    choice("imposed", "Imposed (Q)", "Nutzlast (Q)"),
    choice("snow", "Snow (S)", "Schnee (S)"),
    choice("wind", "Wind (W)", "Wind (W)"),
    choice("accidental", "Accidental (A)", "Außergewöhnlich (A)"),
];
const CATEGORY: &[NormFieldChoice] = &[
    choice("", "None — not an imposed load", "Keine — keine Nutzlast"),
    choice("A", "A — domestic and residential", "A — Wohnflächen"),
    choice("B", "B — offices", "B — Büroflächen"),
    choice("C", "C — congregation areas", "C — Versammlungsflächen"),
    choice("D", "D — shopping areas", "D — Verkaufsflächen"),
    choice("E", "E — storage areas", "E — Lagerflächen"),
    choice("F", "F — light vehicle traffic", "F — Leichte Fahrzeuge"),
    choice("G", "G — medium vehicle traffic", "G — Mittelschwere Fahrzeuge"),
    choice("H", "H — roofs not accessible", "H — Nicht begehbare Dächer"),
];
const LOAD_DURATION: &[NormFieldChoice] = &[
    choice("permanent", "Permanent (> 10 years)", "Ständig (> 10 Jahre)"),
    choice("long", "Long-term (6 months – 10 years)", "Lang (6 Monate – 10 Jahre)"),
    choice("medium", "Medium-term (1 week – 6 months)", "Mittel (1 Woche – 6 Monate)"),
    choice("short", "Short-term (< 1 week)", "Kurz (< 1 Woche)"),
    choice("instantaneous", "Instantaneous", "Sehr kurz"),
];
const FASTENER: &[NormFieldChoice] = &[
    choice("nail", "Nail", "Nagel"),
    choice("screw", "Screw", "Schraube"),
    choice("bolt", "Bolt", "Bolzen"),
    choice("dowel", "Dowel", "Stabdübel"),
];
const BOOL: &[NormFieldChoice] = &[
    choice("true", "Yes", "Ja"),
    choice("false", "No", "Nein"),
];

const TABLE: &[(&str, NormFieldMeta)] = &[
    ("annex", NormFieldMeta { label_en: "National annex", label_de: "Nationaler Anhang", unit: None, choices: Some(ANNEX) }),
    ("members", NormFieldMeta { label_en: "Members", label_de: "Bauteile", unit: None, choices: None }),
    ("members[].id", NormFieldMeta { label_en: "Member id", label_de: "Bauteil-Id", unit: None, choices: None }),
    ("members[].labelEn", NormFieldMeta { label_en: "Label (EN)", label_de: "Bezeichnung (EN)", unit: None, choices: None }),
    ("members[].labelDe", NormFieldMeta { label_en: "Label (DE)", label_de: "Bezeichnung (DE)", unit: None, choices: None }),
    ("members[].role", NormFieldMeta { label_en: "Structural role", label_de: "Tragwerksrolle", unit: None, choices: Some(ROLE) }),
    ("members[].strengthClass", NormFieldMeta { label_en: "Strength class", label_de: "Festigkeitsklasse", unit: None, choices: Some(STRENGTH) }),
    ("members[].serviceClass", NormFieldMeta { label_en: "Service class", label_de: "Nutzungsklasse", unit: None, choices: Some(SERVICE) }),
    ("members[].support", NormFieldMeta { label_en: "Support type", label_de: "Lagerungsart", unit: None, choices: Some(SUPPORT) }),
    ("members[].bM", NormFieldMeta { label_en: "Width b", label_de: "Breite b", unit: Some("m"), choices: None }),
    ("members[].hM", NormFieldMeta { label_en: "Depth h", label_de: "Höhe h", unit: Some("m"), choices: None }),
    ("members[].spanM", NormFieldMeta { label_en: "Span L", label_de: "Spannweite L", unit: Some("m"), choices: None }),
    ("members[].supportLengthM", NormFieldMeta { label_en: "Support length", label_de: "Auflagerlänge", unit: Some("m"), choices: None }),
    ("members[].bearingLengthM", NormFieldMeta { label_en: "Bearing length l", label_de: "Aufstandslänge l", unit: Some("m"), choices: None }),
    ("members[].bucklingLengthYM", NormFieldMeta { label_en: "Buckling length L_y", label_de: "Knicklänge L_y", unit: Some("m"), choices: None }),
    ("members[].bucklingLengthZM", NormFieldMeta { label_en: "Buckling length L_z", label_de: "Knicklänge L_z", unit: Some("m"), choices: None }),
    ("members[].lateralRestraintSpacingM", NormFieldMeta { label_en: "Lateral restraint spacing", label_de: "Abstand seitlicher Halterungen", unit: Some("m"), choices: None }),
    ("members[].notchDepthM", NormFieldMeta { label_en: "Notch depth h_e", label_de: "Ausklinkungstiefe h_e", unit: Some("m"), choices: None }),
    ("members[].notchDistanceM", NormFieldMeta { label_en: "Notch distance x", label_de: "Ausklinkungsabstand x", unit: Some("m"), choices: None }),
    ("members[].mCritNm", NormFieldMeta { label_en: "Critical moment M_crit", label_de: "Kippmoment M_crit", unit: Some("N·m"), choices: None }),
    ("members[].massKgPerM", NormFieldMeta { label_en: "Linear mass", label_de: "Längenbezogene Masse", unit: Some("kg/m"), choices: None }),
    ("members[].massKgPerM2", NormFieldMeta { label_en: "Area mass", label_de: "Flächenbezogene Masse", unit: Some("kg/m²"), choices: None }),
    ("members[].dampingXi", NormFieldMeta { label_en: "Modal damping ratio ξ", label_de: "Modales Dämpfungsmaß ξ", unit: None, choices: None }),
    ("members[].fireDurationS", NormFieldMeta { label_en: "Fire duration", label_de: "Branddauer", unit: Some("s"), choices: None }),
    ("members[].bridgeNObs", NormFieldMeta { label_en: "Observed load cycles per year N_obs", label_de: "Beobachtete Lastzyklen pro Jahr N_obs", unit: Some("1/a"), choices: None }),
    ("members[].bridgeTLYears", NormFieldMeta { label_en: "Design working life t_L", label_de: "Nutzungsdauer t_L", unit: Some("a"), choices: None }),
    ("members[].bridgeBeta", NormFieldMeta { label_en: "Fatigue exponent β", label_de: "Ermüdungsexponent β", unit: None, choices: None }),
    ("members[].bridgeA", NormFieldMeta { label_en: "Fatigue intercept a", label_de: "Ermüdungsparameter a", unit: None, choices: None }),
    ("members[].bridgeB", NormFieldMeta { label_en: "Fatigue slope b", label_de: "Ermüdungsparameter b", unit: None, choices: None }),
    ("members[].bridgeCrowdPerM2", NormFieldMeta { label_en: "Pedestrian crowd density", label_de: "Fußgängerdichte", unit: Some("1/m²"), choices: None }),
    ("members[].actions", NormFieldMeta { label_en: "Characteristic actions", label_de: "Charakteristische Einwirkungen", unit: None, choices: None }),
    ("members[].actions[].id", NormFieldMeta { label_en: "Action id", label_de: "Einwirkungs-Id", unit: None, choices: None }),
    ("members[].actions[].kind", NormFieldMeta { label_en: "Action kind", label_de: "Einwirkungsart", unit: None, choices: Some(ACTION_KIND) }),
    ("members[].actions[].category", NormFieldMeta { label_en: "Imposed load category", label_de: "Nutzlastkategorie", unit: None, choices: Some(CATEGORY) }),
    ("members[].actions[].loadDuration", NormFieldMeta { label_en: "Load-duration class", label_de: "Klasse der Lasteinwirkungsdauer", unit: None, choices: Some(LOAD_DURATION) }),
    ("members[].actions[].qLineNPerM", NormFieldMeta { label_en: "Line load q_k", label_de: "Streckenlast q_k", unit: Some("N/m"), choices: None }),
    ("members[].actions[].fPointN", NormFieldMeta { label_en: "Point load F_k", label_de: "Einzellast F_k", unit: Some("N"), choices: None }),
    ("members[].actions[].mKNm", NormFieldMeta { label_en: "Characteristic moment M_k", label_de: "Charakteristisches Moment M_k", unit: Some("N·m"), choices: None }),
    ("members[].actions[].vKN", NormFieldMeta { label_en: "Characteristic shear V_k", label_de: "Charakteristische Querkraft V_k", unit: Some("N"), choices: None }),
    ("members[].actions[].nKN", NormFieldMeta { label_en: "Characteristic compression N_k", label_de: "Charakteristische Druckkraft N_k", unit: Some("N"), choices: None }),
    ("members[].actions[].nTKN", NormFieldMeta { label_en: "Characteristic tension N_t,k", label_de: "Charakteristische Zugkraft N_t,k", unit: Some("N"), choices: None }),
    ("members[].actions[].fC90KN", NormFieldMeta { label_en: "Characteristic compression perp. F_c,90,k", label_de: "Charakteristische Querdruckkraft F_c,90,k", unit: Some("N"), choices: None }),
    ("connections", NormFieldMeta { label_en: "Connections", label_de: "Verbindungen", unit: None, choices: None }),
    ("connections[].id", NormFieldMeta { label_en: "Connection id", label_de: "Verbindungs-Id", unit: None, choices: None }),
    ("connections[].labelEn", NormFieldMeta { label_en: "Label (EN)", label_de: "Bezeichnung (EN)", unit: None, choices: None }),
    ("connections[].labelDe", NormFieldMeta { label_en: "Label (DE)", label_de: "Bezeichnung (DE)", unit: None, choices: None }),
    ("connections[].fastenerType", NormFieldMeta { label_en: "Fastener type", label_de: "Verbindungsmittel", unit: None, choices: Some(FASTENER) }),
    ("connections[].strengthClass", NormFieldMeta { label_en: "Strength class", label_de: "Festigkeitsklasse", unit: None, choices: Some(STRENGTH) }),
    ("connections[].serviceClass", NormFieldMeta { label_en: "Service class", label_de: "Nutzungsklasse", unit: None, choices: Some(SERVICE) }),
    ("connections[].diameterM", NormFieldMeta { label_en: "Fastener diameter d", label_de: "Durchmesser d", unit: Some("m"), choices: None }),
    ("connections[].number", NormFieldMeta { label_en: "Number of fasteners", label_de: "Anzahl Verbindungsmittel", unit: None, choices: None }),
    ("connections[].rows", NormFieldMeta { label_en: "Rows", label_de: "Reihen", unit: None, choices: None }),
    ("connections[].spacingM", NormFieldMeta { label_en: "Spacing a₁", label_de: "Abstand a₁", unit: Some("m"), choices: None }),
    ("connections[].edgeDistanceM", NormFieldMeta { label_en: "Edge distance a₄,t", label_de: "Randabstand a₄,t", unit: Some("m"), choices: None }),
    ("connections[].endDistanceM", NormFieldMeta { label_en: "End distance a₃,t", label_de: "Hirnholzabstand a₃,t", unit: Some("m"), choices: None }),
    ("connections[].t1M", NormFieldMeta { label_en: "Member thickness t₁", label_de: "Bauteildicke t₁", unit: Some("m"), choices: None }),
    ("connections[].t2M", NormFieldMeta { label_en: "Member thickness t₂", label_de: "Bauteildicke t₂", unit: Some("m"), choices: None }),
    ("connections[].steelPlate", NormFieldMeta { label_en: "Steel plate", label_de: "Stahlblech", unit: None, choices: Some(BOOL) }),
    ("connections[].steelPlateThicknessM", NormFieldMeta { label_en: "Steel plate thickness", label_de: "Stahlblechdicke", unit: Some("m"), choices: None }),
    ("connections[].shearPlanes", NormFieldMeta { label_en: "Shear planes", label_de: "Scherflächen", unit: None, choices: None }),
    ("connections[].fUK", NormFieldMeta { label_en: "Fastener tensile strength f_u,k", label_de: "Zugfestigkeit f_u,k", unit: Some("Pa"), choices: None }),
    ("connections[].actions", NormFieldMeta { label_en: "Characteristic fastener actions", label_de: "Charakteristische Verbindungsmitteleinwirkungen", unit: None, choices: None }),
    ("connections[].actions[].id", NormFieldMeta { label_en: "Action id", label_de: "Einwirkungs-Id", unit: None, choices: None }),
    ("connections[].actions[].kind", NormFieldMeta { label_en: "Action kind", label_de: "Einwirkungsart", unit: None, choices: Some(ACTION_KIND) }),
    ("connections[].actions[].loadDuration", NormFieldMeta { label_en: "Load-duration class", label_de: "Klasse der Lasteinwirkungsdauer", unit: None, choices: Some(LOAD_DURATION) }),
    ("connections[].actions[].fKN", NormFieldMeta { label_en: "Characteristic fastener force F_k", label_de: "Charakteristische Verbindungsmittelkraft F_k", unit: Some("N"), choices: None }),
];

/// 🏷️ Exact / `[]`-wildcard metadata for the EN 1995 timber subject.
pub fn en1995_field_meta(path: &str) -> Option<NormFieldMeta> {
    lookup_norm_field_meta(TABLE, path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_editable_leaf_has_meta_with_both_labels() {
        for (path, meta) in TABLE {
            assert!(!meta.label_en.is_empty() && !meta.label_de.is_empty(), "{path}");
            let resolved = en1995_field_meta(&path.replace("[]", "[0]")).unwrap_or_else(|| panic!("{path} must resolve through an index"));
            assert_eq!(resolved.label_en, meta.label_en);
        }
        assert!(en1995_field_meta("members[3].actions[1].qLineNPerM").is_some_and(|m| m.unit == Some("N/m")));
        assert!(en1995_field_meta("connections[0].actions[0].fKN").is_some_and(|m| m.unit == Some("N")));
        assert!(en1995_field_meta("members[0].strengthClass").is_some_and(|m| m.choices.is_some_and(|c| c.iter().any(|x| x.value == "GL24h" && x.label_de.contains("Brettschichtholz")))));
        assert!(en1995_field_meta("members[0].role").is_some_and(|m| m.choices.is_some_and(|c| c.len() == 4)));
    }

    #[test]
    fn strength_choices_are_exactly_the_tabulated_classes() {
        let values: Vec<&str> = STRENGTH.iter().map(|c| c.value).collect();
        assert_eq!(values, crate::artifact_schema::strength_class_options());
        for value in values {
            assert!(crate::artifact_schema::properties_for_class(value).is_some(), "{value} must be tabulated");
        }
    }
}
