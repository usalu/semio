//! 🗂️ The index of the findings: for every element the codes that name it and its worst severity, and the counts of the model by severity, category, code and storey. The index is a pure function of the ordered findings
//! (derived from derived), so it is projected next to `diagnostics` and never stored. The category of a code is the domain of its slug (`clash.wall-wall` is in the category `clash`), so a new code needs no table row.

use super::{Diagnostic, DiagnosticCode, Severity};
use std::collections::{BTreeMap, BTreeSet};

//#region 🔖️Values
/// 🔢️ How many findings of each severity.
#[derive(semio_framework_value::RetireOwned, Clone, Copy, Debug, Default, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue)]
pub struct SeverityCounts {
    pub error: u32,
    pub warning: u32,
    pub info: u32,
}

impl SeverityCounts {
    /// ➕️ Counts one more finding of `severity`.
    pub fn add(&mut self, severity: Severity) {
        match severity {
            Severity::Error => self.error += 1,
            Severity::Warning => self.warning += 1,
            Severity::Info => self.info += 1,
        }
    }

    /// 🔢️ The findings of one severity.
    pub fn of(&self, severity: Severity) -> u32 {
        match severity {
            Severity::Error => self.error,
            Severity::Warning => self.warning,
            Severity::Info => self.info,
        }
    }

    /// 🔢️ The findings of all severities.
    pub fn total(&self) -> u32 {
        self.error + self.warning + self.info
    }

    /// 🚦️ The most severe severity that was counted; `None` when nothing was.
    pub fn worst(&self) -> Option<Severity> {
        [Severity::Error, Severity::Warning, Severity::Info].into_iter().find(|severity| self.of(*severity) > 0)
    }
}

/// 🧩️ What the findings say about one element: how many name it, its worst severity and the codes (ordered, once each).
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue)]
pub struct ElementFindings {
    pub severity: Severity,
    pub count: u32,
    pub codes: Vec<DiagnosticCode>,
}

/// 🗂️ The index of one set of findings. `elements` holds every id a finding names (elements, storeys, buildings), `categories` and `codes` (by slug) count findings, `storeys` the findings that sit on a storey; `total` counts all.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, Default, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue)]
pub struct DiagnosticIndex {
    pub total: SeverityCounts,
    pub elements: BTreeMap<String, ElementFindings>,
    pub categories: BTreeMap<String, SeverityCounts>,
    pub codes: BTreeMap<String, u32>,
    pub storeys: BTreeMap<String, SeverityCounts>,
}
//#endregion 🔖️Values

//#region 🔖️Category
/// 🏷️ The category of a code: the domain of its slug, for example `clash` for `clash.wall-wall`.
pub fn category_of(code: DiagnosticCode) -> &'static str {
    let slug = code.slug();
    slug.split_once('.').map_or(slug, |(domain, _)| domain)
}

/// 🏷️ Every category in the order of its first code.
pub fn categories() -> Vec<&'static str> {
    let mut seen = BTreeSet::new();
    DiagnosticCode::ALL.iter().map(|code| category_of(*code)).filter(|category| seen.insert(*category)).collect()
}
//#endregion 🔖️Category

impl DiagnosticIndex {
    /// 🗂️ The index of `found`.
    pub fn of(found: &[Diagnostic]) -> Self {
        let mut index = Self::default();
        let mut codes: BTreeMap<&str, BTreeSet<DiagnosticCode>> = BTreeMap::new();
        for finding in found {
            index.total.add(finding.severity);
            index.categories.entry(category_of(finding.code).to_string()).or_default().add(finding.severity);
            *index.codes.entry(finding.code.slug().to_string()).or_default() += 1;
            if let Some(storey) = &finding.storey {
                index.storeys.entry(storey.clone()).or_default().add(finding.severity);
            }
            for id in finding.elements.iter().collect::<BTreeSet<_>>() {
                let entry = index.elements.entry(id.clone()).or_insert(ElementFindings { severity: finding.severity, count: 0, codes: Vec::new() });
                entry.severity = entry.severity.max(finding.severity);
                entry.count += 1;
                codes.entry(id).or_default().insert(finding.code);
            }
        }
        for (id, set) in codes {
            if let Some(entry) = index.elements.get_mut(id) {
                entry.codes = set.into_iter().collect();
            }
        }
        index
    }

    /// 🚦️ The worst severity of the element; `None` when no finding names it.
    pub fn severity_of(&self, id: &str) -> Option<Severity> {
        self.elements.get(id).map(|entry| entry.severity)
    }

    /// 🔢️ The count of findings of one severity on a storey.
    pub fn on_storey(&self, storey: &str) -> SeverityCounts {
        self.storeys.get(storey).copied().unwrap_or_default()
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
