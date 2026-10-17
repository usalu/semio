//! 🎁️ Program outputs (requirement lists, matrices, maps, schedules) and the structured program reports.

use super::*;

//#region 🎁️Outputs
/// 📦️ Abstract output kind for architectural program deliverables.
#[derive(Clone, Copy, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
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

/// 📄️ Structured abstract output payload.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct ProgramOutput {
    pub kind: OutputKind,
    pub title: String,
    pub lines: Vec<String>,
    pub entity_ids: Vec<EntityId>,
}

/// 🏗️ Builds the requested abstract output from a plugin snapshot.
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
//#endregion 🎁️Outputs
//#region 📄️Report
/// 📑️ Structured report payload for export and program rendering.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct ProgramReport {
    pub kind: ReportKind,
    pub title: String,
    pub generated_at: String,
    pub sections: Vec<ReportSection>,
    pub entity_ids: Vec<EntityId>,
}

/// 📎️ One titled section within a plugin report.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct ReportSection {
    pub heading: String,
    pub body: String,
    pub bullets: Vec<String>,
}

/// 🖨️ Builds a structured report for the requested kind.
pub fn build_report(program: &ProgramSnapshot, kind: ReportKind) -> ProgramReport {
    match kind {
        ReportKind::ExecutiveSummary => executive_summary(program),
        ReportKind::ProgramOverview => program_overview(program),
        ReportKind::StakeholderSummary => stakeholder_summary(program),
        ReportKind::RequirementsMatrix => requirements_matrix(program),
        ReportKind::AdjacencyMatrix => adjacency_matrix_report(program),
        ReportKind::GapAnalysis => gap_report(program),
        ReportKind::RiskRegister => risk_register(program),
        ReportKind::DecisionLog => decision_log(program),
        ReportKind::ValidationSummary => validation_summary(program),
        ReportKind::Recommendation => recommendation(program),
        ReportKind::UserSummary => user_summary(program),
        ReportKind::FunctionalSummary => functional_summary(program),
        ReportKind::CapacitySummary => capacity_summary(program),
        ReportKind::WorkflowSummary => workflow_summary(program),
        ReportKind::ComplianceSummary => compliance_summary(program),
        ReportKind::CostSummary => cost_summary(program),
        ReportKind::ScheduleSummary => schedule_summary(program),
        ReportKind::ChangeSummary => change_summary(program),
        ReportKind::OpenIssueSummary => open_issue_summary(program),
        ReportKind::PrioritySummary => priority_summary(program),
        ReportKind::ScenarioSummary => scenario_summary(program),
    }
}

pub(super) fn timestamp(program: &ProgramSnapshot) -> String {
    program.meta.timestamps.updated.clone()
}

fn executive_summary(program: &ProgramSnapshot) -> ProgramReport {
    let summary = status_summary(program);
    ProgramReport {
        kind: ReportKind::ExecutiveSummary,
        title: program.meta.title.clone(),
        generated_at: timestamp(program),
        sections: vec![
            ReportSection {
                heading: "Overview".into(),
                body: program.meta.purpose.text.clone(),
                bullets: vec![format!("{} elements", program.elements.len()), format!("{} requirements", program.requirements.len()), format!("{} stakeholders", program.stakeholders.len())],
            },
            ReportSection { heading: "Status".into(), body: format!("{} total entities tracked", summary.total_entities), bullets: summary.by_status.iter().map(|(status, count)| format!("{status:?}: {count}")).collect() },
        ],
        entity_ids: Vec::new(),
    }
}

fn program_overview(program: &ProgramSnapshot) -> ProgramReport {
    ProgramReport {
        kind: ReportKind::ProgramOverview,
        title: format!("{} — Overview", program.meta.title),
        generated_at: timestamp(program),
        sections: vec![
            ReportSection { heading: "Project".into(), body: program.project.brief_summary.text.clone(), bullets: program.project.objectives.clone() },
            ReportSection { heading: "Scope".into(), body: format!("{} inclusions, {} exclusions", program.project.scope_inclusions.len(), program.project.scope_exclusions.len()), bullets: program.project.deliverables.clone() },
        ],
        entity_ids: vec![program.project.id.clone()],
    }
}

fn stakeholder_summary(program: &ProgramSnapshot) -> ProgramReport {
    ProgramReport {
        kind: ReportKind::StakeholderSummary,
        title: "Stakeholder Summary".into(),
        generated_at: timestamp(program),
        sections: vec![ReportSection {
            heading: "Stakeholders".into(),
            body: format!("{} stakeholder(s)", program.stakeholders.len()),
            bullets: program.stakeholders.iter().map(|s| format!("{} — {} ({:?}/{:?})", s.header.name, s.role, s.influence, s.engagement)).collect(),
        }],
        entity_ids: program.stakeholders.iter().map(|s| s.header.id.clone()).collect(),
    }
}

fn requirements_matrix(program: &ProgramSnapshot) -> ProgramReport {
    let element_names: Vec<String> = program.elements.iter().map(|e| e.header.name.clone()).collect();
    let header = format!("{}\t{}", "Requirement", element_names.join("\t"));
    let mut rows = vec![header];
    for requirement in &program.requirements {
        let cells: Vec<String> = program.elements.iter().map(|element| if requirement.element_ids.contains(&element.header.id) { "X".into() } else { "-".into() }).collect();
        rows.push(format!("{}\t{}", requirement.header.name, cells.join("\t")));
    }
    ProgramReport {
        kind: ReportKind::RequirementsMatrix,
        title: "Requirements Matrix".into(),
        generated_at: timestamp(program),
        sections: vec![ReportSection { heading: "Requirement × Element Grid".into(), body: format!("{}×{} matrix", program.requirements.len(), program.elements.len()), bullets: rows }],
        entity_ids: program.requirements.iter().map(|r| r.header.id.clone()).collect(),
    }
}

fn adjacency_matrix_report(program: &ProgramSnapshot) -> ProgramReport {
    let matrix = adjacency_matrix(program);
    let header: String = format!("{}\t{}", "", matrix.element_ids.iter().map(|id| program.elements.iter().find(|e| &e.header.id == id).map_or(id.0.as_str(), |e| e.header.name.as_str())).collect::<Vec<_>>().join("\t"));
    let mut rows = vec![header];
    for (row_idx, row_id) in matrix.element_ids.iter().enumerate() {
        let name = program.elements.iter().find(|e| &e.header.id == row_id).map_or(row_id.0.as_str(), |e| e.header.name.as_str());
        let cells: Vec<String> = (0..matrix.element_ids.len())
            .map(|col_idx| {
                if row_idx == col_idx {
                    return ".".into();
                }
                let (r, c) = if row_idx > col_idx { (row_idx, col_idx) } else { (col_idx, row_idx) };
                matrix.cells[r][c].as_ref().map_or_else(|| "-".into(), |cell| format!("{:?}/{:.1}", cell.kind, cell.weight))
            })
            .collect();
        rows.push(format!("{name}\t{}", cells.join("\t")));
    }
    ProgramReport {
        kind: ReportKind::AdjacencyMatrix,
        title: "Adjacency Matrix".into(),
        generated_at: timestamp(program),
        sections: vec![ReportSection { heading: "Adjacency Cells".into(), body: format!("{}×{} element matrix", matrix.element_ids.len(), matrix.element_ids.len()), bullets: rows }],
        entity_ids: matrix.element_ids,
    }
}

fn gap_report(program: &ProgramSnapshot) -> ProgramReport {
    let analysis = run_analysis(program, AnalysisKind::Gap);
    ProgramReport {
        kind: ReportKind::GapAnalysis,
        title: "Gap Analysis".into(),
        generated_at: timestamp(program),
        sections: vec![ReportSection { heading: analysis.title, body: analysis.summary, bullets: analysis.findings }],
        entity_ids: analysis.entity_ids,
    }
}

fn risk_register(program: &ProgramSnapshot) -> ProgramReport {
    ProgramReport {
        kind: ReportKind::RiskRegister,
        title: "Risk Register".into(),
        generated_at: timestamp(program),
        sections: vec![ReportSection { heading: "Risks".into(), body: format!("{} risk(s)", program.risks.len()), bullets: program.risks.iter().map(|r| format!("{} — {:?}/{:?}", r.header.name, r.probability, r.impact)).collect() }],
        entity_ids: program.risks.iter().map(|r| r.header.id.clone()).collect(),
    }
}

fn decision_log(program: &ProgramSnapshot) -> ProgramReport {
    ProgramReport {
        kind: ReportKind::DecisionLog,
        title: "Decision Log".into(),
        generated_at: timestamp(program),
        sections: vec![ReportSection {
            heading: "Decisions".into(),
            body: format!("{} decision(s)", program.decisions.len()),
            bullets: program.decisions.iter().map(|d| format!("{} — {:?} ({})", d.header.name, d.approval_status, d.decision_statement.text)).collect(),
        }],
        entity_ids: program.decisions.iter().map(|d| d.header.id.clone()).collect(),
    }
}

fn validation_summary(program: &ProgramSnapshot) -> ProgramReport {
    let diagnostics = validate_plugin(program);
    ProgramReport {
        kind: ReportKind::ValidationSummary,
        title: "Validation Summary".into(),
        generated_at: timestamp(program),
        sections: vec![ReportSection { heading: "Diagnostics".into(), body: format!("{} diagnostic(s)", diagnostics.len()), bullets: diagnostics.iter().map(|d| format!("[{:?}] {}: {}", d.severity, d.code, d.message)).collect() }],
        entity_ids: Vec::new(),
    }
}

pub(super) fn recommendation(program: &ProgramSnapshot) -> ProgramReport {
    let gap = run_analysis(program, AnalysisKind::Gap);
    let conflict = run_analysis(program, AnalysisKind::Conflict);
    ProgramReport {
        kind: ReportKind::Recommendation,
        title: "Recommendations".into(),
        generated_at: timestamp(program),
        sections: vec![ReportSection { heading: "Gaps".into(), body: gap.summary, bullets: gap.findings }, ReportSection { heading: "Conflicts".into(), body: conflict.summary, bullets: conflict.findings }],
        entity_ids: Vec::new(),
    }
}

fn user_summary(program: &ProgramSnapshot) -> ProgramReport {
    ProgramReport {
        kind: ReportKind::UserSummary,
        title: "User Summary".into(),
        generated_at: timestamp(program),
        sections: vec![ReportSection { heading: "User Profiles".into(), body: format!("{} user profile(s)", program.users.len()), bullets: program.users.iter().map(|u| format!("{} — {:?}", u.header.name, u.category)).collect() }],
        entity_ids: program.users.iter().map(|u| u.header.id.clone()).collect(),
    }
}

fn functional_summary(program: &ProgramSnapshot) -> ProgramReport {
    ProgramReport {
        kind: ReportKind::FunctionalSummary,
        title: "Functional Summary".into(),
        generated_at: timestamp(program),
        sections: vec![ReportSection {
            heading: "Functions".into(),
            body: format!("{} function(s), {} activit(ies)", program.functions.len(), program.activities.len()),
            bullets: program.functions.iter().map(|f| format!("{} — {:?} ({})", f.header.name, f.kind, f.purpose.text)).collect(),
        }],
        entity_ids: program.functions.iter().map(|f| f.header.id.clone()).collect(),
    }
}

fn capacity_summary(program: &ProgramSnapshot) -> ProgramReport {
    let analysis = run_analysis(program, AnalysisKind::Capacity);
    ProgramReport {
        kind: ReportKind::CapacitySummary,
        title: "Capacity Summary".into(),
        generated_at: timestamp(program),
        sections: vec![ReportSection { heading: analysis.title, body: analysis.summary, bullets: analysis.findings }],
        entity_ids: Vec::new(),
    }
}

fn workflow_summary(program: &ProgramSnapshot) -> ProgramReport {
    let analysis = run_analysis(program, AnalysisKind::Workflow);
    ProgramReport {
        kind: ReportKind::WorkflowSummary,
        title: "Workflow Summary".into(),
        generated_at: timestamp(program),
        sections: vec![ReportSection { heading: analysis.title, body: analysis.summary, bullets: analysis.findings }],
        entity_ids: analysis.entity_ids,
    }
}

fn compliance_summary(program: &ProgramSnapshot) -> ProgramReport {
    ProgramReport {
        kind: ReportKind::ComplianceSummary,
        title: "Compliance Summary".into(),
        generated_at: timestamp(program),
        sections: vec![
            ReportSection { heading: "Regulatory".into(), body: format!("{} regulatory requirement(s)", program.regulatory.len()), bullets: program.regulatory.iter().map(|r| r.header.name.clone()).collect() },
            ReportSection { heading: "Validations".into(), body: format!("{} validation record(s)", program.validations.len()), bullets: program.validations.iter().map(|v| format!("{} — {:?}", v.header.name, v.result)).collect() },
        ],
        entity_ids: program.regulatory.iter().map(|r| r.header.id.clone()).collect(),
    }
}

fn cost_summary(program: &ProgramSnapshot) -> ProgramReport {
    let analysis = run_analysis(program, AnalysisKind::Cost);
    ProgramReport { kind: ReportKind::CostSummary, title: "Cost Summary".into(), generated_at: timestamp(program), sections: vec![ReportSection { heading: analysis.title, body: analysis.summary, bullets: analysis.findings }], entity_ids: Vec::new() }
}

fn schedule_summary(program: &ProgramSnapshot) -> ProgramReport {
    ProgramReport {
        kind: ReportKind::ScheduleSummary,
        title: "Schedule Summary".into(),
        generated_at: timestamp(program),
        sections: vec![ReportSection {
            heading: "Schedules".into(),
            body: format!("{} schedule requirement(s), {} delivery constraints", program.schedules.len(), program.delivery.len()),
            bullets: program.schedules.iter().map(|s| s.header.name.clone()).collect(),
        }],
        entity_ids: program.schedules.iter().map(|s| s.header.id.clone()).collect(),
    }
}

fn change_summary(program: &ProgramSnapshot) -> ProgramReport {
    ProgramReport {
        kind: ReportKind::ChangeSummary,
        title: "Change Summary".into(),
        generated_at: timestamp(program),
        sections: vec![ReportSection { heading: "Changes".into(), body: format!("{} change record(s)", program.changes.len()), bullets: program.changes.iter().map(|c| format!("{} — {}", c.header.name, c.header.timestamps.updated)).collect() }],
        entity_ids: program.changes.iter().map(|c| c.header.id.clone()).collect(),
    }
}

fn open_issue_summary(program: &ProgramSnapshot) -> ProgramReport {
    let open: Vec<_> = program.issues.iter().filter(|i| !matches!(i.header.status, LifecycleStatus::Closed | LifecycleStatus::Complete)).collect();
    ProgramReport {
        kind: ReportKind::OpenIssueSummary,
        title: "Open Issue Summary".into(),
        generated_at: timestamp(program),
        sections: vec![ReportSection {
            heading: "Open Issues".into(),
            body: format!("{} open of {} total issues", open.len(), program.issues.len()),
            bullets: open.iter().map(|i| format!("{} — {:?}/{:?}", i.header.name, i.severity, i.issue_priority)).collect(),
        }],
        entity_ids: open.iter().map(|i| i.header.id.clone()).collect(),
    }
}

fn priority_summary(program: &ProgramSnapshot) -> ProgramReport {
    ProgramReport {
        kind: ReportKind::PrioritySummary,
        title: "Priority Summary".into(),
        generated_at: timestamp(program),
        sections: vec![ReportSection {
            heading: "Priorities".into(),
            body: format!("{} priority record(s)", program.priorities.len()),
            bullets: program.priorities.iter().map(|p| format!("{} — {:?} weight {:?}", p.header.name, p.ranked_priority, p.weight)).collect(),
        }],
        entity_ids: program.priorities.iter().map(|p| p.header.id.clone()).collect(),
    }
}

fn scenario_summary(program: &ProgramSnapshot) -> ProgramReport {
    let analysis = run_analysis(program, AnalysisKind::Scenario);
    ProgramReport {
        kind: ReportKind::ScenarioSummary,
        title: "Scenario Summary".into(),
        generated_at: timestamp(program),
        sections: vec![ReportSection { heading: analysis.title, body: analysis.summary, bullets: analysis.findings }],
        entity_ids: analysis.entity_ids,
    }
}
//#endregion 📄️Report
