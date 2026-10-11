//! 🧬️ ProgramSnapshot snapshot schema — artifact-lane fields only.

use crate::kernel::*;
use crate::registers::*;
use framework_schema::ArtifactSchema;


//#region 🔖️Snapshot
/// 📸️ Persisted architect program snapshot (persistent fields of the artifact).
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, ArtifactSchema, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(extension = "architect", layout = "lines")]
#[artifact_schema(id = "s.architect.program")]
pub struct ProgramSnapshot {
    #[state(artifact)]
    pub schema: String,
    #[dsl(block)]
    #[state(artifact)]
    pub meta: ProgramMeta,
    #[dsl(block)]
    #[state(artifact)]
    pub project: ProjectDefinition,
    #[dsl(table)]
    #[state(artifact)]
    pub stakeholders: Vec<Stakeholder>,
    #[dsl(table)]
    #[state(artifact)]
    pub users: Vec<UserProfile>,
    #[dsl(table)]
    #[state(artifact)]
    pub activities: Vec<Activity>,
    #[dsl(table)]
    #[state(artifact)]
    pub functions: Vec<Function>,
    #[dsl(table)]
    #[state(artifact)]
    pub elements: Vec<ProgramElement>,
    #[dsl(table)]
    #[state(artifact)]
    pub quantities: Vec<QuantityRequirement>,
    #[dsl(table)]
    #[state(artifact)]
    pub relationships: Vec<Relationship>,
    #[dsl(table)]
    #[state(artifact)]
    pub adjacencies: Vec<Adjacency>,
    #[dsl(table)]
    #[state(artifact)]
    pub processes: Vec<Process>,
    #[dsl(table)]
    #[state(artifact)]
    pub flows: Vec<FlowRequirement>,
    #[dsl(table)]
    #[state(artifact)]
    pub access_rules: Vec<AccessRule>,
    #[dsl(table)]
    #[state(artifact)]
    pub operations: Vec<OperationalRequirement>,
    #[dsl(table)]
    #[state(artifact)]
    pub equipment: Vec<Equipment>,
    #[dsl(table)]
    #[state(artifact)]
    pub resources: Vec<Resource>,
    #[dsl(table)]
    #[state(artifact)]
    pub storage: Vec<StorageRequirement>,
    #[dsl(table)]
    #[state(artifact)]
    pub environmental: Vec<EnvironmentalRequirement>,
    #[dsl(table)]
    #[state(artifact)]
    pub human_factors: Vec<HumanFactorRequirement>,
    #[dsl(table)]
    #[state(artifact)]
    pub accessibility: Vec<AccessibilityRequirement>,
    #[dsl(table)]
    #[state(artifact)]
    pub privacy: Vec<PrivacyRequirement>,
    #[dsl(table)]
    #[state(artifact)]
    pub safety: Vec<SafetyRequirement>,
    #[dsl(table)]
    #[state(artifact)]
    pub security: Vec<SecurityRequirement>,
    #[dsl(table)]
    #[state(artifact)]
    pub regulatory: Vec<RegulatoryRequirement>,
    #[dsl(table)]
    #[state(artifact)]
    pub site_context: Vec<SiteContext>,
    #[dsl(table)]
    #[state(artifact)]
    pub organizational: Vec<OrganizationalRequirement>,
    #[dsl(table)]
    #[state(artifact)]
    pub services: Vec<ServiceRequirement>,
    #[dsl(table)]
    #[state(artifact)]
    pub infrastructure: Vec<InfrastructureRequirement>,
    #[dsl(table)]
    #[state(artifact)]
    pub information: Vec<InformationRequirement>,
    #[dsl(table)]
    #[state(artifact)]
    pub communication: Vec<CommunicationRequirement>,
    #[dsl(table)]
    #[state(artifact)]
    pub wayfinding: Vec<WayfindingRequirement>,
    #[dsl(table)]
    #[state(artifact)]
    pub schedules: Vec<ScheduleRequirement>,
    #[dsl(table)]
    #[state(artifact)]
    pub flexibility: Vec<FlexibilityRequirement>,
    #[dsl(table)]
    #[state(artifact)]
    pub growth: Vec<GrowthPlan>,
    #[dsl(table)]
    #[state(artifact)]
    pub sustainability: Vec<SustainabilityRequirement>,
    #[dsl(table)]
    #[state(artifact)]
    pub resilience: Vec<ResilienceRequirement>,
    #[dsl(table)]
    #[state(artifact)]
    pub costs: Vec<CostRequirement>,
    #[dsl(table)]
    #[state(artifact)]
    pub delivery: Vec<DeliveryConstraint>,
    #[dsl(table)]
    #[state(artifact)]
    pub risks: Vec<Risk>,
    #[dsl(table)]
    #[state(artifact)]
    pub conflicts: Vec<Conflict>,
    #[dsl(table)]
    #[state(artifact)]
    pub requirements: Vec<Requirement>,
    #[dsl(table)]
    #[state(artifact)]
    pub priorities: Vec<PriorityRecord>,
    #[dsl(table)]
    #[state(artifact)]
    pub scenarios: Vec<Scenario>,
    #[dsl(table)]
    #[state(artifact)]
    pub options: Vec<OptionEvaluation>,
    #[dsl(table)]
    #[state(artifact)]
    pub decisions: Vec<Decision>,
    #[dsl(table)]
    #[state(artifact)]
    pub validations: Vec<ValidationRecord>,
    #[dsl(table)]
    #[state(artifact)]
    pub performance: Vec<PerformanceCriterion>,
    #[dsl(table)]
    #[state(artifact)]
    pub quality: Vec<QualityRecord>,
    #[dsl(table)]
    #[state(artifact)]
    pub artifacts: Vec<ArtifactRecord>,
    #[dsl(table)]
    #[state(artifact)]
    pub assumptions: Vec<Assumption>,
    #[dsl(table)]
    #[state(artifact)]
    pub constraints: Vec<ConstraintRecord>,
    #[dsl(table)]
    #[state(artifact)]
    pub compliance_records: Vec<ComplianceRecord>,
    #[dsl(table)]
    #[state(artifact)]
    pub approvals: Vec<ApprovalRecord>,
    #[dsl(table)]
    #[state(artifact)]
    pub meetings: Vec<MeetingRecord>,
    #[dsl(table)]
    #[state(artifact)]
    pub changes: Vec<ChangeRecord>,
    #[dsl(table)]
    #[state(artifact)]
    pub collaboration: Vec<CollaborationRecord>,
    #[dsl(table)]
    #[state(artifact)]
    pub analyses: Vec<AnalysisRecord>,
    #[dsl(table)]
    #[state(artifact)]
    pub reports: Vec<ReportRecord>,
    #[dsl(table)]
    #[state(artifact)]
    pub search_filters: Vec<SearchFilter>,
    #[dsl(table)]
    #[state(artifact)]
    pub status_records: Vec<StatusRecord>,
    #[dsl(table)]
    #[state(artifact)]
    pub workshops: Vec<Workshop>,
    #[dsl(table)]
    #[state(artifact)]
    pub surveys: Vec<Survey>,
    #[dsl(table)]
    #[state(artifact)]
    pub issues: Vec<Issue>,
    #[dsl(table)]
    #[state(artifact)]
    pub audit_events: Vec<AuditEvent>,
    #[dsl(table)]
    #[state(artifact)]
    pub templates: Vec<TemplateRecord>,
    /// 📚️ The rows the composed `knowledge` child is derived from — the PERSISTED payload of that
    /// child slot (`🏭️process`'s `stock_payload`/`step_payloads` pattern). A composed child's content
    /// never travels inside `store::ArtifactChild`, and the react shell answers `Effect::LoadDocument`
    /// with an EMPTY member roster, so `crate::genesis_program_child_pack` can only derive the table
    /// from a field the parent's own pack/DSL round-trips.
    #[dsl(table)]
    #[state(artifact)]
    pub knowledge_payload: Vec<KnowledgeRecord>,
    #[dsl(block)]
    #[child(kind = "s.stdio.semio")]
    #[state(artifact)]
    pub knowledge: crate::ProgramKnowledgeChild,
    /// 🏁️ The rows the composed `benchmarks` child is derived from — see
    /// [`ProgramSnapshot::knowledge_payload`].
    #[dsl(table)]
    #[state(artifact)]
    pub benchmarks_payload: Vec<BenchmarkRecord>,
    #[dsl(block)]
    #[child(kind = "s.stdio.semio")]
    #[state(artifact)]
    pub benchmarks: crate::ProgramBenchmarksChild,
    #[dsl(table)]
    #[state(artifact)]
    pub traces: Vec<TraceLink>,
    #[dsl(block)]
    #[state(artifact)]
    pub governance: Governance,
}

impl Default for ProgramSnapshot {
    fn default() -> Self {
        crate::empty_plugin()
    }
}

//#region 🔖️HandcraftedArtifactCodecs



//#endregion 🔖️HandcraftedArtifactCodecs
//#endregion 🔖️Snapshot

//#region 🔖️ExternalBridges



//#endregion 🔖️ExternalBridges


