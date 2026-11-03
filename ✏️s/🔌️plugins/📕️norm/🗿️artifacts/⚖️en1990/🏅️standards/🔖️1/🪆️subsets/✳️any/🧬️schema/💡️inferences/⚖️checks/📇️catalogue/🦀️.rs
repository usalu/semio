//! 📇️ Verification catalogue of EN 1990: the case input, the outcome of one verification and the registry that maps every catalogue id E90.nn to its evaluator. See [`super`].

use super::*;
use std::collections::BTreeMap;

/// ⚠️ Result of one evaluator: its outcomes or the reason why the inputs are insufficient.
pub type R = Result<Vec<Outcome>, String>;

/// 🧾 Inputs of one catalogue verification (forces in N, lengths in m unless a key says mm, stresses in N/mm²); keys are listed in [`Entry::inputs`].
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
    pub fn s(&self, key: &str) -> Result<&str, String> {
        self.text.get(key).map(String::as_str).ok_or_else(|| format!("missing input {key}"))
    }
    pub fn so<'a>(&'a self, key: &str, default: &'a str) -> &'a str {
        self.text.get(key).map_or(default, String::as_str)
    }
    fn domain(&self) -> Result<Domain, String> {
        match self.so("structure", "building") {
            "building" => Ok(Domain::Building),
            "bridge" => Ok(Domain::Bridge),
            other => Domain::of(other).ok_or_else(|| format!("unknown structure {other}")),
        }
    }
    fn k_fi(&self) -> Result<f64, String> {
        match self.opt("k_fi") {
            Some(k) => Ok(k),
            None => k_fi_for_rc(self.o("rc", 2.0) as u8).ok_or_else(|| "rc must be 1..3".to_string()),
        }
    }
    fn context(&self, set: FactorSet) -> Result<Context, String> {
        let mut ctx = Context::new(self.annex, self.domain()?, set, self.k_fi()?);
        ctx.accidental_leading_psi1 = self.so("lead_psi", "psi1") != "psi2";
        Ok(ctx)
    }
    fn actions(&self) -> Result<Vec<Action>, String> {
        let altitude = self.o("altitude", 0.0);
        let mut actions = Action::parse_list(self.s("actions")?)?;
        for action in actions.iter_mut().filter(|action| action.class == ActionClass::Variable) {
            action.category = resolve_psi_category(self.annex, &action.category, altitude);
        }
        Ok(actions)
    }
}

/// 🎚 How an outcome is judged.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mode {
    Limit,
    Minimum,
    Advisory,
    Info,
}

/// 📐 One quantity comparison or derived value of a verification; `demand` is the effect (or the required minimum) and `capacity` the limit (or the provided value).
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
    /// 📝 German calculation steps; the constructors set both languages from the language-neutral formula.
    pub fn steps_de(mut self, text: impl Into<String>) -> Self {
        self.steps_de = text.into();
        self
    }
    pub fn limit(key: &str, en: &str, de: &str, kind: QuantityKind, demand: f64, capacity: f64, steps: String) -> Self {
        Self::make(kind, Mode::Limit, key, en, de, demand, capacity, steps)
    }
    pub fn minimum(key: &str, en: &str, de: &str, kind: QuantityKind, provided: f64, required: f64, steps: String) -> Self {
        Self::make(kind, Mode::Minimum, key, en, de, required, provided, steps)
    }
    pub fn advisory(key: &str, en: &str, de: &str, kind: QuantityKind, provided: f64, required: f64, steps: String) -> Self {
        Self::make(kind, Mode::Advisory, key, en, de, required, provided, steps)
    }
    pub fn info(key: &str, en: &str, de: &str, kind: QuantityKind, value: f64, steps: String) -> Self {
        Self::make(kind, Mode::Info, key, en, de, 0.0, value, steps)
    }
    pub fn utilization(&self) -> f64 {
        match self.mode {
            Mode::Info => 0.0,
            Mode::Limit => {
                if self.capacity.abs() < f64::EPSILON { 0.0 } else { self.demand / self.capacity }
            }
            Mode::Minimum | Mode::Advisory => {
                if self.capacity >= self.demand { self.demand / self.capacity.max(self.demand).max(f64::EPSILON) } else { self.capacity / self.demand.max(f64::EPSILON) }
            }
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

/// 🗄 Every catalogue entry E90.01 … E90.30 implemented by this artifact.
pub fn registry() -> Vec<&'static Entry> {
    let mut all: Vec<&'static Entry> = Vec::new();
    for group in [BASIS_ENTRIES, ACTION_ENTRIES, COMBINATION_ENTRIES, ANNEX_ENTRIES, RELIABILITY_ENTRIES] {
        all.extend(group.iter());
    }
    all
}

/// 🔎 Entry by catalogue id such as `E90.16`.
pub fn entry(id: &str) -> Option<&'static Entry> {
    registry().into_iter().find(|e| e.id == id)
}

fn loc(en: &str, de: &str) -> LocalizedCopy {
    LocalizedCopy::new(en, de)
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
    let Some(e) = entry(entry_id) else {
        return vec![CheckResult::assess(format!("en1990.{entry_id}.unknown"), "EN 1990", ClauseId::new("EN 1990", "§0", "0"), subject(case), loc("Unknown catalogue entry", "Unbekannter Katalogeintrag"))
            .annex(case.annex)
            .not_applicable(loc(&format!("No catalogue entry {entry_id}."), &format!("Kein Katalogeintrag {entry_id}.")))
            .build()];
    };
    let cited: &[&str] = match e.id {
        "E90.02" => &["ndp.en1990.e90-02.1"],
        "E90.16" if case.annex == AnnexChoice::De => &["ndp.en1990.e90-16.1"],
        "E90.24" => &["ndp.en1990.e90-24.1", "ndp.en1990.e90-24.2"],
        "E90.25" => &["ndp.en1990.e90-25.1", "ndp.en1990.e90-25.2"],
        "E90.26" => &["ndp.en1990.e90-26.1"],
        "E90.27" => &["ndp.en1990.e90-27.2", "ndp.en1990.e90-27.3"],
        "E90.28" => &["ndp.en1990.e90-28.1"],
        _ => &[],
    };
    let (evaluated, read) = semio_s_plugin_norm_compliance::record_ndp_reads(|| (e.eval)(case));
    let base = |key: &str, en: &str, de: &str| {
        let mut builder = CheckResult::assess(format!("en1990.{}.{key}.{}", e.id.to_ascii_lowercase(), case.id), e.part, ClauseId::new("EN 1990", e.part, e.clause), subject(case), loc(en, de)).annex(case.annex);
        for id in cited.iter().chain(read.iter()) {
            builder = builder.ndp(id);
        }
        builder
    };
    match evaluated {
        Err(reason) => vec![base("input", e.en, e.de).not_applicable(semio_s_plugin_norm_compliance::reason_copy(e.id, &reason)).build()],
        Ok(outcomes) => outcomes
            .into_iter()
            .map(|o| {
                let title_en = format!("{} – {}", e.en, o.en);
                let title_de = format!("{} – {}", e.de, o.de);
                let (demand, capacity) = (Quantity::new(o.kind, o.demand), Quantity::new(o.kind, o.capacity));
                let builder = base(&o.key, &title_en, &title_de).explanation(loc(&o.steps, &o.steps_de));
                let mut built = match o.mode {
                    Mode::Info => builder.status(CheckStatus::Pass).build(),
                    Mode::Limit => {
                        let failing = !(o.demand <= o.capacity) || o.capacity <= 0.0;
                        let builder = builder.utilization(demand, capacity);
                        let builder = if failing {
                            builder.remedy(Remedy { applicable: false, ..Remedy::at_most(subject(case), demand, capacity, loc(&format!("Reduce the effect to at most the limit ({}).", o.key), &format!("Beanspruchung auf höchstens den Grenzwert begrenzen ({}).", o.key))) })
                        } else {
                            builder
                        };
                        builder.build()
                    }
                    Mode::Minimum | Mode::Advisory => {
                        let failing = o.capacity < o.demand;
                        let builder = builder.minimum(capacity, demand);
                        let builder = if failing && o.mode == Mode::Minimum {
                            builder.remedy(Remedy { applicable: false, ..Remedy::at_least(subject(case), capacity, demand, loc(&format!("Increase the value to at least the required minimum ({}).", o.key), &format!("Wert auf mindestens den erforderlichen Wert erhöhen ({}).", o.key))) })
                        } else if failing {
                            builder.status(CheckStatus::Warning)
                        } else {
                            builder
                        };
                        builder.build()
                    }
                };
                if o.mode == Mode::Info {
                    built.computed = demand;
                    built.limit = capacity;
                }
                built
            })
            .collect(),
    }
}

/// 🔤 Compact numeric formatting for explanation text.
pub(super) fn f(x: f64) -> String {
    if x.abs() >= 1.0e5 || (x != 0.0 && x.abs() < 1.0e-3) { format!("{x:.4e}") } else { format!("{x:.4}") }
}

const DIM: QuantityKind = QuantityKind::Dimensionless;
const LEN: QuantityKind = QuantityKind::Length;

fn info(key: &str, en: &str, de: &str, value: f64, steps: String) -> Outcome {
    Outcome::info(key, en, de, DIM, value, steps)
}

//#region 🔖️BasisEntries
fn e90_01(c: &Case) -> R {
    let (group, steps) = match c.s("use")? {
        "building" | "crane_runway" | "silo" | "tower" => (1.0, "buildings and building-like structures verify against Annex A1 (Tables A1.1-A1.2)"),
        "bridge" | "footbridge" | "railway" => (2.0, "bridges verify against Annex A2 (Tables A2.1-A2.4)"),
        other => return Err(format!("unknown use {other}")),
    };
    Ok(vec![info("annex-group", "Applicable annex group (1 = A1, 2 = A2)", "Maßgebender Anhang (1 = A1, 2 = A2)", group, format!("{steps}; limit states EQU, STR, GEO, FAT, SLS enabled; UPL and HYD link to EN 1997"))])
}

fn e90_02(c: &Case) -> R {
    let cc = c.n("cc")?;
    let rc = c.o("rc", cc);
    let k = k_fi_for_rc(rc as u8).ok_or("rc must be 1..3")?;
    within(cc, 1.0, 3.0, "cc")?;
    let gamma_f = c.o("gamma_f", gamma_g(c.annex, Domain::Building, FactorSet::StrB).0);
    let mut out = vec![
        info("k-fi", "K_FI for the reliability class", "K_FI der Zuverlässigkeitsklasse", k, format!("K_FI(RC{}) = {} (Table B3)", rc as u8, f(k))),
        info("gamma-f-eff", "Effective γ_F = K_FI · γ_F (unfavourable)", "Wirksames γ_F = K_FI · γ_F (ungünstig)", k * gamma_f, format!("{} · {} = {}", f(k), f(gamma_f), f(k * gamma_f))),
        Outcome::advisory("rc-vs-cc", "RC consistent with CC", "RC konsistent mit CC", DIM, if (rc - cc).abs() < 1e-9 { 1.0 } else { 0.5 }, 1.0, format!("RC{} against CC{} (Annex B.3 override)", rc as u8, cc as u8)),
    ];
    if let Some(declared) = c.opt("k_fi_declared") {
        out.push(Outcome::minimum("k-fi-declared", "Declared K_FI covers the table value", "Deklariertes K_FI deckt Tabellenwert ab", DIM, declared, k, format!("declared {} vs table {}", f(declared), f(k))));
    }
    Ok(out)
}

fn positive(value: f64, name: &str) -> Result<(), String> {
    if value.is_finite() && value > 0.0 { Ok(()) } else { Err(format!("{name} must be a positive finite number")) }
}

fn within(value: f64, lo: f64, hi: f64, name: &str) -> Result<(), String> {
    if value >= lo && value <= hi { Ok(()) } else { Err(format!("{name} must be within {lo}..{hi}")) }
}

fn e90_03(c: &Case) -> R {
    let category = c.n("category")? as u8;
    let (lo, hi) = design_working_life_indicative(category).ok_or("category must be 1..5")?;
    let mut out = vec![info("range-min", "Indicative design working life, lower bound [a]", "Indikative Nutzungsdauer, Untergrenze [a]", lo, format!("Table 2.1 category {category}")), info("range-max", "Indicative design working life, upper bound [a]", "Indikative Nutzungsdauer, Obergrenze [a]", hi, format!("Table 2.1 category {category}"))];
    if let Some(years) = c.opt("years") {
        out.push(Outcome::minimum("years-min", "Design working life above the category range", "Nutzungsdauer über Kategorieuntergrenze", DIM, years, lo, format!("{} a ≥ {} a", f(years), f(lo))));
        out.push(Outcome::limit("years-max", "Design working life within the category range", "Nutzungsdauer innerhalb der Kategorie", DIM, years, hi, format!("{} a ≤ {} a", f(years), f(hi))));
    }
    Ok(out)
}

fn e90_04(c: &Case) -> R {
    let years = c.n("years")?;
    Ok(vec![info("durability-period", "Period the durability measures must cover [a]", "Zeitraum der Dauerhaftigkeitsmaßnahmen [a]", years, "durability flag handed to EN 1992/1993/1995 environmental classes".into()).steps_de("Dauerhaftigkeitskennzeichen für die Umgebungsklassen von EN 1992/1993/1995")])
}

fn e90_05(c: &Case) -> R {
    let rc = c.n("rc")?;
    let (dsl_req, il_req) = annex_b_levels(rc as u8).ok_or("rc must be 1..3")?;
    let mut out = Vec::new();
    for (key, en, de, required) in [("dsl", "Design supervision level", "Entwurfsüberwachung", dsl_req), ("il", "Inspection level", "Ausführungsüberwachung", il_req)] {
        let provided = c.n(key)?;
        within(provided, 1.0, 3.0, key)?;
        out.push(Outcome::advisory(key, en, de, DIM, provided, f64::from(required), format!("{} {} vs recommended level {} for RC{}", key.to_uppercase(), f(provided), required, rc as u8)));
    }
    Ok(out)
}

fn e90_06(c: &Case) -> R {
    let mut out = Vec::new();
    let mut count = 0.0;
    for situation in c.s("situations")?.split(',').map(str::trim).filter(|s| !s.is_empty()) {
        let equations = match situation {
            "persistent" | "transient" => "6.10 or 6.10a/b (STR/GEO), Table A1.2(A) (EQU)",
            "accidental" => "6.11b",
            "seismic" => "6.12b",
            "fatigue" => "6.4.5 with EN 1992/1993/1995 fatigue rules",
            other => return Err(format!("unknown situation {other}")),
        };
        count += 1.0;
        out.push(info(&format!("situation-{situation}"), &format!("Design situation {situation}"), &format!("Bemessungssituation {situation}"), count, equations.into()));
    }
    if count == 0.0 {
        return Err("situations is empty".into());
    }
    Ok(out)
}

fn e90_07(c: &Case) -> R {
    let mut bits = 0.0;
    for (key, left, right, shift) in [("variation", "permanent", "variable", 0), ("origin", "direct", "indirect", 2), ("spatial", "fixed", "free", 3), ("nature", "static", "dynamic", 4), ("effect", "favourable", "unfavourable", 5)] {
        let value = c.s(key)?;
        let index = if key == "variation" && value == "accidental" { 2 } else if value == left { 0 } else if value == right { 1 } else { return Err(format!("{key} must be {left} or {right}")) };
        bits += f64::from(index << shift);
    }
    Ok(vec![info("class-bits", "Action class bit field", "Wirkungsklassen-Bitfeld", bits, "variation(0-1) | origin(2) | spatial(3) | nature(4) | effect(5)".into()).steps_de("Veränderlichkeit(0-1) | Ursprung(2) | räumlich(3) | Art(4) | Wirkung(5)")])
}

fn e90_08(c: &Case) -> R {
    let mut out = Vec::new();
    if let (Some(mean), Some(v)) = (c.opt("g_mean"), c.opt("v_g")) {
        let (sup, inf, single) = if v <= 0.05 { (mean, mean, true) } else { (mean * (1.0 + 1.64 * v), mean * (1.0 - 1.64 * v), false) };
        out.push(info("g-k-sup", "G_k,sup", "G_k,sup", sup, if single { "V_G ≤ 0.05: single value G_k = mean".into() } else { format!("G_m(1 + 1.64 V_G) = {}", f(sup)) }));
        out.push(info("g-k-inf", "G_k,inf", "G_k,inf", inf, if single { "V_G ≤ 0.05: single value G_k = mean".into() } else { format!("G_m(1 − 1.64 V_G) = {}", f(inf)) }));
        out.push(Outcome::advisory("v-g-bound", "V_G small enough for the distribution assumption", "V_G klein genug für die Verteilungsannahme", DIM, 0.1, v.max(1e-9), format!("V_G = {} against 0.1 (separate upper and lower values beyond)", f(v))));
    }
    if let (Some(mean), Some(cov)) = (c.opt("q_mean"), c.opt("q_cov")) {
        let qk = annual_maximum_characteristic(mean, cov);
        out.push(info("q-k", "Q_k = 98 % fractile of the annual maximum", "Q_k = 98 %-Fraktile des Jahresmaximums", qk, format!("Gumbel mean {} V {} → {}", f(mean), f(cov), f(qk))));
    }
    if out.is_empty() {
        return Err("give g_mean and v_g, or q_mean and q_cov".into());
    }
    Ok(out)
}

fn e90_09(c: &Case) -> R {
    let row = match c.text.get("category") {
        Some(category) => psi_lookup(c.annex, &resolve_psi_category(c.annex, category, c.o("altitude", 0.0))).ok_or_else(|| format!("unknown ψ category {category}"))?,
        None => PsiRow { psi_0: c.n("psi0")?, psi_1: c.n("psi1")?, psi_2: c.n("psi2")? },
    };
    Ok(vec![
        info("psi0", "ψ0 (combination value)", "ψ0 (Kombinationswert)", row.psi_0, "Q_rep = ψ0 Q_k".into()),
        info("psi1", "ψ1 (frequent value)", "ψ1 (häufiger Wert)", row.psi_1, "Q_rep = ψ1 Q_k".into()),
        info("psi2", "ψ2 (quasi-permanent value)", "ψ2 (quasi-ständiger Wert)", row.psi_2, "Q_rep = ψ2 Q_k".into()),
        Outcome::minimum("ordered", "ψ0 ≥ ψ1 ≥ ψ2 ≥ 0", "ψ0 ≥ ψ1 ≥ ψ2 ≥ 0", DIM, 1.0 + (row.psi_0 - row.psi_1).min(row.psi_1 - row.psi_2).min(row.psi_2), 1.0, "plausibility of the factor triple".into()).steps_de("Plausibilität des Faktorentripels"),
    ])
}

fn e90_10(c: &Case) -> R {
    let (xk, eta) = (c.n("x_k")?, c.o("eta", 1.0));
    positive(c.opt("gamma_m").unwrap_or(1.0), "gamma_m")?;
    let gamma_m = match c.opt("gamma_m") {
        Some(g) => g,
        None => {
            let material = c.so("material", "other");
            ndp_lookup(c.annex, &format!("ndp.en1990.gM.{material}")).ok_or_else(|| format!("unknown material {material}"))?
        }
    };
    let xd = eta * xk / gamma_m;
    let mut out = vec![info("x-d", "Design property X_d = η X_k / γ_m", "Bemessungswert X_d = η X_k / γ_m", xd, format!("{} · {} / {} = {}", f(eta), f(xk), f(gamma_m), f(xd)))];
    if let Some(ed) = c.opt("e_d") {
        out.push(Outcome::limit("e-d-vs-x-d", "Demand within the design property", "Beanspruchung innerhalb des Bemessungswerts", DIM, ed, xd, format!("{} ≤ {}", f(ed), f(xd))));
    }
    Ok(out)
}

fn e90_11(c: &Case) -> R {
    let (nominal, delta) = (c.n("a_nom")?, c.n("delta_a")?.abs());
    Ok(vec![info("a-d-plus", "a_d = a_nom + Δa", "a_d = a_nom + Δa", nominal + delta, "6.4".into()), info("a-d-minus", "a_d = a_nom − Δa", "a_d = a_nom − Δa", nominal - delta, "6.4".into())])
}

fn e90_12(c: &Case) -> R {
    let analysis = c.s("analysis")?;
    if !matches!(analysis, "first-order" | "second-order" | "plastic" | "non-linear") {
        return Err(format!("unknown analysis {analysis}"));
    }
    let mut out = vec![info("analysis", "Analysis model flag (1 first-order, 2 second-order, 3 plastic, 4 non-linear)", "Berechnungsmodell (1 Theorie I, 2 Theorie II, 3 plastisch, 4 nichtlinear)", match analysis { "first-order" => 1.0, "second-order" => 2.0, "plastic" => 3.0, _ => 4.0 }, "material parts own the α_cr thresholds".into()).steps_de("die Werkstoffteile besitzen die α_cr-Schwellen")];
    if let Some(alpha) = c.opt("alpha_cr") {
        out.push(Outcome::advisory("alpha-cr", "α_cr ≥ 10 permits first-order analysis", "α_cr ≥ 10 erlaubt Theorie I", DIM, alpha, 10.0, format!("α_cr = {}", f(alpha))));
    }
    Ok(out)
}

fn e90_14(_c: &Case) -> R {
    Ok(vec![info("limit-states", "Limit states EQU, STR, GEO, FAT (UPL, HYD link to EN 1997)", "Grenzzustände EQU, STR, GEO, FAT (UPL, HYD siehe EN 1997)", 4.0, "6.4.1 (6.1)-(6.4)".into())])
}

pub const BASIS_ENTRIES: &[Entry] = &[
    Entry { id: "E90.01", part: "§2.1", clause: "2.1", en: "Basic requirements", de: "Grundlegende Anforderungen", inputs: &["use"], eval: e90_01 },
    Entry { id: "E90.02", part: "Annex B", clause: "B3", en: "Consequence and reliability classes", de: "Schadensfolge- und Zuverlässigkeitsklassen", inputs: &["cc", "rc", "gamma_f", "k_fi_declared"], eval: e90_02 },
    Entry { id: "E90.03", part: "§2.3", clause: "Table 2.1", en: "Design working life", de: "Entwurfsnutzungsdauer", inputs: &["category", "years"], eval: e90_03 },
    Entry { id: "E90.04", part: "§2.4", clause: "2.4", en: "Durability", de: "Dauerhaftigkeit", inputs: &["years"], eval: e90_04 },
    Entry { id: "E90.05", part: "Annex B", clause: "B4-B5", en: "Design supervision and inspection levels", de: "Entwurfs- und Ausführungsüberwachung", inputs: &["rc", "dsl", "il"], eval: e90_05 },
    Entry { id: "E90.06", part: "§3.2", clause: "3.2", en: "Design situations", de: "Bemessungssituationen", inputs: &["situations"], eval: e90_06 },
    Entry { id: "E90.07", part: "§4.1.1", clause: "4.1.1", en: "Classification of actions", de: "Einwirkungsklassifikation", inputs: &["variation", "origin", "spatial", "nature", "effect"], eval: e90_07 },
    Entry { id: "E90.08", part: "§4.1.2", clause: "4.1.2", en: "Characteristic values", de: "Charakteristische Werte", inputs: &["g_mean", "v_g", "q_mean", "q_cov"], eval: e90_08 },
    Entry { id: "E90.09", part: "§4.1.3", clause: "4.1.3", en: "Other representative values", de: "Weitere repräsentative Werte", inputs: &["category", "altitude", "psi0", "psi1", "psi2"], eval: e90_09 },
    Entry { id: "E90.10", part: "§4.2", clause: "4.2", en: "Material properties", de: "Materialeigenschaften", inputs: &["x_k", "eta", "gamma_m", "material", "e_d"], eval: e90_10 },
    Entry { id: "E90.11", part: "§4.3", clause: "4.3", en: "Geometrical data", de: "Geometrische Größen", inputs: &["a_nom", "delta_a"], eval: e90_11 },
    Entry { id: "E90.12", part: "§5.1", clause: "5.1.1-5.1.4", en: "Structural analysis models", de: "Berechnungsmodelle", inputs: &["analysis", "alpha_cr"], eval: e90_12 },
    Entry { id: "E90.13", part: "§5.2", clause: "5.2", en: "Design assisted by testing", de: "Versuchsgestützte Bemessung", inputs: &["n", "mean", "v", "v_known", "dist", "eta_d", "gamma_m", "pairs", "v_rt", "g_rt_mean"], eval: e90_30 },
    Entry { id: "E90.14", part: "§6.4.1", clause: "6.4.1", en: "Limit state definitions", de: "Definition der Grenzzustände", inputs: &[], eval: e90_14 },
];
//#endregion 🔖️BasisEntries

//#region 🔖️ActionEntries
fn e90_20(c: &Case) -> R {
    let (gamma_f, gamma_sd, gamma_m, gamma_rd) = (c.n("gamma_f")?, c.o("gamma_sd", 1.0), c.n("gamma_m")?, c.o("gamma_rd", 1.0));
    positive(gamma_m, "gamma_m")?;
    positive(gamma_rd, "gamma_rd")?;
    let (f_rep, x_k, eta, section) = (c.n("f_rep")?, c.n("x_k")?, c.o("eta", 1.0), c.o("section", 1.0));
    let gamma_total = gamma_f * gamma_sd;
    let e_d = gamma_total * f_rep;
    let x_d = eta * x_k / gamma_m;
    let r_d = section * x_d / gamma_rd;
    Ok(vec![
        info("gamma-F", "γ_F = γ_f · γ_Sd", "γ_F = γ_f · γ_Sd", gamma_total, format!("{} · {}", f(gamma_f), f(gamma_sd))),
        info("e-d", "E_d = γ_F F_rep", "E_d = γ_F F_rep", e_d, format!("{} · {}", f(gamma_total), f(f_rep))),
        info("x-d", "X_d = η X_k / γ_m", "X_d = η X_k / γ_m", x_d, format!("{} · {} / {}", f(eta), f(x_k), f(gamma_m))),
        Outcome::limit("verification", "E_d ≤ R_d", "E_d ≤ R_d", DIM, e_d, r_d, format!("E_d = {}, R_d = {}", f(e_d), f(r_d))),
    ])
}

fn parse_pairs(text: &str) -> Result<Vec<(f64, f64)>, String> {
    let rows: Result<Vec<(f64, f64)>, String> = text
        .split(';')
        .map(str::trim)
        .filter(|row| !row.is_empty())
        .map(|row| {
            let (range, cycles) = row.split_once(':').ok_or_else(|| format!("row {row:?} needs first:second"))?;
            Ok((range.trim().parse::<f64>().map_err(|_| format!("bad first value in {row:?}"))?, cycles.trim().parse::<f64>().map_err(|_| format!("bad second value in {row:?}"))?))
        })
        .collect();
    let rows = rows?;
    if rows.is_empty() { Err("no rows".into()) } else { Ok(rows) }
}

fn e90_21(c: &Case) -> R {
    let spectrum = parse_pairs(c.s("spectrum")?)?;
    let delta_c = c.n("delta_sigma_c")? / c.o("gamma_mf", 1.0);
    let gamma_ff = c.o("gamma_ff", 1.0);
    let scaled: Vec<(f64, f64)> = spectrum.iter().map(|(range, cycles)| (range * gamma_ff, *cycles)).collect();
    let damage = miner_damage(&scaled, delta_c);
    Ok(vec![Outcome::limit("miner", "Palmgren-Miner damage Σ n_i/N_i ≤ 1", "Palmgren-Miner-Schädigung Σ n_i/N_i ≤ 1", DIM, damage, 1.0, format!("{} rows, Δσ_C/γ_Mf = {}, damage = {}", scaled.len(), f(delta_c), f(damage)))])
}

pub const ACTION_ENTRIES: &[Entry] = &[
    Entry { id: "E90.20", part: "§6.3", clause: "6.3.1-6.3.2", en: "Partial factors and design values", de: "Teilsicherheitsbeiwerte und Bemessungswerte", inputs: &["gamma_f", "gamma_sd", "gamma_m", "gamma_rd", "f_rep", "x_k", "eta", "section"], eval: e90_20 },
    Entry { id: "E90.21", part: "§6.4.5", clause: "6.4.5", en: "Fatigue limit state (Palmgren-Miner)", de: "Ermüdungsgrenzzustand (Palmgren-Miner)", inputs: &["spectrum", "delta_sigma_c", "gamma_mf", "gamma_ff"], eval: e90_21 },
];
//#endregion 🔖️ActionEntries

//#region 🔖️CombinationEntries
fn limit_or_info(key: &str, en: &str, de: &str, value: f64, r_d: Option<f64>, steps: String) -> Outcome {
    match r_d {
        Some(r) => Outcome::limit(key, en, de, DIM, value, r, steps),
        None => info(key, en, de, value, steps),
    }
}

fn envelope(ctx: &Context, actions: &[Action], equation: Equation, net: bool) -> Result<Option<Combination>, String> {
    Ok(governing(&combinations(ctx, actions, equation)?, net).cloned())
}

fn e90_15(c: &Case) -> R {
    let ctx = c.context(FactorSet::EquA)?;
    let actions = c.actions()?;
    let t_d = c.o("t_d", 0.0);
    let combos = combinations(&ctx, &actions, Equation::E610)?;
    let worst = governing(&combos, true).ok_or("no actions")?;
    let mut out = vec![
        info("e-dst", "E_d,dst (destabilising)", "E_d,dst (destabilisierend)", worst.unfavourable, worst.describe()),
        info("e-stb", "E_d,stb (stabilising)", "E_d,stb (stabilisierend)", worst.stabilising(), "γG,inf on permanent actions whose effect relieves, variable actions never stabilise".into()).steps_de("γG,inf auf ständige Einwirkungen mit entlastender Wirkung, veränderliche Einwirkungen stabilisieren nie"),
        Outcome::limit("equ", "E_d,dst ≤ E_d,stb + T_d", "E_d,dst ≤ E_d,stb + T_d", DIM, worst.unfavourable, worst.stabilising() + t_d, format!("{} ≤ {} + {}", f(worst.unfavourable), f(worst.stabilising()), f(t_d))),
    ];
    if t_d > 0.0 {
        let alternative = envelope(&c.context(FactorSet::StrB)?, &actions, Equation::E610, true)?.ok_or("no actions")?;
        out.push(Outcome::limit("equ-set-b", "E_d,dst ≤ E_d,stb + T_d with the Set B factors (resistance involved, A1.2(A) note)", "E_d,dst ≤ E_d,stb + T_d mit den Beiwerten der Reihe B (Widerstand beteiligt, Hinweis zu A1.2(A))", DIM, alternative.unfavourable, alternative.stabilising() + t_d, format!("{} ≤ {} + {}", f(alternative.unfavourable), f(alternative.stabilising()), f(t_d))));
    }
    Ok(out)
}

fn resistance(c: &Case) -> Option<f64> {
    c.opt("r_d")
}

fn e90_16(c: &Case) -> R {
    let set = match c.so("set", "B") {
        "B" => FactorSet::StrB,
        "C" => FactorSet::GeoC,
        "C-geo2" => FactorSet::Geo2,
        other => return Err(format!("set must be B, C (GEO-3) or C-geo2, got {other}")),
    };
    let ctx = c.context(set)?;
    let actions = c.actions()?;
    let r_d = resistance(c);
    let mut out = Vec::new();
    let e610 = combinations(&ctx, &actions, Equation::E610)?;
    let worst = governing(&e610, false).ok_or("no actions")?;
    out.push(limit_or_info("e610", "Eq. 6.10 governing", "Gl. 6.10 maßgebend", worst.total(), r_d, worst.describe()));
    if offers_610ab(c.annex, ctx.domain) && set == FactorSet::StrB {
        let a = combinations(&ctx, &actions, Equation::E610a)?;
        let b = combinations(&ctx, &actions, Equation::E610b)?;
        let (wa, wb) = (governing(&a, false).ok_or("no actions")?, governing(&b, false).ok_or("no actions")?);
        out.push(limit_or_info("e610a", "Eq. 6.10a governing", "Gl. 6.10a maßgebend", wa.total(), r_d, wa.describe()));
        out.push(limit_or_info("e610b", "Eq. 6.10b governing", "Gl. 6.10b maßgebend", wb.total(), r_d, wb.describe()));
        out.push(limit_or_info("e610ab", "max(6.10a, 6.10b)", "max(6.10a, 6.10b)", wa.total().max(wb.total()), r_d, "E_d = max(6.10a, 6.10b)".into()));
    }
    if c.annex == AnnexChoice::De && set == FactorSet::StrB && actions.iter().filter(|a| a.class == ActionClass::Variable && a.value > 0.0).count() > 1 {
        let g: f64 = actions.iter().filter(|a| matches!(a.class, ActionClass::Permanent) && a.value > 0.0).map(|a| a.value).sum();
        let q: f64 = actions.iter().filter(|a| a.class == ActionClass::Variable && a.value > 0.0).map(|a| a.value).sum();
        let (g_sup, _) = gamma_g(c.annex, ctx.domain, FactorSet::StrB);
        out.push(limit_or_info("simplified", "NA-DE simplified rule γG ΣG + 1.35 ΣQ", "NA-DE vereinfachte Regel γG ΣG + 1,35 ΣQ", ctx.k_fi * (g_sup * g + ndp(c.annex, "ndp.en1990.simplified.gq") * q), r_d, "NA-DE simplified rule on the adverse permanent and variable actions (unverified NDP)".into()).steps_de("vereinfachte NA-DE-Regel für die ungünstigen ständigen und veränderlichen Einwirkungen (ungeprüfter NDP)"));
    }
    Ok(out)
}

fn e90_17(c: &Case) -> R {
    let ctx = c.context(FactorSet::StrB)?;
    let actions = c.actions()?;
    let combos = combinations(&ctx, &actions, Equation::E611b)?;
    let worst = governing(&combos, false).ok_or("no accidental action (class A)")?;
    Ok(vec![limit_or_info("6.11b", "Eq. 6.11b governing", "Gl. 6.11b maßgebend", worst.total(), resistance(c), worst.describe()), info("branches", "Number of generated combinations", "Anzahl erzeugter Kombinationen", combos.len() as f64, "one per A_d and leading variable".into()).steps_de("eine je A_d und Leiteinwirkung")])
}

fn e90_18(c: &Case) -> R {
    let ctx = c.context(FactorSet::StrB)?;
    let actions = c.actions()?;
    let combos = combinations(&ctx, &actions, Equation::E612b)?;
    let worst = governing(&combos, false).ok_or("no seismic action (class E)")?;
    Ok(vec![limit_or_info("6.12b", "Eq. 6.12b governing", "Gl. 6.12b maßgebend", worst.total(), resistance(c), worst.describe())])
}

fn e90_19(c: &Case) -> R {
    let ctx = c.context(FactorSet::StrB)?;
    let actions = c.actions()?;
    let mut out = Vec::new();
    for (equation, key, limit_key, en, de) in [
        (Equation::Characteristic, "6.14b", "cd_char", "Eq. 6.14b characteristic", "Gl. 6.14b charakteristisch"),
        (Equation::Frequent, "6.15b", "cd_freq", "Eq. 6.15b frequent", "Gl. 6.15b häufig"),
        (Equation::QuasiPermanent, "6.16b", "cd_qp", "Eq. 6.16b quasi-permanent", "Gl. 6.16b quasi-ständig"),
    ] {
        let worst = envelope(&ctx, &actions, equation, false)?.ok_or("no actions")?;
        out.push(limit_or_info(key, en, de, worst.total(), c.opt(limit_key), worst.describe()));
    }
    Ok(out)
}

fn e90_23(c: &Case) -> R {
    c.s("structure")?;
    let domain = c.domain()?;
    let mut out = Vec::new();
    for (set, key, title) in [(FactorSet::EquA, "equ", "EQU"), (FactorSet::StrB, "str", "STR"), (FactorSet::GeoC, "geo", "GEO")] {
        let (sup, inf) = gamma_g(c.annex, domain, set);
        out.push(info(&format!("{key}-g-sup"), &format!("{title} γG,sup"), &format!("{title} γG,sup"), sup, format!("Table {}", set.label(domain))));
        out.push(info(&format!("{key}-g-inf"), &format!("{title} γG,inf"), &format!("{title} γG,inf"), inf, format!("Table {}", set.label(domain))));
        let q = gamma_q(c.annex, domain, set, c.so("category", "office"));
        out.push(info(&format!("{key}-q"), &format!("{title} γQ"), &format!("{title} γQ"), q, format!("Table {}", set.label(domain))));
    }
    out.push(info("xi", "ξ (Eq. 6.10b)", "ξ (Gl. 6.10b)", xi(c.annex, domain), "NA-DE offers Eq. 6.10 only".into()));
    Ok(out)
}

pub const COMBINATION_ENTRIES: &[Entry] = &[
    Entry { id: "E90.15", part: "§6.4.2", clause: "6.5-6.8", en: "Equilibrium EQU", de: "Lagesicherheit EQU", inputs: &["actions", "structure", "rc", "t_d", "altitude"], eval: e90_15 },
    Entry { id: "E90.16", part: "§6.4.3.2", clause: "6.10", en: "Fundamental combination STR/GEO", de: "Grundkombination STR/GEO", inputs: &["actions", "structure", "rc", "set", "r_d", "altitude"], eval: e90_16 },
    Entry { id: "E90.17", part: "§6.4.3.3", clause: "6.11b", en: "Accidental combination", de: "Außergewöhnliche Kombination", inputs: &["actions", "structure", "rc", "lead_psi", "r_d", "altitude"], eval: e90_17 },
    Entry { id: "E90.18", part: "§6.4.3.4", clause: "6.12b", en: "Seismic combination", de: "Erdbebenkombination", inputs: &["actions", "structure", "rc", "r_d", "altitude"], eval: e90_18 },
    Entry { id: "E90.19", part: "§6.5.3", clause: "6.14b-6.16b", en: "Serviceability combinations", de: "Gebrauchstauglichkeitskombinationen", inputs: &["actions", "structure", "altitude", "cd_char", "cd_freq", "cd_qp"], eval: e90_19 },
    Entry { id: "E90.23", part: "Annex A1/A2", clause: "A1.2, A2.4", en: "Partial factor sets", de: "Teilsicherheitsbeiwertsätze", inputs: &["structure", "category"], eval: e90_23 },
];
//#endregion 🔖️CombinationEntries

//#region 🔖️AnnexEntries
fn psi_outcomes(c: &Case, prefix: &str, table: &str) -> R {
    let category = resolve_psi_category(c.annex, c.s("category")?, c.o("altitude", 0.0));
    let entry = psi_entry_of(&category).ok_or_else(|| format!("unknown ψ category {category}"))?;
    if !entry.table.starts_with(table) {
        return Err(format!("category {category} belongs to Table {}, not Annex {prefix}", entry.table));
    }
    let row = entry.row(c.annex);
    Ok(vec![
        info("psi0", "ψ0", "ψ0", row.psi_0, format!("{} Table {}", entry.label_en, entry.table)),
        info("psi1", "ψ1", "ψ1", row.psi_1, format!("{} Table {}", entry.label_en, entry.table)),
        info("psi2", "ψ2", "ψ2", row.psi_2, format!("{} Table {}", entry.label_en, entry.table)),
    ])
}

fn e90_22(c: &Case) -> R {
    psi_outcomes(c, "A1", "A1")
}

fn e90_25(c: &Case) -> R {
    psi_outcomes(c, "A2", "A2")
}

fn e90_24(c: &Case) -> R {
    positive(c.n("span")?, "span")?;
    for key in ["ratio_net", "ratio_w3", "drift_ratio", "storey_ratio"] {
        positive(c.o(key, 1.0), key)?;
    }
    let span = c.n("span")? * if c.o("cantilever", 0.0) > 0.5 { 2.0 } else { 1.0 };
    let (wc, w1, w2, w3) = (c.o("w_c", 0.0), c.o("w1", 0.0), c.o("w2", 0.0), c.o("w3", 0.0));
    let w_tot = w1 + w2 + w3;
    let w_net = w_tot - wc;
    let ratio_net = c.o("ratio_net", ndp(c.annex, "ndp.en1990.sls.w_net.ratio"));
    let ratio_w3 = c.o("ratio_w3", ndp(c.annex, "ndp.en1990.sls.w3.ratio"));
    let mut out = vec![
        info("w-tot", "w_tot = w1 + w2 + w3 [mm]", "w_tot = w1 + w2 + w3 [mm]", w_tot, format!("{} + {} + {}", f(w1), f(w2), f(w3))),
        Outcome::limit("w-net-fin", "w_net,fin ≤ L/ratio [mm]", "w_net,fin ≤ L/Faktor [mm]", LEN, w_net, span * 1000.0 / ratio_net, format!("w_net,fin = w_tot − w_c = {}; L = {} m, L/{}", f(w_net), f(span), f(ratio_net))),
        Outcome::limit("w-3", "w3 ≤ L/ratio [mm]", "w3 ≤ L/Faktor [mm]", LEN, w3, span * 1000.0 / ratio_w3, format!("w3 = {}; L/{}", f(w3), f(ratio_w3))),
    ];
    if let Some(f1) = c.opt("f1") {
        let f_min = c.o("f_min", ndp(c.annex, "ndp.en1990.sls.floor_frequency_hz"));
        out.push(Outcome::minimum("vibration", "Natural frequency f1 ≥ f_min [Hz]", "Eigenfrequenz f1 ≥ f_min [Hz]", DIM, f1, f_min, format!("f1 = {} Hz vs {} Hz", f(f1), f(f_min))));
    }
    if let (Some(u), Some(height)) = (c.opt("u"), c.opt("height")) {
        let ratio = c.o("drift_ratio", ndp(c.annex, "ndp.en1990.sls.frame_drift.ratio"));
        out.push(Outcome::limit("drift", "Horizontal displacement u ≤ H/ratio [mm]", "Horizontalverschiebung u ≤ H/Faktor [mm]", LEN, u, height * 1000.0 / ratio, format!("u = {} mm, H/{} = {} mm", f(u), f(ratio), f(height * 1000.0 / ratio))));
    }
    if let (Some(drift), Some(storey)) = (c.opt("u_storey"), c.opt("h_storey")) {
        let ratio = c.o("storey_ratio", ndp(c.annex, "ndp.en1990.sls.storey_drift.ratio"));
        out.push(Outcome::limit("storey-drift", "Storey drift u_i ≤ h_i/ratio [mm]", "Stockwerksverschiebung u_i ≤ h_i/Faktor [mm]", LEN, drift, storey * 1000.0 / ratio, format!("u_i = {} mm, h_i/{} = {} mm", f(drift), f(ratio), f(storey * 1000.0 / ratio))));
    }
    Ok(out)
}

fn e90_26(c: &Case) -> R {
    let actions = c.actions()?;
    let r_d = resistance(c);
    let mut out = Vec::new();
    for (set, key, title) in [(FactorSet::StrB, "str", "STR"), (FactorSet::GeoC, "geo", "GEO set C")] {
        let mut ctx = c.context(set)?;
        ctx.domain = Domain::Bridge;
        let combos = combinations(&ctx, &actions, Equation::E610)?;
        let worst = governing(&combos, false).ok_or("no actions")?;
        out.push(limit_or_info(&format!("{key}-e610"), &format!("{title} Eq. 6.10 governing"), &format!("{title} Gl. 6.10 maßgebend"), worst.total(), r_d, worst.describe()));
    }
    let mut ctx = c.context(FactorSet::EquA)?;
    ctx.domain = Domain::Bridge;
    let worst = envelope(&ctx, &actions, Equation::E610, true)?.ok_or("no actions")?;
    out.push(info("equ-net", "EQU net destabilising effect", "EQU resultierende destabilisierende Wirkung", worst.net(), worst.describe()));
    let mut groups: Vec<&str> = actions.iter().map(|a| a.group.as_str()).filter(|g| !g.is_empty()).collect();
    groups.sort_unstable();
    groups.dedup();
    out.push(info("exclusive-groups", "Mutually exclusive traffic groups", "Sich ausschließende Verkehrsgruppen", groups.len() as f64, groups.join(",")));
    Ok(out)
}

fn e90_27(c: &Case) -> R {
    let mut out = Vec::new();
    if let Some(a) = c.opt("acc_v") {
        let limit = c.o("acc_v_limit", ndp(c.annex, "ndp.en1990.a2.footbridge.acc_v"));
        out.push(Outcome::limit("acc-v", "Vertical acceleration [m/s²]", "Vertikale Beschleunigung [m/s²]", QuantityKind::Acceleration, a, limit, format!("a_v = {} vs {}", f(a), f(limit))));
    }
    if let Some(a) = c.opt("acc_h") {
        let limit = c.o("acc_h_limit", if c.o("crowd", 0.0) > 0.5 { ndp(c.annex, "ndp.en1990.a2.footbridge.acc_h_crowd") } else { ndp(c.annex, "ndp.en1990.a2.footbridge.acc_h") });
        out.push(Outcome::limit("acc-h", "Horizontal acceleration [m/s²]", "Horizontale Beschleunigung [m/s²]", QuantityKind::Acceleration, a, limit, format!("a_h = {} vs {}", f(a), f(limit))));
        out.push(info("lock-in-trigger", "Lock-in trigger acceleration [m/s²]", "Lock-in-Auslöser [m/s²]", ndp(c.annex, "ndp.en1990.a2.footbridge.lock_in"), "horizontal lateral synchronisation".into()).steps_de("horizontale seitliche Synchronisation"));
    }
    if let Some(a) = c.opt("deck_acc") {
        let limit = if c.so("track", "ballasted") == "slab" { ndp(c.annex, "ndp.en1990.a2.rail.acc_slab") } else { ndp(c.annex, "ndp.en1990.a2.rail.acc_ballast") };
        out.push(Outcome::limit("deck-acc", "Rail deck acceleration [m/s²]", "Gleisträger-Beschleunigung [m/s²]", QuantityKind::Acceleration, a, limit, format!("a = {} vs {}", f(a), f(limit))));
    }
    if let (Some(t), Some(v)) = (c.opt("twist"), c.opt("v_kmh")) {
        let limit = if v <= 120.0 { ndp(c.annex, "ndp.en1990.a2.rail.twist_120") } else if v <= 200.0 { ndp(c.annex, "ndp.en1990.a2.rail.twist_200") } else { ndp(c.annex, "ndp.en1990.a2.rail.twist_max") };
        out.push(Outcome::limit("twist", "Deck twist per 3 m [mm]", "Verwindung je 3 m [mm]", LEN, t, limit, format!("t = {} mm/3 m vs {} (v = {} km/h)", f(t), f(limit), f(v))));
    }
    if let (Some(delta), Some(span)) = (c.opt("deflection"), c.opt("span")) {
        let ratio = c.o("deflection_ratio", ndp(c.annex, "ndp.en1990.a2.rail.deflection.ratio"));
        out.push(Outcome::limit("deflection", "Vertical deflection δ ≤ L/ratio [mm]", "Vertikale Durchbiegung δ ≤ L/Faktor [mm]", LEN, delta, span * 1000.0 / ratio, format!("δ = {} mm, L/{} = {} mm", f(delta), f(ratio), f(span * 1000.0 / ratio))));
    }
    if out.is_empty() {
        return Err("give at least one of acc_v, acc_h, deck_acc, twist with v_kmh, deflection with span".into());
    }
    Ok(out)
}

pub const ANNEX_ENTRIES: &[Entry] = &[
    Entry { id: "E90.22", part: "Annex A1", clause: "Table A1.1", en: "ψ factors for buildings", de: "ψ-Beiwerte Hochbau", inputs: &["category", "altitude"], eval: e90_22 },
    Entry { id: "E90.24", part: "Annex A1", clause: "A1.4", en: "Serviceability of buildings", de: "Gebrauchstauglichkeit Hochbau", inputs: &["span", "cantilever", "w_c", "w1", "w2", "w3", "ratio_net", "ratio_w3", "f1", "f_min", "u", "height", "drift_ratio", "u_storey", "h_storey", "storey_ratio"], eval: e90_24 },
    Entry { id: "E90.25", part: "Annex A2", clause: "Tables A2.1-A2.3", en: "ψ factors for bridges", de: "ψ-Beiwerte Brücken", inputs: &["category"], eval: e90_25 },
    Entry { id: "E90.26", part: "Annex A2", clause: "Table A2.4", en: "Partial factors and combinations for bridges", de: "Teilsicherheitsbeiwerte und Kombinationen Brücken", inputs: &["actions", "rc", "r_d"], eval: e90_26 },
    Entry { id: "E90.27", part: "Annex A2", clause: "A2.4.3-A2.4.4", en: "Serviceability of bridges", de: "Gebrauchstauglichkeit Brücken", inputs: &["acc_v", "acc_h", "crowd", "deck_acc", "track", "twist", "v_kmh", "deflection", "span", "deflection_ratio"], eval: e90_27 },
];
//#endregion 🔖️AnnexEntries

//#region 🔖️ReliabilityEntries
fn e90_28(c: &Case) -> R {
    let rc = c.n("rc")? as u8;
    let period = c.o("t_years", 50.0);
    let target = target_reliability_index_for(rc, period).ok_or("rc must be 1..3 and t_years positive")?;
    let mut out = vec![
        info("beta-target", "Target reliability index β", "Zielwert des Zuverlässigkeitsindex β", target, format!("RC{rc}, T = {} a", f(period))),
        info("p-f", "Failure probability Φ(−β)", "Versagenswahrscheinlichkeit Φ(−β)", phi_cdf(-target), format!("β = {}", f(target))),
    ];
    if let Some(b1) = target_reliability_index_for(rc, 1.0) {
        let p50 = -(50.0 * (-phi_cdf(-b1)).ln_1p()).exp_m1();
        out.push(info("beta-50-from-1", "β(50 a) converted from β(1 a)", "β(50 a) umgerechnet aus β(1 a)", -phi_inv(p50), "p_50 = 1 − (1 − p_1)^50".into()));
    }
    if let Some(beta) = c.opt("beta") {
        out.push(Outcome::minimum("beta", "β ≥ β_target", "β ≥ β_Ziel", DIM, beta, target, format!("β = {} vs {}", f(beta), f(target))));
    }
    Ok(out)
}

fn dist(c: &Case, prefix: &str) -> Result<Dist, String> {
    Dist::parse(c.s(&format!("{prefix}_dist"))?, c.n(&format!("{prefix}_mean"))?, c.n(&format!("{prefix}_sd"))?)
}

fn e90_29_multi(c: &Case, r: Dist, e: Dist) -> R {
    let (e2, beta_t) = (dist(c, "e2")?, c.o("beta_t", 3.8));
    let form = form_multi(&r, &[e, e2]);
    if !form.converged || !form.beta.is_finite() {
        return Err(format!("FORM did not converge in {} iterations (indeterminate, no verdict)", form.iterations));
    }
    let formula = "Rackwitz-Fiessler normal tail, g = R − E₁ − E₂".to_string();
    let mut out = vec![
        info("beta", "FORM reliability index β", "FORM-Zuverlässigkeitsindex β", form.beta, format!("{} iterations", form.iterations)),
        info("alpha-r", "α_R", "α_R", form.alpha_r, formula.clone()),
        info("alpha-e", "α_E1", "α_E1", form.alpha_e[0], formula.clone()),
        info("alpha-e2", "α_E2", "α_E2", form.alpha_e[1], formula),
        info("r-d", "Design point R_d", "Bemessungspunkt R_d", form.design_r, "X_d at the FORM design point".into()).steps_de("X_d am FORM-Bemessungspunkt"),
        info("e-d", "Design point E1_d", "Bemessungspunkt E1_d", form.design_e[0], "X_d at the FORM design point".into()).steps_de("X_d am FORM-Bemessungspunkt"),
        info("e2-d", "Design point E2_d", "Bemessungspunkt E2_d", form.design_e[1], "X_d at the FORM design point".into()).steps_de("X_d am FORM-Bemessungspunkt"),
        info("p-f", "Failure probability Φ(−β)", "Versagenswahrscheinlichkeit Φ(−β)", form.failure_probability(), "P_f = Φ(−β)".into()),
        Outcome::minimum("beta-target", "β ≥ β_t", "β ≥ β_t", DIM, form.beta, beta_t, format!("β = {} vs β_t = {}", f(form.beta), f(beta_t))),
    ];
    let (lead, other) = if form.alpha_e[0].abs() >= form.alpha_e[1].abs() { (0, 1) } else { (1, 0) };
    let (alpha_r, alpha_lead) = annex_c_alphas(c.annex, [e.sd(), e2.sd()][lead], r.sd());
    let alpha_second = annex_c_secondary_alpha(c.annex, alpha_lead);
    let (means, sds) = ([e.mean(), e2.mean()], [e.sd(), e2.sd()]);
    out.push(info("r-d-fixed", "R_d with fixed α_R (C.7)", "R_d mit festem α_R (C.7)", r.mean() - alpha_r * beta_t * r.sd(), format!("μ_R − α_R β_t σ_R with α_R = {}", f(alpha_r))));
    out.push(info("e-lead-fixed", "Leading action E_d with α_E (C.7)", "Leiteinwirkung E_d mit α_E (C.7)", means[lead] - alpha_lead * beta_t * sds[lead], format!("E{} leads, α_E = {}", lead + 1, f(alpha_lead))));
    out.push(info("e-second-fixed", "Secondary action E_d with 0.4 α_E (C.7)", "Begleiteinwirkung E_d mit 0,4 α_E (C.7)", means[other] - alpha_second * beta_t * sds[other], format!("E{} accompanies, α_E = {}", other + 1, f(alpha_second))));
    for (key, label, k, design) in [("gamma-r", "γ_R = R_k / R_d", c.opt("r_k"), form.design_r), ("gamma-e", "γ_E1 = E1_d / E1_k", c.opt("e_k"), form.design_e[0]), ("gamma-e2", "γ_E2 = E2_d / E2_k", c.opt("e2_k"), form.design_e[1])] {
        if let Some(k) = k {
            out.push(info(key, label, label, if key == "gamma-r" { k / design } else { design / k }, format!("characteristic value {}", f(k))));
        }
    }
    Ok(out)
}

fn e90_29(c: &Case) -> R {
    let (r, e) = (dist(c, "r")?, dist(c, "e")?);
    if c.text.contains_key("e2_dist") {
        return e90_29_multi(c, r, e);
    }
    let beta_t = c.o("beta_t", 3.8);
    let form = form_r_minus_e(&r, &e);
    if !form.converged || !form.beta.is_finite() {
        return Err(format!("FORM did not converge in {} iterations (indeterminate, no verdict)", form.iterations));
    }
    let (alpha_r, alpha_e) = annex_c_alphas(c.annex, e.sd(), r.sd());
    let r_fixed = r.mean() - alpha_r * beta_t * r.sd();
    let e_fixed = e.mean() - alpha_e * beta_t * e.sd();
    let mut out = vec![
        info("beta", "FORM reliability index β", "FORM-Zuverlässigkeitsindex β", form.beta, format!("{} iterations, converged = {}", form.iterations, form.converged)),
        info("alpha-r", "α_R", "α_R", form.alpha_r, "Rackwitz-Fiessler normal tail".into()).steps_de("Rackwitz-Fießler-Normalverteilungsausläufer"),
        info("alpha-e", "α_E", "α_E", form.alpha_e, "Rackwitz-Fiessler normal tail".into()).steps_de("Rackwitz-Fießler-Normalverteilungsausläufer"),
        info("r-d", "Design point R_d", "Bemessungspunkt R_d", form.design_r, "X_d at the FORM design point".into()).steps_de("X_d am FORM-Bemessungspunkt"),
        info("e-d", "Design point E_d", "Bemessungspunkt E_d", form.design_e, "X_d at the FORM design point".into()).steps_de("X_d am FORM-Bemessungspunkt"),
        info("p-f", "Failure probability Φ(−β)", "Versagenswahrscheinlichkeit Φ(−β)", form.failure_probability(), "P_f = Φ(−β)".into()),
        info("r-d-fixed", "R_d with fixed α_R (C.7)", "R_d mit festem α_R (C.7)", r_fixed, format!("μ_R − α_R β_t σ_R with α_R = {}", f(alpha_r))),
        info("e-d-fixed", "E_d with fixed α_E (C.7)", "E_d mit festem α_E (C.7)", e_fixed, format!("μ_E − α_E β_t σ_E with α_E = {}", f(alpha_e))),
        Outcome::minimum("beta-target", "β ≥ β_t", "β ≥ β_t", DIM, form.beta, beta_t, format!("β = {} vs β_t = {}", f(form.beta), f(beta_t))),
    ];
    if let Some(r_k) = c.opt("r_k") {
        out.push(info("gamma-r", "γ_R = R_k / R_d", "γ_R = R_k / R_d", r_k / form.design_r, format!("R_k = {}", f(r_k))));
    }
    if let Some(e_k) = c.opt("e_k") {
        out.push(info("gamma-e", "γ_E = E_d / E_k", "γ_E = E_d / E_k", form.design_e / e_k, format!("E_k = {}", f(e_k))));
    }
    Ok(out)
}

fn e90_30_standard(c: &Case, pairs: &str) -> R {
    let results = parse_pairs(pairs)?;
    let (eta_d, gamma_m) = (c.o("eta_d", 1.0), c.o("gamma_m", 1.0));
    let fit = standard_evaluation(&results, c.o("v_rt", 0.0), c.n("g_rt_mean")?)?;
    let steps = format!("n = {}, b = Σ r_e r_t / Σ r_t²", results.len());
    Ok(vec![
        info("b", "Mean correction b (D.7.2)", "Mittlerer Korrekturfaktor b (D.7.2)", fit.b, steps),
        info("v-delta", "Error scatter V_δ", "Streuung des Modellfehlers V_δ", fit.v_delta, "V_δ² = exp(s²(ln δ)) − 1".into()),
        info("v-r", "Resistance scatter V_r", "Streuung des Widerstands V_r", fit.v_r, "V_r² = (V_δ² + 1)(V_rt² + 1) − 1".into()),
        info("k-n", "k_n (Table D1, V unknown)", "k_n (Tabelle D1, V unbekannt)", fit.k_n, format!("n = {}", results.len())),
        info("r-k", "Characteristic resistance r_k", "Charakteristischer Widerstand r_k", fit.r_k, format!("b g_rt(X_m) exp(−k_∞ α_rt Q_rt − k_n α_δ Q_δ − Q²/2), k_∞ = {}", f(fit.k_infinity))),
        info("r-d", "Design resistance r_d = η_d r_k / γ_M", "Bemessungswiderstand r_d = η_d r_k / γ_M", eta_d * fit.r_k / gamma_m, format!("{} · {} / {}", f(eta_d), f(fit.r_k), f(gamma_m))),
        Outcome::advisory("sample-size", "V_δ estimated from at least 6 results", "V_δ aus mindestens 6 Versuchen geschätzt", DIM, results.len() as f64, 6.0, format!("n = {}", results.len())),
    ])
}

fn e90_30(c: &Case) -> R {
    if let Some(pairs) = c.text.get("pairs") {
        return e90_30_standard(c, pairs);
    }
    let n = c.n("n")?;
    if n.fract() != 0.0 {
        return Err(format!("n must be a whole number of tests, got {}", f(n)));
    }
    let (mean, eta_d, gamma_m) = (c.n("mean")?, c.o("eta_d", 1.0), c.o("gamma_m", 1.0));
    let v_known = c.o("v_known", 0.0) > 0.5;
    let v = c.n("v")?;
    let kn = k_n(n, v_known).ok_or("n must be at least 1 (V known) or 3 (V unknown)")?;
    let lognormal = c.so("dist", "normal") == "lognormal";
    let x_k = if lognormal {
        let s_y = (1.0 + v * v).ln().sqrt();
        (mean.ln() - 0.5 * s_y * s_y - kn * s_y).exp()
    } else {
        mean * (1.0 - kn * v)
    };
    let x_d = eta_d * x_k / gamma_m;
    let mut out = vec![
        info("k-n", "k_n (Table D1)", "k_n (Tabelle D1)", kn, format!("n = {}, V {}", f(n), if v_known { "known" } else { "unknown" })),
        info("x-k", "Characteristic value X_k", "Charakteristischer Wert X_k", x_k, if lognormal { "exp(m_y − k_n s_y)".into() } else { "m_X (1 − k_n V_X)".into() }),
        info("x-d", "Design value X_d = η_d X_k / γ_m", "Bemessungswert X_d = η_d X_k / γ_m", x_d, format!("{} · {} / {}", f(eta_d), f(x_k), f(gamma_m))),
    ];
    if v_known {
        let kd = k_d_n(n, true).ok_or("n must be at least 1 for Table D2")?;
        let x_direct = eta_d * if lognormal {
            let s_y = (1.0 + v * v).ln().sqrt();
            (mean.ln() - 0.5 * s_y * s_y - kd * s_y).exp()
        } else {
            mean * (1.0 - kd * v)
        };
        out.push(info("k-d-n", "k_d,n (Table D2, V known)", "k_d,n (Tabelle D2, V bekannt)", kd, format!("n = {}", f(n))));
        out.push(info("x-d-direct", "Design value X_d = η_d m (1 − k_d,n V)", "Bemessungswert X_d = η_d m (1 − k_d,n V)", x_direct, format!("{} · {} (1 − {} · {})", f(eta_d), f(mean), f(kd), f(v))));
    }
    if n < 3.0 {
        out.push(Outcome::advisory("few-tests", "At least 3 tests", "Mindestens 3 Versuche", DIM, n, 3.0, format!("n = {}", f(n))));
    }
    if !v_known {
        out.push(Outcome::advisory("sample-size", "V estimated from at least 6 samples", "V aus mindestens 6 Proben geschätzt", DIM, n, 6.0, format!("n = {}", f(n))));
    }
    Ok(out)
}

pub const RELIABILITY_ENTRIES: &[Entry] = &[
    Entry { id: "E90.28", part: "Annex B", clause: "Table B2", en: "Reliability differentiation", de: "Zuverlässigkeitsdifferenzierung", inputs: &["rc", "t_years", "beta"], eval: e90_28 },
    Entry { id: "E90.29", part: "Annex C", clause: "C.5-C.7", en: "Reliability analysis (FORM)", de: "Zuverlässigkeitsanalyse (FORM)", inputs: &["r_dist", "r_mean", "r_sd", "e_dist", "e_mean", "e_sd", "beta_t", "r_k", "e_k", "e2_dist", "e2_mean", "e2_sd", "e2_k"], eval: e90_29 },
    Entry { id: "E90.30", part: "Annex D", clause: "D7", en: "Design assisted by testing", de: "Versuchsgestützte Bemessung", inputs: &["n", "mean", "v", "v_known", "dist", "eta_d", "gamma_m", "pairs", "v_rt", "g_rt_mean"], eval: e90_30 },
];
//#endregion 🔖️ReliabilityEntries

#[path = "📚️inputs/🦀️.rs"]
pub mod case_inputs;

#[path = "🗂️cases/🦀️.rs"]
mod cases;
pub use cases::*;

#[cfg(test)]
#[path = "🧪️tests/🗣️german-reasons/🦀️.rs"]
mod german_reasons;
