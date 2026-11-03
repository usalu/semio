//! 🎚️ Gauges: one scalar grip for every numeric field of a row that has no geometric grip of its own, listed per collection in one table. The drag parameter `s` is the displacement in metres along `+Z` of the gauge anchor, so the field is `value + 20 * step * s`, quantised to its schema step (`x-semio-ui.step`) and clamped to its physical range; no limit is a normative value.
//! 🔗️ Handle laws: [`assert_handle_laws`](crate::scene::projection_kit::test_support::assert_handle_laws).

use super::*;
use crate::model::{BridgeSls, MemberEffect, Member, TestSeries};

pub const STEPS_PER_METRE: f64 = 20.0;
pub const PITCH: f64 = 0.4;
const STRESS: f64 = 1.0e10;
const OPEN: f64 = 1.0e12;

/// 🎚️ One numeric schema field as a grip: `tag` names the handle `<id>:gauge-<tag>`, `step` is the schema step, `lo`/`hi` the physical range.
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

pub const MEMBERS: &[Gauge<Member>] = &[
    gauge!("deflection-limit-ratio", "deflectionLimitRatio", |r| r.deflection_limit_ratio, "1", 50.0, 0.0, 1.0e4, "Deflection limit denominator l/n of", "Nenner der Durchbiegungsgrenze l/n von"),
    gauge!("deflection-limit-ratio-w3", "deflectionLimitRatioW3", |r| r.deflection_limit_ratio_w3, "1", 50.0, 0.0, 1.0e4, "Deflection limit denominator l/n for w3 of", "Nenner der Durchbiegungsgrenze l/n für w3 von"),
    gauge!("vibration-frequency", "vibrationFrequency", |r| r.vibration_frequency, "Hz", 0.1, 0.0, 1.0e3, "Natural frequency of", "Eigenfrequenz von"),
    gauge!("vibration-frequency-min", "vibrationFrequencyMin", |r| r.vibration_frequency_min, "Hz", 0.1, 0.0, 1.0e3, "Minimum natural frequency of", "Mindesteigenfrequenz von"),
    gauge!("frame-height", "frameHeight", |r| r.frame_height, "m", 0.01, 0.0, 1.0e3, "Frame height of", "Rahmenhöhe von"),
    gauge!("frame-drift", "frameDrift", |r| r.frame_drift, "m", 0.001, 0.0, 10.0, "Horizontal frame drift of", "Horizontale Rahmenverschiebung von"),
    gauge!("storey-height", "storeyHeight", |r| r.storey_height, "m", 0.01, 0.0, 100.0, "Storey height of", "Geschosshöhe von"),
    gauge!("storey-drift", "storeyDrift", |r| r.storey_drift, "m", 0.001, 0.0, 10.0, "Horizontal storey drift of", "Horizontale Geschossverschiebung von"),
    gauge!("material-strength-k", "materialStrengthK", |r| r.material_strength_k, "Pa", 500_000.0, 0.0, STRESS, "Characteristic material strength of", "Charakteristische Materialfestigkeit von"),
    gauge!("material-conversion-factor", "materialConversionFactor", |r| r.material_conversion_factor, "1", 0.01, 0.0, 10.0, "Conversion factor of", "Umrechnungsfaktor von"),
    gauge!("material-partial-factor", "materialPartialFactor", |r| r.material_partial_factor, "1", 0.01, 0.0, 10.0, "Material partial factor of", "Teilsicherheitsbeiwert des Materials von"),
    gauge!("material-design-strength", "materialDesignStrength", |r| r.material_design_strength, "Pa", 500_000.0, 0.0, STRESS, "Declared design strength of", "Angesetzte Bemessungsfestigkeit von"),
    gauge!("nominal-dimension", "nominalDimension", |r| r.nominal_dimension, "m", 0.001, 0.0, 100.0, "Nominal dimension of", "Nennmaß von"),
    gauge!("dimension-tolerance", "dimensionTolerance", |r| r.dimension_tolerance, "m", 0.0005, 0.0, 10.0, "Dimension tolerance of", "Maßtoleranz von"),
    gauge!("design-dimension", "designDimension", |r| r.design_dimension, "m", 0.001, 0.0, 100.0, "Declared design dimension of", "Angesetztes Bemessungsmaß von"),
    gauge!("member-alpha-cr", "memberAlphaCr", |r| r.member_alpha_cr, "1", 0.1, 0.0, 1.0e3, "Elastic critical load factor of", "Verzweigungslastfaktor von"),
    gauge!("second-order-analysis", "secondOrderAnalysis", |r| r.second_order_analysis, "1", 1.0, 0.0, 1.0, "Second-order analysis flag of", "Kennzeichen Theorie 2. Ordnung von"),
];

pub const BRIDGE_SLS: &[Gauge<BridgeSls>] = &[
    gauge!("deck-acceleration", "deckAcceleration", |r| r.deck_acceleration, "m/s2", 0.05, 0.0, 100.0, "Deck acceleration of", "Überbaubeschleunigung von"),
    gauge!("deck-acceleration-limit", "deckAccelerationLimit", |r| r.deck_acceleration_limit, "m/s2", 0.05, 0.0, 100.0, "Admissible deck acceleration of", "Zulässige Überbaubeschleunigung von"),
    gauge!("deck-twist", "deckTwist", |r| r.deck_twist, "rad", 0.0001, 0.0, 1.0, "Deck twist of", "Verwindung des Überbaus von"),
    gauge!("deck-twist-limit", "deckTwistLimit", |r| r.deck_twist_limit, "rad", 0.0001, 0.0, 1.0, "Admissible deck twist of", "Zulässige Verwindung von"),
    gauge!("track-gauge", "trackGauge", |r| r.track_gauge, "m", 0.001, 0.0, 10.0, "Track gauge of", "Spurweite von"),
    gauge!("line-speed", "lineSpeed", |r| r.line_speed, "km/h", 5.0, 0.0, 1.0e3, "Maximum line speed of", "Höchstgeschwindigkeit der Strecke von"),
    gauge!("ballast-depth", "ballastDepth", |r| r.ballast_depth, "m", 0.01, 0.0, 10.0, "Ballast depth of", "Schotterhöhe von"),
    gauge!("deck-mass", "deckMass", |r| r.deck_mass, "kg/m", 100.0, 0.0, OPEN, "Deck mass per length of", "Deckmasse je Meter von"),
    gauge!("natural-frequency", "naturalFrequency", |r| r.natural_frequency, "Hz", 0.1, 0.0, 1.0e3, "Deck natural frequency of", "Eigenfrequenz des Decks von"),
    gauge!("end-rotation", "endRotation", |r| r.end_rotation, "rad", 0.0001, 0.0, 1.0, "Deck end rotation of", "Endverdrehung des Decks von"),
    gauge!("basic-wind-speed", "basicWindSpeed", |r| r.basic_wind_speed, "m/s", 0.5, 0.0, 200.0, "Basic wind speed at", "Basiswindgeschwindigkeit bei"),
    gauge!("damping-ratio", "dampingRatio", |r| r.damping_ratio, "1", 0.001, 0.0, 1.0, "Deck damping ratio of", "Dämpfungsgrad des Decks von"),
];

pub const TEST_SERIES: &[Gauge<TestSeries>] = &[
    gauge!("sample-size", "sampleSize", |r| f64::from(r.sample_size), "1", 1.0, 0.0, 1.0e6, "Sample size of", "Stichprobenumfang von"),
    gauge!("variation-known", "variationKnown", |r| r.variation_known, "1", 0.01, 0.0, 10.0, "Known coefficient of variation of", "Bekannter Variationskoeffizient von"),
    gauge!("conversion-factor", "conversionFactor", |r| r.conversion_factor, "1", 0.01, 0.0, 10.0, "Conversion factor of", "Umrechnungsfaktor von"),
    gauge!("theory-variation", "theoryVariation", |r| r.theory_variation, "1", 0.01, 0.0, 10.0, "Theory model variation of", "Variationskoeffizient des Rechenmodells von"),
    gauge!("theory-mean", "theoryMean", |r| r.theory_mean, "1", 0.1, 0.0, OPEN, "Theory mean value of", "Rechenwert des Modells von"),
];

pub const EFFECT: Gauge<MemberEffect> = gauge!("influence", "influence", |r| r.influence, "1", 0.001, -1.0e3, 1.0e3, "Influence coefficient of", "Einflussbeiwert von");

/// 🚫 The numeric schema fields no viewport grip writes: a placement stands in the plane, so its height has no grip.
pub const EXEMPT: &[(&str, &str)] = &[("placements", "z")];

/// 🎚️ The collections the gauge tables cover, with the schema field and schema step of each gauge: the contract of the grip coverage test.
pub fn coverage() -> Vec<(&'static str, Vec<(&'static str, f64)>)> {
    fn fields<T>(table: &[Gauge<T>]) -> Vec<(&'static str, f64)> {
        table.iter().map(|g| (g.field, g.step)).collect()
    }
    vec![("members", fields(MEMBERS)), ("bridgeSls", fields(BRIDGE_SLS)), ("testSeries", fields(TEST_SERIES)), ("effects", vec![(EFFECT.field, EFFECT.step)])]
}

fn one<T>(g: &Gauge<T>, row: &T, entity: &EntityRef, tag: &str, path: String, anchor: Vec3, subject: &str) -> Handle {
    let base = (g.get)(row);
    let k = STEPS_PER_METRE * g.step;
    let (a, b) = ((g.lo.min(base) - base) / k, (g.hi.max(base) - base) / k);
    handle(Spec {
        entity,
        tag,
        kind: HandleKind::ScalarAlong { axis: Vec3::Z },
        anchor,
        grip: GripStyle::Diamond,
        writes: vec![binding(path, Affine { base, k }, g.unit, Some(Quantise::Round { step: g.step }))],
        snap: grid(0.05),
        limits: (a.min(b), a.max(b)),
        name: loc(&format!("{} {subject}", g.en), &format!("{} {subject}", g.de)),
    })
}

/// 🎚️ The gauges of one row as handles: a rack of diamonds beside the entity, `at(i)` giving the anchor of the `i`-th at the offset `dy` from the middle of the rack.
pub fn rack<T>(table: &[Gauge<T>], row: &T, collection: &str, id: &str, entity: &EntityRef, at: impl Fn(f64) -> Vec3) -> Vec<Handle> {
    let n = table.len() as f64;
    table.iter().enumerate().map(|(i, g)| one(g, row, entity, &format!("gauge-{}", g.tag), format!("{collection}[id={id}].{}", g.field), at((i as f64 - (n - 1.0) / 2.0) * PITCH), id)).collect()
}

/// 📶️ The influence coefficient of effect `index` as a handle `<action>:gauge-influence-<member>` on the entity of its action.
pub fn influence(row: &MemberEffect, index: usize, entity: &EntityRef, anchor: Vec3) -> Handle {
    one(&EFFECT, row, entity, &influence_tag(&row.member_id), format!("effects[{index}].influence"), anchor, &format!("{} / {}", row.action_id, row.member_id))
}

/// 🆔️ The handle tag of the influence gauge of the effect on `member_id`.
pub fn influence_tag(member_id: &str) -> String {
    format!("gauge-influence-{member_id}")
}
