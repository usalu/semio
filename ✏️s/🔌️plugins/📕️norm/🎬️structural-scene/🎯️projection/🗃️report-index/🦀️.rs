//! 🗃️ Report index: the check rows of one report grouped by subject entity and kept up to date incrementally, so a projection reads only the rows of the entity it draws and the cache key moves only for subjects whose rows really changed.

use super::fragment_cache::fnv64;
use norm::{CheckReport, CheckReportSummary, CheckResult};
use std::collections::{BTreeMap, BTreeSet};
use std::sync::Arc;

#[derive(Debug)]
struct Slot {
    version: u64,
    positions: Vec<usize>,
    rows: Arc<Vec<CheckResult>>,
}

/// 🗃️ Rows by subject entity (`""` holds the checks without one), each slot versioned; cloning shares the slots.
#[derive(Clone, Debug, Default)]
pub struct ReportIndex {
    slots: BTreeMap<String, Arc<Slot>>,
    summary: CheckReportSummary,
    version: u64,
}

impl PartialEq for ReportIndex {
    fn eq(&self, other: &Self) -> bool {
        self.version == other.version && self.summary == other.summary
    }
}

impl ReportIndex {
    /// 🆕️ The index of a whole report, built from scratch.
    pub fn of(report: &CheckReport) -> Self {
        let mut index = Self::default();
        index.follow(report);
        index
    }

    /// 🩹️ Brings the index to `report` and returns the subjects whose rows changed; slots of unchanged subjects are neither cloned nor re-versioned.
    pub fn follow(&mut self, report: &CheckReport) -> BTreeSet<String> {
        let mut grouped: BTreeMap<&str, Vec<(usize, &CheckResult)>> = BTreeMap::new();
        for (position, check) in report.checks.iter().enumerate() {
            grouped.entry(check.subject.entity_id.as_str()).or_default().push((position, check));
        }
        let mut changed: BTreeSet<String> = self.slots.keys().filter(|id| !grouped.contains_key(id.as_str())).cloned().collect();
        for id in &changed {
            self.slots.remove(id);
        }
        for (id, rows) in grouped {
            let positions: Vec<usize> = rows.iter().map(|(p, _)| *p).collect();
            let known = self.slots.get(id).filter(|slot| slot.rows.len() == rows.len() && slot.rows.iter().zip(&rows).all(|(old, (_, new))| old == *new)).map(|slot| (slot.version, slot.positions == positions, Arc::clone(&slot.rows)));
            match known {
                Some((_, true, _)) => {}
                Some((version, false, shared)) => {
                    self.slots.insert(id.to_owned(), Arc::new(Slot { version, positions, rows: shared }));
                }
                None => {
                    self.version += 1;
                    self.slots.insert(id.to_owned(), Arc::new(Slot { version: self.version, positions, rows: Arc::new(rows.into_iter().map(|(_, c)| c.clone()).collect()) }));
                    changed.insert(id.to_owned());
                }
            }
        }
        if !changed.is_empty() || self.summary != report.summary {
            self.version += 1;
            self.summary = report.summary.clone();
        }
        changed
    }

    fn ordered<'a>(slots: impl Iterator<Item = &'a Arc<Slot>>) -> Vec<CheckResult> {
        let mut rows: Vec<(usize, &CheckResult)> = slots.flat_map(|slot| slot.positions.iter().copied().zip(slot.rows.iter())).collect();
        rows.sort_by_key(|(position, _)| *position);
        rows.into_iter().map(|(_, check)| check.clone()).collect()
    }

    /// ✂️ The rows of `subjects` in report order; the summary is left at its default because a fragment key does not cover it.
    pub fn slice(&self, subjects: &[String]) -> CheckReport {
        let wanted: BTreeSet<&String> = subjects.iter().collect();
        CheckReport { summary: CheckReportSummary::default(), checks: Self::ordered(wanted.into_iter().filter_map(|id| self.slots.get(id))) }
    }

    /// #️⃣ A stamp that moves exactly when the rows of `subjects` change.
    pub fn stamp(&self, subjects: &[String]) -> u64 {
        let parts: Vec<String> = subjects.iter().collect::<BTreeSet<_>>().into_iter().map(|id| format!("{id}={}", self.slots.get(id).map_or(0, |slot| slot.version))).collect();
        fnv64(parts.iter().map(String::as_str))
    }

    /// 🔢️ Moves whenever any row or the summary changed.
    pub fn version(&self) -> u64 {
        self.version
    }

    /// 📊️ The summary of the indexed report.
    pub fn summary(&self) -> &CheckReportSummary {
        &self.summary
    }

    /// 🧾️ The full report again, rows in report order.
    pub fn report(&self) -> CheckReport {
        CheckReport { summary: self.summary.clone(), checks: Self::ordered(self.slots.values()) }
    }
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
