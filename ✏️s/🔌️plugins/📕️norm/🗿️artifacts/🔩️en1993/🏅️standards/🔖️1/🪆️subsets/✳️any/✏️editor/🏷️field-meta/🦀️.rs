//! 🏷️ EN 1993 NormFieldMeta — SI display units + en/de labels (longest-prefix + `[]` wildcards).

use crate::app_surface::{lookup_norm_field_meta, NormFieldChoice, NormFieldMeta};

const fn choice(value: &'static str, label_en: &'static str, label_de: &'static str) -> NormFieldChoice {
    NormFieldChoice { value, label_en, label_de }
}

const ANNEX: &[NormFieldChoice] = &[
    choice("en", "EN (CEN)", "EN (CEN)"),
    choice("de", "Germany (DIN)", "Deutschland (DIN)"),
];
const ACTION_KIND: &[NormFieldChoice] = &[
    choice("permanent", "Permanent G", "Ständig G"),
    choice("imposed", "Imposed Q", "Nutzlast Q"),
    choice("wind", "Wind", "Wind"),
    choice("snow", "Snow", "Schnee"),
    choice("accidental", "Accidental", "Außergewöhnlich"),
];
const ACTION_CATEGORY: &[NormFieldChoice] = &[
    choice("self", "Self-weight", "Eigengewicht"),
    choice("office", "Office / residential", "Büro / Wohnen"),
    choice("shopping", "Shopping / storage", "Verkauf / Lager"),
    choice("wind", "Wind", "Wind"),
    choice("snow", "Snow", "Schnee"),
];
const MEMBER_TYPE: &[NormFieldChoice] = &[
    choice("beam", "Beam", "Träger"),
    choice("column", "Column", "Stütze"),
    choice("beamColumn", "Beam-column", "Biegedruckstab"),
    choice("brace", "Brace", "Verband"),
    choice("tie", "Tie", "Zugstab"),
];
const LOAD_APP: &[NormFieldChoice] = &[
    choice("shearCenter", "Shear centre", "Schubmittelpunkt"),
    choice("topFlange", "Top flange", "Obergurt"),
    choice("bottomFlange", "Bottom flange", "Untergurt"),
];
const ANALYSIS: &[NormFieldChoice] = &[
    choice("elastic", "Elastic", "Elastisch"),
    choice("plastic", "Plastic", "Plastisch"),
];
const MOMENT_DIAGRAM: &[NormFieldChoice] = &[
    choice("linear", "Linear", "Linear"),
    choice("uniform", "Uniform", "Gleichförmig"),
    choice("parabolic", "Parabolic", "Parabolisch"),
];
const MATERIAL_KIND: &[NormFieldChoice] = &[
    choice("carbon", "Carbon steel", "Baustahl"),
    choice("stainless", "Stainless steel", "Nichtrostender Stahl"),
];
const JOINT_KIND: &[NormFieldChoice] = &[
    choice("bolted", "Bolted", "Geschraubt"),
    choice("welded", "Welded", "Geschweißt"),
];
const JOINT_CATEGORY: &[NormFieldChoice] = &[
    choice("A", "Category A — bearing", "Kategorie A — Scher-/Lochleibung"),
    choice("B", "Category B — slip SLS", "Kategorie B — Gleiten GZG"),
    choice("C", "Category C — slip ULS", "Kategorie C — Gleiten GZT"),
];
const FATIGUE_METHOD: &[NormFieldChoice] = &[
    choice("damage_tolerant", "Damage tolerant", "Schadenstolerant"),
    choice("safe_life", "Safe life", "Safe-Life"),
];
const FIRE_RATING: &[NormFieldChoice] = &[
    choice("r30", "R30", "R30"),
    choice("r60", "R60", "R60"),
    choice("r90", "R90", "R90"),
    choice("r120", "R120", "R120"),
];
const SECTION_KIND: &[NormFieldChoice] = &[
    choice("rolledI", "Rolled I", "Walzprofil I"),
    choice("weldedI", "Welded I", "Schweißprofil I"),
    choice("hss", "Hollow section", "Hohlprofil"),
];

const TABLE: &[(&str, NormFieldMeta)] = &[
    ("annex", NormFieldMeta { label_en: "National annex", label_de: "Nationaler Anhang", unit: None, choices: Some(ANNEX) }),
    ("materials", NormFieldMeta { label_en: "Materials", label_de: "Werkstoffe", unit: None, choices: None }),
    ("materials[].id", NormFieldMeta { label_en: "Material id", label_de: "Werkstoff-Id", unit: None, choices: None }),
    ("materials[].grade", NormFieldMeta { label_en: "Steel grade", label_de: "Stahlgüte", unit: None, choices: None }),
    ("materials[].fy", NormFieldMeta { label_en: "Yield strength f_y", label_de: "Streckgrenze f_y", unit: Some("Pa"), choices: None }),
    ("materials[].fu", NormFieldMeta { label_en: "Ultimate strength f_u", label_de: "Zugfestigkeit f_u", unit: Some("Pa"), choices: None }),
    ("materials[].eModulus", NormFieldMeta { label_en: "Modulus E", label_de: "Elastizitätsmodul E", unit: Some("Pa"), choices: None }),
    ("materials[].gModulus", NormFieldMeta { label_en: "Shear modulus G", label_de: "Schubmodul G", unit: Some("Pa"), choices: None }),
    ("materials[].subgrade", NormFieldMeta { label_en: "Subgrade", label_de: "Stahluntergüte", unit: None, choices: None }),
    ("materials[].kind", NormFieldMeta { label_en: "Material kind", label_de: "Werkstoffart", unit: None, choices: Some(MATERIAL_KIND) }),
    ("sections", NormFieldMeta { label_en: "Sections", label_de: "Querschnitte", unit: None, choices: None }),
    ("sections[].id", NormFieldMeta { label_en: "Section id", label_de: "Querschnitt-Id", unit: None, choices: None }),
    ("sections[].designation", NormFieldMeta { label_en: "Designation", label_de: "Bezeichnung", unit: None, choices: None }),
    ("sections[].kind", NormFieldMeta { label_en: "Section kind", label_de: "Querschnittsart", unit: None, choices: Some(SECTION_KIND) }),
    ("sections[].h", NormFieldMeta { label_en: "Depth h", label_de: "Höhe h", unit: Some("m"), choices: None }),
    ("sections[].b", NormFieldMeta { label_en: "Width b", label_de: "Breite b", unit: Some("m"), choices: None }),
    ("sections[].tw", NormFieldMeta { label_en: "Web thickness t_w", label_de: "Stegdicke t_w", unit: Some("m"), choices: None }),
    ("sections[].tf", NormFieldMeta { label_en: "Flange thickness t_f", label_de: "Flanschdicke t_f", unit: Some("m"), choices: None }),
    ("sections[].r", NormFieldMeta { label_en: "Root radius r", label_de: "Ausrundungsradius r", unit: Some("m"), choices: None }),
    ("sections[].area", NormFieldMeta { label_en: "Area A", label_de: "Fläche A", unit: Some("m²"), choices: None }),
    ("sections[].shearAreaY", NormFieldMeta { label_en: "Shear area A_v,y", label_de: "Schubfläche A_v,y", unit: Some("m²"), choices: None }),
    ("sections[].shearAreaZ", NormFieldMeta { label_en: "Shear area A_v,z", label_de: "Schubfläche A_v,z", unit: Some("m²"), choices: None }),
    ("sections[].iy", NormFieldMeta { label_en: "Second moment I_y", label_de: "Trägheitsmoment I_y", unit: Some("m⁴"), choices: None }),
    ("sections[].iz", NormFieldMeta { label_en: "Second moment I_z", label_de: "Trägheitsmoment I_z", unit: Some("m⁴"), choices: None }),
    ("sections[].it", NormFieldMeta { label_en: "Torsion constant I_t", label_de: "Torsionsträgheitsmoment I_t", unit: Some("m⁴"), choices: None }),
    ("sections[].iw", NormFieldMeta { label_en: "Warping constant I_w", label_de: "Wölbwiderstand I_w", unit: Some("m⁶"), choices: None }),
    ("sections[].wElY", NormFieldMeta { label_en: "Elastic modulus W_el,y", label_de: "Elastisches Widerstandsmoment W_el,y", unit: Some("m³"), choices: None }),
    ("sections[].wElZ", NormFieldMeta { label_en: "Elastic modulus W_el,z", label_de: "Elastisches Widerstandsmoment W_el,z", unit: Some("m³"), choices: None }),
    ("sections[].wPlY", NormFieldMeta { label_en: "Plastic modulus W_pl,y", label_de: "Plastisches Widerstandsmoment W_pl,y", unit: Some("m³"), choices: None }),
    ("sections[].wPlZ", NormFieldMeta { label_en: "Plastic modulus W_pl,z", label_de: "Plastisches Widerstandsmoment W_pl,z", unit: Some("m³"), choices: None }),
    ("sections[].areaNet", NormFieldMeta { label_en: "Net area A_net", label_de: "Nettofläche A_net", unit: Some("m²"), choices: None }),
    ("members", NormFieldMeta { label_en: "Members", label_de: "Bauteile", unit: None, choices: None }),
    ("members[].id", NormFieldMeta { label_en: "Member id", label_de: "Bauteil-Id", unit: None, choices: None }),
    ("members[].label", NormFieldMeta { label_en: "Label", label_de: "Bezeichnung", unit: None, choices: None }),
    ("members[].memberType", NormFieldMeta { label_en: "Member type", label_de: "Bauteiltyp", unit: None, choices: Some(MEMBER_TYPE) }),
    ("members[].sectionId", NormFieldMeta { label_en: "Section id", label_de: "Querschnitt-Id", unit: None, choices: None }),
    ("members[].materialId", NormFieldMeta { label_en: "Material id", label_de: "Werkstoff-Id", unit: None, choices: None }),
    ("members[].length", NormFieldMeta { label_en: "Length L", label_de: "Länge L", unit: Some("m"), choices: None }),
    ("members[].bucklingLengthY", NormFieldMeta { label_en: "Buckling length L_cr,y", label_de: "Knicklänge L_cr,y", unit: Some("m"), choices: None }),
    ("members[].bucklingLengthZ", NormFieldMeta { label_en: "Buckling length L_cr,z", label_de: "Knicklänge L_cr,z", unit: Some("m"), choices: None }),
    ("members[].ltbLength", NormFieldMeta { label_en: "LTB length L_cr,LT", label_de: "Kipp-Länge L_cr,LT", unit: Some("m"), choices: None }),
    ("members[].ltbRestraintSpacing", NormFieldMeta { label_en: "LTB restraint spacing", label_de: "Kipp-Halterungsabstand", unit: Some("m"), choices: None }),
    ("members[].loadApplication", NormFieldMeta { label_en: "Load application", label_de: "Lastangriff", unit: None, choices: Some(LOAD_APP) }),
    ("members[].endMomentRatioPsi", NormFieldMeta { label_en: "End-moment ratio ψ", label_de: "Endmomentenverhältnis ψ", unit: None, choices: None }),
    ("members[].momentDiagram", NormFieldMeta { label_en: "Moment diagram", label_de: "Momentenverlauf", unit: None, choices: Some(MOMENT_DIAGRAM) }),
    ("members[].deflectionLimitRatio", NormFieldMeta { label_en: "Deflection limit L/δ", label_de: "Durchbiegungsgrenze L/δ", unit: None, choices: None }),
    ("loadCases", NormFieldMeta { label_en: "Load cases", label_de: "Lastfälle", unit: None, choices: None }),
    ("loadCases[].id", NormFieldMeta { label_en: "Load-case id", label_de: "Lastfall-Id", unit: None, choices: None }),
    ("loadCases[].name", NormFieldMeta { label_en: "Name", label_de: "Name", unit: None, choices: None }),
    ("loadCases[].kind", NormFieldMeta { label_en: "Action kind", label_de: "Einwirkungsart", unit: None, choices: Some(ACTION_KIND) }),
    ("loadCases[].category", NormFieldMeta { label_en: "ψ category", label_de: "ψ-Kategorie", unit: None, choices: Some(ACTION_CATEGORY) }),
    ("memberActions", NormFieldMeta { label_en: "Member actions", label_de: "Bauteilbeanspruchungen", unit: None, choices: None }),
    ("memberActions[].id", NormFieldMeta { label_en: "Action id", label_de: "Beanspruchungs-Id", unit: None, choices: None }),
    ("memberActions[].memberId", NormFieldMeta { label_en: "Member id", label_de: "Bauteil-Id", unit: None, choices: None }),
    ("memberActions[].loadCaseId", NormFieldMeta { label_en: "Load-case id", label_de: "Lastfall-Id", unit: None, choices: None }),
    ("memberActions[].action", NormFieldMeta { label_en: "Design action", label_de: "Bemessungsschnittgrößen", unit: None, choices: None }),
    ("memberActions[].action.n", NormFieldMeta { label_en: "Characteristic N_k", label_de: "Charakteristische N_k", unit: Some("N"), choices: None }),
    ("memberActions[].action.vy", NormFieldMeta { label_en: "Characteristic V_y,k", label_de: "Charakteristische V_y,k", unit: Some("N"), choices: None }),
    ("memberActions[].action.vz", NormFieldMeta { label_en: "Characteristic V_z,k", label_de: "Charakteristische V_z,k", unit: Some("N"), choices: None }),
    ("memberActions[].action.my", NormFieldMeta { label_en: "Characteristic M_y,k", label_de: "Charakteristisches M_y,k", unit: Some("N·m"), choices: None }),
    ("memberActions[].action.mz", NormFieldMeta { label_en: "Characteristic M_z,k", label_de: "Charakteristisches M_z,k", unit: Some("N·m"), choices: None }),
    ("memberActions[].action.t", NormFieldMeta { label_en: "Characteristic T_k", label_de: "Charakteristische T_k", unit: Some("N·m"), choices: None }),
    ("joints", NormFieldMeta { label_en: "Joints", label_de: "Anschlüsse", unit: None, choices: None }),
    ("joints[].id", NormFieldMeta { label_en: "Joint id", label_de: "Anschluss-Id", unit: None, choices: None }),
    ("joints[].kind", NormFieldMeta { label_en: "Joint kind", label_de: "Anschlussart", unit: None, choices: Some(JOINT_KIND) }),
    ("joints[].memberId", NormFieldMeta { label_en: "Member id", label_de: "Bauteil-Id", unit: None, choices: None }),
    ("joints[].boltClass", NormFieldMeta { label_en: "Bolt class", label_de: "Schraubenfestigkeitsklasse", unit: None, choices: None }),
    ("joints[].boltDiameter", NormFieldMeta { label_en: "Bolt diameter d", label_de: "Schraubendurchmesser d", unit: Some("m"), choices: None }),
    ("joints[].boltRows", NormFieldMeta { label_en: "Bolt rows", label_de: "Schraubenreihen", unit: None, choices: None }),
    ("joints[].boltsPerRow", NormFieldMeta { label_en: "Bolts per row", label_de: "Schrauben je Reihe", unit: None, choices: None }),
    ("joints[].pitch", NormFieldMeta { label_en: "Pitch p", label_de: "Lochabstand p", unit: Some("m"), choices: None }),
    ("joints[].endDistance", NormFieldMeta { label_en: "End distance e1", label_de: "Randabstand e1", unit: Some("m"), choices: None }),
    ("joints[].edgeDistance", NormFieldMeta { label_en: "Edge distance e2", label_de: "Randabstand e2", unit: Some("m"), choices: None }),
    ("joints[].shearPlanes", NormFieldMeta { label_en: "Shear planes", label_de: "Scherflächen", unit: None, choices: None }),
    ("joints[].plateThickness", NormFieldMeta { label_en: "Plate thickness", label_de: "Blechdicke", unit: Some("m"), choices: None }),
    ("joints[].plateFu", NormFieldMeta { label_en: "Plate f_u", label_de: "Blech f_u", unit: Some("Pa"), choices: None }),
    ("joints[].weldThroat", NormFieldMeta { label_en: "Weld throat a", label_de: "Nahtdicke a", unit: Some("m"), choices: None }),
    ("joints[].weldLength", NormFieldMeta { label_en: "Weld length ℓ", label_de: "Nahtlänge ℓ", unit: Some("m"), choices: None }),
    ("joints[].weldFu", NormFieldMeta { label_en: "Weld f_u", label_de: "Naht f_u", unit: Some("Pa"), choices: None }),
    ("joints[].weldGrade", NormFieldMeta { label_en: "Weld grade", label_de: "Nahtgüte", unit: None, choices: None }),
    ("joints[].category", NormFieldMeta { label_en: "Connection category", label_de: "Anschlusskategorie", unit: None, choices: Some(JOINT_CATEGORY) }),
    ("joints[].frictionMu", NormFieldMeta { label_en: "Friction coefficient μ", label_de: "Reibungszahl μ", unit: None, choices: None }),
    ("joints[].preloadForce", NormFieldMeta { label_en: "Preload F_p,C", label_de: "Vorspannkraft F_p,C", unit: Some("N"), choices: None }),
    ("joints[].slipFactorKs", NormFieldMeta { label_en: "Slip factor k_s", label_de: "Gleitfaktor k_s", unit: None, choices: None }),
    ("joints[].frictionSurfaces", NormFieldMeta { label_en: "Friction surfaces n", label_de: "Reibflächen n", unit: None, choices: None }),
    ("fatigueDetails", NormFieldMeta { label_en: "Fatigue details", label_de: "Ermüdungsdetails", unit: None, choices: None }),
    ("fatigueDetails[].id", NormFieldMeta { label_en: "Detail id", label_de: "Detail-Id", unit: None, choices: None }),
    ("fatigueDetails[].memberId", NormFieldMeta { label_en: "Member id", label_de: "Bauteil-Id", unit: None, choices: None }),
    ("fatigueDetails[].category", NormFieldMeta { label_en: "Detail category", label_de: "Kerbfall", unit: None, choices: None }),
    ("fatigueDetails[].method", NormFieldMeta { label_en: "Assessment method", label_de: "Nachweisverfahren", unit: None, choices: Some(FATIGUE_METHOD) }),
    ("fireExposures", NormFieldMeta { label_en: "Fire exposures", label_de: "Brandbeanspruchungen", unit: None, choices: None }),
    ("fireExposures[].id", NormFieldMeta { label_en: "Fire id", label_de: "Brand-Id", unit: None, choices: None }),
    ("fireExposures[].memberId", NormFieldMeta { label_en: "Member id", label_de: "Bauteil-Id", unit: None, choices: None }),
    ("fireExposures[].rating", NormFieldMeta { label_en: "Fire rating", label_de: "Feuerwiderstand", unit: None, choices: Some(FIRE_RATING) }),
    ("fireExposures[].protectionThickness", NormFieldMeta { label_en: "Protection thickness d_p", label_de: "Brandschutzdicke d_p", unit: Some("m"), choices: None }),
    ("fireExposures[].sectionFactor", NormFieldMeta { label_en: "Section factor A/V", label_de: "Profilfaktor A/V", unit: Some("1/m"), choices: None }),
    ("fireExposures[].mu0", NormFieldMeta { label_en: "Load level μ₀", label_de: "Ausnutzungsgrad μ₀", unit: None, choices: None }),
    ("fireExposures[].protectionConductivity", NormFieldMeta { label_en: "Protection λ_p", label_de: "Brandschutz λ_p", unit: Some("W/(m·K)"), choices: None }),
    ("fireExposures[].protectionDensity", NormFieldMeta { label_en: "Protection ρ_p", label_de: "Brandschutz ρ_p", unit: Some("kg/m³"), choices: None }),
    ("fireExposures[].protectionSpecificHeat", NormFieldMeta { label_en: "Protection c_p", label_de: "Brandschutz c_p", unit: Some("J/(kg·K)"), choices: None }),
    ("coldFormedMembers", NormFieldMeta { label_en: "Cold-formed members", label_de: "Kaltprofile", unit: None, choices: None }),
    ("coldFormedMembers[].id", NormFieldMeta { label_en: "Id", label_de: "Id", unit: None, choices: None }),
    ("coldFormedMembers[].bBar", NormFieldMeta { label_en: "Flat width b̄", label_de: "ebene Breite b̄", unit: Some("m"), choices: None }),
    ("coldFormedMembers[].thickness", NormFieldMeta { label_en: "Thickness t", label_de: "Dicke t", unit: Some("m"), choices: None }),
    ("coldFormedMembers[].kSigma", NormFieldMeta { label_en: "Buckling factor k_σ", label_de: "Beulfaktor k_σ", unit: None, choices: None }),
    ("coldFormedMembers[].psi", NormFieldMeta { label_en: "Stress ratio ψ", label_de: "Spannungsverhältnis ψ", unit: None, choices: None }),
    ("coldFormedMembers[].fy", NormFieldMeta { label_en: "f_y", label_de: "f_y", unit: Some("Pa"), choices: None }),
    ("coldFormedMembers[].grossResistance", NormFieldMeta { label_en: "Gross resistance", label_de: "Bruttotragfähigkeit", unit: Some("N"), choices: None }),
    ("platedPanels", NormFieldMeta { label_en: "Plated panels", label_de: "Beulpanels", unit: None, choices: None }),
    ("platedPanels[].id", NormFieldMeta { label_en: "Id", label_de: "Id", unit: None, choices: None }),
    ("platedPanels[].a", NormFieldMeta { label_en: "Length a", label_de: "Länge a", unit: Some("m"), choices: None }),
    ("platedPanels[].b", NormFieldMeta { label_en: "Width b", label_de: "Breite b", unit: Some("m"), choices: None }),
    ("platedPanels[].thickness", NormFieldMeta { label_en: "Thickness t", label_de: "Dicke t", unit: Some("m"), choices: None }),
    ("platedPanels[].fy", NormFieldMeta { label_en: "f_y", label_de: "f_y", unit: Some("Pa"), choices: None }),
    ("platedPanels[].kSigma", NormFieldMeta { label_en: "k_σ", label_de: "k_σ", unit: None, choices: None }),
    ("siloShells", NormFieldMeta { label_en: "Silo shells", label_de: "Siloschalen", unit: None, choices: None }),
    ("siloShells[].id", NormFieldMeta { label_en: "Id", label_de: "Id", unit: None, choices: None }),
    ("siloShells[].thickness", NormFieldMeta { label_en: "Thickness t", label_de: "Dicke t", unit: Some("m"), choices: None }),
    ("siloShells[].radius", NormFieldMeta { label_en: "Radius r", label_de: "Radius r", unit: Some("m"), choices: None }),
    ("siloShells[].depth", NormFieldMeta { label_en: "Depth z", label_de: "Tiefe z", unit: Some("m"), choices: None }),
    ("siloShells[].k", NormFieldMeta { label_en: "Lateral pressure ratio k", label_de: "Seitendruckbeiwert k", unit: None, choices: None }),
    ("siloShells[].gamma", NormFieldMeta { label_en: "Bulk unit weight γ", label_de: "Wichte γ", unit: Some("N/m³"), choices: None }),
    ("siloShells[].fy", NormFieldMeta { label_en: "f_y", label_de: "f_y", unit: Some("Pa"), choices: None }),
    ("tensionComponents", NormFieldMeta { label_en: "Tension components", label_de: "Zugkomponenten", unit: None, choices: None }),
    ("tensionComponents[].id", NormFieldMeta { label_en: "Id", label_de: "Id", unit: None, choices: None }),
    ("tensionComponents[].fUk", NormFieldMeta { label_en: "f_uk", label_de: "f_uk", unit: Some("Pa"), choices: None }),
    ("tensionComponents[].fK", NormFieldMeta { label_en: "F_k", label_de: "F_k", unit: Some("N"), choices: None }),
    ("bridgeFatigue", NormFieldMeta { label_en: "Bridge fatigue", label_de: "Brückenermüdung", unit: None, choices: None }),
    ("bridgeFatigue[].id", NormFieldMeta { label_en: "Id", label_de: "Id", unit: None, choices: None }),
    ("bridgeFatigue[].memberId", NormFieldMeta { label_en: "Member id", label_de: "Bauteil-Id", unit: None, choices: None }),
    ("bridgeFatigue[].lambda", NormFieldMeta { label_en: "λ", label_de: "λ", unit: None, choices: None }),
    ("bridgeFatigue[].phi2", NormFieldMeta { label_en: "φ₂", label_de: "φ₂", unit: None, choices: None }),
    ("bridgeFatigue[].deltaSigmaP", NormFieldMeta { label_en: "Δσ_p", label_de: "Δσ_p", unit: Some("Pa"), choices: None }),
    ("bridgeFatigue[].category", NormFieldMeta { label_en: "Detail category", label_de: "Kerbfall", unit: None, choices: None }),
    ("bridgeFatigue[].method", NormFieldMeta { label_en: "Assessment method", label_de: "Nachweisverfahren", unit: None, choices: Some(FATIGUE_METHOD) }),
    ("towerLegs", NormFieldMeta { label_en: "Tower legs", label_de: "Turmstabe", unit: None, choices: None }),
    ("towerLegs[].id", NormFieldMeta { label_en: "Id", label_de: "Id", unit: None, choices: None }),
    ("towerLegs[].memberId", NormFieldMeta { label_en: "Member id", label_de: "Bauteil-Id", unit: None, choices: None }),
    ("towerLegs[].forceCoefficient", NormFieldMeta { label_en: "Force coefficient c_f (Annex B)", label_de: "Kraftbeiwert c_f (Anhang B)", unit: None, choices: None }),
    ("towerLegs[].dynamicFactor", NormFieldMeta { label_en: "Dynamic factor c_d", label_de: "Dynamikfaktor c_d", unit: None, choices: None }),
    ("piles", NormFieldMeta { label_en: "Piles", label_de: "Pfähle", unit: None, choices: None }),
    ("piles[].id", NormFieldMeta { label_en: "Id", label_de: "Id", unit: None, choices: None }),
    ("piles[].sectionId", NormFieldMeta { label_en: "Section id", label_de: "Querschnitt-Id", unit: None, choices: None }),
    ("piles[].materialId", NormFieldMeta { label_en: "Material id", label_de: "Werkstoff-Id", unit: None, choices: None }),
    ("piles[].drivingStress", NormFieldMeta { label_en: "Driving stress", label_de: "Rammspannung", unit: Some("Pa"), choices: None }),
    ("piles[].embeddedLength", NormFieldMeta { label_en: "Embedded length", label_de: "Einbindelänge", unit: Some("m"), choices: None }),
    ("piles[].shaftPerimeter", NormFieldMeta { label_en: "Shaft perimeter", label_de: "Schaftumfang", unit: Some("m"), choices: None }),
    ("craneRunways", NormFieldMeta { label_en: "Crane runways", label_de: "Kranbahnen", unit: None, choices: None }),
    ("craneRunways[].id", NormFieldMeta { label_en: "Id", label_de: "Id", unit: None, choices: None }),
    ("craneRunways[].memberId", NormFieldMeta { label_en: "Member id", label_de: "Bauteil-Id", unit: None, choices: None }),
    ("craneRunways[].wheelContactLength", NormFieldMeta { label_en: "Wheel contact length", label_de: "Radaufstandsbreite", unit: Some("m"), choices: None }),
    ("craneRunways[].dispersion", NormFieldMeta { label_en: "Dispersion", label_de: "Lastausbreitung", unit: Some("m"), choices: None }),
    ("craneRunways[].webThickness", NormFieldMeta { label_en: "Web thickness", label_de: "Stegdicke", unit: Some("m"), choices: None }),
    ("craneRunways[].fy", NormFieldMeta { label_en: "f_y", label_de: "f_y", unit: Some("Pa"), choices: None }),

    ("joints[].gauge", NormFieldMeta { label_en: "Gauge p2", label_de: "Lochabstand p2", unit: Some("m"), choices: None }),
    ("joints[].actions", NormFieldMeta { label_en: "Joint actions", label_de: "Anschluss-Einwirkungen", unit: None, choices: None }),
    ("joints[].actions[].shear", NormFieldMeta { label_en: "Characteristic shear", label_de: "Charakteristische Querkraft", unit: Some("N"), choices: None }),
    ("joints[].actions[].tension", NormFieldMeta { label_en: "Characteristic tension", label_de: "Charakteristische Zugkraft", unit: Some("N"), choices: None }),
    ("joints[].actions[].loadCaseId", NormFieldMeta { label_en: "Load case", label_de: "Lastfall", unit: None, choices: None }),
    ("fatigueDetails[].spectrum", NormFieldMeta { label_en: "Stress-range spectrum", label_de: "Spannungskollektiv", unit: None, choices: None }),
    ("fatigueDetails[].spectrum[].deltaSigma", NormFieldMeta { label_en: "Stress range Δσ", label_de: "Spannungsschwingbreite Δσ", unit: Some("Pa"), choices: None }),
    ("fatigueDetails[].spectrum[].cycles", NormFieldMeta { label_en: "Cycles n", label_de: "Lastspiele n", unit: None, choices: None }),
    ("members[].analysis", NormFieldMeta { label_en: "Analysis type", label_de: "Schnittgrößenermittlung", unit: None, choices: Some(ANALYSIS) }),
    ("coldFormedMembers[].actions", NormFieldMeta { label_en: "Actions", label_de: "Einwirkungen", unit: None, choices: None }),
    ("platedPanels[].actions", NormFieldMeta { label_en: "Actions", label_de: "Einwirkungen", unit: None, choices: None }),
    ("tensionComponents[].actions", NormFieldMeta { label_en: "Actions", label_de: "Einwirkungen", unit: None, choices: None }),
    ("towerLegs[].actions", NormFieldMeta { label_en: "Actions", label_de: "Einwirkungen", unit: None, choices: None }),
    ("piles[].actions", NormFieldMeta { label_en: "Actions", label_de: "Einwirkungen", unit: None, choices: None }),
    ("craneRunways[].actions", NormFieldMeta { label_en: "Wheel load actions", label_de: "Radlast-Einwirkungen", unit: None, choices: None }),
    ("craneRunways[].phi", NormFieldMeta { label_en: "Dynamic factor φ", label_de: "Dynamikfaktor φ", unit: None, choices: None }),

];

/// 🏷️ Exact / `[]`-wildcard / longest-prefix metadata for the EN 1993 steel subject.
pub fn en1993_field_meta(path: &str) -> Option<NormFieldMeta> {
    lookup_norm_field_meta(TABLE, path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::En1993Snapshot;

    fn to_template(path: &str) -> String {
        let mut out = String::new();
        let mut chars = path.chars().peekable();
        while let Some(c) = chars.next() {
            if c == '[' {
                out.push_str("[]");
                while let Some(x) = chars.next() {
                    if x == ']' {
                        break;
                    }
                }
            } else {
                out.push(c);
            }
        }
        out
    }

    fn walk(path: &str, value: &serde_json::Value, missing: &mut Vec<String>) {
        match value {
            serde_json::Value::Object(map) => {
                for (k, v) in map {
                    let child = if path.is_empty() { k.clone() } else { format!("{path}.{k}") };
                    if matches!(v, serde_json::Value::Object(_) | serde_json::Value::Array(_)) {
                        if en1993_field_meta(&child).or_else(|| en1993_field_meta(&to_template(&child))).is_none() {
                            // containers still need labels
                            missing.push(child.clone());
                        }
                        walk(&child, v, missing);
                    } else {
                        let meta = en1993_field_meta(&child).or_else(|| en1993_field_meta(&to_template(&child)));
                        if meta.is_none() {
                            missing.push(child);
                        } else if let Some(meta) = meta {
                            assert!(!meta.label_en.is_empty() && !meta.label_de.is_empty(), "{child}");
                            let _ = meta.unit; // SI or explicit none
                        }
                    }
                }
            }
            serde_json::Value::Array(items) => {
                for (i, v) in items.iter().enumerate() {
                    walk(&format!("{path}[{i}]"), v, missing);
                }
            }
            _ => {}
        }
    }

    #[test]
    fn every_default_snapshot_editable_leaf_has_en_de_meta() {
        let doc = En1993Snapshot::default();
        let json = serde_json::to_value(&doc).expect("serialize");
        let mut missing = Vec::new();
        walk("", &json, &mut missing);
        assert!(missing.is_empty(), "missing field-meta for: {missing:?}");
    }

    #[test]
    fn annex_choices_are_human_labels() {
        let meta = en1993_field_meta("annex").expect("annex");
        let choices = meta.choices.expect("choices");
        assert_eq!(choices[0].label_en, "EN (CEN)");
        assert_eq!(choices[1].label_de, "Deutschland (DIN)");
    }
}
