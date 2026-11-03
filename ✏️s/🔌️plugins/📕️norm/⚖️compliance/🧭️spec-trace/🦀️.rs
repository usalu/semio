//! 🧭️ Spec traceability shared by every norm artifact: the typed twin of `🧬️schema/🔣️.json` (spec id → clause → realisation → entity) and the test helper that proves a trace complete. See [`SpecTraceTable`].

use crate::design_case::{evaluate_case, seed, CaseCatalogue};
use crate::{AnnexChoice, CheckReport, CheckStatus};
use semio_framework_ui_locale::Locale;
use std::collections::BTreeSet;

/// 🌐️ Static bilingual text of a spec trace row: both languages are mandatory and no language is the default.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SpecText {
    pub en: &'static str,
    pub de: &'static str,
}

impl SpecText {
    pub const fn new(en: &'static str, de: &'static str) -> Self {
        Self { en, de }
    }

    /// 🗣️ The text of one locale.
    pub fn resolve(&self, locale: &Locale) -> &'static str {
        match locale {
            Locale::En => self.en,
            Locale::De => self.de,
        }
    }

    /// 📏️ Whether either language is shorter than `min` characters once trimmed.
    pub fn shorter_than(&self, min: usize) -> bool {
        self.en.trim().chars().count() < min || self.de.trim().chars().count() < min
    }
}

/// 🔗️ How a spec id is realised: check-id fragments emitted by an example document or design case, a data table or function, a design-case catalogue entry, or the reason it is not modelled.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Realisation {
    Check(&'static [&'static str]),
    Data(&'static str),
    DesignCase(&'static str),
    NotModelled(SpecText),
}

impl Realisation {
    /// 🏷️ The `kind` tag of the JSON twin.
    pub const fn kind(&self) -> &'static str {
        match self {
            Self::Check(_) => "check",
            Self::Data(_) => "data",
            Self::DesignCase(_) => "design-case",
            Self::NotModelled(_) => "not-modelled",
        }
    }

    /// 🔤️ The realisation as one line in a locale: fragments, data path, entry id or reason.
    pub fn describe(&self, locale: &Locale) -> String {
        match self {
            Self::Check(checks) => checks.join(", "),
            Self::Data(path) => (*path).into(),
            Self::DesignCase(entry) => (*entry).into(),
            Self::NotModelled(reason) => reason.resolve(locale).into(),
        }
    }
}

/// 🧾️ One spec id: its clause, the snapshot entity that carries it (may be empty), its realisation and a bilingual description of what the id requires and how it is realised.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SpecTrace {
    pub id: &'static str,
    pub clause: &'static str,
    pub entity: &'static str,
    pub realisation: Realisation,
    pub description: SpecText,
}

impl SpecTrace {
    pub const fn new(id: &'static str, clause: &'static str, entity: &'static str, realisation: Realisation, description: SpecText) -> Self {
        Self { id, clause, entity, realisation, description }
    }
}

/// 🔢️ Number of spec ids per realisation kind.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct TraceCounts {
    pub check: usize,
    pub data: usize,
    pub design_case: usize,
    pub not_modelled: usize,
}

/// 🗂️ The trace of one artifact: the ids its target spec defines and one [`SpecTrace`] per id, in spec order.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SpecTraceTable {
    pub code: &'static str,
    pub standard: &'static str,
    pub spec: &'static str,
    pub spec_ids: &'static [&'static str],
    pub items: &'static [SpecTrace],
}

impl SpecTraceTable {
    /// 🔎️ The row of a spec id.
    pub fn item(&self, id: &str) -> Option<&'static SpecTrace> {
        self.items.iter().find(|item| item.id == id)
    }

    /// 🔢️ Rows per realisation kind.
    pub fn counts(&self) -> TraceCounts {
        self.items.iter().fold(TraceCounts::default(), |mut counts, item| {
            match item.realisation {
                Realisation::Check(_) => counts.check += 1,
                Realisation::Data(_) => counts.data += 1,
                Realisation::DesignCase(_) => counts.design_case += 1,
                Realisation::NotModelled(_) => counts.not_modelled += 1,
            }
            counts
        })
    }
}

/// 📊️ The standing of one spec id in a report: the checks that realise it, the failing ones and the worst finite utilisation.
#[derive(Clone, Debug, PartialEq)]
pub struct TraceStanding {
    pub id: &'static str,
    pub checks: usize,
    pub failing: usize,
    pub worst_utilization: f64,
}

/// 📊️ The standing of every `Check`-realised spec id in a report, in trace order; the overview verifier of the spec.
pub fn standing(table: &SpecTraceTable, report: &CheckReport) -> Vec<TraceStanding> {
    table
        .items
        .iter()
        .filter_map(|item| match item.realisation {
            Realisation::Check(fragments) => {
                let checks: Vec<_> = report.checks.iter().filter(|check| fragments.iter().any(|fragment| check.id.contains(fragment))).collect();
                Some(TraceStanding {
                    id: item.id,
                    checks: checks.len(),
                    failing: checks.iter().filter(|check| check.status == CheckStatus::Fail).count(),
                    worst_utilization: checks.iter().map(|check| check.utilization).filter(|u| u.is_finite()).fold(0.0, f64::max),
                })
            }
            _ => None,
        })
        .collect()
}

/// 📡️ What example documents and design cases emit: the check ids of their reports and the catalogue entries that produced at least one check.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Emitted {
    check_ids: BTreeSet<String>,
    case_entries: BTreeSet<String>,
}

impl Emitted {
    /// 📥️ Adds the check ids of one report.
    pub fn with_checks<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.check_ids.extend(ids.into_iter().map(|id| id.as_ref().to_owned()));
        self
    }

    /// 📥️ Seeds and evaluates every entry of the case catalogue `C`; an entry that yields checks counts as run and its check ids count as emitted.
    pub fn with_case_catalogue<C: CaseCatalogue>(mut self, annex: AnnexChoice) -> Self {
        for entry in C::entries() {
            let results = evaluate_case::<C>(&seed::<C>(&entry, "spec-trace"), annex);
            if !results.is_empty() {
                self.case_entries.insert(entry.id.to_owned());
            }
            self.check_ids.extend(results.into_iter().map(|result| result.id));
        }
        self
    }

    /// 🎯️ Whether some emitted check id contains the fragment.
    pub fn emits(&self, fragment: &str) -> bool {
        self.check_ids.iter().any(|id| id.contains(fragment))
    }

    /// 🎯️ Whether the catalogue entry produced checks.
    pub fn ran_entry(&self, entry: &str) -> bool {
        self.case_entries.contains(entry)
    }
}

/// 🩺️ Every defect of a trace: ids of the spec without a row, rows without a spec id, duplicates, check fragments no example emits, design-case entries that produce no check, empty data paths and unexplained non-modelled ids.
pub fn problems(table: &SpecTraceTable, emitted: &Emitted) -> Vec<String> {
    let mut found = Vec::new();
    let spec: BTreeSet<&str> = table.spec_ids.iter().copied().collect();
    let mut seen = BTreeSet::new();
    if spec.len() != table.spec_ids.len() {
        found.push("the spec id list has duplicates".to_owned());
    }
    for item in table.items {
        if !seen.insert(item.id) {
            found.push(format!("{}: traced twice", item.id));
        }
        if !spec.contains(item.id) {
            found.push(format!("{}: not an id of the spec", item.id));
        }
        match item.realisation {
            Realisation::Check(checks) => {
                if checks.is_empty() {
                    found.push(format!("{}: no check fragment", item.id));
                }
                for fragment in checks.iter().filter(|fragment| !emitted.emits(fragment)) {
                    found.push(format!("{}: no example document or design case emits a check id containing `{fragment}`", item.id));
                }
            }
            Realisation::Data(path) if path.trim().is_empty() => found.push(format!("{}: empty data path", item.id)),
            Realisation::DesignCase(entry) if !emitted.ran_entry(entry) => found.push(format!("{}: design-case entry `{entry}` produces no check", item.id)),
            Realisation::NotModelled(reason) if reason.shorter_than(20) => found.push(format!("{}: reason too short to explain", item.id)),
            _ if item.description.shorter_than(1) => found.push(format!("{}: description missing in a language", item.id)),
            _ => {}
        }
    }
    found.extend(spec.difference(&seen).map(|id| format!("{id}: untraced")));
    found
}

/// 🛡️ Fails with every defect of [`problems`] listed.
pub fn assert_traced(table: &SpecTraceTable, emitted: &Emitted) {
    let found = problems(table, emitted);
    assert!(found.is_empty(), "{} spec trace has {} defects:\n{}", table.code, found.len(), found.join("\n"));
}

/// 🎪️ The check fragments of `Check` rows that the emitted ids do not contain, in trace order (duplicates removed): what one showcase document cannot reach.
pub fn unreached_fragments(table: &SpecTraceTable, emitted: &Emitted) -> Vec<&'static str> {
    let mut seen = BTreeSet::new();
    table
        .items
        .iter()
        .filter_map(|item| match item.realisation {
            Realisation::Check(fragments) => Some(fragments),
            _ => None,
        })
        .flatten()
        .copied()
        .filter(|fragment| !emitted.emits(fragment) && seen.insert(*fragment))
        .collect()
}

/// 🎪️ Fails unless the fragments the emitted ids miss are exactly the `allowed` ones (fragment, reason of at least 20 characters): a showcase document reaches every check family except the documented ones, and a stale allowance fails too.
pub fn assert_reaches_all_but(table: &SpecTraceTable, emitted: &Emitted, allowed: &[(&str, &str)]) {
    let unreached = unreached_fragments(table, emitted);
    let unexplained: Vec<_> = unreached.iter().filter(|fragment| !allowed.iter().any(|(allowed, _)| allowed == *fragment)).collect();
    let stale: Vec<_> = allowed.iter().map(|(fragment, _)| *fragment).filter(|fragment| !unreached.contains(fragment)).collect();
    let short: Vec<_> = allowed.iter().filter(|(_, reason)| reason.trim().chars().count() < 20).map(|(fragment, _)| *fragment).collect();
    assert!(unexplained.is_empty() && stale.is_empty() && short.is_empty(), "{} showcase: unreached without a reason {unexplained:?}; allowed but reached {stale:?}; reason too short {short:?}", table.code);
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
