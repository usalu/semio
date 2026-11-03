//! 📇️ Verification catalogue of EN 1992: the case input, the outcome of one verification and the registry that maps every catalogue id (M-01 … FA-14) to its evaluator. See [`super`].

use super::*;
use std::collections::BTreeMap;

/// ⚠️ Result of one evaluator: its outcomes or the reason why the inputs are insufficient.
pub type R = Result<Vec<Outcome>, String>;

/// 🧾 Inputs of one catalogue verification in N, mm and N/mm²; keys are listed in [`Entry::inputs`].
#[derive(Clone, Debug)]
pub struct Case {
    pub id: String,
    pub annex: AnnexChoice,
    pub num: BTreeMap<String, f64>,
    pub text: BTreeMap<String, String>,
    pub list: BTreeMap<String, Vec<f64>>,
}

impl Case {
    pub fn new(id: &str, annex: AnnexChoice) -> Self {
        Self { id: id.into(), annex, num: BTreeMap::new(), text: BTreeMap::new(), list: BTreeMap::new() }
    }
    pub fn with_n(mut self, key: &str, value: f64) -> Self {
        self.num.insert(key.into(), value);
        self
    }
    pub fn with_s(mut self, key: &str, value: &str) -> Self {
        self.text.insert(key.into(), value.into());
        self
    }
    pub fn with_l(mut self, key: &str, value: &[f64]) -> Self {
        self.list.insert(key.into(), value.to_vec());
        self
    }
    pub fn n(&self, key: &str) -> Result<f64, String> {
        self.num.get(key).copied().ok_or_else(|| format!("missing input {key}"))
    }
    pub fn pos(&self, key: &str) -> Result<f64, String> {
        let v = self.n(key)?;
        if v > 0.0 { Ok(v) } else { Err(format!("input {key} must be positive")) }
    }
    pub fn opt(&self, key: &str) -> Option<f64> {
        self.num.get(key).copied()
    }
    pub fn o(&self, key: &str, default: f64) -> f64 {
        self.opt(key).unwrap_or(default)
    }
    pub fn s(&self, key: &str) -> Result<&str, String> {
        self.text.get(key).map(String::as_str).ok_or_else(|| format!("missing input {key}"))
    }
    pub fn so<'a>(&'a self, key: &str, default: &'a str) -> &'a str {
        self.text.get(key).map(String::as_str).unwrap_or(default)
    }
    pub fn l(&self, key: &str) -> Result<&[f64], String> {
        self.list.get(key).map(Vec::as_slice).ok_or_else(|| format!("missing input {key}"))
    }
    pub fn de(&self) -> bool {
        self.annex == AnnexChoice::De
    }
    pub fn ndp(&self, symbol: &str) -> f64 {
        ndp(self.annex, symbol)
    }
}

/// 🎚 How an outcome is judged.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mode {
    Limit,
    Minimum,
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
    pub steps_de: String,
}

impl Outcome {
    fn make(kind: QuantityKind, mode: Mode, key: &str, en: &str, de: &str, demand: f64, capacity: f64, steps: String) -> Self {
        Self { key: key.into(), en: en.into(), de: de.into(), kind, demand, capacity, mode, steps_de: steps.clone(), steps }
    }
    pub fn steps_de(mut self, steps_de: impl Into<String>) -> Self {
        self.steps_de = steps_de.into();
        self
    }
    pub fn stress(key: &str, en: &str, de: &str, demand_mpa: f64, capacity_mpa: f64, steps: String) -> Self {
        Self::make(QuantityKind::Stress, Mode::Limit, key, en, de, demand_mpa * 1e6, capacity_mpa * 1e6, steps)
    }
    pub fn force(key: &str, en: &str, de: &str, demand_n: f64, capacity_n: f64, steps: String) -> Self {
        Self::make(QuantityKind::Force, Mode::Limit, key, en, de, demand_n, capacity_n, steps)
    }
    pub fn moment(key: &str, en: &str, de: &str, demand_nmm: f64, capacity_nmm: f64, steps: String) -> Self {
        Self::make(QuantityKind::Moment, Mode::Limit, key, en, de, demand_nmm / 1e3, capacity_nmm / 1e3, steps)
    }
    pub fn length(key: &str, en: &str, de: &str, demand_mm: f64, capacity_mm: f64, steps: String) -> Self {
        Self::make(QuantityKind::Length, Mode::Limit, key, en, de, demand_mm / 1e3, capacity_mm / 1e3, steps)
    }
    pub fn area(key: &str, en: &str, de: &str, demand_mm2: f64, capacity_mm2: f64, steps: String) -> Self {
        Self::make(QuantityKind::Area, Mode::Limit, key, en, de, demand_mm2 / 1e6, capacity_mm2 / 1e6, steps)
    }
    pub fn ratio(key: &str, en: &str, de: &str, demand: f64, capacity: f64, steps: String) -> Self {
        Self::make(QuantityKind::Dimensionless, Mode::Limit, key, en, de, demand, capacity, steps)
    }
    pub fn area_min(key: &str, en: &str, de: &str, provided_mm2: f64, required_mm2: f64, steps: String) -> Self {
        Self::make(QuantityKind::Area, Mode::Minimum, key, en, de, provided_mm2 / 1e6, required_mm2 / 1e6, steps)
    }
    pub fn length_min(key: &str, en: &str, de: &str, provided_mm: f64, required_mm: f64, steps: String) -> Self {
        Self::make(QuantityKind::Length, Mode::Minimum, key, en, de, provided_mm / 1e3, required_mm / 1e3, steps)
    }
    pub fn ratio_min(key: &str, en: &str, de: &str, provided: f64, required: f64, steps: String) -> Self {
        Self::make(QuantityKind::Dimensionless, Mode::Minimum, key, en, de, provided, required, steps)
    }
    pub fn info(key: &str, en: &str, de: &str, kind: QuantityKind, value_si: f64, steps: String) -> Self {
        Self::make(kind, Mode::Info, key, en, de, 0.0, value_si, steps)
    }
    pub fn num(key: &str, en: &str, de: &str, value: f64, steps: String) -> Self {
        Self::info(key, en, de, QuantityKind::Dimensionless, value, steps)
    }
    pub fn mpa(key: &str, en: &str, de: &str, value_mpa: f64, steps: String) -> Self {
        Self::info(key, en, de, QuantityKind::Stress, value_mpa * 1e6, steps)
    }
    pub fn mm(key: &str, en: &str, de: &str, value_mm: f64, steps: String) -> Self {
        Self::info(key, en, de, QuantityKind::Length, value_mm / 1e3, steps)
    }
    pub fn mm2(key: &str, en: &str, de: &str, value_mm2: f64, steps: String) -> Self {
        Self::info(key, en, de, QuantityKind::Area, value_mm2 / 1e6, steps)
    }
    pub fn n(key: &str, en: &str, de: &str, value_n: f64, steps: String) -> Self {
        Self::info(key, en, de, QuantityKind::Force, value_n, steps)
    }
    pub fn nmm(key: &str, en: &str, de: &str, value_nmm: f64, steps: String) -> Self {
        Self::info(key, en, de, QuantityKind::Moment, value_nmm / 1e3, steps)
    }
    pub fn utilization(&self) -> f64 {
        match self.mode {
            Mode::Info => 0.0,
            Mode::Limit => if self.capacity.abs() < f64::EPSILON { if self.demand.abs() < f64::EPSILON { 0.0 } else { f64::INFINITY } } else { self.demand / self.capacity },
            Mode::Minimum => if self.demand.abs() < f64::EPSILON { if self.capacity.abs() < f64::EPSILON { 0.0 } else { f64::INFINITY } } else { self.capacity / self.demand },
        }
    }
    pub fn value(&self) -> f64 {
        if self.mode == Mode::Info { self.capacity } else { self.demand }
    }
}

/// 🗂 One registry row: catalogue id, clause, titles, documented input keys, NDP basis symbols and evaluator.
pub struct Entry {
    pub id: &'static str,
    pub part: &'static str,
    pub clause: &'static str,
    pub en: &'static str,
    pub de: &'static str,
    pub inputs: &'static [&'static str],
    pub basis: &'static [&'static str],
    pub eval: fn(&Case) -> R,
}

/// 🏷 Entry constructor for the domain tables.
pub const fn entry_of(id: &'static str, part: &'static str, clause: &'static str, en: &'static str, de: &'static str, inputs: &'static [&'static str], basis: &'static [&'static str], eval: fn(&Case) -> R) -> Entry {
    Entry { id, part, clause, en, de, inputs, basis, eval }
}

/// 🔤 Compact numeric formatting for explanation text.
pub fn f(x: f64) -> String {
    if x.abs() >= 1000.0 || (x != 0.0 && x.abs() < 0.01) { format!("{x:.4e}") } else { format!("{x:.4}") }
}
