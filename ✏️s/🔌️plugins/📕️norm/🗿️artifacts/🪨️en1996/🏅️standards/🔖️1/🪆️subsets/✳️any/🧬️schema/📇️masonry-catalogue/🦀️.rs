//! 📇️ Verification catalogue of EN 1996: the case input, the outcome of one verification and the registry that maps every catalogue id M-nn to its evaluator. See [`semio_s_artifact_norm_en1996_masonry`].

extern crate semio_s_plugin_norm_mechanics as mechanics;

use semio_s_plugin_norm_compliance::{AnnexChoice, CheckResult, CheckStatus, DesignSituation, LocalizedCopy, Quantity, QuantityKind, Remedy, SubjectRef};
use semio_s_artifact_norm_en1996_model::{BarKind, ChaseKind, ExposureClass, FrostClass, TieKind, WallBar, WallTie, MasonryClass, MasonryWall, MortarClass, MortarType, UnitGroup, UnitMaterial, WallType};
use semio_s_artifact_norm_en1996_masonry_formulas::*;
use semio_s_artifact_norm_en1996_masonry_tables::*;
use std::collections::BTreeMap;

#[path = "🧪️materials/🦀️.rs"]
mod materials;
#[path = "🧱️vertical-load/🦀️.rs"]
mod vertical_load;
#[path = "🪵️shear-and-lateral/🦀️.rs"]
mod shear_and_lateral;
#[path = "🪢️reinforced-special-and-ties/🦀️.rs"]
mod reinforced_special_and_ties;
#[path = "🌦️serviceability-simplified-and-fire/🦀️.rs"]
mod serviceability_simplified_and_fire;
pub use semio_s_artifact_norm_en1996_masonry_tables::numerical_models;
#[path = "🥞️diaphragm/🦀️.rs"]
mod diaphragm;
pub use diaphragm::*;
#[path = "🧮️document-cases/🦀️.rs"]
mod document_cases;
pub use document_cases::bracing_case;
pub use document_cases::{document_case_results, document_cases};
pub use shear_and_lateral::{bracing_distribution, Bracing, WALL_LENGTH_KEYS};
pub use reinforced_special_and_ties::{joint_spacing_limit_m, ROW_KEYS};
pub use serviceability_simplified_and_fire::bed_joint_range_m;

/// ⚠️ Result of one evaluator: its outcomes or the reason why the inputs are insufficient.
pub type R = Result<Vec<Outcome>, String>;

/// 🧾 Inputs of one catalogue verification in SI units (m, Pa, N, N·m); keys are the snake names listed in [`Entry::inputs`].
#[derive(Clone, Debug)]
pub struct Case {
    pub id: String,
    pub annex: AnnexChoice,
    pub num: BTreeMap<String, f64>,
    pub text: BTreeMap<String, String>,
    pub vec: BTreeMap<String, Vec<f64>>,
    pub subject: Option<(String, String, LocalizedCopy)>,
    pub paths: BTreeMap<String, String>,
    pub advisory: bool,
}

impl Case {
    pub fn new(id: &str, annex: AnnexChoice) -> Self {
        Self { id: id.into(), annex, num: BTreeMap::new(), text: BTreeMap::new(), vec: BTreeMap::new(), subject: None, paths: BTreeMap::new(), advisory: false }
    }
    /// 🔗️ Bind the case to a document entity: subject entity, subject path and label, so results resolve in the document.
    pub fn bound(mut self, entity: &str, path: &str, label: LocalizedCopy) -> Self {
        self.subject = Some((entity.into(), path.into(), label));
        self
    }
    /// 💡 Report limit outcomes as warnings, never failures (alternative resistance models).
    pub fn advisory(mut self) -> Self {
        self.advisory = true;
        self
    }
    /// 🔗️ Map an input key to the document path a remedy writes.
    pub fn with_path(mut self, key: &str, path: &str) -> Self {
        self.paths.insert(key.into(), path.into());
        self
    }
    pub fn with_paths<'a>(mut self, paths: impl IntoIterator<Item = (&'a str, String)>) -> Self {
        for (key, path) in paths {
            self.paths.insert(key.into(), path);
        }
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
    pub fn with_v(mut self, key: &str, value: &[f64]) -> Self {
        self.vec.insert(key.into(), value.to_vec());
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
    pub fn v(&self, key: &str) -> Result<&[f64], String> {
        self.vec.get(key).map(Vec::as_slice).ok_or_else(|| format!("missing input {key}"))
    }
    pub fn de(&self) -> bool {
        self.annex == AnnexChoice::De
    }
    pub fn material(&self) -> Result<UnitMaterial, String> {
        match self.so("material", "clay") {
            "clay" => Ok(UnitMaterial::Clay),
            "calcium_silicate" => Ok(UnitMaterial::CalciumSilicate),
            "aerated" | "aac" => Ok(UnitMaterial::Aerated),
            "concrete" => Ok(UnitMaterial::Concrete),
            other => Err(format!("unknown material {other}")),
        }
    }
    pub fn group(&self) -> UnitGroup {
        match self.o("group", 1.0) as u8 {
            2 => UnitGroup::Group2,
            3 => UnitGroup::Group3,
            4 => UnitGroup::Group4,
            _ => UnitGroup::Group1,
        }
    }
    pub fn mortar(&self) -> Result<MortarType, String> {
        match self.so("mortar", "gp") {
            "gp" => Ok(MortarType::GeneralPurpose),
            "tl" => Ok(MortarType::ThinLayer),
            "lw" => Ok(MortarType::Lightweight),
            other => Err(format!("unknown mortar {other}")),
        }
    }
    pub fn gamma_m(&self) -> f64 {
        self.opt("gamma_m").unwrap_or_else(|| {
            let class = match self.o("class", 1.0) as u8 {
                2 => MasonryClass::Class2,
                3 => MasonryClass::Class3,
                4 => MasonryClass::Class4,
                5 => MasonryClass::Class5,
                _ => MasonryClass::Class1,
            };
            if self.de() { class.gamma_m_de(matches!(self.so("situation", "persistent"), "accidental" | "transient" | "seismic")) } else { class.gamma_m_en() }
        })
    }
    pub fn f_k_pa(&self) -> Result<f64, String> {
        match self.opt("fk_pa") {
            Some(v) => Ok(v),
            None => Ok(f_k_mpa_from(self.annex, self.material()?, self.group(), self.mortar()?, self.n("fb_pa")? / 1e6, self.n("fm_pa")? / 1e6) * 1e6),
        }
    }
    pub fn f_d_pa(&self) -> Result<f64, String> {
        match self.opt("fd_pa") {
            Some(v) => Ok(v),
            None => Ok(self.f_k_pa()? / self.gamma_m()),
        }
    }
}

/// 🎚 How an outcome is judged: `Limit` fails above one, `Warn` only warns, `Info` reports a derived value in `capacity`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mode {
    Limit,
    Warn,
    Info,
}

/// 🔧 A remedy of an outcome: the input key to change, its current and required value, and the direction.
#[derive(Clone, Debug)]
pub struct Fix {
    pub key: &'static str,
    pub kind: QuantityKind,
    pub current: f64,
    pub required: f64,
    pub increase: bool,
    pub note: Option<(String, String)>,
}

/// 📐 One quantity comparison or derived value of a verification; values are SI.
#[derive(Clone, Debug)]
pub struct Outcome {
    pub key: &'static str,
    pub en: String,
    pub de: String,
    pub kind: QuantityKind,
    pub demand: f64,
    pub capacity: f64,
    pub mode: Mode,
    pub steps: String,
    pub steps_de: String,
    pub basis: Vec<&'static str>,
    pub fix: Option<Fix>,
}

impl Outcome {
    fn make(kind: QuantityKind, mode: Mode, key: &'static str, en: &str, de: &str, demand: f64, capacity: f64, steps: String) -> Self {
        Self { key, en: en.into(), de: de.into(), kind, demand, capacity, mode, steps_de: steps.clone(), steps, basis: Vec::new(), fix: None }
    }
    pub fn limit(kind: QuantityKind, key: &'static str, en: &str, de: &str, demand: f64, capacity: f64, steps: String) -> Self {
        Self::make(kind, Mode::Limit, key, en, de, demand, capacity, steps)
    }
    pub fn warn(kind: QuantityKind, key: &'static str, en: &str, de: &str, demand: f64, capacity: f64, steps: String) -> Self {
        Self::make(kind, Mode::Warn, key, en, de, demand, capacity, steps)
    }
    pub fn info(kind: QuantityKind, key: &'static str, en: &str, de: &str, value: f64, steps: String) -> Self {
        Self::make(kind, Mode::Info, key, en, de, 0.0, value, steps)
    }
    /// 📝 German calculation steps; the constructors set both languages from the language-neutral formula.
    pub fn steps_de(mut self, text: impl Into<String>) -> Self {
        self.steps_de = text.into();
        self
    }
    pub fn basis(mut self, ids: &[&'static str]) -> Self {
        self.basis.extend_from_slice(ids);
        self
    }
    pub fn fix(mut self, key: &'static str, kind: QuantityKind, current: f64, required: f64, increase: bool) -> Self {
        self.fix = Some(Fix { key, kind, current, required, increase, note: None });
        self
    }
    /// 💬 Word the remedy of the fix in both languages with the computed value instead of the generic "increase … / reduce …".
    pub fn fix_note(mut self, en: String, de: String) -> Self {
        if let Some(fix) = self.fix.as_mut() {
            fix.note = Some((en, de));
        }
        self
    }
    pub fn utilization(&self) -> f64 {
        if self.mode == Mode::Info || self.capacity.abs() < f64::EPSILON { 0.0 } else { self.demand / self.capacity }
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

/// 🗄 Every catalogue entry M-01 … M-95 implemented by this artifact.
pub fn registry() -> Vec<&'static Entry> {
    let mut all: Vec<&'static Entry> = Vec::new();
    for group in [materials::ENTRIES, vertical_load::ENTRIES, shear_and_lateral::ENTRIES, reinforced_special_and_ties::ENTRIES, serviceability_simplified_and_fire::ENTRIES] {
        all.extend(group.iter());
    }
    all
}

/// 🔎 Entry by catalogue id such as `M-14`.
pub fn entry(id: &str) -> Option<&'static Entry> {
    registry().into_iter().find(|e| e.id == id)
}

fn subject(case: &Case, leaf: &str) -> SubjectRef {
    match &case.subject {
        Some((entity, path, label)) => SubjectRef::new(entity, path, label.clone()),
        None => SubjectRef::new(&case.id, format!("catalogue[id={}].{leaf}", case.id), loc(&case.id, &case.id)),
    }
}

fn remedy_target(case: &Case, key: &str) -> Option<SubjectRef> {
    match (&case.subject, case.paths.get(key)) {
        (Some((entity, _, label)), Some(path)) => Some(SubjectRef::new(entity, path, label.clone())),
        (Some(_), None) => None,
        (None, _) => Some(subject(case, &format!("inputs.{key}"))),
    }
}

/// 🧾 Raw outcomes of one catalogue entry, `Err` with the reason when inputs are missing or the id is unknown.
pub fn evaluate_outcomes(entry_id: &str, case: &Case) -> R {
    (entry(entry_id).ok_or_else(|| format!("unknown catalogue entry {entry_id}"))?.eval)(case)
}

/// 🧮 Evaluate one case against its catalogue entry; missing inputs yield a single not-applicable result.
pub fn evaluate_case(entry_id: &str, case: &Case) -> Vec<CheckResult> {
    let Some(e) = entry(entry_id) else {
        return vec![CheckResult::assess(format!("en1996.{entry_id}.unknown"), "EN 1996", clause("EN 1996", "§0", "0"), subject(case, "id"), loc("Unknown catalogue entry", "Unbekannter Katalogeintrag"))
            .annex(case.annex)
            .not_applicable(loc(&format!("No catalogue entry {entry_id}."), &format!("Kein Katalogeintrag {entry_id}.")))
            .build()];
    };
    let base = |key: &str, en: &str, de: &str| {
        CheckResult::assess(format!("en1996.{}.{key}.{}", e.id.to_ascii_lowercase(), case.id), e.part, clause(e.part, &format!("§{}", e.clause), e.clause), subject(case, "inputs"), loc(en, de)).annex(case.annex)
    };
    let (evaluated, read) = semio_s_plugin_norm_compliance::record_ndp_reads(|| (e.eval)(case));
    match evaluated {
        Err(reason) => vec![base("input", e.en, e.de).not_applicable(semio_s_plugin_norm_compliance::reason_copy(e.id, &reason)).build()],
        Ok(outcomes) => outcomes
            .into_iter()
            .map(|o| {
                let basis = if o.basis.is_empty() { String::new() } else { format!(" Unverified NDP basis: {}.", o.basis.join(", ")) };
                let basis_de = if o.basis.is_empty() { String::new() } else { format!(" Ungeprüfte NDP-Grundlage: {}.", o.basis.join(", ")) };
                let mut b = base(o.key, &format!("{} – {}", e.en, o.en), &format!("{} – {}", e.de, o.de))
                    .utilization(Quantity::new(o.kind, o.demand), Quantity::new(o.kind, o.capacity))
                    .explanation(loc(&format!("{}{basis}", o.steps), &format!("{}{basis_de}", o.steps_de)));
                let mode = if case.advisory && o.mode == Mode::Limit { Mode::Warn } else { o.mode };
                if mode == Mode::Warn {
                    b = b.warn_above(1.0).status(CheckStatus::Pass);
                }
                if mode == Mode::Limit && o.utilization() > 1.0 {
                    b = match o.fix.as_ref().and_then(|f| remedy_target(case, f.key).map(|target| (f, target))) {
                        Some((f, target)) => {
                            let (current, required) = (Quantity::new(f.kind, f.current), Quantity::new(f.kind, f.required));
                            let words = match &f.note {
                                Some((en, de)) => loc(en, de),
                                None if f.increase => loc(&format!("Increase {}.", f.key), &format!("{} erhöhen.", f.key)),
                                None => loc(&format!("Reduce {}.", f.key), &format!("{} verringern.", f.key)),
                            };
                            b.remedy(if f.increase { Remedy::at_least(target, current, required, words) } else { Remedy::at_most(target, current, required, words) })
                        }
                        None => {
                            let words = match o.fix.as_ref().and_then(|f| f.note.as_ref()) {
                                Some((en, de)) => loc(en, de),
                                None => loc("Revise the listed inputs.", "Die genannten Eingaben überarbeiten."),
                            };
                            b.remedy(Remedy::one_of(subject(case, "inputs"), e.inputs.iter().map(|s| s.to_string()).collect(), words))
                        }
                    };
                }
                for id in o.basis.iter().chain(read.iter()) {
                    b = b.ndp(&if id.starts_with("ndp.") { (*id).to_owned() } else { format!("ndp.en1996.{id}") });
                }
                b.build()
            })
            .collect(),
    }
}

#[path = "📚️inputs/🦀️.rs"]
pub mod case_inputs;

#[path = "🗂️cases/🦀️.rs"]
mod cases;
pub use cases::*;

#[cfg(test)]
#[path = "🧪️tests/🗣️german-reasons/🦀️.rs"]
mod german_reasons;

/// 📂️ Directory of the catalogue sources, scanned by the superseded-form tests.
pub const CATALOGUE_SOURCE_ROOT: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../..");
