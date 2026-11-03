//! 📜️ National annex selection and table lookups of the norm compliance core. See [`super`].

use super::*;

pub use semio_s_plugin_norm_ndp::*;

#[path = "⚖️action-factors/🦀️.rs"]
pub mod action_factors;
pub use action_factors::{ActionFactors, ReliabilityClass};

/// 🗺️ Annex-dependent value of an NDP entry.
pub trait NdpEntryAnnex {
    fn value(&self, annex: AnnexChoice) -> NdpValue;

    fn verification(&self, annex: AnnexChoice) -> NdpStatus;
}

impl NdpEntryAnnex for NdpEntry {
    fn value(&self, annex: AnnexChoice) -> NdpValue {
        match annex {
            AnnexChoice::En => self.recommended,
            AnnexChoice::De => self.na_de,
        }
    }

    fn verification(&self, annex: AnnexChoice) -> NdpStatus {
        match annex {
            AnnexChoice::En => self.status_en,
            AnnexChoice::De => self.status_de,
        }
    }
}

/// ✅️ Whether the value of the annex is double-keyed from the print; unknown ids count as unverified.
pub fn ndp_is_verified_for(id: &str, annex: AnnexChoice) -> bool {
    ndp(id).is_some_and(|entry| entry.verification(annex) == NdpStatus::Verified)
}

thread_local! {
    static NDP_READS: std::cell::RefCell<Vec<&'static str>> = const { std::cell::RefCell::new(Vec::new()) };
}

/// 📝️ Logs a registry read on this thread so the check that rests on it can cite it; the log holds each id once.
fn log_read(entry: &'static NdpEntry) {
    NDP_READS.with(|reads| {
        let mut list = reads.borrow_mut();
        if !list.contains(&entry.id) {
            list.push(entry.id);
        }
    });
}

/// 🕳️ Why a registry read cannot yield a number: the id is unknown, the annex has no value, the value is table or formula text, or a read without annex choice meets different EN and DE values.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NdpGapReason {
    Unregistered,
    NoValue,
    NotNumeric,
    Divergent,
}

/// 🕳️ A registry read that cannot yield a number; the check resting on it is refused with [`NdpGap::notice`] (see [`CheckBuilder::build`]) and never answered with a default.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NdpGap {
    pub id: String,
    pub annex: AnnexChoice,
    pub reason: NdpGapReason,
}

impl NdpGap {
    /// 🗣️ Localized refusal text, for example `NDP ndp.en1992.vrdcc-k has no value for annex EN`.
    pub fn notice(&self) -> LocalizedCopy {
        let (id, annex) = (&self.id, self.annex.label());
        match self.reason {
            NdpGapReason::Unregistered => LocalizedCopy::new(format!("NDP {id} is not registered"), format!("NDP {id} ist nicht registriert")),
            NdpGapReason::NoValue => LocalizedCopy::new(format!("NDP {id} has no value for annex {annex}"), format!("NDP {id} hat keinen Wert für den Anhang {annex}")),
            NdpGapReason::NotNumeric => LocalizedCopy::new(format!("NDP {id} is not a number for annex {annex}"), format!("NDP {id} ist für den Anhang {annex} keine Zahl")),
            NdpGapReason::Divergent => LocalizedCopy::new(format!("NDP {id} differs between the annexes and needs the annex choice"), format!("NDP {id} weicht zwischen den Anhängen ab und braucht die Anhangswahl")),
        }
    }
}

impl std::fmt::Display for NdpGap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.notice().en)
    }
}

impl std::error::Error for NdpGap {}

thread_local! {
    static NDP_GAPS: std::cell::RefCell<Vec<NdpGap>> = const { std::cell::RefCell::new(Vec::new()) };
}

fn log_gap(gap: NdpGap) {
    NDP_GAPS.with(|gaps| {
        let mut list = gaps.borrow_mut();
        if !list.contains(&gap) {
            list.push(gap);
        }
    });
}

/// 🕳️ Gaps of the reads through [`ndp_number`] and [`ndp_common`] on this thread since the last drain; [`CheckBuilder::build`] drains them and refuses the check.
pub fn drain_ndp_gaps() -> Vec<NdpGap> {
    NDP_GAPS.with(|gaps| std::mem::take(&mut *gaps.borrow_mut()))
}

/// 🔢️ Number of an NDP for the annex or the [`NdpGap`] that says why there is none; the read is logged for [`drain_ndp_reads`].
pub fn ndp_value(id: &str, annex: AnnexChoice) -> Result<f64, NdpGap> {
    let gap = |reason| NdpGap { id: id.to_owned(), annex, reason };
    let entry = ndp(id).ok_or_else(|| gap(NdpGapReason::Unregistered))?;
    log_read(entry);
    match entry.value(annex) {
        NdpValue::Number(value) => Ok(value),
        NdpValue::Unset => Err(gap(NdpGapReason::NoValue)),
        NdpValue::Text(_) => Err(gap(NdpGapReason::NotNumeric)),
    }
}

/// 🔢️ Number of an NDP for the annex, `None` when the id is unknown or has no number; the read is logged for [`drain_ndp_reads`] and the caller owns the absence.
pub fn ndp_number_opt(id: &str, annex: AnnexChoice) -> Option<f64> {
    ndp_value(id, annex).ok()
}

/// 🔢️ Number of an NDP for the annex or `None`; unlike [`ndp_number_opt`] the absence is logged as an [`NdpGap`], for callers that hand it on as NaN.
pub fn ndp_number_or_gap(id: &str, annex: AnnexChoice) -> Option<f64> {
    ndp_value(id, annex).map_err(log_gap).ok()
}

/// 🔢️ Number of an NDP for the annex; a read without number never panics and never defaults: it logs the [`NdpGap`], returns NaN and the check that rests on it ends as a localized refusal (see [`drain_ndp_gaps`]).
pub fn ndp_number(id: &str, annex: AnnexChoice) -> f64 {
    ndp_value(id, annex).unwrap_or_else(|gap| {
        log_gap(gap);
        f64::NAN
    })
}

/// 🤝️ Number of an NDP whose EN-recommended and NA-DE values agree, for code paths without an annex choice; a missing or divergent value logs the [`NdpGap`] and returns NaN like [`ndp_number`]. The registry test `ndp-registry` proves that every id read this way has equal EN and DE numbers.
pub fn ndp_common(id: &str) -> f64 {
    match (ndp_value(id, AnnexChoice::En), ndp_value(id, AnnexChoice::De)) {
        (Ok(en), Ok(de)) if en == de => en,
        (Ok(_), Ok(_)) => {
            log_gap(NdpGap { id: id.to_owned(), annex: AnnexChoice::En, reason: NdpGapReason::Divergent });
            f64::NAN
        }
        (en, de) => {
            en.err().into_iter().chain(de.err()).for_each(log_gap);
            f64::NAN
        }
    }
}

/// 🗂️ Cell `table.key1.key2…` of a typed registry table for the annex; row and column keys are the id segments.
pub fn ndp_cell(table: &str, keys: &[&str], annex: AnnexChoice) -> f64 {
    ndp_number(&format!("{table}.{}", keys.join(".")), annex)
}

/// 🗂️ Cell of a typed registry table, `None` when the row is not registered or not numeric.
pub fn ndp_cell_opt(table: &str, keys: &[&str], annex: AnnexChoice) -> Option<f64> {
    ndp_number_opt(&format!("{table}.{}", keys.join(".")), annex)
}

/// 🗂️ Cell of a typed registry table whose EN and DE values agree.
pub fn ndp_cell_common(table: &str, keys: &[&str]) -> f64 {
    ndp_common(&format!("{table}.{}", keys.join(".")))
}

/// 📈️ Linear interpolation of the typed table `table.<lead…>.<label>` along `axis` (ascending `(label, coordinate)`), clamped to the end rows.
pub fn ndp_interpolate(table: &str, lead: &[&str], axis: &[(&str, f64)], x: f64, annex: AnnexChoice) -> f64 {
    let at = |label: &str| ndp_cell(table, &[lead, &[label]].concat(), annex);
    let x = x.clamp(axis[0].1, axis[axis.len() - 1].1);
    let upper = axis.iter().position(|(_, c)| x <= *c).unwrap_or(axis.len() - 1).max(1.min(axis.len() - 1));
    let (lo, hi) = (axis[upper.saturating_sub(1)], axis[upper]);
    if hi.1 <= lo.1 {
        return at(hi.0);
    }
    let t = (x - lo.1) / (hi.1 - lo.1);
    at(lo.0) + t * (at(hi.0) - at(lo.0))
}

/// 🧾️ Registry ids read through [`ndp_number`] on this thread since the last drain, sorted; build a check right after draining so it cites exactly the parameters its verdict rests on (see [`CheckBuilder::ndp_reads`]).
pub fn drain_ndp_reads() -> Vec<&'static str> {
    let mut ids = NDP_READS.with(|reads| std::mem::take(&mut *reads.borrow_mut()));
    ids.sort_unstable();
    ids
}

/// 🧾️ Runs `run` and returns the registry ids it read; reads logged before the call stay in the log.
pub fn record_ndp_reads<T>(run: impl FnOnce() -> T) -> (T, Vec<&'static str>) {
    let before = NDP_READS.with(|reads| std::mem::take(&mut *reads.borrow_mut()));
    let value = run();
    let read = drain_ndp_reads();
    NDP_READS.with(|reads| *reads.borrow_mut() = before);
    (value, read)
}

/// 🪟️ One annex parameter as the properties window shows it: the registry entry, the value of the active annex, the value of the other annex and whether the two differ.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AnnexParameterRow {
    pub entry: &'static NdpEntry,
    pub active: NdpValue,
    pub other: NdpValue,
    pub differs: bool,
}

/// 🪟️ Every registered parameter whose id starts with `prefix` (for example `ndp.en1993.`) with its value under `annex` and under the other annex, sorted by id; entries without any value are skipped. The inspector lists these rows so a user sees which values the annex choice drives and whether they are verified.
pub fn annex_parameter_rows(prefix: &str, annex: AnnexChoice) -> Vec<AnnexParameterRow> {
    let other = match annex {
        AnnexChoice::En => AnnexChoice::De,
        AnnexChoice::De => AnnexChoice::En,
    };
    let start = NDP_TABLE.partition_point(|entry| entry.id < prefix);
    NDP_TABLE[start..]
        .iter()
        .take_while(|entry| entry.id.starts_with(prefix))
        .filter(|entry| entry.recommended != NdpValue::Unset || entry.na_de != NdpValue::Unset)
        .map(|entry| {
            let (active, other) = (entry.value(annex), entry.value(other));
            AnnexParameterRow { entry, active, other, differs: active != other }
        })
        .collect()
}

// #region 🔖️Annex
/// 🇪️🇺️ National annex selection for Eurocode / DIN EN families.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "compliance-testing"), derive(serde::Serialize, serde::Deserialize))]
pub enum AnnexChoice {
    #[dsl(key = "en")]
    En,
    #[dsl(key = "de")]
    De,
}

impl AnnexChoice {
    pub fn label(self) -> &'static str {
        match self {
            Self::En => "EN",
            Self::De => "DE-NA",
        }
    }
}

/// 🗺️ Trait for national annex parameter overrides.
/// 🔀️ O1 de-dyn: closed set (`NaDe`/`NaEn`) — closes as `NationalAnnexes` via `dyn_enum_close!` in
/// `en1990`'s schema module, where the two impls live (`🎫️tickets/…/📓️terra-dedyn-fleet-norm-report.md`).
#[dyn_enum]
pub trait NationalAnnex {
    fn choice(&self) -> AnnexChoice;
    fn gamma_g(&self) -> f64;
    fn gamma_q(&self) -> f64;
    fn gamma_m(&self, _material: &str) -> f64 {
        1.0
    }
    fn gamma_r(&self) -> f64 {
        1.0
    }
    fn xi(&self, _category: &str) -> f64 {
        1.0
    }
    fn psi_0(&self, category: &str) -> f64;
    fn psi_1(&self, category: &str) -> f64;
    fn psi_2(&self, category: &str) -> f64;
}
// #endregion 🔖️Annex

// #region 🔖️Tables
/// 📊️ One-dimensional table entry for norm lookups.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TableEntry1D {
    pub x: f64,
    pub y: f64,
}

/// 🔍️ Linear interpolation in a sorted 1D table.
pub fn table_lookup_linear(table: &[TableEntry1D], x: f64) -> f64 {
    if table.is_empty() {
        return 0.0;
    }
    if x <= table[0].x {
        return table[0].y;
    }
    if x >= table[table.len() - 1].x {
        return table[table.len() - 1].y;
    }
    for w in table.windows(2) {
        if x >= w[0].x && x <= w[1].x {
            let t = (x - w[0].x) / (w[1].x - w[0].x);
            return w[0].y + t * (w[1].y - w[0].y);
        }
    }
    table[table.len() - 1].y
}

/// 🔍️ Bilinear interpolation on a regular grid.
pub fn table_lookup_bilinear(x: f64, y: f64, x_vals: &[f64], y_vals: &[f64], z: &[f64]) -> f64 {
    let nx = x_vals.len();
    let ny = y_vals.len();
    if nx == 0 || ny == 0 || z.len() < nx * ny {
        return 0.0;
    }
    let xi = x_vals.iter().position(|&v| x <= v).unwrap_or(nx - 1).max(1);
    let yi = y_vals.iter().position(|&v| y <= v).unwrap_or(ny - 1).max(1);
    let x0 = x_vals[xi - 1];
    let x1 = x_vals[xi.min(nx - 1)];
    let y0 = y_vals[yi - 1];
    let y1 = y_vals[yi.min(ny - 1)];
    let tx = if (x1 - x0).abs() < f64::EPSILON { 0.0 } else { ((x - x0) / (x1 - x0)).clamp(0.0, 1.0) };
    let ty = if (y1 - y0).abs() < f64::EPSILON { 0.0 } else { ((y - y0) / (y1 - y0)).clamp(0.0, 1.0) };
    let z00 = z[(yi - 1) * nx + (xi - 1)];
    let z10 = z[(yi - 1) * nx + xi.min(nx - 1)];
    let z01 = z[yi.min(ny - 1) * nx + (xi - 1)];
    let z11 = z[yi.min(ny - 1) * nx + xi.min(nx - 1)];
    let z0 = z00 + tx * (z10 - z00);
    let z1 = z01 + tx * (z11 - z01);
    z0 + ty * (z1 - z0)
}
// #endregion 🔖️Tables
