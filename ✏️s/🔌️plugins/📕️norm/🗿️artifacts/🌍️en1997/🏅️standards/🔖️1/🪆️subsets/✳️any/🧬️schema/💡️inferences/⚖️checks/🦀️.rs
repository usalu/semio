//! ⚖️ EN 1997 compliance checks: the clause-by-clause verification behind the compliance report.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

use semio_s_plugin_norm_compliance::incremental_evaluation::Scope;
use semio_s_plugin_norm_compliance::AnnexChoice;
use semio_s_artifact_norm_en1997_model::{En1997Snapshot, FoundationLoadCase, SoilLayer, SpreadFoundation};

//#region 🔖️ComplianceHelpers
use semio_s_plugin_norm_compliance::{CheckReport, CheckResult, CheckStatus, ClauseId, LocalizedCopy, Quantity, QuantityKind, Remedy, SubjectRef};

pub use semio_s_artifact_norm_en1997_checks_catalogue::*;
pub use semio_s_artifact_norm_en1997_checks_geotechnics::*;

pub use semio_s_artifact_norm_en1997_spec_trace as spec_trace;

#[path = "🎯️ground-derivation/🦀️.rs"]
pub mod ground_derivation;

#[path = "🔭️investigation-extent/🦀️.rs"]
pub mod investigation_extent;

#[path = "🏗️project-checks/🦀️.rs"]
mod project_checks;
pub use project_checks::*;

#[cfg(test)]
#[path = "🧪️tests/📚️demo-document/🦀️.rs"]
mod demo_document;

#[cfg(test)]
#[path = "🧪️tests/🧭️spec-trace/🦀️.rs"]
mod spec_trace_tests;

#[cfg(test)]
#[path = "🧪️tests/🎯️ground-derivation/🦀️.rs"]
mod ground_derivation_tests;

#[cfg(test)]
#[path = "🧪️tests/🛟️downdrag/🦀️.rs"]
mod downdrag_tests;

#[path = "🧮️units/🦀️.rs"]
mod units;
pub use units::*;

//#region 🏷️FieldMetadata
/// 🏷️ EN 1997 FieldSpec lookup — SI units + en/de labels (index wildcards via `[]`).
pub fn lookup_field_meta(path: &str) -> Option<semio_s_plugin_norm_app_surface::FieldSpec> {
    use semio_s_plugin_norm_app_surface::{lookup_norm_field_meta, FieldRange, FieldSpec, NormFieldChoice, UnitId};

    const fn choice(value: &'static str, label_en: &'static str, label_de: &'static str) -> NormFieldChoice {
        NormFieldChoice { value, label_en, label_de }
    }

    const fn ranged(spec: FieldSpec, min: f64, max: f64) -> FieldSpec {
        FieldSpec { range: Some(FieldRange::new(min, max)), ..spec }
    }

    const TENDON: &[NormFieldChoice] = &[
        choice("strand", "Strand tendon", "Litzenanker"),
        choice("bar", "Bar tendon", "Stabanker"),
    ];
    const EMBEDDED_WALL_KIND: &[NormFieldChoice] = &[
        choice("freeEarth", "Free earth support", "Freie Auflagerung"),
        choice("cantilever", "Cantilever (fixed in the ground)", "Einspannung im Boden"),
    ];
    const LAB_TEST_KIND: &[NormFieldChoice] = &[
        choice("oedometer", "Oedometer test", "Ödometerversuch"),
        choice("triaxial", "Triaxial test", "Triaxialversuch"),
        choice("directShear", "Direct shear test", "Rahmenscherversuch"),
    ];
    const PILE_CONCRETE: &[NormFieldChoice] = &[
        choice("notGiven", "Not given (explicit modulus)", "Nicht angegeben (expliziter Modul)"),
        choice("c1215", "C12/15", "C12/15"),
        choice("c1620", "C16/20", "C16/20"),
        choice("c2025", "C20/25", "C20/25"),
        choice("c2530", "C25/30", "C25/30"),
        choice("c3037", "C30/37", "C30/37"),
        choice("c3545", "C35/45", "C35/45"),
        choice("c4050", "C40/50", "C40/50"),
        choice("c4555", "C45/55", "C45/55"),
        choice("c5060", "C50/60", "C50/60"),
    ];
    const ANNEX: &[NormFieldChoice] = &[
        choice("de", "Germany (DIN)", "Deutschland (DIN)"),
        choice("en", "EN (CEN)", "EN (CEN)"),
    ];
    const SITUATION: &[NormFieldChoice] = &[
        choice("bsP", "Persistent (BS-P)", "Ständig (BS-P)"),
        choice("bsT", "Transient (BS-T)", "Vorübergehend (BS-T)"),
        choice("bsA", "Accidental (BS-A)", "Außergewöhnlich (BS-A)"),
    ];
    const APPROACH: &[NormFieldChoice] = &[
        choice("da1str", "DA1 Combination 1 (STR)", "DA1 Kombination 1 (STR)"),
        choice("da1geo", "DA1 Combination 2 (GEO)", "DA1 Kombination 2 (GEO)"),
        choice("da2", "DA2 (A1 + M1 + R2)", "DA2 (A1 + M1 + R2)"),
        choice("da2star", "DA2* (effects factored)", "DA2* (Beanspruchungen faktorisiert)"),
        choice("da3", "DA3 (A1/A2 + M2 + R3)", "DA3 (A1/A2 + M2 + R3)"),
        choice("geo3", "GEO-3 / GZ 1C (A2 + M2 + R3)", "GEO-3 / GZ 1C (A2 + M2 + R3)"),
    ];
    const PILE_TYPE: &[NormFieldChoice] = &[
        choice("bored", "Bored pile", "Bohrpfahl"),
        choice("driven", "Driven pile", "Rammpfahl"),
        choice("cfa", "CFA pile", "Schneckenortbetonpfahl (CFA)"),
    ];
    const EP_MODE: &[NormFieldChoice] = &[
        choice("active", "Active (Rankine/Coulomb)", "Aktiv (Rankine/Coulomb)"),
        choice("atRest", "At rest (Jáky K₀)", "Ruhe (Jáky K₀)"),
        choice("increasedActive", "Increased active (DIN 4085)", "Erhöht aktiv (DIN 4085)"),
    ];
    const WALL_MOVE: &[NormFieldChoice] = &[
        choice("free", "Free (cantilever, sufficient displacement)", "Frei (Kragarm, ausreichende Verschiebung)"),
        choice("rigid", "Rigid (little displacement)", "Starr (geringe Verschiebung)"),
        choice("propped", "Propped / supported", "Gestützt"),
    ];
    const SOIL: &[NormFieldChoice] = &[
        choice("sand", "Sand", "Sand"),
        choice("gravel", "Gravel", "Kies"),
        choice("clay", "Clay", "Ton"),
        choice("silt", "Silt", "Schluff"),
        choice("fill", "Fill", "Auffüllung"),
    ];
    const INVESTIGATION_KIND: &[NormFieldChoice] = &[
        choice("borehole", "Borehole", "Bohrung"),
        choice("cpt", "Cone penetration test (CPT)", "Drucksondierung (CPT)"),
        choice("spt", "Standard penetration test (SPT)", "Rammsondierung (SPT)"),
    ];
    const CHARACTERISTIC_PARAMETER: &[NormFieldChoice] = &[
        choice("undrainedStrength", "Undrained shear strength c_u", "Undränierte Scherfestigkeit c_u"),
        choice("frictionAngle", "Friction angle φ′", "Reibungswinkel φ′"),
    ];
    const PARAMETER_BASIS: &[NormFieldChoice] = &[
        choice("declared", "Declared on the layer", "Auf der Schicht angegeben"),
        choice("derived", "Derived from investigations", "Aus Erkundungen abgeleitet"),
    ];

    const TABLE: &[(&str, FieldSpec)] = &[
        ("structureId", FieldSpec::new("Structure ID", "Bauwerks-ID")),
        ("geotechnicalCategory", FieldSpec::new("Geotechnical category", "Geotechnische Kategorie")),
        ("designSituation", FieldSpec::choice("Design situation (DIN 1054)", "Bemessungssituation (DIN 1054)", SITUATION)),
        ("designApproach", FieldSpec::choice("Design approach", "Nachweisverfahren", APPROACH)),
        ("annex", FieldSpec::choice("National annex", "Nationaler Anhang", ANNEX)),
        ("groundwaterLevel", FieldSpec::quantity("Groundwater level", "Grundwasserstand", UnitId::Metre)),
        ("investigationDepth", FieldSpec::quantity("Investigation depth", "Aufschlusstiefe", UnitId::Metre)),
        ("layers", FieldSpec::new("Soil layers", "Bodenschichten")),
        ("layers[].id", FieldSpec::new("Layer ID", "Schicht-ID")),
        ("layers[].soilType", FieldSpec::choice("Soil type", "Bodenart", SOIL)),
        ("layers[].depthTop", FieldSpec::quantity("Depth top", "Tiefe oben", UnitId::Metre)),
        ("layers[].depthBottom", FieldSpec::quantity("Depth bottom", "Tiefe unten", UnitId::Metre)),
        ("layers[].gamma", FieldSpec::quantity("Unit weight γ", "Wichte γ", UnitId::NewtonPerCubicMetre)),
        ("layers[].gammaPrime", FieldSpec::quantity("Submerged unit weight γ′", "Wichte unter Auftrieb γ′", UnitId::NewtonPerCubicMetre)),
        ("layers[].phiPrimeDeg", FieldSpec::quantity("Friction angle φ′", "Reibungswinkel φ′", UnitId::Degree)),
        ("layers[].cohesionEffective", FieldSpec::quantity("Effective cohesion c′", "Effektive Kohäsion c′", UnitId::Pascal)),
        ("layers[].cohesionUndrained", FieldSpec::quantity("Undrained cohesion c_u", "Undränierte Kohäsion c_u", UnitId::Pascal)),
        ("layers[].oedometricModulus", FieldSpec::quantity("Oedometric modulus E_oed", "Steifemodul E_s / E_oed", UnitId::Pascal)),
        ("layers[].poissonRatio", FieldSpec::new("Poisson ratio ν", "Querdehnzahl ν")),
        ("layers[].cptQc", FieldSpec::quantity("CPT tip resistance q_c", "CPT-Spitzenwiderstand q_c", UnitId::Pascal)),
        ("layers[].sptN", FieldSpec::new("SPT blow count N", "SPT-Schlagzahl N")),
        ("layers[].cptNkt", ranged(FieldSpec::new("Cone factor N_kt (0 = not given)", "Kegelfaktor N_kt (0 = nicht angegeben)"), 0.0, 30.0)),
        ("footings", FieldSpec::new("Spread foundations", "Flachgründungen")),
        ("footings[].id", FieldSpec::new("Footing ID", "Fundament-ID")),
        ("footings[].width", FieldSpec::quantity("Width B", "Breite B", UnitId::Metre)),
        ("footings[].length", FieldSpec::quantity("Length L", "Länge L", UnitId::Metre)),
        ("footings[].embedment", FieldSpec::quantity("Embedment D_f", "Einbindetiefe D_f", UnitId::Metre)),
        ("footings[].baseInclinationDeg", FieldSpec::quantity("Base inclination", "Sohlneigung", UnitId::Degree)),
        ("footings[].settlementLimit", FieldSpec::quantity("Settlement limit", "Setzungsgrenze", UnitId::Metre)),
        ("footings[].loadCases", FieldSpec::new("Load cases", "Lastfälle")),
        ("footings[].loadCases[].id", FieldSpec::new("Load case ID", "Lastfall-ID")),
        ("footings[].loadCases[].designSituation", FieldSpec::choice("Load-case design situation", "Lastfall-Bemessungssituation", SITUATION)),
        ("footings[].loadCases[].verticalPermanent", FieldSpec::quantity("Vertical permanent V_G", "Vertikal ständig V_G", UnitId::Newton)),
        ("footings[].loadCases[].verticalVariable", FieldSpec::quantity("Vertical variable V_Q", "Vertikal veränderlich V_Q", UnitId::Newton)),
        ("footings[].loadCases[].horizontalPermanent", FieldSpec::quantity("Horizontal permanent H_G", "Horizontal ständig H_G", UnitId::Newton)),
        ("footings[].loadCases[].horizontalVariable", FieldSpec::quantity("Horizontal variable H_Q", "Horizontal veränderlich H_Q", UnitId::Newton)),
        ("footings[].loadCases[].momentPermanent", FieldSpec::quantity("Moment permanent M_G", "Moment ständig M_G", UnitId::NewtonMetre)),
        ("footings[].loadCases[].momentVariable", FieldSpec::quantity("Moment variable M_Q", "Moment veränderlich M_Q", UnitId::NewtonMetre)),
        ("piles", FieldSpec::new("Piles", "Pfähle")),
        ("piles[].id", FieldSpec::new("Pile ID", "Pfahl-ID")),
        ("piles[].pileType", FieldSpec::choice("Pile type", "Pfahlart", PILE_TYPE)),
        ("piles[].diameter", FieldSpec::quantity("Diameter D", "Durchmesser D", UnitId::Metre)),
        ("piles[].length", FieldSpec::quantity("Length L", "Länge L", UnitId::Metre)),
        ("piles[].count", FieldSpec::new("Pile count", "Pfahlanzahl")),
        ("piles[].alphaS", FieldSpec::new("Shaft factor α_s", "Mantelfaktor α_s")),
        ("piles[].unitShaftResistance", FieldSpec::quantity("Unit shaft resistance q_s", "Mantelwiderstand q_s", UnitId::Pascal)),
        ("piles[].unitBaseResistance", FieldSpec::quantity("Unit base resistance q_b", "Spitzendruck q_b", UnitId::Pascal)),
        ("piles[].compressionPermanent", FieldSpec::quantity("Compression permanent", "Druck ständig", UnitId::Newton)),
        ("piles[].compressionVariable", FieldSpec::quantity("Compression variable", "Druck veränderlich", UnitId::Newton)),
        ("piles[].tensionPermanent", FieldSpec::quantity("Tension permanent", "Zug ständig", UnitId::Newton)),
        ("piles[].tensionVariable", FieldSpec::quantity("Tension variable", "Zug veränderlich", UnitId::Newton)),
        ("piles[].bendingStiffnessEi", FieldSpec::new("Pile bending stiffness EI [N·m²] (0 = not given)", "Pfahl-Biegesteifigkeit EI [N·m²] (0 = nicht angegeben)")),
        ("piles[].horizontalSubgradeModulus", FieldSpec::quantity("Horizontal subgrade modulus k_h (0 = not given)", "Horizontaler Bettungsmodul k_h (0 = nicht angegeben)", UnitId::NewtonPerCubicMetre)),
        ("piles[].lateralHeadLoad", FieldSpec::quantity("Lateral head load H", "Horizontale Kopflast H", UnitId::Newton)),
        ("piles[].elasticModulus", FieldSpec::quantity("Pile material modulus E_c (0 = from the concrete class)", "Pfahl-Elastizitätsmodul E_c (0 = aus der Betonklasse)", UnitId::Pascal)),
        ("piles[].settlingThickness", FieldSpec::quantity("Settling ground thickness h₁ (0 = none)", "Setzende Bodenschicht h₁ (0 = keine)", UnitId::Metre)),
        ("piles[].negativeFrictionBeta", ranged(FieldSpec::new("Negative friction factor β (0 = 0.25)", "Faktor β der negativen Mantelreibung (0 = 0,25)"), 0.0, 2.0)),
        ("piles[].dynamicTestCount", ranged(FieldSpec::new("Dynamic tests n (0 = none)", "Dynamische Pfahltests n (0 = keine)"), 0.0, 1000.0)),
        ("piles[].dynamicResistanceMean", FieldSpec::quantity("Dynamic test mean resistance", "Mittlerer Widerstand der dynamischen Tests", UnitId::Newton)),
        ("piles[].dynamicResistanceMin", FieldSpec::quantity("Dynamic test lowest resistance", "Kleinster Widerstand der dynamischen Tests", UnitId::Newton)),
        ("piles[].hammerWeight", FieldSpec::quantity("Hammer weight W (0 = none)", "Bärgewicht W (0 = keines)", UnitId::Newton)),
        ("piles[].dropHeight", FieldSpec::quantity("Drop height h", "Fallhöhe h", UnitId::Metre)),
        ("piles[].setPerBlow", FieldSpec::quantity("Set per blow s", "Eindringung je Schlag s", UnitId::Metre)),
        ("piles[].elasticCompression", FieldSpec::quantity("Elastic compression s_e", "Elastische Zusammendrückung s_e", UnitId::Metre)),
        ("piles[].hammerEfficiency", ranged(FieldSpec::new("Hammer efficiency η_h", "Bärwirkungsgrad η_h"), 0.0, 1.0)),
        ("piles[].concreteClass", FieldSpec::choice("Concrete class of the shaft", "Betonfestigkeitsklasse des Schafts", PILE_CONCRETE)),
        ("piles[].testProfiles", FieldSpec::new("Pile test profiles", "Pfahl-Prüfprofile")),
        ("piles[].testProfiles[].id", FieldSpec::new("Test profile ID", "Prüfprofil-ID")),
        ("piles[].testProfiles[].shaftResistance", FieldSpec::quantity("Measured shaft resistance", "Gemessener Mantelwiderstand", UnitId::Newton)),
        ("piles[].testProfiles[].baseResistance", FieldSpec::quantity("Measured base resistance", "Gemessener Spitzenwiderstand", UnitId::Newton)),
        ("retainingWalls", FieldSpec::new("Retaining walls", "Stützbauwerke")),
        ("retainingWalls[].id", FieldSpec::new("Wall ID", "Wand-ID")),
        ("retainingWalls[].height", FieldSpec::quantity("Wall height", "Wandhöhe", UnitId::Metre)),
        ("retainingWalls[].embedment", FieldSpec::quantity("Embedment", "Einbindung", UnitId::Metre)),
        ("retainingWalls[].baseWidth", FieldSpec::quantity("Base width", "Sohlbreite", UnitId::Metre)),
        ("retainingWalls[].stemThickness", FieldSpec::quantity("Stem thickness", "Schaftdicke", UnitId::Metre)),
        ("retainingWalls[].backfillPhiDeg", FieldSpec::quantity("Backfill φ′", "Hinterfüllung φ′", UnitId::Degree)),
        ("retainingWalls[].backfillGamma", FieldSpec::quantity("Backfill γ", "Hinterfüllung γ", UnitId::NewtonPerCubicMetre)),
        ("retainingWalls[].wallFrictionDeg", FieldSpec::quantity("Wall friction δ", "Wandreibung δ", UnitId::Degree)),
        ("retainingWalls[].earthPressureMode", FieldSpec::choice("Earth-pressure mode", "Erddruckansatz", EP_MODE)),
        ("retainingWalls[].wallMovement", FieldSpec::choice("Wall movement class", "Wandbewegungsklasse", WALL_MOVE)),
        ("retainingWalls[].concreteGamma", FieldSpec::quantity("Concrete unit weight γ_c", "Beton-Wichte γ_c", UnitId::NewtonPerCubicMetre)),
        ("retainingWalls[].ocr", FieldSpec::new("OCR", "OCR")),
        ("retainingWalls[].surcharge", FieldSpec::quantity("Surcharge", "Auflast", UnitId::Pascal)),
        ("retainingWalls[].verticalPermanent", FieldSpec::quantity("Vertical permanent", "Vertikal ständig", UnitId::Newton)),
        ("retainingWalls[].horizontalPermanent", FieldSpec::quantity("Horizontal permanent", "Horizontal ständig", UnitId::Newton)),
        ("retainingWalls[].seismicKh", ranged(FieldSpec::new("Seismic coefficient k_h (0 = none)", "Seismischer Beiwert k_h (0 = keiner)"), 0.0, 1.0)),
        ("retainingWalls[].seismicKv", ranged(FieldSpec::new("Seismic coefficient k_v", "Seismischer Beiwert k_v"), -1.0, 1.0)),
        ("retainingWalls[].reinforcementSpacing", FieldSpec::quantity("Reinforcement spacing S_v (0 = none)", "Bewehrungsabstand S_v (0 = unbewehrt)", UnitId::Metre)),
        ("retainingWalls[].reinforcementWidth", FieldSpec::quantity("Reinforcement width b", "Bewehrungsbreite b", UnitId::Metre)),
        ("retainingWalls[].reinforcementFriction", ranged(FieldSpec::new("Reinforcement friction F*", "Reibungsbeiwert F* der Bewehrung"), 0.0, 5.0)),
        ("slopes", FieldSpec::new("Slopes", "Böschungen")),
        ("slopes[].id", FieldSpec::new("Slope ID", "Böschungs-ID")),
        ("slopes[].angleDeg", FieldSpec::quantity("Slope angle", "Böschungswinkel", UnitId::Degree)),
        ("slopes[].height", FieldSpec::quantity("Slope height", "Böschungshöhe", UnitId::Metre)),
        ("slopes[].length", FieldSpec::quantity("Slope length", "Böschungslänge", UnitId::Metre)),
        ("slopes[].governingLayerId", FieldSpec::new("Governing layer id", "Maßgebende Schicht-Id")),
        ("slopes[].circleCentreX", FieldSpec::quantity("Slip circle centre x from crest", "Gleitkreismittelpunkt x ab Krone", UnitId::Metre)),
        ("slopes[].circleCentreZ", FieldSpec::quantity("Slip circle centre height above toe", "Gleitkreismittelpunkt Höhe über Fuß", UnitId::Metre)),
        ("slopes[].circleRadius", FieldSpec::quantity("Slip circle radius (0 = search)", "Gleitkreisradius (0 = Suche)", UnitId::Metre)),
        ("slopes[].width", FieldSpec::quantity("Slope width along its axis (0 = plane strain)", "Böschungsbreite in Achsrichtung (0 = ebener Verformungszustand)", UnitId::Metre)),
        ("upliftCases", FieldSpec::new("Uplift / heave cases", "Aufschwimmen / HYD")),
        ("upliftCases[].id", FieldSpec::new("Uplift case ID", "Aufschwimmfall-ID")),
        ("upliftCases[].permanentStabilizing", FieldSpec::quantity("Permanent stabilizing", "Ständig stabilisierend", UnitId::Newton)),
        ("upliftCases[].permanentDestabilizing", FieldSpec::quantity("Permanent destabilizing", "Ständig destabilisierend", UnitId::Newton)),
        ("upliftCases[].variableDestabilizing", FieldSpec::quantity("Variable destabilizing", "Veränderlich destabilisierend", UnitId::Newton)),
        ("upliftCases[].porePressure", FieldSpec::quantity("Pore pressure u", "Porenwasserdruck u", UnitId::Pascal)),
        ("upliftCases[].totalStress", FieldSpec::quantity("Total stress σ", "Totalspannung σ", UnitId::Pascal)),
        ("anchors", FieldSpec::new("Ground anchors", "Verpressanker")),
        ("anchors[].id", FieldSpec::new("Anchor ID", "Anker-ID")),
        ("anchors[].wallId", FieldSpec::new("Supported wall ID", "Gestützte Wand-ID")),
        ("anchors[].tendon", FieldSpec::choice("Tendon type", "Ankerart", TENDON)),
        ("anchors[].headDepth", ranged(FieldSpec::quantity("Anchor head depth", "Ankerkopftiefe", UnitId::Metre), 0.0, 100.0)),
        ("anchors[].inclinationDeg", ranged(FieldSpec::quantity("Anchor inclination", "Ankerneigung", UnitId::Degree), 0.0, 90.0)),
        ("anchors[].freeLength", ranged(FieldSpec::quantity("Free length", "Freie Länge", UnitId::Metre), 0.0, 100.0)),
        ("anchors[].fixedLength", ranged(FieldSpec::quantity("Fixed (bond) length", "Verankerungslänge", UnitId::Metre), 0.0, 100.0)),
        ("anchors[].fixedDiameter", ranged(FieldSpec::quantity("Grout body diameter", "Verpresskörperdurchmesser", UnitId::Metre), 0.0, 2.0)),
        ("anchors[].bondStress", ranged(FieldSpec::quantity("Ultimate skin friction τ_f", "Grenzmantelreibung τ_f", UnitId::Pascal), 0.0, 5.0e6)),
        ("anchors[].designForce", FieldSpec::quantity("Design anchor force A_d", "Bemessungsankerkraft A_d", UnitId::Newton)),
        ("anchors[].tendonYieldForce", FieldSpec::quantity("Characteristic tendon force P_t,k", "Charakteristische Zuggliedkraft P_t,k", UnitId::Newton)),
        ("anchors[].minFreeLength", ranged(FieldSpec::quantity("Minimum free length", "Mindestfreispiellänge", UnitId::Metre), 0.0, 100.0)),
        ("anchors[].testFactor", ranged(FieldSpec::new("Correlation factor ξ_a", "Korrelationsfaktor ξ_a"), 0.5, 2.0)),
        ("anchors[].tests", FieldSpec::new("Suitability tests", "Eignungsprüfungen")),
        ("anchors[].tests[].id", FieldSpec::new("Test ID", "Prüf-ID")),
        ("anchors[].tests[].pullOutResistance", FieldSpec::quantity("Pull-out resistance", "Herausziehwiderstand", UnitId::Newton)),
        ("embeddedWalls", FieldSpec::new("Embedded walls", "Eingespannte Wände")),
        ("embeddedWalls[].id", FieldSpec::new("Embedded wall ID", "Verbauwand-ID")),
        ("embeddedWalls[].kind", FieldSpec::choice("Support condition", "Auflagerbedingung", EMBEDDED_WALL_KIND)),
        ("embeddedWalls[].retainedHeight", ranged(FieldSpec::quantity("Retained height", "Geländesprung", UnitId::Metre), 0.0, 100.0)),
        ("embeddedWalls[].embedment", ranged(FieldSpec::quantity("Provided embedment", "Vorhandene Einbindetiefe", UnitId::Metre), 0.0, 100.0)),
        ("embeddedWalls[].propDepth", ranged(FieldSpec::quantity("Prop depth", "Stützentiefe", UnitId::Metre), 0.0, 100.0)),
        ("embeddedWalls[].thickness", ranged(FieldSpec::quantity("Wall thickness", "Wanddicke", UnitId::Metre), 0.0, 5.0)),
        ("embeddedWalls[].backfillGamma", FieldSpec::quantity("Backfill unit weight γ", "Wichte der Hinterfüllung γ", UnitId::NewtonPerCubicMetre)),
        ("embeddedWalls[].phiDeg", ranged(FieldSpec::quantity("Friction angle φ′", "Reibungswinkel φ′", UnitId::Degree), 0.0, 60.0)),
        ("embeddedWalls[].deltaActiveDeg", ranged(FieldSpec::quantity("Active wall friction δ_a", "Aktive Wandreibung δ_a", UnitId::Degree), -60.0, 60.0)),
        ("embeddedWalls[].deltaPassiveDeg", ranged(FieldSpec::quantity("Passive wall friction δ_p", "Passive Wandreibung δ_p", UnitId::Degree), -60.0, 60.0)),
        ("embeddedWalls[].surcharge", FieldSpec::quantity("Surcharge", "Auflast", UnitId::Pascal)),
        ("embeddedWalls[].undrainedStrength", FieldSpec::quantity("Undrained strength c_u (0 = drained)", "Undränierte Scherfestigkeit c_u (0 = dräniert)", UnitId::Pascal)),
        ("pileGroups", FieldSpec::new("Pile groups", "Pfahlgruppen")),
        ("pileGroups[].id", FieldSpec::new("Pile group ID", "Pfahlgruppen-ID")),
        ("pileGroups[].pileId", FieldSpec::new("Pile ID", "Pfahl-ID")),
        ("pileGroups[].rows", ranged(FieldSpec::new("Rows", "Reihen"), 1.0, 100.0)),
        ("pileGroups[].columns", ranged(FieldSpec::new("Columns", "Spalten"), 1.0, 100.0)),
        ("pileGroups[].spacing", ranged(FieldSpec::quantity("Pile spacing", "Pfahlabstand", UnitId::Metre), 0.0, 20.0)),
        ("pileGroups[].capVertical", FieldSpec::quantity("Vertical cap load", "Vertikallast auf Kopfplatte", UnitId::Newton)),
        ("pileGroups[].capMomentX", FieldSpec::quantity("Cap moment M_x", "Moment der Kopfplatte M_x", UnitId::NewtonMetre)),
        ("pileGroups[].capMomentY", FieldSpec::quantity("Cap moment M_y", "Moment der Kopfplatte M_y", UnitId::NewtonMetre)),
        ("pileGroups[].singleResistance", FieldSpec::quantity("Single pile resistance R_c,d", "Einzelpfahlwiderstand R_c,d", UnitId::Newton)),
        ("pileGroups[].shaftCohesionSum", FieldSpec::quantity("Perimeter cohesion sum Σ c_u·h", "Mantel-Kohäsionssumme Σ c_u·h", UnitId::Pascal)),
        ("pileGroups[].baseCohesion", FieldSpec::quantity("Base cohesion c_u", "Kohäsion am Fuß c_u", UnitId::Pascal)),
        ("labTests", FieldSpec::new("Laboratory tests", "Laborversuche")),
        ("labTests[].id", FieldSpec::new("Test series ID", "Versuchsreihen-ID")),
        ("labTests[].layerId", FieldSpec::new("Tested layer ID", "Geprüfte Schicht-ID")),
        ("labTests[].kind", FieldSpec::choice("Test kind", "Versuchsart", LAB_TEST_KIND)),
        ("labTests[].voidRatio", ranged(FieldSpec::new("Void ratio e", "Porenzahl e"), 0.0, 5.0)),
        ("labTests[].preloadStress", FieldSpec::quantity("Preload stress", "Vorbelastungsspannung", UnitId::Pascal)),
        ("labTests[].drainageHeight", ranged(FieldSpec::quantity("Drainage height", "Dränagehöhe", UnitId::Metre), 0.0, 1.0)),
        ("labTests[].consolidationTime", FieldSpec::quantity("Consolidation time", "Konsolidationszeit", UnitId::Second)),
        ("labTests[].compressionIndex", ranged(FieldSpec::new("Compression index C_c", "Kompressionsbeiwert C_c"), 0.0, 10.0)),
        ("labTests[].swellingIndex", ranged(FieldSpec::new("Swelling index C_s (0 = 0.1 C_c)", "Schwellbeiwert C_s (0 = 0,1 C_c)"), 0.0, 10.0)),
        ("labTests[].stressIncrement", FieldSpec::quantity("Stress increment Δσ", "Spannungszuwachs Δσ", UnitId::Pascal)),
        ("labTests[].secondaryCompressionIndex", ranged(FieldSpec::new("Secondary compression index C_α", "Sekundärkompressionsbeiwert C_α"), 0.0, 1.0)),
        ("labTests[].designLife", FieldSpec::quantity("Design life t₂", "Nutzungsdauer t₂", UnitId::Second)),
        ("labTests[].points", FieldSpec::new("Test points", "Versuchspunkte")),
        ("labTests[].points[].id", FieldSpec::new("Point ID", "Punkt-ID")),
        ("labTests[].points[].normalStress", FieldSpec::quantity("Normal stress σ", "Normalspannung σ", UnitId::Pascal)),
        ("labTests[].points[].shearStress", FieldSpec::quantity("Shear stress τ", "Schubspannung τ", UnitId::Pascal)),
        ("layerWaters", FieldSpec::new("Layer water regimes", "Schichtwasserverhältnisse")),
        ("layerWaters[].id", FieldSpec::new("Water regime ID", "Wasserregime-ID")),
        ("layerWaters[].layerId", FieldSpec::new("Layer ID", "Schicht-ID")),
        ("layerWaters[].piezometricDepth", FieldSpec::quantity("Piezometric depth", "Piezometrische Tiefe", UnitId::Metre)),
        ("layerWaters[].permeability", FieldSpec::quantity("Permeability k", "Durchlässigkeit k", UnitId::MetrePerSecond)),
        ("layerWaters[].headDifference", FieldSpec::quantity("Head difference Δh", "Potenzialdifferenz Δh", UnitId::Metre)),
        ("layerWaters[].flowLength", ranged(FieldSpec::quantity("Flow length", "Fließweglänge", UnitId::Metre), 0.0, 1000.0)),
        ("layerWaters[].wellRadius", FieldSpec::quantity("Well radius r_w (0 = no well)", "Brunnenradius r_w (0 = kein Brunnen)", UnitId::Metre)),
        ("layerWaters[].wellWaterDepth", FieldSpec::quantity("Water depth in the well", "Wassertiefe im Brunnen", UnitId::Metre)),
        ("layerWaters[].influenceRadius", FieldSpec::quantity("Radius of influence R (0 = Sichardt)", "Reichweite R (0 = Sichardt)", UnitId::Metre)),
        ("layerWaters[].wellCapacity", FieldSpec::quantity("Well capacity", "Brunnenleistung", UnitId::CubicMetrePerSecond)),
        ("characteristicBases", FieldSpec::new("Characteristic value bases", "Grundlagen der charakteristischen Werte")),
        ("characteristicBases[].id", FieldSpec::new("Basis ID", "Grundlagen-ID")),
        ("characteristicBases[].layerId", FieldSpec::new("Layer", "Schicht")),
        ("characteristicBases[].parameter", FieldSpec::choice("Parameter", "Parameter", CHARACTERISTIC_PARAMETER)),
        ("characteristicBases[].basis", FieldSpec::choice("Basis", "Grundlage", PARAMETER_BASIS)),
        ("investigations", FieldSpec::new("Ground investigations", "Baugrunderkundungen")),
        ("investigations[].id", FieldSpec::new("Investigation ID", "Erkundungs-ID")),
        ("investigations[].positionX", FieldSpec::quantity("Position x", "Position x", UnitId::Metre)),
        ("investigations[].positionY", FieldSpec::quantity("Position y", "Position y", UnitId::Metre)),
        ("investigations[].groundLevel", FieldSpec::quantity("Ground level", "Geländehöhe", UnitId::Metre)),
        ("investigations[].depth", FieldSpec::quantity("Investigation depth", "Erkundungstiefe", UnitId::Metre)),
        ("investigations[].kind", FieldSpec::choice("Kind (borehole | cpt | spt)", "Art (borehole | cpt | spt)", INVESTIGATION_KIND)),
        ("investigations[].readings", FieldSpec::new("Readings", "Messwerte")),
        ("investigations[].readings[].id", FieldSpec::new("Reading ID", "Messwert-ID")),
        ("investigations[].readings[].depth", FieldSpec::quantity("Reading depth", "Tiefe des Messwerts", UnitId::Metre)),
        ("investigations[].readings[].measurement", FieldSpec::new("Measurement", "Messgröße")),
        ("placements", FieldSpec::new("Placements", "Platzierungen")),
        ("placements[].id", FieldSpec::new("Placed entity id", "Platzierte Element-Id")),
        ("placements[].x", FieldSpec::quantity("Placement x", "Platzierung x", UnitId::Metre)),
        ("placements[].y", FieldSpec::quantity("Placement y", "Platzierung y", UnitId::Metre)),
        ("placements[].z", FieldSpec::quantity("Placement z", "Platzierung z", UnitId::Metre)),
        ("placements[].yaw", FieldSpec::quantity("Placement yaw about Z", "Platzierung Gierwinkel um Z", UnitId::Radian)),
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
    semio_s_plugin_norm_app_surface::case_input_field_meta(crate::catalogue::case_inputs::VOCABULARY, path).or_else(|| lookup_norm_field_meta(TABLE, path))
}
