//! 🧬️ ProgramSnapshot snapshot schema — persistent fields only.

use crate::artifacts::program::kernel::*;
use crate::artifacts::program::registers::*;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Snapshot
/// 📸️ Persisted architect program snapshot (persistent fields of the artifact).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[dsl(extension = "architect", layout = "lines")]
#[artifact_schema(id = "s.architect.program")]
pub struct ProgramSnapshot {
    #[state(persistent)]
    pub schema: String,
    #[dsl(block)]
    #[state(persistent)]
    pub meta: ProgramMeta,
    #[dsl(block)]
    #[state(persistent)]
    pub project: ProjectDefinition,
    #[dsl(table)]
    #[state(persistent)]
    pub stakeholders: Vec<Stakeholder>,
    #[dsl(table)]
    #[state(persistent)]
    pub users: Vec<UserProfile>,
    #[dsl(table)]
    #[state(persistent)]
    pub activities: Vec<Activity>,
    #[dsl(table)]
    #[state(persistent)]
    pub functions: Vec<Function>,
    #[dsl(table)]
    #[state(persistent)]
    pub elements: Vec<ProgramElement>,
    #[dsl(table)]
    #[state(persistent)]
    pub quantities: Vec<QuantityRequirement>,
    #[dsl(table)]
    #[state(persistent)]
    pub relationships: Vec<Relationship>,
    #[dsl(table)]
    #[state(persistent)]
    pub adjacencies: Vec<Adjacency>,
    #[dsl(table)]
    #[state(persistent)]
    pub processes: Vec<Process>,
    #[dsl(table)]
    #[state(persistent)]
    pub flows: Vec<FlowRequirement>,
    #[dsl(table)]
    #[state(persistent)]
    pub access_rules: Vec<AccessRule>,
    #[dsl(table)]
    #[state(persistent)]
    pub operations: Vec<OperationalRequirement>,
    #[dsl(table)]
    #[state(persistent)]
    pub equipment: Vec<Equipment>,
    #[dsl(table)]
    #[state(persistent)]
    pub resources: Vec<Resource>,
    #[dsl(table)]
    #[state(persistent)]
    pub storage: Vec<StorageRequirement>,
    #[dsl(table)]
    #[state(persistent)]
    pub environmental: Vec<EnvironmentalRequirement>,
    #[dsl(table)]
    #[state(persistent)]
    pub human_factors: Vec<HumanFactorRequirement>,
    #[dsl(table)]
    #[state(persistent)]
    pub accessibility: Vec<AccessibilityRequirement>,
    #[dsl(table)]
    #[state(persistent)]
    pub privacy: Vec<PrivacyRequirement>,
    #[dsl(table)]
    #[state(persistent)]
    pub safety: Vec<SafetyRequirement>,
    #[dsl(table)]
    #[state(persistent)]
    pub security: Vec<SecurityRequirement>,
    #[dsl(table)]
    #[state(persistent)]
    pub regulatory: Vec<RegulatoryRequirement>,
    #[dsl(table)]
    #[state(persistent)]
    pub site_context: Vec<SiteContext>,
    #[dsl(table)]
    #[state(persistent)]
    pub organizational: Vec<OrganizationalRequirement>,
    #[dsl(table)]
    #[state(persistent)]
    pub services: Vec<ServiceRequirement>,
    #[dsl(table)]
    #[state(persistent)]
    pub infrastructure: Vec<InfrastructureRequirement>,
    #[dsl(table)]
    #[state(persistent)]
    pub information: Vec<InformationRequirement>,
    #[dsl(table)]
    #[state(persistent)]
    pub communication: Vec<CommunicationRequirement>,
    #[dsl(table)]
    #[state(persistent)]
    pub wayfinding: Vec<WayfindingRequirement>,
    #[dsl(table)]
    #[state(persistent)]
    pub schedules: Vec<ScheduleRequirement>,
    #[dsl(table)]
    #[state(persistent)]
    pub flexibility: Vec<FlexibilityRequirement>,
    #[dsl(table)]
    #[state(persistent)]
    pub growth: Vec<GrowthPlan>,
    #[dsl(table)]
    #[state(persistent)]
    pub sustainability: Vec<SustainabilityRequirement>,
    #[dsl(table)]
    #[state(persistent)]
    pub resilience: Vec<ResilienceRequirement>,
    #[dsl(table)]
    #[state(persistent)]
    pub costs: Vec<CostRequirement>,
    #[dsl(table)]
    #[state(persistent)]
    pub delivery: Vec<DeliveryConstraint>,
    #[dsl(table)]
    #[state(persistent)]
    pub risks: Vec<Risk>,
    #[dsl(table)]
    #[state(persistent)]
    pub conflicts: Vec<Conflict>,
    #[dsl(table)]
    #[state(persistent)]
    pub requirements: Vec<Requirement>,
    #[dsl(table)]
    #[state(persistent)]
    pub priorities: Vec<PriorityRecord>,
    #[dsl(table)]
    #[state(persistent)]
    pub scenarios: Vec<Scenario>,
    #[dsl(table)]
    #[state(persistent)]
    pub options: Vec<OptionEvaluation>,
    #[dsl(table)]
    #[state(persistent)]
    pub decisions: Vec<Decision>,
    #[dsl(table)]
    #[state(persistent)]
    pub validations: Vec<ValidationRecord>,
    #[dsl(table)]
    #[state(persistent)]
    pub performance: Vec<PerformanceCriterion>,
    #[dsl(table)]
    #[state(persistent)]
    pub quality: Vec<QualityRecord>,
    #[dsl(table)]
    #[state(persistent)]
    pub documents: Vec<DocumentRecord>,
    #[dsl(table)]
    #[state(persistent)]
    pub assumptions: Vec<Assumption>,
    #[dsl(table)]
    #[state(persistent)]
    pub constraints: Vec<ConstraintRecord>,
    #[dsl(table)]
    #[state(persistent)]
    pub compliance_records: Vec<ComplianceRecord>,
    #[dsl(table)]
    #[state(persistent)]
    pub approvals: Vec<ApprovalRecord>,
    #[dsl(table)]
    #[state(persistent)]
    pub meetings: Vec<MeetingRecord>,
    #[dsl(table)]
    #[state(persistent)]
    pub changes: Vec<ChangeRecord>,
    #[dsl(table)]
    #[state(persistent)]
    pub collaboration: Vec<CollaborationRecord>,
    #[dsl(table)]
    #[state(persistent)]
    pub analyses: Vec<AnalysisRecord>,
    #[dsl(table)]
    #[state(persistent)]
    pub reports: Vec<ReportRecord>,
    #[dsl(table)]
    #[state(persistent)]
    pub search_filters: Vec<SearchFilter>,
    #[dsl(table)]
    #[state(persistent)]
    pub status_records: Vec<StatusRecord>,
    #[dsl(table)]
    #[state(persistent)]
    pub workshops: Vec<Workshop>,
    #[dsl(table)]
    #[state(persistent)]
    pub surveys: Vec<Survey>,
    #[dsl(table)]
    #[state(persistent)]
    pub issues: Vec<Issue>,
    #[dsl(table)]
    #[state(persistent)]
    pub audit_events: Vec<AuditEvent>,
    #[dsl(table)]
    #[state(persistent)]
    pub templates: Vec<TemplateRecord>,
    #[dsl(table)]
    #[state(persistent)]
    pub knowledge: Vec<KnowledgeRecord>,
    #[dsl(table)]
    #[state(persistent)]
    pub benchmarks: Vec<BenchmarkRecord>,
    #[dsl(table)]
    #[state(persistent)]
    pub traces: Vec<TraceLink>,
    #[dsl(block)]
    #[state(persistent)]
    pub governance: Governance,
}

impl Default for ProgramSnapshot {
    fn default() -> Self {
        crate::artifacts::program::empty_plugin()
    }
}

//#region 🔖️HandcraftedDocumentCodecs
/// ✉️ P6 handcrafted DocumentDsl/DocumentPack (derive no longer emits these traits).
impl store::DocumentDsl for ProgramSnapshot {
    const EXTENSION: &'static str = "architect";
    fn envelope_id() -> &'static str { "architect.program" }
    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let record = dsl::parse(
            body,
            &Self::__dsl_spec(),
            &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Document },
        )?;
        Self::__dsl_from_record(&record)
    }
    fn print_dsl(&self) -> String {
        let body = dsl::print(&self.__dsl_to_record(), &Self::__dsl_spec(), dsl::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::DocumentDsl>::envelope_id(),
            store::semio_format::Component::Dsl,
            1,
        ).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::DocumentPack for ProgramSnapshot {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let inner = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::DocumentDsl>::envelope_id(),
            store::semio_format::Component::Pack,
            1,
        ).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes)
            .map_err(|e| store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as store::DocumentDsl>::envelope_id() {
            return Err(store::PackError::Schema(format!(
                "pack envelope mismatch: expected {}, got {}",
                <Self as store::DocumentDsl>::envelope_id(),
                envelope.envelope_id()
            )));
        }
        let (record, _report) = store::pack_rt::decode_document(&inner, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)
    }
    fn record_spec() -> Option<dsl::RecordSpec> { Some(Self::__dsl_spec()) }
}
//#endregion 🔖️HandcraftedDocumentCodecs
//#endregion 🔖️Snapshot
