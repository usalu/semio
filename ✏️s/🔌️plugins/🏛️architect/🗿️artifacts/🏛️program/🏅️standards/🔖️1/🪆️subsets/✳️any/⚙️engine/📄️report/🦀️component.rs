//! ⚙️ Architect program artifact engine — the `report` topic.

//! 📄️ ProgramSnapshot reporting — structured reports from `ReportKind`.

use crate::artifacts::program::engine::analyze::run_analysis;
use crate::artifacts::program::kernel::{EntityHeader, EntityId};
use crate::artifacts::program::ProgramSnapshot;
use crate::artifacts::program::registers::{AnalysisKind, ReportKind, ReportRecord, ValidationStatus};
use crate::artifacts::program::engine::status_summary::status_summary;
use crate::artifacts::program::engine::validate::validate_plugin;
use serde::{Deserialize, Serialize};

// #region 🔖️ProgramReport
/// @emoji 📑️ Structured report payload for export and program rendering.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProgramReport {
    pub kind: ReportKind,
    pub title: String,
    pub generated_at: String,
    pub sections: Vec<ReportSection>,
    pub entity_ids: Vec<EntityId>,
}

/// @emoji 📎️ One titled section within a plugin report.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReportSection {
    pub heading: String,
    pub body: String,
    pub bullets: Vec<String>,
}
// #endregion

// #region 🔖️BuildReport
/// @emoji 🖨️ Builds a structured report for the requested kind.
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

/// @emoji 📝️ Builds a report and appends a `ReportRecord` to the program.
pub fn build_report_and_record(program: &mut ProgramSnapshot, kind: ReportKind) -> ProgramReport {
    let report = build_report(program, kind);
    let record = ReportRecord {
        header: EntityHeader::new(EntityId::new_serial("report", "report"), report.title.clone()),
        kind,
        title: report.title.clone(),
        audience: Vec::new(),
        sections: report.sections.iter().map(|s| s.heading.clone()).collect(),
        generated_at: Some(report.generated_at.clone()),
        generated_by: None,
        analysis_ids: Vec::new(),
        format: Some("structured".into()),
        file_ref: None,
        distribution_list: Vec::new(),
        approval_status: ValidationStatus::Pending,
        approver_id: None,
        version: program.meta.revision.clone(),
        template_id: None,
        parameters: Vec::new(),
        confidentiality: None,
        expiry_date: None,
        related_decision_ids: Vec::new(),
    };
    program.reports.push(record);
    report
}

fn timestamp(program: &ProgramSnapshot) -> String {
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
    let matrix = crate::artifacts::program::engine::adjacency::adjacency_matrix(program);
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

fn recommendation(program: &ProgramSnapshot) -> ProgramReport {
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
    ProgramReport {
        kind: ReportKind::CostSummary,
        title: "Cost Summary".into(),
        generated_at: timestamp(program),
        sections: vec![ReportSection { heading: analysis.title, body: analysis.summary, bullets: analysis.findings }],
        entity_ids: Vec::new(),
    }
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
    let open: Vec<_> = program.issues.iter().filter(|i| !matches!(i.header.status, crate::artifacts::program::kernel::LifecycleStatus::Closed | crate::artifacts::program::kernel::LifecycleStatus::Complete)).collect();
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
// #endregion

#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::program::sample_plugin;

    #[test]
    fn executive_summary_includes_counts() {
        let report = build_report(&sample_plugin(), ReportKind::ExecutiveSummary);
        assert_eq!(report.kind, ReportKind::ExecutiveSummary);
        assert!(!report.sections.is_empty());
    }

    #[test]
    fn requirements_matrix_has_grid_rows() {
        let report = build_report(&sample_plugin(), ReportKind::RequirementsMatrix);
        assert!(!report.sections[0].bullets.is_empty());
        assert!(report.sections[0].bullets[0].contains('\t'));
    }

    #[test]
    fn build_report_and_record_persists() {
        let mut program = sample_plugin();
        let before = program.reports.len();
        build_report_and_record(&mut program, ReportKind::AdjacencyMatrix);
        assert_eq!(program.reports.len(), before + 1);
    }
}
