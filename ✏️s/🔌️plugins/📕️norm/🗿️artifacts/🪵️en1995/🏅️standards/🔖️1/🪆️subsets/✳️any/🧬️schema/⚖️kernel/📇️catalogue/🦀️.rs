//! 📇️ Verification catalogue of EN 1995: the case input, the outcome of one verification and the registry that maps every catalogue id T-nn to its evaluator. See [`super`].

use super::*;
use std::collections::BTreeMap;

/// ⚠️ Result of one evaluator: its outcomes or the reason why the inputs are insufficient.
pub type R = Result<Vec<Outcome>, String>;

/// 🧾 Inputs of one catalogue verification in standard units (N, mm, N/mm², degrees); keys are the camel-free snake names listed in [`Entry::inputs`].
#[derive(Clone, Debug)]
pub struct Case {
    pub id: String,
    pub annex: AnnexChoice,
    pub num: BTreeMap<String, f64>,
    pub text: BTreeMap<String, String>,
    pub label: Option<(String, String)>,
}

impl Case {
    pub fn new(id: &str, annex: AnnexChoice) -> Self {
        Self { id: id.into(), annex, num: BTreeMap::new(), text: BTreeMap::new(), label: None }
    }
    pub fn with_label(mut self, en: &str, de: &str) -> Self {
        self.label = Some((en.into(), de.into()));
        self
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
    pub fn s(&self, key: &str) -> Result<&str, String> {
        self.text.get(key).map(String::as_str).ok_or_else(|| format!("missing input {key}"))
    }
    pub fn so<'a>(&'a self, key: &str, default: &'a str) -> &'a str {
        self.text.get(key).map(String::as_str).unwrap_or(default)
    }
    pub fn de(&self) -> bool {
        self.annex == AnnexChoice::De
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
    fn make(kind: QuantityKind, mode: Mode, key: &str, en: &str, de: &str, demand: f64, capacity: f64, steps: String) -> Self {
        Self { key: key.into(), en: en.into(), de: de.into(), kind, demand, capacity, mode, steps }
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
    pub fn ratio(key: &str, en: &str, de: &str, demand: f64, capacity: f64, steps: String) -> Self {
        Self::make(QuantityKind::Dimensionless, Mode::Limit, key, en, de, demand, capacity, steps)
    }
    pub fn accel(key: &str, en: &str, de: &str, demand: f64, capacity: f64, steps: String) -> Self {
        Self::make(QuantityKind::Acceleration, Mode::Limit, key, en, de, demand, capacity, steps)
    }
    pub fn info(key: &str, en: &str, de: &str, kind: QuantityKind, value_si: f64, steps: String) -> Self {
        Self::make(kind, Mode::Info, key, en, de, 0.0, value_si, steps)
    }
    pub fn utilization(&self) -> f64 {
        if self.mode == Mode::Info {
            0.0
        } else if self.capacity.abs() < f64::EPSILON {
            if self.demand > 0.0 { f64::INFINITY } else { 0.0 }
        } else {
            self.demand / self.capacity
        }
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

/// 🗄 Every catalogue entry T-01 … T-124 implemented by this artifact.
pub fn registry() -> Vec<&'static Entry> {
    let mut all: Vec<&'static Entry> = Vec::new();
    for group in [cross_sections::ENTRIES, stability::ENTRIES, varying_sections::ENTRIES, serviceability::ENTRIES, connections::ENTRIES, components::ENTRIES, fire::ENTRIES, bridges::ENTRIES] {
        all.extend(group.iter());
    }
    all
}

/// 🔎 Entry by catalogue id such as `T-45`.
pub fn entry(id: &str) -> Option<&'static Entry> {
    registry().into_iter().find(|e| e.id == id)
}

fn subject(case: &Case) -> SubjectRef {
    let (en, de) = case.label.clone().unwrap_or_else(|| (case.id.clone(), case.id.clone()));
    SubjectRef::new(&case.id, format!("cases[id={}]", case.id), loc(&en, &de))
}

/// 🧾 Raw outcomes of one catalogue entry, `Err` with the reason when inputs are missing or the id is unknown.
pub fn evaluate_outcomes(entry_id: &str, case: &Case) -> R {
    (entry(entry_id).ok_or_else(|| format!("unknown catalogue entry {entry_id}"))?.eval)(case)
}

/// 🧮 Evaluate one case against its catalogue entry; missing inputs yield a single not-applicable result.
pub fn evaluate_case(entry_id: &str, case: &Case) -> Vec<CheckResult> {
    let Some(e) = entry(entry_id) else {
        return vec![CheckResult::assess(format!("en1995.{entry_id}.unknown"), "EN 1995", ClauseId::new("EN 1995", "§0", "0"), subject(case), loc("Unknown catalogue entry", "Unbekannter Katalogeintrag"))
            .annex(case.annex)
            .not_applicable(loc(&format!("No catalogue entry {entry_id}."), &format!("Kein Katalogeintrag {entry_id}.")))
            .build()];
    };
    let base = |key: &str, en: &str, de: &str| {
        CheckResult::assess(format!("en1995.{}.{key}.{}", e.id.to_ascii_lowercase(), case.id), e.part, ClauseId::new(e.part, format!("§{}", e.clause), e.clause), subject(case), loc(en, de)).annex(case.annex)
    };
    let (evaluated, read) = semio_s_plugin_norm_compliance::record_ndp_reads(|| (e.eval)(case));
    match evaluated {
        Err(reason) => vec![base("input", e.en, e.de).not_applicable(semio_s_plugin_norm_compliance::reason_copy(e.id, &reason)).build()],
        Ok(outcomes) => outcomes
            .into_iter()
            .map(|o| {
                let title_en = format!("{} – {}", e.en, o.en);
                let title_de = format!("{} – {}", e.de, o.de);
                base(&o.key, &title_en, &title_de)
                    .remedies_attached_by("evaluate_case")
                    .utilization(Quantity::new(o.kind, o.demand), Quantity::new(o.kind, o.capacity))
                    .explanation(loc(&o.steps, &o.steps))
                    .ndps(&read)
                    .build()
            })
            .collect(),
    }
}

/// 🌲 Material data shared by the verifications: tabulated row, k_mod, γ_M and size factor source.
pub struct Mat {
    pub row: StrengthRow,
    pub kmod: f64,
    pub gamma: f64,
    pub accidental: bool,
}

impl Mat {
    pub fn load(c: &Case) -> Result<Self, String> {
        let class = c.s("class")?;
        let row = strength_row(class).ok_or_else(|| format!("unknown strength class {class}"))?;
        let accidental = matches!(c.so("situation", "persistent"), "accidental" | "fire");
        let duration = parse_load_duration(c.so("duration", "medium"));
        let sc = c.o("sc", 1.0) as u8;
        let kmod = match c.opt("kmod") {
            Some(k) => k,
            None if matches!(c.so("situation", "persistent"), "fire") => 1.0,
            None => k_mod_for(c.so("material", product_key(row.product)), sc, duration).ok_or("k_mod not allowed for service class")?,
        };
        let gamma = match c.opt("gamma_m") {
            Some(g) => g,
            None => gamma_m_for(c.annex, c.so("material_gamma", product_key(row.product)), accidental).ok_or("unknown γM material")?,
        };
        Ok(Self { row, kmod, gamma, accidental })
    }
    pub fn fd(&self, characteristic: f64) -> f64 {
        self.kmod * characteristic / self.gamma
    }
    pub fn kh_bending(&self, c: &Case, dim_mm: f64) -> f64 {
        c.opt("kh").unwrap_or_else(|| size_factor_kh(self.row.product, self.row.rho_k, dim_mm, c.o("s_lvl", 0.12)))
    }
}

/// 📏 Size factor k_h of EN 1995-1-1 3.2(3), 3.3, 3.4 for the dimension that governs the strength; solid timber has no size factor when its characteristic density `rho_k` (kg/m³, from the strength-class row) is above the limit of the registry row `ndp.en1995.kh.density-limit` (700 kg/m³, EN 1995-1-1 3.2(3)), every other solid class including the hardwood classes up to D60 has.
pub fn size_factor_kh(product: TimberProduct, rho_k: f64, dim_mm: f64, lvl_exponent: f64) -> f64 {
    let h = dim_mm.max(1.0);
    match product {
        TimberProduct::Solid if rho_k > ndp_common("ndp.en1995.kh.density-limit") => 1.0,
        TimberProduct::Solid => (150.0 / h).powf(0.2).clamp(1.0, 1.3),
        TimberProduct::Glulam => (600.0 / h).powf(0.1).clamp(1.0, 1.1),
        TimberProduct::Lvl => (300.0 / h).powf(lvl_exponent).clamp(1.0, 1.2),
        TimberProduct::Clt => 1.0,
    }
}

/// 📐 Rectangular or user-supplied cross-section values in mm, mm², mm⁴, mm³.
pub struct Sec {
    pub b: f64,
    pub h: f64,
    pub a: f64,
    pub iy: f64,
    pub iz: f64,
    pub wy: f64,
    pub wz: f64,
}

impl Sec {
    pub fn load(c: &Case) -> Result<Self, String> {
        let b = c.o("b", 0.0);
        let h = c.o("h", 0.0);
        let a = c.opt("area").unwrap_or(b * h);
        let iy = c.opt("i_y").unwrap_or(b * h.powi(3) / 12.0);
        let iz = c.opt("i_z").unwrap_or(h * b.powi(3) / 12.0);
        let wy = c.opt("w_y").unwrap_or(b * h * h / 6.0);
        let wz = c.opt("w_z").unwrap_or(h * b * b / 6.0);
        if a <= 0.0 || iy <= 0.0 || wy <= 0.0 {
            return Err("missing input b and h (or area, i_y, w_y)".into());
        }
        Ok(Self { b, h, a, iy, iz, wy, wz })
    }
    pub fn km(&self, c: &Case) -> f64 {
        c.opt("km").unwrap_or(if c.so("shape", "rect") == "rect" { 0.7 } else { 1.0 })
    }
}

/// 🔤 Compact numeric formatting for explanation text.
pub(super) fn f(x: f64) -> String {
    if x.abs() >= 1000.0 || (x != 0.0 && x.abs() < 0.01) { format!("{x:.4e}") } else { format!("{x:.4}") }
}

#[path = "📚️inputs/🦀️.rs"]
pub mod case_inputs;

#[path = "🗂️cases/🦀️.rs"]
mod cases;
pub use cases::*;

#[cfg(test)]
#[path = "🧪️tests/🗣️german-reasons/🦀️.rs"]
mod german_reasons;
