//! 🏛️ Architect plugin — architectural program DocumentApp bundled as a hot-swappable WASM plugin.

use protocol::CollectionOperation;
use protocol::{Operation, OperationDiff};
use semio_framework_plugin::{
    create_default_layout, ui_inspector_groups_to_tree, ui_inspector_mixed_number, ui_inspector_mixed_text, ui_inspector_mixed_toggle, ui_inspector_readonly_field, ui_stack_vertical, ui_text, ui_tree_stamp_presence, ActionArgDef, ActionArgOption,
    ActionDefinition, ActionDescriptor, ActionKind, App, BlockListScene, ConfigView, DocumentApp, DocumentView, Emit, Fault, HostEffect, Label, LocalizedLabel, NodeGraphEdgeRecord, NodeGraphNodeRecord, NodeGraphPortRecord, NodeGraphScene,
    NodeGraphViewport, PanelGroup, SurfaceKind, UiComponentSceneNode, UiFieldNode, UiInputNode, UiInspectorFieldGroup, UiNode, UiNumberStepperNode, UiPresence, UiStackNode, UiToggleNode, UiTreeItemNode, UiTreeNode, UiTreeSectionNode,
    FRAMEWORK_PANEL_TAB_CATALOGUE_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, FRAMEWORK_PANEL_TAB_DOCUMENT_ID, FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL, FRAMEWORK_PANEL_TAB_INSPECTION_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
};
use semio_s_plugin_architect_spine::{
    adjacency_matrix, apply_template, audit_trail, build_report, detect_adjacency_conflicts, empty_plugin, export_registers_csv, import_registers_csv, normalize_pair, run_analysis, sample_plugin, search_plugin, status_summary, trace_chain,
    trace_impact, undirected_edges, validate_plugin, Adjacency, AdjacencyKind, AdjacencyPatch, AnalysisKind, AnalysisRecord, AnalysisResult, ConnectionKind, EngagementLevel, EntityHeader, EntityId, Function, FunctionKind, InfluenceLevel, Issue,
    IssueSeverity, MergeStrategy, Program, ProgramElement, ProgramElementKind, ProgramElementPatch, ProgramOperation, ProgramReport, ReportKind, ReportRecord, Requirement, RequirementKind, Risk, RiskLevel, SearchQuery, Stakeholder, StakeholderPatch,
    TextField, TraceChain, TraceKind, TraceLink, UserCategory, UserProfile, ValidationStatus, ARCHITECT_PROGRAM_SCHEMA,
};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashSet;

//#region 🔖️Constants
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
//#endregion 🔖️Constants

//#region 🔖️Config
/// @emoji 🧮️ B1: `ArchitectApp`'s `DocumentApp::Config` — the pure replacement for the pre-B1
/// `RefCell<ArchitectPlayRuntime>` app-struct field. Every former ephemeral runtime field (selection,
/// active register, search, cached report/analysis JSON, adjacency filter, graph camera) now lives
/// here, written via whole-snapshot `ArchitectConfigOperation::Snapshot`s from `ArchitectApp::handle`
/// (mirrors `norm::NormConfig`'s single-shared-shape precedent for a monolithic, non-crate-split app).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslDocument)]
#[serde(rename_all = "camelCase", default)]
#[dsl(extension = "architectcfg")]
#[dsl(layout = "lines")]
pub struct ArchitectConfig {
    pub selected_ids: Vec<String>,
    pub active_register: String,
    pub search_query: String,
    /// 🔎️ `Vec<SearchQuery>` serialized as JSON — `SearchQuery` has no `dsl::DslField` binding of its
    /// own, so (like `positions_json`/`camera_json` on other migrated apps) it round-trips as text.
    pub search_history_json: String,
    /// 📋️ The currently rendered `ProgramReport` (`ARCHITECT_BODY_REPORT`), serialized as JSON.
    pub active_report_json: String,
    /// 🐛️ Generic last-action-result debug dump (search hits / validation diagnostics / analysis
    /// result / report) — the pre-B1 `last_report_json` field, renamed since it no longer overlaps
    /// with `active_report_json` above.
    pub last_result_json: String,
    /// 🧮️ The last computed `AnalysisResult`, serialized as JSON — write-only state today (no render
    /// path reads it back), kept for state fidelity with the pre-B1 runtime.
    pub last_analysis_json: String,
    pub adjacency_kind_filter: Option<AdjacencyKind>,
    pub graph_camera_x: f64,
    pub graph_camera_y: f64,
    pub graph_camera_zoom: f64,
}

impl Default for ArchitectConfig {
    fn default() -> Self {
        Self {
            selected_ids: Vec::new(),
            active_register: String::new(),
            search_query: String::new(),
            search_history_json: String::new(),
            active_report_json: String::new(),
            last_result_json: String::new(),
            last_analysis_json: String::new(),
            adjacency_kind_filter: None,
            graph_camera_x: 0.0,
            graph_camera_y: 0.0,
            graph_camera_zoom: 1.0,
        }
    }
}

impl store::ConfigRecord for ArchitectConfig {}

impl OperationDiff<ArchitectConfig> for ArchitectConfig {
    fn apply(&self, _base: &ArchitectConfig) -> ArchitectConfig {
        self.clone()
    }
    fn absorb(&mut self, other: Self) {
        *self = other;
    }
}

/// @emoji 🧮️ `ArchitectConfig`'s operation enum — a single whole-snapshot `Snapshot` variant is the
/// generic inverse every `ArchitectApp::handle` config edit uses (mirrors `norm::NormConfigOperation`
/// and `cad`'s `snapshot_of` helper; architect's config has no single hot-path field worth its own
/// granular operation variant the way `NormConfig::selected_check_index` did).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslOps)]
pub enum ArchitectConfigOperation {
    #[dsl(key = "snapshot")]
    Snapshot {
        #[dsl(block)]
        config: ArchitectConfig,
    },
}

impl Operation<ArchitectConfig> for ArchitectConfigOperation {
    type Diff = ArchitectConfig;

    fn diff(&self, _base: &ArchitectConfig) -> ArchitectConfig {
        match self {
            ArchitectConfigOperation::Snapshot { config } => config.clone(),
        }
    }

    fn backwards(&self, base: &ArchitectConfig) -> Vec<Self> {
        vec![ArchitectConfigOperation::Snapshot { config: base.clone() }]
    }
}

/// 🧮️ Reads `cfg.active_register`, defaulting to `"elements"` for a config that predates
/// `ArchitectApp::initial_config`'s default (or was constructed bare in a test).
fn active_register(cfg: &ArchitectConfig) -> &str {
    if cfg.active_register.is_empty() {
        "elements"
    } else {
        cfg.active_register.as_str()
    }
}

fn parse_search_history(cfg: &ArchitectConfig) -> Vec<SearchQuery> {
    serde_json::from_str(&cfg.search_history_json).unwrap_or_default()
}

fn parse_active_report(cfg: &ArchitectConfig) -> Option<ProgramReport> {
    if cfg.active_report_json.is_empty() {
        return None;
    }
    serde_json::from_str(&cfg.active_report_json).ok()
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

//#endregion 🔖️Config

//#region 🔖️Helpers
fn architect_action(action: &str, args: Option<Value>) -> ActionDescriptor {
    semio_framework_plugin::ActionFactory::new(ARCHITECT_APP_ID).action(action, args)
}

fn tree_item(id: impl Into<String>, label: impl Into<String>) -> UiTreeItemNode {
    UiTreeItemNode::base(id, Label::data(label.into()))
}

fn tree_item_with_action(id: impl Into<String>, label: impl Into<String>, description: Option<String>, action: ActionDescriptor) -> UiTreeItemNode {
    UiTreeItemNode { description, action: Some(action), menu: None, ..UiTreeItemNode::base(id, Label::data(label.into())) }
}

fn tree_section(id: impl Into<String>, label: Option<String>, items: Vec<UiTreeItemNode>) -> UiTreeSectionNode {
    UiTreeSectionNode { id: id.into(), label: label.map(Label::data), default_open: Some(true), presence: UiPresence::default(), items }
}

fn tree_node(mut sections: Vec<UiTreeSectionNode>, selected_ids: Option<Vec<String>>) -> UiNode {
    if let Some(ids) = selected_ids {
        ui_tree_stamp_presence(&mut sections, &ids.into_iter().collect::<HashSet<_>>(), &HashSet::new());
    }
    UiNode::Tree(UiTreeNode { sections, presence: UiPresence::default(), selected_ids: None, highlighted_ids: None, selection_change: None, drop_action: None, menu: None })
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
        area: semio_s_plugin_architect_spine::QuantitySpec::default(),
        volume: semio_s_plugin_architect_spine::QuantitySpec::default(),
        height: semio_s_plugin_architect_spine::QuantitySpec::default(),
        occupancy: semio_s_plugin_architect_spine::QuantitySpec::default(),
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
        issue_priority: semio_s_plugin_architect_spine::Priority::Preferred,
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
        criticality: semio_s_plugin_architect_spine::Priority::Preferred,
        performance_targets: Vec::new(),
        service_level: None,
        operating_hours: None,
        staffing: semio_s_plugin_architect_spine::QuantitySpec::default(),
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

fn add_register_item_operation(program: &Program, register: &str, label: &str) -> Option<(ProgramOperation, EntityId)> {
    macro_rules! add {
        ($field:ident, $operation:ident, $item:expr) => {{
            let item = $item;
            let id = item.header.id.clone();
            (ProgramOperation::$operation(CollectionOperation::Add { id: id.clone(), at: program.$field.len(), item }), id)
        }};
    }
    Some(match register {
        "elements" => {
            let item = default_element(label);
            let id = item.header.id.clone();
            (ProgramOperation::Elements(CollectionOperation::Add { id: id.clone(), at: program.elements.len(), item }), id)
        }
        "stakeholders" => add!(stakeholders, Stakeholders, default_stakeholder(label)),
        "requirements" => add!(requirements, Requirements, default_requirement(label)),
        "risks" => add!(risks, Risks, default_risk(label)),
        "issues" => add!(issues, Issues, default_issue(label)),
        "functions" => add!(functions, Functions, default_function(label)),
        "users" => add!(users, Users, default_user(label)),
        "activities" => {
            let item: semio_s_plugin_architect_spine::Activity = default_from_json("activities", label, json!({ "code": "ACT", "category": "general", "activityType": "general" }))?;
            add!(activities, Activities, item)
        }
        "assumptions" => add!(assumptions, Assumptions, default_from_json::<semio_s_plugin_architect_spine::Assumption>("assumptions", label, json!({ "statement": { "text": "" }, "validationStatus": "pending" }),)?),
        "constraints" => {
            add!(
                constraints,
                Constraints,
                default_from_json::<semio_s_plugin_architect_spine::ConstraintRecord>("constraints", label, json!({ "constraintType": "general", "summary": { "text": "" }, "severity": "medium", "complianceStatus": "pending" }),)?
            )
        }
        "compliance_records" => {
            add!(
                compliance_records,
                ComplianceRecords,
                default_from_json::<semio_s_plugin_architect_spine::ComplianceRecord>("compliance", label, json!({ "standardRef": "", "obligation": { "text": "" }, "complianceStatus": "pending", "severity": "medium" }),)?
            )
        }
        "approvals" => add!(
            approvals,
            Approvals,
            default_from_json::<semio_s_plugin_architect_spine::ApprovalRecord>(
                "approvals",
                label,
                json!({
                    "approvalType": "general",
                    "subjectId": EntityId::new_serial("subject"),
                    "approvalStatus": "draft"
                }),
            )?
        ),
        "meetings" => add!(meetings, Meetings, default_from_json::<semio_s_plugin_architect_spine::MeetingRecord>("meetings", label, json!({ "meetingType": "workshop", "quorumMet": false, "meetingStatus": "draft" }),)?),
        "analyses" => add!(analyses, Analyses, default_from_json::<AnalysisRecord>("analysis", label, json!({ "kind": "gap", "title": label, "outputSummary": { "text": "" } }),)?),
        "reports" => add!(reports, Reports, default_from_json::<ReportRecord>("report", label, json!({ "kind": "executiveSummary", "title": label, "approvalStatus": "pending", "version": "0" }),)?),
        "templates" => add!(templates, Templates, default_from_json::<semio_s_plugin_architect_spine::TemplateRecord>("template", label, json!({ "templateType": "sector", "version": "1", "approvalStatus": "pending", "usageCount": 0 }),)?),
        "traces" => {
            let from = program.elements.first().map_or_else(|| EntityId::new_serial("from"), |element| element.header.id.clone());
            let to = program.elements.get(1).map_or_else(|| EntityId::new_serial("to"), |element| element.header.id.clone());
            let item = TraceLink::new(from, to, TraceKind::FunctionToProgramElement);
            let id = item.id.clone();
            (ProgramOperation::Traces(CollectionOperation::Add { id: id.clone(), at: program.traces.len(), item }), id)
        }
        _ => return None,
    })
}

fn remove_register_item_operation(register: &str, entity_id: EntityId) -> Option<ProgramOperation> {
    macro_rules! remove {
        ($operation:ident) => {
            ProgramOperation::$operation(CollectionOperation::Remove { id: entity_id })
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

fn patch_register_item_operation(register: &str, entity_id: EntityId, patch: Value) -> Option<ProgramOperation> {
    macro_rules! patch {
        ($operation:ident, $ty:ty) => {
            ProgramOperation::$operation(CollectionOperation::Patch { id: entity_id, patch: serde_json::from_value::<$ty>(patch).ok()? })
        };
    }
    Some(match register {
        "stakeholders" => patch!(Stakeholders, StakeholderPatch),
        "elements" => patch!(Elements, ProgramElementPatch),
        "adjacencies" => patch!(Adjacencies, AdjacencyPatch),
        "requirements" => patch!(Requirements, semio_s_plugin_architect_spine::RequirementPatch),
        "risks" => patch!(Risks, semio_s_plugin_architect_spine::RiskPatch),
        "issues" => patch!(Issues, semio_s_plugin_architect_spine::IssuePatch),
        "functions" => patch!(Functions, semio_s_plugin_architect_spine::FunctionPatch),
        "users" => patch!(Users, semio_s_plugin_architect_spine::UserProfilePatch),
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
        label: Label::data(label),
        child: Box::new(UiNode::Input(UiInputNode {
            id: format!("{field_id}.input"),
            input_kind: "text".into(),
            value: mixed.value,
            placeholder: mixed.placeholder.map(Label::data),
            commit: Some("blur".into()),
            on_change: inspector_patch_action(register_id, entity_id, &json!({ key: patch_value })),
            min: None,
            max: None,
            step: None,
            accept: None,
            presence: UiPresence::default(),
            menu: None,
        })),
        description: None,
        required: None,
        error: None,
        presence: UiPresence::default(),
        menu: None,
    })
}

fn inspector_number_field(register_id: &str, entity_id: &str, field_id: &str, label: &str, values: &[f64], key: &str) -> UiNode {
    let mixed = ui_inspector_mixed_number(values);
    let patch_value = mixed.value;
    UiNode::Field(UiFieldNode {
        id: field_id.into(),
        label: Label::data(label),
        child: Box::new(UiNode::NumberStepper(UiNumberStepperNode {
            id: format!("{field_id}.stepper"),
            value: mixed.value,
            step: 0.1,
            uniform: mixed.uniform,
            on_absolute: inspector_patch_action(register_id, entity_id, &json!({ key: patch_value })),
            on_delta: inspector_patch_action(register_id, entity_id, &json!({ key: patch_value })),
            presence: UiPresence::default(),
            menu: None,
        })),
        description: None,
        required: None,
        error: None,
        presence: UiPresence::default(),
        menu: None,
    })
}

fn inspector_toggle_field(register_id: &str, entity_id: &str, field_id: &str, label: &str, values: &[bool], key: &str) -> UiNode {
    let mixed = ui_inspector_mixed_toggle(values);
    let patch_value = mixed.pressed;
    UiNode::Field(UiFieldNode {
        id: field_id.into(),
        label: Label::data(label),
        child: Box::new(UiNode::Toggle(UiToggleNode {
            id: format!("{field_id}.toggle"),
            icon_id: "check".into(),
            text: Some(Label::data(if mixed.pressed { "Yes" } else { "No" })),
            on_change: inspector_patch_action(register_id, entity_id, &json!({ key: patch_value })),
            presence: UiPresence::selected(mixed.pressed),
            menu: None,
        })),
        description: None,
        required: None,
        error: None,
        presence: UiPresence::default(),
        menu: None,
    })
}

fn empty_component_scene(surface_id: &str, component_kind: SurfaceKind) -> UiComponentSceneNode {
    UiComponentSceneNode {
        surface_id: surface_id.into(),
        controller_id: ARCHITECT_APP_ID.into(),
        component_kind,
        pane_id: None,
        binding_id: None,
        presence: UiPresence::default(),
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
        menu: None,
    }
}

fn parse_entity_id(value: Option<&Value>, key: &str) -> Option<EntityId> {
    value.and_then(|args| args.get(key)).and_then(|v| v.as_str()).map(|s| EntityId(s.into()))
}

fn adjacency_kind_from_id(kind: &str) -> Option<AdjacencyKind> {
    match kind {
        "required" => Some(AdjacencyKind::Required),
        "preferred" => Some(AdjacencyKind::Preferred),
        "optional" => Some(AdjacencyKind::Optional),
        "prohibited" => Some(AdjacencyKind::Prohibited),
        _ => None,
    }
}

fn analysis_kind_from_str(kind: &str) -> AnalysisKind {
    match kind {
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
    }
}

fn report_kind_from_str(kind: &str) -> ReportKind {
    match kind {
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
    }
}

fn analysis_kind_picker_options() -> Vec<ActionArgOption> {
    vec![
        ("gap", "Gap", "Lücke"),
        ("conflict", "Conflict", "Konflikt"),
        ("dependency", "Dependency", "Abhängigkeit"),
        ("capacity", "Capacity", "Kapazität"),
        ("demand", "Demand", "Bedarf"),
        ("utilization", "Utilization", "Auslastung"),
        ("workflow", "Workflow", "Arbeitsablauf"),
        ("risk", "Risk", "Risiko"),
        ("cost", "Cost", "Kosten"),
        ("scenario", "Scenario", "Szenario"),
        ("sensitivity", "Sensitivity", "Sensitivität"),
        ("impact", "Impact", "Auswirkung"),
        ("trend", "Trend", "Trend"),
        ("requirementComparison", "Requirement Comparison", "Anforderungsvergleich"),
        ("requirementClustering", "Requirement Clustering", "Anforderungsclusterung"),
        ("requirementFiltering", "Requirement Filtering", "Anforderungsfilterung"),
        ("requirementSorting", "Requirement Sorting", "Anforderungssortierung"),
        ("requirementScoring", "Requirement Scoring", "Anforderungsbewertung"),
        ("requirementWeighting", "Requirement Weighting", "Anforderungsgewichtung"),
        ("relationshipAnalysis", "Relationship Analysis", "Beziehungsanalyse"),
    ]
    .into_iter()
    .map(|(id, en, de)| ActionArgOption::new(id, LocalizedLabel::native(en, de)))
    .collect()
}

fn report_kind_picker_options() -> Vec<ActionArgOption> {
    vec![
        ("executiveSummary", "Executive Summary", "Kurzfassung"),
        ("programOverview", "Program Overview", "Programmübersicht"),
        ("stakeholderSummary", "Stakeholder Summary", "Stakeholder-Übersicht"),
        ("requirementsMatrix", "Requirements Matrix", "Anforderungsmatrix"),
        ("adjacencyMatrix", "Adjacency Matrix", "Adjazenzmatrix"),
        ("gapAnalysis", "Gap Analysis", "Lückenanalyse"),
        ("riskRegister", "Risk Register", "Risikoregister"),
        ("decisionLog", "Decision Log", "Entscheidungsprotokoll"),
        ("validationSummary", "Validation Summary", "Validierungsübersicht"),
        ("recommendation", "Recommendation", "Empfehlung"),
        ("userSummary", "User Summary", "Nutzerübersicht"),
        ("functionalSummary", "Functional Summary", "Funktionsübersicht"),
        ("capacitySummary", "Capacity Summary", "Kapazitätsübersicht"),
        ("workflowSummary", "Workflow Summary", "Arbeitsablaufübersicht"),
        ("complianceSummary", "Compliance Summary", "Compliance-Übersicht"),
        ("costSummary", "Cost Summary", "Kostenübersicht"),
        ("scheduleSummary", "Schedule Summary", "Terminübersicht"),
        ("changeSummary", "Change Summary", "Änderungsübersicht"),
        ("openIssueSummary", "Open Issue Summary", "Offene-Probleme-Übersicht"),
        ("prioritySummary", "Priority Summary", "Prioritätenübersicht"),
        ("scenarioSummary", "Scenario Summary", "Szenarienübersicht"),
    ]
    .into_iter()
    .map(|(id, en, de)| ActionArgOption::new(id, LocalizedLabel::native(en, de)))
    .collect()
}

fn entity_id_from_json(value: &Value) -> Option<String> {
    value.get("id").and_then(|id| id.as_str()).map(str::to_string).or_else(|| value.get("header").and_then(|header| header.get("id")).and_then(|id| id.as_str()).map(str::to_string))
}

fn entity_name_from_json(value: &Value) -> String {
    value.get("name").and_then(|name| name.as_str()).map(str::to_string).or_else(|| value.get("header").and_then(|header| header.get("name")).and_then(|name| name.as_str()).map(str::to_string)).unwrap_or_else(|| "Untitled".into())
}
//#endregion 🔖️Helpers

//#region 🔖️AdjacencyRender
/// @emoji 🔺️ Signature adjacency matrix — triangle glyph strip plus lower-triangle pair rows.
fn render_adjacency_body(program: &Program, cfg: &ArchitectConfig) -> UiNode {
    let matrix = adjacency_matrix(program);
    let n = matrix.element_ids.len();
    if n == 0 {
        return ui_text(Label::data("Add program elements to edit adjacencies."));
    }

    let mut glyph_rows = Vec::new();
    let mut pair_sections = Vec::new();

    glyph_rows.push(ui_text(Label::data(" ")));
    pair_sections.push(tree_section("architect-adjacency.headers", Some("Columns".into()), matrix.element_ids.iter().enumerate().map(|(index, id)| tree_item(format!("architect-adjacency.col.{index}"), element_label(program, id))).collect()));

    for row in 1..n {
        let row_id = &matrix.element_ids[row];
        let glyph = "▲️".repeat(row);
        glyph_rows.push(ui_text(Label::data(glyph)));

        let mut items = Vec::new();
        for col in 0..row {
            let col_id = &matrix.element_ids[col];
            let cell = &matrix.cells[row][col];
            if let Some(filter) = &cfg.adjacency_kind_filter {
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
        presence: UiPresence::default(),
        activate: None,
        drop_action: None,
        drop_overlay: None,
        children: vec![ui_stack_vertical(glyph_rows), tree_node(pair_sections, None)],
        menu: None,
    })
}
//#endregion 🔖️AdjacencyRender

//#region 🔖️GraphRender
/// 🎥️ Ephemeral node-graph camera — parsed from `nodeGraphViewport`'s JSON payload and, on render,
/// reassembled from `ArchitectConfig`'s flattened `graph_camera_{x,y,zoom}` fields.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct GraphCamera {
    x: f64,
    y: f64,
    zoom: f64,
}

fn graph_media_json(program: &Program, _camera: &GraphCamera) -> (Vec<NodeGraphNodeRecord>, Vec<NodeGraphEdgeRecord>) {
    let count = program.elements.len().max(1);
    let radius = 220.0;
    let center_x = 320.0;
    let center_y = 240.0;
    let nodes: Vec<NodeGraphNodeRecord> = program
        .elements
        .iter()
        .enumerate()
        .map(|(index, element)| {
            let angle = std::f64::consts::TAU * (index as f64) / (count as f64);
            NodeGraphNodeRecord {
                id: element.header.id.to_string(),
                label: Some(element.header.name.clone()),
                x: center_x + radius * angle.cos(),
                y: center_y + radius * angle.sin(),
                width: 108.0,
                height: 44.0,
                inputs: vec![NodeGraphPortRecord { id: "in".into(), label: None, ..Default::default() }],
                outputs: vec![NodeGraphPortRecord { id: "out".into(), label: None, ..Default::default() }],
                ..Default::default()
            }
        })
        .collect();
    let edges: Vec<NodeGraphEdgeRecord> = undirected_edges(program)
        .iter()
        .enumerate()
        .map(|(index, (source, target, weight))| NodeGraphEdgeRecord {
            id: format!("edge-{index}"),
            source_node_id: source.to_string(),
            source_port_id: "out".into(),
            target_node_id: target.to_string(),
            target_port_id: "in".into(),
            label: Some(format!("{weight:.1}")),
        })
        .collect();
    (nodes, edges)
}

fn render_graph_body(program: &Program, cfg: &ArchitectConfig) -> UiNode {
    let camera = GraphCamera { x: cfg.graph_camera_x, y: cfg.graph_camera_y, zoom: cfg.graph_camera_zoom };
    let (nodes, edges) = graph_media_json(program, &camera);
    let viewport = NodeGraphViewport { x: camera.x, y: camera.y, zoom: camera.zoom };
    let mut scene = empty_component_scene(ARCHITECT_BODY_GRAPH, SurfaceKind::NodeGraph);
    scene.node_graph = Some(NodeGraphScene { editable: Some(true), capabilities_json: Some(r#"{"directedness":"undirected"}"#.into()), selection: cfg.selected_ids.clone(), ..NodeGraphScene::base(nodes, edges, viewport) });
    UiNode::ComponentScene(scene)
}
//#endregion 🔖️GraphRender

//#region 🔖️RegisterRender
fn render_register_body(program: &Program, cfg: &ArchitectConfig) -> UiNode {
    let register = active_register(cfg);
    let entities = register_entities(program, register);
    if entities.is_empty() {
        return ui_text(Label::data(format!("No entities in register '{register}'.")));
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
    let selected_id = cfg.selected_ids.first().cloned();
    let mut scene = empty_component_scene(ARCHITECT_BODY_REGISTER, SurfaceKind::BlockList);
    scene.block_list = Some(BlockListScene { steps_json, palette_json, selected_id, dragging_id: None });
    UiNode::ComponentScene(scene)
}

//#endregion 🔖️RegisterRender

//#region 🔖️Panels
fn build_document_tree(program: &Program, cfg: &ArchitectConfig) -> UiNode {
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
                    tree_item("architect-document.meta.entities", format!("Entities tracked: {} (active register: {} / {})", summary.total_entities, active_register(cfg), register_len(program, active_register(cfg)))),
                ],
            ),
            tree_section("architect-document.registers", Some("Registers".into()), register_items),
            tree_section("architect-document.elements", Some("Elements".into()), if element_items.is_empty() { vec![tree_item("architect-document.elements.empty", "(none)")] } else { element_items }),
        ],
        Some(cfg.selected_ids.iter().map(|id| format!("architect-document.element.{id}")).collect()),
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
                    tree_item_with_action("architect-catalogue.import", "Import Program", None, architect_action("importProgramRequest", None)),
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

fn render_report_body(cfg: &ArchitectConfig) -> UiNode {
    let Some(report) = parse_active_report(cfg) else {
        return ui_text(Label::data("Run validation, analysis, or report to populate this panel."));
    };
    let report = &report;
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

fn render_trace_body(program: &Program, cfg: &ArchitectConfig) -> UiNode {
    if cfg.selected_ids.is_empty() {
        return ui_text(Label::data("Select an entity to inspect trace chains and impact."));
    }
    let root = EntityId(cfg.selected_ids[0].clone());
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

fn build_inspection_tree(program: &Program, cfg: &ArchitectConfig) -> UiNode {
    if cfg.selected_ids.is_empty() {
        return ui_stack_vertical(vec![ui_text(Label::data("Select an entity in the document or register view."))]);
    }
    let id = EntityId(cfg.selected_ids[0].clone());
    let register = find_register_for_entity(program, &id).unwrap_or("elements");
    let entity_id = id.to_string();
    if let Some(element) = program.elements.iter().find(|row| row.header.id == id) {
        let fields = vec![
            ui_inspector_readonly_field("architect-inspection.element.id", Label::data("Id"), entity_id.clone()),
            inspector_text_field(register, &entity_id, "architect-inspection.element.name", "Name", std::slice::from_ref(&element.header.name), "name"),
            inspector_text_field(register, &entity_id, "architect-inspection.element.code", "Code", std::slice::from_ref(&element.code), "code"),
            inspector_text_field(register, &entity_id, "architect-inspection.element.level", "Level", &[element.level.clone().unwrap_or_default()], "level"),
            ui_inspector_readonly_field("architect-inspection.element.kind", Label::data("Kind"), format!("{:?}", element.kind)),
        ];
        return ui_inspector_groups_to_tree(&[UiInspectorFieldGroup { id: "architect-inspection.element".into(), label: Label::data("Element"), default_open: Some(true), presence: UiPresence::default(), fields }]);
    }
    if let Some(stakeholder) = program.stakeholders.iter().find(|row| row.header.id == id) {
        let fields = vec![
            ui_inspector_readonly_field("architect-inspection.stakeholder.id", Label::data("Id"), entity_id.clone()),
            inspector_text_field(register, &entity_id, "architect-inspection.stakeholder.name", "Name", std::slice::from_ref(&stakeholder.header.name), "name"),
            inspector_text_field(register, &entity_id, "architect-inspection.stakeholder.role", "Role", std::slice::from_ref(&stakeholder.role), "role"),
            inspector_text_field(register, &entity_id, "architect-inspection.stakeholder.organization", "Organization", std::slice::from_ref(&stakeholder.organization), "organization"),
        ];
        return ui_inspector_groups_to_tree(&[UiInspectorFieldGroup { id: "architect-inspection.stakeholder".into(), label: Label::data("Stakeholder"), default_open: Some(true), presence: UiPresence::default(), fields }]);
    }
    if let Some(adjacency) = program.adjacencies.iter().find(|row| row.header.id == id) {
        let fields = vec![
            ui_inspector_readonly_field("architect-inspection.adjacency.id", Label::data("Id"), entity_id.clone()),
            ui_inspector_readonly_field("architect-inspection.adjacency.pair", Label::data("Pair"), format!("{} ↔ {}", element_label(program, &adjacency.element_a_id), element_label(program, &adjacency.element_b_id))),
            inspector_text_field(register, &entity_id, "architect-inspection.adjacency.kind", "Kind", &[adjacency_kind_label(&adjacency.kind).to_string()], "kind"),
            inspector_number_field(register, &entity_id, "architect-inspection.adjacency.weight", "Weight", &[adjacency.weight], "weight"),
            inspector_text_field(register, &entity_id, "architect-inspection.adjacency.connection", "Connection", &[format!("{:?}", adjacency.connection)], "connection"),
            inspector_text_field(register, &entity_id, "architect-inspection.adjacency.separations", "Separations", &[adjacency.separations.iter().map(|separation| format!("{separation:?}")).collect::<Vec<_>>().join(", ")], "separations"),
            inspector_text_field(register, &entity_id, "architect-inspection.adjacency.internalExternalAccess", "Internal/External Access", &[adjacency.internal_external_access.clone().unwrap_or_default()], "internalExternalAccess"),
            inspector_toggle_field(register, &entity_id, "architect-inspection.adjacency.sharedWall", "Shared Wall", &[adjacency.shared_wall], "sharedWall"),
        ];
        return ui_inspector_groups_to_tree(&[UiInspectorFieldGroup { id: "architect-inspection.adjacency".into(), label: Label::data("Adjacency"), default_open: Some(true), presence: UiPresence::default(), fields }]);
    }
    if let Some(requirement) = program.requirements.iter().find(|row| row.header.id == id) {
        let fields = vec![
            ui_inspector_readonly_field("architect-inspection.requirement.id", Label::data("Id"), entity_id.clone()),
            inspector_text_field(register, &entity_id, "architect-inspection.requirement.name", "Name", std::slice::from_ref(&requirement.header.name), "name"),
            inspector_text_field(register, &entity_id, "architect-inspection.requirement.code", "Code", std::slice::from_ref(&requirement.code), "code"),
            inspector_text_field(register, &entity_id, "architect-inspection.requirement.statement", "Statement", std::slice::from_ref(&requirement.statement.text), "statement"),
        ];
        return ui_inspector_groups_to_tree(&[UiInspectorFieldGroup { id: "architect-inspection.requirement".into(), label: Label::data("Requirement"), default_open: Some(true), presence: UiPresence::default(), fields }]);
    }
    if let Some(risk) = program.risks.iter().find(|row| row.header.id == id) {
        let fields = vec![
            ui_inspector_readonly_field("architect-inspection.risk.id", Label::data("Id"), entity_id.clone()),
            inspector_text_field(register, &entity_id, "architect-inspection.risk.name", "Name", std::slice::from_ref(&risk.header.name), "name"),
            inspector_text_field(register, &entity_id, "architect-inspection.risk.statement", "Statement", std::slice::from_ref(&risk.risk_statement.text), "riskStatement"),
        ];
        return ui_inspector_groups_to_tree(&[UiInspectorFieldGroup { id: "architect-inspection.risk".into(), label: Label::data("Risk"), default_open: Some(true), presence: UiPresence::default(), fields }]);
    }
    let generic_name = register_entities(program, register).into_iter().find(|entity| entity_id_from_json(entity).as_deref() == Some(entity_id.as_str())).map_or_else(|| entity_id.clone(), |entity| entity_name_from_json(&entity));
    ui_inspector_groups_to_tree(&[UiInspectorFieldGroup {
        id: "architect-inspection.generic".into(),
        label: Label::data(format!("{register} entity")),
        default_open: Some(true),
        presence: UiPresence::default(),
        fields: vec![ui_inspector_readonly_field("architect-inspection.generic.id", Label::data("Id"), entity_id.clone()), inspector_text_field(register, &entity_id, "architect-inspection.generic.name", "Name", &[generic_name], "name")],
    }])
}
//#endregion 🔖️Panels

//#region 🔖️Command
/// 🎯️ B1: `ArchitectApp`'s `DocumentApp::Command` — the sole typed dispatch surface for `handle`,
/// one variant per action declared on `create_architect_app`'s `AppBuilder` (replaces the deleted
/// stringly-typed `handle_action` match). JSON blob arguments (patches, CSV, DSL payloads, node-graph
/// edit lists, viewport JSON) stay `String`-typed and are parsed inside `handle` — mirrors
/// `gis2d_protocol::Gis2dCommand`'s `positions_json`/`camera_json` convention for the same reason
/// (their shapes have no `dsl::DslField` binding of their own).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslOps)]
pub enum ArchitectCommand {
    #[dsl(key = "set-selection")]
    SetSelection { ids: Vec<String> },
    #[dsl(key = "select-register")]
    SelectRegister { register_id: String },
    #[dsl(key = "add-register-item")]
    AddRegisterItem { register_id: String, name: String, template_id: Option<String> },
    #[dsl(key = "remove-register-item")]
    RemoveRegisterItem { register_id: String, entity_id: String },
    #[dsl(key = "patch-register-item")]
    PatchRegisterItem { register_id: String, entity_id: String, patch_json: String },
    #[dsl(key = "set-adjacency-field")]
    SetAdjacencyField { entity_id: String, field: String, value_json: String },
    #[dsl(key = "apply-template")]
    ApplyTemplate { template_id: String },
    #[dsl(key = "export-registers-csv")]
    ExportRegistersCsv,
    #[dsl(key = "import-registers-csv")]
    ImportRegistersCsv { csv: String, strategy: String },
    #[dsl(key = "add-element")]
    AddElement { name: String },
    #[dsl(key = "remove-element")]
    RemoveElement { element_id: String },
    #[dsl(key = "run-validation")]
    RunValidation,
    #[dsl(key = "run-analysis")]
    RunAnalysis { analysis_kind: String },
    #[dsl(key = "run-report")]
    RunReport { report_kind: String },
    #[dsl(key = "export-program")]
    ExportProgram,
    #[dsl(key = "import-program-request")]
    ImportProgramRequest,
    #[dsl(key = "import-program")]
    ImportProgram { payload: String },
    #[dsl(key = "node-graph-edit")]
    NodeGraphEdit { operations_json: String },
    #[dsl(key = "node-graph-viewport")]
    NodeGraphViewport { viewport_json: String },
    #[dsl(key = "set-adjacency-kind")]
    SetAdjacencyKind { element_a_id: String, element_b_id: String, kind: Option<String>, cycle: bool },
    #[dsl(key = "search")]
    Search { query: String },
    #[dsl(key = "set-adjacency-filter")]
    SetAdjacencyFilter { kind: Option<String> },
}
//#endregion 🔖️Command

//#region 🔖️ArchitectApp
#[derive(Default)]
struct ArchitectApp;

impl DocumentApp for ArchitectApp {
    type Projection = Program;
    type Operation = ProgramOperation;
    type Config = ArchitectConfig;
    type ConfigOperation = ArchitectConfigOperation;
    type Command = ArchitectCommand;

    fn app_id(&self) -> &str {
        ARCHITECT_APP_ID
    }

    fn document_schema(&self) -> &str {
        ARCHITECT_PROGRAM_SCHEMA
    }

    fn initial_projection(&self) -> Program {
        sample_plugin()
    }

    fn initial_config(&self) -> ArchitectConfig {
        ArchitectConfig { active_register: "elements".into(), ..ArchitectConfig::default() }
    }

    /// 🏷️ Maps each `ArchitectCommand` variant back to the action id it was declared under in
    /// `create_architect_app` — used by `VcsDocumentApp` for command-log labeling and the registry's
    /// View/Shell kind-discipline check.
    fn command_id(&self, command: &ArchitectCommand) -> &str {
        match command {
            ArchitectCommand::SetSelection { .. } => "setSelection",
            ArchitectCommand::SelectRegister { .. } => "selectRegister",
            ArchitectCommand::AddRegisterItem { .. } => "addRegisterItem",
            ArchitectCommand::RemoveRegisterItem { .. } => "removeRegisterItem",
            ArchitectCommand::PatchRegisterItem { .. } => "patchRegisterItem",
            ArchitectCommand::SetAdjacencyField { .. } => "setAdjacencyField",
            ArchitectCommand::ApplyTemplate { .. } => "applyTemplate",
            ArchitectCommand::ExportRegistersCsv => "exportRegistersCsv",
            ArchitectCommand::ImportRegistersCsv { .. } => "importRegistersCsv",
            ArchitectCommand::AddElement { .. } => "addElement",
            ArchitectCommand::RemoveElement { .. } => "removeElement",
            ArchitectCommand::RunValidation => "runValidation",
            ArchitectCommand::RunAnalysis { .. } => "runAnalysis",
            ArchitectCommand::RunReport { .. } => "runReport",
            ArchitectCommand::ExportProgram => "exportProgram",
            ArchitectCommand::ImportProgramRequest => "importProgramRequest",
            ArchitectCommand::ImportProgram { .. } => "importProgram",
            ArchitectCommand::NodeGraphEdit { .. } => "nodeGraphEdit",
            ArchitectCommand::NodeGraphViewport { .. } => "nodeGraphViewport",
            ArchitectCommand::SetAdjacencyKind { .. } => "setAdjacencyKind",
            ArchitectCommand::Search { .. } => "search",
            ArchitectCommand::SetAdjacencyFilter { .. } => "setAdjacencyFilter",
        }
    }

    /// 🎯️ Maps host action id + JSON args onto `ArchitectCommand` — React/wgpu still speak the
    /// stringly `{action,args}` wire; this is the typed-command bridge until those call sites send
    /// `OpBinary` bytes directly (mirrors `gis2d`'s `command_from_action`).
    fn command_from_action(&self, action: &str, args: Option<&Value>) -> Result<ArchitectCommand, Fault> {
        let str_field = |key: &str| args.and_then(|value| value.get(key)).and_then(Value::as_str).map(str::to_string);
        let bool_field = |key: &str| args.and_then(|value| value.get(key)).and_then(Value::as_bool);
        match action {
            "setSelection" => Ok(ArchitectCommand::SetSelection { ids: args.and_then(|value| value.get("ids")).and_then(Value::as_array).map(|ids| ids.iter().filter_map(|value| value.as_str().map(str::to_string)).collect()).unwrap_or_default() }),
            "selectRegister" => Ok(ArchitectCommand::SelectRegister { register_id: parse_register_id(args).unwrap_or_default() }),
            "addRegisterItem" => Ok(ArchitectCommand::AddRegisterItem { register_id: parse_register_id(args).unwrap_or_default(), name: str_field("name").unwrap_or_else(|| "New Item".into()), template_id: str_field("templateId") }),
            "removeRegisterItem" => Ok(ArchitectCommand::RemoveRegisterItem { register_id: parse_register_id(args).unwrap_or_default(), entity_id: parse_entity_id_from_args(args, "entityId").map(|id| id.0).unwrap_or_default() }),
            "patchRegisterItem" => Ok(ArchitectCommand::PatchRegisterItem {
                register_id: parse_register_id(args).unwrap_or_default(),
                entity_id: parse_entity_id_from_args(args, "entityId").map(|id| id.0).unwrap_or_default(),
                patch_json: args.and_then(|value| value.get("patch")).map(Value::to_string).unwrap_or_else(|| "null".into()),
            }),
            "setAdjacencyField" => Ok(ArchitectCommand::SetAdjacencyField {
                entity_id: parse_entity_id_from_args(args, "entityId").map(|id| id.0).unwrap_or_default(),
                field: str_field("field").unwrap_or_default(),
                value_json: args.and_then(|value| value.get("value")).map(Value::to_string).unwrap_or_else(|| "null".into()),
            }),
            "applyTemplate" => Ok(ArchitectCommand::ApplyTemplate { template_id: parse_entity_id_from_args(args, "templateId").map(|id| id.0).unwrap_or_default() }),
            "exportRegistersCsv" => Ok(ArchitectCommand::ExportRegistersCsv),
            "importRegistersCsv" => Ok(ArchitectCommand::ImportRegistersCsv { csv: str_field("csv").unwrap_or_default(), strategy: str_field("strategy").unwrap_or_else(|| "upsert".into()) }),
            "addElement" => Ok(ArchitectCommand::AddElement { name: str_field("name").unwrap_or_else(|| "New Room".into()) }),
            "removeElement" => Ok(ArchitectCommand::RemoveElement { element_id: str_field("elementId").or_else(|| str_field("id")).unwrap_or_default() }),
            "runValidation" => Ok(ArchitectCommand::RunValidation),
            "runAnalysis" => Ok(ArchitectCommand::RunAnalysis { analysis_kind: str_field("analysisKind").unwrap_or_else(|| "gap".into()) }),
            "runReport" => Ok(ArchitectCommand::RunReport { report_kind: str_field("reportKind").unwrap_or_else(|| "executiveSummary".into()) }),
            "exportProgram" => Ok(ArchitectCommand::ExportProgram),
            "importProgramRequest" => Ok(ArchitectCommand::ImportProgramRequest),
            "importProgram" => Ok(ArchitectCommand::ImportProgram { payload: str_field("payload").or_else(|| str_field("dsl")).unwrap_or_default() }),
            "nodeGraphEdit" => Ok(ArchitectCommand::NodeGraphEdit { operations_json: args.and_then(|value| value.get("operations")).map(Value::to_string).unwrap_or_else(|| "[]".into()) }),
            "nodeGraphViewport" => Ok(ArchitectCommand::NodeGraphViewport { viewport_json: str_field("viewportJson").unwrap_or_default() }),
            "setAdjacencyKind" => Ok(ArchitectCommand::SetAdjacencyKind {
                element_a_id: parse_entity_id(args, "elementAId").map(|id| id.0).unwrap_or_default(),
                element_b_id: parse_entity_id(args, "elementBId").map(|id| id.0).unwrap_or_default(),
                kind: str_field("kind"),
                cycle: bool_field("cycle").unwrap_or(false),
            }),
            "search" => Ok(ArchitectCommand::Search { query: str_field("query").unwrap_or_default() }),
            "setAdjacencyFilter" => Ok(ArchitectCommand::SetAdjacencyFilter { kind: str_field("kind") }),
            other => Err(format!("architect: unhandled action id {other}")),
        }
    }

    /// 🧩️ The pure heart of the app — see `DocumentApp::handle`'s docs for the general contract.
    /// Every former `RefCell<ArchitectPlayRuntime>` mutation now clones `cfg.projection`, edits the
    /// clone, and emits it as a single whole-snapshot `ArchitectConfigOperation::Snapshot` alongside
    /// any document operations (mirrors `cad`'s `snapshot_of` helper pattern).
    fn handle(&self, command: &ArchitectCommand, doc: &DocumentView<'_, Program>, cfg: &ConfigView<'_, ArchitectConfig>) -> Result<Emit<ProgramOperation, ArchitectConfigOperation>, Fault> {
        let program = doc.projection;
        let base_config = cfg.projection;
        let snapshot = |next: ArchitectConfig| vec![ArchitectConfigOperation::Snapshot { config: next }];
        match command {
            ArchitectCommand::SetSelection { ids } => {
                let mut next = base_config.clone();
                next.selected_ids = ids.clone();
                Ok(Emit::config(snapshot(next))
            }
            ArchitectCommand::SelectRegister { register_id } => {
                let mut next = base_config.clone();
                next.active_register = register_id.clone();
                next.selected_ids.clear();
                Ok(Emit::config(snapshot(next))
            }
            ArchitectCommand::AddRegisterItem { register_id, name, template_id } => {
                if let Some(template_id) = template_id {
                    let template_id = EntityId(template_id.clone());
                    if let Some(template) = program.templates.iter().find(|row| row.header.id == template_id).cloned() {
                        let mut scratch = program.clone();
                        return Ok(Emit::operations(apply_template(&mut scratch, &template));
                    }
                }
                let Some((operation, id)) = add_register_item_operation(program, register_id, name) else {
                    return Ok(Emit::default();
                };
                let mut next = base_config.clone();
                next.active_register = register_id.clone();
                next.selected_ids = vec![id.to_string()];
                Emit { document_operations: vec![operation], config_operations: snapshot(next), ..Default::default() }
            }
            ArchitectCommand::RemoveRegisterItem { register_id, entity_id } => {
                let entity_id = EntityId(entity_id.clone());
                let mut next = base_config.clone();
                next.selected_ids.retain(|selected| selected != &entity_id.0);
                let mut operations = Vec::new();
                if let Some(operation) = remove_register_item_operation(register_id, entity_id.clone()) {
                    operations.push(operation);
                }
                if register_id == "elements" {
                    for adjacency in program.adjacencies.iter().filter(|row| row.element_a_id == entity_id || row.element_b_id == entity_id) {
                        operations.push(ProgramOperation::ClearAdjacency { id: adjacency.header.id.clone() });
                    }
                }
                Emit { document_operations: operations, config_operations: snapshot(next), ..Default::default() }
            }
            ArchitectCommand::PatchRegisterItem { register_id, entity_id, patch_json } => {
                let Ok(patch) = serde_json::from_str::<Value>(patch_json) else {
                    return Ok(Emit::default();
                };
                match patch_register_item_operation(register_id, EntityId(entity_id.clone()), patch) {
                    Some(operation) => Ok(Emit::operations(vec![operation]),
                    None => Ok(Emit::default()),
                }
            }
            ArchitectCommand::SetAdjacencyField { entity_id, field, value_json } => {
                let Ok(value) = serde_json::from_str::<Value>(value_json) else {
                    return Ok(Emit::default();
                };
                let mut patch = serde_json::Map::new();
                patch.insert(field.clone(), value);
                match patch_register_item_operation("adjacencies", EntityId(entity_id.clone()), Value::Object(patch)) {
                    Some(operation) => Ok(Emit::operations(vec![operation]),
                    None => Ok(Emit::default()),
                }
            }
            ArchitectCommand::ApplyTemplate { template_id } => {
                let template_id = EntityId(template_id.clone());
                let Some(template) = program.templates.iter().find(|row| row.header.id == template_id).cloned() else {
                    return Ok(Emit::default();
                };
                let mut scratch = program.clone();
                Ok(Emit::operations(apply_template(&mut scratch, &template))
            }
            ArchitectCommand::ExportRegistersCsv => {
                let csv = export_registers_csv(program).unwrap_or_default();
                Ok(Emit::effect(HostEffect::DownloadMediaExport { filename: format!("{}.registers.csv", program.meta.document_id), mime_type: "text/csv".into(), data: csv, encoding: None })
            }
            ArchitectCommand::ImportRegistersCsv { csv, strategy } => {
                let strategy = match strategy.as_str() {
                    "replace" => MergeStrategy::Replace,
                    "skipDuplicates" => MergeStrategy::SkipDuplicates,
                    _ => MergeStrategy::Upsert,
                };
                let mut next_program = program.clone();
                if import_registers_csv(&mut next_program, csv, strategy).is_err() {
                    return Ok(Emit::default();
                }
                Ok(Emit::operations(vec![ProgramOperation::SetProgram { program: Box::new(next_program) }])
            }
            ArchitectCommand::AddElement { name } => {
                let element = default_element(name.clone());
                let id = element.header.id.to_string();
                let mut next = base_config.clone();
                next.selected_ids = vec![id];
                next.active_register = "elements".into();
                Emit { document_operations: vec![ProgramOperation::Elements(CollectionOperation::Add { id: element.header.id.clone(), at: program.elements.len(), item: element })], config_operations: snapshot(next), ..Default::default() }
            }
            ArchitectCommand::RemoveElement { element_id } => {
                let mut next = base_config.clone();
                next.selected_ids.retain(|selected| selected != element_id);
                let mut operations = vec![ProgramOperation::Elements(CollectionOperation::Remove { id: EntityId(element_id.clone()) })];
                for adjacency in program.adjacencies.iter().filter(|row| &row.element_a_id.0 == element_id || &row.element_b_id.0 == element_id) {
                    operations.push(ProgramOperation::ClearAdjacency { id: adjacency.header.id.clone() });
                }
                Emit { document_operations: operations, config_operations: snapshot(next), ..Default::default() }
            }
            ArchitectCommand::RunValidation => {
                let diagnostics = validate_plugin(program);
                let mut next = base_config.clone();
                next.last_result_json = serde_json::to_string_pretty(&diagnostics).unwrap_or_else(|_| "{}".into());
                Ok(Emit::config(snapshot(next))
            }
            ArchitectCommand::RunAnalysis { analysis_kind } => {
                let kind = analysis_kind_from_str(analysis_kind);
                let result = run_analysis(program, kind);
                let record = analysis_record_from(program, kind, &result);
                let mut next = base_config.clone();
                let result_json = serde_json::to_string_pretty(&result).unwrap_or_else(|_| "{}".into());
                next.last_analysis_json = result_json.clone();
                next.last_result_json = result_json;
                Emit { document_operations: vec![ProgramOperation::Analyses(CollectionOperation::Add { id: record.header.id.clone(), at: program.analyses.len(), item: record })], config_operations: snapshot(next), ..Default::default() }
            }
            ArchitectCommand::RunReport { report_kind } => {
                let kind = report_kind_from_str(report_kind);
                let report = build_report(program, kind);
                let record = report_record_from(program, kind, &report);
                let mut next = base_config.clone();
                next.active_report_json = serde_json::to_string(&report).unwrap_or_else(|_| "{}".into());
                next.last_result_json = serde_json::to_string_pretty(&report).unwrap_or_else(|_| "{}".into());
                Emit { document_operations: vec![ProgramOperation::Reports(CollectionOperation::Add { id: record.header.id.clone(), at: program.reports.len(), item: record })], config_operations: snapshot(next), ..Default::default() }
            }
            ArchitectCommand::ExportProgram => {
                let dsl_text = store::DocumentDsl::print_dsl(program);
                Ok(Emit::effect(HostEffect::DownloadMediaExport { filename: format!("{}.architect.dsl", program.meta.document_id), mime_type: "text/plain".into(), data: dsl_text, encoding: None })
            }
            ArchitectCommand::ImportProgramRequest => {
                Ok(Emit::effect(HostEffect::RequestFileOpen { accept: ".dsl,.architect.dsl,.spk,.ops,application/octet-stream,text/plain".into(), read_as: None, import_action: "importProgram".into(), multiple: false })
            }
            ArchitectCommand::ImportProgram { payload } => {
                let Ok(next_program) = <Program as store::DocumentDsl>::parse_dsl(payload) else {
                    return Ok(Emit::default();
                };
                let mut next = base_config.clone();
                next.selected_ids.clear();
                Emit { document_operations: vec![ProgramOperation::SetProgram { program: Box::new(next_program) }], config_operations: snapshot(next), ..Default::default() }
            }
            ArchitectCommand::NodeGraphEdit { operations_json } => {
                let edit_operations: Vec<Value> = serde_json::from_str(operations_json).unwrap_or_default();
                let mut emitted = Vec::new();
                for operation in edit_operations {
                    match operation.get("operation").and_then(Value::as_str).unwrap_or("") {
                        "connect" => {
                            let source = operation.get("sourceNodeId").and_then(Value::as_str);
                            let target = operation.get("targetNodeId").and_then(Value::as_str);
                            if let (Some(source), Some(target)) = (source, target) {
                                let a = EntityId(source.into());
                                let b = EntityId(target.into());
                                let kind = find_adjacency(program, &a, &b).map_or(AdjacencyKind::Preferred, |row| row.kind.clone());
                                emitted.push(ProgramOperation::SetAdjacency { adjacency: new_adjacency(program, &a, &b, kind) });
                            }
                        }
                        "deleteSelection" => {
                            if let Some(ids) = operation.get("nodeIds").and_then(|value| serde_json::from_value::<Vec<String>>(value.clone()).ok()) {
                                for id in ids {
                                    emitted.push(ProgramOperation::Elements(CollectionOperation::Remove { id: EntityId(id.clone()) }));
                                    for adjacency in program.adjacencies.iter().filter(|row| row.element_a_id.0 == id || row.element_b_id.0 == id) {
                                        emitted.push(ProgramOperation::ClearAdjacency { id: adjacency.header.id.clone() });
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                }
                if emitted.is_empty() {
                    Ok(Emit::default()
                } else {
                    Ok(Emit::operations(emitted)
                }
            }
            ArchitectCommand::NodeGraphViewport { viewport_json } => {
                let Ok(camera) = serde_json::from_str::<GraphCamera>(viewport_json) else {
                    return Ok(Emit::default();
                };
                let mut next = base_config.clone();
                next.graph_camera_x = camera.x;
                next.graph_camera_y = camera.y;
                next.graph_camera_zoom = camera.zoom;
                Ok(Emit::config(snapshot(next))
            }
            ArchitectCommand::SetAdjacencyKind { element_a_id, element_b_id, kind, cycle } => {
                let a = EntityId(element_a_id.clone());
                let b = EntityId(element_b_id.clone());
                let explicit = kind.as_deref().and_then(adjacency_kind_from_id);
                let existing = find_adjacency(program, &a, &b);
                let next_kind = if *cycle { next_adjacency_kind(existing.map(|row| &row.kind)) } else { explicit.or_else(|| next_adjacency_kind(existing.map(|row| &row.kind))) };
                match next_kind {
                    Some(kind) => {
                        let adjacency = if let Some(row) = existing {
                            let mut updated = row.clone();
                            updated.kind = kind;
                            updated
                        } else {
                            new_adjacency(program, &a, &b, kind)
                        };
                        Ok(Emit::operations(vec![ProgramOperation::SetAdjacency { adjacency }])
                    }
                    None => {
                        if let Some(row) = existing {
                            Ok(Emit::operations(vec![ProgramOperation::ClearAdjacency { id: row.header.id.clone() }])
                        } else {
                            Ok(Emit::default()
                        }
                    }
                }
            }
            ArchitectCommand::Search { query } => {
                let mut history = parse_search_history(base_config);
                let hits = search_plugin(program, &SearchQuery { keywords: query.split_whitespace().map(str::to_string).collect(), ..SearchQuery::default() }, None, Some(&mut history));
                let mut next = base_config.clone();
                next.search_query = query.clone();
                next.selected_ids = hits.iter().take(8).map(|hit| hit.entity_id.to_string()).collect();
                next.search_history_json = serde_json::to_string(&history).unwrap_or_else(|_| "[]".into());
                next.last_result_json = serde_json::to_string_pretty(&hits).unwrap_or_else(|_| "[]".into());
                Ok(Emit::config(snapshot(next))
            }
            ArchitectCommand::SetAdjacencyFilter { kind } => {
                let mut next = base_config.clone();
                next.adjacency_kind_filter = kind.as_deref().and_then(adjacency_kind_from_id);
                Ok(Emit::config(snapshot(next))
            }
        }
    }

    fn render(&self, body_key: &str, doc: &DocumentView<'_, Program>, cfg: &ConfigView<'_, ArchitectConfig>) -> UiNode {
        let program = doc.projection;
        let config = cfg.projection;
        match body_key {
            ARCHITECT_BODY_ADJACENCY => render_adjacency_body(program, config),
            ARCHITECT_BODY_GRAPH => render_graph_body(program, config),
            ARCHITECT_BODY_REGISTER => render_register_body(program, config),
            ARCHITECT_BODY_REPORT => render_report_body(config),
            ARCHITECT_BODY_TRACE => render_trace_body(program, config),
            ARCHITECT_BODY_DOCUMENT => build_document_tree(program, config),
            ARCHITECT_BODY_CATALOGUE => build_catalogue_tree(),
            ARCHITECT_BODY_INSPECTION => build_inspection_tree(program, config),
            _ => ui_text(Label::data(format!("Unknown body: {body_key}"))),
        }
    }
}
//#endregion 🔖️ArchitectApp

//#region 🔖️Manifest
fn create_architect_app() -> App {
    App::from_builder(
        App::builder(ARCHITECT_APP_ID, LocalizedLabel::native("Architect", "Architekt"))
            .document(["semio", "architect"])
            .icon_id("architect")
            .mode("edit", LocalizedLabel::native("Edit", "Bearbeiten"), "pencil")
            .mode("review", LocalizedLabel::native("Review", "Überprüfen"), "search")
            .mode("report", LocalizedLabel::native("Report", "Bericht"), "bar-chart-3")
            .default_mode_id("edit")
            .window_kind(ARCHITECT_WINDOW_ADJACENCY, LocalizedLabel::native("Adjacency", "Adjazenz"), ARCHITECT_BODY_ADJACENCY, SurfaceKind::Canvas2d, "grid-3x3")
            .window_kind(ARCHITECT_WINDOW_GRAPH, LocalizedLabel::native("Graph", "Graph"), ARCHITECT_BODY_GRAPH, SurfaceKind::NodeGraph, "architect-graph")
            .window_kind(ARCHITECT_WINDOW_REGISTER, LocalizedLabel::native("Register", "Register"), ARCHITECT_BODY_REGISTER, SurfaceKind::BlockList, "list")
            .window_kind(ARCHITECT_WINDOW_REPORT, LocalizedLabel::native("Report", "Bericht"), ARCHITECT_BODY_REPORT, SurfaceKind::TextEditor, "file-text")
            .window_kind(ARCHITECT_WINDOW_TRACE, LocalizedLabel::native("Trace", "Nachverfolgung"), ARCHITECT_BODY_TRACE, SurfaceKind::TextEditor, "file-code")
            .panel_tab(FRAMEWORK_PANEL_TAB_DOCUMENT_ID, LocalizedLabel::native(FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL, "Dokument"), PanelGroup::Workbench, ARCHITECT_BODY_DOCUMENT)
            .panel_tab(FRAMEWORK_PANEL_TAB_CATALOGUE_ID, LocalizedLabel::native(FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, "Katalog"), PanelGroup::Workbench, ARCHITECT_BODY_CATALOGUE)
            .panel_tab(FRAMEWORK_PANEL_TAB_INSPECTION_ID, LocalizedLabel::native(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, "Inspektion"), PanelGroup::Details, ARCHITECT_BODY_INSPECTION)
            .operation("setAdjacencyKind", LocalizedLabel::native("Set Adjacency Kind", "Adjazenzart festlegen"))
            .operation("addRegisterItem", LocalizedLabel::native("Add Register Item", "Registereintrag hinzufügen"))
            .operation("removeRegisterItem", LocalizedLabel::native("Remove Register Item", "Registereintrag entfernen"))
            .operation("patchRegisterItem", LocalizedLabel::native("Patch Register Item", "Registereintrag patchen"))
            .operation("importProgram", LocalizedLabel::native("Import Program", "Programm importieren"))
            .operation("importRegistersCsv", LocalizedLabel::native("Import Registers CSV", "Register CSV importieren"))
            .operation("applyTemplate", LocalizedLabel::native("Apply Template", "Vorlage anwenden"))
            .operation("nodeGraphEdit", LocalizedLabel::native("Node Graph Edit", "Knotengraph bearbeiten"))
            .view_action("nodeGraphViewport", LocalizedLabel::native("Node Graph Viewport", "Knotengraph-Ansicht"))
            .view_action("selectRegister", LocalizedLabel::native("Select Register", "Register wählen"))
            .view_action("addElement", LocalizedLabel::native("Add Element", "Element hinzufügen"))
            .view_action("removeElement", LocalizedLabel::native("Remove Element", "Element entfernen"))
            .view_action("setAdjacencyField", LocalizedLabel::native("Set Adjacency Field", "Adjazenzfeld setzen"))
            .view_action("runValidation", LocalizedLabel::native("Run Validation", "Validierung ausführen"))
            .view_action("runAnalysis", LocalizedLabel::native("Run Analysis", "Analyse ausführen"))
            .view_action("runReport", LocalizedLabel::native("Run Report", "Bericht erzeugen"))
            .view_action("search", LocalizedLabel::native("Search", "Suchen"))
            .view_action("setSelection", LocalizedLabel::native("Set Selection", "Auswahl festlegen"))
            .shell_action("exportProgram", LocalizedLabel::native("Export Program", "Programm exportieren"))
            .shell_action("exportRegistersCsv", LocalizedLabel::native("Export Registers CSV", "Register CSV exportieren"))
            .action_with(ActionDefinition { in_palette: false, ..ActionDefinition::new_catalog("setAdjacencyFilter", LocalizedLabel::native("Set Adjacency Filter", "Adjazenzfilter setzen"), ActionKind::View) })
            .action_args("selectRegister", vec![ActionArgDef::select("registerId", LocalizedLabel::native("Register", "Register"), REGISTER_IDS.iter().map(|register| ActionArgOption::new(*register, LocalizedLabel::data(*register))).collect())])
            .action_args(
                "addRegisterItem",
                vec![
                    ActionArgDef::select("registerId", LocalizedLabel::native("Register", "Register"), REGISTER_IDS.iter().map(|register| ActionArgOption::new(*register, LocalizedLabel::data(*register))).collect()),
                    ActionArgDef::text("name", LocalizedLabel::native("Name", "Name")),
                    ActionArgDef::text("templateId", LocalizedLabel::native("Template Id", "Vorlagen-ID")),
                ],
            )
            .action_args(
                "removeRegisterItem",
                vec![
                    ActionArgDef::select("registerId", LocalizedLabel::native("Register", "Register"), REGISTER_IDS.iter().map(|register| ActionArgOption::new(*register, LocalizedLabel::data(*register))).collect()),
                    ActionArgDef::text("entityId", LocalizedLabel::native("Entity Id", "Entitäts-ID")),
                ],
            )
            .action_args(
                "patchRegisterItem",
                vec![
                    ActionArgDef::select("registerId", LocalizedLabel::native("Register", "Register"), REGISTER_IDS.iter().map(|register| ActionArgOption::new(*register, LocalizedLabel::data(*register))).collect()),
                    ActionArgDef::text("entityId", LocalizedLabel::native("Entity Id", "Entitäts-ID")),
                    ActionArgDef::text("patch", LocalizedLabel::native("Patch JSON", "Patch-JSON")),
                ],
            )
            .action_args("applyTemplate", vec![ActionArgDef::text("templateId", LocalizedLabel::native("Template Id", "Vorlagen-ID"))])
            .action_args(
                "importRegistersCsv",
                vec![
                    ActionArgDef::text("csv", LocalizedLabel::native("CSV", "CSV")),
                    ActionArgDef::select(
                        "strategy",
                        LocalizedLabel::native("Strategy", "Strategie"),
                        vec![
                            ActionArgOption::new("upsert", LocalizedLabel::native("Upsert", "Upsert")),
                            ActionArgOption::new("replace", LocalizedLabel::native("Replace", "Ersetzen")),
                            ActionArgOption::new("skipDuplicates", LocalizedLabel::native("Skip Duplicates", "Duplikate überspringen")),
                        ],
                    ),
                ],
            )
            .action_args(
                "setAdjacencyKind",
                vec![ActionArgDef::select(
                    "kind",
                    LocalizedLabel::native("Kind", "Art"),
                    vec![
                        ActionArgOption::new("required", LocalizedLabel::native("Required", "Erforderlich")),
                        ActionArgOption::new("preferred", LocalizedLabel::native("Preferred", "Bevorzugt")),
                        ActionArgOption::new("optional", LocalizedLabel::native("Optional", "Optional")),
                        ActionArgOption::new("prohibited", LocalizedLabel::native("Prohibited", "Verboten")),
                    ],
                )],
            )
            .action_args("runAnalysis", vec![ActionArgDef::select("analysisKind", LocalizedLabel::native("Analysis", "Analyse"), analysis_kind_picker_options())])
            .action_args("runReport", vec![ActionArgDef::select("reportKind", LocalizedLabel::native("Report", "Bericht"), report_kind_picker_options())])
            .action_args("search", vec![ActionArgDef::text("query", LocalizedLabel::native("Query", "Suchanfrage"))])
            .action_args("importProgram", vec![ActionArgDef::text("payload", LocalizedLabel::native("Program DSL", "Programm-DSL"))])
            .default_layout(create_default_layout(
                &[ARCHITECT_WINDOW_ADJACENCY.into(), ARCHITECT_WINDOW_GRAPH.into(), ARCHITECT_WINDOW_REGISTER.into(), ARCHITECT_WINDOW_REPORT.into()],
                "row",
                Some(&[30.0, 30.0, 20.0, 20.0]),
                Some(&["Adjacency".into(), "Graph".into(), "Register".into(), "Report".into()]),
            )),
    )
    .example("sample", LocalizedLabel::native("Sample Clinic", "Beispielklinik"), serde_json::to_string(&sample_plugin()).expect("sample_plugin is a static hand-built fixture with no non-finite floats or non-UTF8 keys"), "cylinder")
    .example("empty", LocalizedLabel::native("Empty Program", "Leeres Programm"), serde_json::to_string(&empty_plugin()).expect("empty_plugin is a static hand-built fixture with no non-finite floats or non-UTF8 keys"), "file")
    .workflow("architect", "Architect", "data")
}

fn register_architect_exports() {
    semio_framework_plugin::plugin_runtime::register_document_codec_for_app::<ArchitectApp>(ARCHITECT_PROGRAM_SCHEMA);
}

semio_framework_plugin::semio_plugin! {
    id: "architect",
    label: "Architect",
    version: "0.1.0",
    setup: register_architect_exports,
    apps: [ create_architect_app => ArchitectApp ],
}
//#endregion 🔖️Manifest

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use semio_framework_plugin::HistoryView;

    //#region 🔖️Harness
    fn with_doc_view<R>(program: &Program, run: impl FnOnce(DocumentView<'_, Program>) -> R) -> R {
        let history = HistoryView::empty();
        run(DocumentView { projection: program, history: &history })
    }

    fn render_direct(app: &ArchitectApp, body_key: &str, doc: &DocumentView<'_, Program>, config: &ArchitectConfig) -> UiNode {
        app.render(body_key, doc, &ConfigView { projection: config })
    }

    /// 🔀️ WORKFLOWS-END-TO-END-TYPED-PORTS test-only bridge: drives a typed `ArchitectCommand`
    /// through `handle` against a bare `ArchitectApp` (unwrapped, config defaulted unless supplied) —
    /// mirrors `cad`'s `drive`/`drive_with_config` test harness.
    fn drive(app: &ArchitectApp, program: &Program, command: ArchitectCommand) -> Emit<ProgramOperation, ArchitectConfigOperation> {
        drive_with_config(app, program, command, &app.initial_config())
    }

    fn drive_with_config(app: &ArchitectApp, program: &Program, command: ArchitectCommand, config: &ArchitectConfig) -> Emit<ProgramOperation, ArchitectConfigOperation> {
        let history = HistoryView::empty();
        let doc = DocumentView { projection: program, history: &history };
        let cfg = ConfigView { projection: config };
        app.handle(&command, &doc, &cfg)
    }

    /// 🧮️ Folds an `Emit`'s `config_operations` onto a base `ArchitectConfig` — mirrors what
    /// `VcsDocumentApp`'s config store does when it dispatches them.
    fn config_after(emit: &Emit<ProgramOperation, ArchitectConfigOperation>, base: &ArchitectConfig) -> ArchitectConfig {
        let mut next = base.clone();
        for operation in &emit.config_operations {
            next = operation.diff(&next);
        }
        next
    }
    //#endregion 🔖️Harness

    #[test]
    fn adjacency_matrix_renders_triangle_strip() {
        let app = ArchitectApp;
        let program = sample_plugin();
        let config = app.initial_config();
        with_doc_view(&program, |doc| {
            let node = render_direct(&app, ARCHITECT_BODY_ADJACENCY, &doc, &config);
            let json = serde_json::to_string(&node).unwrap();
            assert!(json.contains('▲'));
            assert!(json.contains("Reception"));
        });
    }

    #[test]
    fn graph_body_emits_node_graph_scene() {
        let app = ArchitectApp;
        let program = sample_plugin();
        let config = app.initial_config();
        with_doc_view(&program, |doc| {
            let node = render_direct(&app, ARCHITECT_BODY_GRAPH, &doc, &config);
            let json = serde_json::to_string(&node).unwrap();
            assert!(json.contains("node-graph"));
        });
    }

    #[test]
    fn set_adjacency_kind_cycles_required_to_preferred() {
        let app = ArchitectApp;
        let program = sample_plugin();
        let adjacency = program.adjacencies.first().expect("adjacency");
        let emit = drive(&app, &program, ArchitectCommand::SetAdjacencyKind { element_a_id: adjacency.element_a_id.0.clone(), element_b_id: adjacency.element_b_id.0.clone(), kind: None, cycle: true });
        assert!(matches!(
            emit.document_operations.first(),
            Some(ProgramOperation::SetAdjacency { adjacency: updated }) if updated.kind == AdjacencyKind::Preferred
        ));
    }

    #[test]
    fn run_validation_populates_last_result_json() {
        let app = ArchitectApp;
        let program = sample_plugin();
        let initial = app.initial_config();
        let emit = drive_with_config(&app, &program, ArchitectCommand::RunValidation, &initial);
        let config = config_after(&emit, &initial);
        assert!(!config.last_result_json.is_empty());
    }

    #[test]
    fn search_finds_sample_elements() {
        let app = ArchitectApp;
        let program = sample_plugin();
        let initial = app.initial_config();
        let emit = drive_with_config(&app, &program, ArchitectCommand::Search { query: "Reception".into() }, &initial);
        let config = config_after(&emit, &initial);
        assert!(!config.selected_ids.is_empty());
        assert!(!config.search_history_json.is_empty());
    }

    #[test]
    fn select_register_switches_active_register() {
        let app = ArchitectApp;
        let program = sample_plugin();
        let initial = app.initial_config();
        let emit = drive_with_config(&app, &program, ArchitectCommand::SelectRegister { register_id: "stakeholders".into() }, &initial);
        let config = config_after(&emit, &initial);
        assert_eq!(config.active_register, "stakeholders");
        assert!(!register_entities(&program, "stakeholders").is_empty());
    }

    #[test]
    fn patch_register_item_updates_element_name() {
        let app = ArchitectApp;
        let program = sample_plugin();
        let element_id = program.elements[0].header.id.clone();
        let emit = drive(&app, &program, ArchitectCommand::PatchRegisterItem { register_id: "elements".into(), entity_id: element_id.0.clone(), patch_json: json!({ "name": "Updated Reception" }).to_string() });
        assert!(matches!(
            emit.document_operations.first(),
            Some(ProgramOperation::Elements(CollectionOperation::Patch { patch, .. })) if patch.name.as_deref() == Some("Updated Reception")
        ));
    }

    #[test]
    fn formatted_report_renders_section_headings() {
        let app = ArchitectApp;
        let program = sample_plugin();
        let initial = app.initial_config();
        let emit = drive_with_config(&app, &program, ArchitectCommand::RunReport { report_kind: "executiveSummary".into() }, &initial);
        let config = config_after(&emit, &initial);
        with_doc_view(&program, |doc| {
            let node = render_direct(&app, ARCHITECT_BODY_REPORT, &doc, &config);
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
            let kind = analysis_kind_from_str(&option.value);
            let mapped = format!("{kind:?}");
            assert!(!mapped.is_empty(), "missing mapping for {}", option.value);
        }
        assert_eq!(analysis_kind_from_str("relationshipAnalysis"), AnalysisKind::RelationshipAnalysis);
    }

    #[test]
    fn import_registers_csv_action_sets_plugin() {
        let app = ArchitectApp;
        let program = sample_plugin();
        let csv = export_registers_csv(&program).expect("export csv");
        let emit = drive(&app, &program, ArchitectCommand::ImportRegistersCsv { csv, strategy: "upsert".into() });
        assert!(matches!(emit.document_operations.first(), Some(ProgramOperation::SetProgram { .. })));
    }

    /// 🎯️ `command_from_action` is the shell-facing bridge — spot-check a representative sample of
    /// action ids round-trip into the expected typed `ArchitectCommand` variant.
    #[test]
    fn command_from_action_bridges_declared_actions() {
        let app = ArchitectApp;
        assert!(matches!(app.command_from_action("runValidation", None), Ok(ArchitectCommand::RunValidation)));
        assert!(matches!(app.command_from_action("search", Some(&json!({ "query": "hall" }))), Ok(ArchitectCommand::Search { query }) if query == "hall"));
        assert!(matches!(
            app.command_from_action("selectRegister", Some(&json!({ "registerId": "risks" }))),
            Ok(ArchitectCommand::SelectRegister { register_id }) if register_id == "risks"
        ));
        assert!(app.command_from_action("notARealAction", None).is_err());
    }
}
//#endregion 🧪️Tests
