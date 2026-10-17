//! 🧰️ Assessment vocabulary of the EN 1995 timber checks: the governing combination, the grain and lever tables and the solve and finish steps of one assessment. See [`super`].

use super::*;

pub fn governing_uls(combos: &[LoadCombo]) -> Option<&LoadCombo> {
    combos
        .iter()
        .filter(|c| c.kind == ComboKind::Uls)
        .max_by(|a, b| {
            a.m_ed_nm
                .abs()
                .partial_cmp(&b.m_ed_nm.abs())
                .unwrap_or(std::cmp::Ordering::Equal)
        })
}

//#region 🔖️Assessment
/// 🧾 One assessed check before its remedies are attached — carries the utilization the builder keeps private.
pub(super) struct Assessed<T: 'static> {
    pub(super) id: String,
    pub(super) utilization: f64,
    builder: CheckBuilder,
    pub(super) levers: &'static [Lever<T>],
}

/// 📏 Resolution a lever's required value is rounded up to.
#[derive(Clone, Copy)]
enum Grain {
    Millimetre,
    Unit,
    Count,
}

impl Grain {
    fn resolution(self) -> f64 {
        match self {
            Self::Millimetre => 1e-3,
            Self::Unit | Self::Count => 1.0,
        }
    }
    fn round_up(self, x: f64) -> f64 {
        let r = self.resolution();
        (x / r - 1e-9).ceil() * r
    }
    /// 🔢 Whether `applyRemedy` can write the value — it writes floats, which integer fields reject.
    fn applies_as_float(self) -> bool {
        !matches!(self, Self::Count)
    }
}

/// 🎚 One input field a remedy may raise, with its localized label for remedy copy.
struct Lever<T> {
    pub(super) leaf: &'static str,
    pub(super) label_en: &'static str,
    pub(super) label_de: &'static str,
    unit: &'static str,
    grain: Grain,
    pub(super) kind: QuantityKind,
    get: fn(&T) -> f64,
    set: fn(&mut T, f64),
}

impl<T> Lever<T> {
    fn display(&self, value: f64) -> String {
        match self.grain {
            Grain::Millimetre => format!("{:.0} mm", value * 1000.0),
            Grain::Unit => format!("{value:.0} {}", self.unit),
            Grain::Count => format!("{value:.0}"),
        }
    }
    fn quantity(&self, value: f64) -> Quantity {
        Quantity::new(self.kind, value)
    }
}

pub(super) const H: Lever<TimberMember> = Lever { leaf: "hM", label_en: "section depth h", label_de: "Querschnittshöhe h", unit: "m", grain: Grain::Millimetre, kind: QuantityKind::Length, get: |m| m.h_m, set: |m, v| m.h_m = v };

pub(super) const B: Lever<TimberMember> = Lever { leaf: "bM", label_en: "section width b", label_de: "Querschnittsbreite b", unit: "m", grain: Grain::Millimetre, kind: QuantityKind::Length, get: |m| m.b_m, set: |m, v| m.b_m = v };

const BEARING: Lever<TimberMember> = Lever { leaf: "bearingLengthM", label_en: "bearing length l", label_de: "Aufstandslänge l", unit: "m", grain: Grain::Millimetre, kind: QuantityKind::Length, get: |m| m.bearing_length_m, set: |m, v| m.bearing_length_m = v };

const MCRIT: Lever<TimberMember> = Lever { leaf: "mCritNm", label_en: "critical moment M_crit (lateral restraint)", label_de: "Kippmoment M_crit (seitliche Halterung)", unit: "N·m", grain: Grain::Unit, kind: QuantityKind::Moment, get: |m| m.m_crit_nm, set: |m, v| m.m_crit_nm = v };

const MASS: Lever<TimberMember> = Lever { leaf: "massKgPerM", label_en: "linear mass", label_de: "längenbezogene Masse", unit: "kg/m", grain: Grain::Unit, kind: QuantityKind::Mass, get: |m| m.mass_kg_per_m, set: |m, v| m.mass_kg_per_m = v };

const MASS2: Lever<TimberMember> = Lever { leaf: "massKgPerM2", label_en: "floor mass per area", label_de: "flächenbezogene Deckenmasse", unit: "kg/m²", grain: Grain::Unit, kind: QuantityKind::Mass, get: |m| m.mass_kg_per_m2, set: |m, v| m.mass_kg_per_m2 = v };

const DAMPING: Lever<TimberMember> = Lever { leaf: "dampingXi", label_en: "modal damping ratio", label_de: "modale Dämpfung", unit: "", grain: Grain::Unit, kind: QuantityKind::Dimensionless, get: |m| m.damping_xi, set: |m, v| m.damping_xi = v };

const NUMBER: Lever<TimberConnection> = Lever { leaf: "number", label_en: "number of fasteners", label_de: "Anzahl der Verbindungsmittel", unit: "", grain: Grain::Count, kind: QuantityKind::Dimensionless, get: |c| c.number as f64, set: |c, v| c.number = v.round().max(0.0) as u32 };

const DIAMETER: Lever<TimberConnection> = Lever { leaf: "diameterM", label_en: "fastener diameter d", label_de: "Durchmesser d", unit: "m", grain: Grain::Millimetre, kind: QuantityKind::Length, get: |c| c.diameter_m, set: |c, v| c.diameter_m = v };

const SPACING: Lever<TimberConnection> = Lever { leaf: "spacingM", label_en: "spacing a₁", label_de: "Abstand a₁", unit: "m", grain: Grain::Millimetre, kind: QuantityKind::Length, get: |c| c.spacing_m, set: |c, v| c.spacing_m = v };

const END: Lever<TimberConnection> = Lever { leaf: "endDistanceM", label_en: "end distance a₃,t", label_de: "Hirnholzabstand a₃,t", unit: "m", grain: Grain::Millimetre, kind: QuantityKind::Length, get: |c| c.end_distance_m, set: |c, v| c.end_distance_m = v };

const EDGE: Lever<TimberConnection> = Lever { leaf: "edgeDistanceM", label_en: "edge distance a₄,t", label_de: "Randabstand a₄,t", unit: "m", grain: Grain::Millimetre, kind: QuantityKind::Length, get: |c| c.edge_distance_m, set: |c, v| c.edge_distance_m = v };

pub(super) const BENDING_LEVERS: &[Lever<TimberMember>] = &[H, MCRIT];

pub(super) const DEPTH_LEVERS: &[Lever<TimberMember>] = &[H];

pub(super) const SECTION_LEVERS: &[Lever<TimberMember>] = &[H, B];

pub(super) const WIDTH_LEVERS: &[Lever<TimberMember>] = &[B];

pub(super) const BEARING_LEVERS: &[Lever<TimberMember>] = &[BEARING];

pub(super) const MASS_LEVERS: &[Lever<TimberMember>] = &[MASS, MASS2, DAMPING];

pub(super) const JOHANSEN_LEVERS: &[Lever<TimberConnection>] = &[NUMBER, DIAMETER];

pub(super) const SPACING_LEVERS: &[Lever<TimberConnection>] = &[SPACING];

pub(super) const END_LEVERS: &[Lever<TimberConnection>] = &[END];

pub(super) const EDGE_LEVERS: &[Lever<TimberConnection>] = &[EDGE];

/// 🧾 Header shared by every assessed check (identity, clause, subject and annex).
pub(super) struct Head {
    pub(super) id: String,
    pub(super) part: &'static str,
    pub(super) clause: ClauseId,
    pub(super) subject: SubjectRef,
    pub(super) title: LocalizedCopy,
    pub(super) annex: AnnexChoice,
}

pub(super) fn assessed<T>(head: Head, computed: Quantity, limit: Quantity, explanation: LocalizedCopy, levers: &'static [Lever<T>]) -> Assessed<T> {
    let utilization = if limit.value.abs() < f64::EPSILON { 0.0 } else { computed.value / limit.value };
    let builder = CheckResult::assess(head.id.clone(), head.part, head.clause, head.subject, head.title)
        .utilization(computed, limit)
        .annex(head.annex)
        .explanation(explanation);
    Assessed { id: head.id, utilization, builder, levers }
}

/// 🔎 Smallest value of `lever` (rounded up to its grain) at which check `id` passes, re-evaluating through `probe`.
fn solve<T: Clone>(item: &T, id: &str, lever: &Lever<T>, probe: &dyn Fn(&T) -> Vec<(String, f64)>) -> Option<f64> {
    let passes = |x: f64| {
        let mut candidate = item.clone();
        (lever.set)(&mut candidate, x);
        probe(&candidate).into_iter().any(|(cid, u)| cid == id && u <= 1.0)
    };
    let current = (lever.get)(item).max(0.0);
    let mut low = current;
    let mut high = if current > 0.0 { current } else { lever.grain.resolution() };
    let mut feasible = false;
    for _ in 0..64 {
        high *= 1.25;
        if passes(high) {
            feasible = true;
            break;
        }
        low = high;
    }
    if !feasible {
        return None;
    }
    for _ in 0..48 {
        if high - low <= lever.grain.resolution() * 0.25 {
            break;
        }
        let mid = 0.5 * (low + high);
        if passes(mid) {
            high = mid;
        } else {
            low = mid;
        }
    }
    let mut value = lever.grain.round_up(high);
    for _ in 0..64 {
        if value > current && passes(value) {
            return Some(value);
        }
        value = lever.grain.round_up(value + lever.grain.resolution());
    }
    None
}

/// 🩹 Builds every assessed check, attaching one verified remedy per lever that alone flips a failing check.
pub(super) fn finish<T: Clone>(item: &T, checks: Vec<Assessed<T>>, subject: fn(&T, &str) -> SubjectRef, probe: &dyn Fn(&T) -> Vec<(String, f64)>) -> Vec<CheckResult> {
    checks
        .into_iter()
        .map(|check| {
            let mut builder = check.builder;
            if check.utilization > 1.0 {
                let mut solved = 0usize;
                for lever in check.levers {
                    let current = (lever.get)(item);
                    if let Some(required) = solve(item, &check.id, lever, probe) {
                        solved += 1;
                        let mut remedy = Remedy::at_least(
                            subject(item, lever.leaf),
                            lever.quantity(current),
                            lever.quantity(required),
                            loc(
                                &format!("Increase {} from {} to at least {}.", lever.label_en, lever.display(current), lever.display(required)),
                                &format!("{} von {} auf mindestens {} erhöhen.", lever.label_de, lever.display(current), lever.display(required)),
                            ),
                        );
                        remedy.applicable = lever.grain.applies_as_float();
                        builder = builder.remedy(remedy);
                    }
                }
                if solved == 0 {
                    if let Some(lever) = check.levers.first() {
                        let current = (lever.get)(item);
                        let mut remedy = Remedy::at_least(
                            subject(item, lever.leaf),
                            lever.quantity(current),
                            lever.quantity(current),
                            loc(
                                &format!("Raising the {} alone cannot satisfy this check — combine it with other measures.", lever.label_en),
                                &format!("{} allein zu erhöhen genügt nicht — mit weiteren Maßnahmen kombinieren.", lever.label_de),
                            ),
                        );
                        remedy.applicable = false;
                        builder = builder.remedy(remedy);
                    }
                }
            }
            builder.build()
        })
        .collect()
}
//#endregion 🔖️Assessment
