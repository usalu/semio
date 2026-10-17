//! ⚖️ EN 1997 compliance checks: the clause-by-clause verification behind the compliance report.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

use semio_s_plugin_norm_compliance::AnnexChoice;
use semio_s_artifact_norm_en1997_model::{En1997Snapshot, FoundationLoadCase, SoilLayer, SpreadFoundation};

//#region 🔖️ComplianceHelpers
use semio_s_plugin_norm_compliance::{CheckReport, CheckResult, CheckStatus, ClauseId, LocalizedCopy, Quantity, QuantityKind, Remedy, SubjectRef};

#[path = "🧭️design-approach-and-parameters/🦀️.rs"]
mod design_approach_and_parameters;
pub use design_approach_and_parameters::*;

#[path = "📐️geotechnical-parts/🦀️.rs"]
mod geotechnical_parts;
pub use geotechnical_parts::*;

#[path = "🪨️layer-and-duplicate-id-helpers/🦀️.rs"]
mod layer_and_duplicate_id_helpers;
use layer_and_duplicate_id_helpers::*;

#[path = "🏗️project-checks/🦀️.rs"]
mod project_checks;
pub use project_checks::*;

#[path = "🧱️pile-wall-slope-and-uplift-checks/🦀️.rs"]
mod pile_wall_slope_and_uplift_checks;
use pile_wall_slope_and_uplift_checks::*;

//#region 🏷️FieldMetadata
/// 🏷️ EN 1997 NormFieldMeta lookup — SI units + en/de labels (index wildcards via `[]`).
pub fn lookup_field_meta(path: &str) -> Option<semio_s_plugin_norm_app_surface::NormFieldMeta> {
    use semio_s_plugin_norm_app_surface::{lookup_norm_field_meta, NormFieldChoice, NormFieldMeta};

    const fn choice(value: &'static str, label_en: &'static str, label_de: &'static str) -> NormFieldChoice {
        NormFieldChoice { value, label_en, label_de }
    }

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
        choice("da2", "DA2 / DA2* (GEO-2)", "DA2 / DA2* (GEO-2)"),
        choice("geo2", "GEO-2", "GEO-2"),
        choice("da3", "DA3", "DA3"),
        choice("geo3", "GEO-3", "GEO-3"),
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

    const TABLE: &[(&str, NormFieldMeta)] = &[
        ("structureId", NormFieldMeta { label_en: "Structure ID", label_de: "Bauwerks-ID", unit: None, choices: None }),
        ("geotechnicalCategory", NormFieldMeta { label_en: "Geotechnical category", label_de: "Geotechnische Kategorie", unit: None, choices: None }),
        ("designSituation", NormFieldMeta { label_en: "Design situation (DIN 1054)", label_de: "Bemessungssituation (DIN 1054)", unit: None, choices: Some(SITUATION) }),
        ("designApproach", NormFieldMeta { label_en: "Design approach", label_de: "Nachweisverfahren", unit: None, choices: Some(APPROACH) }),
        ("annex", NormFieldMeta { label_en: "National annex", label_de: "Nationaler Anhang", unit: None, choices: Some(ANNEX) }),
        ("groundwaterLevel", NormFieldMeta { label_en: "Groundwater level", label_de: "Grundwasserstand", unit: Some("m"), choices: None }),
        ("investigationDepth", NormFieldMeta { label_en: "Investigation depth", label_de: "Aufschlusstiefe", unit: Some("m"), choices: None }),
        ("layers", NormFieldMeta { label_en: "Soil layers", label_de: "Bodenschichten", unit: None, choices: None }),
        ("layers[].id", NormFieldMeta { label_en: "Layer ID", label_de: "Schicht-ID", unit: None, choices: None }),
        ("layers[].soilType", NormFieldMeta { label_en: "Soil type", label_de: "Bodenart", unit: None, choices: Some(SOIL) }),
        ("layers[].depthTop", NormFieldMeta { label_en: "Depth top", label_de: "Tiefe oben", unit: Some("m"), choices: None }),
        ("layers[].depthBottom", NormFieldMeta { label_en: "Depth bottom", label_de: "Tiefe unten", unit: Some("m"), choices: None }),
        ("layers[].gamma", NormFieldMeta { label_en: "Unit weight γ", label_de: "Wichte γ", unit: Some("N/m³"), choices: None }),
        ("layers[].gammaPrime", NormFieldMeta { label_en: "Submerged unit weight γ′", label_de: "Wichte unter Auftrieb γ′", unit: Some("N/m³"), choices: None }),
        ("layers[].phiPrimeDeg", NormFieldMeta { label_en: "Friction angle φ′", label_de: "Reibungswinkel φ′", unit: Some("°"), choices: None }),
        ("layers[].cohesionEffective", NormFieldMeta { label_en: "Effective cohesion c′", label_de: "Effektive Kohäsion c′", unit: Some("Pa"), choices: None }),
        ("layers[].cohesionUndrained", NormFieldMeta { label_en: "Undrained cohesion c_u", label_de: "Undränierte Kohäsion c_u", unit: Some("Pa"), choices: None }),
        ("layers[].oedometricModulus", NormFieldMeta { label_en: "Oedometric modulus E_oed", label_de: "Steifemodul E_s / E_oed", unit: Some("Pa"), choices: None }),
        ("layers[].poissonRatio", NormFieldMeta { label_en: "Poisson ratio ν", label_de: "Querdehnzahl ν", unit: None, choices: None }),
        ("layers[].cptQc", NormFieldMeta { label_en: "CPT tip resistance q_c", label_de: "CPT-Spitzenwiderstand q_c", unit: Some("Pa"), choices: None }),
        ("layers[].sptN", NormFieldMeta { label_en: "SPT blow count N", label_de: "SPT-Schlagzahl N", unit: None, choices: None }),
        ("footings", NormFieldMeta { label_en: "Spread foundations", label_de: "Flachgründungen", unit: None, choices: None }),
        ("footings[].id", NormFieldMeta { label_en: "Footing ID", label_de: "Fundament-ID", unit: None, choices: None }),
        ("footings[].width", NormFieldMeta { label_en: "Width B", label_de: "Breite B", unit: Some("m"), choices: None }),
        ("footings[].length", NormFieldMeta { label_en: "Length L", label_de: "Länge L", unit: Some("m"), choices: None }),
        ("footings[].embedment", NormFieldMeta { label_en: "Embedment D_f", label_de: "Einbindetiefe D_f", unit: Some("m"), choices: None }),
        ("footings[].baseInclinationDeg", NormFieldMeta { label_en: "Base inclination", label_de: "Sohlneigung", unit: Some("°"), choices: None }),
        ("footings[].settlementLimit", NormFieldMeta { label_en: "Settlement limit", label_de: "Setzungsgrenze", unit: Some("m"), choices: None }),
        ("footings[].loadCases", NormFieldMeta { label_en: "Load cases", label_de: "Lastfälle", unit: None, choices: None }),
        ("footings[].loadCases[].id", NormFieldMeta { label_en: "Load case ID", label_de: "Lastfall-ID", unit: None, choices: None }),
        ("footings[].loadCases[].designSituation", NormFieldMeta { label_en: "Load-case design situation", label_de: "Lastfall-Bemessungssituation", unit: None, choices: Some(SITUATION) }),
        ("footings[].loadCases[].verticalPermanent", NormFieldMeta { label_en: "Vertical permanent V_G", label_de: "Vertikal ständig V_G", unit: Some("N"), choices: None }),
        ("footings[].loadCases[].verticalVariable", NormFieldMeta { label_en: "Vertical variable V_Q", label_de: "Vertikal veränderlich V_Q", unit: Some("N"), choices: None }),
        ("footings[].loadCases[].horizontalPermanent", NormFieldMeta { label_en: "Horizontal permanent H_G", label_de: "Horizontal ständig H_G", unit: Some("N"), choices: None }),
        ("footings[].loadCases[].horizontalVariable", NormFieldMeta { label_en: "Horizontal variable H_Q", label_de: "Horizontal veränderlich H_Q", unit: Some("N"), choices: None }),
        ("footings[].loadCases[].momentPermanent", NormFieldMeta { label_en: "Moment permanent M_G", label_de: "Moment ständig M_G", unit: Some("N·m"), choices: None }),
        ("footings[].loadCases[].momentVariable", NormFieldMeta { label_en: "Moment variable M_Q", label_de: "Moment veränderlich M_Q", unit: Some("N·m"), choices: None }),
        ("piles", NormFieldMeta { label_en: "Piles", label_de: "Pfähle", unit: None, choices: None }),
        ("piles[].id", NormFieldMeta { label_en: "Pile ID", label_de: "Pfahl-ID", unit: None, choices: None }),
        ("piles[].pileType", NormFieldMeta { label_en: "Pile type", label_de: "Pfahlart", unit: None, choices: Some(PILE_TYPE) }),
        ("piles[].diameter", NormFieldMeta { label_en: "Diameter D", label_de: "Durchmesser D", unit: Some("m"), choices: None }),
        ("piles[].length", NormFieldMeta { label_en: "Length L", label_de: "Länge L", unit: Some("m"), choices: None }),
        ("piles[].count", NormFieldMeta { label_en: "Pile count", label_de: "Pfahlanzahl", unit: None, choices: None }),
        ("piles[].alphaS", NormFieldMeta { label_en: "Shaft factor α_s", label_de: "Mantelfaktor α_s", unit: None, choices: None }),
        ("piles[].unitShaftResistance", NormFieldMeta { label_en: "Unit shaft resistance q_s", label_de: "Mantelwiderstand q_s", unit: Some("Pa"), choices: None }),
        ("piles[].unitBaseResistance", NormFieldMeta { label_en: "Unit base resistance q_b", label_de: "Spitzendruck q_b", unit: Some("Pa"), choices: None }),
        ("piles[].compressionPermanent", NormFieldMeta { label_en: "Compression permanent", label_de: "Druck ständig", unit: Some("N"), choices: None }),
        ("piles[].compressionVariable", NormFieldMeta { label_en: "Compression variable", label_de: "Druck veränderlich", unit: Some("N"), choices: None }),
        ("piles[].tensionPermanent", NormFieldMeta { label_en: "Tension permanent", label_de: "Zug ständig", unit: Some("N"), choices: None }),
        ("piles[].tensionVariable", NormFieldMeta { label_en: "Tension variable", label_de: "Zug veränderlich", unit: Some("N"), choices: None }),
        ("piles[].testProfiles", NormFieldMeta { label_en: "Pile test profiles", label_de: "Pfahl-Prüfprofile", unit: None, choices: None }),
        ("piles[].testProfiles[].id", NormFieldMeta { label_en: "Test profile ID", label_de: "Prüfprofil-ID", unit: None, choices: None }),
        ("piles[].testProfiles[].shaftResistance", NormFieldMeta { label_en: "Measured shaft resistance", label_de: "Gemessener Mantelwiderstand", unit: Some("N"), choices: None }),
        ("piles[].testProfiles[].baseResistance", NormFieldMeta { label_en: "Measured base resistance", label_de: "Gemessener Spitzenwiderstand", unit: Some("N"), choices: None }),
        ("retainingWalls", NormFieldMeta { label_en: "Retaining walls", label_de: "Stützbauwerke", unit: None, choices: None }),
        ("retainingWalls[].id", NormFieldMeta { label_en: "Wall ID", label_de: "Wand-ID", unit: None, choices: None }),
        ("retainingWalls[].height", NormFieldMeta { label_en: "Wall height", label_de: "Wandhöhe", unit: Some("m"), choices: None }),
        ("retainingWalls[].embedment", NormFieldMeta { label_en: "Embedment", label_de: "Einbindung", unit: Some("m"), choices: None }),
        ("retainingWalls[].baseWidth", NormFieldMeta { label_en: "Base width", label_de: "Sohlbreite", unit: Some("m"), choices: None }),
        ("retainingWalls[].stemThickness", NormFieldMeta { label_en: "Stem thickness", label_de: "Schaftdicke", unit: Some("m"), choices: None }),
        ("retainingWalls[].backfillPhiDeg", NormFieldMeta { label_en: "Backfill φ′", label_de: "Hinterfüllung φ′", unit: Some("°"), choices: None }),
        ("retainingWalls[].backfillGamma", NormFieldMeta { label_en: "Backfill γ", label_de: "Hinterfüllung γ", unit: Some("N/m³"), choices: None }),
        ("retainingWalls[].wallFrictionDeg", NormFieldMeta { label_en: "Wall friction δ", label_de: "Wandreibung δ", unit: Some("°"), choices: None }),
        ("retainingWalls[].earthPressureMode", NormFieldMeta { label_en: "Earth-pressure mode", label_de: "Erddruckansatz", unit: None, choices: Some(EP_MODE) }),
        ("retainingWalls[].wallMovement", NormFieldMeta { label_en: "Wall movement class", label_de: "Wandbewegungsklasse", unit: None, choices: Some(WALL_MOVE) }),
        ("retainingWalls[].concreteGamma", NormFieldMeta { label_en: "Concrete unit weight γ_c", label_de: "Beton-Wichte γ_c", unit: Some("N/m³"), choices: None }),
        ("retainingWalls[].ocr", NormFieldMeta { label_en: "OCR", label_de: "OCR", unit: None, choices: None }),
        ("retainingWalls[].surcharge", NormFieldMeta { label_en: "Surcharge", label_de: "Auflast", unit: Some("Pa"), choices: None }),
        ("retainingWalls[].verticalPermanent", NormFieldMeta { label_en: "Vertical permanent", label_de: "Vertikal ständig", unit: Some("N"), choices: None }),
        ("retainingWalls[].horizontalPermanent", NormFieldMeta { label_en: "Horizontal permanent", label_de: "Horizontal ständig", unit: Some("N"), choices: None }),
        ("slopes", NormFieldMeta { label_en: "Slopes", label_de: "Böschungen", unit: None, choices: None }),
        ("slopes[].id", NormFieldMeta { label_en: "Slope ID", label_de: "Böschungs-ID", unit: None, choices: None }),
        ("slopes[].angleDeg", NormFieldMeta { label_en: "Slope angle", label_de: "Böschungswinkel", unit: Some("°"), choices: None }),
        ("slopes[].height", NormFieldMeta { label_en: "Slope height", label_de: "Böschungshöhe", unit: Some("m"), choices: None }),
        ("slopes[].length", NormFieldMeta { label_en: "Slope length", label_de: "Böschungslänge", unit: Some("m"), choices: None }),
        ("slopes[].governingLayerId", NormFieldMeta { label_en: "Governing layer id", label_de: "Maßgebende Schicht-Id", unit: None, choices: None }),
        ("upliftCases", NormFieldMeta { label_en: "Uplift / heave cases", label_de: "Aufschwimmen / HYD", unit: None, choices: None }),
        ("upliftCases[].id", NormFieldMeta { label_en: "Uplift case ID", label_de: "Aufschwimmfall-ID", unit: None, choices: None }),
        ("upliftCases[].permanentStabilizing", NormFieldMeta { label_en: "Permanent stabilizing", label_de: "Ständig stabilisierend", unit: Some("N"), choices: None }),
        ("upliftCases[].permanentDestabilizing", NormFieldMeta { label_en: "Permanent destabilizing", label_de: "Ständig destabilisierend", unit: Some("N"), choices: None }),
        ("upliftCases[].variableDestabilizing", NormFieldMeta { label_en: "Variable destabilizing", label_de: "Veränderlich destabilisierend", unit: Some("N"), choices: None }),
        ("upliftCases[].porePressure", NormFieldMeta { label_en: "Pore pressure u", label_de: "Porenwasserdruck u", unit: Some("Pa"), choices: None }),
        ("upliftCases[].totalStress", NormFieldMeta { label_en: "Total stress σ", label_de: "Totalspannung σ", unit: Some("Pa"), choices: None }),
    ];
    lookup_norm_field_meta(TABLE, path)
}
