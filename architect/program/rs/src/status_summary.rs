//! 📊 Status summary — aggregate lifecycle counts across registers.

use crate::kernel::{EntityHeader, LifecycleStatus};
use crate::program::Program;
use serde::{Deserialize, Serialize};

// #region 🔖StatusSummary
/// @emoji 📈 Aggregated status histogram across all header-bearing registers.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StatusSummary {
    pub total_entities: usize,
    pub by_status: Vec<(LifecycleStatus, usize)>,
    pub by_register: Vec<RegisterStatusCount>,
}

/// @emoji 📁 Per-register entity count and dominant status.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegisterStatusCount {
    pub register: String,
    pub count: usize,
    pub draft_count: usize,
    pub approved_count: usize,
}
// #endregion

// #region 🔖Aggregate
/// @emoji 🧮 Aggregates lifecycle status counts from every major register collection.
pub fn status_summary(program: &Program) -> StatusSummary {
    let mut tallies: Vec<(LifecycleStatus, usize)> = Vec::new();
    let mut registers = Vec::new();

    let collections: Vec<(&str, Vec<&EntityHeader>)> = vec![
        (
            "stakeholders",
            program.stakeholders.iter().map(|e| &e.header).collect(),
        ),
        ("users", program.users.iter().map(|e| &e.header).collect()),
        ("activities", program.activities.iter().map(|e| &e.header).collect()),
        ("functions", program.functions.iter().map(|e| &e.header).collect()),
        ("elements", program.elements.iter().map(|e| &e.header).collect()),
        ("requirements", program.requirements.iter().map(|e| &e.header).collect()),
        ("adjacencies", program.adjacencies.iter().map(|e| &e.header).collect()),
        ("risks", program.risks.iter().map(|e| &e.header).collect()),
        ("issues", program.issues.iter().map(|e| &e.header).collect()),
    ];

    let mut total = 0usize;
    for (name, headers) in collections {
        let count = headers.len();
        total += count;
        let draft_count = headers
            .iter()
            .filter(|h| h.status == LifecycleStatus::Draft)
            .count();
        let approved_count = headers
            .iter()
            .filter(|h| h.status == LifecycleStatus::Approved)
            .count();
        for header in headers {
            bump_status(&mut tallies, header.status);
        }
        registers.push(RegisterStatusCount {
            register: name.into(),
            count,
            draft_count,
            approved_count,
        });
    }

    tallies.sort_by_key(|(status, _)| format!("{status:?}"));

    StatusSummary {
        total_entities: total,
        by_status: tallies,
        by_register: registers,
    }
}

fn bump_status(tallies: &mut Vec<(LifecycleStatus, usize)>, status: LifecycleStatus) {
    if let Some((_, count)) = tallies.iter_mut().find(|(s, _)| *s == status) {
        *count += 1;
    } else {
        tallies.push((status, 1));
    }
}
// #endregion

#[cfg(test)]
mod tests {
    use super::*;
    use crate::program::sample_program;

    #[test]
    fn sample_program_status_summary_counts_elements() {
        let summary = status_summary(&sample_program());
        assert!(summary.total_entities >= 2);
        let elements = summary.by_register.iter().find(|r| r.register == "elements").expect("elements");
        assert_eq!(elements.count, 2);
    }
}
