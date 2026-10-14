//! 🗂️ Architect play app — the register catalog: which registers exist, how to enumerate and
//! inspect their rows, how to build the add/remove/patch operations for them, and how to coerce the
//! host's stringly action args into the artifact's typed kinds.
//!
//! App level (not artifact engine) on purpose: every function here exists to serve the app's command
//! and panel layers, several produce framework `ActionArgOption`s, and the artifact has no other
//! consumer that would benefit from owning them.

use crate::editor::architect::chrome::{element_label, entity_to_json};
use crate::standards::v1::subsets::any::schema::mutations::ProgramMutation;
use crate::standards::v1::subsets::any::schema::registers::{RegisterKey, RegisterRow};
use crate::standards::v1::subsets::any::schema::registers::{Adjacency, AdjacencyKind, AnalysisKind, AnalysisRecord, ConnectionKind, EngagementLevel, Function, FunctionKind, InfluenceLevel, Issue, IssueSeverity, ProgramElement, ProgramElementKind, ReportKind, ReportRecord, Requirement, RequirementKind, Risk, RiskLevel, Stakeholder, UserCategory, UserProfile, ValidationStatus};
use crate::standards::v1::subsets::any::schema::mutations as leaves;
use crate::standards::v1::subsets::any::schema::inferences::AnalysisResult;
use crate::standards::v1::subsets::any::schema::inferences::ProgramReport;
use crate::standards::v1::subsets::any::schema::normalize_pair;
use crate::standards::v1::subsets::any::schema::{kernel::{EntityHeader, EntityId, TextField, TraceKind, TraceLink}, snapshot::ProgramSnapshot};
use semio_framework_value::DslValue as Value;
use semio_framework_plugin::ActionArgOption;
use semio_framework_ui_locale::LocalizedLabel;

pub const REGISTER_IDS: &[&str] = &[
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

pub fn next_adjacency_kind(current: Option<&AdjacencyKind>) -> Option<AdjacencyKind> {
    match current {
        None => Some(AdjacencyKind::Required),
        Some(AdjacencyKind::Required) => Some(AdjacencyKind::Preferred),
        Some(AdjacencyKind::Preferred) => Some(AdjacencyKind::Optional),
        Some(AdjacencyKind::Optional) => Some(AdjacencyKind::Prohibited),
        Some(AdjacencyKind::Prohibited) => None,
    }
}

pub fn find_adjacency<'a>(program: &'a ProgramSnapshot, a: &EntityId, b: &EntityId) -> Option<&'a Adjacency> {
    let (left, right) = normalize_pair(a, b);
    program.adjacencies.iter().find(|row| row.element_a_id == left && row.element_b_id == right)
}

pub fn default_element(name: impl Into<String>) -> ProgramElement {
    ProgramElement {
        header: EntityHeader::new(EntityId::new_serial("element", "element"), name),
        code: String::new(),
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
    }
}

pub fn new_adjacency(program: &ProgramSnapshot, a: &EntityId, b: &EntityId, kind: AdjacencyKind) -> Adjacency {
    let (left, right) = normalize_pair(a, b);
    Adjacency {
        header: EntityHeader::new(EntityId::new_serial("adjacency", "adjacency"), format!("{} ↔ {}", element_label(program, &left), element_label(program, &right))),
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

pub fn parse_register_id(args: Option<&Value>) -> Option<String> {
    args.and_then(|value| value.get("registerId").or_else(|| value.get("register")).and_then(|v| v.as_str()).map(str::to_string))
}

pub fn parse_entity_id_from_args(args: Option<&Value>, key: &str) -> Option<EntityId> {
    args.and_then(|value| value.get(key)).and_then(|v| v.as_str()).map(|s| EntityId(s.into()))
}

pub fn register_entities(program: &ProgramSnapshot, register: &str) -> Vec<Value> {
    if register == "benchmarks" {
        return program.benchmarks_payload.iter().map(entity_to_json).collect();
    }
    if register == "knowledge" {
        return program.knowledge_payload.iter().map(entity_to_json).collect();
    }
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
        "documents" => artifacts,
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
        "traces" => traces,
    }
}

pub fn register_len(program: &ProgramSnapshot, register: &str) -> usize {
    register_entities(program, register).len()
}

pub fn find_register_for_entity(program: &ProgramSnapshot, id: &EntityId) -> Option<&'static str> {
    if program.traces.iter().any(|row| row.id == *id) {
        return Some("traces");
    }
    if program.benchmarks_payload.iter().any(|row| row.header.id == *id) {
        return Some("benchmarks");
    }
    if program.knowledge_payload.iter().any(|row| row.header.id == *id) {
        return Some("knowledge");
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
        "documents" => artifacts,
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
    }
    None
}

pub fn default_entity_header(register: &str, label: &str) -> EntityHeader {
    let prefix = register.trim_end_matches('s').trim_end_matches("_records");
    EntityHeader::new(EntityId::new_serial(prefix, prefix), label)
}

pub fn default_stakeholder(label: &str) -> Stakeholder {
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

pub fn default_requirement(label: &str) -> Requirement {
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

pub fn default_risk(label: &str) -> Risk {
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

pub fn default_issue(label: &str) -> Issue {
    Issue {
        header: default_entity_header("issues", label),
        issue_type: String::new(),
        summary: TextField::plain(""),
        issue_description: TextField::plain(""),
        severity: IssueSeverity::Minor,
        issue_priority: crate::standards::v1::subsets::any::schema::kernel::Priority::Preferred,
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

pub fn default_function(label: &str) -> Function {
    Function {
        header: default_entity_header("functions", label),
        code: String::new(),
        kind: FunctionKind::Primary,
        purpose: TextField::plain(""),
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
        regulatory_refs: Vec::new(),
        future_changes: Vec::new(),
        owner_stakeholder_id: None,
        success_metrics: Vec::new(),
        hierarchy_parent_id: None,
        conflict_ids: Vec::new(),
    }
}

pub fn default_user(label: &str) -> UserProfile {
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

pub fn default_from_json<T: semio_framework_value::FromValue>(register: &str, label: &str, extra: Value) -> Option<T> {
    let mut fields = match extra {
        semio_framework_value::DslValue::Object(fields) => fields,
        _ => Vec::new(),
    };
    fields.push(("id".to_string(), semio_framework_value::ToValue::to_value(&EntityId::new_serial(register, register))));
    fields.push(("name".to_string(), semio_framework_value::DslValue::String(label.to_string())));
    semio_framework_value::FromValue::from_value(semio_framework_value::DslValue::Object(fields)).ok()
}

pub fn add_register_item_operation(program: &ProgramSnapshot, register: &str, label: &str) -> Option<(ProgramMutation, EntityId)> {
    macro_rules! create {
        ($item:expr) => {{
            let row = RegisterRow::from($item);
            let id = row.header().id.clone();
            (ProgramMutation::CreateRegisterRow(leaves::create_register_row::CreateRegisterRow { row, index: None }), id)
        }};
    }
    Some(match register {
        "stakeholders" => create!(default_stakeholder(label)),
        "users" => create!(default_user(label)),
        "activities" => {
            let item: crate::standards::v1::subsets::any::schema::registers::Activity = default_from_json(
                "activities",
                label,
                semio_framework_value::DslValue::object([("code".to_string(), semio_framework_value::DslValue::String("ACT".to_string())), ("category".to_string(), semio_framework_value::DslValue::String("general".to_string())), ("activityType".to_string(), semio_framework_value::DslValue::String("general".to_string()))]),
            )?;
            create!(item)
        }
        "functions" => create!(default_function(label)),
        "elements" => create!(default_element(label)),
        "risks" => create!(default_risk(label)),
        "requirements" => create!(default_requirement(label)),
        "assumptions" => create!(default_from_json::<crate::standards::v1::subsets::any::schema::registers::Assumption>(
                "assumptions",
                label,
                semio_framework_value::DslValue::object([("statement".to_string(), semio_framework_value::DslValue::object([("text".to_string(), semio_framework_value::DslValue::String(String::new()))])), ("validationStatus".to_string(), semio_framework_value::DslValue::String("pending".to_string()))]),
            )?
        ),
        "constraints" => {
            create!(default_from_json::<crate::standards::v1::subsets::any::schema::registers::ConstraintRecord>(
                    "constraints",
                    label,
                    semio_framework_value::DslValue::object([
                        ("constraintType".to_string(), semio_framework_value::DslValue::String("general".to_string())),
                        ("summary".to_string(), semio_framework_value::DslValue::object([("text".to_string(), semio_framework_value::DslValue::String(String::new()))])),
                        ("severity".to_string(), semio_framework_value::DslValue::String("medium".to_string())),
                        ("complianceStatus".to_string(), semio_framework_value::DslValue::String("pending".to_string())),
                    ]),
                )?
            )
        }
        "compliance_records" => {
            create!(default_from_json::<crate::standards::v1::subsets::any::schema::registers::ComplianceRecord>(
                    "compliance",
                    label,
                    semio_framework_value::DslValue::object([
                        ("standardRef".to_string(), semio_framework_value::DslValue::String(String::new())),
                        ("obligation".to_string(), semio_framework_value::DslValue::object([("text".to_string(), semio_framework_value::DslValue::String(String::new()))])),
                        ("complianceStatus".to_string(), semio_framework_value::DslValue::String("pending".to_string())),
                        ("severity".to_string(), semio_framework_value::DslValue::String("medium".to_string())),
                    ]),
                )?
            )
        }
        "approvals" => create!(default_from_json::<crate::standards::v1::subsets::any::schema::registers::ApprovalRecord>(
                "approvals",
                label,
                semio_framework_value::DslValue::object([
                    ("approvalType".to_string(), semio_framework_value::DslValue::String("general".to_string())),
                    ("subjectId".to_string(), semio_framework_value::ToValue::to_value(&EntityId::new_serial("subject", "approvalStatus"))),
                    ("approvalStatus".to_string(), semio_framework_value::DslValue::String("draft".to_string())),
                ]),
            )?
        ),
        "meetings" => {
            create!(default_from_json::<crate::standards::v1::subsets::any::schema::registers::MeetingRecord>(
                    "meetings",
                    label,
                    semio_framework_value::DslValue::object([("meetingType".to_string(), semio_framework_value::DslValue::String("workshop".to_string())), ("quorumMet".to_string(), semio_framework_value::DslValue::Bool(false)), ("meetingStatus".to_string(), semio_framework_value::DslValue::String("draft".to_string()))]),
                )?
            )
        }
        "analyses" => create!(default_from_json::<AnalysisRecord>(
                "analysis",
                label,
                semio_framework_value::DslValue::object([("kind".to_string(), semio_framework_value::DslValue::String("gap".to_string())), ("title".to_string(), semio_framework_value::DslValue::String(label.to_string())), ("outputSummary".to_string(), semio_framework_value::DslValue::object([("text".to_string(), semio_framework_value::DslValue::String(String::new()))]))]),
            )?
        ),
        "reports" => create!(default_from_json::<ReportRecord>(
                "report",
                label,
                semio_framework_value::DslValue::object([
                    ("kind".to_string(), semio_framework_value::DslValue::String("executiveSummary".to_string())),
                    ("title".to_string(), semio_framework_value::DslValue::String(label.to_string())),
                    ("approvalStatus".to_string(), semio_framework_value::DslValue::String("pending".to_string())),
                    ("version".to_string(), semio_framework_value::DslValue::String("0".to_string())),
                ]),
            )?
        ),
        "issues" => create!(default_issue(label)),
        "templates" => create!(default_from_json::<crate::standards::v1::subsets::any::schema::registers::TemplateRecord>(
                "template",
                label,
                semio_framework_value::DslValue::object([
                    ("templateType".to_string(), semio_framework_value::DslValue::String("sector".to_string())),
                    ("version".to_string(), semio_framework_value::DslValue::String("1".to_string())),
                    ("approvalStatus".to_string(), semio_framework_value::DslValue::String("pending".to_string())),
                    ("usageCount".to_string(), semio_framework_value::DslValue::uint(0)),
                ]),
            )?
        ),
        "traces" => {
            let from = program.elements.first().map_or_else(|| EntityId::new_serial("from", "from"), |element| element.header.id.clone());
            let to = program.elements.get(1).map_or_else(|| EntityId::new_serial("to", "to"), |element| element.header.id.clone());
            let item = TraceLink::new(from, to, TraceKind::FunctionToProgramElement);
            let id = item.id.clone();
            (ProgramMutation::ConnectTrace(leaves::connect_trace::ConnectTrace { trace: item, index: None }), id)
        }
        _ => return None,
    })
}

/// 🗂️ The register-catalog key an editor register id names: the editor's ids are the diff fields, except `documents` (`artifacts`).
pub fn register_key(register: &str) -> Option<RegisterKey> {
    match register {
        "documents" => Some(RegisterKey::Document),
        field => RegisterKey::from_diff_field(field),
    }
}

pub fn remove_register_item_operation(register: &str, entity_id: EntityId) -> Option<ProgramMutation> {
    Some(match register {
        "adjacencies" => ProgramMutation::DisconnectAdjacency(leaves::disconnect_adjacency::DisconnectAdjacency { id: entity_id }),
        "traces" => ProgramMutation::DisconnectTrace(leaves::disconnect_trace::DisconnectTrace { id: entity_id }),
        register => ProgramMutation::DeleteRegisterRow(leaves::delete_register_row::DeleteRegisterRow { register: register_key(register)?, id: entity_id }),
    })
}

fn merged(base: Value, patch: &Value) -> Option<Value> {
    let (Value::Object(mut base), Value::Object(patch_fields)) = (base, patch) else { return None };
    for (key, entry) in patch_fields {
        match base.iter_mut().find(|(existing_key, _)| existing_key == key) {
            Some((_, existing_value)) => *existing_value = entry.clone(),
            None => base.push((key.clone(), entry.clone())),
        }
    }
    Some(Value::Object(base))
}

fn merge_json_patch<T: semio_framework_value::ToValue + semio_framework_value::FromValue>(existing: &T, patch: &Value) -> Option<T> {
    semio_framework_value::FromValue::from_value(merged(semio_framework_value::ToValue::to_value(existing), patch)?).ok()
}

/// 🩹 Merges `patch` into the typed row a tagged [`RegisterRow`] carries, keeping its register tag.
fn merge_row_patch(existing: &RegisterRow, patch: &Value) -> Option<RegisterRow> {
    let Value::Object(tagged) = semio_framework_value::ToValue::to_value(existing) else { return None };
    let tagged = tagged.into_iter().map(|(key, value)| Some((key.clone(), if key == "row" { merged(value, patch)? } else { value }))).collect::<Option<Vec<_>>>()?;
    semio_framework_value::FromValue::from_value(Value::Object(tagged)).ok()
}

pub fn patch_register_item_operation(program: &ProgramSnapshot, register: &str, entity_id: &EntityId, patch: &Value) -> Option<ProgramMutation> {
    if register == "adjacencies" {
        let existing = program.adjacencies.iter().find(|row| row.header.id == *entity_id)?;
        return Some(ProgramMutation::ConnectAdjacency(leaves::connect_adjacency::ConnectAdjacency { adjacency: merge_json_patch(existing, patch)?, index: None }));
    }
    let key = register_key(register)?;
    let existing = key.row_at(program, key.position(program, entity_id)?)?;
    Some(ProgramMutation::ReplaceRegisterRow(leaves::replace_register_row::ReplaceRegisterRow { row: merge_row_patch(&existing, patch)? }))
}
pub fn analysis_record_from(program: &ProgramSnapshot, kind: AnalysisKind, result: &AnalysisResult) -> AnalysisRecord {
    AnalysisRecord {
        header: EntityHeader::new(EntityId::new_serial("analysis", "analysis"), result.title.clone()),
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

pub fn report_record_from(program: &ProgramSnapshot, kind: ReportKind, report: &ProgramReport) -> ReportRecord {
    ReportRecord {
        header: EntityHeader::new(EntityId::new_serial("report", "report"), report.title.clone()),
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

pub fn parse_entity_id(value: Option<&Value>, key: &str) -> Option<EntityId> {
    value.and_then(|args| args.get(key)).and_then(|v| v.as_str()).map(|s| EntityId(s.into()))
}

pub fn adjacency_kind_from_id(kind: &str) -> Option<AdjacencyKind> {
    match kind {
        "required" => Some(AdjacencyKind::Required),
        "preferred" => Some(AdjacencyKind::Preferred),
        "optional" => Some(AdjacencyKind::Optional),
        "prohibited" => Some(AdjacencyKind::Prohibited),
        _ => None,
    }
}

pub fn analysis_kind_from_str(kind: &str) -> Option<AnalysisKind> {
    Some(match kind {
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
        _ => return None,
    })
}

pub fn report_kind_from_str(kind: &str) -> Option<ReportKind> {
    Some(match kind {
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
        _ => return None,
    })
}

pub fn analysis_kind_picker_options() -> Vec<ActionArgOption> {
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

pub fn report_kind_picker_options() -> Vec<ActionArgOption> {
    vec![
        ("executiveSummary", "Executive Summary", "Kurzfassung"),
        ("programOverview", "ProgramSnapshot Overview", "Programmübersicht"),
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
