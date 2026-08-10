//! ⚙️ Architect program artifact engine — the `outputs` topic.

//! 📤️ Abstract output types — §65 builders for program deliverables.

use crate::artifacts::program::engine::analyze::run_analysis;
use crate::artifacts::program::kernel::EntityId;
use crate::artifacts::program::ProgramSnapshot;
use crate::artifacts::program::registers::{AnalysisKind, ReportKind};
use crate::artifacts::program::engine::report::build_report;
use serde::{Deserialize, Serialize};

// #region 🔖️OutputKind
/// @emoji 📦️ Abstract output kind for architectural program deliverables.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum OutputKind {
    RequirementLists,
    FunctionalHierarchies,
    ActivityTaxonomies,
    RelationshipMatrices,
    AdjacencyMatrices,
    DependencyNetworks,
    PriorityMatrices,
    ResponsibilityMatrices,
    DecisionTrees,
    ProcessMaps,
    WorkflowDescriptions,
    UserJourneys,
    ScenarioNarratives,
    RiskMatrices,
    ComplianceMatrices,
    CapacitySchedules,
    EquipmentSchedules,
    EvaluationFrameworks,
    PerformanceSpecifications,
    ProgramReports,
}

/// @emoji 📄️ Structured abstract output payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProgramOutput {
    pub kind: OutputKind,
    pub title: String,
    pub lines: Vec<String>,
    pub entity_ids: Vec<EntityId>,
}
// #endregion

// #region 🔖️Builders
/// @emoji 🏗️ Builds the requested abstract output from a plugin snapshot.
pub fn build_output(program: &ProgramSnapshot, kind: OutputKind) -> ProgramOutput {
    match kind {
        OutputKind::RequirementLists => requirement_lists(program),
        OutputKind::FunctionalHierarchies => functional_hierarchies(program),
        OutputKind::ActivityTaxonomies => activity_taxonomies(program),
        OutputKind::RelationshipMatrices => relationship_matrices(program),
        OutputKind::AdjacencyMatrices => adjacency_matrices(program),
        OutputKind::DependencyNetworks => dependency_networks(program),
        OutputKind::PriorityMatrices => priority_matrices(program),
        OutputKind::ResponsibilityMatrices => responsibility_matrices(program),
        OutputKind::DecisionTrees => decision_trees(program),
        OutputKind::ProcessMaps => process_maps(program),
        OutputKind::WorkflowDescriptions => workflow_descriptions(program),
        OutputKind::UserJourneys => user_journeys(program),
        OutputKind::ScenarioNarratives => scenario_narratives(program),
        OutputKind::RiskMatrices => risk_matrices(program),
        OutputKind::ComplianceMatrices => compliance_matrices(program),
        OutputKind::CapacitySchedules => capacity_schedules(program),
        OutputKind::EquipmentSchedules => equipment_schedules(program),
        OutputKind::EvaluationFrameworks => evaluation_frameworks(program),
        OutputKind::PerformanceSpecifications => performance_specifications(program),
        OutputKind::ProgramReports => program_reports(program),
    }
}

fn requirement_lists(program: &ProgramSnapshot) -> ProgramOutput {
    ProgramOutput {
        kind: OutputKind::RequirementLists,
        title: "Requirement Lists".into(),
        lines: program.requirements.iter().map(|r| format!("[{:?}] {} — {}", r.kind, r.header.name, r.statement.text)).collect(),
        entity_ids: program.requirements.iter().map(|r| r.header.id.clone()).collect(),
    }
}

fn functional_hierarchies(program: &ProgramSnapshot) -> ProgramOutput {
    let roots: Vec<_> = program.functions.iter().filter(|f| f.hierarchy_parent_id.is_none()).collect();
    let mut lines = Vec::new();
    for root in roots {
        lines.push(root.header.name.clone());
        for child in program.functions.iter().filter(|f| f.hierarchy_parent_id.as_ref() == Some(&root.header.id)) {
            lines.push(format!("  └️─️ {}", child.header.name));
        }
    }
    ProgramOutput { kind: OutputKind::FunctionalHierarchies, title: "Functional Hierarchies".into(), lines, entity_ids: program.functions.iter().map(|f| f.header.id.clone()).collect() }
}

fn activity_taxonomies(program: &ProgramSnapshot) -> ProgramOutput {
    let mut lines = Vec::new();
    for activity in &program.activities {
        lines.push(format!("{} / {} / {}", activity.category, activity.activity_type, activity.header.name));
    }
    ProgramOutput { kind: OutputKind::ActivityTaxonomies, title: "Activity Taxonomies".into(), lines, entity_ids: program.activities.iter().map(|a| a.header.id.clone()).collect() }
}

fn relationship_matrices(program: &ProgramSnapshot) -> ProgramOutput {
    let lines: Vec<String> = program.relationships.iter().map(|r| format!("{:?}: {} → {}", r.kind, r.source_id, r.target_id)).collect();
    ProgramOutput { kind: OutputKind::RelationshipMatrices, title: "Relationship Matrices".into(), lines, entity_ids: program.relationships.iter().map(|r| r.header.id.clone()).collect() }
}

fn adjacency_matrices(program: &ProgramSnapshot) -> ProgramOutput {
    let report = build_report(program, ReportKind::AdjacencyMatrix);
    ProgramOutput { kind: OutputKind::AdjacencyMatrices, title: "Adjacency Matrices".into(), lines: report.sections.into_iter().flat_map(|s| s.bullets).collect(), entity_ids: report.entity_ids }
}

fn dependency_networks(program: &ProgramSnapshot) -> ProgramOutput {
    let analysis = run_analysis(program, AnalysisKind::Dependency);
    ProgramOutput { kind: OutputKind::DependencyNetworks, title: "Dependency Networks".into(), lines: analysis.findings, entity_ids: analysis.entity_ids }
}

fn priority_matrices(program: &ProgramSnapshot) -> ProgramOutput {
    let lines: Vec<String> = program.priorities.iter().map(|p| format!("{} — {:?} rank {:?} weight {:?}", p.header.name, p.ranked_priority, p.rank, p.weight)).collect();
    ProgramOutput { kind: OutputKind::PriorityMatrices, title: "Priority Matrices".into(), lines, entity_ids: program.priorities.iter().map(|p| p.header.id.clone()).collect() }
}

fn responsibility_matrices(program: &ProgramSnapshot) -> ProgramOutput {
    let lines: Vec<String> = program.governance.responsibilities.iter().chain(program.governance.roles.iter()).cloned().collect();
    ProgramOutput { kind: OutputKind::ResponsibilityMatrices, title: "Responsibility Matrices".into(), lines, entity_ids: vec![program.governance.id.clone()] }
}

fn decision_trees(program: &ProgramSnapshot) -> ProgramOutput {
    let lines: Vec<String> = program.decisions.iter().map(|d| format!("{} → option {:?} ({})", d.header.name, d.selected_option_id, d.decision_statement.text)).collect();
    ProgramOutput { kind: OutputKind::DecisionTrees, title: "Decision Trees".into(), lines, entity_ids: program.decisions.iter().map(|d| d.header.id.clone()).collect() }
}

fn process_maps(program: &ProgramSnapshot) -> ProgramOutput {
    let lines: Vec<String> = program.processes.iter().map(|p| format!("{}: {}", p.header.name, p.steps.join(" → "))).collect();
    ProgramOutput { kind: OutputKind::ProcessMaps, title: "Process Maps".into(), lines, entity_ids: program.processes.iter().map(|p| p.header.id.clone()).collect() }
}

fn workflow_descriptions(program: &ProgramSnapshot) -> ProgramOutput {
    let analysis = run_analysis(program, AnalysisKind::Workflow);
    ProgramOutput { kind: OutputKind::WorkflowDescriptions, title: "Workflow Descriptions".into(), lines: analysis.findings, entity_ids: analysis.entity_ids }
}

fn user_journeys(program: &ProgramSnapshot) -> ProgramOutput {
    let mut lines = Vec::new();
    for user in &program.users {
        let activities: Vec<_> = program.activities.iter().filter(|a| a.user_profile_ids.contains(&user.header.id)).map(|a| a.header.name.as_str()).collect();
        lines.push(format!("{}: {}", user.header.name, activities.join(" → ")));
    }
    ProgramOutput { kind: OutputKind::UserJourneys, title: "User Journeys".into(), lines, entity_ids: program.users.iter().map(|u| u.header.id.clone()).collect() }
}

fn scenario_narratives(program: &ProgramSnapshot) -> ProgramOutput {
    let lines: Vec<String> = program.scenarios.iter().map(|s| format!("{} — {}", s.header.name, s.hypothesis.text)).collect();
    ProgramOutput { kind: OutputKind::ScenarioNarratives, title: "Scenario Narratives".into(), lines, entity_ids: program.scenarios.iter().map(|s| s.header.id.clone()).collect() }
}

fn risk_matrices(program: &ProgramSnapshot) -> ProgramOutput {
    let lines: Vec<String> = program.risks.iter().map(|r| format!("{} — {:?}/{:?}", r.header.name, r.probability, r.impact)).collect();
    ProgramOutput { kind: OutputKind::RiskMatrices, title: "Risk Matrices".into(), lines, entity_ids: program.risks.iter().map(|r| r.header.id.clone()).collect() }
}

fn compliance_matrices(program: &ProgramSnapshot) -> ProgramOutput {
    let lines: Vec<String> = program.regulatory.iter().map(|r| format!("{} {} — {:?}", r.code, r.title, r.verification_status)).collect();
    ProgramOutput { kind: OutputKind::ComplianceMatrices, title: "Compliance Matrices".into(), lines, entity_ids: program.regulatory.iter().map(|r| r.header.id.clone()).collect() }
}

fn capacity_schedules(program: &ProgramSnapshot) -> ProgramOutput {
    let analysis = run_analysis(program, AnalysisKind::Capacity);
    let schedule_lines: Vec<String> = program.schedules.iter().map(|s| s.header.name.clone()).collect();
    ProgramOutput { kind: OutputKind::CapacitySchedules, title: "Capacity Schedules".into(), lines: analysis.findings.into_iter().chain(schedule_lines).collect(), entity_ids: program.schedules.iter().map(|s| s.header.id.clone()).collect() }
}

fn equipment_schedules(program: &ProgramSnapshot) -> ProgramOutput {
    let lines: Vec<String> = program.equipment.iter().map(|e| format!("{} — qty {:?}", e.header.name, e.quantity.target)).collect();
    ProgramOutput { kind: OutputKind::EquipmentSchedules, title: "Equipment Schedules".into(), lines, entity_ids: program.equipment.iter().map(|e| e.header.id.clone()).collect() }
}

fn evaluation_frameworks(program: &ProgramSnapshot) -> ProgramOutput {
    let lines: Vec<String> = program.performance.iter().map(|p| format!("{} — {}", p.header.name, p.criterion)).collect();
    ProgramOutput { kind: OutputKind::EvaluationFrameworks, title: "Evaluation Frameworks".into(), lines, entity_ids: program.performance.iter().map(|p| p.header.id.clone()).collect() }
}

fn performance_specifications(program: &ProgramSnapshot) -> ProgramOutput {
    let lines: Vec<String> = program.performance.iter().map(|p| format!("{} target {:?} {:?}", p.header.name, p.target, p.unit)).collect();
    ProgramOutput { kind: OutputKind::PerformanceSpecifications, title: "Performance Specifications".into(), lines, entity_ids: program.performance.iter().map(|p| p.header.id.clone()).collect() }
}

fn program_reports(program: &ProgramSnapshot) -> ProgramOutput {
    let lines: Vec<String> = program.reports.iter().map(|r| format!("{:?} — {}", r.kind, r.title)).collect();
    ProgramOutput { kind: OutputKind::ProgramReports, title: "ProgramSnapshot Reports".into(), lines, entity_ids: program.reports.iter().map(|r| r.header.id.clone()).collect() }
}
// #endregion

#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::program::sample_plugin;

    #[test]
    fn requirement_lists_output_nonempty_for_sample() {
        let output = build_output(&sample_plugin(), OutputKind::RequirementLists);
        assert_eq!(output.kind, OutputKind::RequirementLists);
    }

    #[test]
    fn adjacency_matrices_output_uses_matrix_cells() {
        let output = build_output(&sample_plugin(), OutputKind::AdjacencyMatrices);
        assert!(!output.lines.is_empty());
    }
}