//! 🧬️ ProgramSnapshot diff schema — sparse field delta over the artifact.
//!
//! Every collection is a positional `removed / inserted / moved / modified` delta (`protocol::list_delta`), every singleton section
//! (`meta`, `project`, `governance`) a `set / patch` edit, and `knowledge` / `benchmarks` are deltas over the rows their
//! composed child tables are derived from. The diff never carries a whole after-snapshot; `apply` derives the composed
//! child handles from the patched rows.

use crate::kernel::*;
use crate::registers::*;
use framework_schema::ArtifactSchema;

//#region 🔖️Diff
/// 🔺️ Sparse field delta for the program artifact.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, ArtifactSchema, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase", default, deny_unknown_fields)]
#[artifact_schema(id = "s.architect.program")]
pub struct ProgramDiff {
    #[state(artifact)]
    pub schema: Option<String>,
    #[state(artifact)]
    pub meta: Option<ProgramMetaEdit>,
    #[state(artifact)]
    pub project: Option<ProjectDefinitionEdit>,
    #[state(artifact)]
    pub stakeholders: Option<ProgramStakeholdersDelta>,
    #[state(artifact)]
    pub users: Option<ProgramUsersDelta>,
    #[state(artifact)]
    pub activities: Option<ProgramActivitiesDelta>,
    #[state(artifact)]
    pub functions: Option<ProgramFunctionsDelta>,
    #[state(artifact)]
    pub elements: Option<ProgramElementsDelta>,
    #[state(artifact)]
    pub quantities: Option<ProgramQuantitiesDelta>,
    #[state(artifact)]
    pub relationships: Option<ProgramRelationshipsDelta>,
    #[state(artifact)]
    pub adjacencies: Option<ProgramAdjacenciesDelta>,
    #[state(artifact)]
    pub processes: Option<ProgramProcessesDelta>,
    #[state(artifact)]
    pub flows: Option<ProgramFlowsDelta>,
    #[state(artifact)]
    pub access_rules: Option<ProgramAccessRulesDelta>,
    #[state(artifact)]
    pub operations: Option<ProgramOperationsDelta>,
    #[state(artifact)]
    pub equipment: Option<ProgramEquipmentDelta>,
    #[state(artifact)]
    pub resources: Option<ProgramResourcesDelta>,
    #[state(artifact)]
    pub storage: Option<ProgramStorageDelta>,
    #[state(artifact)]
    pub environmental: Option<ProgramEnvironmentalDelta>,
    #[state(artifact)]
    pub human_factors: Option<ProgramHumanFactorsDelta>,
    #[state(artifact)]
    pub accessibility: Option<ProgramAccessibilityDelta>,
    #[state(artifact)]
    pub privacy: Option<ProgramPrivacyDelta>,
    #[state(artifact)]
    pub safety: Option<ProgramSafetyDelta>,
    #[state(artifact)]
    pub security: Option<ProgramSecurityDelta>,
    #[state(artifact)]
    pub regulatory: Option<ProgramRegulatoryDelta>,
    #[state(artifact)]
    pub site_context: Option<ProgramSiteContextDelta>,
    #[state(artifact)]
    pub organizational: Option<ProgramOrganizationalDelta>,
    #[state(artifact)]
    pub services: Option<ProgramServicesDelta>,
    #[state(artifact)]
    pub infrastructure: Option<ProgramInfrastructureDelta>,
    #[state(artifact)]
    pub information: Option<ProgramInformationDelta>,
    #[state(artifact)]
    pub communication: Option<ProgramCommunicationDelta>,
    #[state(artifact)]
    pub wayfinding: Option<ProgramWayfindingDelta>,
    #[state(artifact)]
    pub schedules: Option<ProgramSchedulesDelta>,
    #[state(artifact)]
    pub flexibility: Option<ProgramFlexibilityDelta>,
    #[state(artifact)]
    pub growth: Option<ProgramGrowthDelta>,
    #[state(artifact)]
    pub sustainability: Option<ProgramSustainabilityDelta>,
    #[state(artifact)]
    pub resilience: Option<ProgramResilienceDelta>,
    #[state(artifact)]
    pub costs: Option<ProgramCostsDelta>,
    #[state(artifact)]
    pub delivery: Option<ProgramDeliveryDelta>,
    #[state(artifact)]
    pub risks: Option<ProgramRisksDelta>,
    #[state(artifact)]
    pub conflicts: Option<ProgramConflictsDelta>,
    #[state(artifact)]
    pub requirements: Option<ProgramRequirementsDelta>,
    #[state(artifact)]
    pub priorities: Option<ProgramPrioritiesDelta>,
    #[state(artifact)]
    pub scenarios: Option<ProgramScenariosDelta>,
    #[state(artifact)]
    pub options: Option<ProgramOptionsDelta>,
    #[state(artifact)]
    pub decisions: Option<ProgramDecisionsDelta>,
    #[state(artifact)]
    pub validations: Option<ProgramValidationsDelta>,
    #[state(artifact)]
    pub performance: Option<ProgramPerformanceDelta>,
    #[state(artifact)]
    pub quality: Option<ProgramQualityDelta>,
    #[state(artifact)]
    pub artifacts: Option<ProgramArtifactsDelta>,
    #[state(artifact)]
    pub assumptions: Option<ProgramAssumptionsDelta>,
    #[state(artifact)]
    pub constraints: Option<ProgramConstraintsDelta>,
    #[state(artifact)]
    pub compliance_records: Option<ProgramComplianceRecordsDelta>,
    #[state(artifact)]
    pub approvals: Option<ProgramApprovalsDelta>,
    #[state(artifact)]
    pub meetings: Option<ProgramMeetingsDelta>,
    #[state(artifact)]
    pub changes: Option<ProgramChangesDelta>,
    #[state(artifact)]
    pub collaboration: Option<ProgramCollaborationDelta>,
    #[state(artifact)]
    pub analyses: Option<ProgramAnalysesDelta>,
    #[state(artifact)]
    pub reports: Option<ProgramReportsDelta>,
    #[state(artifact)]
    pub search_filters: Option<ProgramSearchFiltersDelta>,
    #[state(artifact)]
    pub status_records: Option<ProgramStatusRecordsDelta>,
    #[state(artifact)]
    pub workshops: Option<ProgramWorkshopsDelta>,
    #[state(artifact)]
    pub surveys: Option<ProgramSurveysDelta>,
    #[state(artifact)]
    pub issues: Option<ProgramIssuesDelta>,
    #[state(artifact)]
    pub audit_events: Option<ProgramAuditEventsDelta>,
    #[state(artifact)]
    pub templates: Option<ProgramTemplatesDelta>,
    /// 📚️ Row delta of the composed knowledge table — `apply` re-derives the child handle from the patched rows.
    #[state(artifact)]
    pub knowledge: Option<ProgramKnowledgeDelta>,
    /// 🏁️ Row delta of the composed benchmarks table — `apply` re-derives the child handle from the patched rows.
    #[state(artifact)]
    pub benchmarks: Option<ProgramBenchmarksDelta>,
    #[state(artifact)]
    pub traces: Option<ProgramTracesDelta>,
    #[state(artifact)]
    pub governance: Option<GovernanceEdit>,
}
//#endregion 🔖️Diff

//#region 🔖️DeltaHelpers
protocol::list_delta! {
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(test, serde(rename_all = "camelCase"))]
    /// 🧩 Positional row delta of `stakeholders`: rows removed at their base index, inserted at their after index, moved, and modified by sparse patch.
    pub ProgramStakeholdersDelta { removal: ProgramStakeholdersRemoval, insertion: ProgramStakeholdersInsertion, relocation: ProgramStakeholdersRelocation, modification: ProgramStakeholdersPatchEntry, row: Stakeholder, patch: StakeholderPatch, list: Vec<Stakeholder>, key: String = |row| row.header.id.0.clone() }
}

protocol::list_delta! {
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(test, serde(rename_all = "camelCase"))]
    /// 🧩 Positional row delta of `users`: rows removed at their base index, inserted at their after index, moved, and modified by sparse patch.
    pub ProgramUsersDelta { removal: ProgramUsersRemoval, insertion: ProgramUsersInsertion, relocation: ProgramUsersRelocation, modification: ProgramUsersPatchEntry, row: UserProfile, patch: UserProfilePatch, list: Vec<UserProfile>, key: String = |row| row.header.id.0.clone() }
}

protocol::list_delta! {
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(test, serde(rename_all = "camelCase"))]
    /// 🧩 Positional row delta of `activities`: rows removed at their base index, inserted at their after index, moved, and modified by sparse patch.
    pub ProgramActivitiesDelta { removal: ProgramActivitiesRemoval, insertion: ProgramActivitiesInsertion, relocation: ProgramActivitiesRelocation, modification: ProgramActivitiesPatchEntry, row: Activity, patch: ActivityPatch, list: Vec<Activity>, key: String = |row| row.header.id.0.clone() }
}

protocol::list_delta! {
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(test, serde(rename_all = "camelCase"))]
    /// 🧩 Positional row delta of `functions`: rows removed at their base index, inserted at their after index, moved, and modified by sparse patch.
    pub ProgramFunctionsDelta { removal: ProgramFunctionsRemoval, insertion: ProgramFunctionsInsertion, relocation: ProgramFunctionsRelocation, modification: ProgramFunctionsPatchEntry, row: Function, patch: FunctionPatch, list: Vec<Function>, key: String = |row| row.header.id.0.clone() }
}

protocol::list_delta! {
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(test, serde(rename_all = "camelCase"))]
    /// 🧩 Positional row delta of `elements`: rows removed at their base index, inserted at their after index, moved, and modified by sparse patch.
    pub ProgramElementsDelta { removal: ProgramElementsRemoval, insertion: ProgramElementsInsertion, relocation: ProgramElementsRelocation, modification: ProgramElementsPatchEntry, row: ProgramElement, patch: ProgramElementPatch, list: Vec<ProgramElement>, key: String = |row| row.header.id.0.clone() }
}

protocol::list_delta! {
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(test, serde(rename_all = "camelCase"))]
    /// 🧩 Positional row delta of `quantities`: rows removed at their base index, inserted at their after index, moved, and modified by sparse patch.
    pub ProgramQuantitiesDelta { removal: ProgramQuantitiesRemoval, insertion: ProgramQuantitiesInsertion, relocation: ProgramQuantitiesRelocation, modification: ProgramQuantitiesPatchEntry, row: QuantityRequirement, patch: QuantityRequirementPatch, list: Vec<QuantityRequirement>, key: String = |row| row.header.id.0.clone() }
}

protocol::list_delta! {
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(test, serde(rename_all = "camelCase"))]
    /// 🧩 Positional row delta of `relationships`: rows removed at their base index, inserted at their after index, moved, and modified by sparse patch.
    pub ProgramRelationshipsDelta { removal: ProgramRelationshipsRemoval, insertion: ProgramRelationshipsInsertion, relocation: ProgramRelationshipsRelocation, modification: ProgramRelationshipsPatchEntry, row: Relationship, patch: RelationshipPatch, list: Vec<Relationship>, key: String = |row| row.header.id.0.clone() }
}

protocol::list_delta! {
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(test, serde(rename_all = "camelCase"))]
    /// 🧩 Positional row delta of `adjacencies`: rows removed at their base index, inserted at their after index, moved, and modified by sparse patch.
    pub ProgramAdjacenciesDelta { removal: ProgramAdjacenciesRemoval, insertion: ProgramAdjacenciesInsertion, relocation: ProgramAdjacenciesRelocation, modification: ProgramAdjacenciesPatchEntry, row: Adjacency, patch: AdjacencyPatch, list: Vec<Adjacency>, key: String = |row| row.header.id.0.clone() }
}

protocol::list_delta! {
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(test, serde(rename_all = "camelCase"))]
    /// 🧩 Positional row delta of `processes`: rows removed at their base index, inserted at their after index, moved, and modified by sparse patch.
    pub ProgramProcessesDelta { removal: ProgramProcessesRemoval, insertion: ProgramProcessesInsertion, relocation: ProgramProcessesRelocation, modification: ProgramProcessesPatchEntry, row: Process, patch: ProcessPatch, list: Vec<Process>, key: String = |row| row.header.id.0.clone() }
}

protocol::list_delta! {
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(test, serde(rename_all = "camelCase"))]
    /// 🧩 Positional row delta of `flows`: rows removed at their base index, inserted at their after index, moved, and modified by sparse patch.
    pub ProgramFlowsDelta { removal: ProgramFlowsRemoval, insertion: ProgramFlowsInsertion, relocation: ProgramFlowsRelocation, modification: ProgramFlowsPatchEntry, row: FlowRequirement, patch: FlowRequirementPatch, list: Vec<FlowRequirement>, key: String = |row| row.header.id.0.clone() }
}

protocol::list_delta! {
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(test, serde(rename_all = "camelCase"))]
    /// 🧩 Positional row delta of `access_rules`: rows removed at their base index, inserted at their after index, moved, and modified by sparse patch.
    pub ProgramAccessRulesDelta { removal: ProgramAccessRulesRemoval, insertion: ProgramAccessRulesInsertion, relocation: ProgramAccessRulesRelocation, modification: ProgramAccessRulesPatchEntry, row: AccessRule, patch: AccessRulePatch, list: Vec<AccessRule>, key: String = |row| row.header.id.0.clone() }
}

protocol::list_delta! {
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(test, serde(rename_all = "camelCase"))]
    /// 🧩 Positional row delta of `operations`: rows removed at their base index, inserted at their after index, moved, and modified by sparse patch.
    pub ProgramOperationsDelta { removal: ProgramOperationsRemoval, insertion: ProgramOperationsInsertion, relocation: ProgramOperationsRelocation, modification: ProgramOperationsPatchEntry, row: OperationalRequirement, patch: OperationalRequirementPatch, list: Vec<OperationalRequirement>, key: String = |row| row.header.id.0.clone() }
}

protocol::list_delta! {
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(test, serde(rename_all = "camelCase"))]
    /// 🧩 Positional row delta of `equipment`: rows removed at their base index, inserted at their after index, moved, and modified by sparse patch.
    pub ProgramEquipmentDelta { removal: ProgramEquipmentRemoval, insertion: ProgramEquipmentInsertion, relocation: ProgramEquipmentRelocation, modification: ProgramEquipmentPatchEntry, row: Equipment, patch: EquipmentPatch, list: Vec<Equipment>, key: String = |row| row.header.id.0.clone() }
}

protocol::list_delta! {
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(test, serde(rename_all = "camelCase"))]
    /// 🧩 Positional row delta of `resources`: rows removed at their base index, inserted at their after index, moved, and modified by sparse patch.
    pub ProgramResourcesDelta { removal: ProgramResourcesRemoval, insertion: ProgramResourcesInsertion, relocation: ProgramResourcesRelocation, modification: ProgramResourcesPatchEntry, row: Resource, patch: ResourcePatch, list: Vec<Resource>, key: String = |row| row.header.id.0.clone() }
}

protocol::list_delta! {
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(test, serde(rename_all = "camelCase"))]
    /// 🧩 Positional row delta of `storage`: rows removed at their base index, inserted at their after index, moved, and modified by sparse patch.
    pub ProgramStorageDelta { removal: ProgramStorageRemoval, insertion: ProgramStorageInsertion, relocation: ProgramStorageRelocation, modification: ProgramStoragePatchEntry, row: StorageRequirement, patch: StorageRequirementPatch, list: Vec<StorageRequirement>, key: String = |row| row.header.id.0.clone() }
}

protocol::list_delta! {
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(test, serde(rename_all = "camelCase"))]
    /// 🧩 Positional row delta of `environmental`: rows removed at their base index, inserted at their after index, moved, and modified by sparse patch.
    pub ProgramEnvironmentalDelta { removal: ProgramEnvironmentalRemoval, insertion: ProgramEnvironmentalInsertion, relocation: ProgramEnvironmentalRelocation, modification: ProgramEnvironmentalPatchEntry, row: EnvironmentalRequirement, patch: EnvironmentalRequirementPatch, list: Vec<EnvironmentalRequirement>, key: String = |row| row.header.id.0.clone() }
}

protocol::list_delta! {
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(test, serde(rename_all = "camelCase"))]
    /// 🧩 Positional row delta of `human_factors`: rows removed at their base index, inserted at their after index, moved, and modified by sparse patch.
    pub ProgramHumanFactorsDelta { removal: ProgramHumanFactorsRemoval, insertion: ProgramHumanFactorsInsertion, relocation: ProgramHumanFactorsRelocation, modification: ProgramHumanFactorsPatchEntry, row: HumanFactorRequirement, patch: HumanFactorRequirementPatch, list: Vec<HumanFactorRequirement>, key: String = |row| row.header.id.0.clone() }
}

protocol::list_delta! {
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(test, serde(rename_all = "camelCase"))]
    /// 🧩 Positional row delta of `accessibility`: rows removed at their base index, inserted at their after index, moved, and modified by sparse patch.
    pub ProgramAccessibilityDelta { removal: ProgramAccessibilityRemoval, insertion: ProgramAccessibilityInsertion, relocation: ProgramAccessibilityRelocation, modification: ProgramAccessibilityPatchEntry, row: AccessibilityRequirement, patch: AccessibilityRequirementPatch, list: Vec<AccessibilityRequirement>, key: String = |row| row.header.id.0.clone() }
}

protocol::list_delta! {
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(test, serde(rename_all = "camelCase"))]
    /// 🧩 Positional row delta of `privacy`: rows removed at their base index, inserted at their after index, moved, and modified by sparse patch.
    pub ProgramPrivacyDelta { removal: ProgramPrivacyRemoval, insertion: ProgramPrivacyInsertion, relocation: ProgramPrivacyRelocation, modification: ProgramPrivacyPatchEntry, row: PrivacyRequirement, patch: PrivacyRequirementPatch, list: Vec<PrivacyRequirement>, key: String = |row| row.header.id.0.clone() }
}

protocol::list_delta! {
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(test, serde(rename_all = "camelCase"))]
    /// 🧩 Positional row delta of `safety`: rows removed at their base index, inserted at their after index, moved, and modified by sparse patch.
    pub ProgramSafetyDelta { removal: ProgramSafetyRemoval, insertion: ProgramSafetyInsertion, relocation: ProgramSafetyRelocation, modification: ProgramSafetyPatchEntry, row: SafetyRequirement, patch: SafetyRequirementPatch, list: Vec<SafetyRequirement>, key: String = |row| row.header.id.0.clone() }
}

protocol::list_delta! {
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(test, serde(rename_all = "camelCase"))]
    /// 🧩 Positional row delta of `security`: rows removed at their base index, inserted at their after index, moved, and modified by sparse patch.
    pub ProgramSecurityDelta { removal: ProgramSecurityRemoval, insertion: ProgramSecurityInsertion, relocation: ProgramSecurityRelocation, modification: ProgramSecurityPatchEntry, row: SecurityRequirement, patch: SecurityRequirementPatch, list: Vec<SecurityRequirement>, key: String = |row| row.header.id.0.clone() }
}

protocol::list_delta! {
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(test, serde(rename_all = "camelCase"))]
    /// 🧩 Positional row delta of `regulatory`: rows removed at their base index, inserted at their after index, moved, and modified by sparse patch.
    pub ProgramRegulatoryDelta { removal: ProgramRegulatoryRemoval, insertion: ProgramRegulatoryInsertion, relocation: ProgramRegulatoryRelocation, modification: ProgramRegulatoryPatchEntry, row: RegulatoryRequirement, patch: RegulatoryRequirementPatch, list: Vec<RegulatoryRequirement>, key: String = |row| row.header.id.0.clone() }
}

protocol::list_delta! {
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(test, serde(rename_all = "camelCase"))]
    /// 🧩 Positional row delta of `site_context`: rows removed at their base index, inserted at their after index, moved, and modified by sparse patch.
    pub ProgramSiteContextDelta { removal: ProgramSiteContextRemoval, insertion: ProgramSiteContextInsertion, relocation: ProgramSiteContextRelocation, modification: ProgramSiteContextPatchEntry, row: SiteContext, patch: SiteContextPatch, list: Vec<SiteContext>, key: String = |row| row.header.id.0.clone() }
}

protocol::list_delta! {
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(test, serde(rename_all = "camelCase"))]
    /// 🧩 Positional row delta of `organizational`: rows removed at their base index, inserted at their after index, moved, and modified by sparse patch.
    pub ProgramOrganizationalDelta { removal: ProgramOrganizationalRemoval, insertion: ProgramOrganizationalInsertion, relocation: ProgramOrganizationalRelocation, modification: ProgramOrganizationalPatchEntry, row: OrganizationalRequirement, patch: OrganizationalRequirementPatch, list: Vec<OrganizationalRequirement>, key: String = |row| row.header.id.0.clone() }
}

protocol::list_delta! {
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(test, serde(rename_all = "camelCase"))]
    /// 🧩 Positional row delta of `services`: rows removed at their base index, inserted at their after index, moved, and modified by sparse patch.
    pub ProgramServicesDelta { removal: ProgramServicesRemoval, insertion: ProgramServicesInsertion, relocation: ProgramServicesRelocation, modification: ProgramServicesPatchEntry, row: ServiceRequirement, patch: ServiceRequirementPatch, list: Vec<ServiceRequirement>, key: String = |row| row.header.id.0.clone() }
}

protocol::list_delta! {
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(test, serde(rename_all = "camelCase"))]
    /// 🧩 Positional row delta of `infrastructure`: rows removed at their base index, inserted at their after index, moved, and modified by sparse patch.
    pub ProgramInfrastructureDelta { removal: ProgramInfrastructureRemoval, insertion: ProgramInfrastructureInsertion, relocation: ProgramInfrastructureRelocation, modification: ProgramInfrastructurePatchEntry, row: InfrastructureRequirement, patch: InfrastructureRequirementPatch, list: Vec<InfrastructureRequirement>, key: String = |row| row.header.id.0.clone() }
}

protocol::list_delta! {
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(test, serde(rename_all = "camelCase"))]
    /// 🧩 Positional row delta of `information`: rows removed at their base index, inserted at their after index, moved, and modified by sparse patch.
    pub ProgramInformationDelta { removal: ProgramInformationRemoval, insertion: ProgramInformationInsertion, relocation: ProgramInformationRelocation, modification: ProgramInformationPatchEntry, row: InformationRequirement, patch: InformationRequirementPatch, list: Vec<InformationRequirement>, key: String = |row| row.header.id.0.clone() }
}

protocol::list_delta! {
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(test, serde(rename_all = "camelCase"))]
    /// 🧩 Positional row delta of `communication`: rows removed at their base index, inserted at their after index, moved, and modified by sparse patch.
    pub ProgramCommunicationDelta { removal: ProgramCommunicationRemoval, insertion: ProgramCommunicationInsertion, relocation: ProgramCommunicationRelocation, modification: ProgramCommunicationPatchEntry, row: CommunicationRequirement, patch: CommunicationRequirementPatch, list: Vec<CommunicationRequirement>, key: String = |row| row.header.id.0.clone() }
}

protocol::list_delta! {
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(test, serde(rename_all = "camelCase"))]
    /// 🧩 Positional row delta of `wayfinding`: rows removed at their base index, inserted at their after index, moved, and modified by sparse patch.
    pub ProgramWayfindingDelta { removal: ProgramWayfindingRemoval, insertion: ProgramWayfindingInsertion, relocation: ProgramWayfindingRelocation, modification: ProgramWayfindingPatchEntry, row: WayfindingRequirement, patch: WayfindingRequirementPatch, list: Vec<WayfindingRequirement>, key: String = |row| row.header.id.0.clone() }
}

protocol::list_delta! {
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(test, serde(rename_all = "camelCase"))]
    /// 🧩 Positional row delta of `schedules`: rows removed at their base index, inserted at their after index, moved, and modified by sparse patch.
    pub ProgramSchedulesDelta { removal: ProgramSchedulesRemoval, insertion: ProgramSchedulesInsertion, relocation: ProgramSchedulesRelocation, modification: ProgramSchedulesPatchEntry, row: ScheduleRequirement, patch: ScheduleRequirementPatch, list: Vec<ScheduleRequirement>, key: String = |row| row.header.id.0.clone() }
}

protocol::list_delta! {
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(test, serde(rename_all = "camelCase"))]
    /// 🧩 Positional row delta of `flexibility`: rows removed at their base index, inserted at their after index, moved, and modified by sparse patch.
    pub ProgramFlexibilityDelta { removal: ProgramFlexibilityRemoval, insertion: ProgramFlexibilityInsertion, relocation: ProgramFlexibilityRelocation, modification: ProgramFlexibilityPatchEntry, row: FlexibilityRequirement, patch: FlexibilityRequirementPatch, list: Vec<FlexibilityRequirement>, key: String = |row| row.header.id.0.clone() }
}

protocol::list_delta! {
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(test, serde(rename_all = "camelCase"))]
    /// 🧩 Positional row delta of `growth`: rows removed at their base index, inserted at their after index, moved, and modified by sparse patch.
    pub ProgramGrowthDelta { removal: ProgramGrowthRemoval, insertion: ProgramGrowthInsertion, relocation: ProgramGrowthRelocation, modification: ProgramGrowthPatchEntry, row: GrowthPlan, patch: GrowthPlanPatch, list: Vec<GrowthPlan>, key: String = |row| row.header.id.0.clone() }
}

protocol::list_delta! {
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(test, serde(rename_all = "camelCase"))]
    /// 🧩 Positional row delta of `sustainability`: rows removed at their base index, inserted at their after index, moved, and modified by sparse patch.
    pub ProgramSustainabilityDelta { removal: ProgramSustainabilityRemoval, insertion: ProgramSustainabilityInsertion, relocation: ProgramSustainabilityRelocation, modification: ProgramSustainabilityPatchEntry, row: SustainabilityRequirement, patch: SustainabilityRequirementPatch, list: Vec<SustainabilityRequirement>, key: String = |row| row.header.id.0.clone() }
}

protocol::list_delta! {
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(test, serde(rename_all = "camelCase"))]
    /// 🧩 Positional row delta of `resilience`: rows removed at their base index, inserted at their after index, moved, and modified by sparse patch.
    pub ProgramResilienceDelta { removal: ProgramResilienceRemoval, insertion: ProgramResilienceInsertion, relocation: ProgramResilienceRelocation, modification: ProgramResiliencePatchEntry, row: ResilienceRequirement, patch: ResilienceRequirementPatch, list: Vec<ResilienceRequirement>, key: String = |row| row.header.id.0.clone() }
}

protocol::list_delta! {
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(test, serde(rename_all = "camelCase"))]
    /// 🧩 Positional row delta of `costs`: rows removed at their base index, inserted at their after index, moved, and modified by sparse patch.
    pub ProgramCostsDelta { removal: ProgramCostsRemoval, insertion: ProgramCostsInsertion, relocation: ProgramCostsRelocation, modification: ProgramCostsPatchEntry, row: CostRequirement, patch: CostRequirementPatch, list: Vec<CostRequirement>, key: String = |row| row.header.id.0.clone() }
}

protocol::list_delta! {
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(test, serde(rename_all = "camelCase"))]
    /// 🧩 Positional row delta of `delivery`: rows removed at their base index, inserted at their after index, moved, and modified by sparse patch.
    pub ProgramDeliveryDelta { removal: ProgramDeliveryRemoval, insertion: ProgramDeliveryInsertion, relocation: ProgramDeliveryRelocation, modification: ProgramDeliveryPatchEntry, row: DeliveryConstraint, patch: DeliveryConstraintPatch, list: Vec<DeliveryConstraint>, key: String = |row| row.header.id.0.clone() }
}

protocol::list_delta! {
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(test, serde(rename_all = "camelCase"))]
    /// 🧩 Positional row delta of `risks`: rows removed at their base index, inserted at their after index, moved, and modified by sparse patch.
    pub ProgramRisksDelta { removal: ProgramRisksRemoval, insertion: ProgramRisksInsertion, relocation: ProgramRisksRelocation, modification: ProgramRisksPatchEntry, row: Risk, patch: RiskPatch, list: Vec<Risk>, key: String = |row| row.header.id.0.clone() }
}

protocol::list_delta! {
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(test, serde(rename_all = "camelCase"))]
    /// 🧩 Positional row delta of `conflicts`: rows removed at their base index, inserted at their after index, moved, and modified by sparse patch.
    pub ProgramConflictsDelta { removal: ProgramConflictsRemoval, insertion: ProgramConflictsInsertion, relocation: ProgramConflictsRelocation, modification: ProgramConflictsPatchEntry, row: Conflict, patch: ConflictPatch, list: Vec<Conflict>, key: String = |row| row.header.id.0.clone() }
}

protocol::list_delta! {
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(test, serde(rename_all = "camelCase"))]
    /// 🧩 Positional row delta of `requirements`: rows removed at their base index, inserted at their after index, moved, and modified by sparse patch.
    pub ProgramRequirementsDelta { removal: ProgramRequirementsRemoval, insertion: ProgramRequirementsInsertion, relocation: ProgramRequirementsRelocation, modification: ProgramRequirementsPatchEntry, row: Requirement, patch: RequirementPatch, list: Vec<Requirement>, key: String = |row| row.header.id.0.clone() }
}

protocol::list_delta! {
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(test, serde(rename_all = "camelCase"))]
    /// 🧩 Positional row delta of `priorities`: rows removed at their base index, inserted at their after index, moved, and modified by sparse patch.
    pub ProgramPrioritiesDelta { removal: ProgramPrioritiesRemoval, insertion: ProgramPrioritiesInsertion, relocation: ProgramPrioritiesRelocation, modification: ProgramPrioritiesPatchEntry, row: PriorityRecord, patch: PriorityRecordPatch, list: Vec<PriorityRecord>, key: String = |row| row.header.id.0.clone() }
}

protocol::list_delta! {
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(test, serde(rename_all = "camelCase"))]
    /// 🧩 Positional row delta of `scenarios`: rows removed at their base index, inserted at their after index, moved, and modified by sparse patch.
    pub ProgramScenariosDelta { removal: ProgramScenariosRemoval, insertion: ProgramScenariosInsertion, relocation: ProgramScenariosRelocation, modification: ProgramScenariosPatchEntry, row: Scenario, patch: ScenarioPatch, list: Vec<Scenario>, key: String = |row| row.header.id.0.clone() }
}

protocol::list_delta! {
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(test, serde(rename_all = "camelCase"))]
    /// 🧩 Positional row delta of `options`: rows removed at their base index, inserted at their after index, moved, and modified by sparse patch.
    pub ProgramOptionsDelta { removal: ProgramOptionsRemoval, insertion: ProgramOptionsInsertion, relocation: ProgramOptionsRelocation, modification: ProgramOptionsPatchEntry, row: OptionEvaluation, patch: OptionEvaluationPatch, list: Vec<OptionEvaluation>, key: String = |row| row.header.id.0.clone() }
}

protocol::list_delta! {
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(test, serde(rename_all = "camelCase"))]
    /// 🧩 Positional row delta of `decisions`: rows removed at their base index, inserted at their after index, moved, and modified by sparse patch.
    pub ProgramDecisionsDelta { removal: ProgramDecisionsRemoval, insertion: ProgramDecisionsInsertion, relocation: ProgramDecisionsRelocation, modification: ProgramDecisionsPatchEntry, row: Decision, patch: DecisionPatch, list: Vec<Decision>, key: String = |row| row.header.id.0.clone() }
}

protocol::list_delta! {
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(test, serde(rename_all = "camelCase"))]
    /// 🧩 Positional row delta of `validations`: rows removed at their base index, inserted at their after index, moved, and modified by sparse patch.
    pub ProgramValidationsDelta { removal: ProgramValidationsRemoval, insertion: ProgramValidationsInsertion, relocation: ProgramValidationsRelocation, modification: ProgramValidationsPatchEntry, row: ValidationRecord, patch: ValidationRecordPatch, list: Vec<ValidationRecord>, key: String = |row| row.header.id.0.clone() }
}

protocol::list_delta! {
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(test, serde(rename_all = "camelCase"))]
    /// 🧩 Positional row delta of `performance`: rows removed at their base index, inserted at their after index, moved, and modified by sparse patch.
    pub ProgramPerformanceDelta { removal: ProgramPerformanceRemoval, insertion: ProgramPerformanceInsertion, relocation: ProgramPerformanceRelocation, modification: ProgramPerformancePatchEntry, row: PerformanceCriterion, patch: PerformanceCriterionPatch, list: Vec<PerformanceCriterion>, key: String = |row| row.header.id.0.clone() }
}

protocol::list_delta! {
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(test, serde(rename_all = "camelCase"))]
    /// 🧩 Positional row delta of `quality`: rows removed at their base index, inserted at their after index, moved, and modified by sparse patch.
    pub ProgramQualityDelta { removal: ProgramQualityRemoval, insertion: ProgramQualityInsertion, relocation: ProgramQualityRelocation, modification: ProgramQualityPatchEntry, row: QualityRecord, patch: QualityRecordPatch, list: Vec<QualityRecord>, key: String = |row| row.header.id.0.clone() }
}

protocol::list_delta! {
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(test, serde(rename_all = "camelCase"))]
    /// 🧩 Positional row delta of `artifacts`: rows removed at their base index, inserted at their after index, moved, and modified by sparse patch.
    pub ProgramArtifactsDelta { removal: ProgramArtifactsRemoval, insertion: ProgramArtifactsInsertion, relocation: ProgramArtifactsRelocation, modification: ProgramArtifactsPatchEntry, row: ArtifactRecord, patch: ArtifactRecordPatch, list: Vec<ArtifactRecord>, key: String = |row| row.header.id.0.clone() }
}

protocol::list_delta! {
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(test, serde(rename_all = "camelCase"))]
    /// 🧩 Positional row delta of `assumptions`: rows removed at their base index, inserted at their after index, moved, and modified by sparse patch.
    pub ProgramAssumptionsDelta { removal: ProgramAssumptionsRemoval, insertion: ProgramAssumptionsInsertion, relocation: ProgramAssumptionsRelocation, modification: ProgramAssumptionsPatchEntry, row: Assumption, patch: AssumptionPatch, list: Vec<Assumption>, key: String = |row| row.header.id.0.clone() }
}

protocol::list_delta! {
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(test, serde(rename_all = "camelCase"))]
    /// 🧩 Positional row delta of `constraints`: rows removed at their base index, inserted at their after index, moved, and modified by sparse patch.
    pub ProgramConstraintsDelta { removal: ProgramConstraintsRemoval, insertion: ProgramConstraintsInsertion, relocation: ProgramConstraintsRelocation, modification: ProgramConstraintsPatchEntry, row: ConstraintRecord, patch: ConstraintRecordPatch, list: Vec<ConstraintRecord>, key: String = |row| row.header.id.0.clone() }
}

protocol::list_delta! {
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(test, serde(rename_all = "camelCase"))]
    /// 🧩 Positional row delta of `compliance_records`: rows removed at their base index, inserted at their after index, moved, and modified by sparse patch.
    pub ProgramComplianceRecordsDelta { removal: ProgramComplianceRecordsRemoval, insertion: ProgramComplianceRecordsInsertion, relocation: ProgramComplianceRecordsRelocation, modification: ProgramComplianceRecordsPatchEntry, row: ComplianceRecord, patch: ComplianceRecordPatch, list: Vec<ComplianceRecord>, key: String = |row| row.header.id.0.clone() }
}

protocol::list_delta! {
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(test, serde(rename_all = "camelCase"))]
    /// 🧩 Positional row delta of `approvals`: rows removed at their base index, inserted at their after index, moved, and modified by sparse patch.
    pub ProgramApprovalsDelta { removal: ProgramApprovalsRemoval, insertion: ProgramApprovalsInsertion, relocation: ProgramApprovalsRelocation, modification: ProgramApprovalsPatchEntry, row: ApprovalRecord, patch: ApprovalRecordPatch, list: Vec<ApprovalRecord>, key: String = |row| row.header.id.0.clone() }
}

protocol::list_delta! {
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(test, serde(rename_all = "camelCase"))]
    /// 🧩 Positional row delta of `meetings`: rows removed at their base index, inserted at their after index, moved, and modified by sparse patch.
    pub ProgramMeetingsDelta { removal: ProgramMeetingsRemoval, insertion: ProgramMeetingsInsertion, relocation: ProgramMeetingsRelocation, modification: ProgramMeetingsPatchEntry, row: MeetingRecord, patch: MeetingRecordPatch, list: Vec<MeetingRecord>, key: String = |row| row.header.id.0.clone() }
}

protocol::list_delta! {
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(test, serde(rename_all = "camelCase"))]
    /// 🧩 Positional row delta of `changes`: rows removed at their base index, inserted at their after index, moved, and modified by sparse patch.
    pub ProgramChangesDelta { removal: ProgramChangesRemoval, insertion: ProgramChangesInsertion, relocation: ProgramChangesRelocation, modification: ProgramChangesPatchEntry, row: ChangeRecord, patch: ChangeRecordPatch, list: Vec<ChangeRecord>, key: String = |row| row.header.id.0.clone() }
}

protocol::list_delta! {
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(test, serde(rename_all = "camelCase"))]
    /// 🧩 Positional row delta of `collaboration`: rows removed at their base index, inserted at their after index, moved, and modified by sparse patch.
    pub ProgramCollaborationDelta { removal: ProgramCollaborationRemoval, insertion: ProgramCollaborationInsertion, relocation: ProgramCollaborationRelocation, modification: ProgramCollaborationPatchEntry, row: CollaborationRecord, patch: CollaborationRecordPatch, list: Vec<CollaborationRecord>, key: String = |row| row.header.id.0.clone() }
}

protocol::list_delta! {
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(test, serde(rename_all = "camelCase"))]
    /// 🧩 Positional row delta of `analyses`: rows removed at their base index, inserted at their after index, moved, and modified by sparse patch.
    pub ProgramAnalysesDelta { removal: ProgramAnalysesRemoval, insertion: ProgramAnalysesInsertion, relocation: ProgramAnalysesRelocation, modification: ProgramAnalysesPatchEntry, row: AnalysisRecord, patch: AnalysisRecordPatch, list: Vec<AnalysisRecord>, key: String = |row| row.header.id.0.clone() }
}

protocol::list_delta! {
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(test, serde(rename_all = "camelCase"))]
    /// 🧩 Positional row delta of `reports`: rows removed at their base index, inserted at their after index, moved, and modified by sparse patch.
    pub ProgramReportsDelta { removal: ProgramReportsRemoval, insertion: ProgramReportsInsertion, relocation: ProgramReportsRelocation, modification: ProgramReportsPatchEntry, row: ReportRecord, patch: ReportRecordPatch, list: Vec<ReportRecord>, key: String = |row| row.header.id.0.clone() }
}

protocol::list_delta! {
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(test, serde(rename_all = "camelCase"))]
    /// 🧩 Positional row delta of `search_filters`: rows removed at their base index, inserted at their after index, moved, and modified by sparse patch.
    pub ProgramSearchFiltersDelta { removal: ProgramSearchFiltersRemoval, insertion: ProgramSearchFiltersInsertion, relocation: ProgramSearchFiltersRelocation, modification: ProgramSearchFiltersPatchEntry, row: SearchFilter, patch: SearchFilterPatch, list: Vec<SearchFilter>, key: String = |row| row.header.id.0.clone() }
}

protocol::list_delta! {
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(test, serde(rename_all = "camelCase"))]
    /// 🧩 Positional row delta of `status_records`: rows removed at their base index, inserted at their after index, moved, and modified by sparse patch.
    pub ProgramStatusRecordsDelta { removal: ProgramStatusRecordsRemoval, insertion: ProgramStatusRecordsInsertion, relocation: ProgramStatusRecordsRelocation, modification: ProgramStatusRecordsPatchEntry, row: StatusRecord, patch: StatusRecordPatch, list: Vec<StatusRecord>, key: String = |row| row.header.id.0.clone() }
}

protocol::list_delta! {
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(test, serde(rename_all = "camelCase"))]
    /// 🧩 Positional row delta of `workshops`: rows removed at their base index, inserted at their after index, moved, and modified by sparse patch.
    pub ProgramWorkshopsDelta { removal: ProgramWorkshopsRemoval, insertion: ProgramWorkshopsInsertion, relocation: ProgramWorkshopsRelocation, modification: ProgramWorkshopsPatchEntry, row: Workshop, patch: WorkshopPatch, list: Vec<Workshop>, key: String = |row| row.header.id.0.clone() }
}

protocol::list_delta! {
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(test, serde(rename_all = "camelCase"))]
    /// 🧩 Positional row delta of `surveys`: rows removed at their base index, inserted at their after index, moved, and modified by sparse patch.
    pub ProgramSurveysDelta { removal: ProgramSurveysRemoval, insertion: ProgramSurveysInsertion, relocation: ProgramSurveysRelocation, modification: ProgramSurveysPatchEntry, row: Survey, patch: SurveyPatch, list: Vec<Survey>, key: String = |row| row.header.id.0.clone() }
}

protocol::list_delta! {
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(test, serde(rename_all = "camelCase"))]
    /// 🧩 Positional row delta of `issues`: rows removed at their base index, inserted at their after index, moved, and modified by sparse patch.
    pub ProgramIssuesDelta { removal: ProgramIssuesRemoval, insertion: ProgramIssuesInsertion, relocation: ProgramIssuesRelocation, modification: ProgramIssuesPatchEntry, row: Issue, patch: IssuePatch, list: Vec<Issue>, key: String = |row| row.header.id.0.clone() }
}

protocol::list_delta! {
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(test, serde(rename_all = "camelCase"))]
    /// 🧩 Positional row delta of `audit_events`: rows removed at their base index, inserted at their after index, moved, and modified by sparse patch.
    pub ProgramAuditEventsDelta { removal: ProgramAuditEventsRemoval, insertion: ProgramAuditEventsInsertion, relocation: ProgramAuditEventsRelocation, modification: ProgramAuditEventsPatchEntry, row: AuditEvent, patch: AuditEventPatch, list: Vec<AuditEvent>, key: String = |row| row.header.id.0.clone() }
}

protocol::list_delta! {
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(test, serde(rename_all = "camelCase"))]
    /// 🧩 Positional row delta of `templates`: rows removed at their base index, inserted at their after index, moved, and modified by sparse patch.
    pub ProgramTemplatesDelta { removal: ProgramTemplatesRemoval, insertion: ProgramTemplatesInsertion, relocation: ProgramTemplatesRelocation, modification: ProgramTemplatesPatchEntry, row: TemplateRecord, patch: TemplateRecordPatch, list: Vec<TemplateRecord>, key: String = |row| row.header.id.0.clone() }
}

protocol::list_delta! {
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(test, serde(rename_all = "camelCase"))]
    /// 🧩 Positional row delta of `traces`: rows removed at their base index, inserted at their after index, moved, and modified by sparse patch.
    pub ProgramTracesDelta { removal: ProgramTracesRemoval, insertion: ProgramTracesInsertion, relocation: ProgramTracesRelocation, modification: ProgramTracesPatchEntry, row: TraceLink, patch: TraceLinkPatch, list: Vec<TraceLink>, key: String = |row| row.id.0.clone() }
}

protocol::list_delta! {
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(test, serde(rename_all = "camelCase"))]
    /// 🧩 Positional row delta of `knowledge`: rows removed at their base index, inserted at their after index, moved, and modified by sparse patch.
    pub ProgramKnowledgeDelta { removal: ProgramKnowledgeRemoval, insertion: ProgramKnowledgeInsertion, relocation: ProgramKnowledgeRelocation, modification: ProgramKnowledgePatchEntry, row: KnowledgeRecord, patch: KnowledgeRecordPatch, list: Vec<KnowledgeRecord>, key: String = |row| row.header.id.0.clone() }
}

protocol::list_delta! {
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(test, serde(rename_all = "camelCase"))]
    /// 🧩 Positional row delta of `benchmarks`: rows removed at their base index, inserted at their after index, moved, and modified by sparse patch.
    pub ProgramBenchmarksDelta { removal: ProgramBenchmarksRemoval, insertion: ProgramBenchmarksInsertion, relocation: ProgramBenchmarksRelocation, modification: ProgramBenchmarksPatchEntry, row: BenchmarkRecord, patch: BenchmarkRecordPatch, list: Vec<BenchmarkRecord>, key: String = |row| row.header.id.0.clone() }
}

/// ✏️ Edit of the `meta` section: `set` replaces the whole section (a `replace-meta` kind), `patch` then rewrites only the
/// patched fields (a `rename-meta` kind); a patch cannot clear an optional field, so a clear travels as `set`.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", default, deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
pub struct ProgramMetaEdit {
    pub set: Option<ProgramMeta>,
    pub patch: Option<ProgramMetaPatch>,
}

/// ✏️ Edit of the `project` section: `set` replaces the whole section (a `replace-project` kind), `patch` then rewrites only the
/// patched fields (a `rename-project` kind); a patch cannot clear an optional field, so a clear travels as `set`.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", default, deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
pub struct ProjectDefinitionEdit {
    pub set: Option<ProjectDefinition>,
    pub patch: Option<ProjectDefinitionPatch>,
}

/// ✏️ Edit of the `governance` section: `set` replaces the whole section (a `replace-governance` kind), `patch` then rewrites only the
/// patched fields (a `rename-governance` kind); a patch cannot clear an optional field, so a clear travels as `set`.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", default, deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
pub struct GovernanceEdit {
    pub set: Option<Governance>,
    pub patch: Option<GovernancePatch>,
}

//#endregion 🔖️DeltaHelpers

use crate::ProgramSnapshot;
use protocol::list_delta::RowPatch;
use protocol::{ApplyCapability, DiffAlgebra, MutationApplyResult, MutationDiff};

//#region 🔖️SectionEdits
macro_rules! impl_section_edit {
    ($edit:ty, $value:ty, $patch:ty) => {
        impl $edit {
            /// 🎯️ A whole-section replacement.
            pub fn replacing(value: $value) -> Self {
                Self { set: Some(value), patch: None }
            }

            /// 🩹 A sparse field patch.
            pub fn patching(patch: $patch) -> Self {
                Self { set: None, patch: Some(patch) }
            }

            /// 🔑️ Writes the edit into `value` under the capability: the replacement first, then the patched fields.
            fn write(&self, value: &mut $value, capability: ApplyCapability) -> MutationApplyResult<()> {
                if let Some(set) = &self.set {
                    *value = set.clone();
                }
                if let Some(patch) = &self.patch {
                    patch.commit_into(value, capability)?;
                }
                Ok(())
            }

            /// ➕️ Composes this edit with a later one: a later replacement wins, a later patch merges into the patch (the replacement is applied first, the patch last).
            fn compose(mut self, later: Self) -> Self {
                if later.set.is_some() {
                    return later;
                }
                if let Some(patch) = later.patch {
                    match &mut self.patch {
                        Some(existing) => existing.absorb(patch),
                        slot => *slot = Some(patch),
                    }
                }
                self
            }

            /// 🔁️ The edit that puts the edited fields of `base` back.
            fn undo(&self, base: &$value) -> Self {
                match (&self.set, &self.patch) {
                    (None, Some(patch)) => Self::patching(patch.inverse(base)),
                    _ => Self::replacing(base.clone()),
                }
            }

            fn is_empty(&self) -> bool {
                self.set.is_none() && self.patch.as_ref().is_none_or(|patch| RowPatch::<$value>::is_empty(patch))
            }
        }
    };
}

impl_section_edit!(ProgramMetaEdit, ProgramMeta, ProgramMetaPatch);
impl_section_edit!(ProjectDefinitionEdit, ProjectDefinition, ProjectDefinitionPatch);
impl_section_edit!(GovernanceEdit, Governance, GovernancePatch);

fn compose_edit<E>(first: Option<E>, later: Option<E>, compose: impl FnOnce(E, E) -> E) -> Option<E> {
    match (first, later) {
        (Some(first), Some(later)) => Some(compose(first, later)),
        (first, None) => first,
        (None, later) => later,
    }
}
//#endregion 🔖️SectionEdits

//#region 🔖️CollectionList
/// 🗂️ Expands `$each!(context.. field "wire")` once per positional collection of the diff.
macro_rules! program_collections {
    ($each:ident, $($context:tt)*) => {
        $each!($($context)* stakeholders "stakeholders");
        $each!($($context)* users "users");
        $each!($($context)* activities "activities");
        $each!($($context)* functions "functions");
        $each!($($context)* elements "elements");
        $each!($($context)* quantities "quantities");
        $each!($($context)* relationships "relationships");
        $each!($($context)* adjacencies "adjacencies");
        $each!($($context)* processes "processes");
        $each!($($context)* flows "flows");
        $each!($($context)* access_rules "accessRules");
        $each!($($context)* operations "operations");
        $each!($($context)* equipment "equipment");
        $each!($($context)* resources "resources");
        $each!($($context)* storage "storage");
        $each!($($context)* environmental "environmental");
        $each!($($context)* human_factors "humanFactors");
        $each!($($context)* accessibility "accessibility");
        $each!($($context)* privacy "privacy");
        $each!($($context)* safety "safety");
        $each!($($context)* security "security");
        $each!($($context)* regulatory "regulatory");
        $each!($($context)* site_context "siteContext");
        $each!($($context)* organizational "organizational");
        $each!($($context)* services "services");
        $each!($($context)* infrastructure "infrastructure");
        $each!($($context)* information "information");
        $each!($($context)* communication "communication");
        $each!($($context)* wayfinding "wayfinding");
        $each!($($context)* schedules "schedules");
        $each!($($context)* flexibility "flexibility");
        $each!($($context)* growth "growth");
        $each!($($context)* sustainability "sustainability");
        $each!($($context)* resilience "resilience");
        $each!($($context)* costs "costs");
        $each!($($context)* delivery "delivery");
        $each!($($context)* risks "risks");
        $each!($($context)* conflicts "conflicts");
        $each!($($context)* requirements "requirements");
        $each!($($context)* priorities "priorities");
        $each!($($context)* scenarios "scenarios");
        $each!($($context)* options "options");
        $each!($($context)* decisions "decisions");
        $each!($($context)* validations "validations");
        $each!($($context)* performance "performance");
        $each!($($context)* quality "quality");
        $each!($($context)* artifacts "artifacts");
        $each!($($context)* assumptions "assumptions");
        $each!($($context)* constraints "constraints");
        $each!($($context)* compliance_records "complianceRecords");
        $each!($($context)* approvals "approvals");
        $each!($($context)* meetings "meetings");
        $each!($($context)* changes "changes");
        $each!($($context)* collaboration "collaboration");
        $each!($($context)* analyses "analyses");
        $each!($($context)* reports "reports");
        $each!($($context)* search_filters "searchFilters");
        $each!($($context)* status_records "statusRecords");
        $each!($($context)* workshops "workshops");
        $each!($($context)* surveys "surveys");
        $each!($($context)* issues "issues");
        $each!($($context)* audit_events "auditEvents");
        $each!($($context)* templates "templates");
        $each!($($context)* traces "traces");
    };
}
//#endregion 🔖️CollectionList

//#region 🔖️ProgramDiffAlgebra
macro_rules! commit_collection {
    ($diff:expr, $next:ident, $capability:ident, $field:ident $wire:literal) => {
        if let Some(delta) = &$diff.$field {
            $next.$field = delta.commit_onto(&$next.$field, $capability).map_err(|error| error.under([$wire]))?;
        }
    };
}

macro_rules! absorb_collection {
    ($first:expr, $later:ident, $field:ident $wire:literal) => {
        if let Some(later) = $later.$field {
            let mut merged = $first.$field.take().unwrap_or_default();
            merged.absorb(later);
            $first.$field = (!merged.is_empty()).then_some(merged);
        }
    };
}

macro_rules! inverse_collection {
    ($diff:expr, $base:ident, $undo:ident, $field:ident $wire:literal) => {
        if let Some(delta) = &$diff.$field {
            let inverse = delta.inverse(&$base.$field);
            $undo.$field = (!inverse.is_empty()).then_some(inverse);
        }
    };
}

macro_rules! empty_collection {
    ($diff:expr, $verdict:ident, $field:ident $wire:literal) => {
        $verdict &= $diff.$field.as_ref().is_none_or(|delta| delta.is_empty());
    };
}

impl MutationDiff<ProgramSnapshot> for ProgramDiff {
    fn apply(&self, base: &ProgramSnapshot, capability: ApplyCapability) -> MutationApplyResult<ProgramSnapshot> {
        let mut next = base.clone();
        if let Some(schema) = &self.schema {
            next.schema = schema.clone();
        }
        if let Some(edit) = &self.meta {
            edit.write(&mut next.meta, capability).map_err(|error| error.under(["meta"]))?;
        }
        if let Some(edit) = &self.project {
            edit.write(&mut next.project, capability).map_err(|error| error.under(["project"]))?;
        }
        if let Some(edit) = &self.governance {
            edit.write(&mut next.governance, capability).map_err(|error| error.under(["governance"]))?;
        }
        program_collections!(commit_collection, self, next, capability,);
        if let Some(delta) = &self.knowledge {
            next.knowledge_payload = delta.commit_onto(&next.knowledge_payload, capability).map_err(|error| error.under(["knowledge"]))?;
            next.knowledge = crate::knowledge_child_from_records(&next.knowledge_payload);
        }
        if let Some(delta) = &self.benchmarks {
            next.benchmarks_payload = delta.commit_onto(&next.benchmarks_payload, capability).map_err(|error| error.under(["benchmarks"]))?;
            next.benchmarks = crate::benchmarks_child_from_records(&next.benchmarks_payload);
        }
        Ok(next)
    }

    fn absorb(&mut self, other: Self) {
        if other.schema.is_some() {
            self.schema = other.schema;
        }
        self.meta = compose_edit(self.meta.take(), other.meta, ProgramMetaEdit::compose);
        self.project = compose_edit(self.project.take(), other.project, ProjectDefinitionEdit::compose);
        self.governance = compose_edit(self.governance.take(), other.governance, GovernanceEdit::compose);
        program_collections!(absorb_collection, self, other,);
        if let Some(delta) = other.knowledge {
            let mut merged = self.knowledge.take().unwrap_or_default();
            merged.absorb(delta);
            self.knowledge = (!merged.is_empty()).then_some(merged);
        }
        if let Some(delta) = other.benchmarks {
            let mut merged = self.benchmarks.take().unwrap_or_default();
            merged.absorb(delta);
            self.benchmarks = (!merged.is_empty()).then_some(merged);
        }
    }
}

impl DiffAlgebra<ProgramSnapshot> for ProgramDiff {
    fn inverse(&self, base: &ProgramSnapshot) -> Self {
        let mut undo = Self::default();
        undo.schema = self.schema.as_ref().map(|_| base.schema.clone());
        undo.meta = self.meta.as_ref().map(|edit| edit.undo(&base.meta));
        undo.project = self.project.as_ref().map(|edit| edit.undo(&base.project));
        undo.governance = self.governance.as_ref().map(|edit| edit.undo(&base.governance));
        program_collections!(inverse_collection, self, base, undo,);
        if let Some(delta) = &self.knowledge {
            let inverse = delta.inverse(&base.knowledge_payload);
            undo.knowledge = (!inverse.is_empty()).then_some(inverse);
        }
        if let Some(delta) = &self.benchmarks {
            let inverse = delta.inverse(&base.benchmarks_payload);
            undo.benchmarks = (!inverse.is_empty()).then_some(inverse);
        }
        undo
    }

    fn is_empty(&self) -> bool {
        let mut verdict = self.schema.is_none() && self.meta.as_ref().is_none_or(ProgramMetaEdit::is_empty) && self.project.as_ref().is_none_or(ProjectDefinitionEdit::is_empty) && self.governance.as_ref().is_none_or(GovernanceEdit::is_empty);
        program_collections!(empty_collection, self, verdict,);
        verdict && self.knowledge.as_ref().is_none_or(|delta| delta.is_empty()) && self.benchmarks.as_ref().is_none_or(|delta| delta.is_empty())
    }
}
//#endregion 🔖️ProgramDiffAlgebra

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
