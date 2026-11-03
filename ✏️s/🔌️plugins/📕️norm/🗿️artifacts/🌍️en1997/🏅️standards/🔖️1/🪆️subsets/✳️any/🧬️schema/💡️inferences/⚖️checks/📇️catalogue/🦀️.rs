//! 📇️ Verification catalogue of EN 1997: the case input, the outcome of one verification and the registry that maps every catalogue id (PF, G, GEO, EQU, UPL, HYD, SLS, PILE, RW, SLOPE, EMB, GI, ANC, DEW) to its evaluator. See [`super`].

use super::*;
use std::collections::BTreeMap;

#[path = "🧮️factors/🦀️.rs"]
pub mod factors;
#[path = "🧱️earth-pressure/🦀️.rs"]
pub mod earth_pressure;
#[path = "🏛️foundations/🦀️.rs"]
pub mod foundations;
#[path = "📉️settlement/🦀️.rs"]
pub mod settlement;
#[path = "🪵️piles/🦀️.rs"]
pub mod piles;
#[path = "⚓️walls-and-anchors/🦀️.rs"]
pub mod walls_and_anchors;
#[path = "⛰️slopes/🦀️.rs"]
pub mod slopes;
#[path = "🔬️investigation/🦀️.rs"]
pub mod investigation;
#[path = "💧hydraulics/🦀️.rs"]
pub mod hydraulics;
#[path = "🌊️seepage/🦀️.rs"]
pub mod seepage;
#[path = "🧗️wall-sections/🦀️.rs"]
pub mod wall_sections;
#[path = "🛤️embankments/🦀️.rs"]
pub mod embankments;

/// ⚠️ Result of one evaluator: its outcomes or the reason why the inputs are insufficient.
pub type R = Result<Vec<Outcome>, String>;

/// 🧾 Inputs of one catalogue verification in SI units (N, Pa, m, N·m, degrees); keys are the snake names listed in [`Entry::inputs`].
#[derive(Clone, Debug)]
pub struct Case {
    pub id: String,
    pub annex: AnnexChoice,
    pub num: BTreeMap<String, f64>,
    pub text: BTreeMap<String, String>,
}

impl Case {
    pub fn new(id: &str, annex: AnnexChoice) -> Self {
        Self { id: id.into(), annex, num: BTreeMap::new(), text: BTreeMap::new() }
    }
    pub fn with_n(mut self, key: &str, value: f64) -> Self {
        self.num.insert(key.into(), value);
        self
    }
    pub fn with_s(mut self, key: &str, value: &str) -> Self {
        self.text.insert(key.into(), value.into());
        self
    }
    pub fn n(&self, key: &str) -> Result<f64, String> {
        self.num.get(key).copied().ok_or_else(|| format!("missing input {key}"))
    }
    pub fn opt(&self, key: &str) -> Option<f64> {
        self.num.get(key).copied()
    }
    pub fn o(&self, key: &str, default: f64) -> f64 {
        self.opt(key).unwrap_or(default)
    }
    pub fn so<'a>(&'a self, key: &str, default: &'a str) -> &'a str {
        self.text.get(key).map(String::as_str).unwrap_or(default)
    }
    pub fn de(&self) -> bool {
        self.annex == AnnexChoice::De
    }
    pub fn gamma_w(&self) -> f64 {
        self.o("gamma_w", if self.de() { 10_000.0 } else { 9_810.0 })
    }
    pub fn factors(&self) -> AnnexParams {
        let approach = parse_design_approach(self.so("approach", "da2"));
        resolve_params(approach, self.annex, self.so("situation", "bsP"))
    }
    pub fn upl(&self) -> UplParams {
        resolve_upl_params(self.annex, self.so("situation", "bsP"))
    }
    pub fn positive(&self, key: &str) -> Result<f64, String> {
        let value = self.n(key)?;
        if value > 0.0 && value.is_finite() { Ok(value) } else { Err(format!("input {key} must be positive")) }
    }
    pub fn series(&self, prefix: &str) -> Vec<f64> {
        (1..=64).map_while(|i| self.opt(&format!("{prefix}{i}"))).collect()
    }
}

/// 🎚 How an outcome is judged.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mode {
    Limit,
    Info,
}

/// 📐 One quantity comparison or derived value of a verification; values are SI (Pa, N, m, N·m).
#[derive(Clone, Debug)]
pub struct Outcome {
    pub key: String,
    pub en: String,
    pub de: String,
    pub kind: QuantityKind,
    pub demand: f64,
    pub capacity: f64,
    pub mode: Mode,
    pub steps: String,
}

impl Outcome {
    pub fn limit(kind: QuantityKind, key: &str, en: &str, de: &str, demand: f64, capacity: f64) -> Self {
        let steps = format!("{key}: demand {} vs resistance {}", f(demand), f(capacity));
        Self { key: key.into(), en: en.into(), de: de.into(), kind, demand, capacity, mode: Mode::Limit, steps }
    }
    pub fn info(kind: QuantityKind, key: &str, en: &str, de: &str, value: f64) -> Self {
        let steps = format!("{key} = {}", f(value));
        Self { key: key.into(), en: en.into(), de: de.into(), kind, demand: 0.0, capacity: value, mode: Mode::Info, steps }
    }
    pub fn force(key: &str, en: &str, de: &str, demand: f64, capacity: f64) -> Self {
        Self::limit(QuantityKind::Force, key, en, de, demand, capacity)
    }
    pub fn moment(key: &str, en: &str, de: &str, demand: f64, capacity: f64) -> Self {
        Self::limit(QuantityKind::Moment, key, en, de, demand, capacity)
    }
    pub fn pressure(key: &str, en: &str, de: &str, demand: f64, capacity: f64) -> Self {
        Self::limit(QuantityKind::Pressure, key, en, de, demand, capacity)
    }
    pub fn length(key: &str, en: &str, de: &str, demand: f64, capacity: f64) -> Self {
        Self::limit(QuantityKind::Length, key, en, de, demand, capacity)
    }
    pub fn ratio(key: &str, en: &str, de: &str, demand: f64, capacity: f64) -> Self {
        Self::limit(QuantityKind::Dimensionless, key, en, de, demand, capacity)
    }
    pub fn number(key: &str, en: &str, de: &str, value: f64) -> Self {
        Self::info(QuantityKind::Dimensionless, key, en, de, value)
    }
    pub fn newton(key: &str, en: &str, de: &str, value: f64) -> Self {
        Self::info(QuantityKind::Force, key, en, de, value)
    }
    pub fn pascal(key: &str, en: &str, de: &str, value: f64) -> Self {
        Self::info(QuantityKind::Pressure, key, en, de, value)
    }
    pub fn metre(key: &str, en: &str, de: &str, value: f64) -> Self {
        Self::info(QuantityKind::Length, key, en, de, value)
    }
    pub fn newton_metre(key: &str, en: &str, de: &str, value: f64) -> Self {
        Self::info(QuantityKind::Moment, key, en, de, value)
    }
    pub fn utilization(&self) -> f64 {
        if self.mode == Mode::Info {
            0.0
        } else if !self.demand.is_finite() || !self.capacity.is_finite() {
            1.0e6
        } else if self.capacity <= f64::EPSILON {
            if self.demand <= f64::EPSILON { 0.0 } else { 1.0e6 }
        } else {
            (self.demand / self.capacity).min(1.0e6)
        }
    }
    pub fn value(&self) -> f64 {
        if self.mode == Mode::Info { self.capacity } else { self.utilization() }
    }
}

/// 🗂 One registry row: catalogue id, clause, titles, documented input keys and evaluator.
pub struct Entry {
    pub id: &'static str,
    pub part: &'static str,
    pub clause: &'static str,
    pub en: &'static str,
    pub de: &'static str,
    pub inputs: &'static [&'static str],
    pub eval: fn(&Case) -> R,
}

/// 🗄 Every catalogue entry of this artifact.
pub fn registry() -> Vec<&'static Entry> {
    let mut all: Vec<&'static Entry> = Vec::new();
    for group in [factors::ENTRIES, earth_pressure::ENTRIES, foundations::ENTRIES, settlement::ENTRIES, piles::ENTRIES, walls_and_anchors::ENTRIES, slopes::ENTRIES, investigation::ENTRIES, hydraulics::ENTRIES, seepage::ENTRIES, wall_sections::ENTRIES, embankments::ENTRIES] {
        all.extend(group.iter());
    }
    all
}

/// 🔎 Entry by catalogue id such as `PILE-01`.
pub fn entry(id: &str) -> Option<&'static Entry> {
    registry().into_iter().find(|e| e.id == id)
}

fn subject(case: &Case) -> SubjectRef {
    SubjectRef::new(&case.id, format!("catalogue[id={}]", case.id), loc(&case.id, &case.id))
}

/// 🧾 Raw outcomes of one catalogue entry, `Err` with the reason when inputs are missing or the id is unknown.
pub fn evaluate_outcomes(entry_id: &str, case: &Case) -> R {
    (entry(entry_id).ok_or_else(|| format!("unknown catalogue entry {entry_id}"))?.eval)(case)
}

/// 🧮 Evaluate one case against its catalogue entry; missing inputs yield a single not-applicable result.
pub fn evaluate_case(entry_id: &str, case: &Case) -> Vec<CheckResult> {
    match entry(entry_id) {
        None => outcomes_to_results(entry_id, case, Err(String::new())),
        Some(e) => outcomes_to_results(entry_id, case, (e.eval)(case)),
    }
}

/// 🧮 Converts the raw outcomes of one catalogue entry (possibly extended by the caller with derived comparisons) into check results.
pub fn outcomes_to_results(entry_id: &str, case: &Case, outcomes: R) -> Vec<CheckResult> {
    let Some(e) = entry(entry_id) else {
        return vec![CheckResult::assess(format!("en1997.{entry_id}.unknown"), "EN 1997", ClauseId::new("EN 1997", "0", "0"), subject(case), loc("Unknown catalogue entry", "Unbekannter Katalogeintrag"))
            .annex(case.annex)
            .not_applicable(loc(&format!("No catalogue entry {entry_id}."), &format!("Kein Katalogeintrag {entry_id}.")))
            .build()];
    };
    let base = |key: &str, en: &str, de: &str| {
        CheckResult::assess(format!("en1997.{}.{key}.{}", e.id.to_ascii_lowercase(), case.id), e.part, ClauseId::new(e.part, e.clause.split('.').next().unwrap_or(e.clause), e.clause), subject(case), loc(en, de)).annex(case.annex)
    };
    let read = semio_s_plugin_norm_compliance::drain_ndp_reads();
    match outcomes {
        Err(reason) => vec![base("input", e.en, e.de).not_applicable(semio_s_plugin_norm_compliance::reason_copy(e.id, &reason)).build()],
        Ok(outcomes) => outcomes
            .into_iter()
            .map(|o| {
                let title_en = format!("{} – {}", e.en, o.en);
                let title_de = format!("{} – {}", e.de, o.de);
                let mut builder = base(&o.key, &title_en, &title_de).explanation(loc(&o.steps, &o.steps));
                if o.mode == Mode::Limit {
                    builder = builder.utilization(Quantity::new(o.kind, o.demand), Quantity::new(o.kind, o.capacity));
                    if o.utilization() > 1.0 {
                        builder = builder.remedy(Remedy::at_most(
                            subject(case),
                            Quantity::new(o.kind, o.demand),
                            Quantity::new(o.kind, o.capacity),
                            loc("Reduce the action effect or raise the resistance of this verification.", "Einwirkung verringern oder Widerstand dieses Nachweises erhöhen."),
                        ));
                    }
                } else {
                    builder = builder.utilization(Quantity::new(o.kind, 0.0), Quantity::new(o.kind, o.capacity.abs().max(1.0)));
                }
                builder.ndps(&read).build()
            })
            .collect(),
    }
}

pub fn rad(deg: f64) -> f64 {
    deg.to_radians()
}

pub fn f(x: f64) -> String {
    if x.abs() >= 1.0e6 || (x != 0.0 && x.abs() < 0.01) { format!("{x:.4e}") } else { format!("{x:.4}") }
}

pub fn gate(ok: bool, reason: &str) -> Result<(), String> {
    if ok { Ok(()) } else { Err(reason.into()) }
}

pub use semio_s_artifact_norm_en1997_checks_case_inputs as case_inputs;

#[path = "🗂️cases/🦀️.rs"]
mod cases;
pub use cases::*;

#[cfg(test)]
#[path = "🧪️tests/🗣️german-reasons/🦀️.rs"]
mod german_reasons;

#[cfg(test)]
#[path = "🧪️tests/📚️demo-document/🦀️.rs"]
mod demo_document;
