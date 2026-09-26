//! 🧰 Shared builders for ISO 16757 compliance checks.

use crate::document::{
    AnnexChoice, CheckResult, CheckStatus, ClauseId, LocalizedCopy, Quantity, QuantityKind, Remedy, SubjectRef,
};
use crate::Names;
use crate::LocalizedText;

pub fn clause(part: &str, section: &str) -> ClauseId {
    ClauseId::new("ISO 16757", part, section)
}

pub fn copy(en: impl Into<String>, de: impl Into<String>) -> LocalizedCopy {
    LocalizedCopy::new(en, de)
}

pub fn subject(entity_id: impl Into<String>, path: impl Into<String>, en: impl Into<String>, de: impl Into<String>) -> SubjectRef {
    SubjectRef::new(entity_id, path, copy(en, de))
}

pub fn catalogue_subject() -> SubjectRef {
    subject("catalogue", "catalogue", "Catalogue", "Katalog")
}

pub fn q_dim(value: f64) -> Quantity {
    Quantity::new(QuantityKind::Dimensionless, value)
}

pub fn q_len(value: f64) -> Quantity {
    Quantity::new(QuantityKind::Length, value)
}

pub fn q_vol(value: f64) -> Quantity {
    Quantity::new(QuantityKind::Volume, value)
}

pub fn part_label(part: &str) -> String {
    format!("ISO 16757-{part}")
}

pub fn name_locales(names: &Names) -> Vec<String> {
    let mut locales = vec![names.preferred.locale.clone()];
    for alt in &names.alternatives {
        if !locales.iter().any(|l| l == &alt.locale) {
            locales.push(alt.locale.clone());
        }
    }
    locales
}

pub fn names_cover(names: &Names, required: &[String]) -> Vec<String> {
    let have = name_locales(names);
    required.iter().filter(|l| !have.iter().any(|h| h == *l)).cloned().collect()
}

pub fn text_for_locale(names: &Names, locale: &str) -> Option<String> {
    if names.preferred.locale == locale {
        return Some(names.preferred.text.clone());
    }
    names.alternatives.iter().find(|t| t.locale == locale).map(|t| t.text.clone())
}

pub fn assess(
    id: impl Into<String>,
    part: &str,
    section: &str,
    subject: SubjectRef,
    title: LocalizedCopy,
    explanation: LocalizedCopy,
    status: CheckStatus,
    computed: Quantity,
    limit: Quantity,
    remedies: Vec<Remedy>,
) -> CheckResult {
    let mut b = CheckResult::assess(id, part_label(part), clause(part, section), subject, title)
        .utilization(computed, limit)
        .status(status)
        .explanation(explanation)
        .annex(AnnexChoice::En);
    for r in remedies {
        b = b.remedy(r);
    }
    b.build()
}

pub fn pass(
    id: impl Into<String>,
    part: &str,
    section: &str,
    subject: SubjectRef,
    title: LocalizedCopy,
    explanation: LocalizedCopy,
) -> CheckResult {
    CheckResult::assess(id, part_label(part), clause(part, section), subject, title)
        .explanation(explanation)
        .status(CheckStatus::Pass)
        .annex(AnnexChoice::En)
        .build()
}

pub fn fail(
    id: impl Into<String>,
    part: &str,
    section: &str,
    subject: SubjectRef,
    title: LocalizedCopy,
    explanation: LocalizedCopy,
    remedies: Vec<Remedy>,
) -> CheckResult {
    assess(id, part, section, subject, title, explanation, CheckStatus::Fail, q_dim(2.0), q_dim(1.0), remedies)
}

pub fn na(
    id: impl Into<String>,
    part: &str,
    section: &str,
    subject: SubjectRef,
    title: LocalizedCopy,
    reason: LocalizedCopy,
) -> CheckResult {
    CheckResult::assess(id, part_label(part), clause(part, section), subject, title)
        .not_applicable(reason)
        .annex(AnnexChoice::En)
        .build()
}

pub fn ensure_locale_on_names(names: &Names, locale: &str, placeholder: &str) -> Names {
    let mut next = names.clone();
    if text_for_locale(&next, locale).is_some() {
        return next;
    }
    next.alternatives.push(LocalizedText { locale: locale.into(), text: placeholder.into() });
    next
}
