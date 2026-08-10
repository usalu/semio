//! 🧬️ ProgramSnapshot diff schema — sparse field delta over the artifact.

use crate::artifacts::program::kernel::*;
use crate::artifacts::program::registers::*;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};
use crate::artifacts::program::ProgramSnapshot;

//#region 🔖️Diff
/// 🔺️ Sparse field delta for the program artifact.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.architect.program")]
pub struct ProgramDiff {
    #[state(persistent)] pub artifact: Option<Box<crate::artifacts::program::schema::ProgramArtifact>>,
    #[state(persistent)] pub schema: Option<String>,
    #[state(persistent)] pub meta: Option<ProgramMeta>,
    #[state(persistent)] pub project: Option<ProjectDefinition>,
    #[state(persistent)] pub stakeholders: Option<ProgramStakeholdersDelta>,
    #[state(persistent)] pub users: Option<ProgramUsersDelta>,
    #[state(persistent)] pub activities: Option<ProgramActivitiesDelta>,
    #[state(persistent)] pub functions: Option<ProgramFunctionsDelta>,
    #[state(persistent)] pub elements: Option<ProgramElementsDelta>,
    #[state(persistent)] pub quantities: Option<ProgramQuantitiesDelta>,
    #[state(persistent)] pub relationships: Option<ProgramRelationshipsDelta>,
    #[state(persistent)] pub adjacencies: Option<ProgramAdjacenciesDelta>,
    #[state(persistent)] pub processes: Option<ProgramProcessesDelta>,
    #[state(persistent)] pub flows: Option<ProgramFlowsDelta>,
    #[state(persistent)] pub access_rules: Option<ProgramAccessRulesDelta>,
    #[state(persistent)] pub operations: Option<ProgramOperationsDelta>,
    #[state(persistent)] pub equipment: Option<ProgramEquipmentDelta>,
    #[state(persistent)] pub resources: Option<ProgramResourcesDelta>,
    #[state(persistent)] pub storage: Option<ProgramStorageDelta>,
    #[state(persistent)] pub environmental: Option<ProgramEnvironmentalDelta>,
    #[state(persistent)] pub human_factors: Option<ProgramHumanFactorsDelta>,
    #[state(persistent)] pub accessibility: Option<ProgramAccessibilityDelta>,
    #[state(persistent)] pub privacy: Option<ProgramPrivacyDelta>,
    #[state(persistent)] pub safety: Option<ProgramSafetyDelta>,
    #[state(persistent)] pub security: Option<ProgramSecurityDelta>,
    #[state(persistent)] pub regulatory: Option<ProgramRegulatoryDelta>,
    #[state(persistent)] pub site_context: Option<ProgramSiteContextDelta>,
    #[state(persistent)] pub organizational: Option<ProgramOrganizationalDelta>,
    #[state(persistent)] pub services: Option<ProgramServicesDelta>,
    #[state(persistent)] pub infrastructure: Option<ProgramInfrastructureDelta>,
    #[state(persistent)] pub information: Option<ProgramInformationDelta>,
    #[state(persistent)] pub communication: Option<ProgramCommunicationDelta>,
    #[state(persistent)] pub wayfinding: Option<ProgramWayfindingDelta>,
    #[state(persistent)] pub schedules: Option<ProgramSchedulesDelta>,
    #[state(persistent)] pub flexibility: Option<ProgramFlexibilityDelta>,
    #[state(persistent)] pub growth: Option<ProgramGrowthDelta>,
    #[state(persistent)] pub sustainability: Option<ProgramSustainabilityDelta>,
    #[state(persistent)] pub resilience: Option<ProgramResilienceDelta>,
    #[state(persistent)] pub costs: Option<ProgramCostsDelta>,
    #[state(persistent)] pub delivery: Option<ProgramDeliveryDelta>,
    #[state(persistent)] pub risks: Option<ProgramRisksDelta>,
    #[state(persistent)] pub conflicts: Option<ProgramConflictsDelta>,
    #[state(persistent)] pub requirements: Option<ProgramRequirementsDelta>,
    #[state(persistent)] pub priorities: Option<ProgramPrioritiesDelta>,
    #[state(persistent)] pub scenarios: Option<ProgramScenariosDelta>,
    #[state(persistent)] pub options: Option<ProgramOptionsDelta>,
    #[state(persistent)] pub decisions: Option<ProgramDecisionsDelta>,
    #[state(persistent)] pub validations: Option<ProgramValidationsDelta>,
    #[state(persistent)] pub performance: Option<ProgramPerformanceDelta>,
    #[state(persistent)] pub quality: Option<ProgramQualityDelta>,
    #[state(persistent)] pub documents: Option<ProgramArtifactsDelta>,
    #[state(persistent)] pub assumptions: Option<ProgramAssumptionsDelta>,
    #[state(persistent)] pub constraints: Option<ProgramConstraintsDelta>,
    #[state(persistent)] pub compliance_records: Option<ProgramComplianceRecordsDelta>,
    #[state(persistent)] pub approvals: Option<ProgramApprovalsDelta>,
    #[state(persistent)] pub meetings: Option<ProgramMeetingsDelta>,
    #[state(persistent)] pub changes: Option<ProgramChangesDelta>,
    #[state(persistent)] pub collaboration: Option<ProgramCollaborationDelta>,
    #[state(persistent)] pub analyses: Option<ProgramAnalysesDelta>,
    #[state(persistent)] pub reports: Option<ProgramReportsDelta>,
    #[state(persistent)] pub search_filters: Option<ProgramSearchFiltersDelta>,
    #[state(persistent)] pub status_records: Option<ProgramStatusRecordsDelta>,
    #[state(persistent)] pub workshops: Option<ProgramWorkshopsDelta>,
    #[state(persistent)] pub surveys: Option<ProgramSurveysDelta>,
    #[state(persistent)] pub issues: Option<ProgramIssuesDelta>,
    #[state(persistent)] pub audit_events: Option<ProgramAuditEventsDelta>,
    #[state(persistent)] pub templates: Option<ProgramTemplatesDelta>,
    #[state(persistent)] pub knowledge: Option<ProgramKnowledgeDelta>,
    #[state(persistent)] pub benchmarks: Option<ProgramBenchmarksDelta>,
    #[state(persistent)] pub traces: Option<ProgramTracesDelta>,
    #[state(persistent)] pub governance: Option<Governance>,
    #[state(shared_ui)] pub selected_ids: Option<ProgramStringList>,
    #[state(shared_ui)] pub active_register: Option<String>,
    #[state(shared_ui)] pub adjacency_kind_filter: Option<Option<AdjacencyKind>>,
    #[state(shared_ui)] pub active_report_json: Option<String>,
    #[state(local_ui)] pub search_query: Option<String>,
    #[state(local_ui)] pub search_history_json: Option<String>,
    #[state(local_ui)] pub last_result_json: Option<String>,
    #[state(local_ui)] pub last_analysis_json: Option<String>,
    #[state(local_ui)] pub graph_camera_x: Option<f64>,
    #[state(local_ui)] pub graph_camera_y: Option<f64>,
    #[state(local_ui)] pub graph_camera_zoom: Option<f64>,
}
//#endregion 🔖️Diff

//#region 🔖️DeltaHelpers
/// 📋 String-list wrapper so optional list diffs stay scalar across formats.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct ProgramStringList { pub values: Vec<String>, }

/// 🧩 Identified-collection delta for `stakeholders`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct ProgramStakeholdersDelta {
    pub added: Vec<Stakeholder>,
    pub removed: Vec<String>,
    pub patched: Vec<ProgramStakeholdersPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched `Stakeholder` entry.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProgramStakeholdersPatchEntry {
    pub id: String,
    pub patch: StakeholderPatch,
}

/// 🧩 Identified-collection delta for `users`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct ProgramUsersDelta {
    pub added: Vec<UserProfile>,
    pub removed: Vec<String>,
    pub patched: Vec<ProgramUsersPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched `UserProfile` entry.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProgramUsersPatchEntry {
    pub id: String,
    pub patch: UserProfilePatch,
}

/// 🧩 Identified-collection delta for `activities`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct ProgramActivitiesDelta {
    pub added: Vec<Activity>,
    pub removed: Vec<String>,
    pub patched: Vec<ProgramActivitiesPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched `Activity` entry.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProgramActivitiesPatchEntry {
    pub id: String,
    pub patch: ActivityPatch,
}

/// 🧩 Identified-collection delta for `functions`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct ProgramFunctionsDelta {
    pub added: Vec<Function>,
    pub removed: Vec<String>,
    pub patched: Vec<ProgramFunctionsPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched `Function` entry.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProgramFunctionsPatchEntry {
    pub id: String,
    pub patch: FunctionPatch,
}

/// 🧩 Identified-collection delta for `elements`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct ProgramElementsDelta {
    pub added: Vec<ProgramElement>,
    pub removed: Vec<String>,
    pub patched: Vec<ProgramElementsPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched `ProgramElement` entry.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProgramElementsPatchEntry {
    pub id: String,
    pub patch: ProgramElementPatch,
}

/// 🧩 Identified-collection delta for `quantities`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct ProgramQuantitiesDelta {
    pub added: Vec<QuantityRequirement>,
    pub removed: Vec<String>,
    pub patched: Vec<ProgramQuantitiesPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched `QuantityRequirement` entry.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProgramQuantitiesPatchEntry {
    pub id: String,
    pub patch: QuantityRequirementPatch,
}

/// 🧩 Identified-collection delta for `relationships`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct ProgramRelationshipsDelta {
    pub added: Vec<Relationship>,
    pub removed: Vec<String>,
    pub patched: Vec<ProgramRelationshipsPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched `Relationship` entry.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProgramRelationshipsPatchEntry {
    pub id: String,
    pub patch: RelationshipPatch,
}

/// 🧩 Identified-collection delta for `adjacencies`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct ProgramAdjacenciesDelta {
    pub added: Vec<Adjacency>,
    pub removed: Vec<String>,
    pub patched: Vec<ProgramAdjacenciesPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched `Adjacency` entry.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProgramAdjacenciesPatchEntry {
    pub id: String,
    pub patch: AdjacencyPatch,
}

/// 🧩 Identified-collection delta for `processes`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct ProgramProcessesDelta {
    pub added: Vec<Process>,
    pub removed: Vec<String>,
    pub patched: Vec<ProgramProcessesPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched `Process` entry.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProgramProcessesPatchEntry {
    pub id: String,
    pub patch: ProcessPatch,
}

/// 🧩 Identified-collection delta for `flows`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct ProgramFlowsDelta {
    pub added: Vec<FlowRequirement>,
    pub removed: Vec<String>,
    pub patched: Vec<ProgramFlowsPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched `FlowRequirement` entry.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProgramFlowsPatchEntry {
    pub id: String,
    pub patch: FlowRequirementPatch,
}

/// 🧩 Identified-collection delta for `access_rules`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct ProgramAccessRulesDelta {
    pub added: Vec<AccessRule>,
    pub removed: Vec<String>,
    pub patched: Vec<ProgramAccessRulesPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched `AccessRule` entry.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProgramAccessRulesPatchEntry {
    pub id: String,
    pub patch: AccessRulePatch,
}

/// 🧩 Identified-collection delta for `operations`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct ProgramOperationsDelta {
    pub added: Vec<OperationalRequirement>,
    pub removed: Vec<String>,
    pub patched: Vec<ProgramOperationsPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched `OperationalRequirement` entry.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProgramOperationsPatchEntry {
    pub id: String,
    pub patch: OperationalRequirementPatch,
}

/// 🧩 Identified-collection delta for `equipment`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct ProgramEquipmentDelta {
    pub added: Vec<Equipment>,
    pub removed: Vec<String>,
    pub patched: Vec<ProgramEquipmentPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched `Equipment` entry.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProgramEquipmentPatchEntry {
    pub id: String,
    pub patch: EquipmentPatch,
}

/// 🧩 Identified-collection delta for `resources`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct ProgramResourcesDelta {
    pub added: Vec<Resource>,
    pub removed: Vec<String>,
    pub patched: Vec<ProgramResourcesPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched `Resource` entry.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProgramResourcesPatchEntry {
    pub id: String,
    pub patch: ResourcePatch,
}

/// 🧩 Identified-collection delta for `storage`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct ProgramStorageDelta {
    pub added: Vec<StorageRequirement>,
    pub removed: Vec<String>,
    pub patched: Vec<ProgramStoragePatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched `StorageRequirement` entry.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProgramStoragePatchEntry {
    pub id: String,
    pub patch: StorageRequirementPatch,
}

/// 🧩 Identified-collection delta for `environmental`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct ProgramEnvironmentalDelta {
    pub added: Vec<EnvironmentalRequirement>,
    pub removed: Vec<String>,
    pub patched: Vec<ProgramEnvironmentalPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched `EnvironmentalRequirement` entry.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProgramEnvironmentalPatchEntry {
    pub id: String,
    pub patch: EnvironmentalRequirementPatch,
}

/// 🧩 Identified-collection delta for `human_factors`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct ProgramHumanFactorsDelta {
    pub added: Vec<HumanFactorRequirement>,
    pub removed: Vec<String>,
    pub patched: Vec<ProgramHumanFactorsPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched `HumanFactorRequirement` entry.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProgramHumanFactorsPatchEntry {
    pub id: String,
    pub patch: HumanFactorRequirementPatch,
}

/// 🧩 Identified-collection delta for `accessibility`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct ProgramAccessibilityDelta {
    pub added: Vec<AccessibilityRequirement>,
    pub removed: Vec<String>,
    pub patched: Vec<ProgramAccessibilityPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched `AccessibilityRequirement` entry.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProgramAccessibilityPatchEntry {
    pub id: String,
    pub patch: AccessibilityRequirementPatch,
}

/// 🧩 Identified-collection delta for `privacy`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct ProgramPrivacyDelta {
    pub added: Vec<PrivacyRequirement>,
    pub removed: Vec<String>,
    pub patched: Vec<ProgramPrivacyPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched `PrivacyRequirement` entry.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProgramPrivacyPatchEntry {
    pub id: String,
    pub patch: PrivacyRequirementPatch,
}

/// 🧩 Identified-collection delta for `safety`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct ProgramSafetyDelta {
    pub added: Vec<SafetyRequirement>,
    pub removed: Vec<String>,
    pub patched: Vec<ProgramSafetyPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched `SafetyRequirement` entry.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProgramSafetyPatchEntry {
    pub id: String,
    pub patch: SafetyRequirementPatch,
}

/// 🧩 Identified-collection delta for `security`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct ProgramSecurityDelta {
    pub added: Vec<SecurityRequirement>,
    pub removed: Vec<String>,
    pub patched: Vec<ProgramSecurityPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched `SecurityRequirement` entry.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProgramSecurityPatchEntry {
    pub id: String,
    pub patch: SecurityRequirementPatch,
}

/// 🧩 Identified-collection delta for `regulatory`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct ProgramRegulatoryDelta {
    pub added: Vec<RegulatoryRequirement>,
    pub removed: Vec<String>,
    pub patched: Vec<ProgramRegulatoryPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched `RegulatoryRequirement` entry.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProgramRegulatoryPatchEntry {
    pub id: String,
    pub patch: RegulatoryRequirementPatch,
}

/// 🧩 Identified-collection delta for `site_context`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct ProgramSiteContextDelta {
    pub added: Vec<SiteContext>,
    pub removed: Vec<String>,
    pub patched: Vec<ProgramSiteContextPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched `SiteContext` entry.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProgramSiteContextPatchEntry {
    pub id: String,
    pub patch: SiteContextPatch,
}

/// 🧩 Identified-collection delta for `organizational`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct ProgramOrganizationalDelta {
    pub added: Vec<OrganizationalRequirement>,
    pub removed: Vec<String>,
    pub patched: Vec<ProgramOrganizationalPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched `OrganizationalRequirement` entry.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProgramOrganizationalPatchEntry {
    pub id: String,
    pub patch: OrganizationalRequirementPatch,
}

/// 🧩 Identified-collection delta for `services`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct ProgramServicesDelta {
    pub added: Vec<ServiceRequirement>,
    pub removed: Vec<String>,
    pub patched: Vec<ProgramServicesPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched `ServiceRequirement` entry.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProgramServicesPatchEntry {
    pub id: String,
    pub patch: ServiceRequirementPatch,
}

/// 🧩 Identified-collection delta for `infrastructure`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct ProgramInfrastructureDelta {
    pub added: Vec<InfrastructureRequirement>,
    pub removed: Vec<String>,
    pub patched: Vec<ProgramInfrastructurePatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched `InfrastructureRequirement` entry.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProgramInfrastructurePatchEntry {
    pub id: String,
    pub patch: InfrastructureRequirementPatch,
}

/// 🧩 Identified-collection delta for `information`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct ProgramInformationDelta {
    pub added: Vec<InformationRequirement>,
    pub removed: Vec<String>,
    pub patched: Vec<ProgramInformationPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched `InformationRequirement` entry.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProgramInformationPatchEntry {
    pub id: String,
    pub patch: InformationRequirementPatch,
}

/// 🧩 Identified-collection delta for `communication`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct ProgramCommunicationDelta {
    pub added: Vec<CommunicationRequirement>,
    pub removed: Vec<String>,
    pub patched: Vec<ProgramCommunicationPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched `CommunicationRequirement` entry.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProgramCommunicationPatchEntry {
    pub id: String,
    pub patch: CommunicationRequirementPatch,
}

/// 🧩 Identified-collection delta for `wayfinding`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct ProgramWayfindingDelta {
    pub added: Vec<WayfindingRequirement>,
    pub removed: Vec<String>,
    pub patched: Vec<ProgramWayfindingPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched `WayfindingRequirement` entry.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProgramWayfindingPatchEntry {
    pub id: String,
    pub patch: WayfindingRequirementPatch,
}

/// 🧩 Identified-collection delta for `schedules`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct ProgramSchedulesDelta {
    pub added: Vec<ScheduleRequirement>,
    pub removed: Vec<String>,
    pub patched: Vec<ProgramSchedulesPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched `ScheduleRequirement` entry.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProgramSchedulesPatchEntry {
    pub id: String,
    pub patch: ScheduleRequirementPatch,
}

/// 🧩 Identified-collection delta for `flexibility`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct ProgramFlexibilityDelta {
    pub added: Vec<FlexibilityRequirement>,
    pub removed: Vec<String>,
    pub patched: Vec<ProgramFlexibilityPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched `FlexibilityRequirement` entry.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProgramFlexibilityPatchEntry {
    pub id: String,
    pub patch: FlexibilityRequirementPatch,
}

/// 🧩 Identified-collection delta for `growth`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct ProgramGrowthDelta {
    pub added: Vec<GrowthPlan>,
    pub removed: Vec<String>,
    pub patched: Vec<ProgramGrowthPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched `GrowthPlan` entry.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProgramGrowthPatchEntry {
    pub id: String,
    pub patch: GrowthPlanPatch,
}

/// 🧩 Identified-collection delta for `sustainability`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct ProgramSustainabilityDelta {
    pub added: Vec<SustainabilityRequirement>,
    pub removed: Vec<String>,
    pub patched: Vec<ProgramSustainabilityPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched `SustainabilityRequirement` entry.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProgramSustainabilityPatchEntry {
    pub id: String,
    pub patch: SustainabilityRequirementPatch,
}

/// 🧩 Identified-collection delta for `resilience`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct ProgramResilienceDelta {
    pub added: Vec<ResilienceRequirement>,
    pub removed: Vec<String>,
    pub patched: Vec<ProgramResiliencePatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched `ResilienceRequirement` entry.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProgramResiliencePatchEntry {
    pub id: String,
    pub patch: ResilienceRequirementPatch,
}

/// 🧩 Identified-collection delta for `costs`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct ProgramCostsDelta {
    pub added: Vec<CostRequirement>,
    pub removed: Vec<String>,
    pub patched: Vec<ProgramCostsPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched `CostRequirement` entry.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProgramCostsPatchEntry {
    pub id: String,
    pub patch: CostRequirementPatch,
}

/// 🧩 Identified-collection delta for `delivery`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct ProgramDeliveryDelta {
    pub added: Vec<DeliveryConstraint>,
    pub removed: Vec<String>,
    pub patched: Vec<ProgramDeliveryPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched `DeliveryConstraint` entry.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProgramDeliveryPatchEntry {
    pub id: String,
    pub patch: DeliveryConstraintPatch,
}

/// 🧩 Identified-collection delta for `risks`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct ProgramRisksDelta {
    pub added: Vec<Risk>,
    pub removed: Vec<String>,
    pub patched: Vec<ProgramRisksPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched `Risk` entry.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProgramRisksPatchEntry {
    pub id: String,
    pub patch: RiskPatch,
}

/// 🧩 Identified-collection delta for `conflicts`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct ProgramConflictsDelta {
    pub added: Vec<Conflict>,
    pub removed: Vec<String>,
    pub patched: Vec<ProgramConflictsPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched `Conflict` entry.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProgramConflictsPatchEntry {
    pub id: String,
    pub patch: ConflictPatch,
}

/// 🧩 Identified-collection delta for `requirements`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct ProgramRequirementsDelta {
    pub added: Vec<Requirement>,
    pub removed: Vec<String>,
    pub patched: Vec<ProgramRequirementsPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched `Requirement` entry.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProgramRequirementsPatchEntry {
    pub id: String,
    pub patch: RequirementPatch,
}

/// 🧩 Identified-collection delta for `priorities`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct ProgramPrioritiesDelta {
    pub added: Vec<PriorityRecord>,
    pub removed: Vec<String>,
    pub patched: Vec<ProgramPrioritiesPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched `PriorityRecord` entry.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProgramPrioritiesPatchEntry {
    pub id: String,
    pub patch: PriorityRecordPatch,
}

/// 🧩 Identified-collection delta for `scenarios`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct ProgramScenariosDelta {
    pub added: Vec<Scenario>,
    pub removed: Vec<String>,
    pub patched: Vec<ProgramScenariosPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched `Scenario` entry.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProgramScenariosPatchEntry {
    pub id: String,
    pub patch: ScenarioPatch,
}

/// 🧩 Identified-collection delta for `options`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct ProgramOptionsDelta {
    pub added: Vec<OptionEvaluation>,
    pub removed: Vec<String>,
    pub patched: Vec<ProgramOptionsPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched `OptionEvaluation` entry.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProgramOptionsPatchEntry {
    pub id: String,
    pub patch: OptionEvaluationPatch,
}

/// 🧩 Identified-collection delta for `decisions`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct ProgramDecisionsDelta {
    pub added: Vec<Decision>,
    pub removed: Vec<String>,
    pub patched: Vec<ProgramDecisionsPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched `Decision` entry.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProgramDecisionsPatchEntry {
    pub id: String,
    pub patch: DecisionPatch,
}

/// 🧩 Identified-collection delta for `validations`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct ProgramValidationsDelta {
    pub added: Vec<ValidationRecord>,
    pub removed: Vec<String>,
    pub patched: Vec<ProgramValidationsPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched `ValidationRecord` entry.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProgramValidationsPatchEntry {
    pub id: String,
    pub patch: ValidationRecordPatch,
}

/// 🧩 Identified-collection delta for `performance`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct ProgramPerformanceDelta {
    pub added: Vec<PerformanceCriterion>,
    pub removed: Vec<String>,
    pub patched: Vec<ProgramPerformancePatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched `PerformanceCriterion` entry.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProgramPerformancePatchEntry {
    pub id: String,
    pub patch: PerformanceCriterionPatch,
}

/// 🧩 Identified-collection delta for `quality`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct ProgramQualityDelta {
    pub added: Vec<QualityRecord>,
    pub removed: Vec<String>,
    pub patched: Vec<ProgramQualityPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched `QualityRecord` entry.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProgramQualityPatchEntry {
    pub id: String,
    pub patch: QualityRecordPatch,
}

/// 🧩 Identified-collection delta for `documents`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct ProgramArtifactsDelta {
    pub added: Vec<ArtifactRecord>,
    pub removed: Vec<String>,
    pub patched: Vec<ProgramArtifactsPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched `ArtifactRecord` entry.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProgramArtifactsPatchEntry {
    pub id: String,
    pub patch: ArtifactRecordPatch,
}

/// 🧩 Identified-collection delta for `assumptions`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct ProgramAssumptionsDelta {
    pub added: Vec<Assumption>,
    pub removed: Vec<String>,
    pub patched: Vec<ProgramAssumptionsPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched `Assumption` entry.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProgramAssumptionsPatchEntry {
    pub id: String,
    pub patch: AssumptionPatch,
}

/// 🧩 Identified-collection delta for `constraints`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct ProgramConstraintsDelta {
    pub added: Vec<ConstraintRecord>,
    pub removed: Vec<String>,
    pub patched: Vec<ProgramConstraintsPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched `ConstraintRecord` entry.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProgramConstraintsPatchEntry {
    pub id: String,
    pub patch: ConstraintRecordPatch,
}

/// 🧩 Identified-collection delta for `compliance_records`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct ProgramComplianceRecordsDelta {
    pub added: Vec<ComplianceRecord>,
    pub removed: Vec<String>,
    pub patched: Vec<ProgramComplianceRecordsPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched `ComplianceRecord` entry.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProgramComplianceRecordsPatchEntry {
    pub id: String,
    pub patch: ComplianceRecordPatch,
}

/// 🧩 Identified-collection delta for `approvals`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct ProgramApprovalsDelta {
    pub added: Vec<ApprovalRecord>,
    pub removed: Vec<String>,
    pub patched: Vec<ProgramApprovalsPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched `ApprovalRecord` entry.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProgramApprovalsPatchEntry {
    pub id: String,
    pub patch: ApprovalRecordPatch,
}

/// 🧩 Identified-collection delta for `meetings`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct ProgramMeetingsDelta {
    pub added: Vec<MeetingRecord>,
    pub removed: Vec<String>,
    pub patched: Vec<ProgramMeetingsPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched `MeetingRecord` entry.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProgramMeetingsPatchEntry {
    pub id: String,
    pub patch: MeetingRecordPatch,
}

/// 🧩 Identified-collection delta for `changes`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct ProgramChangesDelta {
    pub added: Vec<ChangeRecord>,
    pub removed: Vec<String>,
    pub patched: Vec<ProgramChangesPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched `ChangeRecord` entry.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProgramChangesPatchEntry {
    pub id: String,
    pub patch: ChangeRecordPatch,
}

/// 🧩 Identified-collection delta for `collaboration`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct ProgramCollaborationDelta {
    pub added: Vec<CollaborationRecord>,
    pub removed: Vec<String>,
    pub patched: Vec<ProgramCollaborationPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched `CollaborationRecord` entry.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProgramCollaborationPatchEntry {
    pub id: String,
    pub patch: CollaborationRecordPatch,
}

/// 🧩 Identified-collection delta for `analyses`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct ProgramAnalysesDelta {
    pub added: Vec<AnalysisRecord>,
    pub removed: Vec<String>,
    pub patched: Vec<ProgramAnalysesPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched `AnalysisRecord` entry.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProgramAnalysesPatchEntry {
    pub id: String,
    pub patch: AnalysisRecordPatch,
}

/// 🧩 Identified-collection delta for `reports`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct ProgramReportsDelta {
    pub added: Vec<ReportRecord>,
    pub removed: Vec<String>,
    pub patched: Vec<ProgramReportsPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched `ReportRecord` entry.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProgramReportsPatchEntry {
    pub id: String,
    pub patch: ReportRecordPatch,
}

/// 🧩 Identified-collection delta for `search_filters`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct ProgramSearchFiltersDelta {
    pub added: Vec<SearchFilter>,
    pub removed: Vec<String>,
    pub patched: Vec<ProgramSearchFiltersPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched `SearchFilter` entry.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProgramSearchFiltersPatchEntry {
    pub id: String,
    pub patch: SearchFilterPatch,
}

/// 🧩 Identified-collection delta for `status_records`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct ProgramStatusRecordsDelta {
    pub added: Vec<StatusRecord>,
    pub removed: Vec<String>,
    pub patched: Vec<ProgramStatusRecordsPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched `StatusRecord` entry.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProgramStatusRecordsPatchEntry {
    pub id: String,
    pub patch: StatusRecordPatch,
}

/// 🧩 Identified-collection delta for `workshops`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct ProgramWorkshopsDelta {
    pub added: Vec<Workshop>,
    pub removed: Vec<String>,
    pub patched: Vec<ProgramWorkshopsPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched `Workshop` entry.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProgramWorkshopsPatchEntry {
    pub id: String,
    pub patch: WorkshopPatch,
}

/// 🧩 Identified-collection delta for `surveys`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct ProgramSurveysDelta {
    pub added: Vec<Survey>,
    pub removed: Vec<String>,
    pub patched: Vec<ProgramSurveysPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched `Survey` entry.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProgramSurveysPatchEntry {
    pub id: String,
    pub patch: SurveyPatch,
}

/// 🧩 Identified-collection delta for `issues`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct ProgramIssuesDelta {
    pub added: Vec<Issue>,
    pub removed: Vec<String>,
    pub patched: Vec<ProgramIssuesPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched `Issue` entry.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProgramIssuesPatchEntry {
    pub id: String,
    pub patch: IssuePatch,
}

/// 🧩 Identified-collection delta for `audit_events`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct ProgramAuditEventsDelta {
    pub added: Vec<AuditEvent>,
    pub removed: Vec<String>,
    pub patched: Vec<ProgramAuditEventsPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched `AuditEvent` entry.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProgramAuditEventsPatchEntry {
    pub id: String,
    pub patch: AuditEventPatch,
}

/// 🧩 Identified-collection delta for `templates`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct ProgramTemplatesDelta {
    pub added: Vec<TemplateRecord>,
    pub removed: Vec<String>,
    pub patched: Vec<ProgramTemplatesPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched `TemplateRecord` entry.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProgramTemplatesPatchEntry {
    pub id: String,
    pub patch: TemplateRecordPatch,
}

/// 🧩 Identified-collection delta for `knowledge`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct ProgramKnowledgeDelta {
    pub added: Vec<KnowledgeRecord>,
    pub removed: Vec<String>,
    pub patched: Vec<ProgramKnowledgePatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched `KnowledgeRecord` entry.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProgramKnowledgePatchEntry {
    pub id: String,
    pub patch: KnowledgeRecordPatch,
}

/// 🧩 Identified-collection delta for `benchmarks`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct ProgramBenchmarksDelta {
    pub added: Vec<BenchmarkRecord>,
    pub removed: Vec<String>,
    pub patched: Vec<ProgramBenchmarksPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched `BenchmarkRecord` entry.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProgramBenchmarksPatchEntry {
    pub id: String,
    pub patch: BenchmarkRecordPatch,
}

/// 🧩 Identified-collection delta for `traces`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct ProgramTracesDelta {
    pub added: Vec<TraceLink>,
    pub removed: Vec<String>,
    pub patched: Vec<ProgramTracesPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched `TraceLink` entry.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProgramTracesPatchEntry {
    pub id: String,
    pub patch: TraceLinkPatch,
}

//#endregion 🔖️DeltaHelpers
