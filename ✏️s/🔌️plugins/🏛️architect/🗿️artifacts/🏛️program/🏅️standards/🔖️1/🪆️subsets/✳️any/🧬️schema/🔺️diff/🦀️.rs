//! 🧬️ ProgramSnapshot diff schema — sparse field delta over the artifact.

use crate::kernel::*;
use crate::registers::*;
use framework_schema::ArtifactSchema;

//#region 🔖️Diff
/// 🔺️ Sparse field delta for the program artifact.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase", default, deny_unknown_fields)]
#[artifact_schema(id = "s.architect.program")]
pub struct ProgramDiff {
    #[state(artifact)]
    pub artifact: Option<Box<crate::schema::ProgramArtifact>>,
    #[state(artifact)]
    pub schema: Option<String>,
    #[state(artifact)]
    pub meta: Option<ProgramMeta>,
    #[state(artifact)]
    pub project: Option<ProjectDefinition>,
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
    /// 📚️ Replacement rows for the composed knowledge table — the persisted payload the
    /// replacement handle below was minted from; they travel together so an applied diff leaves the
    /// parent able to re-derive its child through `crate::genesis_program_child_pack`.
    #[state(artifact)]
    pub knowledge_payload: Option<Vec<crate::KnowledgeRecord>>,
    /// 🧩️ Replacement handle for the composed knowledge table.
    #[state(artifact)]
    pub knowledge: Option<crate::ProgramKnowledgeChild>,
    /// 🏁️ Replacement rows for the composed benchmarks table — see [`ProgramDiff::knowledge_payload`].
    #[state(artifact)]
    pub benchmarks_payload: Option<Vec<crate::BenchmarkRecord>>,
    /// 🧩️ Replacement handle for the composed benchmarks table.
    #[state(artifact)]
    pub benchmarks: Option<crate::ProgramBenchmarksChild>,
    #[state(artifact)]
    pub traces: Option<ProgramTracesDelta>,
    #[state(artifact)]
    pub governance: Option<Governance>,
}
//#endregion 🔖️Diff

//#region 🔖️DeltaHelpers
/// 📋 String-list wrapper so optional list diffs stay scalar across formats.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", default, deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
pub struct ProgramStringList {
    pub values: Vec<String>,
}

/// 🧩 Identified-collection delta for `stakeholders`.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", default, deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
pub struct ProgramStakeholdersDelta {
    pub added: Vec<Stakeholder>,
    pub removed: Vec<String>,
    pub patched: Vec<ProgramStakeholdersPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched `Stakeholder` entry.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct ProgramStakeholdersPatchEntry {
    pub id: String,
    pub patch: StakeholderPatch,
}

/// 🧩 Identified-collection delta for `users`.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", default, deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
pub struct ProgramUsersDelta {
    pub added: Vec<UserProfile>,
    pub removed: Vec<String>,
    pub patched: Vec<ProgramUsersPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched `UserProfile` entry.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct ProgramUsersPatchEntry {
    pub id: String,
    pub patch: UserProfilePatch,
}

/// 🧩 Identified-collection delta for `activities`.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", default, deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
pub struct ProgramActivitiesDelta {
    pub added: Vec<Activity>,
    pub removed: Vec<String>,
    pub patched: Vec<ProgramActivitiesPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched `Activity` entry.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct ProgramActivitiesPatchEntry {
    pub id: String,
    pub patch: ActivityPatch,
}

/// 🧩 Identified-collection delta for `functions`.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", default, deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
pub struct ProgramFunctionsDelta {
    pub added: Vec<Function>,
    pub removed: Vec<String>,
    pub patched: Vec<ProgramFunctionsPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched `Function` entry.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct ProgramFunctionsPatchEntry {
    pub id: String,
    pub patch: FunctionPatch,
}

/// 🧩 Identified-collection delta for `elements`.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", default, deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
pub struct ProgramElementsDelta {
    pub added: Vec<ProgramElement>,
    pub removed: Vec<String>,
    pub patched: Vec<ProgramElementsPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched `ProgramElement` entry.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct ProgramElementsPatchEntry {
    pub id: String,
    pub patch: ProgramElementPatch,
}

/// 🧩 Identified-collection delta for `quantities`.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", default, deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
pub struct ProgramQuantitiesDelta {
    pub added: Vec<QuantityRequirement>,
    pub removed: Vec<String>,
    pub patched: Vec<ProgramQuantitiesPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched `QuantityRequirement` entry.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct ProgramQuantitiesPatchEntry {
    pub id: String,
    pub patch: QuantityRequirementPatch,
}

/// 🧩 Identified-collection delta for `relationships`.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", default, deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
pub struct ProgramRelationshipsDelta {
    pub added: Vec<Relationship>,
    pub removed: Vec<String>,
    pub patched: Vec<ProgramRelationshipsPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched `Relationship` entry.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct ProgramRelationshipsPatchEntry {
    pub id: String,
    pub patch: RelationshipPatch,
}

/// 🧩 Identified-collection delta for `adjacencies`.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", default, deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
pub struct ProgramAdjacenciesDelta {
    pub added: Vec<Adjacency>,
    pub removed: Vec<String>,
    pub patched: Vec<ProgramAdjacenciesPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched `Adjacency` entry.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct ProgramAdjacenciesPatchEntry {
    pub id: String,
    pub patch: AdjacencyPatch,
}

/// 🧩 Identified-collection delta for `processes`.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", default, deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
pub struct ProgramProcessesDelta {
    pub added: Vec<Process>,
    pub removed: Vec<String>,
    pub patched: Vec<ProgramProcessesPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched `Process` entry.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct ProgramProcessesPatchEntry {
    pub id: String,
    pub patch: ProcessPatch,
}

/// 🧩 Identified-collection delta for `flows`.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", default, deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
pub struct ProgramFlowsDelta {
    pub added: Vec<FlowRequirement>,
    pub removed: Vec<String>,
    pub patched: Vec<ProgramFlowsPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched `FlowRequirement` entry.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct ProgramFlowsPatchEntry {
    pub id: String,
    pub patch: FlowRequirementPatch,
}

/// 🧩 Identified-collection delta for `access_rules`.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", default, deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
pub struct ProgramAccessRulesDelta {
    pub added: Vec<AccessRule>,
    pub removed: Vec<String>,
    pub patched: Vec<ProgramAccessRulesPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched `AccessRule` entry.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct ProgramAccessRulesPatchEntry {
    pub id: String,
    pub patch: AccessRulePatch,
}

/// 🧩 Identified-collection delta for `operations`.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", default, deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
pub struct ProgramOperationsDelta {
    pub added: Vec<OperationalRequirement>,
    pub removed: Vec<String>,
    pub patched: Vec<ProgramOperationsPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched `OperationalRequirement` entry.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct ProgramOperationsPatchEntry {
    pub id: String,
    pub patch: OperationalRequirementPatch,
}

/// 🧩 Identified-collection delta for `equipment`.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", default, deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
pub struct ProgramEquipmentDelta {
    pub added: Vec<Equipment>,
    pub removed: Vec<String>,
    pub patched: Vec<ProgramEquipmentPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched `Equipment` entry.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct ProgramEquipmentPatchEntry {
    pub id: String,
    pub patch: EquipmentPatch,
}

/// 🧩 Identified-collection delta for `resources`.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", default, deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
pub struct ProgramResourcesDelta {
    pub added: Vec<Resource>,
    pub removed: Vec<String>,
    pub patched: Vec<ProgramResourcesPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched `Resource` entry.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct ProgramResourcesPatchEntry {
    pub id: String,
    pub patch: ResourcePatch,
}

/// 🧩 Identified-collection delta for `storage`.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", default, deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
pub struct ProgramStorageDelta {
    pub added: Vec<StorageRequirement>,
    pub removed: Vec<String>,
    pub patched: Vec<ProgramStoragePatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched `StorageRequirement` entry.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct ProgramStoragePatchEntry {
    pub id: String,
    pub patch: StorageRequirementPatch,
}

/// 🧩 Identified-collection delta for `environmental`.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", default, deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
pub struct ProgramEnvironmentalDelta {
    pub added: Vec<EnvironmentalRequirement>,
    pub removed: Vec<String>,
    pub patched: Vec<ProgramEnvironmentalPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched `EnvironmentalRequirement` entry.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct ProgramEnvironmentalPatchEntry {
    pub id: String,
    pub patch: EnvironmentalRequirementPatch,
}

/// 🧩 Identified-collection delta for `human_factors`.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", default, deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
pub struct ProgramHumanFactorsDelta {
    pub added: Vec<HumanFactorRequirement>,
    pub removed: Vec<String>,
    pub patched: Vec<ProgramHumanFactorsPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched `HumanFactorRequirement` entry.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct ProgramHumanFactorsPatchEntry {
    pub id: String,
    pub patch: HumanFactorRequirementPatch,
}

/// 🧩 Identified-collection delta for `accessibility`.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", default, deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
pub struct ProgramAccessibilityDelta {
    pub added: Vec<AccessibilityRequirement>,
    pub removed: Vec<String>,
    pub patched: Vec<ProgramAccessibilityPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched `AccessibilityRequirement` entry.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct ProgramAccessibilityPatchEntry {
    pub id: String,
    pub patch: AccessibilityRequirementPatch,
}

/// 🧩 Identified-collection delta for `privacy`.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", default, deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
pub struct ProgramPrivacyDelta {
    pub added: Vec<PrivacyRequirement>,
    pub removed: Vec<String>,
    pub patched: Vec<ProgramPrivacyPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched `PrivacyRequirement` entry.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct ProgramPrivacyPatchEntry {
    pub id: String,
    pub patch: PrivacyRequirementPatch,
}

/// 🧩 Identified-collection delta for `safety`.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", default, deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
pub struct ProgramSafetyDelta {
    pub added: Vec<SafetyRequirement>,
    pub removed: Vec<String>,
    pub patched: Vec<ProgramSafetyPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched `SafetyRequirement` entry.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct ProgramSafetyPatchEntry {
    pub id: String,
    pub patch: SafetyRequirementPatch,
}

/// 🧩 Identified-collection delta for `security`.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", default, deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
pub struct ProgramSecurityDelta {
    pub added: Vec<SecurityRequirement>,
    pub removed: Vec<String>,
    pub patched: Vec<ProgramSecurityPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched `SecurityRequirement` entry.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct ProgramSecurityPatchEntry {
    pub id: String,
    pub patch: SecurityRequirementPatch,
}

/// 🧩 Identified-collection delta for `regulatory`.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", default, deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
pub struct ProgramRegulatoryDelta {
    pub added: Vec<RegulatoryRequirement>,
    pub removed: Vec<String>,
    pub patched: Vec<ProgramRegulatoryPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched `RegulatoryRequirement` entry.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct ProgramRegulatoryPatchEntry {
    pub id: String,
    pub patch: RegulatoryRequirementPatch,
}

/// 🧩 Identified-collection delta for `site_context`.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", default, deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
pub struct ProgramSiteContextDelta {
    pub added: Vec<SiteContext>,
    pub removed: Vec<String>,
    pub patched: Vec<ProgramSiteContextPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched `SiteContext` entry.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct ProgramSiteContextPatchEntry {
    pub id: String,
    pub patch: SiteContextPatch,
}

/// 🧩 Identified-collection delta for `organizational`.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", default, deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
pub struct ProgramOrganizationalDelta {
    pub added: Vec<OrganizationalRequirement>,
    pub removed: Vec<String>,
    pub patched: Vec<ProgramOrganizationalPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched `OrganizationalRequirement` entry.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct ProgramOrganizationalPatchEntry {
    pub id: String,
    pub patch: OrganizationalRequirementPatch,
}

/// 🧩 Identified-collection delta for `services`.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", default, deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
pub struct ProgramServicesDelta {
    pub added: Vec<ServiceRequirement>,
    pub removed: Vec<String>,
    pub patched: Vec<ProgramServicesPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched `ServiceRequirement` entry.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct ProgramServicesPatchEntry {
    pub id: String,
    pub patch: ServiceRequirementPatch,
}

/// 🧩 Identified-collection delta for `infrastructure`.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", default, deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
pub struct ProgramInfrastructureDelta {
    pub added: Vec<InfrastructureRequirement>,
    pub removed: Vec<String>,
    pub patched: Vec<ProgramInfrastructurePatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched `InfrastructureRequirement` entry.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct ProgramInfrastructurePatchEntry {
    pub id: String,
    pub patch: InfrastructureRequirementPatch,
}

/// 🧩 Identified-collection delta for `information`.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", default, deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
pub struct ProgramInformationDelta {
    pub added: Vec<InformationRequirement>,
    pub removed: Vec<String>,
    pub patched: Vec<ProgramInformationPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched `InformationRequirement` entry.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct ProgramInformationPatchEntry {
    pub id: String,
    pub patch: InformationRequirementPatch,
}

/// 🧩 Identified-collection delta for `communication`.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", default, deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
pub struct ProgramCommunicationDelta {
    pub added: Vec<CommunicationRequirement>,
    pub removed: Vec<String>,
    pub patched: Vec<ProgramCommunicationPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched `CommunicationRequirement` entry.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct ProgramCommunicationPatchEntry {
    pub id: String,
    pub patch: CommunicationRequirementPatch,
}

/// 🧩 Identified-collection delta for `wayfinding`.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", default, deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
pub struct ProgramWayfindingDelta {
    pub added: Vec<WayfindingRequirement>,
    pub removed: Vec<String>,
    pub patched: Vec<ProgramWayfindingPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched `WayfindingRequirement` entry.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct ProgramWayfindingPatchEntry {
    pub id: String,
    pub patch: WayfindingRequirementPatch,
}

/// 🧩 Identified-collection delta for `schedules`.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", default, deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
pub struct ProgramSchedulesDelta {
    pub added: Vec<ScheduleRequirement>,
    pub removed: Vec<String>,
    pub patched: Vec<ProgramSchedulesPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched `ScheduleRequirement` entry.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct ProgramSchedulesPatchEntry {
    pub id: String,
    pub patch: ScheduleRequirementPatch,
}

/// 🧩 Identified-collection delta for `flexibility`.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", default, deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
pub struct ProgramFlexibilityDelta {
    pub added: Vec<FlexibilityRequirement>,
    pub removed: Vec<String>,
    pub patched: Vec<ProgramFlexibilityPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched `FlexibilityRequirement` entry.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct ProgramFlexibilityPatchEntry {
    pub id: String,
    pub patch: FlexibilityRequirementPatch,
}

/// 🧩 Identified-collection delta for `growth`.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", default, deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
pub struct ProgramGrowthDelta {
    pub added: Vec<GrowthPlan>,
    pub removed: Vec<String>,
    pub patched: Vec<ProgramGrowthPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched `GrowthPlan` entry.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct ProgramGrowthPatchEntry {
    pub id: String,
    pub patch: GrowthPlanPatch,
}

/// 🧩 Identified-collection delta for `sustainability`.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", default, deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
pub struct ProgramSustainabilityDelta {
    pub added: Vec<SustainabilityRequirement>,
    pub removed: Vec<String>,
    pub patched: Vec<ProgramSustainabilityPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched `SustainabilityRequirement` entry.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct ProgramSustainabilityPatchEntry {
    pub id: String,
    pub patch: SustainabilityRequirementPatch,
}

/// 🧩 Identified-collection delta for `resilience`.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", default, deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
pub struct ProgramResilienceDelta {
    pub added: Vec<ResilienceRequirement>,
    pub removed: Vec<String>,
    pub patched: Vec<ProgramResiliencePatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched `ResilienceRequirement` entry.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct ProgramResiliencePatchEntry {
    pub id: String,
    pub patch: ResilienceRequirementPatch,
}

/// 🧩 Identified-collection delta for `costs`.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", default, deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
pub struct ProgramCostsDelta {
    pub added: Vec<CostRequirement>,
    pub removed: Vec<String>,
    pub patched: Vec<ProgramCostsPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched `CostRequirement` entry.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct ProgramCostsPatchEntry {
    pub id: String,
    pub patch: CostRequirementPatch,
}

/// 🧩 Identified-collection delta for `delivery`.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", default, deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
pub struct ProgramDeliveryDelta {
    pub added: Vec<DeliveryConstraint>,
    pub removed: Vec<String>,
    pub patched: Vec<ProgramDeliveryPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched `DeliveryConstraint` entry.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct ProgramDeliveryPatchEntry {
    pub id: String,
    pub patch: DeliveryConstraintPatch,
}

/// 🧩 Identified-collection delta for `risks`.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", default, deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
pub struct ProgramRisksDelta {
    pub added: Vec<Risk>,
    pub removed: Vec<String>,
    pub patched: Vec<ProgramRisksPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched `Risk` entry.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct ProgramRisksPatchEntry {
    pub id: String,
    pub patch: RiskPatch,
}

/// 🧩 Identified-collection delta for `conflicts`.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", default, deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
pub struct ProgramConflictsDelta {
    pub added: Vec<Conflict>,
    pub removed: Vec<String>,
    pub patched: Vec<ProgramConflictsPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched `Conflict` entry.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct ProgramConflictsPatchEntry {
    pub id: String,
    pub patch: ConflictPatch,
}

/// 🧩 Identified-collection delta for `requirements`.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", default, deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
pub struct ProgramRequirementsDelta {
    pub added: Vec<Requirement>,
    pub removed: Vec<String>,
    pub patched: Vec<ProgramRequirementsPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched `Requirement` entry.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct ProgramRequirementsPatchEntry {
    pub id: String,
    pub patch: RequirementPatch,
}

/// 🧩 Identified-collection delta for `priorities`.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", default, deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
pub struct ProgramPrioritiesDelta {
    pub added: Vec<PriorityRecord>,
    pub removed: Vec<String>,
    pub patched: Vec<ProgramPrioritiesPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched `PriorityRecord` entry.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct ProgramPrioritiesPatchEntry {
    pub id: String,
    pub patch: PriorityRecordPatch,
}

/// 🧩 Identified-collection delta for `scenarios`.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", default, deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
pub struct ProgramScenariosDelta {
    pub added: Vec<Scenario>,
    pub removed: Vec<String>,
    pub patched: Vec<ProgramScenariosPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched `Scenario` entry.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct ProgramScenariosPatchEntry {
    pub id: String,
    pub patch: ScenarioPatch,
}

/// 🧩 Identified-collection delta for `options`.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", default, deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
pub struct ProgramOptionsDelta {
    pub added: Vec<OptionEvaluation>,
    pub removed: Vec<String>,
    pub patched: Vec<ProgramOptionsPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched `OptionEvaluation` entry.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct ProgramOptionsPatchEntry {
    pub id: String,
    pub patch: OptionEvaluationPatch,
}

/// 🧩 Identified-collection delta for `decisions`.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", default, deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
pub struct ProgramDecisionsDelta {
    pub added: Vec<Decision>,
    pub removed: Vec<String>,
    pub patched: Vec<ProgramDecisionsPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched `Decision` entry.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct ProgramDecisionsPatchEntry {
    pub id: String,
    pub patch: DecisionPatch,
}

/// 🧩 Identified-collection delta for `validations`.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", default, deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
pub struct ProgramValidationsDelta {
    pub added: Vec<ValidationRecord>,
    pub removed: Vec<String>,
    pub patched: Vec<ProgramValidationsPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched `ValidationRecord` entry.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct ProgramValidationsPatchEntry {
    pub id: String,
    pub patch: ValidationRecordPatch,
}

/// 🧩 Identified-collection delta for `performance`.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", default, deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
pub struct ProgramPerformanceDelta {
    pub added: Vec<PerformanceCriterion>,
    pub removed: Vec<String>,
    pub patched: Vec<ProgramPerformancePatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched `PerformanceCriterion` entry.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct ProgramPerformancePatchEntry {
    pub id: String,
    pub patch: PerformanceCriterionPatch,
}

/// 🧩 Identified-collection delta for `quality`.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", default, deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
pub struct ProgramQualityDelta {
    pub added: Vec<QualityRecord>,
    pub removed: Vec<String>,
    pub patched: Vec<ProgramQualityPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched `QualityRecord` entry.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct ProgramQualityPatchEntry {
    pub id: String,
    pub patch: QualityRecordPatch,
}

/// 🧩 Identified-collection delta for `documents`.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", default, deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
pub struct ProgramArtifactsDelta {
    pub added: Vec<ArtifactRecord>,
    pub removed: Vec<String>,
    pub patched: Vec<ProgramArtifactsPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched `ArtifactRecord` entry.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct ProgramArtifactsPatchEntry {
    pub id: String,
    pub patch: ArtifactRecordPatch,
}

/// 🧩 Identified-collection delta for `assumptions`.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", default, deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
pub struct ProgramAssumptionsDelta {
    pub added: Vec<Assumption>,
    pub removed: Vec<String>,
    pub patched: Vec<ProgramAssumptionsPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched `Assumption` entry.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct ProgramAssumptionsPatchEntry {
    pub id: String,
    pub patch: AssumptionPatch,
}

/// 🧩 Identified-collection delta for `constraints`.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", default, deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
pub struct ProgramConstraintsDelta {
    pub added: Vec<ConstraintRecord>,
    pub removed: Vec<String>,
    pub patched: Vec<ProgramConstraintsPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched `ConstraintRecord` entry.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct ProgramConstraintsPatchEntry {
    pub id: String,
    pub patch: ConstraintRecordPatch,
}

/// 🧩 Identified-collection delta for `compliance_records`.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", default, deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
pub struct ProgramComplianceRecordsDelta {
    pub added: Vec<ComplianceRecord>,
    pub removed: Vec<String>,
    pub patched: Vec<ProgramComplianceRecordsPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched `ComplianceRecord` entry.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct ProgramComplianceRecordsPatchEntry {
    pub id: String,
    pub patch: ComplianceRecordPatch,
}

/// 🧩 Identified-collection delta for `approvals`.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", default, deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
pub struct ProgramApprovalsDelta {
    pub added: Vec<ApprovalRecord>,
    pub removed: Vec<String>,
    pub patched: Vec<ProgramApprovalsPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched `ApprovalRecord` entry.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct ProgramApprovalsPatchEntry {
    pub id: String,
    pub patch: ApprovalRecordPatch,
}

/// 🧩 Identified-collection delta for `meetings`.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", default, deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
pub struct ProgramMeetingsDelta {
    pub added: Vec<MeetingRecord>,
    pub removed: Vec<String>,
    pub patched: Vec<ProgramMeetingsPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched `MeetingRecord` entry.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct ProgramMeetingsPatchEntry {
    pub id: String,
    pub patch: MeetingRecordPatch,
}

/// 🧩 Identified-collection delta for `changes`.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", default, deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
pub struct ProgramChangesDelta {
    pub added: Vec<ChangeRecord>,
    pub removed: Vec<String>,
    pub patched: Vec<ProgramChangesPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched `ChangeRecord` entry.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct ProgramChangesPatchEntry {
    pub id: String,
    pub patch: ChangeRecordPatch,
}

/// 🧩 Identified-collection delta for `collaboration`.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", default, deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
pub struct ProgramCollaborationDelta {
    pub added: Vec<CollaborationRecord>,
    pub removed: Vec<String>,
    pub patched: Vec<ProgramCollaborationPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched `CollaborationRecord` entry.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct ProgramCollaborationPatchEntry {
    pub id: String,
    pub patch: CollaborationRecordPatch,
}

/// 🧩 Identified-collection delta for `analyses`.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", default, deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
pub struct ProgramAnalysesDelta {
    pub added: Vec<AnalysisRecord>,
    pub removed: Vec<String>,
    pub patched: Vec<ProgramAnalysesPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched `AnalysisRecord` entry.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct ProgramAnalysesPatchEntry {
    pub id: String,
    pub patch: AnalysisRecordPatch,
}

/// 🧩 Identified-collection delta for `reports`.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", default, deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
pub struct ProgramReportsDelta {
    pub added: Vec<ReportRecord>,
    pub removed: Vec<String>,
    pub patched: Vec<ProgramReportsPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched `ReportRecord` entry.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct ProgramReportsPatchEntry {
    pub id: String,
    pub patch: ReportRecordPatch,
}

/// 🧩 Identified-collection delta for `search_filters`.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", default, deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
pub struct ProgramSearchFiltersDelta {
    pub added: Vec<SearchFilter>,
    pub removed: Vec<String>,
    pub patched: Vec<ProgramSearchFiltersPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched `SearchFilter` entry.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct ProgramSearchFiltersPatchEntry {
    pub id: String,
    pub patch: SearchFilterPatch,
}

/// 🧩 Identified-collection delta for `status_records`.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", default, deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
pub struct ProgramStatusRecordsDelta {
    pub added: Vec<StatusRecord>,
    pub removed: Vec<String>,
    pub patched: Vec<ProgramStatusRecordsPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched `StatusRecord` entry.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct ProgramStatusRecordsPatchEntry {
    pub id: String,
    pub patch: StatusRecordPatch,
}

/// 🧩 Identified-collection delta for `workshops`.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", default, deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
pub struct ProgramWorkshopsDelta {
    pub added: Vec<Workshop>,
    pub removed: Vec<String>,
    pub patched: Vec<ProgramWorkshopsPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched `Workshop` entry.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct ProgramWorkshopsPatchEntry {
    pub id: String,
    pub patch: WorkshopPatch,
}

/// 🧩 Identified-collection delta for `surveys`.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", default, deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
pub struct ProgramSurveysDelta {
    pub added: Vec<Survey>,
    pub removed: Vec<String>,
    pub patched: Vec<ProgramSurveysPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched `Survey` entry.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct ProgramSurveysPatchEntry {
    pub id: String,
    pub patch: SurveyPatch,
}

/// 🧩 Identified-collection delta for `issues`.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", default, deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
pub struct ProgramIssuesDelta {
    pub added: Vec<Issue>,
    pub removed: Vec<String>,
    pub patched: Vec<ProgramIssuesPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched `Issue` entry.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct ProgramIssuesPatchEntry {
    pub id: String,
    pub patch: IssuePatch,
}

/// 🧩 Identified-collection delta for `audit_events`.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", default, deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
pub struct ProgramAuditEventsDelta {
    pub added: Vec<AuditEvent>,
    pub removed: Vec<String>,
    pub patched: Vec<ProgramAuditEventsPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched `AuditEvent` entry.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct ProgramAuditEventsPatchEntry {
    pub id: String,
    pub patch: AuditEventPatch,
}

/// 🧩 Identified-collection delta for `templates`.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", default, deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
pub struct ProgramTemplatesDelta {
    pub added: Vec<TemplateRecord>,
    pub removed: Vec<String>,
    pub patched: Vec<ProgramTemplatesPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched `TemplateRecord` entry.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct ProgramTemplatesPatchEntry {
    pub id: String,
    pub patch: TemplateRecordPatch,
}

/// 🧩 Identified-collection delta for `traces`.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", default, deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
pub struct ProgramTracesDelta {
    pub added: Vec<TraceLink>,
    pub removed: Vec<String>,
    pub patched: Vec<ProgramTracesPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched `TraceLink` entry.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct ProgramTracesPatchEntry {
    pub id: String,
    pub patch: TraceLinkPatch,
}

//#endregion 🔖️DeltaHelpers

use crate::schema::ProgramArtifact;
use crate::ProgramSnapshot;
use protocol::Identified;
use protocol::MutationDiff;
use protocol::Patchable;

impl ProgramDiff {
    /// 🧬️ Apply every field entry onto a full artifact.
    pub fn apply_to_artifact(&self, artifact: &ProgramArtifact) -> protocol::MutationApplyResult<ProgramArtifact> {
        Ok({
            if let Some(replacement) = &self.artifact {
                return Ok((**replacement).clone());
            }
            let mut next = artifact.clone();
            if let Some(v) = &self.schema {
                next.schema = v.clone();
            }
            if let Some(v) = &self.meta {
                next.meta = v.clone();
            }
            if let Some(v) = &self.project {
                next.project = v.clone();
            }
            if let Some(v) = &self.governance {
                next.governance = v.clone();
            }

            if let Some(delta) = &self.stakeholders {
                apply_collection_delta(&mut next.stakeholders, &delta.added, &delta.removed, &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(), &delta.reordered).map_err(|error| error.under(["stakeholders"]))?;
            }
            if let Some(delta) = &self.users {
                apply_collection_delta(&mut next.users, &delta.added, &delta.removed, &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(), &delta.reordered).map_err(|error| error.under(["users"]))?;
            }
            if let Some(delta) = &self.activities {
                apply_collection_delta(&mut next.activities, &delta.added, &delta.removed, &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(), &delta.reordered).map_err(|error| error.under(["activities"]))?;
            }
            if let Some(delta) = &self.functions {
                apply_collection_delta(&mut next.functions, &delta.added, &delta.removed, &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(), &delta.reordered).map_err(|error| error.under(["functions"]))?;
            }
            if let Some(delta) = &self.elements {
                apply_collection_delta(&mut next.elements, &delta.added, &delta.removed, &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(), &delta.reordered).map_err(|error| error.under(["elements"]))?;
            }
            if let Some(delta) = &self.quantities {
                apply_collection_delta(&mut next.quantities, &delta.added, &delta.removed, &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(), &delta.reordered).map_err(|error| error.under(["quantities"]))?;
            }
            if let Some(delta) = &self.relationships {
                apply_collection_delta(&mut next.relationships, &delta.added, &delta.removed, &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(), &delta.reordered).map_err(|error| error.under(["relationships"]))?;
            }
            if let Some(delta) = &self.adjacencies {
                apply_collection_delta(&mut next.adjacencies, &delta.added, &delta.removed, &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(), &delta.reordered).map_err(|error| error.under(["adjacencies"]))?;
            }
            if let Some(delta) = &self.processes {
                apply_collection_delta(&mut next.processes, &delta.added, &delta.removed, &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(), &delta.reordered).map_err(|error| error.under(["processes"]))?;
            }
            if let Some(delta) = &self.flows {
                apply_collection_delta(&mut next.flows, &delta.added, &delta.removed, &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(), &delta.reordered).map_err(|error| error.under(["flows"]))?;
            }
            if let Some(delta) = &self.access_rules {
                apply_collection_delta(&mut next.access_rules, &delta.added, &delta.removed, &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(), &delta.reordered).map_err(|error| error.under(["accessRules"]))?;
            }
            if let Some(delta) = &self.operations {
                apply_collection_delta(&mut next.operations, &delta.added, &delta.removed, &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(), &delta.reordered).map_err(|error| error.under(["operations"]))?;
            }
            if let Some(delta) = &self.equipment {
                apply_collection_delta(&mut next.equipment, &delta.added, &delta.removed, &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(), &delta.reordered).map_err(|error| error.under(["equipment"]))?;
            }
            if let Some(delta) = &self.resources {
                apply_collection_delta(&mut next.resources, &delta.added, &delta.removed, &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(), &delta.reordered).map_err(|error| error.under(["resources"]))?;
            }
            if let Some(delta) = &self.storage {
                apply_collection_delta(&mut next.storage, &delta.added, &delta.removed, &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(), &delta.reordered).map_err(|error| error.under(["storage"]))?;
            }
            if let Some(delta) = &self.environmental {
                apply_collection_delta(&mut next.environmental, &delta.added, &delta.removed, &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(), &delta.reordered).map_err(|error| error.under(["environmental"]))?;
            }
            if let Some(delta) = &self.human_factors {
                apply_collection_delta(&mut next.human_factors, &delta.added, &delta.removed, &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(), &delta.reordered).map_err(|error| error.under(["humanFactors"]))?;
            }
            if let Some(delta) = &self.accessibility {
                apply_collection_delta(&mut next.accessibility, &delta.added, &delta.removed, &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(), &delta.reordered).map_err(|error| error.under(["accessibility"]))?;
            }
            if let Some(delta) = &self.privacy {
                apply_collection_delta(&mut next.privacy, &delta.added, &delta.removed, &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(), &delta.reordered).map_err(|error| error.under(["privacy"]))?;
            }
            if let Some(delta) = &self.safety {
                apply_collection_delta(&mut next.safety, &delta.added, &delta.removed, &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(), &delta.reordered).map_err(|error| error.under(["safety"]))?;
            }
            if let Some(delta) = &self.security {
                apply_collection_delta(&mut next.security, &delta.added, &delta.removed, &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(), &delta.reordered).map_err(|error| error.under(["security"]))?;
            }
            if let Some(delta) = &self.regulatory {
                apply_collection_delta(&mut next.regulatory, &delta.added, &delta.removed, &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(), &delta.reordered).map_err(|error| error.under(["regulatory"]))?;
            }
            if let Some(delta) = &self.site_context {
                apply_collection_delta(&mut next.site_context, &delta.added, &delta.removed, &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(), &delta.reordered).map_err(|error| error.under(["siteContext"]))?;
            }
            if let Some(delta) = &self.organizational {
                apply_collection_delta(&mut next.organizational, &delta.added, &delta.removed, &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(), &delta.reordered).map_err(|error| error.under(["organizational"]))?;
            }
            if let Some(delta) = &self.services {
                apply_collection_delta(&mut next.services, &delta.added, &delta.removed, &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(), &delta.reordered).map_err(|error| error.under(["services"]))?;
            }
            if let Some(delta) = &self.infrastructure {
                apply_collection_delta(&mut next.infrastructure, &delta.added, &delta.removed, &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(), &delta.reordered).map_err(|error| error.under(["infrastructure"]))?;
            }
            if let Some(delta) = &self.information {
                apply_collection_delta(&mut next.information, &delta.added, &delta.removed, &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(), &delta.reordered).map_err(|error| error.under(["information"]))?;
            }
            if let Some(delta) = &self.communication {
                apply_collection_delta(&mut next.communication, &delta.added, &delta.removed, &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(), &delta.reordered).map_err(|error| error.under(["communication"]))?;
            }
            if let Some(delta) = &self.wayfinding {
                apply_collection_delta(&mut next.wayfinding, &delta.added, &delta.removed, &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(), &delta.reordered).map_err(|error| error.under(["wayfinding"]))?;
            }
            if let Some(delta) = &self.schedules {
                apply_collection_delta(&mut next.schedules, &delta.added, &delta.removed, &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(), &delta.reordered).map_err(|error| error.under(["schedules"]))?;
            }
            if let Some(delta) = &self.flexibility {
                apply_collection_delta(&mut next.flexibility, &delta.added, &delta.removed, &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(), &delta.reordered).map_err(|error| error.under(["flexibility"]))?;
            }
            if let Some(delta) = &self.growth {
                apply_collection_delta(&mut next.growth, &delta.added, &delta.removed, &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(), &delta.reordered).map_err(|error| error.under(["growth"]))?;
            }
            if let Some(delta) = &self.sustainability {
                apply_collection_delta(&mut next.sustainability, &delta.added, &delta.removed, &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(), &delta.reordered).map_err(|error| error.under(["sustainability"]))?;
            }
            if let Some(delta) = &self.resilience {
                apply_collection_delta(&mut next.resilience, &delta.added, &delta.removed, &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(), &delta.reordered).map_err(|error| error.under(["resilience"]))?;
            }
            if let Some(delta) = &self.costs {
                apply_collection_delta(&mut next.costs, &delta.added, &delta.removed, &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(), &delta.reordered).map_err(|error| error.under(["costs"]))?;
            }
            if let Some(delta) = &self.delivery {
                apply_collection_delta(&mut next.delivery, &delta.added, &delta.removed, &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(), &delta.reordered).map_err(|error| error.under(["delivery"]))?;
            }
            if let Some(delta) = &self.risks {
                apply_collection_delta(&mut next.risks, &delta.added, &delta.removed, &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(), &delta.reordered).map_err(|error| error.under(["risks"]))?;
            }
            if let Some(delta) = &self.conflicts {
                apply_collection_delta(&mut next.conflicts, &delta.added, &delta.removed, &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(), &delta.reordered).map_err(|error| error.under(["conflicts"]))?;
            }
            if let Some(delta) = &self.requirements {
                apply_collection_delta(&mut next.requirements, &delta.added, &delta.removed, &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(), &delta.reordered).map_err(|error| error.under(["requirements"]))?;
            }
            if let Some(delta) = &self.priorities {
                apply_collection_delta(&mut next.priorities, &delta.added, &delta.removed, &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(), &delta.reordered).map_err(|error| error.under(["priorities"]))?;
            }
            if let Some(delta) = &self.scenarios {
                apply_collection_delta(&mut next.scenarios, &delta.added, &delta.removed, &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(), &delta.reordered).map_err(|error| error.under(["scenarios"]))?;
            }
            if let Some(delta) = &self.options {
                apply_collection_delta(&mut next.options, &delta.added, &delta.removed, &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(), &delta.reordered).map_err(|error| error.under(["options"]))?;
            }
            if let Some(delta) = &self.decisions {
                apply_collection_delta(&mut next.decisions, &delta.added, &delta.removed, &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(), &delta.reordered).map_err(|error| error.under(["decisions"]))?;
            }
            if let Some(delta) = &self.validations {
                apply_collection_delta(&mut next.validations, &delta.added, &delta.removed, &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(), &delta.reordered).map_err(|error| error.under(["validations"]))?;
            }
            if let Some(delta) = &self.performance {
                apply_collection_delta(&mut next.performance, &delta.added, &delta.removed, &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(), &delta.reordered).map_err(|error| error.under(["performance"]))?;
            }
            if let Some(delta) = &self.quality {
                apply_collection_delta(&mut next.quality, &delta.added, &delta.removed, &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(), &delta.reordered).map_err(|error| error.under(["quality"]))?;
            }
            if let Some(delta) = &self.artifacts {
                apply_collection_delta(&mut next.artifacts, &delta.added, &delta.removed, &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(), &delta.reordered).map_err(|error| error.under(["artifacts"]))?;
            }
            if let Some(delta) = &self.assumptions {
                apply_collection_delta(&mut next.assumptions, &delta.added, &delta.removed, &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(), &delta.reordered).map_err(|error| error.under(["assumptions"]))?;
            }
            if let Some(delta) = &self.constraints {
                apply_collection_delta(&mut next.constraints, &delta.added, &delta.removed, &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(), &delta.reordered).map_err(|error| error.under(["constraints"]))?;
            }
            if let Some(delta) = &self.compliance_records {
                apply_collection_delta(&mut next.compliance_records, &delta.added, &delta.removed, &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(), &delta.reordered)
                    .map_err(|error| error.under(["complianceRecords"]))?;
            }
            if let Some(delta) = &self.approvals {
                apply_collection_delta(&mut next.approvals, &delta.added, &delta.removed, &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(), &delta.reordered).map_err(|error| error.under(["approvals"]))?;
            }
            if let Some(delta) = &self.meetings {
                apply_collection_delta(&mut next.meetings, &delta.added, &delta.removed, &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(), &delta.reordered).map_err(|error| error.under(["meetings"]))?;
            }
            if let Some(delta) = &self.changes {
                apply_collection_delta(&mut next.changes, &delta.added, &delta.removed, &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(), &delta.reordered).map_err(|error| error.under(["changes"]))?;
            }
            if let Some(delta) = &self.collaboration {
                apply_collection_delta(&mut next.collaboration, &delta.added, &delta.removed, &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(), &delta.reordered).map_err(|error| error.under(["collaboration"]))?;
            }
            if let Some(delta) = &self.analyses {
                apply_collection_delta(&mut next.analyses, &delta.added, &delta.removed, &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(), &delta.reordered).map_err(|error| error.under(["analyses"]))?;
            }
            if let Some(delta) = &self.reports {
                apply_collection_delta(&mut next.reports, &delta.added, &delta.removed, &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(), &delta.reordered).map_err(|error| error.under(["reports"]))?;
            }
            if let Some(delta) = &self.search_filters {
                apply_collection_delta(&mut next.search_filters, &delta.added, &delta.removed, &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(), &delta.reordered).map_err(|error| error.under(["searchFilters"]))?;
            }
            if let Some(delta) = &self.status_records {
                apply_collection_delta(&mut next.status_records, &delta.added, &delta.removed, &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(), &delta.reordered).map_err(|error| error.under(["statusRecords"]))?;
            }
            if let Some(delta) = &self.workshops {
                apply_collection_delta(&mut next.workshops, &delta.added, &delta.removed, &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(), &delta.reordered).map_err(|error| error.under(["workshops"]))?;
            }
            if let Some(delta) = &self.surveys {
                apply_collection_delta(&mut next.surveys, &delta.added, &delta.removed, &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(), &delta.reordered).map_err(|error| error.under(["surveys"]))?;
            }
            if let Some(delta) = &self.issues {
                apply_collection_delta(&mut next.issues, &delta.added, &delta.removed, &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(), &delta.reordered).map_err(|error| error.under(["issues"]))?;
            }
            if let Some(delta) = &self.audit_events {
                apply_collection_delta(&mut next.audit_events, &delta.added, &delta.removed, &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(), &delta.reordered).map_err(|error| error.under(["auditEvents"]))?;
            }
            if let Some(delta) = &self.templates {
                apply_collection_delta(&mut next.templates, &delta.added, &delta.removed, &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(), &delta.reordered).map_err(|error| error.under(["templates"]))?;
            }
            if let Some(records) = &self.knowledge_payload {
                next.knowledge_payload = records.clone();
            }
            if let Some(child) = &self.knowledge {
                next.knowledge = child.clone();
            }
            if let Some(records) = &self.benchmarks_payload {
                next.benchmarks_payload = records.clone();
            }
            if let Some(child) = &self.benchmarks {
                next.benchmarks = child.clone();
            }
            if let Some(delta) = &self.traces {
                apply_collection_delta(&mut next.traces, &delta.added, &delta.removed, &delta.patched.iter().map(|p| (p.id.clone(), p.patch.clone())).collect::<Vec<_>>(), &delta.reordered).map_err(|error| error.under(["traces"]))?;
            }
            next
        })
    }
}

impl MutationDiff<ProgramSnapshot> for ProgramDiff {
    fn apply(&self, base: &ProgramSnapshot) -> protocol::MutationApplyResult<ProgramSnapshot> {
        self.apply_to_artifact(&ProgramArtifact::from_snapshot(base.clone())).map(|artifact| artifact.to_snapshot()).map_err(|error| error.under(["artifact"]))
    }
    fn absorb(&mut self, other: Self) {
        if other.artifact.is_some() {
            *self = other;
            return;
        }
        macro_rules! absorb_opt {
            ($f:ident) => {
                if other.$f.is_some() {
                    self.$f = other.$f;
                }
            };
        }
        absorb_opt!(schema);
        absorb_opt!(meta);
        absorb_opt!(project);
        absorb_opt!(governance);

        if let Some(delta) = other.stakeholders {
            match &mut self.stakeholders {
                Some(existing) => {
                    existing.added.extend(delta.added);
                    existing.removed.extend(delta.removed);
                    existing.patched.extend(delta.patched);
                    if delta.reordered.is_some() {
                        existing.reordered = delta.reordered;
                    }
                }
                None => self.stakeholders = Some(delta),
            }
        }
        if let Some(delta) = other.users {
            match &mut self.users {
                Some(existing) => {
                    existing.added.extend(delta.added);
                    existing.removed.extend(delta.removed);
                    existing.patched.extend(delta.patched);
                    if delta.reordered.is_some() {
                        existing.reordered = delta.reordered;
                    }
                }
                None => self.users = Some(delta),
            }
        }
        if let Some(delta) = other.activities {
            match &mut self.activities {
                Some(existing) => {
                    existing.added.extend(delta.added);
                    existing.removed.extend(delta.removed);
                    existing.patched.extend(delta.patched);
                    if delta.reordered.is_some() {
                        existing.reordered = delta.reordered;
                    }
                }
                None => self.activities = Some(delta),
            }
        }
        if let Some(delta) = other.functions {
            match &mut self.functions {
                Some(existing) => {
                    existing.added.extend(delta.added);
                    existing.removed.extend(delta.removed);
                    existing.patched.extend(delta.patched);
                    if delta.reordered.is_some() {
                        existing.reordered = delta.reordered;
                    }
                }
                None => self.functions = Some(delta),
            }
        }
        if let Some(delta) = other.elements {
            match &mut self.elements {
                Some(existing) => {
                    existing.added.extend(delta.added);
                    existing.removed.extend(delta.removed);
                    existing.patched.extend(delta.patched);
                    if delta.reordered.is_some() {
                        existing.reordered = delta.reordered;
                    }
                }
                None => self.elements = Some(delta),
            }
        }
        if let Some(delta) = other.quantities {
            match &mut self.quantities {
                Some(existing) => {
                    existing.added.extend(delta.added);
                    existing.removed.extend(delta.removed);
                    existing.patched.extend(delta.patched);
                    if delta.reordered.is_some() {
                        existing.reordered = delta.reordered;
                    }
                }
                None => self.quantities = Some(delta),
            }
        }
        if let Some(delta) = other.relationships {
            match &mut self.relationships {
                Some(existing) => {
                    existing.added.extend(delta.added);
                    existing.removed.extend(delta.removed);
                    existing.patched.extend(delta.patched);
                    if delta.reordered.is_some() {
                        existing.reordered = delta.reordered;
                    }
                }
                None => self.relationships = Some(delta),
            }
        }
        if let Some(delta) = other.adjacencies {
            match &mut self.adjacencies {
                Some(existing) => {
                    existing.added.extend(delta.added);
                    existing.removed.extend(delta.removed);
                    existing.patched.extend(delta.patched);
                    if delta.reordered.is_some() {
                        existing.reordered = delta.reordered;
                    }
                }
                None => self.adjacencies = Some(delta),
            }
        }
        if let Some(delta) = other.processes {
            match &mut self.processes {
                Some(existing) => {
                    existing.added.extend(delta.added);
                    existing.removed.extend(delta.removed);
                    existing.patched.extend(delta.patched);
                    if delta.reordered.is_some() {
                        existing.reordered = delta.reordered;
                    }
                }
                None => self.processes = Some(delta),
            }
        }
        if let Some(delta) = other.flows {
            match &mut self.flows {
                Some(existing) => {
                    existing.added.extend(delta.added);
                    existing.removed.extend(delta.removed);
                    existing.patched.extend(delta.patched);
                    if delta.reordered.is_some() {
                        existing.reordered = delta.reordered;
                    }
                }
                None => self.flows = Some(delta),
            }
        }
        if let Some(delta) = other.access_rules {
            match &mut self.access_rules {
                Some(existing) => {
                    existing.added.extend(delta.added);
                    existing.removed.extend(delta.removed);
                    existing.patched.extend(delta.patched);
                    if delta.reordered.is_some() {
                        existing.reordered = delta.reordered;
                    }
                }
                None => self.access_rules = Some(delta),
            }
        }
        if let Some(delta) = other.operations {
            match &mut self.operations {
                Some(existing) => {
                    existing.added.extend(delta.added);
                    existing.removed.extend(delta.removed);
                    existing.patched.extend(delta.patched);
                    if delta.reordered.is_some() {
                        existing.reordered = delta.reordered;
                    }
                }
                None => self.operations = Some(delta),
            }
        }
        if let Some(delta) = other.equipment {
            match &mut self.equipment {
                Some(existing) => {
                    existing.added.extend(delta.added);
                    existing.removed.extend(delta.removed);
                    existing.patched.extend(delta.patched);
                    if delta.reordered.is_some() {
                        existing.reordered = delta.reordered;
                    }
                }
                None => self.equipment = Some(delta),
            }
        }
        if let Some(delta) = other.resources {
            match &mut self.resources {
                Some(existing) => {
                    existing.added.extend(delta.added);
                    existing.removed.extend(delta.removed);
                    existing.patched.extend(delta.patched);
                    if delta.reordered.is_some() {
                        existing.reordered = delta.reordered;
                    }
                }
                None => self.resources = Some(delta),
            }
        }
        if let Some(delta) = other.storage {
            match &mut self.storage {
                Some(existing) => {
                    existing.added.extend(delta.added);
                    existing.removed.extend(delta.removed);
                    existing.patched.extend(delta.patched);
                    if delta.reordered.is_some() {
                        existing.reordered = delta.reordered;
                    }
                }
                None => self.storage = Some(delta),
            }
        }
        if let Some(delta) = other.environmental {
            match &mut self.environmental {
                Some(existing) => {
                    existing.added.extend(delta.added);
                    existing.removed.extend(delta.removed);
                    existing.patched.extend(delta.patched);
                    if delta.reordered.is_some() {
                        existing.reordered = delta.reordered;
                    }
                }
                None => self.environmental = Some(delta),
            }
        }
        if let Some(delta) = other.human_factors {
            match &mut self.human_factors {
                Some(existing) => {
                    existing.added.extend(delta.added);
                    existing.removed.extend(delta.removed);
                    existing.patched.extend(delta.patched);
                    if delta.reordered.is_some() {
                        existing.reordered = delta.reordered;
                    }
                }
                None => self.human_factors = Some(delta),
            }
        }
        if let Some(delta) = other.accessibility {
            match &mut self.accessibility {
                Some(existing) => {
                    existing.added.extend(delta.added);
                    existing.removed.extend(delta.removed);
                    existing.patched.extend(delta.patched);
                    if delta.reordered.is_some() {
                        existing.reordered = delta.reordered;
                    }
                }
                None => self.accessibility = Some(delta),
            }
        }
        if let Some(delta) = other.privacy {
            match &mut self.privacy {
                Some(existing) => {
                    existing.added.extend(delta.added);
                    existing.removed.extend(delta.removed);
                    existing.patched.extend(delta.patched);
                    if delta.reordered.is_some() {
                        existing.reordered = delta.reordered;
                    }
                }
                None => self.privacy = Some(delta),
            }
        }
        if let Some(delta) = other.safety {
            match &mut self.safety {
                Some(existing) => {
                    existing.added.extend(delta.added);
                    existing.removed.extend(delta.removed);
                    existing.patched.extend(delta.patched);
                    if delta.reordered.is_some() {
                        existing.reordered = delta.reordered;
                    }
                }
                None => self.safety = Some(delta),
            }
        }
        if let Some(delta) = other.security {
            match &mut self.security {
                Some(existing) => {
                    existing.added.extend(delta.added);
                    existing.removed.extend(delta.removed);
                    existing.patched.extend(delta.patched);
                    if delta.reordered.is_some() {
                        existing.reordered = delta.reordered;
                    }
                }
                None => self.security = Some(delta),
            }
        }
        if let Some(delta) = other.regulatory {
            match &mut self.regulatory {
                Some(existing) => {
                    existing.added.extend(delta.added);
                    existing.removed.extend(delta.removed);
                    existing.patched.extend(delta.patched);
                    if delta.reordered.is_some() {
                        existing.reordered = delta.reordered;
                    }
                }
                None => self.regulatory = Some(delta),
            }
        }
        if let Some(delta) = other.site_context {
            match &mut self.site_context {
                Some(existing) => {
                    existing.added.extend(delta.added);
                    existing.removed.extend(delta.removed);
                    existing.patched.extend(delta.patched);
                    if delta.reordered.is_some() {
                        existing.reordered = delta.reordered;
                    }
                }
                None => self.site_context = Some(delta),
            }
        }
        if let Some(delta) = other.organizational {
            match &mut self.organizational {
                Some(existing) => {
                    existing.added.extend(delta.added);
                    existing.removed.extend(delta.removed);
                    existing.patched.extend(delta.patched);
                    if delta.reordered.is_some() {
                        existing.reordered = delta.reordered;
                    }
                }
                None => self.organizational = Some(delta),
            }
        }
        if let Some(delta) = other.services {
            match &mut self.services {
                Some(existing) => {
                    existing.added.extend(delta.added);
                    existing.removed.extend(delta.removed);
                    existing.patched.extend(delta.patched);
                    if delta.reordered.is_some() {
                        existing.reordered = delta.reordered;
                    }
                }
                None => self.services = Some(delta),
            }
        }
        if let Some(delta) = other.infrastructure {
            match &mut self.infrastructure {
                Some(existing) => {
                    existing.added.extend(delta.added);
                    existing.removed.extend(delta.removed);
                    existing.patched.extend(delta.patched);
                    if delta.reordered.is_some() {
                        existing.reordered = delta.reordered;
                    }
                }
                None => self.infrastructure = Some(delta),
            }
        }
        if let Some(delta) = other.information {
            match &mut self.information {
                Some(existing) => {
                    existing.added.extend(delta.added);
                    existing.removed.extend(delta.removed);
                    existing.patched.extend(delta.patched);
                    if delta.reordered.is_some() {
                        existing.reordered = delta.reordered;
                    }
                }
                None => self.information = Some(delta),
            }
        }
        if let Some(delta) = other.communication {
            match &mut self.communication {
                Some(existing) => {
                    existing.added.extend(delta.added);
                    existing.removed.extend(delta.removed);
                    existing.patched.extend(delta.patched);
                    if delta.reordered.is_some() {
                        existing.reordered = delta.reordered;
                    }
                }
                None => self.communication = Some(delta),
            }
        }
        if let Some(delta) = other.wayfinding {
            match &mut self.wayfinding {
                Some(existing) => {
                    existing.added.extend(delta.added);
                    existing.removed.extend(delta.removed);
                    existing.patched.extend(delta.patched);
                    if delta.reordered.is_some() {
                        existing.reordered = delta.reordered;
                    }
                }
                None => self.wayfinding = Some(delta),
            }
        }
        if let Some(delta) = other.schedules {
            match &mut self.schedules {
                Some(existing) => {
                    existing.added.extend(delta.added);
                    existing.removed.extend(delta.removed);
                    existing.patched.extend(delta.patched);
                    if delta.reordered.is_some() {
                        existing.reordered = delta.reordered;
                    }
                }
                None => self.schedules = Some(delta),
            }
        }
        if let Some(delta) = other.flexibility {
            match &mut self.flexibility {
                Some(existing) => {
                    existing.added.extend(delta.added);
                    existing.removed.extend(delta.removed);
                    existing.patched.extend(delta.patched);
                    if delta.reordered.is_some() {
                        existing.reordered = delta.reordered;
                    }
                }
                None => self.flexibility = Some(delta),
            }
        }
        if let Some(delta) = other.growth {
            match &mut self.growth {
                Some(existing) => {
                    existing.added.extend(delta.added);
                    existing.removed.extend(delta.removed);
                    existing.patched.extend(delta.patched);
                    if delta.reordered.is_some() {
                        existing.reordered = delta.reordered;
                    }
                }
                None => self.growth = Some(delta),
            }
        }
        if let Some(delta) = other.sustainability {
            match &mut self.sustainability {
                Some(existing) => {
                    existing.added.extend(delta.added);
                    existing.removed.extend(delta.removed);
                    existing.patched.extend(delta.patched);
                    if delta.reordered.is_some() {
                        existing.reordered = delta.reordered;
                    }
                }
                None => self.sustainability = Some(delta),
            }
        }
        if let Some(delta) = other.resilience {
            match &mut self.resilience {
                Some(existing) => {
                    existing.added.extend(delta.added);
                    existing.removed.extend(delta.removed);
                    existing.patched.extend(delta.patched);
                    if delta.reordered.is_some() {
                        existing.reordered = delta.reordered;
                    }
                }
                None => self.resilience = Some(delta),
            }
        }
        if let Some(delta) = other.costs {
            match &mut self.costs {
                Some(existing) => {
                    existing.added.extend(delta.added);
                    existing.removed.extend(delta.removed);
                    existing.patched.extend(delta.patched);
                    if delta.reordered.is_some() {
                        existing.reordered = delta.reordered;
                    }
                }
                None => self.costs = Some(delta),
            }
        }
        if let Some(delta) = other.delivery {
            match &mut self.delivery {
                Some(existing) => {
                    existing.added.extend(delta.added);
                    existing.removed.extend(delta.removed);
                    existing.patched.extend(delta.patched);
                    if delta.reordered.is_some() {
                        existing.reordered = delta.reordered;
                    }
                }
                None => self.delivery = Some(delta),
            }
        }
        if let Some(delta) = other.risks {
            match &mut self.risks {
                Some(existing) => {
                    existing.added.extend(delta.added);
                    existing.removed.extend(delta.removed);
                    existing.patched.extend(delta.patched);
                    if delta.reordered.is_some() {
                        existing.reordered = delta.reordered;
                    }
                }
                None => self.risks = Some(delta),
            }
        }
        if let Some(delta) = other.conflicts {
            match &mut self.conflicts {
                Some(existing) => {
                    existing.added.extend(delta.added);
                    existing.removed.extend(delta.removed);
                    existing.patched.extend(delta.patched);
                    if delta.reordered.is_some() {
                        existing.reordered = delta.reordered;
                    }
                }
                None => self.conflicts = Some(delta),
            }
        }
        if let Some(delta) = other.requirements {
            match &mut self.requirements {
                Some(existing) => {
                    existing.added.extend(delta.added);
                    existing.removed.extend(delta.removed);
                    existing.patched.extend(delta.patched);
                    if delta.reordered.is_some() {
                        existing.reordered = delta.reordered;
                    }
                }
                None => self.requirements = Some(delta),
            }
        }
        if let Some(delta) = other.priorities {
            match &mut self.priorities {
                Some(existing) => {
                    existing.added.extend(delta.added);
                    existing.removed.extend(delta.removed);
                    existing.patched.extend(delta.patched);
                    if delta.reordered.is_some() {
                        existing.reordered = delta.reordered;
                    }
                }
                None => self.priorities = Some(delta),
            }
        }
        if let Some(delta) = other.scenarios {
            match &mut self.scenarios {
                Some(existing) => {
                    existing.added.extend(delta.added);
                    existing.removed.extend(delta.removed);
                    existing.patched.extend(delta.patched);
                    if delta.reordered.is_some() {
                        existing.reordered = delta.reordered;
                    }
                }
                None => self.scenarios = Some(delta),
            }
        }
        if let Some(delta) = other.options {
            match &mut self.options {
                Some(existing) => {
                    existing.added.extend(delta.added);
                    existing.removed.extend(delta.removed);
                    existing.patched.extend(delta.patched);
                    if delta.reordered.is_some() {
                        existing.reordered = delta.reordered;
                    }
                }
                None => self.options = Some(delta),
            }
        }
        if let Some(delta) = other.decisions {
            match &mut self.decisions {
                Some(existing) => {
                    existing.added.extend(delta.added);
                    existing.removed.extend(delta.removed);
                    existing.patched.extend(delta.patched);
                    if delta.reordered.is_some() {
                        existing.reordered = delta.reordered;
                    }
                }
                None => self.decisions = Some(delta),
            }
        }
        if let Some(delta) = other.validations {
            match &mut self.validations {
                Some(existing) => {
                    existing.added.extend(delta.added);
                    existing.removed.extend(delta.removed);
                    existing.patched.extend(delta.patched);
                    if delta.reordered.is_some() {
                        existing.reordered = delta.reordered;
                    }
                }
                None => self.validations = Some(delta),
            }
        }
        if let Some(delta) = other.performance {
            match &mut self.performance {
                Some(existing) => {
                    existing.added.extend(delta.added);
                    existing.removed.extend(delta.removed);
                    existing.patched.extend(delta.patched);
                    if delta.reordered.is_some() {
                        existing.reordered = delta.reordered;
                    }
                }
                None => self.performance = Some(delta),
            }
        }
        if let Some(delta) = other.quality {
            match &mut self.quality {
                Some(existing) => {
                    existing.added.extend(delta.added);
                    existing.removed.extend(delta.removed);
                    existing.patched.extend(delta.patched);
                    if delta.reordered.is_some() {
                        existing.reordered = delta.reordered;
                    }
                }
                None => self.quality = Some(delta),
            }
        }
        if let Some(delta) = other.artifacts {
            match &mut self.artifacts {
                Some(existing) => {
                    existing.added.extend(delta.added);
                    existing.removed.extend(delta.removed);
                    existing.patched.extend(delta.patched);
                    if delta.reordered.is_some() {
                        existing.reordered = delta.reordered;
                    }
                }
                None => self.artifacts = Some(delta),
            }
        }
        if let Some(delta) = other.assumptions {
            match &mut self.assumptions {
                Some(existing) => {
                    existing.added.extend(delta.added);
                    existing.removed.extend(delta.removed);
                    existing.patched.extend(delta.patched);
                    if delta.reordered.is_some() {
                        existing.reordered = delta.reordered;
                    }
                }
                None => self.assumptions = Some(delta),
            }
        }
        if let Some(delta) = other.constraints {
            match &mut self.constraints {
                Some(existing) => {
                    existing.added.extend(delta.added);
                    existing.removed.extend(delta.removed);
                    existing.patched.extend(delta.patched);
                    if delta.reordered.is_some() {
                        existing.reordered = delta.reordered;
                    }
                }
                None => self.constraints = Some(delta),
            }
        }
        if let Some(delta) = other.compliance_records {
            match &mut self.compliance_records {
                Some(existing) => {
                    existing.added.extend(delta.added);
                    existing.removed.extend(delta.removed);
                    existing.patched.extend(delta.patched);
                    if delta.reordered.is_some() {
                        existing.reordered = delta.reordered;
                    }
                }
                None => self.compliance_records = Some(delta),
            }
        }
        if let Some(delta) = other.approvals {
            match &mut self.approvals {
                Some(existing) => {
                    existing.added.extend(delta.added);
                    existing.removed.extend(delta.removed);
                    existing.patched.extend(delta.patched);
                    if delta.reordered.is_some() {
                        existing.reordered = delta.reordered;
                    }
                }
                None => self.approvals = Some(delta),
            }
        }
        if let Some(delta) = other.meetings {
            match &mut self.meetings {
                Some(existing) => {
                    existing.added.extend(delta.added);
                    existing.removed.extend(delta.removed);
                    existing.patched.extend(delta.patched);
                    if delta.reordered.is_some() {
                        existing.reordered = delta.reordered;
                    }
                }
                None => self.meetings = Some(delta),
            }
        }
        if let Some(delta) = other.changes {
            match &mut self.changes {
                Some(existing) => {
                    existing.added.extend(delta.added);
                    existing.removed.extend(delta.removed);
                    existing.patched.extend(delta.patched);
                    if delta.reordered.is_some() {
                        existing.reordered = delta.reordered;
                    }
                }
                None => self.changes = Some(delta),
            }
        }
        if let Some(delta) = other.collaboration {
            match &mut self.collaboration {
                Some(existing) => {
                    existing.added.extend(delta.added);
                    existing.removed.extend(delta.removed);
                    existing.patched.extend(delta.patched);
                    if delta.reordered.is_some() {
                        existing.reordered = delta.reordered;
                    }
                }
                None => self.collaboration = Some(delta),
            }
        }
        if let Some(delta) = other.analyses {
            match &mut self.analyses {
                Some(existing) => {
                    existing.added.extend(delta.added);
                    existing.removed.extend(delta.removed);
                    existing.patched.extend(delta.patched);
                    if delta.reordered.is_some() {
                        existing.reordered = delta.reordered;
                    }
                }
                None => self.analyses = Some(delta),
            }
        }
        if let Some(delta) = other.reports {
            match &mut self.reports {
                Some(existing) => {
                    existing.added.extend(delta.added);
                    existing.removed.extend(delta.removed);
                    existing.patched.extend(delta.patched);
                    if delta.reordered.is_some() {
                        existing.reordered = delta.reordered;
                    }
                }
                None => self.reports = Some(delta),
            }
        }
        if let Some(delta) = other.search_filters {
            match &mut self.search_filters {
                Some(existing) => {
                    existing.added.extend(delta.added);
                    existing.removed.extend(delta.removed);
                    existing.patched.extend(delta.patched);
                    if delta.reordered.is_some() {
                        existing.reordered = delta.reordered;
                    }
                }
                None => self.search_filters = Some(delta),
            }
        }
        if let Some(delta) = other.status_records {
            match &mut self.status_records {
                Some(existing) => {
                    existing.added.extend(delta.added);
                    existing.removed.extend(delta.removed);
                    existing.patched.extend(delta.patched);
                    if delta.reordered.is_some() {
                        existing.reordered = delta.reordered;
                    }
                }
                None => self.status_records = Some(delta),
            }
        }
        if let Some(delta) = other.workshops {
            match &mut self.workshops {
                Some(existing) => {
                    existing.added.extend(delta.added);
                    existing.removed.extend(delta.removed);
                    existing.patched.extend(delta.patched);
                    if delta.reordered.is_some() {
                        existing.reordered = delta.reordered;
                    }
                }
                None => self.workshops = Some(delta),
            }
        }
        if let Some(delta) = other.surveys {
            match &mut self.surveys {
                Some(existing) => {
                    existing.added.extend(delta.added);
                    existing.removed.extend(delta.removed);
                    existing.patched.extend(delta.patched);
                    if delta.reordered.is_some() {
                        existing.reordered = delta.reordered;
                    }
                }
                None => self.surveys = Some(delta),
            }
        }
        if let Some(delta) = other.issues {
            match &mut self.issues {
                Some(existing) => {
                    existing.added.extend(delta.added);
                    existing.removed.extend(delta.removed);
                    existing.patched.extend(delta.patched);
                    if delta.reordered.is_some() {
                        existing.reordered = delta.reordered;
                    }
                }
                None => self.issues = Some(delta),
            }
        }
        if let Some(delta) = other.audit_events {
            match &mut self.audit_events {
                Some(existing) => {
                    existing.added.extend(delta.added);
                    existing.removed.extend(delta.removed);
                    existing.patched.extend(delta.patched);
                    if delta.reordered.is_some() {
                        existing.reordered = delta.reordered;
                    }
                }
                None => self.audit_events = Some(delta),
            }
        }
        if let Some(delta) = other.templates {
            match &mut self.templates {
                Some(existing) => {
                    existing.added.extend(delta.added);
                    existing.removed.extend(delta.removed);
                    existing.patched.extend(delta.patched);
                    if delta.reordered.is_some() {
                        existing.reordered = delta.reordered;
                    }
                }
                None => self.templates = Some(delta),
            }
        }
        if other.knowledge_payload.is_some() {
            self.knowledge_payload = other.knowledge_payload;
        }
        if other.knowledge.is_some() {
            self.knowledge = other.knowledge;
        }
        if other.benchmarks_payload.is_some() {
            self.benchmarks_payload = other.benchmarks_payload;
        }
        if other.benchmarks.is_some() {
            self.benchmarks = other.benchmarks;
        }
        if let Some(delta) = other.traces {
            match &mut self.traces {
                Some(existing) => {
                    existing.added.extend(delta.added);
                    existing.removed.extend(delta.removed);
                    existing.patched.extend(delta.patched);
                    if delta.reordered.is_some() {
                        existing.reordered = delta.reordered;
                    }
                }
                None => self.traces = Some(delta),
            }
        }
    }
}

fn apply_collection_delta<T, P>(items: &mut Vec<T>, added: &[T], removed: &[String], patched: &[(String, P)], reordered: &Option<Vec<String>>) -> protocol::MutationApplyResult<()>
where
    T: Identified<EntityId> + Clone + Patchable<P>,
    P: Clone,
{
    for (index, id) in removed.iter().enumerate() {
        let eid = EntityId(id.clone());
        if !items.iter().any(|item| item.id() == &eid) {
            return Err(protocol::MutationApplyError::new("mutation.apply.missing-target", "removed entity does not exist").at(["removed".to_string(), index.to_string()]));
        }
        if removed[..index].contains(id) {
            return Err(protocol::MutationApplyError::new("mutation.apply.duplicate-target", "entity is removed more than once").at(["removed".to_string(), index.to_string()]));
        }
    }
    for (index, item) in added.iter().enumerate() {
        if items.iter().any(|existing| existing.id() == item.id()) || added[..index].iter().any(|existing| existing.id() == item.id()) {
            return Err(protocol::MutationApplyError::new("mutation.apply.duplicate-target", "added entity identity already exists").at(["added".to_string(), index.to_string()]));
        }
    }
    // 🧲️ A patch may address an entity this SAME delta adds: `MutationDiff::absorb` folds a
    // `create` and a later `patch` of that entity into one delta (`added` + `patched`), and the
    // `absorb(d1, d2).apply(base) == d2.apply(&d1.apply(base))` law only holds if the fold's own
    // additions are visible to its own patches — hence `added` is applied BEFORE `patched` below too.
    for (index, (id, _)) in patched.iter().enumerate() {
        let eid = EntityId(id.clone());
        if !items.iter().any(|item| item.id() == &eid) && !added.iter().any(|item| item.id() == &eid) {
            return Err(protocol::MutationApplyError::new("mutation.apply.missing-target", "patched entity does not exist").at(["patched".to_string(), index.to_string()]));
        }
        if removed.contains(id) {
            return Err(protocol::MutationApplyError::new("mutation.apply.conflicting-target", "entity cannot be removed and patched").at(["patched".to_string(), index.to_string()]));
        }
        if patched[..index].iter().any(|(prior, _)| prior == id) {
            return Err(protocol::MutationApplyError::new("mutation.apply.duplicate-target", "entity is patched more than once").at(["patched".to_string(), index.to_string()]));
        }
    }
    let mut candidate = items.clone();
    for id in removed {
        let eid = EntityId(id.clone());
        candidate.retain(|item| item.id() != &eid);
    }
    candidate.extend(added.iter().cloned());
    for (id, patch) in patched {
        let eid = EntityId(id.clone());
        candidate.iter_mut().find(|item| item.id() == &eid).ok_or_else(|| protocol::MutationApplyError::new("mutation.apply.missing-target", "patched entity does not exist").at(["patched".to_string(), id.clone()]))?.apply_patch(patch);
    }
    for (index, item) in candidate.iter().enumerate() {
        if candidate[..index].iter().any(|prior| prior.id() == item.id()) {
            return Err(protocol::MutationApplyError::new("mutation.apply.duplicate-target", "patch produced a duplicate entity identity").at(["patched"]));
        }
    }
    if let Some(order) = reordered {
        if order.len() != candidate.len() || order.iter().enumerate().any(|(index, id)| order[..index].contains(id) || !candidate.iter().any(|item| item.id().0 == *id)) {
            return Err(protocol::MutationApplyError::new("mutation.apply.invalid-order", "entity reorder must be a complete unique permutation").at(["reordered"]));
        }
        let mut map: std::collections::BTreeMap<String, T> = std::collections::BTreeMap::new();
        for item in candidate.drain(..) {
            map.insert(item.id().0.clone(), item);
        }
        for id in order {
            candidate.push(map.remove(id).ok_or_else(|| protocol::MutationApplyError::new("mutation.apply.missing-target", "reordered entity does not exist").at(["reordered".to_string(), id.clone()]))?);
        }
    }
    *items = candidate;
    Ok(())
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
