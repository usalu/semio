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
    pub(super) not_applicable: bool,
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
            not_applicable: false,
        }
    }
}

impl CheckBuilder {
    pub fn utilization(mut self, computed: Quantity, limit: Quantity) -> Self {
        self.computed = computed;
        self.limit = limit;
        self.utilization = if limit.value.abs() < f64::EPSILON {
            0.0
        } else {
            computed.value / limit.value
        };
        self.status = Some(if self.utilization <= 1.0 { CheckStatus::Pass } else { CheckStatus::Fail });
        self
    }

    pub fn minimum(mut self, computed: Quantity, minimum: Quantity) -> Self {
        self.computed = computed;
        self.limit = minimum;
        let passes = computed.value >= minimum.value;
        self.utilization = if passes {
            minimum.value / computed.value.max(minimum.value)
        } else {
            computed.value / minimum.value.max(f64::EPSILON)
        };
        self.status = Some(if passes { CheckStatus::Pass } else { CheckStatus::Fail });
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

    pub fn not_applicable(mut self, reason: LocalizedCopy) -> Self {
        self.not_applicable = true;
        self.status = Some(CheckStatus::NotApplicable);
        self.explanation = reason;
        self.utilization = 0.0;
        self
    }

    pub fn build(self) -> CheckResult {
        let mut status = self.status.unwrap_or(CheckStatus::Pass);
        if self.not_applicable {
            status = CheckStatus::NotApplicable;
        } else if let (Some(threshold), CheckStatus::Pass) = (self.warn_above, status) {
            if self.utilization > threshold {
                status = CheckStatus::Warning;
            }
        }
        debug_assert!(status != CheckStatus::Fail || !self.remedies.is_empty(), "Fail checks must carry at least one remedy: id={}", self.id);
        CheckResult {
            id: self.id,
            part: self.part,
            clause: self.clause,
            subject: self.subject,
            status,
            title: self.title,
            explanation: self.explanation,
            computed: self.computed,
            limit: self.limit,
            utilization: self.utilization,
            annex: self.annex,
            remedies: self.remedies,
        }
    }
}

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
        }
        if check.status != CheckStatus::NotApplicable {
            summary.worst_utilization = summary.worst_utilization.max(check.utilization);
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
        }
        if check.status != CheckStatus::NotApplicable {
            part.worst_utilization = part.worst_utilization.max(check.utilization);
        }
    }
}
// #endregion 🔖️Check
