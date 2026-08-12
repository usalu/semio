//! ⚙️ Architect program artifact engine — the `status_summary` topic.

//! 📊️ Status summary — aggregate lifecycle counts across registers.

use crate::artifacts::program::kernel::{EntityHeader, LifecycleStatus};
use crate::artifacts::program::ProgramSnapshot;
use crate::artifacts::program::registers::ValidationStatus;
use serde::{Deserialize, Serialize};

// #region 🔖️StatusSummary
/// @emoji 📈️ Aggregated status histogram across all header-bearing registers.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StatusSummary {
    pub total_entities: usize,
    pub by_status: Vec<(LifecycleStatus, usize)>,
    pub by_register: Vec<RegisterStatusCount>,
    pub compliance_status: Vec<(ValidationStatus, usize)>,
    pub validation_status: Vec<(ValidationStatus, usize)>,
    pub decision_status: Vec<(ValidationStatus, usize)>,
    pub action_status: Vec<(LifecycleStatus, usize)>,
}

/// @emoji 📁️ Per-register entity count and dominant status.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegisterStatusCount {
    pub register: String,
    pub count: usize,
    pub draft_count: usize,
    pub approved_count: usize,
}
// #endregion

// #region 🔖️Aggregate
fn bump_status(tallies: &mut Vec<(LifecycleStatus, usize)>, status: LifecycleStatus) {
    if let Some((_, count)) = tallies.iter_mut().find(|(s, _)| *s == status) {
        *count += 1;
    } else {
        tallies.push((status, 1));
    }
}

fn bump_validation(tallies: &mut Vec<(ValidationStatus, usize)>, status: ValidationStatus) {
    if let Some((_, count)) = tallies.iter_mut().find(|(s, _)| *s == status) {
        *count += 1;
    } else {
        tallies.push((status, 1));
    }
}

/// @emoji 🧮️ Aggregates lifecycle status counts from every program register collection.
pub fn status_summary(program: &ProgramSnapshot) -> StatusSummary {
    let mut tallies: Vec<(LifecycleStatus, usize)> = Vec::new();
    let mut registers = Vec::new();
    let mut total = 0usize;

    let mut collect = |name: &str, headers: Vec<&EntityHeader>| {
        let count = headers.len();
        total += count;
        let draft_count = headers.iter().filter(|h| h.status == LifecycleStatus::Draft).count();
        let approved_count = headers.iter().filter(|h| h.status == LifecycleStatus::Approved).count();
        for header in headers {
            bump_status(&mut tallies, header.status);
        }
        registers.push(RegisterStatusCount { register: name.into(), count, draft_count, approved_count });
    };

    collect("stakeholders", program.stakeholders.iter().map(|e| &e.header).collect());
    collect("users", program.users.iter().map(|e| &e.header).collect());
    collect("activities", program.activities.iter().map(|e| &e.header).collect());
    collect("functions", program.functions.iter().map(|e| &e.header).collect());
    collect("elements", program.elements.iter().map(|e| &e.header).collect());
    collect("quantities", program.quantities.iter().map(|e| &e.header).collect());
    collect("relationships", program.relationships.iter().map(|e| &e.header).collect());
    collect("adjacencies", program.adjacencies.iter().map(|e| &e.header).collect());
    collect("processes", program.processes.iter().map(|e| &e.header).collect());
    collect("flows", program.flows.iter().map(|e| &e.header).collect());
    collect("access_rules", program.access_rules.iter().map(|e| &e.header).collect());
    collect("operations", program.operations.iter().map(|e| &e.header).collect());
    collect("equipment", program.equipment.iter().map(|e| &e.header).collect());
    collect("resources", program.resources.iter().map(|e| &e.header).collect());
    collect("storage", program.storage.iter().map(|e| &e.header).collect());
    collect("environmental", program.environmental.iter().map(|e| &e.header).collect());
    collect("human_factors", program.human_factors.iter().map(|e| &e.header).collect());
    collect("accessibility", program.accessibility.iter().map(|e| &e.header).collect());
    collect("privacy", program.privacy.iter().map(|e| &e.header).collect());
    collect("safety", program.safety.iter().map(|e| &e.header).collect());
    collect("security", program.security.iter().map(|e| &e.header).collect());
    collect("regulatory", program.regulatory.iter().map(|e| &e.header).collect());
    collect("site_context", program.site_context.iter().map(|e| &e.header).collect());
    collect("organizational", program.organizational.iter().map(|e| &e.header).collect());
    collect("services", program.services.iter().map(|e| &e.header).collect());
    collect("infrastructure", program.infrastructure.iter().map(|e| &e.header).collect());
    collect("information", program.information.iter().map(|e| &e.header).collect());
    collect("communication", program.communication.iter().map(|e| &e.header).collect());
    collect("wayfinding", program.wayfinding.iter().map(|e| &e.header).collect());
    collect("schedules", program.schedules.iter().map(|e| &e.header).collect());
    collect("flexibility", program.flexibility.iter().map(|e| &e.header).collect());
    collect("growth", program.growth.iter().map(|e| &e.header).collect());
    collect("sustainability", program.sustainability.iter().map(|e| &e.header).collect());
    collect("resilience", program.resilience.iter().map(|e| &e.header).collect());
    collect("costs", program.costs.iter().map(|e| &e.header).collect());
    collect("delivery", program.delivery.iter().map(|e| &e.header).collect());
    collect("risks", program.risks.iter().map(|e| &e.header).collect());
    collect("conflicts", program.conflicts.iter().map(|e| &e.header).collect());
    collect("requirements", program.requirements.iter().map(|e| &e.header).collect());
    collect("priorities", program.priorities.iter().map(|e| &e.header).collect());
    collect("scenarios", program.scenarios.iter().map(|e| &e.header).collect());
    collect("options", program.options.iter().map(|e| &e.header).collect());
    collect("decisions", program.decisions.iter().map(|e| &e.header).collect());
    collect("validations", program.validations.iter().map(|e| &e.header).collect());
    collect("performance", program.performance.iter().map(|e| &e.header).collect());
    collect("quality", program.quality.iter().map(|e| &e.header).collect());
    collect("documents", program.artifacts.iter().map(|e| &e.header).collect());
    collect("changes", program.changes.iter().map(|e| &e.header).collect());
    collect("collaboration", program.collaboration.iter().map(|e| &e.header).collect());
    collect("analyses", program.analyses.iter().map(|e| &e.header).collect());
    collect("reports", program.reports.iter().map(|e| &e.header).collect());
    collect("search_filters", program.search_filters.iter().map(|e| &e.header).collect());
    collect("status_records", program.status_records.iter().map(|e| &e.header).collect());
    collect("workshops", program.workshops.iter().map(|e| &e.header).collect());
    collect("surveys", program.surveys.iter().map(|e| &e.header).collect());
    collect("issues", program.issues.iter().map(|e| &e.header).collect());
    collect("audit_events", program.audit_events.iter().map(|e| &e.header).collect());
    collect("templates", program.templates.iter().map(|e| &e.header).collect());
    collect("knowledge", program.knowledge.iter().map(|e| &e.header).collect());
    collect("benchmarks", program.benchmarks.iter().map(|e| &e.header).collect());

    let mut compliance_status = Vec::new();
    for item in &program.regulatory {
        bump_validation(&mut compliance_status, item.verification_status);
    }
    for item in &program.reports {
        bump_validation(&mut compliance_status, item.approval_status);
    }

    let mut validation_status = Vec::new();
    for item in &program.requirements {
        bump_validation(&mut validation_status, item.validation_status);
    }
    for item in &program.validations {
        bump_validation(&mut validation_status, item.result);
    }

    let mut decision_status = Vec::new();
    for item in &program.decisions {
        bump_validation(&mut decision_status, item.approval_status);
    }

    let mut action_status = Vec::new();
    for item in &program.status_records {
        bump_status(&mut action_status, item.record_status);
    }
    for item in &program.issues {
        bump_status(&mut action_status, item.header.status);
    }

    tallies.sort_by_key(|(status, _)| format!("{status:?}"));

    StatusSummary { total_entities: total, by_status: tallies, by_register: registers, compliance_status, validation_status, decision_status, action_status }
}
// #endregion

#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::program::sample_plugin;

    #[test]
    fn sample_plugin_status_summary_counts_elements() {
        let summary = status_summary(&sample_plugin());
        assert!(summary.total_entities >= 2);
        let elements = summary.by_register.iter().find(|r| r.register == "elements").expect("elements");
        assert_eq!(elements.count, 2);
    }

    #[test]
    fn status_summary_includes_all_major_registers() {
        let summary = status_summary(&sample_plugin());
        for register in ["elements", "stakeholders", "adjacencies", "status_records"] {
            assert!(summary.by_register.iter().any(|r| r.register == register));
        }
    }
}
