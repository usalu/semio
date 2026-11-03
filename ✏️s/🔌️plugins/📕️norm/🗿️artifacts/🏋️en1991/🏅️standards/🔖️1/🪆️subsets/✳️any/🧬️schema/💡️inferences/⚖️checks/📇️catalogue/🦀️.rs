//! 📇️ Verification catalogue of EN 1991: the case input, the outcome of one verification and the registry that maps every catalogue id E11.01 … E4.xx to its evaluator.

use super::*;
use std::collections::BTreeMap;

#[path = "🧱️gravity/🦀️.rs"]
mod gravity;
#[path = "🔥️fire/🦀️.rs"]
mod fire;
#[path = "❄️snow/🦀️.rs"]
mod snow;
#[path = "🌬️wind/🦀️.rs"]
mod wind;
#[path = "🌡️actions/🦀️.rs"]
mod actions;
#[path = "🌉️structures/🦀️.rs"]
mod structures;

/// ⚠️ Result of one evaluator: its outcomes or the reason why the inputs are missing or outside the normative domain.
pub type R = Result<Vec<Outcome>, String>;

/// 🧾 Inputs of one catalogue verification in SI units (m, Pa, N, kg, s) with temperatures in °C, angles in degrees and times as named in [`Entry::inputs`].
#[derive(Clone, Debug)]
pub struct Case {
    pub id: String,
    pub annex: AnnexChoice,
    pub num: BTreeMap<String, f64>,
    pub text: BTreeMap<String, String>,
    pub vec: BTreeMap<String, Vec<f64>>,
    pub subject: Option<(String, String, LocalizedCopy)>,
}

impl Case {
    pub fn new(id: &str, annex: AnnexChoice) -> Self { Self { id: id.into(), annex, num: BTreeMap::new(), text: BTreeMap::new(), vec: BTreeMap::new(), subject: None } }
    /// 🔗️ Bind the case to a document entity so its results resolve in the document.
    pub fn bound(mut self, entity: &str, path: &str, label: LocalizedCopy) -> Self { self.subject = Some((entity.into(), path.into(), label)); self }
    pub fn with_n(mut self, key: &str, value: f64) -> Self { self.num.insert(key.into(), value); self }
    pub fn with_s(mut self, key: &str, value: &str) -> Self { self.text.insert(key.into(), value.into()); self }
    pub fn with_v(mut self, key: &str, value: &[f64]) -> Self { self.vec.insert(key.into(), value.to_vec()); self }
    pub fn n(&self, key: &str) -> Result<f64, String> { self.num.get(key).copied().ok_or_else(|| format!("missing input {key}")) }
    pub fn opt(&self, key: &str) -> Option<f64> { self.num.get(key).copied() }
    pub fn o(&self, key: &str, default: f64) -> f64 { self.opt(key).unwrap_or(default) }
    pub fn s(&self, key: &str) -> Result<&str, String> { self.text.get(key).map(String::as_str).ok_or_else(|| format!("missing input {key}")) }
    pub fn v(&self, key: &str) -> Result<&[f64], String> { self.vec.get(key).map(Vec::as_slice).ok_or_else(|| format!("missing input {key}")) }
    pub fn de(&self) -> bool { self.annex == AnnexChoice::De }
    /// 📏 Positive input, rejected when non-positive or not finite.
    pub fn pos(&self, key: &str) -> Result<f64, String> { let v = self.n(key)?; if v.is_finite() && v > 0.0 { Ok(v) } else { Err(format!("input {key} must be positive")) } }
}

/// 🎚️ Whether an outcome is information, a hard limit or a warning-only limit.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mode { Info, Limit, Warn }

/// 🧾 One computed quantity of a verification, with its capacity when the clause defines a limit.
#[derive(Clone, Debug)]
pub struct Outcome {
    pub key: String,
    pub en: String,
    pub de: String,
    pub kind: QuantityKind,
    pub value: f64,
    pub capacity: f64,
    pub mode: Mode,
    pub steps: String,
    pub steps_de: String,
    pub basis: Vec<&'static str>,
}

impl Outcome {
    fn make(key: &str, en: &str, de: &str, kind: QuantityKind, value: f64, capacity: f64, mode: Mode) -> Self { Self { key: key.into(), en: en.into(), de: de.into(), kind, value, capacity, mode, steps: String::new(), steps_de: String::new(), basis: Vec::new() } }
    /// ℹ️ Informative result without a limit.
    pub fn info(key: &str, en: &str, de: &str, kind: QuantityKind, value: f64) -> Self { Self::make(key, en, de, kind, value, 0.0, Mode::Info) }
    /// 🧱 Result that must not exceed its capacity.
    pub fn limit(key: &str, en: &str, de: &str, kind: QuantityKind, value: f64, capacity: f64) -> Self { Self::make(key, en, de, kind, value, capacity, Mode::Limit) }
    /// ⚠️ Result whose exceedance of the capacity is only a warning (validity range).
    pub fn warn(key: &str, en: &str, de: &str, kind: QuantityKind, value: f64, capacity: f64) -> Self { Self::make(key, en, de, kind, value, capacity, Mode::Warn) }
    /// 📝 Attach the calculation steps.
    pub fn steps(mut self, text: impl Into<String>) -> Self { self.steps = text.into(); self.steps_de = self.steps.clone(); self }
    /// 📝 German calculation steps; call after [`Self::steps`], which sets both languages for language-neutral formulas.
    pub fn steps_de(mut self, text: impl Into<String>) -> Self { self.steps_de = text.into(); self }
    /// 🔖 Attach the ids of the unverified normative values used.
    pub fn basis(mut self, ids: &[&'static str]) -> Self { self.basis = ids.to_vec(); self }
    /// 📊 Utilisation value/capacity, 0 for information.
    pub fn utilization(&self) -> f64 {
        if self.mode == Mode::Info { 0.0 } else if self.capacity.abs() < f64::EPSILON { if self.value.abs() < f64::EPSILON { 0.0 } else { 1.0e6 } } else { self.value / self.capacity }
    }
}

/// 🗂️ One registry row: catalogue id, standard, clause, titles, documented input keys and evaluator.
pub struct Entry {
    pub id: &'static str,
    pub standard: &'static str,
    pub clause: &'static str,
    pub en: &'static str,
    pub de: &'static str,
    pub inputs: &'static [&'static str],
    pub eval: fn(&Case) -> R,
}

/// 🗄️ Every catalogue entry implemented by this artifact.
pub fn registry() -> Vec<&'static Entry> {
    let mut all: Vec<&'static Entry> = Vec::new();
    for group in [gravity::ENTRIES, fire::ENTRIES, snow::ENTRIES, wind::ENTRIES, actions::ENTRIES, structures::ENTRIES] { all.extend(group.iter()); }
    all
}

/// 🔎 Entry by catalogue id such as `E14.05`.
pub fn entry(id: &str) -> Option<&'static Entry> { registry().into_iter().find(|e| e.id == id) }

/// ℹ️ Lower bound that lets an informative value pass with zero utilisation: 0 for non-negative values, twice the value for negative ones.
pub fn info_floor(value: f64) -> f64 { if value >= 0.0 { 0.0 } else { 2.0 * value } }

fn subject(case: &Case, leaf: &str) -> SubjectRef {
    match &case.subject {
        Some((entity, path, label)) => SubjectRef::new(entity, path, label.clone()),
        None => SubjectRef::new(&case.id, format!("catalogue[id={}].{leaf}", case.id), copy(&case.id, &case.id)),
    }
}

/// 🧾 Raw outcomes of one catalogue entry; `Err` carries the reason when inputs are missing, outside the domain or the id is unknown.
pub fn evaluate_outcomes(entry_id: &str, case: &Case) -> R { (entry(entry_id).ok_or_else(|| format!("unknown catalogue entry {entry_id}"))?.eval)(case) }

/// 🧮 Evaluate one case against its catalogue entry; missing inputs yield a single not-applicable result.
pub fn evaluate_case(entry_id: &str, case: &Case) -> Vec<CheckResult> {
    let Some(e) = entry(entry_id) else {
        return vec![CheckResult::assess(format!("en1991.{entry_id}.unknown"), "EN 1991", ClauseId::new("EN 1991", "§0", "0"), subject(case, "id"), copy("Unknown catalogue entry", "Unbekannter Katalogeintrag")).annex(case.annex).not_applicable(copy(&format!("No catalogue entry {entry_id}."), &format!("Kein Katalogeintrag {entry_id}."))).build()];
    };
    let id_base = e.id.to_ascii_lowercase();
    let base = |key: &str, en: &str, de: &str| CheckResult::assess(format!("en1991.{id_base}.{key}.{}", case.id), e.standard, ClauseId::new(e.standard, &format!("§{}", e.clause), e.clause), subject(case, "inputs"), copy(en, de)).annex(case.annex);
    let (evaluated, read) = semio_s_plugin_norm_compliance::record_ndp_reads(|| (e.eval)(case));
    match evaluated {
        Err(reason) => vec![base("input", e.en, e.de).not_applicable(semio_s_plugin_norm_compliance::reason_copy(e.id, &reason)).build()],
        Ok(outcomes) => outcomes
            .into_iter()
            .map(|o| {
                let basis = if o.basis.is_empty() { String::new() } else { format!(" Unverified NDP basis: {}.", o.basis.join(", ")) };
                let basis_de = if o.basis.is_empty() { String::new() } else { format!(" Ungeprüfte NDP-Grundlage: {}.", o.basis.join(", ")) };
                let b = o.basis.iter().copied().chain(read.iter().copied()).fold(base(&o.key, &format!("{} – {}", e.en, o.en), &format!("{} – {}", e.de, o.de)).explanation(copy(&format!("{}{basis}", o.steps), &format!("{}{basis_de}", o.steps_de))), |b, id| b.ndp(id));
                match o.mode {
                    Mode::Info => b.minimum(Quantity::new(o.kind, o.value), Quantity::new(o.kind, info_floor(o.value))).build(),
                    Mode::Warn | Mode::Limit => {
                        let b = b.utilization(Quantity::new(o.kind, o.value), Quantity::new(o.kind, o.capacity));
                        let b = if o.mode == Mode::Warn { b.status(CheckStatus::Pass).warn_above(1.0) } else { b };
                        if o.utilization() > 1.0 { b.remedy(Remedy::one_of(subject(case, "inputs"), e.inputs.iter().map(|s| s.to_string()).collect(), copy("Revise the listed inputs.", "Die genannten Eingaben überarbeiten."))).build() } else { b.build() }
                    }
                }
            })
            .collect(),
    }
}

pub use semio_s_artifact_norm_en1991_checks_case_inputs as case_inputs;

#[path = "🗂️cases/🦀️.rs"]
mod cases;
pub use cases::*;

#[cfg(test)]
#[path = "🧪️tests/🗣️german-reasons/🦀️.rs"]
mod german_reasons;
