//! 🔧️ Dissolved out of the former artifact-tree `⚙️engine` topic files (ticket
//! 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES) — every function that takes `&mut
//! ProgramSnapshot` (constructs/mutates) rather than merely reading it. `🎛️apps/🏛️architect/⚙️engine`
//! exists on disk as the reserved machine-slot stub, but `🦀️.rs` (out of scope for this ticket)
//! never mounts it into the module tree — only two `pub mod engine { ... }` shims exist there and
//! both live in the ARTIFACT-tree shim section, not the Apps section — so populating that directory
//! would produce dead, uncompiled code. Per this packet's own fallback this behavior lands directly
//! on the app top-level instead; the reserved directory is left empty and untouched. This does NOT
//! invent a new state machine: every item below is a plain function/struct, unchanged in shape from
//! its former engine-topic file, just relocated.

use crate::standards::v1::subsets::any::schema::kernel::{EntityHeader, EntityId, PluginError, TextField, TraceKind, TraceLink};
use crate::standards::v1::subsets::any::schema::mutations::ProgramMutation;
use crate::standards::v1::subsets::any::schema::registers::{Activity, Adjacency, AdjacencyKind, AnalysisKind, AnalysisRecord, ConnectionKind, Equipment, Function, FunctionKind, Process, ProgramElement, ProgramElementKind, Relationship, RelationshipKind, ReportKind, ReportRecord, Requirement, RequirementKind, Risk, RiskLevel, Stakeholder, TemplateRecord, UserCategory, UserProfile, ValidationStatus};
use crate::standards::v1::subsets::any::schema::inferences::{build_report, run_analysis};
use crate::standards::v1::subsets::any::io::tables::RegisterCsvRow;
use crate::standards::v1::subsets::any::schema::kernel::normalize_pair;
use crate::standards::v1::subsets::any::schema::snapshot::ProgramSnapshot;
use semio_s_artifact_stdio_csv as stdio_csv;
use semio_s_artifact_stdio_tsv as stdio_tsv;

use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

/// 🔀 Absolute path to the `🧬️mutations` facet's per-register leaves, kept as one alias so the
/// semantic-mutations-overhaul rename (`.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL`)
/// only needs updating here if `🦀️.rs`'s directory wiring ever changes.
use crate::standards::v1::subsets::any::schema::mutations as leaves;
use semio_s_artifact_architect_program_catalog::standards::v1::subsets::any::schema::registers::catalog::RegisterRow;

//#region 📐️Template
/// 📋️ Result of applying a template to a program.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct TemplateApplyResult {
    pub template_id: EntityId,
    pub created_entity_ids: Vec<EntityId>,
    pub messages: Vec<String>,
}

/// 🧩️ Expands a template record into the replayable `ProgramMutation`s that create its rows; it reads no snapshot and writes none.
pub fn apply_template(template: &TemplateRecord) -> Vec<ProgramMutation> {
    let mut operations = Vec::new();
    let mut element_ids = Vec::new();
    for field in &template.default_fields {
        let id = EntityId::new_serial("template-entity", "template-entity");
        match field.as_str() {
            "stakeholder" => {
                let item = Stakeholder {
                    header: EntityHeader::new(id.clone(), format!("{} Stakeholder", template.header.name)),
                    role: "Template".into(),
                    organization: template.source_organization.clone().unwrap_or_default(),
                    department: None,
                    contact_email: None,
                    contact_phone: None,
                    influence: crate::standards::v1::subsets::any::schema::registers::InfluenceLevel::Medium,
                    interest: crate::standards::v1::subsets::any::schema::registers::InfluenceLevel::Medium,
                    engagement: crate::standards::v1::subsets::any::schema::registers::EngagementLevel::Neutral,
                    expectations: template.checklists.clone(),
                    concerns: Vec::new(),
                    requirement_ids: Vec::new(),
                    decision_authority: false,
                    communication_preferences: Vec::new(),
                    reporting_frequency: None,
                    involvement_phases: Vec::new(),
                    availability: None,
                    representative_of: None,
                    delegated_to: None,
                    relationship_to_client: None,
                    power_interest_notes: Vec::new(),
                    stakeholder_type: "Template".into(),
                    influence_strategy: None,
                    communication_channels: Vec::new(),
                    success_metrics: Vec::new(),
                };
                operations.push(ProgramMutation::CreateRegisterRow(leaves::create_register_row::CreateRegisterRow { row: RegisterRow::Stakeholder(item), index: None }));
            }
            "user" => {
                let item = UserProfile {
                    header: EntityHeader::new(id.clone(), format!("{} User", template.header.name)),
                    category: UserCategory::Primary,
                    demographic: None,
                    age_range: None,
                    abilities: Vec::new(),
                    disabilities: Vec::new(),
                    occupation: None,
                    role_title: None,
                    department: None,
                    mobility_profile: Vec::new(),
                    sensory_profile: Vec::new(),
                    cognitive_profile: Vec::new(),
                    behavioral_patterns: Vec::new(),
                    usage_frequency: None,
                    usage_duration: None,
                    peak_usage_times: Vec::new(),
                    technology_proficiency: None,
                    preferences: Vec::new(),
                    pain_points: Vec::new(),
                    goals: template.checklists.clone(),
                    activity_ids: Vec::new(),
                    research_method: None,
                    persona_archetype: None,
                    validated: false,
                    stakeholder_ids: Vec::new(),
                };
                operations.push(ProgramMutation::CreateRegisterRow(leaves::create_register_row::CreateRegisterRow { row: RegisterRow::UserProfile(item), index: None }));
            }
            "activity" => {
                let item = Activity {
                    header: EntityHeader::new(id.clone(), format!("{} Activity", template.header.name)),
                    code: "ACT".into(),
                    category: template.sector.clone().unwrap_or_else(|| "general".into()),
                    frequency: None,
                    duration: None,
                    intensity: None,
                    participants: crate::standards::v1::subsets::any::schema::kernel::QuantitySpec::default(),
                    equipment_ids: Vec::new(),
                    space_requirements: Vec::new(),
                    environmental_needs: Vec::new(),
                    privacy_needs: Vec::new(),
                    accessibility_needs: Vec::new(),
                    adjacent_activities: Vec::new(),
                    sequencing: Vec::new(),
                    peak_periods: Vec::new(),
                    workflow_steps: template.checklists.clone(),
                    inputs: Vec::new(),
                    outputs: Vec::new(),
                    user_profile_ids: Vec::new(),
                    function_ids: Vec::new(),
                    performance_indicators: Vec::new(),
                    activity_type: "template".into(),
                    location_context: None,
                    temporal_pattern: None,
                    supervision_level: None,
                };
                operations.push(ProgramMutation::CreateRegisterRow(leaves::create_register_row::CreateRegisterRow { row: RegisterRow::Activity(item), index: None }));
            }
            "function" => {
                let item = Function {
                    header: EntityHeader::new(id.clone(), format!("{} Function", template.header.name)),
                    code: "FN".into(),
                    kind: FunctionKind::Primary,
                    purpose: TextField::plain(template.standards.join(", ")),
                    criticality: crate::standards::v1::subsets::any::schema::kernel::Priority::Preferred,
                    performance_targets: Vec::new(),
                    service_level: None,
                    operating_hours: None,
                    staffing: crate::standards::v1::subsets::any::schema::kernel::QuantitySpec::default(),
                    equipment_ids: Vec::new(),
                    resource_ids: Vec::new(),
                    activity_ids: Vec::new(),
                    element_ids: Vec::new(),
                    dependencies: Vec::new(),
                    interfaces: Vec::new(),
                    constraints: Vec::new(),
                    quality_criteria: Vec::new(),
                    regulatory_refs: template.standards.clone(),
                    future_changes: Vec::new(),
                    owner_stakeholder_id: None,
                    success_metrics: Vec::new(),
                    hierarchy_parent_id: None,
                    conflict_ids: Vec::new(),
                };
                operations.push(ProgramMutation::CreateRegisterRow(leaves::create_register_row::CreateRegisterRow { row: RegisterRow::Function(item), index: None }));
            }
            "element" | "room" => {
                let item = ProgramElement {
                    header: EntityHeader::new(id.clone(), format!("{} Space", template.header.name)),
                    code: "TPL".into(),
                    kind: ProgramElementKind::Room,
                    parent_id: None,
                    level: None,
                    area: crate::standards::v1::subsets::any::schema::kernel::QuantitySpec::default(),
                    volume: crate::standards::v1::subsets::any::schema::kernel::QuantitySpec::default(),
                    height: crate::standards::v1::subsets::any::schema::kernel::QuantitySpec::default(),
                    occupancy: crate::standards::v1::subsets::any::schema::kernel::QuantitySpec::default(),
                    function_ids: Vec::new(),
                    activity_ids: Vec::new(),
                    user_profile_ids: Vec::new(),
                    adjacency_ids: Vec::new(),
                    quantity_ids: Vec::new(),
                    requirement_ids: Vec::new(),
                    location_hint: None,
                    orientation: None,
                    daylight_requirement: None,
                    acoustic_class: None,
                    security_zone: None,
                    flexibility_notes: Vec::new(),
                    growth_allocation: None,
                    circulation_role: None,
                    visibility_level: None,
                    adjacency_preferences: Vec::new(),
                    environmental_zone: None,
                };
                operations.push(ProgramMutation::CreateRegisterRow(leaves::create_register_row::CreateRegisterRow { row: RegisterRow::ProgramElement(item), index: None }));
                element_ids.push(id.clone());
            }
            "requirement" => {
                let item = Requirement {
                    header: EntityHeader::new(id.clone(), format!("{} Requirement", template.header.name)),
                    code: String::new(),
                    kind: RequirementKind::Functional,
                    statement: TextField::plain(template.standards.join(", ")),
                    rationale: None,
                    source: template.source_organization.clone(),
                    stakeholder_ids: Vec::new(),
                    element_ids: Vec::new(),
                    function_ids: Vec::new(),
                    parent_requirement_id: None,
                    child_requirement_ids: Vec::new(),
                    acceptance_criteria: template.checklists.clone(),
                    verification_method: None,
                    validation_status: ValidationStatus::Pending,
                    conflict_ids: Vec::new(),
                    risk_ids: Vec::new(),
                    cost_estimate: None,
                    schedule_constraint: None,
                    regulatory_refs: template.standards.clone(),
                    trace_links: Vec::new(),
                    superseded_by: None,
                };
                operations.push(ProgramMutation::CreateRegisterRow(leaves::create_register_row::CreateRegisterRow { row: RegisterRow::Requirement(item), index: None }));
            }
            "risk" => {
                let item = Risk {
                    header: EntityHeader::new(id.clone(), format!("{} Risk", template.header.name)),
                    risk_statement: TextField::plain(template.checklists.join("; ")),
                    category: "template".into(),
                    probability: RiskLevel::Medium,
                    impact: RiskLevel::Medium,
                    risk_score: None,
                    causes: Vec::new(),
                    effects: Vec::new(),
                    affected_element_ids: Vec::new(),
                    affected_requirement_ids: Vec::new(),
                    mitigation: Vec::new(),
                    contingency: Vec::new(),
                    owner_id: None,
                    review_date: None,
                    trigger_indicators: Vec::new(),
                    residual_probability: None,
                    residual_impact: None,
                    related_conflict_ids: Vec::new(),
                    escalation_path: Vec::new(),
                    monitoring_plan: None,
                };
                operations.push(ProgramMutation::CreateRegisterRow(leaves::create_register_row::CreateRegisterRow { row: RegisterRow::Risk(item), index: None }));
            }
            "process" => {
                let item = Process {
                    header: EntityHeader::new(id.clone(), format!("{} Process", template.header.name)),
                    code: "PRC".into(),
                    category: template.sector.clone().unwrap_or_else(|| "general".into()),
                    trigger: None,
                    inputs: Vec::new(),
                    outputs: Vec::new(),
                    steps: template.checklists.clone(),
                    actors: Vec::new(),
                    equipment_ids: Vec::new(),
                    element_ids: Vec::new(),
                    duration: None,
                    frequency: None,
                    critical_path: false,
                    bottlenecks: Vec::new(),
                    dependencies: Vec::new(),
                    kpis: Vec::new(),
                    automation_level: None,
                    failure_modes: Vec::new(),
                    improvement_opportunities: Vec::new(),
                    regulatory_refs: template.standards.clone(),
                    owner_id: None,
                    workflow_type: Some("template".into()),
                    handoff_points: Vec::new(),
                    quality_gates: Vec::new(),
                };
                operations.push(ProgramMutation::CreateRegisterRow(leaves::create_register_row::CreateRegisterRow { row: RegisterRow::Process(item), index: None }));
            }
            "equipment" => {
                let item = Equipment {
                    header: EntityHeader::new(id.clone(), format!("{} Equipment", template.header.name)),
                    code: "EQ".into(),
                    category: template.sector.clone().unwrap_or_else(|| "general".into()),
                    manufacturer: None,
                    model: None,
                    quantity: crate::standards::v1::subsets::any::schema::kernel::QuantitySpec::default(),
                    dimensions: None,
                    weight_kg: None,
                    power_kw: None,
                    utility_connections: Vec::new(),
                    ventilation: None,
                    noise_level_db: None,
                    clearance: None,
                    mounting: None,
                    element_ids: Vec::new(),
                    activity_ids: Vec::new(),
                    maintenance_access: Vec::new(),
                    lifecycle_years: None,
                    replacement_cost: None,
                    standards: template.standards.clone(),
                    supplier: None,
                    activity_link_ids: Vec::new(),
                    installation_requirements: Vec::new(),
                    commissioning_notes: Vec::new(),
                    spare_parts: Vec::new(),
                };
                operations.push(ProgramMutation::CreateRegisterRow(leaves::create_register_row::CreateRegisterRow { row: RegisterRow::Equipment(item), index: None }));
            }
            "adjacency" | "adjacency_bundle" if element_ids.len() >= 2 => {
                let (a, b) = normalize_pair(&element_ids[0], &element_ids[1]);
                let adjacency = Adjacency {
                    header: EntityHeader::new(id.clone(), format!("{} Adjacency", template.header.name)),
                    element_a_id: a,
                    element_b_id: b,
                    kind: AdjacencyKind::Preferred,
                    connection: ConnectionKind::Direct,
                    separations: Vec::new(),
                    weight: 1.0,
                    rationale: Some(TextField::plain("template bundle")),
                    distance_max_m: None,
                    distance_min_m: None,
                    level_constraint: None,
                    access_path: None,
                    shared_wall: true,
                    shared_entry: false,
                    traffic_isolation: false,
                    circulation_overlap: true,
                    conflict_ids: Vec::new(),
                    normalized: true,
                    verification_status: ValidationStatus::Pending,
                    source_relationship_id: None,
                    internal_external_access: None,
                };
                operations.push(ProgramMutation::ConnectAdjacency(leaves::connect_adjacency::ConnectAdjacency { adjacency, index: None }));
            }
            _ => {}
        }
    }
    operations
}
//#endregion 📐️Template

//#region 📄️ReportRecord
/// 📝️ Builds a report and the `CreateRegisterRow` mutation for its report record that persists it as a `ReportRecord`.
pub fn build_report_and_record(program: &ProgramSnapshot, kind: ReportKind) -> (crate::standards::v1::subsets::any::schema::inferences::ProgramReport, ProgramMutation) {
    let report = build_report(program, kind);
    let report_record = ReportRecord {
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
    (report, ProgramMutation::CreateRegisterRow(leaves::create_register_row::CreateRegisterRow { row: RegisterRow::ReportRecord(report_record), index: None }))
}
//#endregion 📄️ReportRecord

//#region 🔬️AnalysisRecord
/// 📝️ Runs analysis and the `CreateRegisterRow` mutation for its analysis record that persists it as an `AnalysisRecord`.
pub fn run_analysis_and_record(program: &ProgramSnapshot, kind: AnalysisKind) -> (crate::standards::v1::subsets::any::schema::inferences::AnalysisResult, ProgramMutation) {
    let result = run_analysis(program, kind);
    let analysis_record = AnalysisRecord {
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
    (result, ProgramMutation::CreateRegisterRow(leaves::create_register_row::CreateRegisterRow { row: RegisterRow::AnalysisRecord(analysis_record), index: None }))
}
//#endregion 🔬️AnalysisRecord


//#region 🧭️Trace
/// ⛓️ Ordered chain of trace links from a root entity.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct TraceChain {
    pub root_id: EntityId,
    pub links: Vec<TraceLink>,
}

/// 💥️ Reverse impact set from trace links pointing at an entity.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct ImpactTrace {
    pub target_id: EntityId,
    pub upstream_ids: Vec<EntityId>,
    pub links: Vec<TraceLink>,
}

/// 🔗️ Builds a forward trace chain from `root_id` following kind-appropriate links.
pub fn trace_chain(program: &ProgramSnapshot, root_id: &EntityId) -> TraceChain {
    let traces = effective_traces(program);
    let adjacency = trace_adjacency(&traces);
    let mut visited = HashSet::new();
    let mut links = Vec::new();
    let mut queue = VecDeque::new();
    queue.push_back(root_id.clone());
    visited.insert(root_id.clone());
    while let Some(current) = queue.pop_front() {
        if let Some(outgoing) = adjacency.get(&current) {
            for link in outgoing {
                if !follows_kind_chain(&link.kind) {
                    continue;
                }
                links.push(link.clone());
                if visited.insert(link.to_id.clone()) {
                    queue.push_back(link.to_id.clone());
                }
            }
        }
    }
    TraceChain { root_id: root_id.clone(), links }
}

/// 🔍️ Finds trace links touching `entity_id` (from or to).
pub fn trace_links_for(program: &ProgramSnapshot, entity_id: &EntityId) -> Vec<TraceLink> {
    effective_traces(program).into_iter().filter(|link| &link.from_id == entity_id || &link.to_id == entity_id).collect()
}

/// ↩️ Reverse impact trace — entities that depend on or satisfy `target_id`.
pub fn trace_impact(program: &ProgramSnapshot, target_id: &EntityId) -> ImpactTrace {
    let traces = effective_traces(program);
    let mut upstream = HashSet::new();
    let mut links = Vec::new();
    let mut queue = VecDeque::new();
    queue.push_back(target_id.clone());
    while let Some(current) = queue.pop_front() {
        for link in &traces {
            if link.to_id != current {
                continue;
            }
            if matches!(link.kind, TraceKind::ObjectiveToRequirement | TraceKind::StakeholderToRequirement | TraceKind::FunctionToProgramElement | TraceKind::RequirementToDecision | TraceKind::RequirementToRisk | TraceKind::ConstraintToImpact) {
                links.push(link.clone());
                if upstream.insert(link.from_id.clone()) {
                    queue.push_back(link.from_id.clone());
                }
            }
        }
    }
    ImpactTrace { target_id: target_id.clone(), upstream_ids: upstream.into_iter().collect(), links }
}

/// ➕️ The mutation that appends a trace link to the plugin trace register.
pub fn add_trace_link(from_id: EntityId, to_id: EntityId, kind: TraceKind) -> ProgramMutation {
    ProgramMutation::ConnectTrace(leaves::connect_trace::ConnectTrace { trace: TraceLink::new(from_id, to_id, kind), index: None })
}

/// 🧷️ The plugin trace register plus the requirement-embedded trace links it does not hold yet.
fn effective_traces(program: &ProgramSnapshot) -> Vec<TraceLink> {
    let mut traces = program.traces.clone();
    for requirement in &program.requirements {
        for link in &requirement.trace_links {
            if traces.iter().any(|t| t.id == link.id) {
                continue;
            }
            traces.push(link.clone());
        }
    }
    traces
}

fn follows_kind_chain(kind: &TraceKind) -> bool {
    !matches!(kind, TraceKind::FullAuditTrail)
}

fn trace_adjacency(traces: &[TraceLink]) -> HashMap<EntityId, Vec<TraceLink>> {
    let mut map: HashMap<EntityId, Vec<TraceLink>> = HashMap::new();
    for link in traces {
        map.entry(link.from_id.clone()).or_default().push(link.clone());
    }
    map
}
//#endregion 🧭️Trace

#[cfg(test)]
//#region 🧪️BehaviorTests
include!("../🧪️tests/🔬️behavior-unit/🦀️.rs");
//#endregion 🧪️BehaviorTests
