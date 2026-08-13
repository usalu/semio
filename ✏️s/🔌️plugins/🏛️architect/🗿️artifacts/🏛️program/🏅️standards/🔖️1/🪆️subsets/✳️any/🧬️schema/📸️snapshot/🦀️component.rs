//! 🧬️ ProgramSnapshot snapshot schema — artifact-lane fields only.

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
    #[dsl(block)]
    #[child(kind = "s.stdio.semio.table")]
    #[state(artifact)]
    pub knowledge: crate::artifacts::program::ProgramKnowledgeChild,
    #[dsl(block)]
    #[child(kind = "s.stdio.semio.table")]
    #[state(artifact)]
    pub benchmarks: crate::artifacts::program::ProgramBenchmarksChild,
    #[dsl(table)]
    #[state(artifact)]
    pub traces: Vec<TraceLink>,
    #[dsl(block)]
    #[state(artifact)]
    pub governance: Governance,
}

impl Default for ProgramSnapshot {
    fn default() -> Self {
        crate::artifacts::program::empty_plugin()
    }
}

//#region 🔖️HandcraftedArtifactCodecs
/// ✉️ P6 handcrafted ArtifactDsl/ArtifactPack (derive no longer emits these traits).
impl store::ArtifactDsl for ProgramSnapshot {
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
            <Self as store::ArtifactDsl>::envelope_id(),
            store::semio_format::Component::Dsl,
            1,
        ).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for ProgramSnapshot {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let inner = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::ArtifactDsl>::envelope_id(),
            store::semio_format::Component::Pack,
            1,
        ).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes)
            .map_err(|e| store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as store::ArtifactDsl>::envelope_id() {
            return Err(store::PackError::Schema(format!(
                "pack envelope mismatch: expected {}, got {}",
                <Self as store::ArtifactDsl>::envelope_id(),
                envelope.envelope_id()
            )));
        }
        let (record, _report) = store::pack_rt::decode_document(&inner, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)
    }
    fn record_spec() -> Option<dsl::RecordSpec> { Some(Self::__dsl_spec()) }
}
//#endregion 🔖️HandcraftedArtifactCodecs
//#endregion 🔖️Snapshot
