//! 🗂️ Architect play app — the register catalog: which registers exist, how to enumerate and
//! inspect their rows, how to build the add/remove/patch operations for them, and how to coerce the
//! host's stringly action args into the artifact's typed kinds.
//!
//! App level (not artifact engine) on purpose: every function here exists to serve the app's command
//! and panel layers, several produce framework `ActionArgOption`s, and the artifact has no other
//! consumer that would benefit from owning them.

use crate::apps::architect::chrome::{element_label, entity_to_json};
use crate::artifacts::program::standards::v1::subsets::any::schema::normalize_pair;
use crate::artifacts::program::standards::v1::subsets::any::schema::inferences::AnalysisResult;
use crate::artifacts::program::standards::v1::subsets::any::schema::inferences::ProgramReport;
use crate::artifacts::program::op::ProgramMutation;
use crate::artifacts::program::registers::{
    Adjacency, AdjacencyKind, AnalysisKind, AnalysisRecord, ConnectionKind, EngagementLevel, Function, FunctionKind, InfluenceLevel, Issue, IssueSeverity, ProgramElement, ProgramElementKind, ReportKind,
    ReportRecord, Requirement, RequirementKind, Risk, RiskLevel, Stakeholder, UserCategory, UserProfile, ValidationStatus,
};
use crate::artifacts::program::schema::mutations as leaves;
use crate::artifacts::program::{EntityHeader, EntityId, ProgramSnapshot, TextField, TraceKind, TraceLink};
use semio_framework_plugin::{ActionArgOption, LocalizedLabel};
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::{json, Value};

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
        area: crate::artifacts::program::QuantitySpec::default(),
        volume: crate::artifacts::program::QuantitySpec::default(),
        height: crate::artifacts::program::QuantitySpec::default(),
        occupancy: crate::artifacts::program::QuantitySpec::default(),
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
    // 🧩️ `benchmarks` composes stdio's `table` subset (ticket UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM
    // W4 batch Db) — its rows live behind the working-scene cache, not a direct `Vec<T>` field,
    // so it can't join the generic `program.$field.iter()` macro expansion below.
    if register == "benchmarks" {
        return crate::artifacts::program::program_benchmarks(program).iter().map(entity_to_json).collect();
    }
    if register == "knowledge" {
        return crate::artifacts::program::program_knowledge(program).iter().map(entity_to_json).collect();
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
    // 🧩️ `benchmarks` composes stdio's `table` subset — see `register_entities`'s own comment.
    if crate::artifacts::program::program_benchmarks(program).iter().any(|row| row.header.id == *id) {
        return Some("benchmarks");
    }
    if crate::artifacts::program::program_knowledge(program).iter().any(|row| row.header.id == *id) {
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
        issue_priority: crate::artifacts::program::Priority::Preferred,
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
        criticality: crate::artifacts::program::Priority::Preferred,
        performance_targets: Vec::new(),
        service_level: None,
        operating_hours: None,
        staffing: crate::artifacts::program::QuantitySpec::default(),
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

pub fn default_from_json<T: DeserializeOwned>(register: &str, label: &str, extra: Value) -> Option<T> {
    let mut value = match extra {
        Value::Object(map) => Value::Object(map),
        _ => Value::Object(serde_json::Map::new()),
    };
    if let Value::Object(ref mut map) = value {
        map.insert("id".into(), json!(EntityId::new_serial(register, register)));
        map.insert("name".into(), json!(label));
    }
    serde_json::from_value(value).ok()
}

pub fn add_register_item_operation(program: &ProgramSnapshot, register: &str, label: &str) -> Option<(ProgramMutation, EntityId)> {
    macro_rules! create {
        ($variant:ident, $module:ident, $field:ident, $item:expr) => {{
            let item = $item;
            let id = item.header.id.clone();
            (ProgramMutation::$variant(leaves::$module::mutation::$variant { $field: item }), id)
        }};
    }
    Some(match register {
        "stakeholders" => create!(CreateStakeholder, create_stakeholder, stakeholder, default_stakeholder(label)),
        "users" => create!(CreateUserProfile, create_user_profile, user_profile, default_user(label)),
        "activities" => {
            let item: crate::artifacts::program::Activity = default_from_json("activities", label, json!({ "code": "ACT", "category": "general", "activityType": "general" }))?;
            create!(CreateActivity, create_activity, activity, item)
        }
        "functions" => create!(CreateFunction, create_function, function, default_function(label)),
        "elements" => create!(CreateProgramElement, create_program_element, program_element, default_element(label)),
        "risks" => create!(CreateRisk, create_risk, risk, default_risk(label)),
        "requirements" => create!(CreateRequirement, create_requirement, requirement, default_requirement(label)),
        "assumptions" => create!(CreateAssumption, create_assumption, assumption, default_from_json::<crate::artifacts::program::Assumption>("assumptions", label, json!({ "statement": { "text": "" }, "validationStatus": "pending" }),)?),
        "constraints" => {
            create!(
                CreateConstraintRecord,
                create_constraint_record,
                constraint_record,
                default_from_json::<crate::artifacts::program::ConstraintRecord>("constraints", label, json!({ "constraintType": "general", "summary": { "text": "" }, "severity": "medium", "complianceStatus": "pending" }),)?
            )
        }
        "compliance_records" => {
            create!(
                CreateComplianceRecord,
                create_compliance_record,
                compliance_record,
                default_from_json::<crate::artifacts::program::ComplianceRecord>("compliance", label, json!({ "standardRef": "", "obligation": { "text": "" }, "complianceStatus": "pending", "severity": "medium" }),)?
            )
        }
        "approvals" => create!(
            CreateApprovalRecord,
            create_approval_record,
            approval_record,
            default_from_json::<crate::artifacts::program::ApprovalRecord>(
                "approvals",
                label,
                json!({
                    "approvalType": "general",
                    "subjectId": EntityId::new_serial("subject", "approvalStatus"), "approvalStatus": "draft"
                }),
            )?
        ),
        "meetings" => create!(CreateMeetingRecord, create_meeting_record, meeting_record, default_from_json::<crate::artifacts::program::MeetingRecord>("meetings", label, json!({ "meetingType": "workshop", "quorumMet": false, "meetingStatus": "draft" }),)?),
        "analyses" => create!(CreateAnalysisRecord, create_analysis_record, analysis_record, default_from_json::<AnalysisRecord>("analysis", label, json!({ "kind": "gap", "title": label, "outputSummary": { "text": "" } }),)?),
        "reports" => create!(CreateReportRecord, create_report_record, report_record, default_from_json::<ReportRecord>("report", label, json!({ "kind": "executiveSummary", "title": label, "approvalStatus": "pending", "version": "0" }),)?),
        "issues" => create!(CreateIssue, create_issue, issue, default_issue(label)),
        "templates" => create!(CreateTemplateRecord, create_template_record, template_record, default_from_json::<crate::artifacts::program::TemplateRecord>("template", label, json!({ "templateType": "sector", "version": "1", "approvalStatus": "pending", "usageCount": 0 }),)?),
        "traces" => {
            let from = program.elements.first().map_or_else(|| EntityId::new_serial("from", "from"), |element| element.header.id.clone());
            let to = program.elements.get(1).map_or_else(|| EntityId::new_serial("to", "to"), |element| element.header.id.clone());
            let item = TraceLink::new(from, to, TraceKind::FunctionToProgramElement);
            let id = item.id.clone();
            (ProgramMutation::ConnectTrace(leaves::connect_trace::mutation::ConnectTrace { trace: item }), id)
        }
        _ => return None,
    })
}

pub fn remove_register_item_operation(register: &str, entity_id: EntityId) -> Option<ProgramMutation> {
    macro_rules! delete {
        ($variant:ident, $module:ident) => {
            ProgramMutation::$variant(leaves::$module::mutation::$variant { id: entity_id })
        };
    }
    Some(match register {
        "stakeholders" => delete!(DeleteStakeholder, delete_stakeholder),
        "users" => delete!(DeleteUserProfile, delete_user_profile),
        "activities" => delete!(DeleteActivity, delete_activity),
        "functions" => delete!(DeleteFunction, delete_function),
        "elements" => delete!(DeleteProgramElement, delete_program_element),
        "quantities" => delete!(DeleteQuantityRequirement, delete_quantity_requirement),
        "relationships" => delete!(DeleteRelationship, delete_relationship),
        "adjacencies" => ProgramMutation::DisconnectAdjacency(leaves::disconnect_adjacency::mutation::DisconnectAdjacency { id: entity_id }),
        "processes" => delete!(DeleteProcess, delete_process),
        "flows" => delete!(DeleteFlowRequirement, delete_flow_requirement),
        "access_rules" => delete!(DeleteAccessRule, delete_access_rule),
        "operations" => delete!(DeleteOperationalRequirement, delete_operational_requirement),
        "equipment" => delete!(DeleteEquipment, delete_equipment),
        "resources" => delete!(DeleteResource, delete_resource),
        "storage" => delete!(DeleteStorageRequirement, delete_storage_requirement),
        "environmental" => delete!(DeleteEnvironmentalRequirement, delete_environmental_requirement),
        "human_factors" => delete!(DeleteHumanFactorRequirement, delete_human_factor_requirement),
        "accessibility" => delete!(DeleteAccessibilityRequirement, delete_accessibility_requirement),
        "privacy" => delete!(DeletePrivacyRequirement, delete_privacy_requirement),
        "safety" => delete!(DeleteSafetyRequirement, delete_safety_requirement),
        "security" => delete!(DeleteSecurityRequirement, delete_security_requirement),
        "regulatory" => delete!(DeleteRegulatoryRequirement, delete_regulatory_requirement),
        "site_context" => delete!(DeleteSiteContext, delete_site_context),
        "organizational" => delete!(DeleteOrganizationalRequirement, delete_organizational_requirement),
        "services" => delete!(DeleteServiceRequirement, delete_service_requirement),
        "infrastructure" => delete!(DeleteInfrastructureRequirement, delete_infrastructure_requirement),
        "information" => delete!(DeleteInformationRequirement, delete_information_requirement),
        "communication" => delete!(DeleteCommunicationRequirement, delete_communication_requirement),
        "wayfinding" => delete!(DeleteWayfindingRequirement, delete_wayfinding_requirement),
        "schedules" => delete!(DeleteScheduleRequirement, delete_schedule_requirement),
        "flexibility" => delete!(DeleteFlexibilityRequirement, delete_flexibility_requirement),
        "growth" => delete!(DeleteGrowthPlan, delete_growth_plan),
        "sustainability" => delete!(DeleteSustainabilityRequirement, delete_sustainability_requirement),
        "resilience" => delete!(DeleteResilienceRequirement, delete_resilience_requirement),
        "costs" => delete!(DeleteCostRequirement, delete_cost_requirement),
        "delivery" => delete!(DeleteDeliveryConstraint, delete_delivery_constraint),
        "risks" => delete!(DeleteRisk, delete_risk),
        "conflicts" => delete!(DeleteConflict, delete_conflict),
        "requirements" => delete!(DeleteRequirement, delete_requirement),
        "priorities" => delete!(DeletePriorityRecord, delete_priority_record),
        "scenarios" => delete!(DeleteScenario, delete_scenario),
        "options" => delete!(DeleteOptionEvaluation, delete_option_evaluation),
        "decisions" => delete!(DeleteDecision, delete_decision),
        "validations" => delete!(DeleteValidationRecord, delete_validation_record),
        "performance" => delete!(DeletePerformanceCriterion, delete_performance_criterion),
        "quality" => delete!(DeleteQualityRecord, delete_quality_record),
        "documents" => delete!(DeleteDocument, delete_document),
        "assumptions" => delete!(DeleteAssumption, delete_assumption),
        "constraints" => delete!(DeleteConstraintRecord, delete_constraint_record),
        "compliance_records" => delete!(DeleteComplianceRecord, delete_compliance_record),
        "approvals" => delete!(DeleteApprovalRecord, delete_approval_record),
        "meetings" => delete!(DeleteMeetingRecord, delete_meeting_record),
        "changes" => delete!(DeleteChangeRecord, delete_change_record),
        "collaboration" => delete!(DeleteCollaborationRecord, delete_collaboration_record),
        "analyses" => delete!(DeleteAnalysisRecord, delete_analysis_record),
        "reports" => delete!(DeleteReportRecord, delete_report_record),
        "search_filters" => delete!(DeleteSearchFilter, delete_search_filter),
        "status_records" => delete!(DeleteStatusRecord, delete_status_record),
        "workshops" => delete!(DeleteWorkshop, delete_workshop),
        "surveys" => delete!(DeleteSurvey, delete_survey),
        "issues" => delete!(DeleteIssue, delete_issue),
        "audit_events" => delete!(DeleteAuditEvent, delete_audit_event),
        "templates" => delete!(DeleteTemplateRecord, delete_template_record),
        "knowledge" => delete!(DeleteKnowledgeRecord, delete_knowledge_record),
        "benchmarks" => delete!(DeleteBenchmarkRecord, delete_benchmark_record),
        "traces" => ProgramMutation::DisconnectTrace(leaves::disconnect_trace::mutation::DisconnectTrace { id: entity_id }),
        _ => return None,
    })
}

fn merge_json_patch<T: Clone + Serialize + DeserializeOwned>(existing: &T, patch: &Value) -> Option<T> {
    let mut value = serde_json::to_value(existing).ok()?;
    let (Value::Object(base), Value::Object(patch_map)) = (&mut value, patch) else { return None };
    for (key, entry) in patch_map {
        base.insert(key.clone(), entry.clone());
    }
    serde_json::from_value(value).ok()
}

pub fn patch_register_item_operation(program: &ProgramSnapshot, register: &str, entity_id: EntityId, patch: Value) -> Option<ProgramMutation> {
    Some(match register {
        "stakeholders" => {
            let existing = program.stakeholders.iter().find(|row| row.header.id == entity_id)?;
            let merged = merge_json_patch(existing, &patch)?;
            ProgramMutation::ReplaceStakeholder(leaves::replace_stakeholder::mutation::ReplaceStakeholder { stakeholder: merged })
        }
        "elements" => {
            let existing = program.elements.iter().find(|row| row.header.id == entity_id)?;
            let merged = merge_json_patch(existing, &patch)?;
            ProgramMutation::ReplaceProgramElement(leaves::replace_program_element::mutation::ReplaceProgramElement { program_element: merged })
        }
        "adjacencies" => {
            let existing = program.adjacencies.iter().find(|row| row.header.id == entity_id)?;
            let merged = merge_json_patch(existing, &patch)?;
            ProgramMutation::ConnectAdjacency(leaves::connect_adjacency::mutation::ConnectAdjacency { adjacency: merged })
        }
        "requirements" => {
            let existing = program.requirements.iter().find(|row| row.header.id == entity_id)?;
            let merged = merge_json_patch(existing, &patch)?;
            ProgramMutation::ReplaceRequirement(leaves::replace_requirement::mutation::ReplaceRequirement { requirement: merged })
        }
        "risks" => {
            let existing = program.risks.iter().find(|row| row.header.id == entity_id)?;
            let merged = merge_json_patch(existing, &patch)?;
            ProgramMutation::ReplaceRisk(leaves::replace_risk::mutation::ReplaceRisk { risk: merged })
        }
        "issues" => {
            let existing = program.issues.iter().find(|row| row.header.id == entity_id)?;
            let merged = merge_json_patch(existing, &patch)?;
            ProgramMutation::ReplaceIssue(leaves::replace_issue::mutation::ReplaceIssue { issue: merged })
        }
        "functions" => {
            let existing = program.functions.iter().find(|row| row.header.id == entity_id)?;
            let merged = merge_json_patch(existing, &patch)?;
            ProgramMutation::ReplaceFunction(leaves::replace_function::mutation::ReplaceFunction { function: merged })
        }
        "users" => {
            let existing = program.users.iter().find(|row| row.header.id == entity_id)?;
            let merged = merge_json_patch(existing, &patch)?;
            ProgramMutation::ReplaceUserProfile(leaves::replace_user_profile::mutation::ReplaceUserProfile { user_profile: merged })
        }
        _ => return None,
    })
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

pub fn analysis_kind_from_str(kind: &str) -> AnalysisKind {
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

pub fn report_kind_from_str(kind: &str) -> ReportKind {
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

