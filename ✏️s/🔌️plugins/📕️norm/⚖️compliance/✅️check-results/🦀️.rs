//! ✅️ Check results of the norm compliance core: the status, remedies, check builder, part verdicts and the check report. See [`super`].

use super::*;

// #region 🔖️Check
/// ✅️ Outcome of a single norm compliance check.
#[derive(Clone, Copy, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(any(test, feature = "compliance-testing"), derive(serde::Serialize, serde::Deserialize))]
pub enum CheckStatus {
    Pass,
    Warning,
    Fail,
    NotApplicable,
    UnverifiedBasis,
    Info,
}

/// 🧮️ Utilisation reported for a demand against a vanishing or non-finite capacity; keeps wire JSON finite.
pub const UTILIZATION_CAP: f64 = 1.0e6;

/// 🎯️ Relative slack below which a utilisation within `1 + UTILIZATION_TOLERANCE` of its limit counts as exactly on the limit (a pass); the Python oracles use the same value for boundary rows.
pub const UTILIZATION_TOLERANCE: f64 = 1.0e-9;

/// 🎯️ The one shared limit rule: a utilisation within the relative [`UTILIZATION_TOLERANCE`] of 1 is still within its limit.
pub fn within_limit(utilization: f64) -> bool {
    utilization <= 1.0 + UTILIZATION_TOLERANCE
}

/// 🎯️ The shared status of a utilisation against its limit: [`CheckStatus::Pass`] within [`within_limit`], otherwise [`CheckStatus::Fail`].
pub fn limit_status(utilization: f64) -> CheckStatus {
    if within_limit(utilization) { CheckStatus::Pass } else { CheckStatus::Fail }
}

/// 🎚️ Reserve a passing check should keep below its limit before it is flagged as close to it; the warning threshold is `1 - reserve`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NearLimit {
    reserve: f64,
}

impl NearLimit {
    pub const DEFAULT_RESERVE: f64 = 0.05;

    pub fn with_reserve(reserve: f64) -> Self {
        Self { reserve: if reserve.is_finite() { reserve.clamp(0.0, 1.0) } else { Self::DEFAULT_RESERVE } }
    }

    pub fn reserve(self) -> f64 {
        self.reserve
    }

    pub fn threshold(self) -> f64 {
        1.0 - self.reserve
    }
}

impl Default for NearLimit {
    fn default() -> Self {
        Self::with_reserve(Self::DEFAULT_RESERVE)
    }
}

static NEAR_LIMIT_RESERVE_BITS: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(NearLimit::DEFAULT_RESERVE.to_bits());

/// 🎚️ Sets the process-wide near-limit reserve used by [`CheckBuilder::warn_near_limit`].
pub fn set_near_limit(near_limit: NearLimit) {
    NEAR_LIMIT_RESERVE_BITS.store(near_limit.reserve.to_bits(), std::sync::atomic::Ordering::Relaxed);
}

/// 🎚️ The process-wide near-limit reserve, [`NearLimit::DEFAULT_RESERVE`] until configured.
pub fn near_limit() -> NearLimit {
    NearLimit::with_reserve(f64::from_bits(NEAR_LIMIT_RESERVE_BITS.load(std::sync::atomic::Ordering::Relaxed)))
}

fn ratio(demand: f64, capacity: f64) -> f64 {
    if !demand.is_finite() || !capacity.is_finite() {
        UTILIZATION_CAP
    } else if capacity < f64::EPSILON {
        if demand < f64::EPSILON { 0.0 } else { UTILIZATION_CAP }
    } else {
        (demand / capacity).min(UTILIZATION_CAP)
    }
}

/// 🌐️ Mandatory bilingual copy for norm report text (en + de; no default language).
#[derive(Clone, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(any(test, feature = "compliance-testing"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "compliance-testing"), serde(rename_all = "camelCase"))]
pub struct LocalizedCopy {
    pub en: String,
    pub de: String,
}

impl LocalizedCopy {
    pub fn new(en: impl Into<String>, de: impl Into<String>) -> Self {
        Self { en: en.into(), de: de.into() }
    }

    /// 🗣️ Exact locale match against the framework [`semio_framework_ui_locale::Locale`] axis.
    pub fn resolve(&self, locale: &semio_framework_ui_locale::Locale) -> &str {
        match locale {
            semio_framework_ui_locale::Locale::En => &self.en,
            semio_framework_ui_locale::Locale::De => &self.de,
        }
    }
}

/// 🎯 Stable pointer to the subject entity a check concerns.
#[derive(Clone, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(any(test, feature = "compliance-testing"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "compliance-testing"), serde(rename_all = "camelCase"))]
pub struct SubjectRef {
    pub entity_id: String,
    pub path: String,
    pub label: LocalizedCopy,
}

impl SubjectRef {
    pub fn new(entity_id: impl Into<String>, path: impl Into<String>, label: LocalizedCopy) -> Self {
        Self { entity_id: entity_id.into(), path: path.into(), label }
    }

    pub fn whole(label: LocalizedCopy) -> Self {
        Self::new("", "", label)
    }
}

/// 📐️ How a remedy's required value relates to the subject field.
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(any(test, feature = "compliance-testing"), derive(serde::Serialize, serde::Deserialize))]
pub enum RemedyBound {
    AtLeast,
    AtMost,
    Exactly,
    OneOf,
    Remove,
    Insert,
}

/// 🩹️ Concrete remediation that makes a failing (or warning) check comply.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(any(test, feature = "compliance-testing"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "compliance-testing"), serde(rename_all = "camelCase"))]
pub struct Remedy {
    pub target: SubjectRef,
    pub current: Quantity,
    pub required: Quantity,
    pub bound: RemedyBound,
    pub options: Vec<String>,
    pub action: LocalizedCopy,
    pub applicable: bool,
}

impl Remedy {
    pub fn at_least(target: SubjectRef, current: Quantity, required: Quantity, action: LocalizedCopy) -> Self {
        Self { target, current, required, bound: RemedyBound::AtLeast, options: Vec::new(), action, applicable: true }
    }

    pub fn at_most(target: SubjectRef, current: Quantity, required: Quantity, action: LocalizedCopy) -> Self {
        Self { target, current, required, bound: RemedyBound::AtMost, options: Vec::new(), action, applicable: true }
    }

    pub fn exactly(target: SubjectRef, current: Quantity, required: Quantity, action: LocalizedCopy) -> Self {
        Self { target, current, required, bound: RemedyBound::Exactly, options: Vec::new(), action, applicable: true }
    }

    /// ➖ Removes the list row `target.path` selects (`factorOverrides[id=x]`), for example a deviation row that resets to the annex value.
    pub fn remove(target: SubjectRef, action: LocalizedCopy) -> Self {
        Self {
            target,
            current: Quantity::new(QuantityKind::Dimensionless, 0.0),
            required: Quantity::new(QuantityKind::Dimensionless, 0.0),
            bound: RemedyBound::Remove,
            options: Vec::new(),
            action,
            applicable: true,
        }
    }

    /// ➕ Inserts a typed row at the end of the list `target.path` names (`stiffeners`): `row_json` is the row as JSON, its string value `"$required"` is replaced by `required` (the computed position or size), and the family's insert rule commits it.
    pub fn insert(target: SubjectRef, row_json: impl Into<String>, required: Quantity, action: LocalizedCopy) -> Self {
        Self { target, current: Quantity::new(required.kind, 0.0), required, bound: RemedyBound::Insert, options: vec![row_json.into()], action, applicable: true }
    }

    /// 💡 A notice without a write: it names what to change when no value of a single field can be solved (`applicable: false`, no options), so the checks window shows the text and never offers a one-click apply.
    pub fn advisory(target: SubjectRef, action: LocalizedCopy) -> Self {
        Self { applicable: false, ..Self::one_of(target, Vec::new(), action) }
    }

    pub fn one_of(target: SubjectRef, options: Vec<String>, action: LocalizedCopy) -> Self {
        Self {
            target,
            current: Quantity::new(QuantityKind::Dimensionless, 0.0),
            required: Quantity::new(QuantityKind::Dimensionless, 0.0),
            bound: RemedyBound::OneOf,
            options,
            action,
            applicable: true,
        }
    }
}

/// 📍️ One sampled position of a check: entity-local `at` (`[s]` along a member, `[x, y]` on a plate or `[x, y, z]` in a solid), its utilisation and the governing quantities there.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(any(test, feature = "compliance-testing"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "compliance-testing"), serde(rename_all = "camelCase"))]
pub struct Station {
    pub at: Vec<f64>,
    pub utilization: f64,
    pub values: Vec<Quantity>,
}

impl Station {
    pub fn new(at: &[f64], utilization: f64, values: Vec<Quantity>) -> Self {
        Self { at: at.to_vec(), utilization: if utilization.is_nan() { UTILIZATION_CAP } else { utilization.min(UTILIZATION_CAP) }, values }
    }

    /// 📏️ Station at the member-local coordinate `s` in `[0, 1]`.
    pub fn along(s: f64, utilization: f64) -> Self {
        Self::new(&[s], utilization, Vec::new())
    }

    /// 🗺️ Station at the plate-local point `(x, y)`.
    pub fn point(x: f64, y: f64, utilization: f64) -> Self {
        Self::new(&[x, y], utilization, Vec::new())
    }

    /// 🧊️ Station at the solid-local point `(x, y, z)`.
    pub fn point3(x: f64, y: f64, z: f64, utilization: f64) -> Self {
        Self::new(&[x, y, z], utilization, Vec::new())
    }

    /// 🧮️ Station whose utilisation is `demand / capacity`, keeping both as its governing quantities.
    pub fn ratio_of(at: &[f64], demand: Quantity, capacity: Quantity) -> Self {
        Self::new(at, ratio(demand.value, capacity.value), vec![demand, capacity])
    }

    pub fn with(mut self, value: Quantity) -> Self {
        self.values.push(value);
        self
    }

    /// 🔎️ True for a finite position of one to three coordinates (`[0, 1]` for the member coordinate) with a finite non-negative utilisation.
    pub fn is_well_formed(&self) -> bool {
        (1..=3).contains(&self.at.len()) && self.at.iter().all(|c| c.is_finite()) && (self.at.len() > 1 || (0.0..=1.0).contains(&self.at[0])) && self.utilization.is_finite() && self.utilization >= 0.0
    }
}

/// 📋️ One computed check with clause traceability, subject binding, and remediation.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(any(test, feature = "compliance-testing"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "compliance-testing"), serde(rename_all = "camelCase"))]
pub struct CheckResult {
    pub id: String,
    pub part: String,
    pub clause: ClauseId,
    pub subject: SubjectRef,
    pub status: CheckStatus,
    pub title: LocalizedCopy,
    pub explanation: LocalizedCopy,
    pub computed: Quantity,
    pub limit: Quantity,
    pub utilization: f64,
    pub annex: AnnexChoice,
    pub remedies: Vec<Remedy>,
    pub ndps: Vec<String>,
    pub stations: Vec<Station>,
    pub station_field: String,
}

impl CheckResult {
    /// 📍️ Largest station utilisation, when stations are present.
    pub fn max_station_utilization(&self) -> Option<f64> {
        self.stations.iter().map(|s| s.utilization).reduce(f64::max)
    }

    /// ⚖️ Law: with stations, `utilization == max(stations)` (at least 1 on failure) and every station is well formed.
    pub fn stations_consistent(&self) -> bool {
        let Some(max) = self.max_station_utilization() else { return true };
        let expected = if self.status == CheckStatus::Fail { max.max(1.0) } else { max };
        self.stations.iter().all(Station::is_well_formed) && (expected - self.utilization).abs() <= 1e-12 * expected.abs().max(1.0)
    }

    fn reported_utilization(&self) -> f64 {
        if self.status == CheckStatus::Fail { self.utilization.max(1.0) } else { self.utilization }
    }

    /// 🚧️ True when the verdict rests on at least one unverified nationally determined parameter.
    pub fn unverified_basis(&self) -> bool {
        self.ndps.iter().any(|id| !ndp_is_verified_for(id, self.annex))
    }
}

/// 🧱️ Builder for [`CheckResult`] — the only construction path for family evaluate().
pub struct CheckBuilder {
    pub(super) id: String,
    pub(super) part: String,
    pub(super) clause: ClauseId,
    pub(super) subject: SubjectRef,
    pub(super) title: LocalizedCopy,
    pub(super) explanation: LocalizedCopy,
    computed: Quantity,
    limit: Quantity,
    pub(super) utilization: f64,
    pub(super) status: Option<CheckStatus>,
    pub(super) warn_above: Option<f64>,
    pub(super) annex: AnnexChoice,
    pub(super) remedies: Vec<Remedy>,
    remedies_attached_by: Option<&'static str>,
    pub(super) not_applicable: bool,
    pub(super) ndps: Vec<String>,
    degenerate_capacity: bool,
    stations: Vec<Station>,
    station_field: String,
}

impl CheckResult {
    pub fn assess(
        id: impl Into<String>,
        part: impl Into<String>,
        clause: ClauseId,
        subject: SubjectRef,
        title: LocalizedCopy,
    ) -> CheckBuilder {
        CheckBuilder {
            id: id.into(),
            part: part.into(),
            clause,
            subject,
            title,
            explanation: LocalizedCopy::new("", ""),
            computed: Quantity::new(QuantityKind::Dimensionless, 0.0),
            limit: Quantity::new(QuantityKind::Dimensionless, 0.0),
            utilization: 0.0,
            status: None,
            warn_above: None,
            annex: AnnexChoice::En,
            remedies: Vec::new(),
            remedies_attached_by: None,
            not_applicable: false,
            ndps: Vec::new(),
            degenerate_capacity: false,
            stations: Vec::new(),
            station_field: String::new(),
        }
    }
}

impl CheckBuilder {
    pub fn utilization(mut self, computed: Quantity, limit: Quantity) -> Self {
        self.computed = computed;
        self.limit = limit;
        self.utilization = ratio(computed.value, limit.value);
        self.degenerate_capacity = limit.value < f64::EPSILON && computed.value >= f64::EPSILON;
        self.status = Some(limit_status(self.utilization));
        self
    }

    pub fn minimum(mut self, computed: Quantity, minimum: Quantity) -> Self {
        self.computed = computed;
        self.limit = minimum;
        self.utilization = if minimum.value <= 0.0 && computed.value.is_finite() { 0.0 } else { ratio(minimum.value, computed.value) };
        let passes = computed.value.is_finite() && (computed.value >= minimum.value || (minimum.value > 0.0 && within_limit(ratio(minimum.value, computed.value))));
        self.status = Some(if passes { CheckStatus::Pass } else { CheckStatus::Fail });
        if !passes {
            self.utilization = self.utilization.max(1.0);
        }
        self
    }

    /// 🔖️ Records a nationally determined parameter of [`NDP_TABLE`] this check depends on.
    pub fn ndp(mut self, id: &str) -> Self {
        debug_assert!(ndp(id).is_some(), "unknown NDP id: {id}");
        if !self.ndps.iter().any(|known| known == id) {
            self.ndps.push(id.to_owned());
        }
        self
    }

    /// 🔖️ Records several NDP ids at once; see [`Self::ndp`].
    pub fn ndps(mut self, ids: &[&str]) -> Self {
        for id in ids {
            self = self.ndp(id);
        }
        self
    }

    /// 🔖️ Records every NDP read through [`ndp_number`] since the previous drain on this thread; call it right before `build()`.
    pub fn ndp_reads(self) -> Self {
        self.ndps(&drain_ndp_reads())
    }

    /// 📍️ Attaches sampled stations; the governing utilisation becomes their maximum.
    pub fn stations(mut self, stations: Vec<Station>) -> Self {
        self.stations = stations;
        self
    }

    /// 🗺️ Attaches stations of a named 2D/3D field (for example `soil-pressure`, `wall-phi`, `punching-perimeter`).
    pub fn field(mut self, name: &str, stations: Vec<Station>) -> Self {
        self.station_field = name.to_owned();
        self.stations(stations)
    }

    /// 📏️ Samples `n + 1` stations at `s = i / n` along the member from `f(s) -> (utilisation, governing quantities)`.
    pub fn sampled(self, n: usize, f: impl Fn(f64) -> (f64, Vec<Quantity>)) -> Self {
        let n = n.max(1);
        let stations = (0..=n)
            .map(|i| {
                let s = i as f64 / n as f64;
                let (utilization, values) = f(s);
                Station::new(&[s], utilization, values)
            })
            .collect();
        self.stations(stations)
    }

    /// ℹ️ Informative row without a limit: reported as [`CheckStatus::Info`], never as a failure and never counted in the worst utilisation.
    pub fn informative(mut self, value: Quantity) -> Self {
        self.computed = value;
        self.utilization = 0.0;
        self.status = Some(CheckStatus::Info);
        self
    }

    pub fn status(mut self, status: CheckStatus) -> Self {
        self.status = Some(status);
        self
    }

    pub fn warn_above(mut self, threshold: f64) -> Self {
        self.warn_above = Some(threshold);
        self
    }

    pub fn warn_near_limit(self) -> Self {
        self.warn_above(near_limit().threshold())
    }

    pub fn annex(mut self, annex: AnnexChoice) -> Self {
        self.annex = annex;
        self
    }

    pub fn explanation(mut self, explanation: LocalizedCopy) -> Self {
        self.explanation = explanation;
        self
    }

    pub fn remedy(mut self, remedy: Remedy) -> Self {
        self.remedies.push(remedy);
        self
    }

    /// 🩹 Names the post-pass function of the artifact that attaches the remedies after `build()`; the remedy law (`⚖️compliance/🧪️tests/🩹️remedy-law`) checks that the function exists.
    pub fn remedies_attached_by(mut self, post_pass: &'static str) -> Self {
        self.remedies_attached_by = Some(post_pass);
        self
    }

    pub fn not_applicable(mut self, reason: LocalizedCopy) -> Self {
        self.not_applicable = true;
        self.status = Some(CheckStatus::NotApplicable);
        self.explanation = reason;
        self.utilization = 0.0;
        self
    }

    /// 🩹 Builds the check; a failing check without a remedy (and without a named post-pass) never panics at run time: the result carries the [`MissingRemedy`] notice in its explanation, see [`Self::try_build`].
    pub fn build(self) -> CheckResult {
        let (mut check, missing) = self.assemble();
        if let Some(missing) = missing {
            let notice = missing.notice();
            check.explanation = LocalizedCopy::new(format!("{} {}.", check.explanation.en, notice.en).trim().to_owned(), format!("{} {}.", check.explanation.de, notice.de).trim().to_owned());
        }
        check
    }

    /// 🩹 Builds the check or rejects a failing check that carries no remedy and names no post-pass ([`Self::remedies_attached_by`]).
    pub fn try_build(self) -> Result<CheckResult, MissingRemedy> {
        match self.assemble() {
            (_, Some(missing)) => Err(missing),
            (check, None) => Ok(check),
        }
    }

    fn assemble(mut self) -> (CheckResult, Option<MissingRemedy>) {
        let mut status = self.status.unwrap_or(CheckStatus::Pass);
        if self.not_applicable {
            status = CheckStatus::NotApplicable;
        } else if let (Some(threshold), CheckStatus::Pass) = (self.warn_above, status) {
            if self.utilization > threshold {
                status = CheckStatus::Warning;
            }
        }
        let mut governing = self.utilization;
        if !self.not_applicable && !self.stations.is_empty() {
            governing = self.stations.iter().map(|s| s.utilization).fold(0.0, f64::max);
            if !within_limit(governing) && matches!(status, CheckStatus::Pass | CheckStatus::Warning) {
                status = CheckStatus::Fail;
            }
        }
        let mut explanation = self.explanation;
        if explanation.en.trim().is_empty() {
            explanation.en = format!("Check of {}.", self.title.en);
        }
        if explanation.de.trim().is_empty() {
            explanation.de = format!("Nachweis: {}.", self.title.de);
        }
        if self.degenerate_capacity && status == CheckStatus::Fail {
            explanation = LocalizedCopy::new(
                format!("{} Demand against a non-positive capacity.", explanation.en).trim().to_owned(),
                format!("{} Beanspruchung bei nicht positivem Widerstand.", explanation.de).trim().to_owned(),
            );
        }
        if matches!(status, CheckStatus::Pass | CheckStatus::Warning) {
            let open: Vec<&str> = self.ndps.iter().map(String::as_str).filter(|id| !ndp_is_verified_for(id, self.annex)).collect();
            if !open.is_empty() {
                status = CheckStatus::UnverifiedBasis;
                let list = open.join(", ");
                explanation = LocalizedCopy::new(
                    format!("{} Unverified basis, not a compliant verdict: {list}.", explanation.en).trim().to_owned(),
                    format!("{} Ungeprüfte Grundlage, kein Konformitätsnachweis: {list}.", explanation.de).trim().to_owned(),
                );
            }
        }
        let gaps = if self.not_applicable { Vec::new() } else { drain_ndp_gaps() };
        if !gaps.is_empty() {
            status = CheckStatus::UnverifiedBasis;
            let (en, de): (Vec<String>, Vec<String>) = gaps.iter().map(|gap| (gap.notice().en, gap.notice().de)).unzip();
            explanation = LocalizedCopy::new(format!("{} Refused: {}.", explanation.en, en.join("; ")).trim().to_owned(), format!("{} Verweigert: {}.", explanation.de, de.join("; ")).trim().to_owned());
            for gap in &gaps {
                if !self.ndps.contains(&gap.id) {
                    self.ndps.push(gap.id.clone());
                }
            }
        }
        let utilization = if status == CheckStatus::Fail { ratio(governing, 1.0).max(1.0) } else if governing.is_nan() { UTILIZATION_CAP } else { governing };
        let missing = (status == CheckStatus::Fail && self.remedies.is_empty() && self.remedies_attached_by.is_none()).then(|| MissingRemedy { check_id: self.id.clone() });
        let check = CheckResult {
            id: self.id,
            part: self.part,
            clause: self.clause,
            subject: self.subject,
            status,
            title: self.title,
            explanation,
            computed: self.computed,
            limit: self.limit,
            utilization,
            annex: self.annex,
            remedies: self.remedies,
            ndps: self.ndps,
            stations: if self.not_applicable { Vec::new() } else { self.stations },
            station_field: self.station_field,
        };
        (check, missing)
    }
}

/// 🩹 A failing check was built without a remedy: every failing check names what to change (dimension, material, arrangement).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MissingRemedy {
    pub check_id: String,
}

impl MissingRemedy {
    /// 🩹 Internal diagnostic shown like any note.
    pub fn notice(&self) -> LocalizedCopy {
        LocalizedCopy::new("No remedy defined for this failing check", "Für diese nicht erfüllte Nachweisführung ist keine Abhilfe hinterlegt")
    }
}

impl std::fmt::Display for MissingRemedy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Fail checks must carry at least one remedy: id={}", self.check_id)
    }
}

impl std::error::Error for MissingRemedy {}

/// 📊️ Per-part rollup of check statuses.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(any(test, feature = "compliance-testing"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "compliance-testing"), serde(rename_all = "camelCase"))]
pub struct PartVerdict {
    pub part: String,
    pub pass: u32,
    pub warning: u32,
    pub fail: u32,
    pub not_applicable: u32,
    pub unverified: u32,
    pub info: u32,
    pub worst_utilization: f64,
    pub complies: bool,
}

/// 📊️ Aggregate rollup for a full compliance report.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(any(test, feature = "compliance-testing"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "compliance-testing"), serde(rename_all = "camelCase"))]
pub struct CheckReportSummary {
    pub total: u32,
    pub pass: u32,
    pub warning: u32,
    pub fail: u32,
    pub not_applicable: u32,
    pub unverified: u32,
    pub info: u32,
    pub worst_utilization: f64,
    pub complies: bool,
    pub parts: Vec<PartVerdict>,
}

impl Default for CheckReportSummary {
    fn default() -> Self {
        Self {
            total: 0,
            pass: 0,
            warning: 0,
            fail: 0,
            not_applicable: 0,
            unverified: 0,
            info: 0,
            worst_utilization: 0.0,
            complies: true,
            parts: Vec::new(),
        }
    }
}

/// 📑️ Aggregated compliance report for a norm computation run.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(any(test, feature = "compliance-testing"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "compliance-testing"), serde(rename_all = "camelCase"))]
pub struct CheckReport {
    pub summary: CheckReportSummary,
    pub checks: Vec<CheckResult>,
}

impl CheckReport {
    pub fn push(&mut self, check: CheckResult) {
        Self::accumulate(&mut self.summary, &check);
        self.checks.push(check);
    }

    pub fn extend(&mut self, checks: impl IntoIterator<Item = CheckResult>) {
        for check in checks {
            self.push(check);
        }
    }

    pub fn complies(&self) -> bool {
        self.summary.complies
    }

    pub fn worst_utilization(&self) -> f64 {
        self.summary.worst_utilization
    }

    pub fn failing(&self) -> impl Iterator<Item = &CheckResult> {
        self.checks.iter().filter(|c| c.status == CheckStatus::Fail)
    }

    pub fn unverified(&self) -> impl Iterator<Item = &CheckResult> {
        self.checks.iter().filter(|c| c.status == CheckStatus::UnverifiedBasis)
    }

    /// 📋️ Distinct unverified NDP ids behind this report, in first-use order.
    pub fn unverified_ndps(&self) -> Vec<&str> {
        let mut ids: Vec<&str> = Vec::new();
        for (id, annex) in self.checks.iter().flat_map(|c| c.ndps.iter().map(move |id| (id.as_str(), c.annex))) {
            if !ndp_is_verified_for(id, annex) && !ids.contains(&id) {
                ids.push(id);
            }
        }
        ids
    }

    pub fn by_part(&self) -> Vec<(&str, Vec<&CheckResult>)> {
        let mut groups: Vec<(&str, Vec<&CheckResult>)> = Vec::new();
        for check in &self.checks {
            if let Some((_, bucket)) = groups.iter_mut().find(|(part, _)| *part == check.part.as_str()) {
                bucket.push(check);
            } else {
                groups.push((check.part.as_str(), vec![check]));
            }
        }
        groups
    }

    fn accumulate(summary: &mut CheckReportSummary, check: &CheckResult) {
        summary.total += 1;
        match check.status {
            CheckStatus::Pass => summary.pass += 1,
            CheckStatus::Warning => summary.warning += 1,
            CheckStatus::Fail => {
                summary.fail += 1;
                summary.complies = false;
            }
            CheckStatus::NotApplicable => summary.not_applicable += 1,
            CheckStatus::Info => summary.info += 1,
            CheckStatus::UnverifiedBasis => {
                summary.unverified += 1;
                summary.complies = false;
            }
        }
        if !matches!(check.status, CheckStatus::NotApplicable | CheckStatus::Info) {
            summary.worst_utilization = summary.worst_utilization.max(check.reported_utilization());
        }
        let part = if let Some(existing) = summary.parts.iter_mut().find(|p| p.part == check.part) {
            existing
        } else {
            summary.parts.push(PartVerdict {
                part: check.part.clone(),
                pass: 0,
                warning: 0,
                fail: 0,
                not_applicable: 0,
                unverified: 0,
                info: 0,
                worst_utilization: 0.0,
                complies: true,
            });
            summary.parts.last_mut().expect("just pushed")
        };
        match check.status {
            CheckStatus::Pass => part.pass += 1,
            CheckStatus::Warning => part.warning += 1,
            CheckStatus::Fail => {
                part.fail += 1;
                part.complies = false;
            }
            CheckStatus::NotApplicable => part.not_applicable += 1,
            CheckStatus::Info => part.info += 1,
            CheckStatus::UnverifiedBasis => {
                part.unverified += 1;
                part.complies = false;
            }
        }
        if !matches!(check.status, CheckStatus::NotApplicable | CheckStatus::Info) {
            part.worst_utilization = part.worst_utilization.max(check.reported_utilization());
        }
    }
}

use semio_framework_value::DslValue;
use std::collections::{BTreeMap, BTreeSet};

/// 🧭️ Who owns a check: the top-level collections whose rows are drawn as scene entities (every collection when empty) and the id prefixes of the documented document-level checks that concern the whole document and have no owner. `file` makes `subject.entity_id` the owner, so the checks window, the selection mirror and the 3D scene read one owner.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Owners {
    pub collections: &'static [&'static str],
    pub document_level: &'static [&'static str],
    pub filed: bool,
}

impl Owners {
    /// 🧭️ Every row owns and no check is document-level.
    pub const ROWS: Self = Self { collections: &[], document_level: &[], filed: true };
    /// 🧭️ Keeps every subject as the emitter named it, for a family that files its checks under its own aggregate entities.
    pub const AS_EMITTED: Self = Self { collections: &[], document_level: &[], filed: false };

    /// 🧭️ The rows of `collections` own; checks whose id starts with one of `document_level` belong to the document.
    pub const fn new(collections: &'static [&'static str], document_level: &'static [&'static str]) -> Self {
        Self { collections, document_level, filed: true }
    }

    /// 🌐️ Whether `check_id` is a documented document-level check.
    pub fn is_document_level(&self, check_id: &str) -> bool {
        self.document_level.iter().any(|prefix| check_id.starts_with(prefix))
    }

    fn owns(&self, collection: &str) -> bool {
        self.collections.is_empty() || self.collections.contains(&collection)
    }

    /// 🆔️ The ids of the rows of the owning collections.
    pub fn owner_ids(&self, document: &DslValue) -> BTreeSet<String> {
        collections_of(document).into_iter().filter(|(name, _)| self.owns(name)).flat_map(|(_, rows)| rows.into_iter().filter_map(row_id)).map(str::to_owned).collect()
    }

    /// 🧭️ Files every check of `report` under its owner. The owner is, in this order: the subject entity when it is an owning row; the owning row whose id the check id embeds as a run of dot-separated segments (`en1992.6.2.shear.beam-1` belongs to `beam-1`, the longest run wins, then the leftmost); the first owning row that names the subject row in one of its fields (a section row is owned by the member that uses it); none, which clears the subject for a documented document-level check and leaves it as emitted otherwise.
    pub fn file(&self, document: &DslValue, report: &mut CheckReport) {
        if !self.filed {
            return;
        }
        let collections = collections_of(document);
        let owners: Vec<(&str, &DslValue)> = collections.iter().filter(|(name, _)| self.owns(name)).flat_map(|(_, rows)| rows.iter().copied().filter_map(|row| row_id(row).map(|id| (id, row)))).collect();
        let owner_ids: BTreeSet<&str> = owners.iter().map(|(id, _)| *id).collect();
        let referenced: BTreeSet<&str> = collections.iter().filter(|(name, _)| !self.owns(name)).flat_map(|(_, rows)| rows.iter().copied().filter_map(row_id)).collect();
        let mut redirects: BTreeMap<String, Option<String>> = BTreeMap::new();
        for check in &mut report.checks {
            let subject = check.subject.entity_id.as_str();
            if owner_ids.contains(subject) {
                continue;
            }
            let redirected = referenced.contains(subject).then(|| redirects.entry(subject.to_owned()).or_insert_with(|| owners.iter().find(|(id, row)| *id != subject && names_row(row, subject)).map(|(id, _)| (*id).to_owned())).clone()).flatten();
            let owner = embedded_owner(&check.id, &owner_ids).map(str::to_owned).or(redirected);
            match owner {
                Some(owner) => check.subject.entity_id = owner,
                None if self.is_document_level(&check.id) => check.subject.entity_id.clear(),
                None => {}
            }
        }
    }
}

fn collections_of(document: &DslValue) -> Vec<(&str, Vec<&DslValue>)> {
    let DslValue::Object(fields) = document else { return Vec::new() };
    fields.iter().filter_map(|(name, field)| if let DslValue::Array(rows) = field { Some((name.as_str(), rows.iter().collect())) } else { None }).collect()
}

fn row_id(row: &DslValue) -> Option<&str> {
    let DslValue::Object(entries) = row else { return None };
    entries.iter().find_map(|(key, value)| if let ("id", DslValue::String(id)) = (key.as_str(), value) { Some(id.as_str()) } else { None })
}

fn names_row(row: &DslValue, id: &str) -> bool {
    match row {
        DslValue::String(text) => text == id,
        DslValue::Array(items) => items.iter().any(|item| names_row(item, id)),
        DslValue::Object(entries) => entries.iter().filter(|(key, _)| key != "id").any(|(_, value)| names_row(value, id)),
        _ => false,
    }
}

fn embedded_owner<'a>(check_id: &str, owners: &BTreeSet<&'a str>) -> Option<&'a str> {
    let segments: Vec<&str> = check_id.split('.').collect();
    (1..=segments.len()).rev().find_map(|len| (0..=segments.len() - len).find_map(|start| owners.get(segments[start..start + len].join(".").as_str()).copied()))
}
// #endregion 🔖️Check
