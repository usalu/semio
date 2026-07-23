//! 🏛️ Architect plugin — architectural program DocumentApp bundled as a hot-swappable WASM component.

use architect_program::{
    adjacency_matrix, apply_template, audit_trail, build_report, detect_adjacency_conflicts, empty_program, export_json, export_registers_csv, import_json, import_registers_csv, normalize_pair, run_analysis, sample_program, search_program,
    status_summary, trace_chain, trace_impact, undirected_edges, validate_program, Adjacency, AdjacencyKind, AdjacencyPatch, AnalysisKind, AnalysisRecord, AnalysisResult, ConnectionKind, EngagementLevel, EntityHeader, EntityId, Function,
    FunctionKind, InfluenceLevel, Issue, IssueSeverity, MergeStrategy, Program, ProgramElement, ProgramElementKind, ProgramElementPatch, ProgramOp, ProgramReport, ReportKind, ReportRecord, Requirement, RequirementKind, Risk, RiskLevel, SearchQuery,
    Stakeholder, StakeholderPatch, TextField, TraceChain, TraceKind, TraceLink, UserCategory, UserProfile, ValidationStatus, ARCHITECT_PROGRAM_SCHEMA,
};
use semio_framework_plugin::{
    create_default_layout, ui_inspector_groups_to_tree, ui_inspector_mixed_number, ui_inspector_mixed_text, ui_inspector_mixed_toggle, ui_inspector_readonly_field, ui_stack_vertical, ui_text, ActionArgDef, ActionArgOption, ActionDefinition,
    ActionDescriptor, ActionEmit, ActionKind, App, AppLabelsOverlay, BlockListScene, DocumentApp, DocumentView, HostEffect, NodeGraphScene, PanelGroup, SurfaceKind, UiComponentSceneNode, UiFieldNode, UiInputNode, UiInspectorFieldGroup, UiNode,
    UiNumberStepperNode, UiStackNode, UiToggleNode, UiTreeItemNode, UiTreeNode, UiTreeSectionNode, ViewState, FRAMEWORK_PANEL_TAB_CATALOGUE_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, FRAMEWORK_PANEL_TAB_DOCUMENT_ID, FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL,
    FRAMEWORK_PANEL_TAB_INSPECTION_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;
use vcs::CollectionOp;

//#region 🔖Constants
const ARCHITECT_APP_ID: &str = "architect";
const ARCHITECT_BODY_ADJACENCY: &str = "architect.adjacency";
const ARCHITECT_BODY_GRAPH: &str = "architect.graph";
const ARCHITECT_BODY_REGISTER: &str = "architect.register";
const ARCHITECT_BODY_REPORT: &str = "architect.report";
const ARCHITECT_BODY_DOCUMENT: &str = "architect.document";
const ARCHITECT_BODY_CATALOGUE: &str = "architect.catalogue";
const ARCHITECT_BODY_INSPECTION: &str = "architect.inspection";
const ARCHITECT_BODY_TRACE: &str = "architect.trace";
const ARCHITECT_WINDOW_ADJACENCY: &str = "architect-adjacency";
const ARCHITECT_WINDOW_GRAPH: &str = "architect-graph";
const ARCHITECT_WINDOW_REGISTER: &str = "architect-register";
const ARCHITECT_WINDOW_REPORT: &str = "architect-report";
const ARCHITECT_WINDOW_TRACE: &str = "architect-trace";

const REGISTER_IDS: &[&str] = &[
    "stakeholders",
    "users",
    "activities",
    "functions",
    "elements",
    "quantities",
    "relationships",
    "adjacencies",
    "processes",
    "flows",
    "access_rules",
    "operations",
    "equipment",
    "resources",
    "storage",
    "environmental",
    "human_factors",
    "accessibility",
    "privacy",
    "safety",
    "security",
    "regulatory",
    "site_context",
    "organizational",
    "services",
    "infrastructure",
    "information",
    "communication",
    "wayfinding",
    "schedules",
    "flexibility",
    "growth",
    "sustainability",
    "resilience",
    "costs",
    "delivery",
    "risks",
    "conflicts",
    "requirements",
    "priorities",
    "scenarios",
    "options",
    "decisions",
    "validations",
    "performance",
    "quality",
    "documents",
    "assumptions",
    "constraints",
    "compliance_records",
    "approvals",
    "meetings",
    "changes",
    "collaboration",
    "analyses",
    "reports",
    "search_filters",
    "status_records",
    "workshops",
    "surveys",
    "issues",
    "audit_events",
    "templates",
    "knowledge",
    "benchmarks",
    "traces",
];
//#endregion 🔖Constants

//#region 🔖Runtime
/// @emoji 👁️ Ephemeral per-session view state — selection, active register, search, and cached reports.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
struct ArchitectPlayRuntime {
    selected_ids: Vec<String>,
    active_register: String,
    search_query: String,
    search_history: Vec<SearchQuery>,
    last_report: Option<ProgramReport>,
    last_report_json: String,
    last_analysis: Option<AnalysisResult>,
    adjacency_kind_filter: Option<AdjacencyKind>,
    graph_camera: GraphCamera,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct GraphCamera {
    x: f64,
    y: f64,
    zoom: f64,
}

impl Default for GraphCamera {
    fn default() -> Self {
        Self { x: 0.0, y: 0.0, zoom: 1.0 }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RegisterBlockStep {
    id: String,
    title: String,
    blocks: Vec<RegisterBlockItem>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RegisterBlockItem {
    id: String,
    label: String,
    kind: String,
}

//#endregion 🔖Runtime

//#region 🔖Helpers
fn architect_action(action: &str, args: Option<Value>) -> ActionDescriptor {
    ActionDescriptor { controller_id: ARCHITECT_APP_ID.into(), action: action.into(), args }
}

fn tree_item(id: impl Into<String>, label: impl Into<String>) -> UiTreeItemNode {
    UiTreeItemNode {
        id: id.into(),
        label: label.into(),
        description: None,
        icon_id: None,
        selected: None,
        loading: None,
        waiting: None,
        default_open: None,
        action: None,
        hover_action: None,
        unhover_action: None,
        actions: None,
        draggable: None,
        drag_data: None,
        items: None,
        control: None,
        is_hidden: None,
    }
}

fn tree_item_with_action(id: impl Into<String>, label: impl Into<String>, description: Option<String>, action: ActionDescriptor) -> UiTreeItemNode {
    UiTreeItemNode {
        id: id.into(),
        label: label.into(),
        description,
        icon_id: None,
        selected: None,
        loading: None,
        waiting: None,
        default_open: None,
        action: Some(action),
        hover_action: None,
        unhover_action: None,
        actions: None,
        draggable: None,
        drag_data: None,
        items: None,
        control: None,
        is_hidden: None,
    }
}

fn tree_section(id: impl Into<String>, label: Option<String>, items: Vec<UiTreeItemNode>) -> UiTreeSectionNode {
    UiTreeSectionNode { id: id.into(), label, default_open: Some(true), loading: None, waiting: None, items }
}

fn tree_node(sections: Vec<UiTreeSectionNode>, selected_ids: Option<Vec<String>>) -> UiNode {
    UiNode::Tree(UiTreeNode { sections, loading: None, waiting: None, selected_ids, highlighted_ids: None, selection_change: None, drop_action: None })
}

fn element_label(program: &Program, id: &EntityId) -> String {
    program.elements.iter().find(|element| &element.header.id == id).map_or_else(|| id.to_string(), |element| element.header.name.clone())
}

fn adjacency_kind_label(kind: &AdjacencyKind) -> &'static str {
    match kind {
        AdjacencyKind::Required => "Required",
        AdjacencyKind::Preferred => "Preferred",
        AdjacencyKind::Optional => "Optional",
        AdjacencyKind::Prohibited => "Prohibited",
    }
}

fn next_adjacency_kind(current: Option<&AdjacencyKind>) -> Option<AdjacencyKind> {
    match current {
        None => Some(AdjacencyKind::Required),
        Some(AdjacencyKind::Required) => Some(AdjacencyKind::Preferred),
        Some(AdjacencyKind::Preferred) => Some(AdjacencyKind::Optional),
        Some(AdjacencyKind::Optional) => Some(AdjacencyKind::Prohibited),
        Some(AdjacencyKind::Prohibited) => None,
    }
}

fn find_adjacency<'a>(program: &'a Program, a: &EntityId, b: &EntityId) -> Option<&'a Adjacency> {
    let (left, right) = normalize_pair(a, b);
    program.adjacencies.iter().find(|row| row.element_a_id == left && row.element_b_id == right)
}

fn default_element(name: impl Into<String>) -> ProgramElement {
    ProgramElement {
        header: EntityHeader::new(EntityId::new_serial("element"), name),
        code: String::new(),
        kind: ProgramElementKind::Room,
        parent_id: None,
        level: None,
        area: architect_program::QuantitySpec::default(),
        volume: architect_program::QuantitySpec::default(),
        height: architect_program::QuantitySpec::default(),
        occupancy: architect_program::QuantitySpec::default(),
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
    }
}

fn new_adjacency(program: &Program, a: &EntityId, b: &EntityId, kind: AdjacencyKind) -> Adjacency {
    let (left, right) = normalize_pair(a, b);
    Adjacency {
        header: EntityHeader::new(EntityId::new_serial("adjacency"), format!("{} ↔ {}", element_label(program, &left), element_label(program, &right))),
        element_a_id: left,
        element_b_id: right,
        kind,
        connection: ConnectionKind::Direct,
        separations: Vec::new(),
        weight: 1.0,
        rationale: None,
        distance_max_m: None,
        distance_min_m: None,
        level_constraint: None,
        access_path: None,
        shared_wall: false,
        shared_entry: false,
        traffic_isolation: false,
        circulation_overlap: false,
        conflict_ids: Vec::new(),
        normalized: true,
        verification_status: ValidationStatus::Pending,
        source_relationship_id: None,
        internal_external_access: None,
    }
}

fn store_runtime_json<T: Serialize>(runtime: &mut ArchitectPlayRuntime, value: &T) {
    runtime.last_report_json = serde_json::to_string_pretty(value).unwrap_or_else(|_| "{}".into());
}

fn parse_register_id(args: Option<&Value>) -> Option<String> {
    args.and_then(|value| value.get("registerId").or_else(|| value.get("register")).and_then(|v| v.as_str()).map(str::to_string))
}

fn parse_entity_id_from_args(args: Option<&Value>, key: &str) -> Option<EntityId> {
    args.and_then(|value| value.get(key)).and_then(|v| v.as_str()).map(|s| EntityId(s.into()))
}

fn entity_to_json<T: Serialize>(entity: &T) -> Value {
    serde_json::to_value(entity).unwrap_or(Value::Null)
}

fn register_entities(program: &Program, register: &str) -> Vec<Value> {
    macro_rules! collect {
        ($($name:literal => $field:ident),+ $(,)?) => {
            match register {
                $( $name => program.$field.iter().map(entity_to_json).collect(), )+
                _ => Vec::new(),
            }
        };
    }
    collect! {
        "stakeholders" => stakeholders,
        "users" => users,
        "activities" => activities,
        "functions" => functions,
        "elements" => elements,
        "quantities" => quantities,
        "relationships" => relationships,
        "adjacencies" => adjacencies,
        "processes" => processes,
        "flows" => flows,
        "access_rules" => access_rules,
        "operations" => operations,
        "equipment" => equipment,
        "resources" => resources,
        "storage" => storage,
        "environmental" => environmental,
        "human_factors" => human_factors,
        "accessibility" => accessibility,
        "privacy" => privacy,
        "safety" => safety,
        "security" => security,
        "regulatory" => regulatory,
        "site_context" => site_context,
        "organizational" => organizational,
        "services" => services,
        "infrastructure" => infrastructure,
        "information" => information,
        "communication" => communication,
        "wayfinding" => wayfinding,
        "schedules" => schedules,
        "flexibility" => flexibility,
        "growth" => growth,
        "sustainability" => sustainability,
        "resilience" => resilience,
        "costs" => costs,
        "delivery" => delivery,
        "risks" => risks,
        "conflicts" => conflicts,
        "requirements" => requirements,
        "priorities" => priorities,
        "scenarios" => scenarios,
        "options" => options,
        "decisions" => decisions,
        "validations" => validations,
        "performance" => performance,
        "quality" => quality,
        "documents" => documents,
        "assumptions" => assumptions,
        "constraints" => constraints,
        "compliance_records" => compliance_records,
        "approvals" => approvals,
        "meetings" => meetings,
        "changes" => changes,
        "collaboration" => collaboration,
        "analyses" => analyses,
        "reports" => reports,
        "search_filters" => search_filters,
        "status_records" => status_records,
        "workshops" => workshops,
        "surveys" => surveys,
        "issues" => issues,
        "audit_events" => audit_events,
        "templates" => templates,
        "knowledge" => knowledge,
        "benchmarks" => benchmarks,
        "traces" => traces,
    }
}

fn register_len(program: &Program, register: &str) -> usize {
    register_entities(program, register).len()
}

fn find_register_for_entity(program: &Program, id: &EntityId) -> Option<&'static str> {
    if program.traces.iter().any(|row| row.id == *id) {
        return Some("traces");
    }
    macro_rules! find {
        ($($name:literal => $field:ident),+ $(,)?) => {
            $( if program.$field.iter().any(|row| row.header.id == *id) { return Some($name); } )+
        };
    }
    find! {
        "stakeholders" => stakeholders,
        "users" => users,
        "activities" => activities,
        "functions" => functions,
        "elements" => elements,
        "quantities" => quantities,
        "relationships" => relationships,
        "adjacencies" => adjacencies,
        "processes" => processes,
        "flows" => flows,
        "access_rules" => access_rules,
        "operations" => operations,
        "equipment" => equipment,
        "resources" => resources,
        "storage" => storage,
        "environmental" => environmental,
        "human_factors" => human_factors,
        "accessibility" => accessibility,
        "privacy" => privacy,
        "safety" => safety,
        "security" => security,
        "regulatory" => regulatory,
        "site_context" => site_context,
        "organizational" => organizational,
        "services" => services,
        "infrastructure" => infrastructure,
        "information" => information,
        "communication" => communication,
        "wayfinding" => wayfinding,
        "schedules" => schedules,
        "flexibility" => flexibility,
        "growth" => growth,
        "sustainability" => sustainability,
        "resilience" => resilience,
        "costs" => costs,
        "delivery" => delivery,
        "risks" => risks,
        "conflicts" => conflicts,
        "requirements" => requirements,
        "priorities" => priorities,
        "scenarios" => scenarios,
        "options" => options,
        "decisions" => decisions,
        "validations" => validations,
        "performance" => performance,
        "quality" => quality,
        "documents" => documents,
        "assumptions" => assumptions,
        "constraints" => constraints,
        "compliance_records" => compliance_records,
        "approvals" => approvals,
        "meetings" => meetings,
        "changes" => changes,
        "collaboration" => collaboration,
        "analyses" => analyses,
        "reports" => reports,
        "search_filters" => search_filters,
        "status_records" => status_records,
        "workshops" => workshops,
        "surveys" => surveys,
        "issues" => issues,
        "audit_events" => audit_events,
        "templates" => templates,
        "knowledge" => knowledge,
        "benchmarks" => benchmarks,
    }
    None
}

fn default_entity_header(register: &str, label: &str) -> EntityHeader {
    let prefix = register.trim_end_matches('s').trim_end_matches("_records");
    EntityHeader::new(EntityId::new_serial(prefix), label)
}

fn default_stakeholder(label: &str) -> Stakeholder {
    Stakeholder {
        header: default_entity_header("stakeholders", label),
        role: String::new(),
        organization: String::new(),
        department: None,
        contact_email: None,
        contact_phone: None,
        influence: InfluenceLevel::Medium,
        interest: InfluenceLevel::Medium,
        engagement: EngagementLevel::Neutral,
        expectations: Vec::new(),
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
        stakeholder_type: String::new(),
        influence_strategy: None,
        communication_channels: Vec::new(),
        success_metrics: Vec::new(),
    }
}

fn default_requirement(label: &str) -> Requirement {
    Requirement {
        header: default_entity_header("requirements", label),
        code: String::new(),
        kind: RequirementKind::Functional,
        statement: TextField::plain(""),
        rationale: None,
        source: None,
        stakeholder_ids: Vec::new(),
        element_ids: Vec::new(),
        function_ids: Vec::new(),
        parent_requirement_id: None,
        child_requirement_ids: Vec::new(),
        acceptance_criteria: Vec::new(),
        verification_method: None,
        validation_status: ValidationStatus::Pending,
        conflict_ids: Vec::new(),
        risk_ids: Vec::new(),
        cost_estimate: None,
        schedule_constraint: None,
        regulatory_refs: Vec::new(),
        trace_links: Vec::new(),
        superseded_by: None,
    }
}

fn default_risk(label: &str) -> Risk {
    Risk {
        header: default_entity_header("risks", label),
        risk_statement: TextField::plain(""),
        category: String::new(),
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
    }
}

fn default_issue(label: &str) -> Issue {
    Issue {
        header: default_entity_header("issues", label),
        issue_type: String::new(),
        summary: TextField::plain(""),
        issue_description: TextField::plain(""),
        severity: IssueSeverity::Minor,
        issue_priority: architect_program::Priority::Preferred,
        reporter_id: None,
        assignee_id: None,
        affected_entity_ids: Vec::new(),
        root_cause: None,
        resolution: None,
        workaround: None,
        due_date: None,
        resolved_date: None,
        related_conflict_ids: Vec::new(),
        related_risk_ids: Vec::new(),
        decision_id: None,
        comments: Vec::new(),
        attachments: Vec::new(),
        escalation_level: None,
    }
}

fn default_function(label: &str) -> Function {
    Function {
        header: default_entity_header("functions", label),
        code: String::new(),
        kind: FunctionKind::Primary,
        purpose: TextField::plain(""),
        criticality: architect_program::Priority::Preferred,
        performance_targets: Vec::new(),
        service_level: None,
        operating_hours: None,
        staffing: architect_program::QuantitySpec::default(),
        equipment_ids: Vec::new(),
        resource_ids: Vec::new(),
        activity_ids: Vec::new(),
        element_ids: Vec::new(),
        dependencies: Vec::new(),
        interfaces: Vec::new(),
        constraints: Vec::new(),
        quality_criteria: Vec::new(),
        regulatory_refs: Vec::new(),
        future_changes: Vec::new(),
        owner_stakeholder_id: None,
        success_metrics: Vec::new(),
        hierarchy_parent_id: None,
        conflict_ids: Vec::new(),
    }
}

fn default_user(label: &str) -> UserProfile {
    UserProfile {
        header: default_entity_header("users", label),
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
        goals: Vec::new(),
        activity_ids: Vec::new(),
        research_method: None,
        persona_archetype: None,
        validated: false,
        stakeholder_ids: Vec::new(),
    }
}

fn default_from_json<T: DeserializeOwned>(register: &str, label: &str, extra: Value) -> Option<T> {
    let mut value = match extra {
        Value::Object(map) => Value::Object(map),
        _ => Value::Object(serde_json::Map::new()),
    };
    if let Value::Object(ref mut map) = value {
        map.insert("id".into(), json!(EntityId::new_serial(register)));
        map.insert("name".into(), json!(label));
    }
    serde_json::from_value(value).ok()
}

fn add_register_item_op(program: &Program, register: &str, label: &str) -> Option<(ProgramOp, EntityId)> {
    macro_rules! add {
        ($field:ident, $op:ident, $item:expr) => {{
            let item = $item;
            let id = item.header.id.clone();
            (ProgramOp::$op(CollectionOp::Add { index: program.$field.len(), item }), id)
        }};
    }
    Some(match register {
        "elements" => {
            let item = default_element(label);
            let id = item.header.id.clone();
            (ProgramOp::Elements(CollectionOp::Add { index: program.elements.len(), item }), id)
        }
        "stakeholders" => add!(stakeholders, Stakeholders, default_stakeholder(label)),
        "requirements" => add!(requirements, Requirements, default_requirement(label)),
        "risks" => add!(risks, Risks, default_risk(label)),
        "issues" => add!(issues, Issues, default_issue(label)),
        "functions" => add!(functions, Functions, default_function(label)),
        "users" => add!(users, Users, default_user(label)),
        "activities" => {
            let item: architect_program::Activity = default_from_json("activities", label, json!({ "code": "ACT", "category": "general", "activityType": "general" }))?;
            add!(activities, Activities, item)
        }
        "assumptions" => add!(assumptions, Assumptions, default_from_json::<architect_program::Assumption>("assumptions", label, json!({ "statement": { "text": "" }, "validationStatus": "pending" }),)?),
        "constraints" => {
            add!(constraints, Constraints, default_from_json::<architect_program::ConstraintRecord>("constraints", label, json!({ "constraintType": "general", "summary": { "text": "" }, "severity": "medium", "complianceStatus": "pending" }),)?)
        }
        "compliance_records" => {
            add!(compliance_records, ComplianceRecords, default_from_json::<architect_program::ComplianceRecord>("compliance", label, json!({ "standardRef": "", "obligation": { "text": "" }, "complianceStatus": "pending", "severity": "medium" }),)?)
        }
        "approvals" => add!(
            approvals,
            Approvals,
            default_from_json::<architect_program::ApprovalRecord>(
                "approvals",
                label,
                json!({
                    "approvalType": "general",
                    "subjectId": EntityId::new_serial("subject"),
                    "approvalStatus": "draft"
                }),
            )?
        ),
        "meetings" => add!(meetings, Meetings, default_from_json::<architect_program::MeetingRecord>("meetings", label, json!({ "meetingType": "workshop", "quorumMet": false, "meetingStatus": "draft" }),)?),
        "analyses" => add!(analyses, Analyses, default_from_json::<AnalysisRecord>("analysis", label, json!({ "kind": "gap", "title": label, "outputSummary": { "text": "" } }),)?),
        "reports" => add!(reports, Reports, default_from_json::<ReportRecord>("report", label, json!({ "kind": "executiveSummary", "title": label, "approvalStatus": "pending", "version": "0" }),)?),
        "templates" => add!(templates, Templates, default_from_json::<architect_program::TemplateRecord>("template", label, json!({ "templateType": "sector", "version": "1", "approvalStatus": "pending", "usageCount": 0 }),)?),
        "traces" => {
            let from = program.elements.first().map_or_else(|| EntityId::new_serial("from"), |element| element.header.id.clone());
            let to = program.elements.get(1).map_or_else(|| EntityId::new_serial("to"), |element| element.header.id.clone());
            let item = TraceLink::new(from, to, TraceKind::FunctionToProgramElement);
            let id = item.id.clone();
            (ProgramOp::Traces(CollectionOp::Add { index: program.traces.len(), item }), id)
        }
        _ => return None,
    })
}

fn remove_register_item_op(register: &str, entity_id: EntityId) -> Option<ProgramOp> {
    macro_rules! remove {
        ($op:ident) => {
            ProgramOp::$op(CollectionOp::Remove { id: entity_id })
        };
    }
    Some(match register {
        "stakeholders" => remove!(Stakeholders),
        "users" => remove!(Users),
        "activities" => remove!(Activities),
        "functions" => remove!(Functions),
        "elements" => remove!(Elements),
        "quantities" => remove!(Quantities),
        "relationships" => remove!(Relationships),
        "adjacencies" => remove!(Adjacencies),
        "processes" => remove!(Processes),
        "flows" => remove!(Flows),
        "access_rules" => remove!(AccessRules),
        "operations" => remove!(Operations),
        "equipment" => remove!(Equipment),
        "resources" => remove!(Resources),
        "storage" => remove!(Storage),
        "environmental" => remove!(Environmental),
        "human_factors" => remove!(HumanFactors),
        "accessibility" => remove!(Accessibility),
        "privacy" => remove!(Privacy),
        "safety" => remove!(Safety),
        "security" => remove!(Security),
        "regulatory" => remove!(Regulatory),
        "site_context" => remove!(SiteContext),
        "organizational" => remove!(Organizational),
        "services" => remove!(Services),
        "infrastructure" => remove!(Infrastructure),
        "information" => remove!(Information),
        "communication" => remove!(Communication),
        "wayfinding" => remove!(Wayfinding),
        "schedules" => remove!(Schedules),
        "flexibility" => remove!(Flexibility),
        "growth" => remove!(Growth),
        "sustainability" => remove!(Sustainability),
        "resilience" => remove!(Resilience),
        "costs" => remove!(Costs),
        "delivery" => remove!(Delivery),
        "risks" => remove!(Risks),
        "conflicts" => remove!(Conflicts),
        "requirements" => remove!(Requirements),
        "priorities" => remove!(Priorities),
        "scenarios" => remove!(Scenarios),
        "options" => remove!(Options),
        "decisions" => remove!(Decisions),
        "validations" => remove!(Validations),
        "performance" => remove!(Performance),
        "quality" => remove!(Quality),
        "documents" => remove!(Documents),
        "assumptions" => remove!(Assumptions),
        "constraints" => remove!(Constraints),
        "compliance_records" => remove!(ComplianceRecords),
        "approvals" => remove!(Approvals),
        "meetings" => remove!(Meetings),
        "changes" => remove!(Changes),
        "collaboration" => remove!(Collaboration),
        "analyses" => remove!(Analyses),
        "reports" => remove!(Reports),
        "search_filters" => remove!(SearchFilters),
        "status_records" => remove!(StatusRecords),
        "workshops" => remove!(Workshops),
        "surveys" => remove!(Surveys),
        "issues" => remove!(Issues),
        "audit_events" => remove!(AuditEvents),
        "templates" => remove!(Templates),
        "knowledge" => remove!(Knowledge),
        "benchmarks" => remove!(Benchmarks),
        "traces" => remove!(Traces),
        _ => return None,
    })
}

fn patch_register_item_op(register: &str, entity_id: EntityId, patch: Value) -> Option<ProgramOp> {
    macro_rules! patch {
        ($op:ident, $ty:ty) => {
            ProgramOp::$op(CollectionOp::Patch { id: entity_id, patch: serde_json::from_value::<$ty>(patch).ok()? })
        };
    }
    Some(match register {
        "stakeholders" => patch!(Stakeholders, StakeholderPatch),
        "elements" => patch!(Elements, ProgramElementPatch),
        "adjacencies" => patch!(Adjacencies, AdjacencyPatch),
        "requirements" => patch!(Requirements, architect_program::RequirementPatch),
        "risks" => patch!(Risks, architect_program::RiskPatch),
        "issues" => patch!(Issues, architect_program::IssuePatch),
        "functions" => patch!(Functions, architect_program::FunctionPatch),
        "users" => patch!(Users, architect_program::UserProfilePatch),
        _ => return None,
    })
}

fn analysis_record_from(program: &Program, kind: AnalysisKind, result: &AnalysisResult) -> AnalysisRecord {
    AnalysisRecord {
        header: EntityHeader::new(EntityId::new_serial("analysis"), result.title.clone()),
        kind,
        title: result.title.clone(),
        parameters: Vec::new(),
        input_entity_ids: result.entity_ids.clone(),
        output_summary: TextField::plain(&result.summary),
        findings: result.findings.clone(),
        metrics: result.metrics.iter().map(|metric| format!("{}={}{}", metric.name, metric.value, metric.unit.as_deref().unwrap_or(""))).collect(),
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
    }
}

fn report_record_from(program: &Program, kind: ReportKind, report: &ProgramReport) -> ReportRecord {
    ReportRecord {
        header: EntityHeader::new(EntityId::new_serial("report"), report.title.clone()),
        kind,
        title: report.title.clone(),
        audience: Vec::new(),
        sections: report.sections.iter().map(|section| section.heading.clone()).collect(),
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
    }
}

fn inspector_patch_action(register_id: &str, entity_id: &str, patch: &Value) -> ActionDescriptor {
    architect_action("patchRegisterItem", Some(json!({ "registerId": register_id, "entityId": entity_id, "patch": patch })))
}

fn inspector_text_field(register_id: &str, entity_id: &str, field_id: &str, label: &str, values: &[String], key: &str) -> UiNode {
    let mixed = ui_inspector_mixed_text(values);
    let patch_value = mixed.value.clone();
    UiNode::Field(UiFieldNode {
        id: field_id.into(),
        label: label.into(),
        child: Box::new(UiNode::Input(UiInputNode {
            id: format!("{field_id}.input"),
            input_kind: "text".into(),
            value: mixed.value,
            placeholder: mixed.placeholder,
            commit: Some("blur".into()),
            on_change: inspector_patch_action(register_id, entity_id, &json!({ key: patch_value })),
            min: None,
            max: None,
            step: None,
            accept: None,
        })),
        description: None,
        required: None,
        error: None,
    })
}

fn inspector_number_field(register_id: &str, entity_id: &str, field_id: &str, label: &str, values: &[f64], key: &str) -> UiNode {
    let mixed = ui_inspector_mixed_number(values);
    let patch_value = mixed.value;
    UiNode::Field(UiFieldNode {
        id: field_id.into(),
        label: label.into(),
        child: Box::new(UiNode::NumberStepper(UiNumberStepperNode {
            id: format!("{field_id}.stepper"),
            value: mixed.value,
            step: 0.1,
            uniform: mixed.uniform,
            on_absolute: inspector_patch_action(register_id, entity_id, &json!({ key: patch_value })),
            on_delta: inspector_patch_action(register_id, entity_id, &json!({ key: patch_value })),
        })),
        description: None,
        required: None,
        error: None,
    })
}

fn inspector_toggle_field(register_id: &str, entity_id: &str, field_id: &str, label: &str, values: &[bool], key: &str) -> UiNode {
    let mixed = ui_inspector_mixed_toggle(values);
    let patch_value = mixed.pressed;
    UiNode::Field(UiFieldNode {
        id: field_id.into(),
        label: label.into(),
        child: Box::new(UiNode::Toggle(UiToggleNode {
            id: format!("{field_id}.toggle"),
            icon_id: "check".into(),
            pressed: mixed.pressed,
            text: Some(if mixed.pressed { "Yes".into() } else { "No".into() }),
            on_change: inspector_patch_action(register_id, entity_id, &json!({ key: patch_value })),
        })),
        description: None,
        required: None,
        error: None,
    })
}

fn empty_component_scene(surface_id: &str, component_kind: SurfaceKind) -> UiComponentSceneNode {
    UiComponentSceneNode {
        surface_id: surface_id.into(),
        controller_id: ARCHITECT_APP_ID.into(),
        component_kind,
        pane_id: None,
        binding_id: None,
        canvas_2d: None,
        world_3d: None,
        node_graph: None,
        text_editor: None,
        table: None,
        paint_2d: None,
        virtual_file_system: None,
        tiled_map: None,
        board2d: None,
        icon_render: None,
        ink_canvas: None,
        graph_timeline: None,
        block_list: None,
        diff_view: None,
        event_feed: None,
    }
}

fn parse_entity_id(value: Option<&Value>, key: &str) -> Option<EntityId> {
    value.and_then(|args| args.get(key)).and_then(|v| v.as_str()).map(|s| EntityId(s.into()))
}

fn parse_adjacency_kind(value: Option<&Value>) -> Option<AdjacencyKind> {
    value.and_then(|args| args.get("kind")).and_then(|v| v.as_str()).and_then(|kind| match kind {
        "required" => Some(AdjacencyKind::Required),
        "preferred" => Some(AdjacencyKind::Preferred),
        "optional" => Some(AdjacencyKind::Optional),
        "prohibited" => Some(AdjacencyKind::Prohibited),
        _ => None,
    })
}

fn analysis_kind_from_args(args: Option<&Value>) -> AnalysisKind {
    args.and_then(|value| value.get("analysisKind")).and_then(|v| v.as_str()).map_or(AnalysisKind::Gap, |kind| match kind {
        "gap" => AnalysisKind::Gap,
        "conflict" => AnalysisKind::Conflict,
        "dependency" => AnalysisKind::Dependency,
        "capacity" => AnalysisKind::Capacity,
        "demand" => AnalysisKind::Demand,
        "utilization" => AnalysisKind::Utilization,
        "workflow" => AnalysisKind::Workflow,
        "risk" => AnalysisKind::Risk,
        "cost" => AnalysisKind::Cost,
        "scenario" => AnalysisKind::Scenario,
        "sensitivity" => AnalysisKind::Sensitivity,
        "impact" => AnalysisKind::Impact,
        "trend" => AnalysisKind::Trend,
        "requirementComparison" => AnalysisKind::RequirementComparison,
        "requirementClustering" => AnalysisKind::RequirementClustering,
        "requirementFiltering" => AnalysisKind::RequirementFiltering,
        "requirementSorting" => AnalysisKind::RequirementSorting,
        "requirementScoring" => AnalysisKind::RequirementScoring,
        "requirementWeighting" => AnalysisKind::RequirementWeighting,
        "relationshipAnalysis" => AnalysisKind::RelationshipAnalysis,
        _ => AnalysisKind::Gap,
    })
}

fn report_kind_from_args(args: Option<&Value>) -> ReportKind {
    args.and_then(|value| value.get("reportKind")).and_then(|v| v.as_str()).map_or(ReportKind::ExecutiveSummary, |kind| match kind {
        "executiveSummary" => ReportKind::ExecutiveSummary,
        "programOverview" => ReportKind::ProgramOverview,
        "stakeholderSummary" => ReportKind::StakeholderSummary,
        "requirementsMatrix" => ReportKind::RequirementsMatrix,
        "adjacencyMatrix" => ReportKind::AdjacencyMatrix,
        "gapAnalysis" => ReportKind::GapAnalysis,
        "riskRegister" => ReportKind::RiskRegister,
        "decisionLog" => ReportKind::DecisionLog,
        "validationSummary" => ReportKind::ValidationSummary,
        "recommendation" => ReportKind::Recommendation,
        "userSummary" => ReportKind::UserSummary,
        "functionalSummary" => ReportKind::FunctionalSummary,
        "capacitySummary" => ReportKind::CapacitySummary,
        "workflowSummary" => ReportKind::WorkflowSummary,
        "complianceSummary" => ReportKind::ComplianceSummary,
        "costSummary" => ReportKind::CostSummary,
        "scheduleSummary" => ReportKind::ScheduleSummary,
        "changeSummary" => ReportKind::ChangeSummary,
        "openIssueSummary" => ReportKind::OpenIssueSummary,
        "prioritySummary" => ReportKind::PrioritySummary,
        "scenarioSummary" => ReportKind::ScenarioSummary,
        _ => ReportKind::ExecutiveSummary,
    })
}

fn analysis_kind_picker_options() -> Vec<ActionArgOption> {
    vec![
        ("gap", "Gap"),
        ("conflict", "Conflict"),
        ("dependency", "Dependency"),
        ("capacity", "Capacity"),
        ("demand", "Demand"),
        ("utilization", "Utilization"),
        ("workflow", "Workflow"),
        ("risk", "Risk"),
        ("cost", "Cost"),
        ("scenario", "Scenario"),
        ("sensitivity", "Sensitivity"),
        ("impact", "Impact"),
        ("trend", "Trend"),
        ("requirementComparison", "Requirement Comparison"),
        ("requirementClustering", "Requirement Clustering"),
        ("requirementFiltering", "Requirement Filtering"),
        ("requirementSorting", "Requirement Sorting"),
        ("requirementScoring", "Requirement Scoring"),
        ("requirementWeighting", "Requirement Weighting"),
        ("relationshipAnalysis", "Relationship Analysis"),
    ]
    .into_iter()
    .map(|(id, label)| ActionArgOption::new(id, label))
    .collect()
}

fn report_kind_picker_options() -> Vec<ActionArgOption> {
    vec![
        ("executiveSummary", "Executive Summary"),
        ("programOverview", "Program Overview"),
        ("stakeholderSummary", "Stakeholder Summary"),
        ("requirementsMatrix", "Requirements Matrix"),
        ("adjacencyMatrix", "Adjacency Matrix"),
        ("gapAnalysis", "Gap Analysis"),
        ("riskRegister", "Risk Register"),
        ("decisionLog", "Decision Log"),
        ("validationSummary", "Validation Summary"),
        ("recommendation", "Recommendation"),
        ("userSummary", "User Summary"),
        ("functionalSummary", "Functional Summary"),
        ("capacitySummary", "Capacity Summary"),
        ("workflowSummary", "Workflow Summary"),
        ("complianceSummary", "Compliance Summary"),
        ("costSummary", "Cost Summary"),
        ("scheduleSummary", "Schedule Summary"),
        ("changeSummary", "Change Summary"),
        ("openIssueSummary", "Open Issue Summary"),
        ("prioritySummary", "Priority Summary"),
        ("scenarioSummary", "Scenario Summary"),
    ]
    .into_iter()
    .map(|(id, label)| ActionArgOption::new(id, label))
    .collect()
}

fn entity_id_from_json(value: &Value) -> Option<String> {
    value.get("id").and_then(|id| id.as_str()).map(str::to_string).or_else(|| value.get("header").and_then(|header| header.get("id")).and_then(|id| id.as_str()).map(str::to_string))
}

fn entity_name_from_json(value: &Value) -> String {
    value.get("name").and_then(|name| name.as_str()).map(str::to_string).or_else(|| value.get("header").and_then(|header| header.get("name")).and_then(|name| name.as_str()).map(str::to_string)).unwrap_or_else(|| "Untitled".into())
}
//#endregion 🔖Helpers

//#region 🔖AdjacencyRender
/// @emoji 🔺 Signature adjacency matrix — triangle glyph strip plus lower-triangle pair rows.
fn render_adjacency_body(program: &Program, runtime: &ArchitectPlayRuntime) -> UiNode {
    let matrix = adjacency_matrix(program);
    let n = matrix.element_ids.len();
    if n == 0 {
        return ui_text("Add program elements to edit adjacencies.");
    }

    let mut glyph_rows = Vec::new();
    let mut pair_sections = Vec::new();

    glyph_rows.push(ui_text(" "));
    pair_sections.push(tree_section("architect-adjacency.headers", Some("Columns".into()), matrix.element_ids.iter().enumerate().map(|(index, id)| tree_item(format!("architect-adjacency.col.{index}"), element_label(program, id))).collect()));

    for row in 1..n {
        let row_id = &matrix.element_ids[row];
        let glyph = "▲".repeat(row);
        glyph_rows.push(ui_text(glyph));

        let mut items = Vec::new();
        for col in 0..row {
            let col_id = &matrix.element_ids[col];
            let cell = &matrix.cells[row][col];
            if let Some(filter) = &runtime.adjacency_kind_filter {
                match cell {
                    Some(existing) if &existing.kind != filter => continue,
                    None => continue,
                    _ => {}
                }
            }
            let kind_label = cell.as_ref().map_or_else(|| "—".into(), |existing| adjacency_kind_label(&existing.kind).to_string());
            let label = format!("{} ↔ {} [{kind_label}]", element_label(program, col_id), element_label(program, row_id));
            items.push(tree_item_with_action(
                format!("architect-adjacency.pair.{}-{}", col_id, row_id),
                label,
                None,
                architect_action(
                    "setAdjacencyKind",
                    Some(json!({
                        "elementAId": col_id,
                        "elementBId": row_id,
                        "cycle": true
                    })),
                ),
            ));
        }

        pair_sections.push(tree_section(format!("architect-adjacency.row.{row}"), Some(element_label(program, row_id)), items));
    }

    let conflicts = detect_adjacency_conflicts(program);
    if !conflicts.is_empty() {
        pair_sections.push(tree_section(
            "architect-adjacency.conflicts",
            Some(format!("Conflicts ({})", conflicts.len())),
            conflicts.iter().map(|conflict| tree_item(format!("architect-adjacency.conflict.{}", conflict.adjacency_a_id), &conflict.message)).collect(),
        ));
    }

    UiNode::Stack(UiStackNode {
        direction: "row".into(),
        gap: Some("0.5rem".into()),
        padding: None,
        id: Some("architect-adjacency.matrix".into()),
        selected: None,
        loading: None,
        waiting: None,
        activate: None,
        drop_action: None,
        drop_overlay: None,
        children: vec![ui_stack_vertical(glyph_rows), tree_node(pair_sections, None)],
    })
}
//#endregion 🔖AdjacencyRender

//#region 🔖GraphRender
fn graph_media_json(program: &Program, _camera: &GraphCamera) -> (String, String) {
    let count = program.elements.len().max(1);
    let radius = 220.0;
    let center_x = 320.0;
    let center_y = 240.0;
    let nodes: Vec<Value> = program
        .elements
        .iter()
        .enumerate()
        .map(|(index, element)| {
            let angle = std::f64::consts::TAU * (index as f64) / (count as f64);
            json!({
                "id": element.header.id,
                "label": element.header.name,
                "x": center_x + radius * angle.cos(),
                "y": center_y + radius * angle.sin(),
                "width": 108.0,
                "height": 44.0,
                "inputs": [{"id": "in"}],
                "outputs": [{"id": "out"}],
            })
        })
        .collect();
    let edges: Vec<Value> = undirected_edges(program)
        .iter()
        .enumerate()
        .map(|(index, (source, target, weight))| {
            json!({
                "id": format!("edge-{index}"),
                "sourceNodeId": source,
                "sourcePortId": "out",
                "targetNodeId": target,
                "targetPortId": "in",
                "label": format!("{weight:.1}"),
            })
        })
        .collect();
    (serde_json::to_string(&nodes).unwrap_or_else(|_| "[]".into()), serde_json::to_string(&edges).unwrap_or_else(|_| "[]".into()))
}

fn render_graph_body(program: &Program, runtime: &ArchitectPlayRuntime) -> UiNode {
    let (nodes_json, edges_json) = graph_media_json(program, &runtime.graph_camera);
    let viewport_json = serde_json::to_string(&runtime.graph_camera).unwrap_or_else(|_| r#"{"x":0,"y":0,"zoom":1}"#.into());
    let mut scene = empty_component_scene(ARCHITECT_BODY_GRAPH, SurfaceKind::NodeGraph);
    scene.node_graph = Some(NodeGraphScene {
        editable: Some(true),
        capabilities_json: Some(r#"{"directedness":"undirected"}"#.into()),
        selection_json: if runtime.selected_ids.is_empty() { None } else { Some(serde_json::to_string(&runtime.selected_ids).unwrap_or_else(|_| "[]".into())) },
        ..NodeGraphScene::base(nodes_json, edges_json, viewport_json)
    });
    UiNode::ComponentScene(scene)
}
//#endregion 🔖GraphRender

//#region 🔖RegisterRender
fn render_register_body(program: &Program, runtime: &ArchitectPlayRuntime) -> UiNode {
    let register = if runtime.active_register.is_empty() { "elements" } else { runtime.active_register.as_str() };
    let entities = register_entities(program, register);
    if entities.is_empty() {
        return ui_text(format!("No entities in register '{register}'."));
    }

    let steps: Vec<RegisterBlockStep> = entities
        .iter()
        .filter_map(|entity| {
            let id = entity_id_from_json(entity)?;
            let name = entity_name_from_json(entity);
            Some(RegisterBlockStep { id: id.clone(), title: name.clone(), blocks: vec![RegisterBlockItem { id: format!("{id}-block"), label: name, kind: register.into() }] })
        })
        .collect();
    let steps_json = serde_json::to_string(&steps).unwrap_or_else(|_| "[]".into());
    let palette_json = serde_json::to_string(&[json!({
        "blockKind": register,
        "label": register,
        "iconId": "square",
    })])
    .unwrap_or_else(|_| "[]".into());
    let selected_id = runtime.selected_ids.first().cloned();
    let mut scene = empty_component_scene(ARCHITECT_BODY_REGISTER, SurfaceKind::BlockList);
    scene.block_list = Some(BlockListScene { steps_json, palette_json, selected_id, dragging_id: None });
    UiNode::ComponentScene(scene)
}

//#endregion 🔖RegisterRender

//#region 🔖Panels
fn build_document_tree(program: &Program, runtime: &ArchitectPlayRuntime) -> UiNode {
    let summary = status_summary(program);
    let element_items: Vec<UiTreeItemNode> = program
        .elements
        .iter()
        .map(|element| {
            tree_item_with_action(
                format!("architect-document.element.{}", element.header.id),
                format!("{} ({:?})", element.header.name, element.kind),
                Some(element.header.id.to_string()),
                architect_action("setSelection", Some(json!({ "ids": [element.header.id] }))),
            )
        })
        .collect();
    let register_items: Vec<UiTreeItemNode> = summary
        .by_register
        .iter()
        .map(|row| tree_item_with_action(format!("architect-document.register.{}", row.register), format!("{} ({})", row.register, row.count), None, architect_action("selectRegister", Some(json!({ "registerId": row.register })))))
        .collect();
    tree_node(
        vec![
            tree_section(
                "architect-document.meta",
                Some("Program".into()),
                vec![
                    tree_item("architect-document.meta.title", format!("Title: {}", program.meta.title)),
                    tree_item("architect-document.meta.project", format!("Project: {} ({})", program.project.client_name, program.project.code)),
                    tree_item("architect-document.meta.entities", format!("Entities tracked: {} (active register: {} / {})", summary.total_entities, runtime.active_register, register_len(program, &runtime.active_register))),
                ],
            ),
            tree_section("architect-document.registers", Some("Registers".into()), register_items),
            tree_section("architect-document.elements", Some("Elements".into()), if element_items.is_empty() { vec![tree_item("architect-document.elements.empty", "(none)")] } else { element_items }),
        ],
        Some(runtime.selected_ids.iter().map(|id| format!("architect-document.element.{id}")).collect()),
    )
}

fn build_catalogue_tree() -> UiNode {
    let register_items: Vec<UiTreeItemNode> =
        REGISTER_IDS.iter().map(|register| tree_item_with_action(format!("architect-catalogue.register.{register}"), *register, None, architect_action("selectRegister", Some(json!({ "registerId": register }))))).collect();
    tree_node(
        vec![
            tree_section(
                "architect-catalogue.actions",
                Some(FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL.into()),
                vec![
                    tree_item_with_action("architect-catalogue.add-item", "Add Register Item", None, architect_action("addRegisterItem", Some(json!({ "registerId": "elements", "template": null })))),
                    tree_item_with_action("architect-catalogue.validate", "Run Validation", None, architect_action("runValidation", None)),
                    tree_item_with_action("architect-catalogue.analysis", "Run Analysis", None, architect_action("runAnalysis", Some(json!({ "analysisKind": "gap" })))),
                    tree_item_with_action("architect-catalogue.report", "Run Report", None, architect_action("runReport", Some(json!({ "reportKind": "executiveSummary" })))),
                    tree_item_with_action("architect-catalogue.export", "Export Program", None, architect_action("exportProgram", None)),
                    tree_item_with_action("architect-catalogue.import", "Import Program", None, architect_action("importProgram", Some(json!({ "json": "" })))),
                    tree_item_with_action("architect-catalogue.export-csv", "Export Registers CSV", None, architect_action("exportRegistersCsv", None)),
                    tree_item_with_action("architect-catalogue.import-csv", "Import Registers CSV", None, architect_action("importRegistersCsv", Some(json!({ "csv": "", "strategy": "upsert" })))),
                    tree_item_with_action("architect-catalogue.apply-template", "Apply Template", None, architect_action("applyTemplate", Some(json!({ "templateId": "" })))),
                    tree_item_with_action("architect-catalogue.search", "Search Program", None, architect_action("search", Some(json!({ "query": "" })))),
                ],
            ),
            tree_section("architect-catalogue.registers", Some("Registers".into()), register_items),
        ],
        None,
    )
}

fn render_report_body(runtime: &ArchitectPlayRuntime) -> UiNode {
    let Some(report) = &runtime.last_report else {
        return ui_text("Run validation, analysis, or report to populate this panel.");
    };
    let sections: Vec<UiTreeSectionNode> = report
        .sections
        .iter()
        .enumerate()
        .map(|(index, section)| {
            let mut items = Vec::new();
            if !section.body.is_empty() {
                items.push(tree_item(format!("architect-report.section.{index}.body"), &section.body));
            }
            for (bullet_index, bullet) in section.bullets.iter().enumerate() {
                items.push(tree_item(format!("architect-report.section.{index}.bullet.{bullet_index}"), format!("• {bullet}")));
            }
            tree_section(format!("architect-report.section.{index}"), Some(section.heading.clone()), items)
        })
        .collect();
    tree_node(
        vec![tree_section("architect-report.meta", Some(report.title.clone()), vec![tree_item("architect-report.kind", format!("Kind: {:?}", report.kind)), tree_item("architect-report.generated", format!("Generated: {}", report.generated_at))])]
            .into_iter()
            .chain(sections)
            .collect(),
        None,
    )
}

fn render_trace_body(program: &Program, runtime: &ArchitectPlayRuntime) -> UiNode {
    if runtime.selected_ids.is_empty() {
        return ui_text("Select an entity to inspect trace chains and impact.");
    }
    let root = EntityId(runtime.selected_ids[0].clone());
    let mut scratch = program.clone();
    let chain: TraceChain = trace_chain(&mut scratch, &root);
    let impact = trace_impact(&mut scratch, &root);
    let trail = audit_trail(program, Some(&root));
    let chain_items: Vec<UiTreeItemNode> = chain.links.iter().enumerate().map(|(index, link)| tree_item(format!("architect-trace.chain.{index}"), format!("{:?}: {} → {}", link.kind, link.from_id, link.to_id))).collect();
    let impact_items: Vec<UiTreeItemNode> = impact.upstream_ids.iter().enumerate().map(|(index, id)| tree_item(format!("architect-trace.impact.{index}"), id.to_string())).collect();
    let audit_items: Vec<UiTreeItemNode> = trail.events.iter().take(12).enumerate().map(|(index, event)| tree_item(format!("architect-trace.audit.{index}"), format!("{:?} @ {} — {}", event.action, event.timestamp, event.header.name))).collect();
    tree_node(
        vec![
            tree_section("architect-trace.chain", Some(format!("Trace Chain ({})", chain.links.len())), if chain_items.is_empty() { vec![tree_item("architect-trace.chain.empty", "(no links)")] } else { chain_items }),
            tree_section("architect-trace.impact", Some(format!("Impact ({})", impact.upstream_ids.len())), if impact_items.is_empty() { vec![tree_item("architect-trace.impact.empty", "(no upstream)")] } else { impact_items }),
            tree_section("architect-trace.audit", Some(format!("Audit Trail ({})", trail.events.len())), if audit_items.is_empty() { vec![tree_item("architect-trace.audit.empty", "(no events)")] } else { audit_items }),
        ],
        None,
    )
}

fn build_inspection_tree(program: &Program, runtime: &ArchitectPlayRuntime) -> UiNode {
    if runtime.selected_ids.is_empty() {
        return ui_stack_vertical(vec![ui_text("Select an entity in the document or register view.")]);
    }
    let id = EntityId(runtime.selected_ids[0].clone());
    let register = find_register_for_entity(program, &id).unwrap_or("elements");
    let entity_id = id.to_string();
    if let Some(element) = program.elements.iter().find(|row| row.header.id == id) {
        let fields = vec![
            ui_inspector_readonly_field("architect-inspection.element.id", "Id", entity_id.clone()),
            inspector_text_field(register, &entity_id, "architect-inspection.element.name", "Name", std::slice::from_ref(&element.header.name), "name"),
            inspector_text_field(register, &entity_id, "architect-inspection.element.code", "Code", std::slice::from_ref(&element.code), "code"),
            inspector_text_field(register, &entity_id, "architect-inspection.element.level", "Level", &[element.level.clone().unwrap_or_default()], "level"),
            ui_inspector_readonly_field("architect-inspection.element.kind", "Kind", format!("{:?}", element.kind)),
        ];
        return ui_inspector_groups_to_tree(&[UiInspectorFieldGroup { id: "architect-inspection.element".into(), label: "Element".into(), default_open: Some(true), fields }]);
    }
    if let Some(stakeholder) = program.stakeholders.iter().find(|row| row.header.id == id) {
        let fields = vec![
            ui_inspector_readonly_field("architect-inspection.stakeholder.id", "Id", entity_id.clone()),
            inspector_text_field(register, &entity_id, "architect-inspection.stakeholder.name", "Name", std::slice::from_ref(&stakeholder.header.name), "name"),
            inspector_text_field(register, &entity_id, "architect-inspection.stakeholder.role", "Role", std::slice::from_ref(&stakeholder.role), "role"),
            inspector_text_field(register, &entity_id, "architect-inspection.stakeholder.organization", "Organization", std::slice::from_ref(&stakeholder.organization), "organization"),
        ];
        return ui_inspector_groups_to_tree(&[UiInspectorFieldGroup { id: "architect-inspection.stakeholder".into(), label: "Stakeholder".into(), default_open: Some(true), fields }]);
    }
    if let Some(adjacency) = program.adjacencies.iter().find(|row| row.header.id == id) {
        let fields = vec![
            ui_inspector_readonly_field("architect-inspection.adjacency.id", "Id", entity_id.clone()),
            ui_inspector_readonly_field("architect-inspection.adjacency.pair", "Pair", format!("{} ↔ {}", element_label(program, &adjacency.element_a_id), element_label(program, &adjacency.element_b_id))),
            inspector_text_field(register, &entity_id, "architect-inspection.adjacency.kind", "Kind", &[adjacency_kind_label(&adjacency.kind).to_string()], "kind"),
            inspector_number_field(register, &entity_id, "architect-inspection.adjacency.weight", "Weight", &[adjacency.weight], "weight"),
            inspector_text_field(register, &entity_id, "architect-inspection.adjacency.connection", "Connection", &[format!("{:?}", adjacency.connection)], "connection"),
            inspector_text_field(register, &entity_id, "architect-inspection.adjacency.separations", "Separations", &[adjacency.separations.iter().map(|separation| format!("{separation:?}")).collect::<Vec<_>>().join(", ")], "separations"),
            inspector_text_field(register, &entity_id, "architect-inspection.adjacency.internalExternalAccess", "Internal/External Access", &[adjacency.internal_external_access.clone().unwrap_or_default()], "internalExternalAccess"),
            inspector_toggle_field(register, &entity_id, "architect-inspection.adjacency.sharedWall", "Shared Wall", &[adjacency.shared_wall], "sharedWall"),
        ];
        return ui_inspector_groups_to_tree(&[UiInspectorFieldGroup { id: "architect-inspection.adjacency".into(), label: "Adjacency".into(), default_open: Some(true), fields }]);
    }
    if let Some(requirement) = program.requirements.iter().find(|row| row.header.id == id) {
        let fields = vec![
            ui_inspector_readonly_field("architect-inspection.requirement.id", "Id", entity_id.clone()),
            inspector_text_field(register, &entity_id, "architect-inspection.requirement.name", "Name", std::slice::from_ref(&requirement.header.name), "name"),
            inspector_text_field(register, &entity_id, "architect-inspection.requirement.code", "Code", std::slice::from_ref(&requirement.code), "code"),
            inspector_text_field(register, &entity_id, "architect-inspection.requirement.statement", "Statement", std::slice::from_ref(&requirement.statement.text), "statement"),
        ];
        return ui_inspector_groups_to_tree(&[UiInspectorFieldGroup { id: "architect-inspection.requirement".into(), label: "Requirement".into(), default_open: Some(true), fields }]);
    }
    if let Some(risk) = program.risks.iter().find(|row| row.header.id == id) {
        let fields = vec![
            ui_inspector_readonly_field("architect-inspection.risk.id", "Id", entity_id.clone()),
            inspector_text_field(register, &entity_id, "architect-inspection.risk.name", "Name", std::slice::from_ref(&risk.header.name), "name"),
            inspector_text_field(register, &entity_id, "architect-inspection.risk.statement", "Statement", std::slice::from_ref(&risk.risk_statement.text), "riskStatement"),
        ];
        return ui_inspector_groups_to_tree(&[UiInspectorFieldGroup { id: "architect-inspection.risk".into(), label: "Risk".into(), default_open: Some(true), fields }]);
    }
    let generic_name = register_entities(program, register).into_iter().find(|entity| entity_id_from_json(entity).as_deref() == Some(entity_id.as_str())).map_or_else(|| entity_id.clone(), |entity| entity_name_from_json(&entity));
    ui_inspector_groups_to_tree(&[UiInspectorFieldGroup {
        id: "architect-inspection.generic".into(),
        label: format!("{register} entity"),
        default_open: Some(true),
        fields: vec![ui_inspector_readonly_field("architect-inspection.generic.id", "Id", entity_id.clone()), inspector_text_field(register, &entity_id, "architect-inspection.generic.name", "Name", &[generic_name], "name")],
    }])
}
//#endregion 🔖Panels

//#region 🔖ArchitectApp
#[derive(Default)]
struct ArchitectApp {
    runtime: ArchitectPlayRuntime,
}

impl ArchitectApp {
    fn ensure_default_register(&mut self) {
        if self.runtime.active_register.is_empty() {
            self.runtime.active_register = "elements".into();
        }
    }
}

impl DocumentApp for ArchitectApp {
    type Projection = Program;
    type Op = ProgramOp;

    fn app_id(&self) -> &str {
        ARCHITECT_APP_ID
    }

    fn document_schema(&self) -> &str {
        ARCHITECT_PROGRAM_SCHEMA
    }

    fn initial_projection(&self) -> Program {
        sample_program()
    }

    fn handle_action(&mut self, action: &str, args: Option<&Value>, doc: &DocumentView<'_, Program>, _view_state: &ViewState) -> ActionEmit<ProgramOp> {
        self.ensure_default_register();
        let program = doc.projection;
        match action {
            "setSelection" => {
                if let Some(ids) = args.and_then(|value| value.get("ids")).and_then(|value| value.as_array()) {
                    self.runtime.selected_ids = ids.iter().filter_map(|value| value.as_str().map(str::to_string)).collect();
                }
                ActionEmit::default()
            }
            "selectRegister" => {
                if let Some(register) = parse_register_id(args) {
                    self.runtime.active_register = register;
                    self.runtime.selected_ids.clear();
                }
                ActionEmit::default()
            }
            "addRegisterItem" => {
                let Some(register) = parse_register_id(args) else {
                    return ActionEmit::default();
                };
                let label = args.and_then(|value| value.get("name")).and_then(|value| value.as_str()).unwrap_or("New Item");
                if let Some(template_id) = args.and_then(|value| value.get("templateId")).and_then(|value| value.as_str()) {
                    let template_id = EntityId(template_id.into());
                    if let Some(template) = program.templates.iter().find(|row| row.header.id == template_id).cloned() {
                        let mut scratch = program.clone();
                        let ops = apply_template(&mut scratch, &template);
                        return ActionEmit::ops(ops);
                    }
                }
                let Some((op, id)) = add_register_item_op(program, &register, label) else {
                    return ActionEmit::default();
                };
                self.runtime.active_register = register;
                self.runtime.selected_ids = vec![id.to_string()];
                ActionEmit::ops(vec![op])
            }
            "removeRegisterItem" => {
                let Some(register) = parse_register_id(args) else {
                    return ActionEmit::default();
                };
                let Some(entity_id) = parse_entity_id_from_args(args, "entityId") else {
                    return ActionEmit::default();
                };
                self.runtime.selected_ids.retain(|selected| selected != &entity_id.0);
                let mut ops = Vec::new();
                if let Some(op) = remove_register_item_op(&register, entity_id.clone()) {
                    ops.push(op);
                }
                if register == "elements" {
                    for adjacency in program.adjacencies.iter().filter(|row| row.element_a_id == entity_id || row.element_b_id == entity_id) {
                        ops.push(ProgramOp::ClearAdjacency { id: adjacency.header.id.clone() });
                    }
                }
                ActionEmit::ops(ops)
            }
            "patchRegisterItem" => {
                let Some(register) = parse_register_id(args) else {
                    return ActionEmit::default();
                };
                let Some(entity_id) = parse_entity_id_from_args(args, "entityId") else {
                    return ActionEmit::default();
                };
                let Some(patch) = args.and_then(|value| value.get("patch")).cloned() else {
                    return ActionEmit::default();
                };
                if let Some(op) = patch_register_item_op(&register, entity_id, patch) {
                    ActionEmit::ops(vec![op])
                } else {
                    ActionEmit::default()
                }
            }
            "setAdjacencyField" => {
                let Some(entity_id) = parse_entity_id_from_args(args, "entityId") else {
                    return ActionEmit::default();
                };
                let field = args.and_then(|value| value.get("field")).and_then(|value| value.as_str());
                let value = args.and_then(|value| value.get("value")).cloned();
                let (Some(field), Some(value)) = (field, value) else {
                    return ActionEmit::default();
                };
                let mut patch = serde_json::Map::new();
                patch.insert(field.into(), value);
                if let Some(op) = patch_register_item_op("adjacencies", entity_id, Value::Object(patch)) {
                    ActionEmit::ops(vec![op])
                } else {
                    ActionEmit::default()
                }
            }
            "search" => {
                if let Some(query) = args.and_then(|value| value.get("query")).and_then(|value| value.as_str()) {
                    self.runtime.search_query = query.into();
                    let hits = search_program(program, &SearchQuery { keywords: query.split_whitespace().map(str::to_string).collect(), ..SearchQuery::default() }, None, Some(&mut self.runtime.search_history));
                    self.runtime.selected_ids = hits.iter().take(8).map(|hit| hit.entity_id.to_string()).collect();
                    store_runtime_json(&mut self.runtime, &hits);
                }
                ActionEmit::default()
            }
            "setAdjacencyKind" => {
                let a = parse_entity_id(args, "elementAId");
                let b = parse_entity_id(args, "elementBId");
                let (Some(a), Some(b)) = (a, b) else {
                    return ActionEmit::default();
                };
                let cycle = args.and_then(|value| value.get("cycle")).and_then(|value| value.as_bool()).unwrap_or(false);
                let explicit = parse_adjacency_kind(args);
                let existing = find_adjacency(program, &a, &b);
                let next = if cycle { next_adjacency_kind(existing.map(|row| &row.kind)) } else { explicit.or_else(|| next_adjacency_kind(existing.map(|row| &row.kind))) };
                match next {
                    Some(kind) => {
                        let adjacency = if let Some(row) = existing {
                            let mut updated = row.clone();
                            updated.kind = kind;
                            updated
                        } else {
                            new_adjacency(program, &a, &b, kind)
                        };
                        ActionEmit::ops(vec![ProgramOp::SetAdjacency { adjacency }])
                    }
                    None => {
                        if let Some(row) = existing {
                            ActionEmit::ops(vec![ProgramOp::ClearAdjacency { id: row.header.id.clone() }])
                        } else {
                            ActionEmit::default()
                        }
                    }
                }
            }
            "addElement" => {
                let name = args.and_then(|value| value.get("name")).and_then(|value| value.as_str()).unwrap_or("New Room");
                let element = default_element(name);
                let id = element.header.id.to_string();
                self.runtime.selected_ids = vec![id];
                self.runtime.active_register = "elements".into();
                ActionEmit::ops(vec![ProgramOp::Elements(CollectionOp::Add { index: program.elements.len(), item: element })])
            }
            "removeElement" => {
                let id = args.and_then(|value| value.get("elementId")).or_else(|| args.and_then(|value| value.get("id"))).and_then(|value| value.as_str());
                let Some(id) = id else {
                    return ActionEmit::default();
                };
                self.runtime.selected_ids.retain(|selected| selected != id);
                let mut ops = vec![ProgramOp::Elements(CollectionOp::Remove { id: EntityId(id.into()) })];
                for adjacency in program.adjacencies.iter().filter(|row| row.element_a_id.0 == id || row.element_b_id.0 == id) {
                    ops.push(ProgramOp::ClearAdjacency { id: adjacency.header.id.clone() });
                }
                ActionEmit::ops(ops)
            }
            "runValidation" => {
                let diagnostics = validate_program(program);
                store_runtime_json(&mut self.runtime, &diagnostics);
                ActionEmit::default()
            }
            "runAnalysis" => {
                let kind = analysis_kind_from_args(args);
                let result = run_analysis(program, kind);
                let record = analysis_record_from(program, kind, &result);
                self.runtime.last_analysis = Some(result.clone());
                store_runtime_json(&mut self.runtime, &result);
                ActionEmit::ops(vec![ProgramOp::Analyses(CollectionOp::Add { index: program.analyses.len(), item: record })])
            }
            "runReport" => {
                let kind = report_kind_from_args(args);
                let report = build_report(program, kind);
                let record = report_record_from(program, kind, &report);
                self.runtime.last_report = Some(report.clone());
                store_runtime_json(&mut self.runtime, &report);
                ActionEmit::ops(vec![ProgramOp::Reports(CollectionOp::Add { index: program.reports.len(), item: record })])
            }
            "applyTemplate" => {
                let Some(template_id) = parse_entity_id_from_args(args, "templateId") else {
                    return ActionEmit::default();
                };
                let Some(template) = program.templates.iter().find(|row| row.header.id == template_id).cloned() else {
                    return ActionEmit::default();
                };
                let mut scratch = program.clone();
                ActionEmit::ops(apply_template(&mut scratch, &template))
            }
            "exportRegistersCsv" => {
                let csv = export_registers_csv(program).unwrap_or_default();
                ActionEmit::effect(HostEffect::DownloadMediaExport { filename: format!("{}.registers.csv", program.meta.document_id), mime_type: "text/csv".into(), data: csv, encoding: None })
            }
            "importRegistersCsv" => {
                let Some(csv) = args.and_then(|value| value.get("csv")).and_then(|value| value.as_str()) else {
                    return ActionEmit::default();
                };
                let strategy = args.and_then(|value| value.get("strategy")).and_then(|value| value.as_str()).map_or(MergeStrategy::Upsert, |strategy| match strategy {
                    "replace" => MergeStrategy::Replace,
                    "skipDuplicates" => MergeStrategy::SkipDuplicates,
                    _ => MergeStrategy::Upsert,
                });
                let mut next = program.clone();
                if import_registers_csv(&mut next, csv, strategy).is_err() {
                    return ActionEmit::default();
                }
                ActionEmit::ops(vec![ProgramOp::SetProgram { program: Box::new(next) }])
            }
            "exportProgram" => {
                let json_text = export_json(program).unwrap_or_else(|error| json!({ "error": error.to_string() }).to_string());
                ActionEmit::effect(HostEffect::DownloadMediaExport { filename: format!("{}.architect.json", program.meta.document_id), mime_type: "application/json".into(), data: json_text, encoding: None })
            }
            "importProgram" => {
                let Some(json_text) = args.and_then(|value| value.get("json")).and_then(|value| value.as_str()) else {
                    return ActionEmit::default();
                };
                let Ok(next) = import_json(json_text) else {
                    return ActionEmit::default();
                };
                self.runtime.selected_ids.clear();
                ActionEmit::ops(vec![ProgramOp::SetProgram { program: Box::new(next) }])
            }
            "nodeGraphEdit" => {
                let edit_ops = args.and_then(|value| value.get("ops")).and_then(|value| value.as_array()).cloned().unwrap_or_default();
                let mut emitted = Vec::new();
                for op in edit_ops {
                    match op.get("op").and_then(Value::as_str).unwrap_or("") {
                        "connect" => {
                            let source = op.get("sourceNodeId").and_then(Value::as_str);
                            let target = op.get("targetNodeId").and_then(Value::as_str);
                            if let (Some(source), Some(target)) = (source, target) {
                                let a = EntityId(source.into());
                                let b = EntityId(target.into());
                                let kind = find_adjacency(program, &a, &b).map_or(AdjacencyKind::Preferred, |row| row.kind.clone());
                                emitted.push(ProgramOp::SetAdjacency { adjacency: new_adjacency(program, &a, &b, kind) });
                            }
                        }
                        "deleteSelection" => {
                            if let Some(ids) = op.get("nodeIds").and_then(|value| serde_json::from_value::<Vec<String>>(value.clone()).ok()) {
                                for id in ids {
                                    emitted.push(ProgramOp::Elements(CollectionOp::Remove { id: EntityId(id.clone()) }));
                                    for adjacency in program.adjacencies.iter().filter(|row| row.element_a_id.0 == id || row.element_b_id.0 == id) {
                                        emitted.push(ProgramOp::ClearAdjacency { id: adjacency.header.id.clone() });
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                }
                if emitted.is_empty() {
                    ActionEmit::default()
                } else {
                    ActionEmit::ops(emitted)
                }
            }
            "nodeGraphViewport" => {
                if let Some(viewport_json) = args.and_then(|value| value.get("viewportJson")).and_then(Value::as_str) {
                    if let Ok(camera) = serde_json::from_str::<GraphCamera>(viewport_json) {
                        self.runtime.graph_camera = camera;
                    }
                }
                ActionEmit::default()
            }
            "setAdjacencyFilter" => {
                self.runtime.adjacency_kind_filter = parse_adjacency_kind(args);
                ActionEmit::default()
            }
            _ => ActionEmit::default(),
        }
    }

    fn render(&self, body_key: &str, doc: &DocumentView<'_, Program>, _view_state: &ViewState) -> UiNode {
        let program = doc.projection;
        match body_key {
            ARCHITECT_BODY_ADJACENCY => render_adjacency_body(program, &self.runtime),
            ARCHITECT_BODY_GRAPH => render_graph_body(program, &self.runtime),
            ARCHITECT_BODY_REGISTER => render_register_body(program, &self.runtime),
            ARCHITECT_BODY_REPORT => render_report_body(&self.runtime),
            ARCHITECT_BODY_TRACE => render_trace_body(program, &self.runtime),
            ARCHITECT_BODY_DOCUMENT => build_document_tree(program, &self.runtime),
            ARCHITECT_BODY_CATALOGUE => build_catalogue_tree(),
            ARCHITECT_BODY_INSPECTION => build_inspection_tree(program, &self.runtime),
            _ => ui_text(format!("Unknown body: {body_key}")),
        }
    }

    fn app_labels(&self, view_state: &ViewState) -> AppLabelsOverlay {
        let is_de = view_state.locale.as_deref().is_some_and(|locale| locale.starts_with("de"));
        let mut overlay = AppLabelsOverlay::with_framework_panel_tabs(["framework.panel.document", "framework.panel.catalogue", "framework.panel.inspection"], is_de);
        overlay.window_kind_labels = HashMap::from([
            (ARCHITECT_WINDOW_ADJACENCY.to_string(), "Adjacency".into()),
            (ARCHITECT_WINDOW_GRAPH.to_string(), "Graph".into()),
            (ARCHITECT_WINDOW_REGISTER.to_string(), "Register".into()),
            (ARCHITECT_WINDOW_REPORT.to_string(), "Report".into()),
            (ARCHITECT_WINDOW_TRACE.to_string(), "Trace".into()),
        ]);
        overlay.mode_labels = HashMap::from([("edit".into(), "Edit".into()), ("review".into(), "Review".into()), ("report".into(), "Report".into())]);
        overlay.action_labels = architect_action_labels(is_de);
        overlay.example_labels = HashMap::from([("sample".into(), "Sample Clinic".into()), ("empty".into(), "Empty Program".into())]);
        overlay
    }
}
//#endregion 🔖ArchitectApp

//#region 🔖CommandLabels
fn architect_action_labels(is_de: bool) -> HashMap<String, String> {
    const ENTRIES: &[(&str, &str, &str)] = &[
        ("setAdjacencyKind", "Set Adjacency Kind", "Adjazenzart festlegen"),
        ("selectRegister", "Select Register", "Register wählen"),
        ("addRegisterItem", "Add Register Item", "Registereintrag hinzufügen"),
        ("removeRegisterItem", "Remove Register Item", "Registereintrag entfernen"),
        ("patchRegisterItem", "Patch Register Item", "Registereintrag patchen"),
        ("setAdjacencyField", "Set Adjacency Field", "Adjazenzfeld setzen"),
        ("applyTemplate", "Apply Template", "Vorlage anwenden"),
        ("exportRegistersCsv", "Export Registers CSV", "Register CSV exportieren"),
        ("importRegistersCsv", "Import Registers CSV", "Register CSV importieren"),
        ("addElement", "Add Element", "Element hinzufügen"),
        ("removeElement", "Remove Element", "Element entfernen"),
        ("runValidation", "Run Validation", "Validierung ausführen"),
        ("runAnalysis", "Run Analysis", "Analyse ausführen"),
        ("runReport", "Run Report", "Bericht erzeugen"),
        ("exportProgram", "Export Program", "Programm exportieren"),
        ("importProgram", "Import Program", "Programm importieren"),
        ("nodeGraphEdit", "Node Graph Edit", "Knotengraph bearbeiten"),
        ("nodeGraphViewport", "Node Graph Viewport", "Knotengraph-Ansicht"),
        ("search", "Search", "Suchen"),
        ("setSelection", "Set Selection", "Auswahl festlegen"),
    ];
    ENTRIES.iter().map(|(id, en, de)| ((*id).to_string(), (if is_de { *de } else { *en }).to_string())).collect()
}
//#endregion 🔖CommandLabels

//#region 🔖Manifest
fn create_architect_app() -> App {
    App::from_builder(
        App::builder(ARCHITECT_APP_ID, "Architect")
            .document(["semio", "architect"])
            .icon_id("architect")
            .mode("edit", "Edit")
            .mode("review", "Review")
            .mode("report", "Report")
            .default_mode_id("edit")
            .window_kind(ARCHITECT_WINDOW_ADJACENCY, "Adjacency", ARCHITECT_BODY_ADJACENCY, SurfaceKind::Canvas2d)
            .window_kind(ARCHITECT_WINDOW_GRAPH, "Graph", ARCHITECT_BODY_GRAPH, SurfaceKind::NodeGraph)
            .window_kind(ARCHITECT_WINDOW_REGISTER, "Register", ARCHITECT_BODY_REGISTER, SurfaceKind::BlockList)
            .window_kind(ARCHITECT_WINDOW_REPORT, "Report", ARCHITECT_BODY_REPORT, SurfaceKind::TextEditor)
            .window_kind(ARCHITECT_WINDOW_TRACE, "Trace", ARCHITECT_BODY_TRACE, SurfaceKind::TextEditor)
            .panel_tab(FRAMEWORK_PANEL_TAB_DOCUMENT_ID, FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL, PanelGroup::Workbench, ARCHITECT_BODY_DOCUMENT)
            .panel_tab(FRAMEWORK_PANEL_TAB_CATALOGUE_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, PanelGroup::Workbench, ARCHITECT_BODY_CATALOGUE)
            .panel_tab(FRAMEWORK_PANEL_TAB_INSPECTION_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, PanelGroup::Details, ARCHITECT_BODY_INSPECTION)
            .operation("setAdjacencyKind", "Set Adjacency Kind")
            .operation("addRegisterItem", "Add Register Item")
            .operation("removeRegisterItem", "Remove Register Item")
            .operation("patchRegisterItem", "Patch Register Item")
            .operation("importProgram", "Import Program")
            .operation("importRegistersCsv", "Import Registers CSV")
            .operation("applyTemplate", "Apply Template")
            .operation("nodeGraphEdit", "Node Graph Edit")
            .operation("nodeGraphViewport", "Node Graph Viewport")
            .view_action("selectRegister", "Select Register")
            .view_action("addElement", "Add Element")
            .view_action("removeElement", "Remove Element")
            .view_action("setAdjacencyField", "Set Adjacency Field")
            .view_action("runValidation", "Run Validation")
            .view_action("runAnalysis", "Run Analysis")
            .view_action("runReport", "Run Report")
            .view_action("search", "Search")
            .view_action("setSelection", "Set Selection")
            .shell_action("exportProgram", "Export Program")
            .shell_action("exportRegistersCsv", "Export Registers CSV")
            .action_with(ActionDefinition { in_palette: false, ..ActionDefinition::new("setAdjacencyFilter", "Set Adjacency Filter", ActionKind::View) })
            .action_args("selectRegister", vec![ActionArgDef::select("registerId", "Register", REGISTER_IDS.iter().map(|register| ActionArgOption::new(*register, *register)).collect())])
            .action_args(
                "addRegisterItem",
                vec![ActionArgDef::select("registerId", "Register", REGISTER_IDS.iter().map(|register| ActionArgOption::new(*register, *register)).collect()), ActionArgDef::text("name", "Name"), ActionArgDef::text("templateId", "Template Id")],
            )
            .action_args("removeRegisterItem", vec![ActionArgDef::select("registerId", "Register", REGISTER_IDS.iter().map(|register| ActionArgOption::new(*register, *register)).collect()), ActionArgDef::text("entityId", "Entity Id")])
            .action_args(
                "patchRegisterItem",
                vec![ActionArgDef::select("registerId", "Register", REGISTER_IDS.iter().map(|register| ActionArgOption::new(*register, *register)).collect()), ActionArgDef::text("entityId", "Entity Id"), ActionArgDef::text("patch", "Patch JSON")],
            )
            .action_args("applyTemplate", vec![ActionArgDef::text("templateId", "Template Id")])
            .action_args(
                "importRegistersCsv",
                vec![
                    ActionArgDef::text("csv", "CSV"),
                    ActionArgDef::select("strategy", "Strategy", vec![ActionArgOption::new("upsert", "Upsert"), ActionArgOption::new("replace", "Replace"), ActionArgOption::new("skipDuplicates", "Skip Duplicates")]),
                ],
            )
            .action_args(
                "setAdjacencyKind",
                vec![ActionArgDef::select(
                    "kind",
                    "Kind",
                    vec![ActionArgOption::new("required", "Required"), ActionArgOption::new("preferred", "Preferred"), ActionArgOption::new("optional", "Optional"), ActionArgOption::new("prohibited", "Prohibited")],
                )],
            )
            .action_args("runAnalysis", vec![ActionArgDef::select("analysisKind", "Analysis", analysis_kind_picker_options())])
            .action_args("runReport", vec![ActionArgDef::select("reportKind", "Report", report_kind_picker_options())])
            .action_args("search", vec![ActionArgDef::text("query", "Query")])
            .action_args("importProgram", vec![ActionArgDef::text("json", "Program JSON")])
            .default_layout(create_default_layout(
                &[ARCHITECT_WINDOW_ADJACENCY.into(), ARCHITECT_WINDOW_GRAPH.into(), ARCHITECT_WINDOW_REGISTER.into(), ARCHITECT_WINDOW_REPORT.into()],
                "row",
                Some(&[30.0, 30.0, 20.0, 20.0]),
                Some(&["Adjacency".into(), "Graph".into(), "Register".into(), "Report".into()]),
            )),
    )
    .example("sample", "Sample Clinic", serde_json::to_string(&sample_program()).expect("sample_program is a static hand-built fixture with no non-finite floats or non-UTF8 keys"))
    .example("empty", "Empty Program", serde_json::to_string(&empty_program()).expect("empty_program is a static hand-built fixture with no non-finite floats or non-UTF8 keys"))
    .program("architect", "Architect", "data")
}

fn register_architect_exports() {}

semio_framework_plugin::semio_plugin! {
    id: "architect",
    label: "Architect",
    version: "0.1.0",
    setup: register_architect_exports,
    apps: [ create_architect_app => ArchitectApp ],
}
//#endregion 🔖Manifest

//#region 🧪Tests
#[cfg(test)]
mod tests {
    use super::*;
    use semio_framework_plugin::HistoryView;

    fn with_doc_view<R>(program: &Program, run: impl FnOnce(DocumentView<'_, Program>) -> R) -> R {
        let history = HistoryView { columns: Vec::new(), can_undo: false, can_redo: false, active_alternative_id: None, current_checkpoint_id: None };
        run(DocumentView { projection: program, history: &history })
    }

    #[test]
    fn adjacency_matrix_renders_triangle_strip() {
        let app = ArchitectApp::default();
        let program = sample_program();
        with_doc_view(&program, |doc| {
            let node = app.render(ARCHITECT_BODY_ADJACENCY, &doc, &ViewState::default());
            let json = serde_json::to_string(&node).unwrap();
            assert!(json.contains('▲'));
            assert!(json.contains("Reception"));
        });
    }

    #[test]
    fn graph_body_emits_node_graph_scene() {
        let app = ArchitectApp::default();
        let program = sample_program();
        with_doc_view(&program, |doc| {
            let node = app.render(ARCHITECT_BODY_GRAPH, &doc, &ViewState::default());
            let json = serde_json::to_string(&node).unwrap();
            assert!(json.contains("node-graph"));
        });
    }

    #[test]
    fn set_adjacency_kind_cycles_required_to_preferred() {
        let mut app = ArchitectApp::default();
        let program = sample_program();
        let adjacency = program.adjacencies.first().expect("adjacency");
        with_doc_view(&program, |doc| {
            let emit = app.handle_action(
                "setAdjacencyKind",
                Some(&json!({
                    "elementAId": adjacency.element_a_id,
                    "elementBId": adjacency.element_b_id,
                    "cycle": true
                })),
                &doc,
                &ViewState::default(),
            );
            assert!(matches!(
                emit.ops.first(),
                Some(ProgramOp::SetAdjacency { adjacency: updated }) if updated.kind == AdjacencyKind::Preferred
            ));
        });
    }

    #[test]
    fn run_validation_populates_report_json() {
        let mut app = ArchitectApp::default();
        let program = sample_program();
        with_doc_view(&program, |doc| {
            app.handle_action("runValidation", None, &doc, &ViewState::default());
        });
        assert!(!app.runtime.last_report_json.is_empty());
    }

    #[test]
    fn search_finds_sample_elements() {
        let mut app = ArchitectApp::default();
        let program = sample_program();
        with_doc_view(&program, |doc| {
            app.handle_action("search", Some(&json!({ "query": "Reception" })), &doc, &ViewState::default());
        });
        assert!(!app.runtime.selected_ids.is_empty());
        assert!(!app.runtime.search_history.is_empty());
    }

    #[test]
    fn select_register_switches_active_register() {
        let mut app = ArchitectApp::default();
        let program = sample_program();
        with_doc_view(&program, |doc| {
            app.handle_action("selectRegister", Some(&json!({ "registerId": "stakeholders" })), &doc, &ViewState::default());
        });
        assert_eq!(app.runtime.active_register, "stakeholders");
        assert!(!register_entities(&program, "stakeholders").is_empty());
    }

    #[test]
    fn patch_register_item_updates_element_name() {
        let mut app = ArchitectApp::default();
        let program = sample_program();
        let element_id = program.elements[0].header.id.clone();
        with_doc_view(&program, |doc| {
            let emit = app.handle_action(
                "patchRegisterItem",
                Some(&json!({
                    "registerId": "elements",
                    "entityId": element_id,
                    "patch": { "name": "Updated Reception" }
                })),
                &doc,
                &ViewState::default(),
            );
            assert!(matches!(
                emit.ops.first(),
                Some(ProgramOp::Elements(CollectionOp::Patch { patch, .. })) if patch.name.as_deref() == Some("Updated Reception")
            ));
        });
    }

    #[test]
    fn formatted_report_renders_section_headings() {
        let mut app = ArchitectApp::default();
        let program = sample_program();
        with_doc_view(&program, |doc| {
            app.handle_action("runReport", Some(&json!({ "reportKind": "executiveSummary" })), &doc, &ViewState::default());
            let node = app.render(ARCHITECT_BODY_REPORT, &doc, &ViewState::default());
            let json = serde_json::to_string(&node).unwrap();
            assert!(json.contains("Overview"));
            assert!(json.contains("architect-report.section"));
        });
    }

    #[test]
    fn analysis_kind_picker_maps_all_variants() {
        let options = analysis_kind_picker_options();
        assert_eq!(options.len(), 20);
        for option in &options {
            let kind = analysis_kind_from_args(Some(&json!({ "analysisKind": option.value })));
            let mapped = format!("{kind:?}");
            assert!(!mapped.is_empty(), "missing mapping for {}", option.value);
        }
        assert_eq!(analysis_kind_from_args(Some(&json!({ "analysisKind": "relationshipAnalysis" }))), AnalysisKind::RelationshipAnalysis);
    }

    #[test]
    fn import_registers_csv_action_sets_program() {
        let mut app = ArchitectApp::default();
        let program = sample_program();
        let csv = export_registers_csv(&program).expect("export csv");
        with_doc_view(&program, |doc| {
            let emit = app.handle_action("importRegistersCsv", Some(&json!({ "csv": csv, "strategy": "upsert" })), &doc, &ViewState::default());
            assert!(matches!(emit.ops.first(), Some(ProgramOp::SetProgram { .. })));
        });
    }
}
//#endregion 🧪Tests
