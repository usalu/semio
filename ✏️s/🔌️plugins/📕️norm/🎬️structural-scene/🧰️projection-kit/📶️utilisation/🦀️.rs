//! 📶️ Utilisation: bindings, tones and clause badges derived from a [`norm::CheckReport`].

use crate::*;
use crate::station_fields::station_values;
use norm::{CheckReport, CheckResult};

pub fn status(s: norm::CheckStatus) -> CheckStatus {
    match s {
        norm::CheckStatus::Pass => CheckStatus::Pass,
        norm::CheckStatus::Warning | norm::CheckStatus::UnverifiedBasis => CheckStatus::Warning,
        norm::CheckStatus::Fail => CheckStatus::Fail,
        norm::CheckStatus::NotApplicable | norm::CheckStatus::Info => CheckStatus::NotApplicable,
    }
}

/// 📶️ The applicable check with the largest finite utilisation among those accepted by `keep`.
pub fn worst_where(report: &CheckReport, keep: impl Fn(&CheckResult) -> bool) -> Option<&CheckResult> {
    report.checks.iter().filter(|c| c.status != norm::CheckStatus::NotApplicable && c.utilization.is_finite() && keep(c)).max_by(|a, b| a.utilization.total_cmp(&b.utilization))
}

pub fn binding_of_check(c: &CheckResult) -> UtilisationBinding {
    UtilisationBinding { check_id: c.id.clone(), ratio: c.utilization.max(0.0), status: status(c.status), per_station: Some(station_values(c)).filter(|v| !v.is_empty()), stale: false }
}

/// 🌡️ `base` re-aimed at one sample of its own check: `ratio` replaces the governing ratio and the status follows the shared colour-map thresholds (warning from 0.9, failure above 1.0), so a part of the entity is toned by its own value on the CVD-safe utilisation ramps instead of the governing one.
pub fn binding_at_ratio(base: &UtilisationBinding, ratio: f64) -> UtilisationBinding {
    let ratio = if ratio.is_finite() { ratio.max(0.0) } else { 0.0 };
    let status = match base.status {
        CheckStatus::NotApplicable => CheckStatus::NotApplicable,
        _ if ratio > 1.0 => CheckStatus::Fail,
        _ if ratio >= 0.9 => CheckStatus::Warning,
        _ => CheckStatus::Pass,
    };
    UtilisationBinding { check_id: base.check_id.clone(), ratio, status, per_station: None, stale: base.stale }
}

/// 📶️ The governing binding among the checks accepted by `keep`.
pub fn binding_where(report: &CheckReport, keep: impl Fn(&CheckResult) -> bool) -> Option<UtilisationBinding> {
    worst_where(report, keep).map(binding_of_check)
}

/// 📶️ The governing binding of any of the document rows `ids`.
pub fn binding_of_subjects(report: &CheckReport, ids: &[&str]) -> Option<UtilisationBinding> {
    binding_where(report, |c| ids.contains(&c.subject.entity_id.as_str()))
}

/// 📶️ The governing check of `subject` whose id starts with any of `prefixes` (every check when empty).
pub fn worst_of_subject<'a>(report: &'a CheckReport, subject: &str, prefixes: &[&str]) -> Option<&'a CheckResult> {
    worst_where(report, |c| c.subject.entity_id == subject && (prefixes.is_empty() || prefixes.iter().any(|p| c.id.starts_with(p))))
}

pub fn binding_of_subject(report: &CheckReport, subject: &str, prefixes: &[&str]) -> Option<UtilisationBinding> {
    worst_of_subject(report, subject, prefixes).map(binding_of_check)
}

/// 📜️ The check with exactly this id.
pub fn check_with<'a>(report: &'a CheckReport, id: &str) -> Option<&'a CheckResult> {
    report.checks.iter().find(|c| c.id == id)
}

/// 📜️ The clause of a check as scene clause badge.
pub fn clause_of(report: &CheckReport, check_id: &str) -> Option<ClauseId> {
    check_with(report, check_id).map(|c| ClauseId { family: c.clause.family.clone(), part: c.clause.part.clone(), section: c.clause.section.clone() })
}

pub fn tone_of(binding: &Option<UtilisationBinding>) -> Tone {
    match binding.as_ref().map(|b| b.status) {
        Some(CheckStatus::Fail) => Tone::Error,
        Some(CheckStatus::Warning) => Tone::Warning,
        Some(CheckStatus::Pass) => Tone::Info,
        _ => Tone::Neutral,
    }
}

/// 🗣️ The bilingual suffix `, utilisation 0.83` an accessible detail appends to a bound entity.
pub fn utilisation_words(binding: &Option<UtilisationBinding>) -> (String, String) {
    binding.as_ref().map_or_else(Default::default, |b| (format!(", utilisation {:.2}", b.ratio), format!(", Ausnutzung {:.2}", b.ratio)))
}
