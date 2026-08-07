//! 🔁️ Architect program artifact — the typed document operation surface: `ProgramOperation`,
//! its apply/invert kernel and its `OpText`/`OpBinary` wire codecs (constitutional: op).
//!
//! `CollectionOperation` per register plus meta, adjacency, and bulk set. The `ProgramDiff` carrier
//! these operations materialize lives next door in `🔺️diff/🦀️component.rs`.


//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar


use crate::artifacts::program::diff::ProgramDiff;
use crate::artifacts::program::engine::adjacency::{clear_adjacency, set_adjacency};
use crate::artifacts::program::kernel::{EntityId, TraceLink};
use crate::artifacts::program::registers::*;
use crate::artifacts::program::Program;
use protocol::{apply_collection_operation, invert_collection_operation, CollectionOperation, Operation, Patchable};
use serde::{Deserialize, Serialize};

// #region 🔖️ProgramOperation
/// @emoji 🧩️ Typed program document operation for VCS replay and undo.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "operation", rename_all = "camelCase")]
#[allow(
    clippy::large_enum_variant,
    reason = "~65 variants each wrap CollectionOperation<EntityId, T, TPatch> for a different program register T (Stakeholder..Benchmark); sizes inherently vary with T and boxing every payload is a much larger, separately-scoped restructuring (all apply_collection_operation/invert_collection_operation call sites + external construction sites) — SetProgram (the one outsized, genuinely-fixable single-field outlier) is already boxed"
)]
pub enum ProgramOperation {
    Stakeholders(CollectionOperation<EntityId, Stakeholder, StakeholderPatch>),
    Users(CollectionOperation<EntityId, UserProfile, UserProfilePatch>),
    Activities(CollectionOperation<EntityId, Activity, ActivityPatch>),
    Functions(CollectionOperation<EntityId, Function, FunctionPatch>),
    Elements(CollectionOperation<EntityId, ProgramElement, ProgramElementPatch>),
    Quantities(CollectionOperation<EntityId, QuantityRequirement, QuantityRequirementPatch>),
    Relationships(CollectionOperation<EntityId, Relationship, RelationshipPatch>),
    Adjacencies(CollectionOperation<EntityId, Adjacency, AdjacencyPatch>),
    Processes(CollectionOperation<EntityId, Process, ProcessPatch>),
    Flows(CollectionOperation<EntityId, FlowRequirement, FlowRequirementPatch>),
    AccessRules(CollectionOperation<EntityId, AccessRule, AccessRulePatch>),
    Operations(CollectionOperation<EntityId, OperationalRequirement, OperationalRequirementPatch>),
    Equipment(CollectionOperation<EntityId, Equipment, EquipmentPatch>),
    Resources(CollectionOperation<EntityId, Resource, ResourcePatch>),
    Storage(CollectionOperation<EntityId, StorageRequirement, StorageRequirementPatch>),
    Environmental(CollectionOperation<EntityId, EnvironmentalRequirement, EnvironmentalRequirementPatch>),
    HumanFactors(CollectionOperation<EntityId, HumanFactorRequirement, HumanFactorRequirementPatch>),
    Accessibility(CollectionOperation<EntityId, AccessibilityRequirement, AccessibilityRequirementPatch>),
    Privacy(CollectionOperation<EntityId, PrivacyRequirement, PrivacyRequirementPatch>),
    Safety(CollectionOperation<EntityId, SafetyRequirement, SafetyRequirementPatch>),
    Security(CollectionOperation<EntityId, SecurityRequirement, SecurityRequirementPatch>),
    Regulatory(CollectionOperation<EntityId, RegulatoryRequirement, RegulatoryRequirementPatch>),
    SiteContext(CollectionOperation<EntityId, SiteContext, SiteContextPatch>),
    Organizational(CollectionOperation<EntityId, OrganizationalRequirement, OrganizationalRequirementPatch>),
    Services(CollectionOperation<EntityId, ServiceRequirement, ServiceRequirementPatch>),
    Infrastructure(CollectionOperation<EntityId, InfrastructureRequirement, InfrastructureRequirementPatch>),
    Information(CollectionOperation<EntityId, InformationRequirement, InformationRequirementPatch>),
    Communication(CollectionOperation<EntityId, CommunicationRequirement, CommunicationRequirementPatch>),
    Wayfinding(CollectionOperation<EntityId, WayfindingRequirement, WayfindingRequirementPatch>),
    Schedules(CollectionOperation<EntityId, ScheduleRequirement, ScheduleRequirementPatch>),
    Flexibility(CollectionOperation<EntityId, FlexibilityRequirement, FlexibilityRequirementPatch>),
    Growth(CollectionOperation<EntityId, GrowthPlan, GrowthPlanPatch>),
    Sustainability(CollectionOperation<EntityId, SustainabilityRequirement, SustainabilityRequirementPatch>),
    Resilience(CollectionOperation<EntityId, ResilienceRequirement, ResilienceRequirementPatch>),
    Costs(CollectionOperation<EntityId, CostRequirement, CostRequirementPatch>),
    Delivery(CollectionOperation<EntityId, DeliveryConstraint, DeliveryConstraintPatch>),
    Risks(CollectionOperation<EntityId, Risk, RiskPatch>),
    Conflicts(CollectionOperation<EntityId, Conflict, ConflictPatch>),
    Requirements(CollectionOperation<EntityId, Requirement, RequirementPatch>),
    Priorities(CollectionOperation<EntityId, PriorityRecord, PriorityRecordPatch>),
    Scenarios(CollectionOperation<EntityId, Scenario, ScenarioPatch>),
    Options(CollectionOperation<EntityId, OptionEvaluation, OptionEvaluationPatch>),
    Decisions(CollectionOperation<EntityId, Decision, DecisionPatch>),
    Validations(CollectionOperation<EntityId, ValidationRecord, ValidationRecordPatch>),
    Performance(CollectionOperation<EntityId, PerformanceCriterion, PerformanceCriterionPatch>),
    Quality(CollectionOperation<EntityId, QualityRecord, QualityRecordPatch>),
    Documents(CollectionOperation<EntityId, DocumentRecord, DocumentRecordPatch>),
    Changes(CollectionOperation<EntityId, ChangeRecord, ChangeRecordPatch>),
    Collaboration(CollectionOperation<EntityId, CollaborationRecord, CollaborationRecordPatch>),
    Analyses(CollectionOperation<EntityId, AnalysisRecord, AnalysisRecordPatch>),
    Reports(CollectionOperation<EntityId, ReportRecord, ReportRecordPatch>),
    SearchFilters(CollectionOperation<EntityId, SearchFilter, SearchFilterPatch>),
    StatusRecords(CollectionOperation<EntityId, StatusRecord, StatusRecordPatch>),
    Workshops(CollectionOperation<EntityId, Workshop, WorkshopPatch>),
    Surveys(CollectionOperation<EntityId, Survey, SurveyPatch>),
    Issues(CollectionOperation<EntityId, Issue, IssuePatch>),
    AuditEvents(CollectionOperation<EntityId, AuditEvent, AuditEventPatch>),
    Templates(CollectionOperation<EntityId, TemplateRecord, TemplateRecordPatch>),
    Knowledge(CollectionOperation<EntityId, KnowledgeRecord, KnowledgeRecordPatch>),
    Benchmarks(CollectionOperation<EntityId, BenchmarkRecord, BenchmarkRecordPatch>),
    Assumptions(CollectionOperation<EntityId, Assumption, AssumptionPatch>),
    Constraints(CollectionOperation<EntityId, ConstraintRecord, ConstraintRecordPatch>),
    ComplianceRecords(CollectionOperation<EntityId, ComplianceRecord, ComplianceRecordPatch>),
    Approvals(CollectionOperation<EntityId, ApprovalRecord, ApprovalRecordPatch>),
    Meetings(CollectionOperation<EntityId, MeetingRecord, MeetingRecordPatch>),
    Traces(CollectionOperation<EntityId, TraceLink, TraceLinkPatch>),
    UpdateMeta { patch: ProgramMetaPatch },
    UpdateProject { patch: ProjectDefinitionPatch },
    UpdateGovernance { patch: GovernancePatch },
    SetAdjacency { adjacency: Adjacency },
    ClearAdjacency { id: EntityId },
    SetProgram { program: Box<Program> },
}

//#region 🔖️OpText
/// @emoji 📝️ Hand-written (not `#[derive(dsl::DslOps)]`): every collection-register variant here
/// wraps `vcs::CollectionOperation<EntityId, T, TPatch>` — a type foreign to this crate (defined
/// in `vcs`), so the orphan rule blocks `impl dsl::DslField for CollectionOperation<..>` from
/// this crate, which is exactly what the derive's single-field tuple-variant delegation needs.
/// Restructuring `ProgramOperation`'s ~65 variants into per-register named-field shapes (mirroring
/// `fem2d`'s `Fem2dOperation::SetNode { index, node }`/`RemoveNode { id }`) would ripple into every
/// `architect-spine` construction/match site already keyed on `ProgramOperation::X(CollectionOperation::Y{..})`
/// (21+ call sites, several macro-generated) — out of this ticket's scope, and `architect-spine`
/// isn't part of it. A compact JSON-line encoding satisfies `OpText`'s actual law (`parse_op(print_op(op))
/// == op`, one physical line — compact JSON never emits a raw `\n`, only the escaped `\\n`) without
/// touching any of that call-site surface, mirroring the same escape-hatch spirit the dsl engine's own
/// `serde_json::Value`/`Shape::Value` binding already uses for genuinely untyped fields.
impl protocol::OpText for ProgramOperation {
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        serde_json::from_str(line.trim()).map_err(|e| store::TextError::new(format!("invalid program operation: {e}"), store::TextSpan::at(1, 1)))
    }

    fn print_op(&self) -> String {
        serde_json::to_string(self).expect("ProgramOperation always serializes")
    }
}

/// @emoji 🌱️ Binary twin of the `OpText` escape hatch above — same rationale, plain JSON bytes
/// (mirrors `store::pack_rt`'s JSON-bridge treatment of `DocumentPack for serde_json::Value`,
/// one level down at the op-payload granularity instead of a whole doc).
impl protocol::OpBinary for ProgramOperation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        serde_json::to_vec(self).map_err(|error| protocol::ProtocolError::Malformed { what: "program operation", offset: 0, detail: error.to_string() })
    }

    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        serde_json::from_slice(bytes).map_err(|error| protocol::ProtocolError::Malformed { what: "program operation", offset: 0, detail: error.to_string() })
    }
}
//#endregion 🔖️OpText

/// @emoji 🩹️ Inverse patch carrier for trace link collection operations.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TraceLinkPatch {
    pub from_id: Option<EntityId>,
    pub to_id: Option<EntityId>,
    pub kind: Option<crate::artifacts::program::kernel::TraceKind>,
    pub label: Option<Option<String>>,
}

//#region 🔖️OpCodec
/// 🎞️ Handcrafted OpText (P6).
impl protocol::OpText for TraceLinkPatch {
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        let variants = <Self as dsl::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            let probe = format!("{} ", keyword);
            if line == keyword.as_str() || line.starts_with(&probe) {
                let record = dsl::parse(
                    line,
                    &spec_fn(),
                    &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline },
                )?;
                return <Self as dsl::DslVariants>::from_named_record(keyword, &record);
            }
        }
        Err(dsl::__rt::field_error(format!("unknown operation line '{line}'")))
    }
    fn print_op(&self) -> String {
        let (keyword, record) = <Self as dsl::DslVariants>::to_named_record(self);
        let variants = <Self as dsl::DslVariants>::variants();
        let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec must exist for its own keyword");
        dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)
    }
}

/// 🎯️ Handcrafted OpBinary (P6).
impl protocol::OpBinary for TraceLinkPatch {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        const OP_BINARY_FORMAT: u8 = 1;
        let (keyword, record) = <Self as dsl::DslVariants>::to_named_record(self);
        let variants = <Self as dsl::DslVariants>::variants();
        let ordinal = variants.iter().position(|(k, _)| *k == keyword).ok_or(protocol::ProtocolError::Malformed {
            what: "op variant",
            offset: 0,
            detail: format!("keyword {keyword:?} is not a declared variant"),
        })?;
        let spec = (variants[ordinal].1)();
        let body = store::pack_rt::encode_record_body(&spec, &record, &store::PackEncodeOptions::default()).map_err(protocol::ProtocolError::from)?;
        let mut out = Vec::with_capacity(body.len() + 3);
        out.push(OP_BINARY_FORMAT);
        store::pack_rt::write_varint_u64(&mut out, ordinal as u64);
        out.extend_from_slice(&body);
        Ok(out)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        const OP_BINARY_FORMAT: u8 = 1;
        let mut reader = store::pack_rt::ByteReader::new(bytes);
        let format = reader.read_u8()?;
        if format != OP_BINARY_FORMAT {
            return Err(protocol::ProtocolError::Malformed { what: "op format", offset: 0, detail: format!("unsupported op format {format}") });
        }
        let ordinal = reader.read_varint_u64()?;
        let variants = <Self as dsl::DslVariants>::variants();
        let (keyword, spec_fn) = variants.get(ordinal as usize).ok_or(protocol::ProtocolError::Malformed {
            what: "op variant",
            offset: 1,
            detail: format!("ordinal {ordinal} out of range for {} declared variants", variants.len()),
        })?;
        let spec = spec_fn();
        let body = &bytes[reader.position()..];
        let (record, _report) = store::pack_rt::decode_record_body(body, &spec, &store::PackDecodeOptions::default()).map_err(protocol::ProtocolError::from)?;
        <Self as dsl::DslVariants>::from_named_record(keyword, &record).map_err(|error| protocol::ProtocolError::Malformed {
            what: "op record",
            offset: reader.position() as u64,
            detail: error.to_string(),
        })
    }
}
//#endregion 🔖️OpCodec


impl Patchable<TraceLinkPatch> for TraceLink {
    fn apply_patch(&mut self, patch: &TraceLinkPatch) {
        if let Some(value) = &patch.from_id {
            self.from_id = value.clone();
        }
        if let Some(value) = &patch.to_id {
            self.to_id = value.clone();
        }
        if let Some(value) = &patch.kind {
            self.kind = value.clone();
        }
        if let Some(value) = &patch.label {
            self.label = value.clone();
        }
    }

    fn diff_patch(&self, other: &Self) -> Option<TraceLinkPatch> {
        Some(TraceLinkPatch {
            from_id: Some(other.from_id.clone()),
            to_id: Some(other.to_id.clone()),
            kind: Some(other.kind.clone()),
            label: Some(other.label.clone()),
        })
    }
}
// #endregion

// #region 🔖️Apply
/// @emoji ▶️ Applies one plugin operation to the document in place.
pub fn apply_plugin_operation(program: &mut Program, operation: &ProgramOperation) {
    match operation {
        ProgramOperation::Stakeholders(collection_operation) => apply_collection_operation(&mut program.stakeholders, collection_operation),
        ProgramOperation::Users(collection_operation) => apply_collection_operation(&mut program.users, collection_operation),
        ProgramOperation::Activities(collection_operation) => apply_collection_operation(&mut program.activities, collection_operation),
        ProgramOperation::Functions(collection_operation) => apply_collection_operation(&mut program.functions, collection_operation),
        ProgramOperation::Elements(collection_operation) => apply_collection_operation(&mut program.elements, collection_operation),
        ProgramOperation::Quantities(collection_operation) => apply_collection_operation(&mut program.quantities, collection_operation),
        ProgramOperation::Relationships(collection_operation) => apply_collection_operation(&mut program.relationships, collection_operation),
        ProgramOperation::Adjacencies(collection_operation) => apply_collection_operation(&mut program.adjacencies, collection_operation),
        ProgramOperation::Processes(collection_operation) => apply_collection_operation(&mut program.processes, collection_operation),
        ProgramOperation::Flows(collection_operation) => apply_collection_operation(&mut program.flows, collection_operation),
        ProgramOperation::AccessRules(collection_operation) => apply_collection_operation(&mut program.access_rules, collection_operation),
        ProgramOperation::Operations(collection_operation) => apply_collection_operation(&mut program.operations, collection_operation),
        ProgramOperation::Equipment(collection_operation) => apply_collection_operation(&mut program.equipment, collection_operation),
        ProgramOperation::Resources(collection_operation) => apply_collection_operation(&mut program.resources, collection_operation),
        ProgramOperation::Storage(collection_operation) => apply_collection_operation(&mut program.storage, collection_operation),
        ProgramOperation::Environmental(collection_operation) => apply_collection_operation(&mut program.environmental, collection_operation),
        ProgramOperation::HumanFactors(collection_operation) => apply_collection_operation(&mut program.human_factors, collection_operation),
        ProgramOperation::Accessibility(collection_operation) => apply_collection_operation(&mut program.accessibility, collection_operation),
        ProgramOperation::Privacy(collection_operation) => apply_collection_operation(&mut program.privacy, collection_operation),
        ProgramOperation::Safety(collection_operation) => apply_collection_operation(&mut program.safety, collection_operation),
        ProgramOperation::Security(collection_operation) => apply_collection_operation(&mut program.security, collection_operation),
        ProgramOperation::Regulatory(collection_operation) => apply_collection_operation(&mut program.regulatory, collection_operation),
        ProgramOperation::SiteContext(collection_operation) => apply_collection_operation(&mut program.site_context, collection_operation),
        ProgramOperation::Organizational(collection_operation) => apply_collection_operation(&mut program.organizational, collection_operation),
        ProgramOperation::Services(collection_operation) => apply_collection_operation(&mut program.services, collection_operation),
        ProgramOperation::Infrastructure(collection_operation) => apply_collection_operation(&mut program.infrastructure, collection_operation),
        ProgramOperation::Information(collection_operation) => apply_collection_operation(&mut program.information, collection_operation),
        ProgramOperation::Communication(collection_operation) => apply_collection_operation(&mut program.communication, collection_operation),
        ProgramOperation::Wayfinding(collection_operation) => apply_collection_operation(&mut program.wayfinding, collection_operation),
        ProgramOperation::Schedules(collection_operation) => apply_collection_operation(&mut program.schedules, collection_operation),
        ProgramOperation::Flexibility(collection_operation) => apply_collection_operation(&mut program.flexibility, collection_operation),
        ProgramOperation::Growth(collection_operation) => apply_collection_operation(&mut program.growth, collection_operation),
        ProgramOperation::Sustainability(collection_operation) => apply_collection_operation(&mut program.sustainability, collection_operation),
        ProgramOperation::Resilience(collection_operation) => apply_collection_operation(&mut program.resilience, collection_operation),
        ProgramOperation::Costs(collection_operation) => apply_collection_operation(&mut program.costs, collection_operation),
        ProgramOperation::Delivery(collection_operation) => apply_collection_operation(&mut program.delivery, collection_operation),
        ProgramOperation::Risks(collection_operation) => apply_collection_operation(&mut program.risks, collection_operation),
        ProgramOperation::Conflicts(collection_operation) => apply_collection_operation(&mut program.conflicts, collection_operation),
        ProgramOperation::Requirements(collection_operation) => apply_collection_operation(&mut program.requirements, collection_operation),
        ProgramOperation::Priorities(collection_operation) => apply_collection_operation(&mut program.priorities, collection_operation),
        ProgramOperation::Scenarios(collection_operation) => apply_collection_operation(&mut program.scenarios, collection_operation),
        ProgramOperation::Options(collection_operation) => apply_collection_operation(&mut program.options, collection_operation),
        ProgramOperation::Decisions(collection_operation) => apply_collection_operation(&mut program.decisions, collection_operation),
        ProgramOperation::Validations(collection_operation) => apply_collection_operation(&mut program.validations, collection_operation),
        ProgramOperation::Performance(collection_operation) => apply_collection_operation(&mut program.performance, collection_operation),
        ProgramOperation::Quality(collection_operation) => apply_collection_operation(&mut program.quality, collection_operation),
        ProgramOperation::Documents(collection_operation) => apply_collection_operation(&mut program.documents, collection_operation),
        ProgramOperation::Changes(collection_operation) => apply_collection_operation(&mut program.changes, collection_operation),
        ProgramOperation::Collaboration(collection_operation) => apply_collection_operation(&mut program.collaboration, collection_operation),
        ProgramOperation::Analyses(collection_operation) => apply_collection_operation(&mut program.analyses, collection_operation),
        ProgramOperation::Reports(collection_operation) => apply_collection_operation(&mut program.reports, collection_operation),
        ProgramOperation::SearchFilters(collection_operation) => apply_collection_operation(&mut program.search_filters, collection_operation),
        ProgramOperation::StatusRecords(collection_operation) => apply_collection_operation(&mut program.status_records, collection_operation),
        ProgramOperation::Workshops(collection_operation) => apply_collection_operation(&mut program.workshops, collection_operation),
        ProgramOperation::Surveys(collection_operation) => apply_collection_operation(&mut program.surveys, collection_operation),
        ProgramOperation::Issues(collection_operation) => apply_collection_operation(&mut program.issues, collection_operation),
        ProgramOperation::AuditEvents(collection_operation) => apply_collection_operation(&mut program.audit_events, collection_operation),
        ProgramOperation::Templates(collection_operation) => apply_collection_operation(&mut program.templates, collection_operation),
        ProgramOperation::Knowledge(collection_operation) => apply_collection_operation(&mut program.knowledge, collection_operation),
        ProgramOperation::Benchmarks(collection_operation) => apply_collection_operation(&mut program.benchmarks, collection_operation),
        ProgramOperation::Assumptions(collection_operation) => apply_collection_operation(&mut program.assumptions, collection_operation),
        ProgramOperation::Constraints(collection_operation) => apply_collection_operation(&mut program.constraints, collection_operation),
        ProgramOperation::ComplianceRecords(collection_operation) => {
            apply_collection_operation(&mut program.compliance_records, collection_operation);
        }
        ProgramOperation::Approvals(collection_operation) => apply_collection_operation(&mut program.approvals, collection_operation),
        ProgramOperation::Meetings(collection_operation) => apply_collection_operation(&mut program.meetings, collection_operation),
        ProgramOperation::Traces(collection_operation) => apply_collection_operation(&mut program.traces, collection_operation),
        ProgramOperation::UpdateMeta { patch } => {
            program.meta.apply_patch(patch);
        }
        ProgramOperation::UpdateProject { patch } => {
            program.project.apply_patch(patch);
        }
        ProgramOperation::UpdateGovernance { patch } => {
            program.governance.apply_patch(patch);
        }
        ProgramOperation::SetAdjacency { adjacency } => set_adjacency(program, adjacency.clone()),
        ProgramOperation::ClearAdjacency { id } => clear_adjacency(program, id),
        ProgramOperation::SetProgram { program: replacement } => *program = (**replacement).clone(),
    }
}

/// @emoji ↩️ Computes the inverse operation from pre-state for undo.
pub fn invert_plugin_operation(program: &Program, operation: &ProgramOperation) -> ProgramOperation {
    match operation {
        ProgramOperation::Stakeholders(collection_operation) => ProgramOperation::Stakeholders(invert_collection_operation(&program.stakeholders, collection_operation)),
        ProgramOperation::Users(collection_operation) => ProgramOperation::Users(invert_collection_operation(&program.users, collection_operation)),
        ProgramOperation::Activities(collection_operation) => ProgramOperation::Activities(invert_collection_operation(&program.activities, collection_operation)),
        ProgramOperation::Functions(collection_operation) => ProgramOperation::Functions(invert_collection_operation(&program.functions, collection_operation)),
        ProgramOperation::Elements(collection_operation) => ProgramOperation::Elements(invert_collection_operation(&program.elements, collection_operation)),
        ProgramOperation::Quantities(collection_operation) => ProgramOperation::Quantities(invert_collection_operation(&program.quantities, collection_operation)),
        ProgramOperation::Relationships(collection_operation) => ProgramOperation::Relationships(invert_collection_operation(&program.relationships, collection_operation)),
        ProgramOperation::Adjacencies(collection_operation) => ProgramOperation::Adjacencies(invert_collection_operation(&program.adjacencies, collection_operation)),
        ProgramOperation::Processes(collection_operation) => ProgramOperation::Processes(invert_collection_operation(&program.processes, collection_operation)),
        ProgramOperation::Flows(collection_operation) => ProgramOperation::Flows(invert_collection_operation(&program.flows, collection_operation)),
        ProgramOperation::AccessRules(collection_operation) => ProgramOperation::AccessRules(invert_collection_operation(&program.access_rules, collection_operation)),
        ProgramOperation::Operations(collection_operation) => ProgramOperation::Operations(invert_collection_operation(&program.operations, collection_operation)),
        ProgramOperation::Equipment(collection_operation) => ProgramOperation::Equipment(invert_collection_operation(&program.equipment, collection_operation)),
        ProgramOperation::Resources(collection_operation) => ProgramOperation::Resources(invert_collection_operation(&program.resources, collection_operation)),
        ProgramOperation::Storage(collection_operation) => ProgramOperation::Storage(invert_collection_operation(&program.storage, collection_operation)),
        ProgramOperation::Environmental(collection_operation) => ProgramOperation::Environmental(invert_collection_operation(&program.environmental, collection_operation)),
        ProgramOperation::HumanFactors(collection_operation) => ProgramOperation::HumanFactors(invert_collection_operation(&program.human_factors, collection_operation)),
        ProgramOperation::Accessibility(collection_operation) => ProgramOperation::Accessibility(invert_collection_operation(&program.accessibility, collection_operation)),
        ProgramOperation::Privacy(collection_operation) => ProgramOperation::Privacy(invert_collection_operation(&program.privacy, collection_operation)),
        ProgramOperation::Safety(collection_operation) => ProgramOperation::Safety(invert_collection_operation(&program.safety, collection_operation)),
        ProgramOperation::Security(collection_operation) => ProgramOperation::Security(invert_collection_operation(&program.security, collection_operation)),
        ProgramOperation::Regulatory(collection_operation) => ProgramOperation::Regulatory(invert_collection_operation(&program.regulatory, collection_operation)),
        ProgramOperation::SiteContext(collection_operation) => ProgramOperation::SiteContext(invert_collection_operation(&program.site_context, collection_operation)),
        ProgramOperation::Organizational(collection_operation) => ProgramOperation::Organizational(invert_collection_operation(&program.organizational, collection_operation)),
        ProgramOperation::Services(collection_operation) => ProgramOperation::Services(invert_collection_operation(&program.services, collection_operation)),
        ProgramOperation::Infrastructure(collection_operation) => ProgramOperation::Infrastructure(invert_collection_operation(&program.infrastructure, collection_operation)),
        ProgramOperation::Information(collection_operation) => ProgramOperation::Information(invert_collection_operation(&program.information, collection_operation)),
        ProgramOperation::Communication(collection_operation) => ProgramOperation::Communication(invert_collection_operation(&program.communication, collection_operation)),
        ProgramOperation::Wayfinding(collection_operation) => ProgramOperation::Wayfinding(invert_collection_operation(&program.wayfinding, collection_operation)),
        ProgramOperation::Schedules(collection_operation) => ProgramOperation::Schedules(invert_collection_operation(&program.schedules, collection_operation)),
        ProgramOperation::Flexibility(collection_operation) => ProgramOperation::Flexibility(invert_collection_operation(&program.flexibility, collection_operation)),
        ProgramOperation::Growth(collection_operation) => ProgramOperation::Growth(invert_collection_operation(&program.growth, collection_operation)),
        ProgramOperation::Sustainability(collection_operation) => ProgramOperation::Sustainability(invert_collection_operation(&program.sustainability, collection_operation)),
        ProgramOperation::Resilience(collection_operation) => ProgramOperation::Resilience(invert_collection_operation(&program.resilience, collection_operation)),
        ProgramOperation::Costs(collection_operation) => ProgramOperation::Costs(invert_collection_operation(&program.costs, collection_operation)),
        ProgramOperation::Delivery(collection_operation) => ProgramOperation::Delivery(invert_collection_operation(&program.delivery, collection_operation)),
        ProgramOperation::Risks(collection_operation) => ProgramOperation::Risks(invert_collection_operation(&program.risks, collection_operation)),
        ProgramOperation::Conflicts(collection_operation) => ProgramOperation::Conflicts(invert_collection_operation(&program.conflicts, collection_operation)),
        ProgramOperation::Requirements(collection_operation) => ProgramOperation::Requirements(invert_collection_operation(&program.requirements, collection_operation)),
        ProgramOperation::Priorities(collection_operation) => ProgramOperation::Priorities(invert_collection_operation(&program.priorities, collection_operation)),
        ProgramOperation::Scenarios(collection_operation) => ProgramOperation::Scenarios(invert_collection_operation(&program.scenarios, collection_operation)),
        ProgramOperation::Options(collection_operation) => ProgramOperation::Options(invert_collection_operation(&program.options, collection_operation)),
        ProgramOperation::Decisions(collection_operation) => ProgramOperation::Decisions(invert_collection_operation(&program.decisions, collection_operation)),
        ProgramOperation::Validations(collection_operation) => ProgramOperation::Validations(invert_collection_operation(&program.validations, collection_operation)),
        ProgramOperation::Performance(collection_operation) => ProgramOperation::Performance(invert_collection_operation(&program.performance, collection_operation)),
        ProgramOperation::Quality(collection_operation) => ProgramOperation::Quality(invert_collection_operation(&program.quality, collection_operation)),
        ProgramOperation::Documents(collection_operation) => ProgramOperation::Documents(invert_collection_operation(&program.documents, collection_operation)),
        ProgramOperation::Changes(collection_operation) => ProgramOperation::Changes(invert_collection_operation(&program.changes, collection_operation)),
        ProgramOperation::Collaboration(collection_operation) => ProgramOperation::Collaboration(invert_collection_operation(&program.collaboration, collection_operation)),
        ProgramOperation::Analyses(collection_operation) => ProgramOperation::Analyses(invert_collection_operation(&program.analyses, collection_operation)),
        ProgramOperation::Reports(collection_operation) => ProgramOperation::Reports(invert_collection_operation(&program.reports, collection_operation)),
        ProgramOperation::SearchFilters(collection_operation) => ProgramOperation::SearchFilters(invert_collection_operation(&program.search_filters, collection_operation)),
        ProgramOperation::StatusRecords(collection_operation) => ProgramOperation::StatusRecords(invert_collection_operation(&program.status_records, collection_operation)),
        ProgramOperation::Workshops(collection_operation) => ProgramOperation::Workshops(invert_collection_operation(&program.workshops, collection_operation)),
        ProgramOperation::Surveys(collection_operation) => ProgramOperation::Surveys(invert_collection_operation(&program.surveys, collection_operation)),
        ProgramOperation::Issues(collection_operation) => ProgramOperation::Issues(invert_collection_operation(&program.issues, collection_operation)),
        ProgramOperation::AuditEvents(collection_operation) => ProgramOperation::AuditEvents(invert_collection_operation(&program.audit_events, collection_operation)),
        ProgramOperation::Templates(collection_operation) => ProgramOperation::Templates(invert_collection_operation(&program.templates, collection_operation)),
        ProgramOperation::Knowledge(collection_operation) => ProgramOperation::Knowledge(invert_collection_operation(&program.knowledge, collection_operation)),
        ProgramOperation::Benchmarks(collection_operation) => ProgramOperation::Benchmarks(invert_collection_operation(&program.benchmarks, collection_operation)),
        ProgramOperation::Assumptions(collection_operation) => ProgramOperation::Assumptions(invert_collection_operation(&program.assumptions, collection_operation)),
        ProgramOperation::Constraints(collection_operation) => ProgramOperation::Constraints(invert_collection_operation(&program.constraints, collection_operation)),
        ProgramOperation::ComplianceRecords(collection_operation) => ProgramOperation::ComplianceRecords(invert_collection_operation(&program.compliance_records, collection_operation)),
        ProgramOperation::Approvals(collection_operation) => ProgramOperation::Approvals(invert_collection_operation(&program.approvals, collection_operation)),
        ProgramOperation::Meetings(collection_operation) => ProgramOperation::Meetings(invert_collection_operation(&program.meetings, collection_operation)),
        ProgramOperation::Traces(collection_operation) => ProgramOperation::Traces(invert_collection_operation(&program.traces, collection_operation)),
        ProgramOperation::UpdateMeta { patch } => {
            let prior = program.meta.clone();
            let mut probe = prior.clone();
            probe.apply_patch(patch);
            let inverse = probe.diff_patch(&prior).expect("diff_patch always produces a snapshot patch");
            ProgramOperation::UpdateMeta { patch: inverse }
        }
        ProgramOperation::UpdateProject { patch } => {
            let prior = program.project.clone();
            let mut probe = prior.clone();
            probe.apply_patch(patch);
            let inverse = probe.diff_patch(&prior).expect("diff_patch always produces a snapshot patch");
            ProgramOperation::UpdateProject { patch: inverse }
        }
        ProgramOperation::UpdateGovernance { patch } => {
            let prior = program.governance.clone();
            let mut probe = prior.clone();
            probe.apply_patch(patch);
            let inverse = probe.diff_patch(&prior).expect("diff_patch always produces a snapshot patch");
            ProgramOperation::UpdateGovernance { patch: inverse }
        }
        ProgramOperation::SetAdjacency { adjacency } => {
            if let Some(existing) = program.adjacencies.iter().find(|row| row.header.id == adjacency.header.id) {
                ProgramOperation::SetAdjacency { adjacency: existing.clone() }
            } else {
                ProgramOperation::ClearAdjacency { id: adjacency.header.id.clone() }
            }
        }
        ProgramOperation::ClearAdjacency { id } => match program.adjacencies.iter().find(|row| &row.header.id == id).cloned() {
            Some(existing) => ProgramOperation::SetAdjacency { adjacency: existing },
            None => ProgramOperation::ClearAdjacency { id: id.clone() },
        },
        ProgramOperation::SetProgram { .. } => ProgramOperation::SetProgram { program: Box::new(program.clone()) },
    }
}
// #endregion

// #region 🔖️Operation
impl Operation<Program> for ProgramOperation {
    type Diff = ProgramDiff;

    fn diff(&self, _projection: &Program) -> ProgramDiff {
        ProgramDiff { operations: vec![self.clone()] }
    }

    fn backwards(&self, projection: &Program) -> Vec<Self> {
        vec![invert_plugin_operation(projection, self)]
    }
}
// #endregion

#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::program::kernel::*;
    use crate::artifacts::program::{empty_plugin, sample_plugin};
    use protocol::OpText;

    #[test]
    fn update_meta_round_trips_undo() {
        let mut program = empty_plugin();
        let operation = ProgramOperation::UpdateMeta { patch: ProgramMetaPatch { title: Some("Clinic".into()), ..Default::default() } };
        let inverse = invert_plugin_operation(&program, &operation);
        apply_plugin_operation(&mut program, &operation);
        assert_eq!(program.meta.title, "Clinic");
        apply_plugin_operation(&mut program, &inverse);
        assert_ne!(program.meta.title, "Clinic");
    }

    #[test]
    fn add_stakeholder_via_collection_operation() {
        let mut program = sample_plugin();
        let before = program.stakeholders.len();
        let stakeholder = Stakeholder {
            header: EntityHeader::new(EntityId::new_serial("stakeholder", "Nurse Lead"), "Nurse Lead"),
            role: "Clinical".into(),
            organization: "Sample Health".into(),
            department: None,
            contact_email: None,
            contact_phone: None,
            influence: InfluenceLevel::Medium,
            interest: InfluenceLevel::High,
            engagement: EngagementLevel::Supportive,
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
            stakeholder_type: "Clinical".into(),
            influence_strategy: None,
            communication_channels: Vec::new(),
            success_metrics: Vec::new(),
        };
        let id = stakeholder.header.id.clone();
        let operation = ProgramOperation::Stakeholders(CollectionOperation::Add { index: program.stakeholders.len(), item: stakeholder });
        apply_plugin_operation(&mut program, &operation);
        assert_eq!(program.stakeholders.len(), before + 1);
        let undo = invert_plugin_operation(&program, &operation);
        apply_plugin_operation(&mut program, &undo);
        assert!(!program.stakeholders.iter().any(|s| s.header.id == id));
    }

    #[test]
    fn set_plugin_bulk_replace() {
        let mut program = empty_plugin();
        let sample = sample_plugin();
        apply_plugin_operation(&mut program, &ProgramOperation::SetProgram { program: Box::new(sample.clone()) });
        assert_eq!(program.elements.len(), sample.elements.len());
    }

    #[test]
    fn update_project_round_trips_undo() {
        let mut program = empty_plugin();
        let operation = ProgramOperation::UpdateProject { patch: ProjectDefinitionPatch { code: Some("CLN-002".into()), ..Default::default() } };
        let inverse = invert_plugin_operation(&program, &operation);
        apply_plugin_operation(&mut program, &operation);
        assert_eq!(program.project.code, "CLN-002");
        apply_plugin_operation(&mut program, &inverse);
        assert_ne!(program.project.code, "CLN-002");
    }

    #[test]
    fn update_governance_round_trips_undo() {
        let mut program = empty_plugin();
        let operation = ProgramOperation::UpdateGovernance { patch: GovernancePatch { framework: Some("RACI".into()), ..Default::default() } };
        let inverse = invert_plugin_operation(&program, &operation);
        apply_plugin_operation(&mut program, &operation);
        assert_eq!(program.governance.framework, "RACI");
        apply_plugin_operation(&mut program, &inverse);
        assert_ne!(program.governance.framework, "RACI");
    }

    #[test]
    fn set_and_clear_adjacency_round_trips_undo() {
        let mut program = sample_plugin();
        let before = program.adjacencies.clone();
        let mut new_adjacency = before[0].clone();
        new_adjacency.header.id = EntityId::new_serial("adjacency", "adjacency");
        new_adjacency.element_a_id = EntityId::new_serial("element", "element");
        new_adjacency.element_b_id = EntityId::new_serial("element", "element");
        new_adjacency.weight = 5.0;
        let set_op = ProgramOperation::SetAdjacency { adjacency: new_adjacency.clone() };
        let set_undo = invert_plugin_operation(&program, &set_op);
        assert!(matches!(set_undo, ProgramOperation::ClearAdjacency { .. }));
        apply_plugin_operation(&mut program, &set_op);
        assert_eq!(program.adjacencies.len(), before.len() + 1);
        assert!(program.adjacencies.iter().any(|a| a.header.id == new_adjacency.header.id));
        apply_plugin_operation(&mut program, &set_undo);
        assert_eq!(program.adjacencies.len(), before.len());
        assert!(!program.adjacencies.iter().any(|a| a.header.id == new_adjacency.header.id));

        let clear_op = ProgramOperation::ClearAdjacency { id: before[0].header.id.clone() };
        let clear_undo = invert_plugin_operation(&program, &clear_op);
        assert!(matches!(clear_undo, ProgramOperation::SetAdjacency { .. }));
        apply_plugin_operation(&mut program, &clear_op);
        assert!(!program.adjacencies.iter().any(|a| a.header.id == before[0].header.id));
        apply_plugin_operation(&mut program, &clear_undo);
        assert!(program.adjacencies.iter().any(|a| a.header.id == before[0].header.id));
    }

    #[test]
    fn dispatches_traces_add_and_invert() {
        let mut program = empty_plugin();
        let link = TraceLink::new(EntityId::new_serial("tfrom", "tfrom"), EntityId::new_serial("tto", "tto"), TraceKind::RequirementToDecision);
        let operation = ProgramOperation::Traces(CollectionOperation::Add { index: 0, item: link });
        apply_plugin_operation(&mut program, &operation);
        assert_eq!(program.traces.len(), 1);
        let undo = invert_plugin_operation(&program, &operation);
        apply_plugin_operation(&mut program, &undo);
        assert!(program.traces.is_empty());
    }

    #[test]
    fn dispatches_access_rules_add_and_invert() {
        let mut program = empty_plugin();
        let item = AccessRule {
            header: EntityHeader::new(EntityId::new_serial("accessrule0", "AccessRule 0"), "AccessRule 0"),
            subject_ids: Vec::new(),
            resource_ids: Vec::new(),
            access_level: AccessLevel::Public,
            access_mode: AccessMode::Unrestricted,
            authentication: Vec::new(),
            authorization: Vec::new(),
            time_restrictions: Vec::new(),
            escort_policy: None,
            visitor_policy: None,
            emergency_override: false,
            audit_required: false,
            badge_required: false,
            biometric_required: false,
            zone_ids: Vec::new(),
            exceptions: Vec::new(),
            regulatory_basis: Vec::new(),
            enforcement_method: None,
            revocation_policy: None,
            training_required: false,
            owner_id: None,
        };
        let operation = ProgramOperation::AccessRules(CollectionOperation::Add { index: 0, item });
        apply_plugin_operation(&mut program, &operation);
        assert_eq!(program.access_rules.len(), 1);
        let undo = invert_plugin_operation(&program, &operation);
        apply_plugin_operation(&mut program, &undo);
        assert!(program.access_rules.is_empty());
    }

    #[test]
    fn dispatches_accessibility_add_and_invert() {
        let mut program = empty_plugin();
        let item = AccessibilityRequirement {
            header: EntityHeader::new(EntityId::new_serial("accessibilityrequirement1", "AccessibilityRequirement 1"), "AccessibilityRequirement 1"),
            standard: String::new(),
            level: None,
            user_profile_ids: Vec::new(),
            element_ids: Vec::new(),
            route_ids: Vec::new(),
            clear_width_m: None,
            clear_height_m: None,
            turning_circle_m: None,
            ramp_slope: None,
            lift_required: false,
            tactile_guidance: false,
            hearing_loop: false,
            visual_contrast: false,
            signage_requirements: Vec::new(),
            controls_height: None,
            emergency_evacuation: Vec::new(),
            service_animal_policy: None,
            companion_seating: false,
            verification_plan: None,
            exceptions: Vec::new(),
            wcag_conformance: None,
            universal_design_principles: Vec::new(),
        };
        let operation = ProgramOperation::Accessibility(CollectionOperation::Add { index: 0, item });
        apply_plugin_operation(&mut program, &operation);
        assert_eq!(program.accessibility.len(), 1);
        let undo = invert_plugin_operation(&program, &operation);
        apply_plugin_operation(&mut program, &undo);
        assert!(program.accessibility.is_empty());
    }

    #[test]
    fn dispatches_activities_add_and_invert() {
        let mut program = empty_plugin();
        let item = Activity {
            header: EntityHeader::new(EntityId::new_serial("activity2", "Activity 2"), "Activity 2"),
            code: String::new(),
            category: String::new(),
            frequency: None,
            duration: None,
            intensity: None,
            participants: QuantitySpec::default(),
            equipment_ids: Vec::new(),
            space_requirements: Vec::new(),
            environmental_needs: Vec::new(),
            privacy_needs: Vec::new(),
            accessibility_needs: Vec::new(),
            adjacent_activities: Vec::new(),
            sequencing: Vec::new(),
            peak_periods: Vec::new(),
            workflow_steps: Vec::new(),
            inputs: Vec::new(),
            outputs: Vec::new(),
            user_profile_ids: Vec::new(),
            function_ids: Vec::new(),
            performance_indicators: Vec::new(),
            activity_type: String::new(),
            location_context: None,
            temporal_pattern: None,
            supervision_level: None,
        };
        let operation = ProgramOperation::Activities(CollectionOperation::Add { index: 0, item });
        apply_plugin_operation(&mut program, &operation);
        assert_eq!(program.activities.len(), 1);
        let undo = invert_plugin_operation(&program, &operation);
        apply_plugin_operation(&mut program, &undo);
        assert!(program.activities.is_empty());
    }

    #[test]
    fn dispatches_adjacencies_add_and_invert() {
        let mut program = empty_plugin();
        let item = Adjacency {
            header: EntityHeader::new(EntityId::new_serial("adjacency3", "Adjacency 3"), "Adjacency 3"),
            element_a_id: EntityId::new_serial("base3", "base3"),
            element_b_id: EntityId::new_serial("base3", "base3"),
            kind: AdjacencyKind::Required,
            connection: ConnectionKind::Direct,
            separations: Vec::new(),
            weight: 0.0,
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
            normalized: false,
            verification_status: ValidationStatus::Pending,
            source_relationship_id: None,
            internal_external_access: None,
        };
        let operation = ProgramOperation::Adjacencies(CollectionOperation::Add { index: 0, item });
        apply_plugin_operation(&mut program, &operation);
        assert_eq!(program.adjacencies.len(), 1);
        let undo = invert_plugin_operation(&program, &operation);
        apply_plugin_operation(&mut program, &undo);
        assert!(program.adjacencies.is_empty());
    }

    #[test]
    fn dispatches_analyses_add_and_invert() {
        let mut program = empty_plugin();
        let item = AnalysisRecord {
            header: EntityHeader::new(EntityId::new_serial("analysisrecord4", "AnalysisRecord 4"), "AnalysisRecord 4"),
            kind: AnalysisKind::Gap,
            title: String::new(),
            parameters: Vec::new(),
            input_entity_ids: Vec::new(),
            output_summary: TextField::default(),
            findings: Vec::new(),
            metrics: Vec::new(),
            charts: Vec::new(),
            run_by: None,
            run_at: None,
            duration_ms: None,
            tool_version: None,
            scenario_id: None,
            report_id: None,
            confidence: None,
            limitations: Vec::new(),
            recommendations: Vec::new(),
            raw_result_ref: None,
        };
        let operation = ProgramOperation::Analyses(CollectionOperation::Add { index: 0, item });
        apply_plugin_operation(&mut program, &operation);
        assert_eq!(program.analyses.len(), 1);
        let undo = invert_plugin_operation(&program, &operation);
        apply_plugin_operation(&mut program, &undo);
        assert!(program.analyses.is_empty());
    }

    #[test]
    fn dispatches_approvals_add_and_invert() {
        let mut program = empty_plugin();
        let item = ApprovalRecord {
            header: EntityHeader::new(EntityId::new_serial("approvalrecord5", "ApprovalRecord 5"), "ApprovalRecord 5"),
            approval_type: String::new(),
            subject_id: EntityId::new_serial("base5", "base5"),
            approver_ids: Vec::new(),
            approval_date: None,
            conditions: Vec::new(),
            approval_status: LifecycleStatus::Draft,
            expiry_date: None,
            delegation_chain: Vec::new(),
            evidence_refs: Vec::new(),
            related_decision_id: None,
            related_change_id: None,
            authority_basis: Vec::new(),
            signature_method: None,
            rejection_reason: None,
            resubmission_date: None,
            notification_list: Vec::new(),
            workflow_step: None,
            version: None,
            audit_trail_ref: None,
        };
        let operation = ProgramOperation::Approvals(CollectionOperation::Add { index: 0, item });
        apply_plugin_operation(&mut program, &operation);
        assert_eq!(program.approvals.len(), 1);
        let undo = invert_plugin_operation(&program, &operation);
        apply_plugin_operation(&mut program, &undo);
        assert!(program.approvals.is_empty());
    }

    #[test]
    fn dispatches_assumptions_add_and_invert() {
        let mut program = empty_plugin();
        let item = Assumption {
            header: EntityHeader::new(EntityId::new_serial("assumption6", "Assumption 6"), "Assumption 6"),
            statement: TextField::default(),
            basis: None,
            confidence_level: None,
            impact_if_false: None,
            related_entity_ids: Vec::new(),
            validation_status: ValidationStatus::Pending,
            validated_by: None,
            validation_date: None,
            owner_id: None,
            review_cycle: None,
            source: None,
            category: None,
            dependencies: Vec::new(),
            mitigation: Vec::new(),
            linked_requirement_ids: Vec::new(),
            linked_risk_ids: Vec::new(),
            expiration_date: None,
            status_notes: Vec::new(),
            document_refs: Vec::new(),
        };
        let operation = ProgramOperation::Assumptions(CollectionOperation::Add { index: 0, item });
        apply_plugin_operation(&mut program, &operation);
        assert_eq!(program.assumptions.len(), 1);
        let undo = invert_plugin_operation(&program, &operation);
        apply_plugin_operation(&mut program, &undo);
        assert!(program.assumptions.is_empty());
    }

    #[test]
    fn dispatches_audit_events_add_and_invert() {
        let mut program = empty_plugin();
        let item = AuditEvent {
            header: EntityHeader::new(EntityId::new_serial("auditevent7", "AuditEvent 7"), "AuditEvent 7"),
            action: AuditAction::Created,
            actor_id: None,
            subject_id: EntityId::new_serial("base7", "base7"),
            subject_kind: String::new(),
            timestamp: String::new(),
            details: TextField::default(),
            before_state: None,
            after_state: None,
            ip_address: None,
            client: None,
            session_id: None,
            change_record_id: None,
            trace_link: None,
            success: false,
            error_message: None,
            correlation_id: None,
            compliance_tags: Vec::new(),
            retention_until: None,
        };
        let operation = ProgramOperation::AuditEvents(CollectionOperation::Add { index: 0, item });
        apply_plugin_operation(&mut program, &operation);
        assert_eq!(program.audit_events.len(), 1);
        let undo = invert_plugin_operation(&program, &operation);
        apply_plugin_operation(&mut program, &undo);
        assert!(program.audit_events.is_empty());
    }

    #[test]
    fn dispatches_benchmarks_add_and_invert() {
        let mut program = empty_plugin();
        let item = BenchmarkRecord {
            header: EntityHeader::new(EntityId::new_serial("benchmarkrecord8", "BenchmarkRecord 8"), "BenchmarkRecord 8"),
            benchmark_name: String::new(),
            sector: String::new(),
            metric: String::new(),
            value: 0.0,
            unit: String::new(),
            sample_size: None,
            source: None,
            collection_year: None,
            geography: None,
            building_type: None,
            confidence: None,
            methodology: None,
            applicable_element_kinds: Vec::new(),
            related_requirement_ids: Vec::new(),
            comparison_notes: Vec::new(),
            limitations: Vec::new(),
            license: None,
            knowledge_id: None,
            last_verified: None,
        };
        let operation = ProgramOperation::Benchmarks(CollectionOperation::Add { index: 0, item });
        apply_plugin_operation(&mut program, &operation);
        assert_eq!(program.benchmarks.len(), 1);
        let undo = invert_plugin_operation(&program, &operation);
        apply_plugin_operation(&mut program, &undo);
        assert!(program.benchmarks.is_empty());
    }

    #[test]
    fn dispatches_changes_add_and_invert() {
        let mut program = empty_plugin();
        let item = ChangeRecord {
            header: EntityHeader::new(EntityId::new_serial("changerecord9", "ChangeRecord 9"), "ChangeRecord 9"),
            change_type: String::new(),
            summary: TextField::default(),
            reason: TextField::default(),
            requested_by: None,
            approved_by: None,
            change_date: None,
            effective_date: None,
            impacted_entity_ids: Vec::new(),
            before_snapshot: None,
            after_snapshot: None,
            cost_impact: None,
            schedule_impact: None,
            risk_impact: Vec::new(),
            approval_status: ValidationStatus::Pending,
            rollback_plan: Vec::new(),
            communication_plan: Vec::new(),
            version_from: None,
            version_to: None,
            audit_event_ids: Vec::new(),
        };
        let operation = ProgramOperation::Changes(CollectionOperation::Add { index: 0, item });
        apply_plugin_operation(&mut program, &operation);
        assert_eq!(program.changes.len(), 1);
        let undo = invert_plugin_operation(&program, &operation);
        apply_plugin_operation(&mut program, &undo);
        assert!(program.changes.is_empty());
    }

    #[test]
    fn dispatches_collaboration_add_and_invert() {
        let mut program = empty_plugin();
        let item = CollaborationRecord {
            header: EntityHeader::new(EntityId::new_serial("collaborationrecord10", "CollaborationRecord 10"), "CollaborationRecord 10"),
            session_type: String::new(),
            title: String::new(),
            participants: Vec::new(),
            facilitator_id: None,
            start_time: None,
            end_time: None,
            location: None,
            agenda: Vec::new(),
            outcomes: Vec::new(),
            action_items: Vec::new(),
            decision_ids: Vec::new(),
            issue_ids: Vec::new(),
            document_ids: Vec::new(),
            recording_ref: None,
            feedback: Vec::new(),
            follow_up_date: None,
            workshop_id: None,
            survey_id: None,
        };
        let operation = ProgramOperation::Collaboration(CollectionOperation::Add { index: 0, item });
        apply_plugin_operation(&mut program, &operation);
        assert_eq!(program.collaboration.len(), 1);
        let undo = invert_plugin_operation(&program, &operation);
        apply_plugin_operation(&mut program, &undo);
        assert!(program.collaboration.is_empty());
    }

    #[test]
    fn dispatches_communication_add_and_invert() {
        let mut program = empty_plugin();
        let item = CommunicationRequirement {
            header: EntityHeader::new(EntityId::new_serial("communicationrequirement11", "CommunicationRequirement 11"), "CommunicationRequirement 11"),
            channel: String::new(),
            audience_ids: Vec::new(),
            message_types: Vec::new(),
            frequency: None,
            medium: Vec::new(),
            language: Vec::new(),
            accessibility: Vec::new(),
            emergency_use: false,
            two_way: false,
            recording_policy: None,
            signage_locations: Vec::new(),
            technology: Vec::new(),
            escalation_path: Vec::new(),
            feedback_loop: false,
            privacy_controls: Vec::new(),
            element_ids: Vec::new(),
            standards: Vec::new(),
            owner_id: None,
            templates: Vec::new(),
        };
        let operation = ProgramOperation::Communication(CollectionOperation::Add { index: 0, item });
        apply_plugin_operation(&mut program, &operation);
        assert_eq!(program.communication.len(), 1);
        let undo = invert_plugin_operation(&program, &operation);
        apply_plugin_operation(&mut program, &undo);
        assert!(program.communication.is_empty());
    }

    #[test]
    fn dispatches_compliance_records_add_and_invert() {
        let mut program = empty_plugin();
        let item = ComplianceRecord {
            header: EntityHeader::new(EntityId::new_serial("compliancerecord12", "ComplianceRecord 12"), "ComplianceRecord 12"),
            standard_ref: String::new(),
            obligation: TextField::default(),
            compliance_status: ValidationStatus::Pending,
            evidence_refs: Vec::new(),
            auditor_id: None,
            audit_date: None,
            next_review: None,
            affected_entity_ids: Vec::new(),
            gap_analysis: Vec::new(),
            remediation_plan: Vec::new(),
            owner_id: None,
            severity: RiskLevel::Negligible,
            regulatory_body: None,
            certification_target: None,
            waiver_status: None,
            related_requirement_ids: Vec::new(),
            monitoring_method: None,
            reporting_frequency: None,
            penalties: Vec::new(),
            corrective_actions: Vec::new(),
            document_refs: Vec::new(),
        };
        let operation = ProgramOperation::ComplianceRecords(CollectionOperation::Add { index: 0, item });
        apply_plugin_operation(&mut program, &operation);
        assert_eq!(program.compliance_records.len(), 1);
        let undo = invert_plugin_operation(&program, &operation);
        apply_plugin_operation(&mut program, &undo);
        assert!(program.compliance_records.is_empty());
    }

    #[test]
    fn dispatches_conflicts_add_and_invert() {
        let mut program = empty_plugin();
        let item = Conflict {
            header: EntityHeader::new(EntityId::new_serial("conflict13", "Conflict 13"), "Conflict 13"),
            kind: ConflictKind::Adjacency,
            summary: TextField::default(),
            entity_a_id: EntityId::new_serial("base13", "base13"),
            entity_b_id: EntityId::new_serial("base13", "base13"),
            severity: IssueSeverity::Cosmetic,
            detected_by: None,
            detection_date: None,
            trade_off_options: Vec::new(),
            recommended_resolution: None,
            decision_id: None,
            stakeholder_ids: Vec::new(),
            requirement_ids: Vec::new(),
            cost_impact: None,
            schedule_impact: None,
            quality_impact: Vec::new(),
            resolution_status: ValidationStatus::Pending,
            owner_id: None,
            escalation_level: None,
            related_risk_ids: Vec::new(),
        };
        let operation = ProgramOperation::Conflicts(CollectionOperation::Add { index: 0, item });
        apply_plugin_operation(&mut program, &operation);
        assert_eq!(program.conflicts.len(), 1);
        let undo = invert_plugin_operation(&program, &operation);
        apply_plugin_operation(&mut program, &undo);
        assert!(program.conflicts.is_empty());
    }

    #[test]
    fn dispatches_constraints_add_and_invert() {
        let mut program = empty_plugin();
        let item = ConstraintRecord {
            header: EntityHeader::new(EntityId::new_serial("constraintrecord14", "ConstraintRecord 14"), "ConstraintRecord 14"),
            constraint_type: String::new(),
            summary: TextField::default(),
            severity: RiskLevel::Negligible,
            affected_entity_ids: Vec::new(),
            source: None,
            regulatory_basis: Vec::new(),
            mitigation_options: Vec::new(),
            owner_id: None,
            effective_date: None,
            expiry_date: None,
            waiver_status: None,
            waiver_approver: None,
            impact_assessment: None,
            resolution_plan: Vec::new(),
            related_requirement_ids: Vec::new(),
            related_decision_ids: Vec::new(),
            monitoring_frequency: None,
            compliance_status: ValidationStatus::Pending,
            exceptions: Vec::new(),
            trace_links: Vec::new(),
            escalation_contact_id: None,
        };
        let operation = ProgramOperation::Constraints(CollectionOperation::Add { index: 0, item });
        apply_plugin_operation(&mut program, &operation);
        assert_eq!(program.constraints.len(), 1);
        let undo = invert_plugin_operation(&program, &operation);
        apply_plugin_operation(&mut program, &undo);
        assert!(program.constraints.is_empty());
    }

    #[test]
    fn dispatches_costs_add_and_invert() {
        let mut program = empty_plugin();
        let item = CostRequirement {
            header: EntityHeader::new(EntityId::new_serial("costrequirement15", "CostRequirement 15"), "CostRequirement 15"),
            cost_item: String::new(),
            basis: CostBasis::Capital,
            amount: None,
            currency: String::new(),
            quantity_basis: None,
            unit_cost: None,
            contingency_percent: None,
            escalation_rate: None,
            funding_source: None,
            element_ids: Vec::new(),
            requirement_ids: Vec::new(),
            phase: None,
            cash_flow_profile: Vec::new(),
            value_engineering_notes: Vec::new(),
            benchmark_ref: None,
            approval_status: ValidationStatus::Pending,
            owner_id: None,
            assumptions: Vec::new(),
            sensitivity_factors: Vec::new(),
        };
        let operation = ProgramOperation::Costs(CollectionOperation::Add { index: 0, item });
        apply_plugin_operation(&mut program, &operation);
        assert_eq!(program.costs.len(), 1);
        let undo = invert_plugin_operation(&program, &operation);
        apply_plugin_operation(&mut program, &undo);
        assert!(program.costs.is_empty());
    }

    #[test]
    fn dispatches_decisions_add_and_invert() {
        let mut program = empty_plugin();
        let item = Decision {
            header: EntityHeader::new(EntityId::new_serial("decision16", "Decision 16"), "Decision 16"),
            decision_statement: TextField::default(),
            context: TextField::default(),
            options_considered: Vec::new(),
            selected_option_id: None,
            rationale: TextField::default(),
            decision_maker_ids: Vec::new(),
            consulted_ids: Vec::new(),
            informed_ids: Vec::new(),
            decision_date: None,
            effective_date: None,
            reversal_conditions: Vec::new(),
            impacted_requirement_ids: Vec::new(),
            impacted_element_ids: Vec::new(),
            cost_impact: None,
            schedule_impact: None,
            risk_impact: Vec::new(),
            approval_status: ValidationStatus::Pending,
            meeting_ref: None,
            document_refs: Vec::new(),
        };
        let operation = ProgramOperation::Decisions(CollectionOperation::Add { index: 0, item });
        apply_plugin_operation(&mut program, &operation);
        assert_eq!(program.decisions.len(), 1);
        let undo = invert_plugin_operation(&program, &operation);
        apply_plugin_operation(&mut program, &undo);
        assert!(program.decisions.is_empty());
    }

    #[test]
    fn dispatches_delivery_add_and_invert() {
        let mut program = empty_plugin();
        let item = DeliveryConstraint {
            header: EntityHeader::new(EntityId::new_serial("deliveryconstraint17", "DeliveryConstraint 17"), "DeliveryConstraint 17"),
            constraint_type: String::new(),
            constraint_details: TextField::default(),
            phase: DeliveryPhase::Concept,
            hard_deadline: None,
            soft_deadline: None,
            impacted_element_ids: Vec::new(),
            impacted_requirement_ids: Vec::new(),
            work_hours: None,
            noise_restrictions: Vec::new(),
            access_restrictions: Vec::new(),
            site_logistics: Vec::new(),
            procurement_lead_time: None,
            approval_gates: Vec::new(),
            occupancy_constraints: Vec::new(),
            weather_windows: Vec::new(),
            penalty_clauses: Vec::new(),
            mitigation_options: Vec::new(),
            owner_id: None,
            risk_ids: Vec::new(),
            constraint_status: LifecycleStatus::Draft,
        };
        let operation = ProgramOperation::Delivery(CollectionOperation::Add { index: 0, item });
        apply_plugin_operation(&mut program, &operation);
        assert_eq!(program.delivery.len(), 1);
        let undo = invert_plugin_operation(&program, &operation);
        apply_plugin_operation(&mut program, &undo);
        assert!(program.delivery.is_empty());
    }

    #[test]
    fn dispatches_documents_add_and_invert() {
        let mut program = empty_plugin();
        let item = DocumentRecord {
            header: EntityHeader::new(EntityId::new_serial("documentrecord18", "DocumentRecord 18"), "DocumentRecord 18"),
            document_type: String::new(),
            title: String::new(),
            version: String::new(),
            file_ref: None,
            format: None,
            author_ids: Vec::new(),
            reviewer_ids: Vec::new(),
            approver_ids: Vec::new(),
            issue_date: None,
            revision_date: None,
            distribution_list: Vec::new(),
            related_entity_ids: Vec::new(),
            classification: None,
            retention_period: None,
            access_controls: Vec::new(),
            supersedes: None,
            document_status: LifecycleStatus::Draft,
            checksum: None,
            source_system: None,
        };
        let operation = ProgramOperation::Documents(CollectionOperation::Add { index: 0, item });
        apply_plugin_operation(&mut program, &operation);
        assert_eq!(program.documents.len(), 1);
        let undo = invert_plugin_operation(&program, &operation);
        apply_plugin_operation(&mut program, &undo);
        assert!(program.documents.is_empty());
    }

    #[test]
    fn dispatches_elements_add_and_invert() {
        let mut program = empty_plugin();
        let item = ProgramElement {
            header: EntityHeader::new(EntityId::new_serial("programelement19", "ProgramElement 19"), "ProgramElement 19"),
            code: String::new(),
            kind: ProgramElementKind::Building,
            parent_id: None,
            level: None,
            area: QuantitySpec::default(),
            volume: QuantitySpec::default(),
            height: QuantitySpec::default(),
            occupancy: QuantitySpec::default(),
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
        let operation = ProgramOperation::Elements(CollectionOperation::Add { index: 0, item });
        apply_plugin_operation(&mut program, &operation);
        assert_eq!(program.elements.len(), 1);
        let undo = invert_plugin_operation(&program, &operation);
        apply_plugin_operation(&mut program, &undo);
        assert!(program.elements.is_empty());
    }

    #[test]
    fn dispatches_environmental_add_and_invert() {
        let mut program = empty_plugin();
        let item = EnvironmentalRequirement {
            header: EntityHeader::new(EntityId::new_serial("environmentalrequirement20", "EnvironmentalRequirement 20"), "EnvironmentalRequirement 20"),
            parameter_kind: EnvironmentalParameter::Temperature,
            parameter: String::new(),
            target_value: None,
            unit: None,
            min_value: None,
            max_value: None,
            comfort_band: None,
            measurement_method: None,
            monitoring_frequency: None,
            element_ids: Vec::new(),
            occupancy_basis: None,
            seasonal_variation: Vec::new(),
            energy_implications: Vec::new(),
            standards: Vec::new(),
            certification_targets: Vec::new(),
            outdoor_conditions: Vec::new(),
            ventilation_strategy: None,
            daylight_target: None,
            acoustic_target: None,
            iaq_target: None,
            verification_plan: None,
        };
        let operation = ProgramOperation::Environmental(CollectionOperation::Add { index: 0, item });
        apply_plugin_operation(&mut program, &operation);
        assert_eq!(program.environmental.len(), 1);
        let undo = invert_plugin_operation(&program, &operation);
        apply_plugin_operation(&mut program, &undo);
        assert!(program.environmental.is_empty());
    }

    #[test]
    fn dispatches_equipment_add_and_invert() {
        let mut program = empty_plugin();
        let item = Equipment {
            header: EntityHeader::new(EntityId::new_serial("equipment21", "Equipment 21"), "Equipment 21"),
            code: String::new(),
            category: String::new(),
            manufacturer: None,
            model: None,
            quantity: QuantitySpec::default(),
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
            standards: Vec::new(),
            supplier: None,
            activity_link_ids: Vec::new(),
            installation_requirements: Vec::new(),
            commissioning_notes: Vec::new(),
            spare_parts: Vec::new(),
        };
        let operation = ProgramOperation::Equipment(CollectionOperation::Add { index: 0, item });
        apply_plugin_operation(&mut program, &operation);
        assert_eq!(program.equipment.len(), 1);
        let undo = invert_plugin_operation(&program, &operation);
        apply_plugin_operation(&mut program, &undo);
        assert!(program.equipment.is_empty());
    }

    #[test]
    fn dispatches_flexibility_add_and_invert() {
        let mut program = empty_plugin();
        let item = FlexibilityRequirement {
            header: EntityHeader::new(EntityId::new_serial("flexibilityrequirement22", "FlexibilityRequirement 22"), "FlexibilityRequirement 22"),
            flexibility_type: String::new(),
            element_ids: Vec::new(),
            adaptation_scenarios: Vec::new(),
            modularity_level: None,
            reconfiguration_time: None,
            cost_of_change: None,
            technology_readiness: None,
            future_function_ids: Vec::new(),
            demountable_partitions: false,
            raised_floor: false,
            overhead_services: false,
            expansion_direction: Vec::new(),
            contraction_scenario: Vec::new(),
            multi_use_potential: Vec::new(),
            furniture_strategy: Vec::new(),
            infrastructure_spare_capacity: Vec::new(),
            lease_implications: Vec::new(),
            owner_id: None,
        };
        let operation = ProgramOperation::Flexibility(CollectionOperation::Add { index: 0, item });
        apply_plugin_operation(&mut program, &operation);
        assert_eq!(program.flexibility.len(), 1);
        let undo = invert_plugin_operation(&program, &operation);
        apply_plugin_operation(&mut program, &undo);
        assert!(program.flexibility.is_empty());
    }

    #[test]
    fn dispatches_flows_add_and_invert() {
        let mut program = empty_plugin();
        let item = FlowRequirement {
            header: EntityHeader::new(EntityId::new_serial("flowrequirement23", "FlowRequirement 23"), "FlowRequirement 23"),
            from_element_id: EntityId::new_serial("base23", "base23"),
            to_element_id: EntityId::new_serial("base23", "base23"),
            kind: FlowKind::People,
            flow_type: String::new(),
            direction: FlowDirection::OneWay,
            volume: QuantitySpec::default(),
            peak_rate: None,
            clear_width_m: None,
            clear_height_m: None,
            separation_requirements: Vec::new(),
            access_level: AccessLevel::Public,
            time_windows: Vec::new(),
            equipment_clearance: None,
            signage_required: false,
            escort_required: false,
            emergency_route: false,
            barrier_free: false,
            monitoring_required: false,
            process_id: None,
            conflict_ids: Vec::new(),
            verification_method: None,
        };
        let operation = ProgramOperation::Flows(CollectionOperation::Add { index: 0, item });
        apply_plugin_operation(&mut program, &operation);
        assert_eq!(program.flows.len(), 1);
        let undo = invert_plugin_operation(&program, &operation);
        apply_plugin_operation(&mut program, &undo);
        assert!(program.flows.is_empty());
    }

    #[test]
    fn dispatches_functions_add_and_invert() {
        let mut program = empty_plugin();
        let item = Function {
            header: EntityHeader::new(EntityId::new_serial("function24", "Function 24"), "Function 24"),
            code: String::new(),
            kind: FunctionKind::Primary,
            purpose: TextField::default(),
            criticality: Priority::Mandatory,
            performance_targets: Vec::new(),
            service_level: None,
            operating_hours: None,
            staffing: QuantitySpec::default(),
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
        };
        let operation = ProgramOperation::Functions(CollectionOperation::Add { index: 0, item });
        apply_plugin_operation(&mut program, &operation);
        assert_eq!(program.functions.len(), 1);
        let undo = invert_plugin_operation(&program, &operation);
        apply_plugin_operation(&mut program, &undo);
        assert!(program.functions.is_empty());
    }

    #[test]
    fn dispatches_growth_add_and_invert() {
        let mut program = empty_plugin();
        let item = GrowthPlan {
            header: EntityHeader::new(EntityId::new_serial("growthplan25", "GrowthPlan 25"), "GrowthPlan 25"),
            horizon_years: 0,
            growth_rate: None,
            headcount_growth: QuantitySpec::default(),
            area_growth: QuantitySpec::default(),
            phases: Vec::new(),
            trigger_events: Vec::new(),
            expansion_element_ids: Vec::new(),
            reserve_areas: Vec::new(),
            infrastructure_headroom: Vec::new(),
            budget_envelope: None,
            funding_sources: Vec::new(),
            risk_factors: Vec::new(),
            decision_points: Vec::new(),
            scenario_ids: Vec::new(),
            decommission_plan: Vec::new(),
            relocation_strategy: Vec::new(),
            stakeholder_impact: Vec::new(),
            regulatory_considerations: Vec::new(),
            owner_id: None,
        };
        let operation = ProgramOperation::Growth(CollectionOperation::Add { index: 0, item });
        apply_plugin_operation(&mut program, &operation);
        assert_eq!(program.growth.len(), 1);
        let undo = invert_plugin_operation(&program, &operation);
        apply_plugin_operation(&mut program, &undo);
        assert!(program.growth.is_empty());
    }

    #[test]
    fn dispatches_human_factors_add_and_invert() {
        let mut program = empty_plugin();
        let item = HumanFactorRequirement {
            header: EntityHeader::new(EntityId::new_serial("humanfactorrequirement26", "HumanFactorRequirement 26"), "HumanFactorRequirement 26"),
            aspect: HumanFactorAspect::Ergonomics,
            factor: String::new(),
            user_profile_ids: Vec::new(),
            activity_ids: Vec::new(),
            ergonomic_criteria: Vec::new(),
            cognitive_load: None,
            visual_demands: Vec::new(),
            auditory_demands: Vec::new(),
            posture_requirements: Vec::new(),
            reach_envelope: None,
            lighting_for_tasks: Vec::new(),
            thermal_comfort: Vec::new(),
            privacy_needs: Vec::new(),
            social_interaction: Vec::new(),
            stress_factors: Vec::new(),
            mitigation_measures: Vec::new(),
            training_needs: Vec::new(),
            standards: Vec::new(),
            research_basis: Vec::new(),
            element_ids: Vec::new(),
            verification_method: None,
        };
        let operation = ProgramOperation::HumanFactors(CollectionOperation::Add { index: 0, item });
        apply_plugin_operation(&mut program, &operation);
        assert_eq!(program.human_factors.len(), 1);
        let undo = invert_plugin_operation(&program, &operation);
        apply_plugin_operation(&mut program, &undo);
        assert!(program.human_factors.is_empty());
    }

    #[test]
    fn dispatches_information_add_and_invert() {
        let mut program = empty_plugin();
        let item = InformationRequirement {
            header: EntityHeader::new(EntityId::new_serial("informationrequirement27", "InformationRequirement 27"), "InformationRequirement 27"),
            information_type: String::new(),
            format: None,
            source_system: None,
            destination_systems: Vec::new(),
            update_frequency: None,
            retention_period: None,
            access_controls: Vec::new(),
            classification: None,
            quality_criteria: Vec::new(),
            metadata_requirements: Vec::new(),
            integration_points: Vec::new(),
            backup_requirements: Vec::new(),
            disaster_recovery: Vec::new(),
            privacy_controls: Vec::new(),
            audit_trail: false,
            element_ids: Vec::new(),
            stakeholder_ids: Vec::new(),
            standards: Vec::new(),
            owner_id: None,
        };
        let operation = ProgramOperation::Information(CollectionOperation::Add { index: 0, item });
        apply_plugin_operation(&mut program, &operation);
        assert_eq!(program.information.len(), 1);
        let undo = invert_plugin_operation(&program, &operation);
        apply_plugin_operation(&mut program, &undo);
        assert!(program.information.is_empty());
    }

    #[test]
    fn dispatches_infrastructure_add_and_invert() {
        let mut program = empty_plugin();
        let item = InfrastructureRequirement {
            header: EntityHeader::new(EntityId::new_serial("infrastructurerequirement28", "InfrastructureRequirement 28"), "InfrastructureRequirement 28"),
            system: String::new(),
            category: String::new(),
            capacity: QuantitySpec::default(),
            redundancy: None,
            distribution: Vec::new(),
            entry_points: Vec::new(),
            utility_source: None,
            standby_power: false,
            monitoring: Vec::new(),
            maintenance_access: Vec::new(),
            standards: Vec::new(),
            element_ids: Vec::new(),
            peak_demand: None,
            diversity_factor: None,
            future_expansion: Vec::new(),
            interface_requirements: Vec::new(),
            commissioning: Vec::new(),
            lifecycle_cost: None,
            owner_id: None,
        };
        let operation = ProgramOperation::Infrastructure(CollectionOperation::Add { index: 0, item });
        apply_plugin_operation(&mut program, &operation);
        assert_eq!(program.infrastructure.len(), 1);
        let undo = invert_plugin_operation(&program, &operation);
        apply_plugin_operation(&mut program, &undo);
        assert!(program.infrastructure.is_empty());
    }

    #[test]
    fn dispatches_issues_add_and_invert() {
        let mut program = empty_plugin();
        let item = Issue {
            header: EntityHeader::new(EntityId::new_serial("issue29", "Issue 29"), "Issue 29"),
            issue_type: String::new(),
            summary: TextField::default(),
            issue_description: TextField::default(),
            severity: IssueSeverity::Cosmetic,
            issue_priority: Priority::Mandatory,
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
        };
        let operation = ProgramOperation::Issues(CollectionOperation::Add { index: 0, item });
        apply_plugin_operation(&mut program, &operation);
        assert_eq!(program.issues.len(), 1);
        let undo = invert_plugin_operation(&program, &operation);
        apply_plugin_operation(&mut program, &undo);
        assert!(program.issues.is_empty());
    }

    #[test]
    fn dispatches_knowledge_add_and_invert() {
        let mut program = empty_plugin();
        let item = KnowledgeRecord {
            header: EntityHeader::new(EntityId::new_serial("knowledgerecord30", "KnowledgeRecord 30"), "KnowledgeRecord 30"),
            topic: String::new(),
            category: String::new(),
            summary: TextField::default(),
            content: TextField::default(),
            sources: Vec::new(),
            references: Vec::new(),
            lessons_learned: Vec::new(),
            best_practices: Vec::new(),
            applicable_sectors: Vec::new(),
            related_entity_kinds: Vec::new(),
            author_ids: Vec::new(),
            expertise_level: None,
            validation_status: ValidationStatus::Pending,
            last_reviewed: None,
            keywords: Vec::new(),
            attachments: Vec::new(),
            citations: Vec::new(),
            usage_count: 0,
        };
        let operation = ProgramOperation::Knowledge(CollectionOperation::Add { index: 0, item });
        apply_plugin_operation(&mut program, &operation);
        assert_eq!(program.knowledge.len(), 1);
        let undo = invert_plugin_operation(&program, &operation);
        apply_plugin_operation(&mut program, &undo);
        assert!(program.knowledge.is_empty());
    }

    #[test]
    fn dispatches_meetings_add_and_invert() {
        let mut program = empty_plugin();
        let item = MeetingRecord {
            header: EntityHeader::new(EntityId::new_serial("meetingrecord31", "MeetingRecord 31"), "MeetingRecord 31"),
            meeting_type: String::new(),
            scheduled_date: None,
            duration: None,
            location: None,
            chair_id: None,
            attendee_ids: Vec::new(),
            agenda_items: Vec::new(),
            minutes: None,
            action_items: Vec::new(),
            decisions_made: Vec::new(),
            document_refs: Vec::new(),
            follow_up_date: None,
            recording_ref: None,
            quorum_met: false,
            meeting_status: LifecycleStatus::Draft,
            workshop_id: None,
            stakeholder_ids: Vec::new(),
            requirement_ids: Vec::new(),
            issue_ids: Vec::new(),
            approval_ids: Vec::new(),
        };
        let operation = ProgramOperation::Meetings(CollectionOperation::Add { index: 0, item });
        apply_plugin_operation(&mut program, &operation);
        assert_eq!(program.meetings.len(), 1);
        let undo = invert_plugin_operation(&program, &operation);
        apply_plugin_operation(&mut program, &undo);
        assert!(program.meetings.is_empty());
    }

    #[test]
    fn dispatches_operations_add_and_invert() {
        let mut program = empty_plugin();
        let item = OperationalRequirement {
            header: EntityHeader::new(EntityId::new_serial("operationalrequirement32", "OperationalRequirement 32"), "OperationalRequirement 32"),
            operation: String::new(),
            service_level: None,
            operating_hours: None,
            staffing: QuantitySpec::default(),
            maintenance_interval: None,
            cleaning_regime: None,
            turnaround_time: None,
            redundancy: None,
            uptime_target: None,
            response_time: None,
            equipment_ids: Vec::new(),
            element_ids: Vec::new(),
            process_ids: Vec::new(),
            utilities: Vec::new(),
            waste_streams: Vec::new(),
            contingency_plan: Vec::new(),
            training_requirements: Vec::new(),
            sop_references: Vec::new(),
            kpi_targets: Vec::new(),
            owner_id: None,
            service_category: None,
            shift_pattern: None,
            sla_target: None,
            escalation_contact_id: None,
        };
        let operation = ProgramOperation::Operations(CollectionOperation::Add { index: 0, item });
        apply_plugin_operation(&mut program, &operation);
        assert_eq!(program.operations.len(), 1);
        let undo = invert_plugin_operation(&program, &operation);
        apply_plugin_operation(&mut program, &undo);
        assert!(program.operations.is_empty());
    }

    #[test]
    fn dispatches_options_add_and_invert() {
        let mut program = empty_plugin();
        let item = OptionEvaluation {
            header: EntityHeader::new(EntityId::new_serial("optionevaluation33", "OptionEvaluation 33"), "OptionEvaluation 33"),
            option_name: String::new(),
            option_description: TextField::default(),
            scenario_id: None,
            criteria_ids: Vec::new(),
            scores: Vec::new(),
            weighted_score: None,
            cost_estimate: None,
            schedule_estimate: None,
            risk_summary: Vec::new(),
            benefits: Vec::new(),
            drawbacks: Vec::new(),
            assumptions: Vec::new(),
            dependencies: Vec::new(),
            stakeholder_feedback: Vec::new(),
            recommendation: None,
            decision_id: None,
            evaluation_status: ValidationStatus::Pending,
            evaluator_ids: Vec::new(),
            evaluation_date: None,
        };
        let operation = ProgramOperation::Options(CollectionOperation::Add { index: 0, item });
        apply_plugin_operation(&mut program, &operation);
        assert_eq!(program.options.len(), 1);
        let undo = invert_plugin_operation(&program, &operation);
        apply_plugin_operation(&mut program, &undo);
        assert!(program.options.is_empty());
    }

    #[test]
    fn dispatches_organizational_add_and_invert() {
        let mut program = empty_plugin();
        let item = OrganizationalRequirement {
            header: EntityHeader::new(EntityId::new_serial("organizationalrequirement34", "OrganizationalRequirement 34"), "OrganizationalRequirement 34"),
            department: String::new(),
            reporting_line: None,
            headcount: QuantitySpec::default(),
            growth_plan_id: None,
            work_patterns: Vec::new(),
            collaboration_model: None,
            hierarchy_levels: Vec::new(),
            decision_making: Vec::new(),
            culture_notes: Vec::new(),
            change_readiness: None,
            union_considerations: Vec::new(),
            training_needs: Vec::new(),
            element_ids: Vec::new(),
            stakeholder_ids: Vec::new(),
            service_requirement_ids: Vec::new(),
            branding_requirements: Vec::new(),
            wellness_plugins: Vec::new(),
            diversity_goals: Vec::new(),
            owner_id: None,
        };
        let operation = ProgramOperation::Organizational(CollectionOperation::Add { index: 0, item });
        apply_plugin_operation(&mut program, &operation);
        assert_eq!(program.organizational.len(), 1);
        let undo = invert_plugin_operation(&program, &operation);
        apply_plugin_operation(&mut program, &undo);
        assert!(program.organizational.is_empty());
    }

    #[test]
    fn dispatches_performance_add_and_invert() {
        let mut program = empty_plugin();
        let item = PerformanceCriterion {
            header: EntityHeader::new(EntityId::new_serial("performancecriterion35", "PerformanceCriterion 35"), "PerformanceCriterion 35"),
            criterion: String::new(),
            metric: String::new(),
            target: None,
            unit: None,
            minimum: None,
            maximum: None,
            measurement_method: None,
            frequency: None,
            requirement_ids: Vec::new(),
            element_ids: Vec::new(),
            baseline: None,
            benchmark_ref: None,
            weight: None,
            data_source: None,
            reporting_cadence: None,
            owner_id: None,
            verification_plan: None,
            penalty_threshold: None,
            incentive_threshold: None,
        };
        let operation = ProgramOperation::Performance(CollectionOperation::Add { index: 0, item });
        apply_plugin_operation(&mut program, &operation);
        assert_eq!(program.performance.len(), 1);
        let undo = invert_plugin_operation(&program, &operation);
        apply_plugin_operation(&mut program, &undo);
        assert!(program.performance.is_empty());
    }

    #[test]
    fn dispatches_priorities_add_and_invert() {
        let mut program = empty_plugin();
        let item = PriorityRecord {
            header: EntityHeader::new(EntityId::new_serial("priorityrecord36", "PriorityRecord 36"), "PriorityRecord 36"),
            subject_id: EntityId::new_serial("base36", "base36"),
            subject_kind: String::new(),
            ranked_priority: Priority::Mandatory,
            rank: None,
            weight: None,
            rationale: None,
            decision_id: None,
            stakeholder_ids: Vec::new(),
            effective_from: None,
            effective_until: None,
            review_cycle: None,
            dependencies: Vec::new(),
            conflicts: Vec::new(),
            scoring_method: None,
            score: None,
            criteria: Vec::new(),
            approved_by: None,
            approval_date: None,
            ranking_notes: Vec::new(),
        };
        let operation = ProgramOperation::Priorities(CollectionOperation::Add { index: 0, item });
        apply_plugin_operation(&mut program, &operation);
        assert_eq!(program.priorities.len(), 1);
        let undo = invert_plugin_operation(&program, &operation);
        apply_plugin_operation(&mut program, &undo);
        assert!(program.priorities.is_empty());
    }

    #[test]
    fn dispatches_privacy_add_and_invert() {
        let mut program = empty_plugin();
        let item = PrivacyRequirement {
            header: EntityHeader::new(EntityId::new_serial("privacyrequirement37", "PrivacyRequirement 37"), "PrivacyRequirement 37"),
            privacy_kind: PrivacyKind::Public,
            privacy_type: String::new(),
            level: None,
            subject_ids: Vec::new(),
            element_ids: Vec::new(),
            visual_privacy: Vec::new(),
            acoustic_privacy: Vec::new(),
            data_privacy: Vec::new(),
            screening_required: false,
            enclosure_required: false,
            access_restrictions: Vec::new(),
            observation_risk: None,
            regulatory_basis: Vec::new(),
            cultural_considerations: Vec::new(),
            technology_controls: Vec::new(),
            signage: Vec::new(),
            monitoring_restrictions: Vec::new(),
            retention_policy: None,
            breach_response: Vec::new(),
            owner_id: None,
        };
        let operation = ProgramOperation::Privacy(CollectionOperation::Add { index: 0, item });
        apply_plugin_operation(&mut program, &operation);
        assert_eq!(program.privacy.len(), 1);
        let undo = invert_plugin_operation(&program, &operation);
        apply_plugin_operation(&mut program, &undo);
        assert!(program.privacy.is_empty());
    }

    #[test]
    fn dispatches_processes_add_and_invert() {
        let mut program = empty_plugin();
        let item = Process {
            header: EntityHeader::new(EntityId::new_serial("process38", "Process 38"), "Process 38"),
            code: String::new(),
            category: String::new(),
            trigger: None,
            inputs: Vec::new(),
            outputs: Vec::new(),
            steps: Vec::new(),
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
            regulatory_refs: Vec::new(),
            owner_id: None,
            workflow_type: None,
            handoff_points: Vec::new(),
            quality_gates: Vec::new(),
        };
        let operation = ProgramOperation::Processes(CollectionOperation::Add { index: 0, item });
        apply_plugin_operation(&mut program, &operation);
        assert_eq!(program.processes.len(), 1);
        let undo = invert_plugin_operation(&program, &operation);
        apply_plugin_operation(&mut program, &undo);
        assert!(program.processes.is_empty());
    }

    #[test]
    fn dispatches_quality_add_and_invert() {
        let mut program = empty_plugin();
        let item = QualityRecord {
            header: EntityHeader::new(EntityId::new_serial("qualityrecord39", "QualityRecord 39"), "QualityRecord 39"),
            quality_topic: String::new(),
            standard: None,
            target_level: None,
            inspection_points: Vec::new(),
            acceptance_criteria: Vec::new(),
            testing_requirements: Vec::new(),
            sample_rate: None,
            defect_categories: Vec::new(),
            corrective_action_process: Vec::new(),
            element_ids: Vec::new(),
            requirement_ids: Vec::new(),
            supplier_requirements: Vec::new(),
            documentation_requirements: Vec::new(),
            training_requirements: Vec::new(),
            audit_schedule: None,
            kpis: Vec::new(),
            owner_id: None,
            certification_targets: Vec::new(),
            continuous_improvement: Vec::new(),
        };
        let operation = ProgramOperation::Quality(CollectionOperation::Add { index: 0, item });
        apply_plugin_operation(&mut program, &operation);
        assert_eq!(program.quality.len(), 1);
        let undo = invert_plugin_operation(&program, &operation);
        apply_plugin_operation(&mut program, &undo);
        assert!(program.quality.is_empty());
    }

    #[test]
    fn dispatches_quantities_add_and_invert() {
        let mut program = empty_plugin();
        let item = QuantityRequirement {
            header: EntityHeader::new(EntityId::new_serial("quantityrequirement40", "QuantityRequirement 40"), "QuantityRequirement 40"),
            target_element_id: EntityId::new_serial("base40", "base40"),
            metric: String::new(),
            quantity: QuantitySpec::default(),
            basis: None,
            calculation_method: None,
            source: None,
            benchmark_ref: None,
            tolerance_percent: None,
            peak_factor: None,
            growth_factor: None,
            unit_cost: None,
            currency: None,
            verification_method: None,
            related_requirement_ids: Vec::new(),
            assumptions: Vec::new(),
            constraints: Vec::new(),
            schedule_phase: None,
            responsible_party: None,
            last_verified: None,
            variance_notes: Vec::new(),
        };
        let operation = ProgramOperation::Quantities(CollectionOperation::Add { index: 0, item });
        apply_plugin_operation(&mut program, &operation);
        assert_eq!(program.quantities.len(), 1);
        let undo = invert_plugin_operation(&program, &operation);
        apply_plugin_operation(&mut program, &undo);
        assert!(program.quantities.is_empty());
    }

    #[test]
    fn dispatches_regulatory_add_and_invert() {
        let mut program = empty_plugin();
        let item = RegulatoryRequirement {
            header: EntityHeader::new(EntityId::new_serial("regulatoryrequirement41", "RegulatoryRequirement 41"), "RegulatoryRequirement 41"),
            jurisdiction: String::new(),
            code: String::new(),
            clause: None,
            title: String::new(),
            requirement_text: TextField::default(),
            applicability: Vec::new(),
            element_ids: Vec::new(),
            compliance_method: None,
            evidence_required: Vec::new(),
            authority: None,
            effective_date: None,
            expiry_date: None,
            penalties: Vec::new(),
            exemptions: Vec::new(),
            related_requirement_ids: Vec::new(),
            interpretation_notes: Vec::new(),
            verification_status: ValidationStatus::Pending,
            consultant_refs: Vec::new(),
            update_source: None,
        };
        let operation = ProgramOperation::Regulatory(CollectionOperation::Add { index: 0, item });
        apply_plugin_operation(&mut program, &operation);
        assert_eq!(program.regulatory.len(), 1);
        let undo = invert_plugin_operation(&program, &operation);
        apply_plugin_operation(&mut program, &undo);
        assert!(program.regulatory.is_empty());
    }

    #[test]
    fn dispatches_relationships_add_and_invert() {
        let mut program = empty_plugin();
        let item = Relationship {
            header: EntityHeader::new(EntityId::new_serial("relationship42", "Relationship 42"), "Relationship 42"),
            source_id: EntityId::new_serial("base42", "base42"),
            target_id: EntityId::new_serial("base42", "base42"),
            kind: RelationshipKind::Contains,
            strength: None,
            directional: false,
            rationale: None,
            constraints: Vec::new(),
            conditions: Vec::new(),
            relationship_priority: Priority::Mandatory,
            valid_from: None,
            valid_until: None,
            evidence: Vec::new(),
            conflict_ids: Vec::new(),
            trace_links: Vec::new(),
            bidirectional: false,
            distance_constraint_m: None,
            capacity_constraint: None,
            regulatory_basis: Vec::new(),
            review_cycle: None,
            owner_id: None,
            proximity_requirement: None,
            compatibility_requirement: None,
            incompatibility_requirement: None,
            separation_requirements: Vec::new(),
        };
        let operation = ProgramOperation::Relationships(CollectionOperation::Add { index: 0, item });
        apply_plugin_operation(&mut program, &operation);
        assert_eq!(program.relationships.len(), 1);
        let undo = invert_plugin_operation(&program, &operation);
        apply_plugin_operation(&mut program, &undo);
        assert!(program.relationships.is_empty());
    }

    #[test]
    fn dispatches_reports_add_and_invert() {
        let mut program = empty_plugin();
        let item = ReportRecord {
            header: EntityHeader::new(EntityId::new_serial("reportrecord43", "ReportRecord 43"), "ReportRecord 43"),
            kind: ReportKind::ExecutiveSummary,
            title: String::new(),
            audience: Vec::new(),
            sections: Vec::new(),
            generated_at: None,
            generated_by: None,
            analysis_ids: Vec::new(),
            format: None,
            file_ref: None,
            distribution_list: Vec::new(),
            approval_status: ValidationStatus::Pending,
            approver_id: None,
            version: String::new(),
            template_id: None,
            parameters: Vec::new(),
            confidentiality: None,
            expiry_date: None,
            related_decision_ids: Vec::new(),
        };
        let operation = ProgramOperation::Reports(CollectionOperation::Add { index: 0, item });
        apply_plugin_operation(&mut program, &operation);
        assert_eq!(program.reports.len(), 1);
        let undo = invert_plugin_operation(&program, &operation);
        apply_plugin_operation(&mut program, &undo);
        assert!(program.reports.is_empty());
    }

    #[test]
    fn dispatches_requirements_add_and_invert() {
        let mut program = empty_plugin();
        let item = Requirement {
            header: EntityHeader::new(EntityId::new_serial("requirement44", "Requirement 44"), "Requirement 44"),
            code: String::new(),
            kind: RequirementKind::Functional,
            statement: TextField::default(),
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
        };
        let operation = ProgramOperation::Requirements(CollectionOperation::Add { index: 0, item });
        apply_plugin_operation(&mut program, &operation);
        assert_eq!(program.requirements.len(), 1);
        let undo = invert_plugin_operation(&program, &operation);
        apply_plugin_operation(&mut program, &undo);
        assert!(program.requirements.is_empty());
    }

    #[test]
    fn dispatches_resilience_add_and_invert() {
        let mut program = empty_plugin();
        let item = ResilienceRequirement {
            header: EntityHeader::new(EntityId::new_serial("resiliencerequirement45", "ResilienceRequirement 45"), "ResilienceRequirement 45"),
            hazard: String::new(),
            risk_level: RiskLevel::Negligible,
            scenario: None,
            recovery_time: None,
            recovery_point: None,
            redundancy: Vec::new(),
            hardening_measures: Vec::new(),
            backup_systems: Vec::new(),
            alternate_sites: Vec::new(),
            supply_chain: Vec::new(),
            communication_plan: Vec::new(),
            drill_requirements: Vec::new(),
            element_ids: Vec::new(),
            infrastructure_ids: Vec::new(),
            standards: Vec::new(),
            insurance_implications: Vec::new(),
            climate_adaptation: Vec::new(),
            owner_id: None,
            verification_plan: None,
        };
        let operation = ProgramOperation::Resilience(CollectionOperation::Add { index: 0, item });
        apply_plugin_operation(&mut program, &operation);
        assert_eq!(program.resilience.len(), 1);
        let undo = invert_plugin_operation(&program, &operation);
        apply_plugin_operation(&mut program, &undo);
        assert!(program.resilience.is_empty());
    }

    #[test]
    fn dispatches_resources_add_and_invert() {
        let mut program = empty_plugin();
        let item = Resource {
            header: EntityHeader::new(EntityId::new_serial("resource46", "Resource 46"), "Resource 46"),
            code: String::new(),
            category: String::new(),
            resource_type: String::new(),
            quantity: QuantitySpec::default(),
            mobility: None,
            sharing_model: None,
            allocation: None,
            element_ids: Vec::new(),
            activity_ids: Vec::new(),
            user_profile_ids: Vec::new(),
            storage_requirement_id: None,
            durability: None,
            cleaning_requirements: Vec::new(),
            replacement_cycle: None,
            cost_per_unit: None,
            supplier: None,
            standards: Vec::new(),
            ergonomic_notes: Vec::new(),
            customization: Vec::new(),
            disposal_notes: Vec::new(),
            furniture_class: None,
            ergonomics_rating: None,
            sharing_ratio: None,
        };
        let operation = ProgramOperation::Resources(CollectionOperation::Add { index: 0, item });
        apply_plugin_operation(&mut program, &operation);
        assert_eq!(program.resources.len(), 1);
        let undo = invert_plugin_operation(&program, &operation);
        apply_plugin_operation(&mut program, &undo);
        assert!(program.resources.is_empty());
    }

    #[test]
    fn dispatches_risks_add_and_invert() {
        let mut program = empty_plugin();
        let item = Risk {
            header: EntityHeader::new(EntityId::new_serial("risk47", "Risk 47"), "Risk 47"),
            risk_statement: TextField::default(),
            category: String::new(),
            probability: RiskLevel::Negligible,
            impact: RiskLevel::Negligible,
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
        let operation = ProgramOperation::Risks(CollectionOperation::Add { index: 0, item });
        apply_plugin_operation(&mut program, &operation);
        assert_eq!(program.risks.len(), 1);
        let undo = invert_plugin_operation(&program, &operation);
        apply_plugin_operation(&mut program, &undo);
        assert!(program.risks.is_empty());
    }

    #[test]
    fn dispatches_safety_add_and_invert() {
        let mut program = empty_plugin();
        let item = SafetyRequirement {
            header: EntityHeader::new(EntityId::new_serial("safetyrequirement48", "SafetyRequirement 48"), "SafetyRequirement 48"),
            safety_domain: SafetyDomain::LifeSafety,
            hazard: String::new(),
            risk_level: RiskLevel::Negligible,
            affected_element_ids: Vec::new(),
            affected_user_ids: Vec::new(),
            mitigation_measures: Vec::new(),
            ppe_requirements: Vec::new(),
            emergency_procedures: Vec::new(),
            evacuation_requirements: Vec::new(),
            fire_protection: Vec::new(),
            structural_safety: Vec::new(),
            slip_trip_fall: Vec::new(),
            chemical_safety: Vec::new(),
            electrical_safety: Vec::new(),
            machinery_safety: Vec::new(),
            standards: Vec::new(),
            inspection_frequency: None,
            training_requirements: Vec::new(),
            incident_reporting: Vec::new(),
            residual_risk: None,
        };
        let operation = ProgramOperation::Safety(CollectionOperation::Add { index: 0, item });
        apply_plugin_operation(&mut program, &operation);
        assert_eq!(program.safety.len(), 1);
        let undo = invert_plugin_operation(&program, &operation);
        apply_plugin_operation(&mut program, &undo);
        assert!(program.safety.is_empty());
    }

    #[test]
    fn dispatches_scenarios_add_and_invert() {
        let mut program = empty_plugin();
        let item = Scenario {
            header: EntityHeader::new(EntityId::new_serial("scenario49", "Scenario 49"), "Scenario 49"),
            code: String::new(),
            hypothesis: TextField::default(),
            assumptions: Vec::new(),
            variables: Vec::new(),
            element_ids: Vec::new(),
            requirement_ids: Vec::new(),
            growth_plan_id: None,
            probability: None,
            impact_summary: None,
            cost_delta: None,
            area_delta: None,
            headcount_delta: None,
            schedule_delta: None,
            risk_ids: Vec::new(),
            option_ids: Vec::new(),
            baseline: false,
            preferred: false,
            analysis_ids: Vec::new(),
            owner_id: None,
        };
        let operation = ProgramOperation::Scenarios(CollectionOperation::Add { index: 0, item });
        apply_plugin_operation(&mut program, &operation);
        assert_eq!(program.scenarios.len(), 1);
        let undo = invert_plugin_operation(&program, &operation);
        apply_plugin_operation(&mut program, &undo);
        assert!(program.scenarios.is_empty());
    }

    #[test]
    fn dispatches_schedules_add_and_invert() {
        let mut program = empty_plugin();
        let item = ScheduleRequirement {
            header: EntityHeader::new(EntityId::new_serial("schedulerequirement50", "ScheduleRequirement 50"), "ScheduleRequirement 50"),
            milestone: String::new(),
            phase: DeliveryPhase::Concept,
            start_date: None,
            end_date: None,
            duration: None,
            dependencies: Vec::new(),
            predecessors: Vec::new(),
            successors: Vec::new(),
            critical: false,
            float_days: None,
            resource_requirements: Vec::new(),
            occupancy_impact: Vec::new(),
            phasing_strategy: None,
            decant_requirements: Vec::new(),
            commissioning_window: None,
            stakeholder_ids: Vec::new(),
            risk_ids: Vec::new(),
            contingency_days: None,
            reporting_cadence: None,
            owner_id: None,
        };
        let operation = ProgramOperation::Schedules(CollectionOperation::Add { index: 0, item });
        apply_plugin_operation(&mut program, &operation);
        assert_eq!(program.schedules.len(), 1);
        let undo = invert_plugin_operation(&program, &operation);
        apply_plugin_operation(&mut program, &undo);
        assert!(program.schedules.is_empty());
    }

    #[test]
    fn dispatches_search_filters_add_and_invert() {
        let mut program = empty_plugin();
        let item = SearchFilter {
            header: EntityHeader::new(EntityId::new_serial("searchfilter51", "SearchFilter 51"), "SearchFilter 51"),
            filter_name: String::new(),
            filter_description: None,
            keywords: Vec::new(),
            categories: Vec::new(),
            owner_ids: Vec::new(),
            statuses: Vec::new(),
            priorities: Vec::new(),
            sources: Vec::new(),
            date_from: None,
            date_to: None,
            entity_kinds: Vec::new(),
            tag_filters: Vec::new(),
            sort_field: None,
            sort_direction: None,
            is_public: false,
            created_by: None,
            last_used: None,
            use_count: 0,
            pinned: false,
        };
        let operation = ProgramOperation::SearchFilters(CollectionOperation::Add { index: 0, item });
        apply_plugin_operation(&mut program, &operation);
        assert_eq!(program.search_filters.len(), 1);
        let undo = invert_plugin_operation(&program, &operation);
        apply_plugin_operation(&mut program, &undo);
        assert!(program.search_filters.is_empty());
    }

    #[test]
    fn dispatches_security_add_and_invert() {
        let mut program = empty_plugin();
        let item = SecurityRequirement {
            header: EntityHeader::new(EntityId::new_serial("securityrequirement52", "SecurityRequirement 52"), "SecurityRequirement 52"),
            control_kind: SecurityControlKind::AccessControl,
            threat: String::new(),
            risk_level: RiskLevel::Negligible,
            asset_ids: Vec::new(),
            zone_ids: Vec::new(),
            access_level: AccessLevel::Public,
            perimeter_controls: Vec::new(),
            surveillance: Vec::new(),
            intrusion_detection: Vec::new(),
            cybersecurity: Vec::new(),
            screening: Vec::new(),
            visitor_management: Vec::new(),
            key_management: Vec::new(),
            standards: Vec::new(),
            response_procedures: Vec::new(),
            drill_frequency: None,
            liaison_contacts: Vec::new(),
            classified_level: None,
            redundancy: Vec::new(),
            audit_requirements: Vec::new(),
        };
        let operation = ProgramOperation::Security(CollectionOperation::Add { index: 0, item });
        apply_plugin_operation(&mut program, &operation);
        assert_eq!(program.security.len(), 1);
        let undo = invert_plugin_operation(&program, &operation);
        apply_plugin_operation(&mut program, &undo);
        assert!(program.security.is_empty());
    }

    #[test]
    fn dispatches_services_add_and_invert() {
        let mut program = empty_plugin();
        let item = ServiceRequirement {
            header: EntityHeader::new(EntityId::new_serial("servicerequirement53", "ServiceRequirement 53"), "ServiceRequirement 53"),
            service_name: String::new(),
            service_type: String::new(),
            provider: None,
            service_level: None,
            operating_hours: None,
            capacity: QuantitySpec::default(),
            response_time: None,
            queue_management: Vec::new(),
            customer_profiles: Vec::new(),
            element_ids: Vec::new(),
            equipment_ids: Vec::new(),
            staffing: QuantitySpec::default(),
            quality_metrics: Vec::new(),
            cost_model: None,
            contract_refs: Vec::new(),
            dependencies: Vec::new(),
            failure_impact: None,
            backup_service: Vec::new(),
            feedback_channels: Vec::new(),
        };
        let operation = ProgramOperation::Services(CollectionOperation::Add { index: 0, item });
        apply_plugin_operation(&mut program, &operation);
        assert_eq!(program.services.len(), 1);
        let undo = invert_plugin_operation(&program, &operation);
        apply_plugin_operation(&mut program, &undo);
        assert!(program.services.is_empty());
    }

    #[test]
    fn dispatches_site_context_add_and_invert() {
        let mut program = empty_plugin();
        let item = SiteContext {
            header: EntityHeader::new(EntityId::new_serial("sitecontext54", "SiteContext 54"), "SiteContext 54"),
            site_name: String::new(),
            address: None,
            latitude: None,
            longitude: None,
            elevation_m: None,
            climate_zone: None,
            seismic_zone: None,
            flood_risk: None,
            soil_conditions: Vec::new(),
            utilities_available: Vec::new(),
            access_roads: Vec::new(),
            public_transit: Vec::new(),
            neighbors: Vec::new(),
            views: Vec::new(),
            noise_sources: Vec::new(),
            environmental_constraints: Vec::new(),
            heritage_constraints: Vec::new(),
            zoning: None,
            max_height_m: None,
            max_coverage: None,
        };
        let operation = ProgramOperation::SiteContext(CollectionOperation::Add { index: 0, item });
        apply_plugin_operation(&mut program, &operation);
        assert_eq!(program.site_context.len(), 1);
        let undo = invert_plugin_operation(&program, &operation);
        apply_plugin_operation(&mut program, &undo);
        assert!(program.site_context.is_empty());
    }

    #[test]
    fn dispatches_stakeholders_add_and_invert() {
        let mut program = empty_plugin();
        let item = Stakeholder {
            header: EntityHeader::new(EntityId::new_serial("stakeholder55", "Stakeholder 55"), "Stakeholder 55"),
            role: String::new(),
            organization: String::new(),
            department: None,
            contact_email: None,
            contact_phone: None,
            influence: InfluenceLevel::Low,
            interest: InfluenceLevel::Low,
            engagement: EngagementLevel::Unaware,
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
        };
        let operation = ProgramOperation::Stakeholders(CollectionOperation::Add { index: 0, item });
        apply_plugin_operation(&mut program, &operation);
        assert_eq!(program.stakeholders.len(), 1);
        let undo = invert_plugin_operation(&program, &operation);
        apply_plugin_operation(&mut program, &undo);
        assert!(program.stakeholders.is_empty());
    }

    #[test]
    fn dispatches_status_records_add_and_invert() {
        let mut program = empty_plugin();
        let item = StatusRecord {
            header: EntityHeader::new(EntityId::new_serial("statusrecord56", "StatusRecord 56"), "StatusRecord 56"),
            subject_id: EntityId::new_serial("base56", "base56"),
            subject_kind: String::new(),
            record_status: LifecycleStatus::Draft,
            previous_status: None,
            changed_by: None,
            changed_at: None,
            reason: None,
            blockers: Vec::new(),
            next_actions: Vec::new(),
            due_date: None,
            progress_percent: None,
            health: None,
            escalation_level: None,
            related_issue_ids: Vec::new(),
            related_risk_ids: Vec::new(),
            milestone_id: None,
            reporting_period: None,
            status_notes: Vec::new(),
        };
        let operation = ProgramOperation::StatusRecords(CollectionOperation::Add { index: 0, item });
        apply_plugin_operation(&mut program, &operation);
        assert_eq!(program.status_records.len(), 1);
        let undo = invert_plugin_operation(&program, &operation);
        apply_plugin_operation(&mut program, &undo);
        assert!(program.status_records.is_empty());
    }

    #[test]
    fn dispatches_storage_add_and_invert() {
        let mut program = empty_plugin();
        let item = StorageRequirement {
            header: EntityHeader::new(EntityId::new_serial("storagerequirement57", "StorageRequirement 57"), "StorageRequirement 57"),
            stored_item: String::new(),
            storage_class: StorageClass::General,
            quantity: QuantitySpec::default(),
            volume_m3: None,
            weight_kg: None,
            temperature_range: None,
            humidity_range: None,
            security_level: AccessLevel::Public,
            hazard_class: None,
            retention_period: None,
            access_frequency: None,
            element_ids: Vec::new(),
            equipment_ids: Vec::new(),
            handling_equipment: Vec::new(),
            fire_protection: Vec::new(),
            ventilation: None,
            organization_system: None,
            growth_allowance: None,
            regulatory_refs: Vec::new(),
            owner_id: None,
        };
        let operation = ProgramOperation::Storage(CollectionOperation::Add { index: 0, item });
        apply_plugin_operation(&mut program, &operation);
        assert_eq!(program.storage.len(), 1);
        let undo = invert_plugin_operation(&program, &operation);
        apply_plugin_operation(&mut program, &undo);
        assert!(program.storage.is_empty());
    }

    #[test]
    fn dispatches_surveys_add_and_invert() {
        let mut program = empty_plugin();
        let item = Survey {
            header: EntityHeader::new(EntityId::new_serial("survey58", "Survey 58"), "Survey 58"),
            survey_type: String::new(),
            title: String::new(),
            objectives: Vec::new(),
            questions: Vec::new(),
            target_audience: Vec::new(),
            distribution_channels: Vec::new(),
            launch_date: None,
            close_date: None,
            response_count: 0,
            response_rate: None,
            findings: Vec::new(),
            themes: Vec::new(),
            recommendations: Vec::new(),
            confidentiality: None,
            consent_process: Vec::new(),
            analysis_id: None,
            workshop_id: None,
            owner_id: None,
            survey_status: LifecycleStatus::Draft,
        };
        let operation = ProgramOperation::Surveys(CollectionOperation::Add { index: 0, item });
        apply_plugin_operation(&mut program, &operation);
        assert_eq!(program.surveys.len(), 1);
        let undo = invert_plugin_operation(&program, &operation);
        apply_plugin_operation(&mut program, &undo);
        assert!(program.surveys.is_empty());
    }

    #[test]
    fn dispatches_sustainability_add_and_invert() {
        let mut program = empty_plugin();
        let item = SustainabilityRequirement {
            header: EntityHeader::new(EntityId::new_serial("sustainabilityrequirement59", "SustainabilityRequirement 59"), "SustainabilityRequirement 59"),
            topic: String::new(),
            target: None,
            metric: None,
            baseline: None,
            target_value: None,
            unit: None,
            certification: Vec::new(),
            standards: Vec::new(),
            element_ids: Vec::new(),
            strategies: Vec::new(),
            materials_preferences: Vec::new(),
            energy_strategy: Vec::new(),
            water_strategy: Vec::new(),
            waste_strategy: Vec::new(),
            biodiversity: Vec::new(),
            embodied_carbon: None,
            operational_carbon: None,
            reporting_requirements: Vec::new(),
            verification_plan: None,
            owner_id: None,
        };
        let operation = ProgramOperation::Sustainability(CollectionOperation::Add { index: 0, item });
        apply_plugin_operation(&mut program, &operation);
        assert_eq!(program.sustainability.len(), 1);
        let undo = invert_plugin_operation(&program, &operation);
        apply_plugin_operation(&mut program, &undo);
        assert!(program.sustainability.is_empty());
    }

    #[test]
    fn dispatches_templates_add_and_invert() {
        let mut program = empty_plugin();
        let item = TemplateRecord {
            header: EntityHeader::new(EntityId::new_serial("templaterecord60", "TemplateRecord 60"), "TemplateRecord 60"),
            template_type: String::new(),
            sector: None,
            project_type: None,
            version: String::new(),
            content_ref: None,
            entity_kinds: Vec::new(),
            default_fields: Vec::new(),
            checklists: Vec::new(),
            standards: Vec::new(),
            applicability: Vec::new(),
            author_id: None,
            approval_status: ValidationStatus::Pending,
            usage_count: 0,
            last_applied: None,
            customization_notes: Vec::new(),
            related_knowledge_ids: Vec::new(),
            benchmark_ids: Vec::new(),
            license: None,
            source_organization: None,
        };
        let operation = ProgramOperation::Templates(CollectionOperation::Add { index: 0, item });
        apply_plugin_operation(&mut program, &operation);
        assert_eq!(program.templates.len(), 1);
        let undo = invert_plugin_operation(&program, &operation);
        apply_plugin_operation(&mut program, &undo);
        assert!(program.templates.is_empty());
    }

    #[test]
    fn dispatches_users_add_and_invert() {
        let mut program = empty_plugin();
        let item = UserProfile {
            header: EntityHeader::new(EntityId::new_serial("userprofile61", "UserProfile 61"), "UserProfile 61"),
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
        };
        let operation = ProgramOperation::Users(CollectionOperation::Add { index: 0, item });
        apply_plugin_operation(&mut program, &operation);
        assert_eq!(program.users.len(), 1);
        let undo = invert_plugin_operation(&program, &operation);
        apply_plugin_operation(&mut program, &undo);
        assert!(program.users.is_empty());
    }

    #[test]
    fn dispatches_validations_add_and_invert() {
        let mut program = empty_plugin();
        let item = ValidationRecord {
            header: EntityHeader::new(EntityId::new_serial("validationrecord62", "ValidationRecord 62"), "ValidationRecord 62"),
            subject_id: EntityId::new_serial("base62", "base62"),
            subject_kind: String::new(),
            validation_type: String::new(),
            method: None,
            criteria: Vec::new(),
            result: ValidationStatus::Pending,
            evidence: Vec::new(),
            validator_ids: Vec::new(),
            validation_date: None,
            next_review_date: None,
            findings: Vec::new(),
            non_conformities: Vec::new(),
            corrective_actions: Vec::new(),
            waivers: Vec::new(),
            standards: Vec::new(),
            trace_links: Vec::new(),
            report_id: None,
            confidence_level: None,
            validation_notes: Vec::new(),
        };
        let operation = ProgramOperation::Validations(CollectionOperation::Add { index: 0, item });
        apply_plugin_operation(&mut program, &operation);
        assert_eq!(program.validations.len(), 1);
        let undo = invert_plugin_operation(&program, &operation);
        apply_plugin_operation(&mut program, &undo);
        assert!(program.validations.is_empty());
    }

    #[test]
    fn dispatches_wayfinding_add_and_invert() {
        let mut program = empty_plugin();
        let item = WayfindingRequirement {
            header: EntityHeader::new(EntityId::new_serial("wayfindingrequirement63", "WayfindingRequirement 63"), "WayfindingRequirement 63"),
            user_profile_ids: Vec::new(),
            element_ids: Vec::new(),
            destination_types: Vec::new(),
            signage_types: Vec::new(),
            languages: Vec::new(),
            tactile_required: false,
            audio_required: false,
            digital_wayfinding: false,
            landmark_strategy: Vec::new(),
            color_coding: Vec::new(),
            symbol_standards: Vec::new(),
            decision_points: Vec::new(),
            maximum_signage_distance_m: None,
            lighting_requirements: Vec::new(),
            maintenance_plan: None,
            emergency_egress: Vec::new(),
            visitor_journey: Vec::new(),
            staff_journey: Vec::new(),
            brand_integration: Vec::new(),
        };
        let operation = ProgramOperation::Wayfinding(CollectionOperation::Add { index: 0, item });
        apply_plugin_operation(&mut program, &operation);
        assert_eq!(program.wayfinding.len(), 1);
        let undo = invert_plugin_operation(&program, &operation);
        apply_plugin_operation(&mut program, &undo);
        assert!(program.wayfinding.is_empty());
    }

    #[test]
    fn dispatches_workshops_add_and_invert() {
        let mut program = empty_plugin();
        let item = Workshop {
            header: EntityHeader::new(EntityId::new_serial("workshop64", "Workshop 64"), "Workshop 64"),
            workshop_type: String::new(),
            objectives: Vec::new(),
            agenda: Vec::new(),
            facilitator_id: None,
            participants: Vec::new(),
            scheduled_start: None,
            scheduled_end: None,
            location: None,
            materials: Vec::new(),
            methods: Vec::new(),
            outputs: Vec::new(),
            decisions: Vec::new(),
            issues: Vec::new(),
            follow_up_actions: Vec::new(),
            feedback: Vec::new(),
            recording_ref: None,
            budget: None,
            workshop_status: LifecycleStatus::Draft,
            survey_ids: Vec::new(),
        };
        let operation = ProgramOperation::Workshops(CollectionOperation::Add { index: 0, item });
        apply_plugin_operation(&mut program, &operation);
        assert_eq!(program.workshops.len(), 1);
        let undo = invert_plugin_operation(&program, &operation);
        apply_plugin_operation(&mut program, &undo);
        assert!(program.workshops.is_empty());
    }

    // #region 🔖️OpText
    #[test]
    fn update_meta_op_text_round_trips() {
        store::test_support::assert_op_line_round_trip(&ProgramOperation::UpdateMeta { patch: ProgramMetaPatch { title: Some("Clinic".into()), ..Default::default() } });
    }

    #[test]
    fn add_stakeholder_op_text_round_trips() {
        let stakeholder = Stakeholder {
            header: EntityHeader::new(EntityId::new_serial("stakeholder", "Nurse Lead"), "Nurse Lead"),
            role: "Clinical".into(),
            organization: "Sample Health".into(),
            department: None,
            contact_email: None,
            contact_phone: None,
            influence: InfluenceLevel::Medium,
            interest: InfluenceLevel::High,
            engagement: EngagementLevel::Supportive,
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
            stakeholder_type: "Internal".into(),
            influence_strategy: None,
            communication_channels: Vec::new(),
            success_metrics: Vec::new(),
        };
        let operation = ProgramOperation::Stakeholders(CollectionOperation::Add { index: 0, item: stakeholder });
        store::test_support::assert_op_line_round_trip(&operation);
    }

    #[test]
    fn remove_and_move_op_text_round_trip() {
        store::test_support::assert_op_line_round_trip(&ProgramOperation::Stakeholders(CollectionOperation::Remove { id: EntityId::new_serial("stakeholder", "stakeholder") }));
        store::test_support::assert_op_line_round_trip(&ProgramOperation::Stakeholders(CollectionOperation::Move { id: EntityId::new_serial("stakeholder", "stakeholder"), to_index: 2 }));
    }

    #[test]
    fn set_adjacency_and_clear_adjacency_op_text_round_trip() {
        let program = sample_plugin();
        let adjacency = program.adjacencies[0].clone();
        store::test_support::assert_op_line_round_trip(&ProgramOperation::SetAdjacency { adjacency });
        store::test_support::assert_op_line_round_trip(&ProgramOperation::ClearAdjacency { id: EntityId::new_serial("adjacency", "adjacency") });
    }

    #[test]
    fn set_plugin_op_text_round_trips() {
        store::test_support::assert_op_line_round_trip(&ProgramOperation::SetProgram { program: Box::new(sample_plugin()) });
    }

    #[test]
    fn op_text_print_op_is_always_one_line() {
        let printed = ProgramOperation::UpdateMeta { patch: ProgramMetaPatch { title: Some("multi\nline\ntitle".into()), ..Default::default() } }.print_op();
        assert!(!printed.contains('\n'), "print_op must be one line, got: {printed:?}");
        let parsed = <ProgramOperation as OpText>::parse_op(&printed).expect("parse_op");
        assert_eq!(parsed, ProgramOperation::UpdateMeta { patch: ProgramMetaPatch { title: Some("multi\nline\ntitle".into()), ..Default::default() } });
    }
    // #endregion 🔖️OpText
}