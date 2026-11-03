//! 🎚️ Gauges: one scalar grip for every numeric field of a row that has no geometric grip of its own, listed per collection in one table. The drag parameter `s` is the displacement in metres along `+Z` of the gauge anchor, so the field is `value + 20 * step * s`, quantised to its schema step and held inside its physical range; the table rows are the contract checked against the snapshot schema by the grip coverage test.
//! 🔗️ Handle laws: [`assert_handle_laws`](crate::scene::projection_kit::test_support::assert_handle_laws).

use super::*;
use crate::model::{Anchor, EmbeddedWall, Investigation, LabTest, LayerWater, Pile, PileGroup, RetainingWall, Slope, SoilLayer, SpreadFoundation, UpliftCase};

pub const STEPS_PER_METRE: f64 = 20.0;
const PITCH: f64 = 0.4;
const ANGLE: f64 = 90.0;
const STRESS: f64 = 1.0e9;
const FORCE: f64 = 1.0e10;
const OPEN: f64 = 1.0e12;

pub struct Gauge<T> {
    pub tag: &'static str,
    pub field: &'static str,
    pub get: fn(&T) -> f64,
    pub unit: &'static str,
    pub step: f64,
    pub lo: f64,
    pub hi: f64,
    pub en: &'static str,
    pub de: &'static str,
}

macro_rules! gauge {
    ($tag:literal, $field:literal, $get:expr, $unit:literal, $step:expr, $lo:expr, $hi:expr, $en:literal, $de:literal) => {
        Gauge { tag: $tag, field: $field, get: $get, unit: $unit, step: $step, lo: $lo, hi: $hi, en: $en, de: $de }
    };
}

pub const RETAINING_WALLS: &[Gauge<RetainingWall>] = &[
    gauge!("backfill-phi", "backfillPhiDeg", |r| r.backfill_phi_deg, "deg", 0.5, 0.0, ANGLE, "Backfill friction angle of wall", "Reibungswinkel der Hinterfüllung der Wand"),
    gauge!("backfill-gamma", "backfillGamma", |r| r.backfill_gamma, "N/m3", 100.0, 0.0, 3.0e4, "Backfill unit weight of wall", "Wichte der Hinterfüllung der Wand"),
    gauge!("wall-friction", "wallFrictionDeg", |r| r.wall_friction_deg, "deg", 0.5, 0.0, ANGLE, "Wall friction angle of wall", "Wandreibungswinkel der Wand"),
    gauge!("ocr", "ocr", |r| r.ocr, "-", 0.05, 1.0, 100.0, "Overconsolidation ratio behind wall", "Überkonsolidierungsgrad hinter der Wand"),
    gauge!("concrete-gamma", "concreteGamma", |r| r.concrete_gamma, "N/m3", 100.0, 0.0, 3.0e4, "Concrete unit weight of wall", "Betonwichte der Wand"),
    gauge!("surcharge", "surcharge", |r| r.surcharge, "Pa", 1000.0, 0.0, STRESS, "Surcharge behind wall", "Auflast hinter der Wand"),
    gauge!("vertical-permanent", "verticalPermanent", |r| r.vertical_permanent, "N", 1000.0, 0.0, FORCE, "Permanent vertical load on wall", "Ständige Vertikallast auf der Wand"),
    gauge!("horizontal-permanent", "horizontalPermanent", |r| r.horizontal_permanent, "N", 1000.0, 0.0, FORCE, "Permanent horizontal load on wall", "Ständige Horizontallast auf der Wand"),
    gauge!("seismic-kh", "seismicKh", |r| r.seismic_kh, "-", 0.01, 0.0, 1.0, "Horizontal seismic coefficient of wall", "Horizontaler Erdbebenbeiwert der Wand"),
    gauge!("seismic-kv", "seismicKv", |r| r.seismic_kv, "-", 0.01, 0.0, 1.0, "Vertical seismic coefficient of wall", "Vertikaler Erdbebenbeiwert der Wand"),
    gauge!("reinforcement-width", "reinforcementWidth", |r| r.reinforcement_width, "m", 0.01, 0.0, 10.0, "Reinforcement strip width of wall", "Bewehrungsbandbreite der Wand"),
    gauge!("reinforcement-friction", "reinforcementFriction", |r| r.reinforcement_friction, "-", 0.01, 0.0, 2.0, "Reinforcement soil friction coefficient of wall", "Reibungsbeiwert der Bewehrung der Wand"),
];

pub const EMBEDDED_WALLS: &[Gauge<EmbeddedWall>] = &[
    gauge!("thickness", "thickness", |r| r.thickness, "m", 0.05, 0.05, 5.0, "Thickness of embedded wall", "Dicke der eingespannten Wand"),
    gauge!("backfill-gamma", "backfillGamma", |r| r.backfill_gamma, "N/m3", 100.0, 0.0, 3.0e4, "Unit weight of ground at embedded wall", "Wichte des Bodens an der eingespannten Wand"),
    gauge!("phi", "phiDeg", |r| r.phi_deg, "deg", 0.5, 0.0, ANGLE, "Friction angle of ground at embedded wall", "Reibungswinkel des Bodens an der eingespannten Wand"),
    gauge!("delta-active", "deltaActiveDeg", |r| r.delta_active_deg, "deg", 0.5, 0.0, ANGLE, "Active wall friction angle of embedded wall", "Aktiver Wandreibungswinkel der eingespannten Wand"),
    gauge!("delta-passive", "deltaPassiveDeg", |r| r.delta_passive_deg, "deg", 0.5, 0.0, ANGLE, "Passive wall friction angle of embedded wall", "Passiver Wandreibungswinkel der eingespannten Wand"),
    gauge!("surcharge", "surcharge", |r| r.surcharge, "Pa", 1000.0, 0.0, STRESS, "Surcharge at embedded wall", "Auflast an der eingespannten Wand"),
    gauge!("undrained-strength", "undrainedStrength", |r| r.undrained_strength, "Pa", 1000.0, 0.0, STRESS, "Undrained strength at embedded wall", "Undränierte Scherfestigkeit an der eingespannten Wand"),
];

pub const ANCHORS: &[Gauge<Anchor>] = &[
    gauge!("head-depth", "headDepth", |r| r.head_depth, "m", 0.05, 0.0, 100.0, "Head depth of anchor", "Kopftiefe des Ankers"),
    gauge!("fixed-diameter", "fixedDiameter", |r| r.fixed_diameter, "m", 0.05, 0.01, 2.0, "Fixed-length diameter of anchor", "Verpresskörperdurchmesser des Ankers"),
    gauge!("bond-stress", "bondStress", |r| r.bond_stress, "Pa", 1000.0, 0.0, STRESS, "Bond stress of anchor", "Verbundspannung des Ankers"),
    gauge!("design-force", "designForce", |r| r.design_force, "N", 1000.0, 0.0, FORCE, "Design force of anchor", "Bemessungskraft des Ankers"),
    gauge!("tendon-yield-force", "tendonYieldForce", |r| r.tendon_yield_force, "N", 1000.0, 0.0, FORCE, "Tendon yield force of anchor", "Streckgrenzenkraft des Zugglieds des Ankers"),
    gauge!("min-free-length", "minFreeLength", |r| r.min_free_length, "m", 0.05, 0.0, 100.0, "Minimum free length of anchor", "Mindestfreilänge des Ankers"),
    gauge!("test-factor", "testFactor", |r| r.test_factor, "-", 0.05, 0.0, 10.0, "Test load factor of anchor", "Prüflastfaktor des Ankers"),
];

pub const PILES: &[Gauge<Pile>] = &[
    gauge!("alpha-s", "alphaS", |r| r.alpha_s, "-", 0.01, 0.0, 2.0, "Shaft adhesion factor of pile", "Mantelhaftungsbeiwert des Pfahls"),
    gauge!("unit-shaft-resistance", "unitShaftResistance", |r| r.unit_shaft_resistance, "Pa", 1000.0, 0.0, STRESS, "Unit shaft resistance of pile", "Mantelreibung des Pfahls"),
    gauge!("unit-base-resistance", "unitBaseResistance", |r| r.unit_base_resistance, "Pa", 100000.0, 0.0, STRESS, "Unit base resistance of pile", "Spitzendruck des Pfahls"),
    gauge!("compression-permanent", "compressionPermanent", |r| r.compression_permanent, "N", 1000.0, 0.0, FORCE, "Permanent compression on pile", "Ständige Druckkraft auf den Pfahl"),
    gauge!("compression-variable", "compressionVariable", |r| r.compression_variable, "N", 1000.0, 0.0, FORCE, "Variable compression on pile", "Veränderliche Druckkraft auf den Pfahl"),
    gauge!("tension-permanent", "tensionPermanent", |r| r.tension_permanent, "N", 1000.0, 0.0, FORCE, "Permanent tension on pile", "Ständige Zugkraft auf den Pfahl"),
    gauge!("tension-variable", "tensionVariable", |r| r.tension_variable, "N", 1000.0, 0.0, FORCE, "Variable tension on pile", "Veränderliche Zugkraft auf den Pfahl"),
    gauge!("bending-stiffness", "bendingStiffnessEi", |r| r.bending_stiffness_ei, "N*m2", 1.0e6, 0.0, OPEN, "Bending stiffness of pile", "Biegesteifigkeit des Pfahls"),
    gauge!("subgrade-modulus", "horizontalSubgradeModulus", |r| r.horizontal_subgrade_modulus, "N/m3", 1.0e5, 0.0, OPEN, "Horizontal subgrade modulus at pile", "Horizontaler Bettungsmodul am Pfahl"),
    gauge!("lateral-head-load", "lateralHeadLoad", |r| r.lateral_head_load, "N", 1000.0, 0.0, FORCE, "Lateral head load on pile", "Horizontale Kopflast auf den Pfahl"),
    gauge!("elastic-modulus", "elasticModulus", |r| r.elastic_modulus, "Pa", 1.0e9, 0.0, OPEN, "Elastic modulus of pile", "Elastizitätsmodul des Pfahls"),
    gauge!("negative-friction-beta", "negativeFrictionBeta", |r| r.negative_friction_beta, "-", 0.01, 0.0, 2.0, "Negative skin friction factor of pile", "Beiwert der negativen Mantelreibung des Pfahls"),
    gauge!("dynamic-test-count", "dynamicTestCount", |r| r.dynamic_test_count, "-", 1.0, 0.0, 1000.0, "Number of dynamic tests of pile", "Anzahl der dynamischen Probebelastungen des Pfahls"),
    gauge!("dynamic-resistance-mean", "dynamicResistanceMean", |r| r.dynamic_resistance_mean, "N", 1000.0, 0.0, FORCE, "Mean dynamic resistance of pile", "Mittlerer dynamischer Widerstand des Pfahls"),
    gauge!("dynamic-resistance-min", "dynamicResistanceMin", |r| r.dynamic_resistance_min, "N", 1000.0, 0.0, FORCE, "Lowest dynamic resistance of pile", "Kleinster dynamischer Widerstand des Pfahls"),
    gauge!("hammer-weight", "hammerWeight", |r| r.hammer_weight, "N", 1000.0, 0.0, FORCE, "Hammer weight at pile", "Bärgewicht am Pfahl"),
    gauge!("drop-height", "dropHeight", |r| r.drop_height, "m", 0.01, 0.0, 20.0, "Drop height at pile", "Fallhöhe am Pfahl"),
    gauge!("set-per-blow", "setPerBlow", |r| r.set_per_blow, "m", 0.01, 0.0, 1.0, "Set per blow of pile", "Eindringung je Schlag des Pfahls"),
    gauge!("elastic-compression", "elasticCompression", |r| r.elastic_compression, "m", 0.01, 0.0, 1.0, "Elastic compression of pile", "Elastische Stauchung des Pfahls"),
    gauge!("hammer-efficiency", "hammerEfficiency", |r| r.hammer_efficiency, "-", 0.01, 0.0, 1.0, "Hammer efficiency at pile", "Wirkungsgrad des Bären am Pfahl"),
];

pub const PILE_GROUPS: &[Gauge<PileGroup>] = &[
    gauge!("cap-vertical", "capVertical", |r| r.cap_vertical, "N", 1000.0, 0.0, FORCE, "Vertical load on cap of group", "Vertikallast auf der Kopfplatte der Gruppe"),
    gauge!("cap-moment-x", "capMomentX", |r| r.cap_moment_x, "J", 1000.0, -FORCE, FORCE, "Moment about x on cap of group", "Moment um x auf der Kopfplatte der Gruppe"),
    gauge!("cap-moment-y", "capMomentY", |r| r.cap_moment_y, "J", 1000.0, -FORCE, FORCE, "Moment about y on cap of group", "Moment um y auf der Kopfplatte der Gruppe"),
    gauge!("single-resistance", "singleResistance", |r| r.single_resistance, "N", 1000.0, 0.0, FORCE, "Single-pile resistance of group", "Einzelpfahlwiderstand der Gruppe"),
    gauge!("shaft-cohesion-sum", "shaftCohesionSum", |r| r.shaft_cohesion_sum, "Pa", 1000.0, 0.0, STRESS, "Shaft cohesion sum of group", "Summe der Mantelkohäsion der Gruppe"),
    gauge!("base-cohesion", "baseCohesion", |r| r.base_cohesion, "Pa", 1000.0, 0.0, STRESS, "Base cohesion of group", "Fußkohäsion der Gruppe"),
];

pub const LAB_TESTS: &[Gauge<LabTest>] = &[
    gauge!("void-ratio", "voidRatio", |r| r.void_ratio, "-", 0.05, 0.0, 10.0, "Void ratio of laboratory series", "Porenzahl der Laborversuchsreihe"),
    gauge!("preload-stress", "preloadStress", |r| r.preload_stress, "Pa", 1000.0, 0.0, STRESS, "Preload stress of laboratory series", "Vorbelastung der Laborversuchsreihe"),
    gauge!("drainage-height", "drainageHeight", |r| r.drainage_height, "m", 0.05, 0.0, 10.0, "Drainage height of laboratory series", "Entwässerungshöhe der Laborversuchsreihe"),
    gauge!("consolidation-time", "consolidationTime", |r| r.consolidation_time, "s", 10.0, 0.0, OPEN, "Consolidation time of laboratory series", "Konsolidationszeit der Laborversuchsreihe"),
    gauge!("compression-index", "compressionIndex", |r| r.compression_index, "-", 0.01, 0.0, 5.0, "Compression index of laboratory series", "Kompressionsbeiwert der Laborversuchsreihe"),
    gauge!("swelling-index", "swellingIndex", |r| r.swelling_index, "-", 0.001, 0.0, 5.0, "Swelling index of laboratory series", "Schwellbeiwert der Laborversuchsreihe"),
    gauge!("secondary-compression-index", "secondaryCompressionIndex", |r| r.secondary_compression_index, "-", 0.001, 0.0, 5.0, "Secondary compression index of laboratory series", "Kriechbeiwert der Laborversuchsreihe"),
    gauge!("design-life", "designLife", |r| r.design_life, "s", 31_557_600.0, 0.0, OPEN, "Design life of laboratory series", "Nutzungsdauer der Laborversuchsreihe"),
];

pub const LAYERS: &[Gauge<SoilLayer>] = &[
    gauge!("gamma", "gamma", |r| r.gamma, "N/m3", 500.0, 0.0, 3.0e4, "Unit weight of layer", "Wichte der Schicht"),
    gauge!("gamma-prime", "gammaPrime", |r| r.gamma_prime, "N/m3", 500.0, 0.0, 3.0e4, "Buoyant unit weight of layer", "Wichte unter Auftrieb der Schicht"),
    gauge!("phi-prime", "phiPrimeDeg", |r| r.phi_prime_deg, "deg", 0.5, 0.0, ANGLE, "Effective friction angle of layer", "Effektiver Reibungswinkel der Schicht"),
    gauge!("cohesion-effective", "cohesionEffective", |r| r.cohesion_effective, "Pa", 1000.0, 0.0, STRESS, "Effective cohesion of layer", "Effektive Kohäsion der Schicht"),
    gauge!("cohesion-undrained", "cohesionUndrained", |r| r.cohesion_undrained, "Pa", 1000.0, 0.0, STRESS, "Undrained cohesion of layer", "Undränierte Kohäsion der Schicht"),
    gauge!("oedometric-modulus", "oedometricModulus", |r| r.oedometric_modulus, "Pa", 1.0e6, 0.0, OPEN, "Oedometric modulus of layer", "Steifemodul der Schicht"),
    gauge!("poisson-ratio", "poissonRatio", |r| r.poisson_ratio, "-", 0.01, 0.0, 0.5, "Poisson ratio of layer", "Querdehnzahl der Schicht"),
    gauge!("cpt-qc", "cptQc", |r| r.cpt_qc, "Pa", 500000.0, 0.0, OPEN, "Cone resistance of layer", "Spitzenwiderstand der Schicht"),
    gauge!("spt-n", "sptN", |r| r.spt_n, "-", 1.0, 0.0, 1000.0, "Blow count of layer", "Schlagzahl der Schicht"),
];

pub const LAYER_WATERS: &[Gauge<LayerWater>] = &[
    gauge!("head-difference", "headDifference", |r| r.head_difference, "m", 0.05, 0.0, 1000.0, "Head difference of water regime", "Potenzialdifferenz der Wasserverhältnisse"),
    gauge!("flow-length", "flowLength", |r| r.flow_length, "m", 0.05, 0.0, 1000.0, "Flow length of water regime", "Fließlänge der Wasserverhältnisse"),
    gauge!("well-radius", "wellRadius", |r| r.well_radius, "m", 0.01, 0.0, 10.0, "Well radius of water regime", "Brunnenradius der Wasserverhältnisse"),
    gauge!("influence-radius", "influenceRadius", |r| r.influence_radius, "m", 0.01, 0.0, 10000.0, "Radius of influence of water regime", "Absenkungsreichweite der Wasserverhältnisse"),
    gauge!("well-capacity", "wellCapacity", |r| r.well_capacity, "m3/s", 0.001, 0.0, OPEN, "Well capacity of water regime", "Brunnenleistung der Wasserverhältnisse"),
];

pub const SLOPES: &[Gauge<Slope>] = &[gauge!("length", "length", |r| r.length, "m", 0.05, 0.0, 1000.0, "Crest length of slope", "Kronenlänge der Böschung")];

pub const FOOTINGS: &[Gauge<SpreadFoundation>] = &[
    gauge!("base-inclination", "baseInclinationDeg", |r| r.base_inclination_deg, "deg", 0.5, -ANGLE, ANGLE, "Base inclination of footing", "Sohlneigung des Fundaments"),
    gauge!("settlement-limit", "settlementLimit", |r| r.settlement_limit, "m", 0.001, 0.0, 1.0, "Settlement limit of footing", "Setzungsgrenze des Fundaments"),
];

pub const INVESTIGATIONS: &[Gauge<Investigation>] = &[gauge!("ground-level", "groundLevel", |r| r.ground_level, "m", 0.1, -OPEN, OPEN, "Ground level of ground investigation", "Geländehöhe der Baugrunderkundung")];

pub const UPLIFT_CASES: &[Gauge<UpliftCase>] = &[gauge!("total-stress", "totalStress", |r| r.total_stress, "Pa", 1000.0, 0.0, STRESS, "Total stress of uplift case", "Totale Spannung des Auftriebsfalls")];

/// 🚫 The numeric schema fields no viewport grip writes, with the reason: `permeability` spans orders of magnitude, so a linear drag cannot set it; `cptNkt` has no `change-layer-cpt-nkt` mutation in the edit rules yet (the schema tool has to add it before a grip may write it).
pub const EXEMPT: &[(&str, &str)] = &[("layerWaters", "permeability"), ("layers", "cptNkt")];

/// 🎚️ The collections the gauge tables cover, with the schema fields each table holds: the contract of the grip coverage test.
pub fn coverage() -> Vec<(&'static str, Vec<&'static str>)> {
    fn fields<T>(table: &[Gauge<T>]) -> Vec<&'static str> {
        table.iter().map(|g| g.field).collect()
    }
    vec![
        ("retainingWalls", fields(RETAINING_WALLS)),
        ("embeddedWalls", fields(EMBEDDED_WALLS)),
        ("anchors", fields(ANCHORS)),
        ("piles", fields(PILES)),
        ("pileGroups", fields(PILE_GROUPS)),
        ("labTests", fields(LAB_TESTS)),
        ("layers", fields(LAYERS)),
        ("layerWaters", fields(LAYER_WATERS)),
        ("slopes", fields(SLOPES)),
        ("footings", fields(FOOTINGS)),
        ("investigations", fields(INVESTIGATIONS)),
        ("upliftCases", fields(UPLIFT_CASES)),
    ]
}

/// 🎚️ The gauges of one row as handles: a rack of diamonds beside the entity, `at(i)` giving the anchor of the `i`-th.
pub fn rack<T>(table: &[Gauge<T>], row: &T, collection: &str, id: &str, entity: &EntityRef, at: impl Fn(f64) -> Vec3, out: &mut Vec<Handle>) {
    let n = table.len() as f64;
    for (i, g) in table.iter().enumerate() {
        let base = (g.get)(row);
        let k = STEPS_PER_METRE * g.step;
        out.push(
            Grip {
                id: format!("{id}:gauge-{}", g.tag),
                entity: entity.clone(),
                kind: HandleKind::ScalarAlong { axis: Vec3::Z },
                anchor: at((i as f64 - (n - 1.0) / 2.0) * PITCH),
                style: GripStyle::Diamond,
                writes: vec![write(format!("{collection}[id={id}].{}", g.field), base, k, g.unit, Some(g.step))],
                limits: range(base, k, g.lo.min(base), g.hi.max(base)),
                names: named(g.en, g.de, id),
            }
            .build(),
        );
    }
}
