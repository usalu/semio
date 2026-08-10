//! ⚙️ Architect program artifact engine — the `analyze` topic.

//! 🔬️ ProgramSnapshot analysis — gap, conflict, dependency, capacity, and related kinds.

use crate::artifacts::program::engine::adjacency::detect_adjacency_conflicts;
use crate::artifacts::program::kernel::{DiagnosticSeverity, EntityHeader, EntityId, ProgramDiagnostic, TextField};
use crate::artifacts::program::ProgramSnapshot;
use crate::artifacts::program::registers::{AnalysisKind, AnalysisRecord, RelationshipKind, RiskLevel, ValidationStatus};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

// #region 🔖️AnalysisResult
/// @emoji 📈️ Structured output from `run_analysis`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AnalysisResult {
    pub kind: AnalysisKind,
    pub title: String,
    pub summary: String,
    pub findings: Vec<String>,
    pub metrics: Vec<AnalysisMetric>,
    pub diagnostics: Vec<ProgramDiagnostic>,
    pub entity_ids: Vec<EntityId>,
}

/// @emoji 📊️ Named numeric metric from an analysis run.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AnalysisMetric {
    pub name: String,
    pub value: f64,
    pub unit: Option<String>,
}
// #endregion

// #region 🔖️RunAnalysis
/// @emoji 🧮️ Runs the requested analysis kind over a plugin snapshot.
pub fn run_analysis(program: &ProgramSnapshot, kind: AnalysisKind) -> AnalysisResult {
    match kind {
        AnalysisKind::Gap => analyze_gap(program),
        AnalysisKind::Conflict => analyze_conflict(program),
        AnalysisKind::Dependency => analyze_dependency(program),
        AnalysisKind::Capacity => analyze_capacity(program),
        AnalysisKind::Demand => analyze_demand(program),
        AnalysisKind::Utilization => analyze_utilization(program),
        AnalysisKind::Workflow => analyze_workflow(program),
        AnalysisKind::Risk => analyze_risk(program),
        AnalysisKind::Cost => analyze_cost(program),
        AnalysisKind::Scenario => analyze_scenario(program),
        AnalysisKind::Sensitivity => analyze_sensitivity(program),
        AnalysisKind::Impact => analyze_impact(program),
        AnalysisKind::Trend => analyze_trend(program),
        AnalysisKind::RequirementComparison => analyze_requirement_comparison(program),
        AnalysisKind::RequirementClustering => analyze_requirement_clustering(program),
        AnalysisKind::RequirementFiltering => analyze_requirement_filtering(program),
        AnalysisKind::RequirementSorting => analyze_requirement_sorting(program),
        AnalysisKind::RequirementScoring => analyze_requirement_scoring(program),
        AnalysisKind::RequirementWeighting => analyze_requirement_weighting(program),
        AnalysisKind::RelationshipAnalysis => analyze_relationship(program),
    }
}

/// @emoji 📝️ Runs analysis and appends an `AnalysisRecord` to the program.
pub fn run_analysis_and_record(program: &mut ProgramSnapshot, kind: AnalysisKind) -> AnalysisResult {
    let result = run_analysis(program, kind);
    let record = AnalysisRecord {
        header: EntityHeader::new(EntityId::new_serial("analysis", "analysis"), result.title.clone()),
        kind,
        title: result.title.clone(),
        parameters: Vec::new(),
        input_entity_ids: result.entity_ids.clone(),
        output_summary: TextField::plain(&result.summary),
        findings: result.findings.clone(),
        metrics: result.metrics.iter().map(|m| format!("{}={}{}", m.name, m.value, m.unit.as_deref().unwrap_or(""))).collect(),
        charts: Vec::new(),
        run_by: None,
        run_at: Some(program.meta.timestamps.updated.clone()),
        duration_ms: None,
        tool_version: None,
        scenario_id: None,
        report_id: None,
        confidence: None,
        limitations: Vec::new(),
        recommendations: result.findings.clone(),
        raw_result_ref: None,
    };
    program.analyses.push(record);
    result
}

fn analyze_gap(program: &ProgramSnapshot) -> AnalysisResult {
    let mut findings = Vec::new();
    if program.requirements.is_empty() {
        findings.push("no requirements registered".into());
    }
    if program.elements.is_empty() {
        findings.push("no program elements defined".into());
    }
    let unlinked: Vec<_> = program.requirements.iter().filter(|req| req.element_ids.is_empty() && req.function_ids.is_empty()).map(|req| req.header.id.clone()).collect();
    for id in &unlinked {
        findings.push(format!("requirement {id} is not linked to elements or functions"));
    }
    let elements_without_functions: Vec<_> = program.elements.iter().filter(|e| e.function_ids.is_empty()).map(|e| e.header.id.clone()).collect();
    for id in &elements_without_functions {
        findings.push(format!("element {id} has no assigned functions"));
    }
    AnalysisResult {
        kind: AnalysisKind::Gap,
        title: "Gap Analysis".into(),
        summary: format!("{} gap finding(s)", findings.len()),
        findings,
        metrics: vec![AnalysisMetric { name: "unlinked_requirements".into(), value: unlinked.len() as f64, unit: None }, AnalysisMetric { name: "elements_without_functions".into(), value: elements_without_functions.len() as f64, unit: None }],
        diagnostics: Vec::new(),
        entity_ids: unlinked,
    }
}

fn analyze_conflict(program: &ProgramSnapshot) -> AnalysisResult {
    let adjacency_conflicts = detect_adjacency_conflicts(program);
    let mut findings: Vec<String> = adjacency_conflicts.iter().map(|c| format!("{}: {}", c.adjacency_a_id, c.message)).collect();
    findings.extend(program.conflicts.iter().map(|c| format!("{} — {:?} between {} and {}", c.header.name, c.kind, c.entity_a_id, c.entity_b_id)));
    let open_conflicts = program.conflicts.iter().filter(|c| c.resolution_status != ValidationStatus::Passed).count();
    let findings_len = findings.len();
    AnalysisResult {
        kind: AnalysisKind::Conflict,
        title: "Conflict Analysis".into(),
        summary: format!("{findings_len} conflict(s) detected, {open_conflicts} unresolved"),
        findings,
        metrics: vec![AnalysisMetric { name: "total_conflicts".into(), value: findings_len as f64, unit: None }, AnalysisMetric { name: "open_conflicts".into(), value: open_conflicts as f64, unit: None }],
        diagnostics: adjacency_conflicts
            .into_iter()
            .map(|c| ProgramDiagnostic { severity: DiagnosticSeverity::Error, code: "analysis.conflict".into(), message: c.message, entity_id: Some(c.adjacency_a_id), register: Some("adjacencies".into()) })
            .collect(),
        entity_ids: Vec::new(),
    }
}

fn analyze_dependency(program: &ProgramSnapshot) -> AnalysisResult {
    let depends: Vec<String> = program.relationships.iter().filter(|r| matches!(r.kind, RelationshipKind::DependsOn)).map(|r| format!("{} depends on {}", r.source_id, r.target_id)).collect();
    let process_deps: usize = program.processes.iter().map(|p| p.dependencies.len()).sum();
    AnalysisResult {
        kind: AnalysisKind::Dependency,
        title: "Dependency Analysis".into(),
        summary: format!("{} relationship deps, {process_deps} process deps", depends.len()),
        findings: depends,
        metrics: vec![AnalysisMetric { name: "relationship_count".into(), value: program.relationships.len() as f64, unit: None }, AnalysisMetric { name: "process_dependency_count".into(), value: process_deps as f64, unit: None }],
        diagnostics: Vec::new(),
        entity_ids: Vec::new(),
    }
}

fn analyze_capacity(program: &ProgramSnapshot) -> AnalysisResult {
    let total_area: f64 = program.elements.iter().filter_map(|e| e.area.target).sum();
    let total_occupancy: f64 = program.elements.iter().filter_map(|e| e.occupancy.target.or(e.occupancy.peak)).sum();
    let area_per_person = if total_occupancy > 0.0 { total_area / total_occupancy } else { 0.0 };
    AnalysisResult {
        kind: AnalysisKind::Capacity,
        title: "Capacity Analysis".into(),
        summary: format!("total target area {total_area:.1} m², {total_occupancy:.0} persons, {:.1} m²/person", area_per_person),
        findings: program.elements.iter().filter_map(|e| e.area.target.map(|a| format!("{}: {a:.1} m²", e.header.name))).collect(),
        metrics: vec![
            AnalysisMetric { name: "element_count".into(), value: program.elements.len() as f64, unit: None },
            AnalysisMetric { name: "total_target_area".into(), value: total_area, unit: Some("m2".into()) },
            AnalysisMetric { name: "area_per_person".into(), value: area_per_person, unit: Some("m2/person".into()) },
        ],
        diagnostics: Vec::new(),
        entity_ids: Vec::new(),
    }
}

fn analyze_demand(program: &ProgramSnapshot) -> AnalysisResult {
    let peak_occupancy: f64 = program.elements.iter().filter_map(|e| e.occupancy.peak.or(e.occupancy.target)).sum();
    let schedule_demand = program.schedules.len();
    AnalysisResult {
        kind: AnalysisKind::Demand,
        title: "Demand Analysis".into(),
        summary: format!("aggregate peak/target occupancy {peak_occupancy:.0}, {schedule_demand} schedule constraints"),
        findings: program.schedules.iter().map(|s| format!("schedule {} — {:?}", s.header.name, s.header.status)).collect(),
        metrics: vec![AnalysisMetric { name: "peak_occupancy".into(), value: peak_occupancy, unit: Some("persons".into()) }],
        diagnostics: Vec::new(),
        entity_ids: Vec::new(),
    }
}

fn analyze_utilization(program: &ProgramSnapshot) -> AnalysisResult {
    let activities = program.activities.len();
    let elements = program.elements.len();
    let ratio = if elements == 0 { 0.0 } else { activities as f64 / elements as f64 };
    let equipped = program.equipment.iter().filter(|e| !e.element_ids.is_empty()).count();
    AnalysisResult {
        kind: AnalysisKind::Utilization,
        title: "Utilization Analysis".into(),
        summary: format!("activity/element ratio {ratio:.2}, {equipped} equipment placements"),
        findings: program.equipment.iter().map(|e| format!("{} serves {} element(s)", e.header.name, e.element_ids.len())).collect(),
        metrics: vec![AnalysisMetric { name: "activity_element_ratio".into(), value: ratio, unit: None }],
        diagnostics: Vec::new(),
        entity_ids: Vec::new(),
    }
}

fn analyze_workflow(program: &ProgramSnapshot) -> AnalysisResult {
    let critical: Vec<_> = program.processes.iter().filter(|p| p.critical_path).collect();
    AnalysisResult {
        kind: AnalysisKind::Workflow,
        title: "Workflow Analysis".into(),
        summary: format!("{} processes ({} critical), {} flows", program.processes.len(), critical.len(), program.flows.len()),
        findings: program.processes.iter().map(|p| format!("{} — {} steps, {} actors", p.header.name, p.steps.len(), p.actors.len())).collect(),
        metrics: vec![AnalysisMetric { name: "critical_path_processes".into(), value: critical.len() as f64, unit: None }],
        diagnostics: Vec::new(),
        entity_ids: critical.iter().map(|p| p.header.id.clone()).collect(),
    }
}

fn analyze_risk(program: &ProgramSnapshot) -> AnalysisResult {
    let high: Vec<_> = program.risks.iter().filter(|r| matches!(r.probability, RiskLevel::High | RiskLevel::Critical) || matches!(r.impact, RiskLevel::High | RiskLevel::Critical)).map(|r| r.header.id.clone()).collect();
    let score_sum: f64 = program.risks.iter().map(|r| risk_score(&r.probability) * risk_score(&r.impact)).sum();
    AnalysisResult {
        kind: AnalysisKind::Risk,
        title: "Risk Analysis".into(),
        summary: format!("{} high/critical risk(s), aggregate score {score_sum:.0}", high.len()),
        findings: high.iter().map(|id| format!("risk {id}")).collect(),
        metrics: vec![AnalysisMetric { name: "risk_count".into(), value: program.risks.len() as f64, unit: None }, AnalysisMetric { name: "aggregate_risk_score".into(), value: score_sum, unit: None }],
        diagnostics: Vec::new(),
        entity_ids: high,
    }
}

fn risk_score(level: &RiskLevel) -> f64 {
    match level {
        RiskLevel::Negligible => 0.5,
        RiskLevel::Low => 1.0,
        RiskLevel::Medium => 2.0,
        RiskLevel::High => 3.0,
        RiskLevel::Critical => 4.0,
    }
}

fn analyze_cost(program: &ProgramSnapshot) -> AnalysisResult {
    let total_capital: f64 = program.costs.iter().filter_map(|c| c.amount).sum();
    AnalysisResult {
        kind: AnalysisKind::Cost,
        title: "Cost Analysis".into(),
        summary: format!("{} cost requirements, capital total {total_capital:.0}", program.costs.len()),
        findings: program.costs.iter().filter_map(|c| c.amount.map(|v| format!("{}: {v:.0}", c.header.name))).collect(),
        metrics: vec![AnalysisMetric { name: "capital_cost_total".into(), value: total_capital, unit: Some("currency".into()) }],
        diagnostics: Vec::new(),
        entity_ids: Vec::new(),
    }
}

fn analyze_scenario(program: &ProgramSnapshot) -> AnalysisResult {
    let evaluated = program.options.iter().filter(|o| o.evaluation_status == ValidationStatus::Passed).count();
    AnalysisResult {
        kind: AnalysisKind::Scenario,
        title: "Scenario Analysis".into(),
        summary: format!("{} scenario(s), {} options ({} selected)", program.scenarios.len(), program.options.len(), evaluated),
        findings: program.scenarios.iter().map(|s| s.header.name.clone()).collect(),
        metrics: vec![AnalysisMetric { name: "selected_options".into(), value: evaluated as f64, unit: None }],
        diagnostics: Vec::new(),
        entity_ids: program.scenarios.iter().map(|s| s.header.id.clone()).collect(),
    }
}

fn analyze_sensitivity(program: &ProgramSnapshot) -> AnalysisResult {
    let mandatory = program.requirements.iter().filter(|r| r.header.priority == crate::artifacts::program::kernel::Priority::Mandatory).count();
    AnalysisResult {
        kind: AnalysisKind::Sensitivity,
        title: "Sensitivity Analysis".into(),
        summary: format!("{mandatory} mandatory requirements drive sensitivity"),
        findings: program.priorities.iter().map(|p| format!("{} — {:?}", p.header.name, p.header.priority)).collect(),
        metrics: vec![AnalysisMetric { name: "mandatory_requirement_count".into(), value: mandatory as f64, unit: None }],
        diagnostics: Vec::new(),
        entity_ids: Vec::new(),
    }
}

fn analyze_impact(program: &ProgramSnapshot) -> AnalysisResult {
    let impacted: usize = program.decisions.iter().map(|d| d.impacted_requirement_ids.len() + d.impacted_element_ids.len()).sum();
    AnalysisResult {
        kind: AnalysisKind::Impact,
        title: "Impact Analysis".into(),
        summary: format!("{} decision(s) touching {impacted} requirement/element links", program.decisions.len()),
        findings: program.decisions.iter().map(|d| format!("{} impacts {} requirements", d.header.name, d.impacted_requirement_ids.len())).collect(),
        metrics: vec![AnalysisMetric { name: "impacted_links".into(), value: impacted as f64, unit: None }],
        diagnostics: Vec::new(),
        entity_ids: program.decisions.iter().map(|d| d.header.id.clone()).collect(),
    }
}

fn analyze_trend(program: &ProgramSnapshot) -> AnalysisResult {
    let change_velocity = program.changes.len();
    AnalysisResult {
        kind: AnalysisKind::Trend,
        title: "Trend Analysis".into(),
        summary: format!("{} analysis records, {} changes, {} audit events", program.analyses.len(), change_velocity, program.audit_events.len()),
        findings: program.changes.iter().take(5).map(|c| format!("change {} — {}", c.header.name, c.header.timestamps.updated)).collect(),
        metrics: vec![AnalysisMetric { name: "change_count".into(), value: change_velocity as f64, unit: None }],
        diagnostics: Vec::new(),
        entity_ids: Vec::new(),
    }
}

fn analyze_requirement_comparison(program: &ProgramSnapshot) -> AnalysisResult {
    let mut by_kind: HashMap<String, usize> = HashMap::new();
    for req in &program.requirements {
        *by_kind.entry(format!("{:?}", req.kind)).or_default() += 1;
    }
    let findings: Vec<String> = by_kind.iter().map(|(kind, count)| format!("{kind}: {count} requirement(s)")).collect();
    AnalysisResult {
        kind: AnalysisKind::RequirementComparison,
        title: "Requirement Comparison".into(),
        summary: format!("{} requirement kinds compared across {} items", by_kind.len(), program.requirements.len()),
        findings,
        metrics: vec![AnalysisMetric { name: "requirement_kind_count".into(), value: by_kind.len() as f64, unit: None }],
        diagnostics: Vec::new(),
        entity_ids: program.requirements.iter().map(|r| r.header.id.clone()).collect(),
    }
}

fn analyze_requirement_clustering(program: &ProgramSnapshot) -> AnalysisResult {
    let mut clusters: HashMap<String, Vec<EntityId>> = HashMap::new();
    for req in &program.requirements {
        let key = format!("{:?}-{:?}", req.kind, req.header.priority);
        clusters.entry(key).or_default().push(req.header.id.clone());
    }
    let findings: Vec<String> = clusters.iter().map(|(key, ids)| format!("cluster {key}: {} requirement(s)", ids.len())).collect();
    AnalysisResult {
        kind: AnalysisKind::RequirementClustering,
        title: "Requirement Clustering".into(),
        summary: format!("{} clusters from {} requirements", clusters.len(), program.requirements.len()),
        findings,
        metrics: vec![AnalysisMetric { name: "cluster_count".into(), value: clusters.len() as f64, unit: None }],
        diagnostics: Vec::new(),
        entity_ids: clusters.values().flatten().cloned().collect(),
    }
}

fn analyze_requirement_filtering(program: &ProgramSnapshot) -> AnalysisResult {
    let pending: Vec<_> = program.requirements.iter().filter(|r| r.validation_status == ValidationStatus::Pending).map(|r| r.header.id.clone()).collect();
    let findings: Vec<String> = pending.iter().map(|id| format!("pending validation: {id}")).collect();
    AnalysisResult {
        kind: AnalysisKind::RequirementFiltering,
        title: "Requirement Filtering".into(),
        summary: format!("{} pending of {} total requirements", pending.len(), program.requirements.len()),
        findings,
        metrics: vec![AnalysisMetric { name: "pending_validation_count".into(), value: pending.len() as f64, unit: None }],
        diagnostics: Vec::new(),
        entity_ids: pending,
    }
}

fn analyze_requirement_sorting(program: &ProgramSnapshot) -> AnalysisResult {
    let mut sorted: Vec<_> = program.requirements.iter().collect();
    sorted.sort_by_key(|r| r.header.priority);
    let findings: Vec<String> = sorted.iter().map(|r| format!("{:?} — {}", r.header.priority, r.header.name)).collect();
    AnalysisResult {
        kind: AnalysisKind::RequirementSorting,
        title: "Requirement Sorting".into(),
        summary: format!("{} requirements sorted by priority", sorted.len()),
        findings,
        metrics: vec![AnalysisMetric { name: "sorted_requirement_count".into(), value: sorted.len() as f64, unit: None }],
        diagnostics: Vec::new(),
        entity_ids: sorted.iter().map(|r| r.header.id.clone()).collect(),
    }
}

fn priority_weight(priority: &crate::artifacts::program::kernel::Priority) -> f64 {
    match priority {
        crate::artifacts::program::kernel::Priority::Mandatory => 5.0,
        crate::artifacts::program::kernel::Priority::Essential => 4.0,
        crate::artifacts::program::kernel::Priority::Preferred => 3.0,
        crate::artifacts::program::kernel::Priority::Optional => 2.0,
        crate::artifacts::program::kernel::Priority::Deferred => 1.0,
        crate::artifacts::program::kernel::Priority::Prohibited => 0.0,
    }
}

fn analyze_requirement_scoring(program: &ProgramSnapshot) -> AnalysisResult {
    let mut scored: Vec<(EntityId, f64)> = program
        .requirements
        .iter()
        .map(|r| {
            let base = priority_weight(&r.header.priority);
            let validation = match r.validation_status {
                ValidationStatus::Passed => 1.0,
                ValidationStatus::Pending => 0.5,
                ValidationStatus::Failed => 0.0,
                ValidationStatus::Waived => 0.25,
                ValidationStatus::Deferred => 0.1,
            };
            (r.header.id.clone(), base * validation)
        })
        .collect();
    scored.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
    let findings: Vec<String> = scored.iter().take(10).map(|(id, score)| format!("{id}: score {score:.2}")).collect();
    let total: f64 = scored.iter().map(|(_, s)| s).sum();
    AnalysisResult {
        kind: AnalysisKind::RequirementScoring,
        title: "Requirement Scoring".into(),
        summary: format!("scored {} requirements, total {total:.1}", scored.len()),
        findings,
        metrics: vec![AnalysisMetric { name: "total_requirement_score".into(), value: total, unit: None }],
        diagnostics: Vec::new(),
        entity_ids: scored.into_iter().map(|(id, _)| id).collect(),
    }
}

fn analyze_requirement_weighting(program: &ProgramSnapshot) -> AnalysisResult {
    let mut weights: HashMap<EntityId, f64> = HashMap::new();
    for record in &program.priorities {
        if let Some(weight) = record.weight {
            weights.insert(record.subject_id.clone(), weight);
        }
    }
    let findings: Vec<String> = weights.iter().map(|(id, w)| format!("{id}: weight {w:.2}")).collect();
    let avg = if weights.is_empty() { 0.0 } else { weights.values().sum::<f64>() / weights.len() as f64 };
    AnalysisResult {
        kind: AnalysisKind::RequirementWeighting,
        title: "Requirement Weighting".into(),
        summary: format!("{} weighted subjects, average weight {avg:.2}", weights.len()),
        findings,
        metrics: vec![AnalysisMetric { name: "average_weight".into(), value: avg, unit: None }],
        diagnostics: Vec::new(),
        entity_ids: weights.keys().cloned().collect(),
    }
}

fn analyze_relationship(program: &ProgramSnapshot) -> AnalysisResult {
    let mut nodes: HashSet<EntityId> = HashSet::new();
    for rel in &program.relationships {
        nodes.insert(rel.source_id.clone());
        nodes.insert(rel.target_id.clone());
    }
    let depends = program.relationships.iter().filter(|r| matches!(r.kind, RelationshipKind::DependsOn)).count();
    let conflicts = program.relationships.iter().filter(|r| matches!(r.kind, RelationshipKind::ConflictsWith)).count();
    AnalysisResult {
        kind: AnalysisKind::RelationshipAnalysis,
        title: "Relationship Analysis".into(),
        summary: format!("{} relationships across {} nodes ({} depends, {} conflicts)", program.relationships.len(), nodes.len(), depends, conflicts),
        findings: program.relationships.iter().map(|r| format!("{:?}: {} → {}", r.kind, r.source_id, r.target_id)).collect(),
        metrics: vec![AnalysisMetric { name: "relationship_node_count".into(), value: nodes.len() as f64, unit: None }, AnalysisMetric { name: "dependency_edge_count".into(), value: depends as f64, unit: None }],
        diagnostics: Vec::new(),
        entity_ids: nodes.into_iter().collect(),
    }
}
// #endregion

#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::program::sample_plugin;

    #[test]
    fn gap_analysis_on_sample_plugin() {
        let result = run_analysis(&sample_plugin(), AnalysisKind::Gap);
        assert_eq!(result.kind, AnalysisKind::Gap);
        assert!(!result.findings.is_empty());
    }

    #[test]
    fn capacity_analysis_sums_area() {
        let result = run_analysis(&sample_plugin(), AnalysisKind::Capacity);
        assert!(result.metrics.iter().any(|m| m.name == "total_target_area"));
        assert!(result.metrics.iter().any(|m| m.value > 0.0));
    }

    #[test]
    fn run_analysis_and_record_persists() {
        let mut program = sample_plugin();
        let before = program.analyses.len();
        run_analysis_and_record(&mut program, AnalysisKind::Risk);
        assert_eq!(program.analyses.len(), before + 1);
    }

    #[test]
    fn requirement_clustering_produces_clusters() {
        let result = run_analysis(&sample_plugin(), AnalysisKind::RequirementClustering);
        assert_eq!(result.kind, AnalysisKind::RequirementClustering);
    }
}