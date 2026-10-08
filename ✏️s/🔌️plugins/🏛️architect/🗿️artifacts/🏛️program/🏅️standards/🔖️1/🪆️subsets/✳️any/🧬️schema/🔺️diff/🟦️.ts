/** 🧬️ ProgramSnapshot diff schema — sparse field delta. */

import { architectProgramArtifactGuardExactObject, PROGRAM_ARTIFACT_FIELDS } from "../🟦️.ts";
import type { AccessRule, AccessibilityRequirement, Activity, Adjacency, AnalysisRecord, ApprovalRecord, ArtifactRecord, Assumption, AuditEvent, BenchmarkRecord, ChangeRecord, CollaborationRecord, CommunicationRequirement, ComplianceRecord, Conflict, ConstraintRecord, CostRequirement, Decision, DeliveryConstraint, EnvironmentalRequirement, Equipment, FlexibilityRequirement, FlowRequirement, Function, Governance, GrowthPlan, HumanFactorRequirement, InformationRequirement, InfrastructureRequirement, Issue, KnowledgeRecord, MeetingRecord, OperationalRequirement, OptionEvaluation, OrganizationalRequirement, PerformanceCriterion, PriorityRecord, PrivacyRequirement, Process, ProgramElement, ProgramMeta, ProjectDefinition, QualityRecord, QuantityRequirement, RegulatoryRequirement, Relationship, ReportRecord, Requirement, ResilienceRequirement, Resource, Risk, SafetyRequirement, Scenario, ScheduleRequirement, SearchFilter, SecurityRequirement, ServiceRequirement, SiteContext, Stakeholder, StatusRecord, StorageRequirement, Survey, SustainabilityRequirement, TemplateRecord, TraceLink, UserProfile, ValidationRecord, WayfindingRequirement, Workshop } from "../🟦️.ts";

export interface ProgramDiff {
  /** @state artifact */
  schema: string | null;
  /** @state artifact */
  meta: ProgramMetaEdit | null;
  /** @state artifact */
  project: ProjectDefinitionEdit | null;
  /** @state artifact */
  stakeholders: ProgramStakeholdersDelta | null;
  /** @state artifact */
  users: ProgramUsersDelta | null;
  /** @state artifact */
  activities: ProgramActivitiesDelta | null;
  /** @state artifact */
  functions: ProgramFunctionsDelta | null;
  /** @state artifact */
  elements: ProgramElementsDelta | null;
  /** @state artifact */
  quantities: ProgramQuantitiesDelta | null;
  /** @state artifact */
  relationships: ProgramRelationshipsDelta | null;
  /** @state artifact */
  adjacencies: ProgramAdjacenciesDelta | null;
  /** @state artifact */
  processes: ProgramProcessesDelta | null;
  /** @state artifact */
  flows: ProgramFlowsDelta | null;
  /** @state artifact */
  accessRules: ProgramAccessRulesDelta | null;
  /** @state artifact */
  operations: ProgramOperationsDelta | null;
  /** @state artifact */
  equipment: ProgramEquipmentDelta | null;
  /** @state artifact */
  resources: ProgramResourcesDelta | null;
  /** @state artifact */
  storage: ProgramStorageDelta | null;
  /** @state artifact */
  environmental: ProgramEnvironmentalDelta | null;
  /** @state artifact */
  humanFactors: ProgramHumanFactorsDelta | null;
  /** @state artifact */
  accessibility: ProgramAccessibilityDelta | null;
  /** @state artifact */
  privacy: ProgramPrivacyDelta | null;
  /** @state artifact */
  safety: ProgramSafetyDelta | null;
  /** @state artifact */
  security: ProgramSecurityDelta | null;
  /** @state artifact */
  regulatory: ProgramRegulatoryDelta | null;
  /** @state artifact */
  siteContext: ProgramSiteContextDelta | null;
  /** @state artifact */
  organizational: ProgramOrganizationalDelta | null;
  /** @state artifact */
  services: ProgramServicesDelta | null;
  /** @state artifact */
  infrastructure: ProgramInfrastructureDelta | null;
  /** @state artifact */
  information: ProgramInformationDelta | null;
  /** @state artifact */
  communication: ProgramCommunicationDelta | null;
  /** @state artifact */
  wayfinding: ProgramWayfindingDelta | null;
  /** @state artifact */
  schedules: ProgramSchedulesDelta | null;
  /** @state artifact */
  flexibility: ProgramFlexibilityDelta | null;
  /** @state artifact */
  growth: ProgramGrowthDelta | null;
  /** @state artifact */
  sustainability: ProgramSustainabilityDelta | null;
  /** @state artifact */
  resilience: ProgramResilienceDelta | null;
  /** @state artifact */
  costs: ProgramCostsDelta | null;
  /** @state artifact */
  delivery: ProgramDeliveryDelta | null;
  /** @state artifact */
  risks: ProgramRisksDelta | null;
  /** @state artifact */
  conflicts: ProgramConflictsDelta | null;
  /** @state artifact */
  requirements: ProgramRequirementsDelta | null;
  /** @state artifact */
  priorities: ProgramPrioritiesDelta | null;
  /** @state artifact */
  scenarios: ProgramScenariosDelta | null;
  /** @state artifact */
  options: ProgramOptionsDelta | null;
  /** @state artifact */
  decisions: ProgramDecisionsDelta | null;
  /** @state artifact */
  validations: ProgramValidationsDelta | null;
  /** @state artifact */
  performance: ProgramPerformanceDelta | null;
  /** @state artifact */
  quality: ProgramQualityDelta | null;
  /** @state artifact */
  artifacts: ProgramArtifactsDelta | null;
  /** @state artifact */
  assumptions: ProgramAssumptionsDelta | null;
  /** @state artifact */
  constraints: ProgramConstraintsDelta | null;
  /** @state artifact */
  complianceRecords: ProgramComplianceRecordsDelta | null;
  /** @state artifact */
  approvals: ProgramApprovalsDelta | null;
  /** @state artifact */
  meetings: ProgramMeetingsDelta | null;
  /** @state artifact */
  changes: ProgramChangesDelta | null;
  /** @state artifact */
  collaboration: ProgramCollaborationDelta | null;
  /** @state artifact */
  analyses: ProgramAnalysesDelta | null;
  /** @state artifact */
  reports: ProgramReportsDelta | null;
  /** @state artifact */
  searchFilters: ProgramSearchFiltersDelta | null;
  /** @state artifact */
  statusRecords: ProgramStatusRecordsDelta | null;
  /** @state artifact */
  workshops: ProgramWorkshopsDelta | null;
  /** @state artifact */
  surveys: ProgramSurveysDelta | null;
  /** @state artifact */
  issues: ProgramIssuesDelta | null;
  /** @state artifact */
  auditEvents: ProgramAuditEventsDelta | null;
  /** @state artifact */
  templates: ProgramTemplatesDelta | null;
  /** @state artifact */
  knowledge: ProgramKnowledgeDelta | null;
  /** @state artifact */
  benchmarks: ProgramBenchmarksDelta | null;
  /** @state artifact */
  traces: ProgramTracesDelta | null;
  /** @state artifact */
  governance: GovernanceEdit | null;
}

export const PROGRAM_DIFF_FIELDS = PROGRAM_ARTIFACT_FIELDS.filter((field) => field !== "knowledgePayload" && field !== "benchmarksPayload");

export function parseProgramDiff(value: unknown, at = "$"): ProgramDiff {
  const row = architectProgramArtifactGuardExactObject(value, at, PROGRAM_DIFF_FIELDS);
  return row as unknown as ProgramDiff;
}

export interface ProgramStakeholdersDelta {
  removed: ProgramStakeholdersRemoval[];
  inserted: ProgramStakeholdersInsertion[];
  moved: ProgramStakeholdersRelocation[];
  modified: ProgramStakeholdersPatchEntry[];
}

export interface ProgramStakeholdersInsertion {
  index: number;
  row: Stakeholder;
}

export interface ProgramStakeholdersRemoval {
  id: string;
  index: number;
}

export interface ProgramStakeholdersRelocation {
  id: string;
  from: number;
  to: number;
}

export interface ProgramStakeholdersPatchEntry {
  id: string;
  patch: StakeholderPatch;
}

export interface ProgramUsersDelta {
  removed: ProgramUsersRemoval[];
  inserted: ProgramUsersInsertion[];
  moved: ProgramUsersRelocation[];
  modified: ProgramUsersPatchEntry[];
}

export interface ProgramUsersInsertion {
  index: number;
  row: UserProfile;
}

export interface ProgramUsersRemoval {
  id: string;
  index: number;
}

export interface ProgramUsersRelocation {
  id: string;
  from: number;
  to: number;
}

export interface ProgramUsersPatchEntry {
  id: string;
  patch: UserProfilePatch;
}

export interface ProgramActivitiesDelta {
  removed: ProgramActivitiesRemoval[];
  inserted: ProgramActivitiesInsertion[];
  moved: ProgramActivitiesRelocation[];
  modified: ProgramActivitiesPatchEntry[];
}

export interface ProgramActivitiesInsertion {
  index: number;
  row: Activity;
}

export interface ProgramActivitiesRemoval {
  id: string;
  index: number;
}

export interface ProgramActivitiesRelocation {
  id: string;
  from: number;
  to: number;
}

export interface ProgramActivitiesPatchEntry {
  id: string;
  patch: ActivityPatch;
}

export interface ProgramFunctionsDelta {
  removed: ProgramFunctionsRemoval[];
  inserted: ProgramFunctionsInsertion[];
  moved: ProgramFunctionsRelocation[];
  modified: ProgramFunctionsPatchEntry[];
}

export interface ProgramFunctionsInsertion {
  index: number;
  row: Function;
}

export interface ProgramFunctionsRemoval {
  id: string;
  index: number;
}

export interface ProgramFunctionsRelocation {
  id: string;
  from: number;
  to: number;
}

export interface ProgramFunctionsPatchEntry {
  id: string;
  patch: FunctionPatch;
}

export interface ProgramElementsDelta {
  removed: ProgramElementsRemoval[];
  inserted: ProgramElementsInsertion[];
  moved: ProgramElementsRelocation[];
  modified: ProgramElementsPatchEntry[];
}

export interface ProgramElementsInsertion {
  index: number;
  row: ProgramElement;
}

export interface ProgramElementsRemoval {
  id: string;
  index: number;
}

export interface ProgramElementsRelocation {
  id: string;
  from: number;
  to: number;
}

export interface ProgramElementsPatchEntry {
  id: string;
  patch: ProgramElementPatch;
}

export interface ProgramQuantitiesDelta {
  removed: ProgramQuantitiesRemoval[];
  inserted: ProgramQuantitiesInsertion[];
  moved: ProgramQuantitiesRelocation[];
  modified: ProgramQuantitiesPatchEntry[];
}

export interface ProgramQuantitiesInsertion {
  index: number;
  row: QuantityRequirement;
}

export interface ProgramQuantitiesRemoval {
  id: string;
  index: number;
}

export interface ProgramQuantitiesRelocation {
  id: string;
  from: number;
  to: number;
}

export interface ProgramQuantitiesPatchEntry {
  id: string;
  patch: QuantityRequirementPatch;
}

export interface ProgramRelationshipsDelta {
  removed: ProgramRelationshipsRemoval[];
  inserted: ProgramRelationshipsInsertion[];
  moved: ProgramRelationshipsRelocation[];
  modified: ProgramRelationshipsPatchEntry[];
}

export interface ProgramRelationshipsInsertion {
  index: number;
  row: Relationship;
}

export interface ProgramRelationshipsRemoval {
  id: string;
  index: number;
}

export interface ProgramRelationshipsRelocation {
  id: string;
  from: number;
  to: number;
}

export interface ProgramRelationshipsPatchEntry {
  id: string;
  patch: RelationshipPatch;
}

export interface ProgramAdjacenciesDelta {
  removed: ProgramAdjacenciesRemoval[];
  inserted: ProgramAdjacenciesInsertion[];
  moved: ProgramAdjacenciesRelocation[];
  modified: ProgramAdjacenciesPatchEntry[];
}

export interface ProgramAdjacenciesInsertion {
  index: number;
  row: Adjacency;
}

export interface ProgramAdjacenciesRemoval {
  id: string;
  index: number;
}

export interface ProgramAdjacenciesRelocation {
  id: string;
  from: number;
  to: number;
}

export interface ProgramAdjacenciesPatchEntry {
  id: string;
  patch: AdjacencyPatch;
}

export interface ProgramProcessesDelta {
  removed: ProgramProcessesRemoval[];
  inserted: ProgramProcessesInsertion[];
  moved: ProgramProcessesRelocation[];
  modified: ProgramProcessesPatchEntry[];
}

export interface ProgramProcessesInsertion {
  index: number;
  row: Process;
}

export interface ProgramProcessesRemoval {
  id: string;
  index: number;
}

export interface ProgramProcessesRelocation {
  id: string;
  from: number;
  to: number;
}

export interface ProgramProcessesPatchEntry {
  id: string;
  patch: ProcessPatch;
}

export interface ProgramFlowsDelta {
  removed: ProgramFlowsRemoval[];
  inserted: ProgramFlowsInsertion[];
  moved: ProgramFlowsRelocation[];
  modified: ProgramFlowsPatchEntry[];
}

export interface ProgramFlowsInsertion {
  index: number;
  row: FlowRequirement;
}

export interface ProgramFlowsRemoval {
  id: string;
  index: number;
}

export interface ProgramFlowsRelocation {
  id: string;
  from: number;
  to: number;
}

export interface ProgramFlowsPatchEntry {
  id: string;
  patch: FlowRequirementPatch;
}

export interface ProgramAccessRulesDelta {
  removed: ProgramAccessRulesRemoval[];
  inserted: ProgramAccessRulesInsertion[];
  moved: ProgramAccessRulesRelocation[];
  modified: ProgramAccessRulesPatchEntry[];
}

export interface ProgramAccessRulesInsertion {
  index: number;
  row: AccessRule;
}

export interface ProgramAccessRulesRemoval {
  id: string;
  index: number;
}

export interface ProgramAccessRulesRelocation {
  id: string;
  from: number;
  to: number;
}

export interface ProgramAccessRulesPatchEntry {
  id: string;
  patch: AccessRulePatch;
}

export interface ProgramOperationsDelta {
  removed: ProgramOperationsRemoval[];
  inserted: ProgramOperationsInsertion[];
  moved: ProgramOperationsRelocation[];
  modified: ProgramOperationsPatchEntry[];
}

export interface ProgramOperationsInsertion {
  index: number;
  row: OperationalRequirement;
}

export interface ProgramOperationsRemoval {
  id: string;
  index: number;
}

export interface ProgramOperationsRelocation {
  id: string;
  from: number;
  to: number;
}

export interface ProgramOperationsPatchEntry {
  id: string;
  patch: OperationalRequirementPatch;
}

export interface ProgramEquipmentDelta {
  removed: ProgramEquipmentRemoval[];
  inserted: ProgramEquipmentInsertion[];
  moved: ProgramEquipmentRelocation[];
  modified: ProgramEquipmentPatchEntry[];
}

export interface ProgramEquipmentInsertion {
  index: number;
  row: Equipment;
}

export interface ProgramEquipmentRemoval {
  id: string;
  index: number;
}

export interface ProgramEquipmentRelocation {
  id: string;
  from: number;
  to: number;
}

export interface ProgramEquipmentPatchEntry {
  id: string;
  patch: EquipmentPatch;
}

export interface ProgramResourcesDelta {
  removed: ProgramResourcesRemoval[];
  inserted: ProgramResourcesInsertion[];
  moved: ProgramResourcesRelocation[];
  modified: ProgramResourcesPatchEntry[];
}

export interface ProgramResourcesInsertion {
  index: number;
  row: Resource;
}

export interface ProgramResourcesRemoval {
  id: string;
  index: number;
}

export interface ProgramResourcesRelocation {
  id: string;
  from: number;
  to: number;
}

export interface ProgramResourcesPatchEntry {
  id: string;
  patch: ResourcePatch;
}

export interface ProgramStorageDelta {
  removed: ProgramStorageRemoval[];
  inserted: ProgramStorageInsertion[];
  moved: ProgramStorageRelocation[];
  modified: ProgramStoragePatchEntry[];
}

export interface ProgramStorageInsertion {
  index: number;
  row: StorageRequirement;
}

export interface ProgramStorageRemoval {
  id: string;
  index: number;
}

export interface ProgramStorageRelocation {
  id: string;
  from: number;
  to: number;
}

export interface ProgramStoragePatchEntry {
  id: string;
  patch: StorageRequirementPatch;
}

export interface ProgramEnvironmentalDelta {
  removed: ProgramEnvironmentalRemoval[];
  inserted: ProgramEnvironmentalInsertion[];
  moved: ProgramEnvironmentalRelocation[];
  modified: ProgramEnvironmentalPatchEntry[];
}

export interface ProgramEnvironmentalInsertion {
  index: number;
  row: EnvironmentalRequirement;
}

export interface ProgramEnvironmentalRemoval {
  id: string;
  index: number;
}

export interface ProgramEnvironmentalRelocation {
  id: string;
  from: number;
  to: number;
}

export interface ProgramEnvironmentalPatchEntry {
  id: string;
  patch: EnvironmentalRequirementPatch;
}

export interface ProgramHumanFactorsDelta {
  removed: ProgramHumanFactorsRemoval[];
  inserted: ProgramHumanFactorsInsertion[];
  moved: ProgramHumanFactorsRelocation[];
  modified: ProgramHumanFactorsPatchEntry[];
}

export interface ProgramHumanFactorsInsertion {
  index: number;
  row: HumanFactorRequirement;
}

export interface ProgramHumanFactorsRemoval {
  id: string;
  index: number;
}

export interface ProgramHumanFactorsRelocation {
  id: string;
  from: number;
  to: number;
}

export interface ProgramHumanFactorsPatchEntry {
  id: string;
  patch: HumanFactorRequirementPatch;
}

export interface ProgramAccessibilityDelta {
  removed: ProgramAccessibilityRemoval[];
  inserted: ProgramAccessibilityInsertion[];
  moved: ProgramAccessibilityRelocation[];
  modified: ProgramAccessibilityPatchEntry[];
}

export interface ProgramAccessibilityInsertion {
  index: number;
  row: AccessibilityRequirement;
}

export interface ProgramAccessibilityRemoval {
  id: string;
  index: number;
}

export interface ProgramAccessibilityRelocation {
  id: string;
  from: number;
  to: number;
}

export interface ProgramAccessibilityPatchEntry {
  id: string;
  patch: AccessibilityRequirementPatch;
}

export interface ProgramPrivacyDelta {
  removed: ProgramPrivacyRemoval[];
  inserted: ProgramPrivacyInsertion[];
  moved: ProgramPrivacyRelocation[];
  modified: ProgramPrivacyPatchEntry[];
}

export interface ProgramPrivacyInsertion {
  index: number;
  row: PrivacyRequirement;
}

export interface ProgramPrivacyRemoval {
  id: string;
  index: number;
}

export interface ProgramPrivacyRelocation {
  id: string;
  from: number;
  to: number;
}

export interface ProgramPrivacyPatchEntry {
  id: string;
  patch: PrivacyRequirementPatch;
}

export interface ProgramSafetyDelta {
  removed: ProgramSafetyRemoval[];
  inserted: ProgramSafetyInsertion[];
  moved: ProgramSafetyRelocation[];
  modified: ProgramSafetyPatchEntry[];
}

export interface ProgramSafetyInsertion {
  index: number;
  row: SafetyRequirement;
}

export interface ProgramSafetyRemoval {
  id: string;
  index: number;
}

export interface ProgramSafetyRelocation {
  id: string;
  from: number;
  to: number;
}

export interface ProgramSafetyPatchEntry {
  id: string;
  patch: SafetyRequirementPatch;
}

export interface ProgramSecurityDelta {
  removed: ProgramSecurityRemoval[];
  inserted: ProgramSecurityInsertion[];
  moved: ProgramSecurityRelocation[];
  modified: ProgramSecurityPatchEntry[];
}

export interface ProgramSecurityInsertion {
  index: number;
  row: SecurityRequirement;
}

export interface ProgramSecurityRemoval {
  id: string;
  index: number;
}

export interface ProgramSecurityRelocation {
  id: string;
  from: number;
  to: number;
}

export interface ProgramSecurityPatchEntry {
  id: string;
  patch: SecurityRequirementPatch;
}

export interface ProgramRegulatoryDelta {
  removed: ProgramRegulatoryRemoval[];
  inserted: ProgramRegulatoryInsertion[];
  moved: ProgramRegulatoryRelocation[];
  modified: ProgramRegulatoryPatchEntry[];
}

export interface ProgramRegulatoryInsertion {
  index: number;
  row: RegulatoryRequirement;
}

export interface ProgramRegulatoryRemoval {
  id: string;
  index: number;
}

export interface ProgramRegulatoryRelocation {
  id: string;
  from: number;
  to: number;
}

export interface ProgramRegulatoryPatchEntry {
  id: string;
  patch: RegulatoryRequirementPatch;
}

export interface ProgramSiteContextDelta {
  removed: ProgramSiteContextRemoval[];
  inserted: ProgramSiteContextInsertion[];
  moved: ProgramSiteContextRelocation[];
  modified: ProgramSiteContextPatchEntry[];
}

export interface ProgramSiteContextInsertion {
  index: number;
  row: SiteContext;
}

export interface ProgramSiteContextRemoval {
  id: string;
  index: number;
}

export interface ProgramSiteContextRelocation {
  id: string;
  from: number;
  to: number;
}

export interface ProgramSiteContextPatchEntry {
  id: string;
  patch: SiteContextPatch;
}

export interface ProgramOrganizationalDelta {
  removed: ProgramOrganizationalRemoval[];
  inserted: ProgramOrganizationalInsertion[];
  moved: ProgramOrganizationalRelocation[];
  modified: ProgramOrganizationalPatchEntry[];
}

export interface ProgramOrganizationalInsertion {
  index: number;
  row: OrganizationalRequirement;
}

export interface ProgramOrganizationalRemoval {
  id: string;
  index: number;
}

export interface ProgramOrganizationalRelocation {
  id: string;
  from: number;
  to: number;
}

export interface ProgramOrganizationalPatchEntry {
  id: string;
  patch: OrganizationalRequirementPatch;
}

export interface ProgramServicesDelta {
  removed: ProgramServicesRemoval[];
  inserted: ProgramServicesInsertion[];
  moved: ProgramServicesRelocation[];
  modified: ProgramServicesPatchEntry[];
}

export interface ProgramServicesInsertion {
  index: number;
  row: ServiceRequirement;
}

export interface ProgramServicesRemoval {
  id: string;
  index: number;
}

export interface ProgramServicesRelocation {
  id: string;
  from: number;
  to: number;
}

export interface ProgramServicesPatchEntry {
  id: string;
  patch: ServiceRequirementPatch;
}

export interface ProgramInfrastructureDelta {
  removed: ProgramInfrastructureRemoval[];
  inserted: ProgramInfrastructureInsertion[];
  moved: ProgramInfrastructureRelocation[];
  modified: ProgramInfrastructurePatchEntry[];
}

export interface ProgramInfrastructureInsertion {
  index: number;
  row: InfrastructureRequirement;
}

export interface ProgramInfrastructureRemoval {
  id: string;
  index: number;
}

export interface ProgramInfrastructureRelocation {
  id: string;
  from: number;
  to: number;
}

export interface ProgramInfrastructurePatchEntry {
  id: string;
  patch: InfrastructureRequirementPatch;
}

export interface ProgramInformationDelta {
  removed: ProgramInformationRemoval[];
  inserted: ProgramInformationInsertion[];
  moved: ProgramInformationRelocation[];
  modified: ProgramInformationPatchEntry[];
}

export interface ProgramInformationInsertion {
  index: number;
  row: InformationRequirement;
}

export interface ProgramInformationRemoval {
  id: string;
  index: number;
}

export interface ProgramInformationRelocation {
  id: string;
  from: number;
  to: number;
}

export interface ProgramInformationPatchEntry {
  id: string;
  patch: InformationRequirementPatch;
}

export interface ProgramCommunicationDelta {
  removed: ProgramCommunicationRemoval[];
  inserted: ProgramCommunicationInsertion[];
  moved: ProgramCommunicationRelocation[];
  modified: ProgramCommunicationPatchEntry[];
}

export interface ProgramCommunicationInsertion {
  index: number;
  row: CommunicationRequirement;
}

export interface ProgramCommunicationRemoval {
  id: string;
  index: number;
}

export interface ProgramCommunicationRelocation {
  id: string;
  from: number;
  to: number;
}

export interface ProgramCommunicationPatchEntry {
  id: string;
  patch: CommunicationRequirementPatch;
}

export interface ProgramWayfindingDelta {
  removed: ProgramWayfindingRemoval[];
  inserted: ProgramWayfindingInsertion[];
  moved: ProgramWayfindingRelocation[];
  modified: ProgramWayfindingPatchEntry[];
}

export interface ProgramWayfindingInsertion {
  index: number;
  row: WayfindingRequirement;
}

export interface ProgramWayfindingRemoval {
  id: string;
  index: number;
}

export interface ProgramWayfindingRelocation {
  id: string;
  from: number;
  to: number;
}

export interface ProgramWayfindingPatchEntry {
  id: string;
  patch: WayfindingRequirementPatch;
}

export interface ProgramSchedulesDelta {
  removed: ProgramSchedulesRemoval[];
  inserted: ProgramSchedulesInsertion[];
  moved: ProgramSchedulesRelocation[];
  modified: ProgramSchedulesPatchEntry[];
}

export interface ProgramSchedulesInsertion {
  index: number;
  row: ScheduleRequirement;
}

export interface ProgramSchedulesRemoval {
  id: string;
  index: number;
}

export interface ProgramSchedulesRelocation {
  id: string;
  from: number;
  to: number;
}

export interface ProgramSchedulesPatchEntry {
  id: string;
  patch: ScheduleRequirementPatch;
}

export interface ProgramFlexibilityDelta {
  removed: ProgramFlexibilityRemoval[];
  inserted: ProgramFlexibilityInsertion[];
  moved: ProgramFlexibilityRelocation[];
  modified: ProgramFlexibilityPatchEntry[];
}

export interface ProgramFlexibilityInsertion {
  index: number;
  row: FlexibilityRequirement;
}

export interface ProgramFlexibilityRemoval {
  id: string;
  index: number;
}

export interface ProgramFlexibilityRelocation {
  id: string;
  from: number;
  to: number;
}

export interface ProgramFlexibilityPatchEntry {
  id: string;
  patch: FlexibilityRequirementPatch;
}

export interface ProgramGrowthDelta {
  removed: ProgramGrowthRemoval[];
  inserted: ProgramGrowthInsertion[];
  moved: ProgramGrowthRelocation[];
  modified: ProgramGrowthPatchEntry[];
}

export interface ProgramGrowthInsertion {
  index: number;
  row: GrowthPlan;
}

export interface ProgramGrowthRemoval {
  id: string;
  index: number;
}

export interface ProgramGrowthRelocation {
  id: string;
  from: number;
  to: number;
}

export interface ProgramGrowthPatchEntry {
  id: string;
  patch: GrowthPlanPatch;
}

export interface ProgramSustainabilityDelta {
  removed: ProgramSustainabilityRemoval[];
  inserted: ProgramSustainabilityInsertion[];
  moved: ProgramSustainabilityRelocation[];
  modified: ProgramSustainabilityPatchEntry[];
}

export interface ProgramSustainabilityInsertion {
  index: number;
  row: SustainabilityRequirement;
}

export interface ProgramSustainabilityRemoval {
  id: string;
  index: number;
}

export interface ProgramSustainabilityRelocation {
  id: string;
  from: number;
  to: number;
}

export interface ProgramSustainabilityPatchEntry {
  id: string;
  patch: SustainabilityRequirementPatch;
}

export interface ProgramResilienceDelta {
  removed: ProgramResilienceRemoval[];
  inserted: ProgramResilienceInsertion[];
  moved: ProgramResilienceRelocation[];
  modified: ProgramResiliencePatchEntry[];
}

export interface ProgramResilienceInsertion {
  index: number;
  row: ResilienceRequirement;
}

export interface ProgramResilienceRemoval {
  id: string;
  index: number;
}

export interface ProgramResilienceRelocation {
  id: string;
  from: number;
  to: number;
}

export interface ProgramResiliencePatchEntry {
  id: string;
  patch: ResilienceRequirementPatch;
}

export interface ProgramCostsDelta {
  removed: ProgramCostsRemoval[];
  inserted: ProgramCostsInsertion[];
  moved: ProgramCostsRelocation[];
  modified: ProgramCostsPatchEntry[];
}

export interface ProgramCostsInsertion {
  index: number;
  row: CostRequirement;
}

export interface ProgramCostsRemoval {
  id: string;
  index: number;
}

export interface ProgramCostsRelocation {
  id: string;
  from: number;
  to: number;
}

export interface ProgramCostsPatchEntry {
  id: string;
  patch: CostRequirementPatch;
}

export interface ProgramDeliveryDelta {
  removed: ProgramDeliveryRemoval[];
  inserted: ProgramDeliveryInsertion[];
  moved: ProgramDeliveryRelocation[];
  modified: ProgramDeliveryPatchEntry[];
}

export interface ProgramDeliveryInsertion {
  index: number;
  row: DeliveryConstraint;
}

export interface ProgramDeliveryRemoval {
  id: string;
  index: number;
}

export interface ProgramDeliveryRelocation {
  id: string;
  from: number;
  to: number;
}

export interface ProgramDeliveryPatchEntry {
  id: string;
  patch: DeliveryConstraintPatch;
}

export interface ProgramRisksDelta {
  removed: ProgramRisksRemoval[];
  inserted: ProgramRisksInsertion[];
  moved: ProgramRisksRelocation[];
  modified: ProgramRisksPatchEntry[];
}

export interface ProgramRisksInsertion {
  index: number;
  row: Risk;
}

export interface ProgramRisksRemoval {
  id: string;
  index: number;
}

export interface ProgramRisksRelocation {
  id: string;
  from: number;
  to: number;
}

export interface ProgramRisksPatchEntry {
  id: string;
  patch: RiskPatch;
}

export interface ProgramConflictsDelta {
  removed: ProgramConflictsRemoval[];
  inserted: ProgramConflictsInsertion[];
  moved: ProgramConflictsRelocation[];
  modified: ProgramConflictsPatchEntry[];
}

export interface ProgramConflictsInsertion {
  index: number;
  row: Conflict;
}

export interface ProgramConflictsRemoval {
  id: string;
  index: number;
}

export interface ProgramConflictsRelocation {
  id: string;
  from: number;
  to: number;
}

export interface ProgramConflictsPatchEntry {
  id: string;
  patch: ConflictPatch;
}

export interface ProgramRequirementsDelta {
  removed: ProgramRequirementsRemoval[];
  inserted: ProgramRequirementsInsertion[];
  moved: ProgramRequirementsRelocation[];
  modified: ProgramRequirementsPatchEntry[];
}

export interface ProgramRequirementsInsertion {
  index: number;
  row: Requirement;
}

export interface ProgramRequirementsRemoval {
  id: string;
  index: number;
}

export interface ProgramRequirementsRelocation {
  id: string;
  from: number;
  to: number;
}

export interface ProgramRequirementsPatchEntry {
  id: string;
  patch: RequirementPatch;
}

export interface ProgramPrioritiesDelta {
  removed: ProgramPrioritiesRemoval[];
  inserted: ProgramPrioritiesInsertion[];
  moved: ProgramPrioritiesRelocation[];
  modified: ProgramPrioritiesPatchEntry[];
}

export interface ProgramPrioritiesInsertion {
  index: number;
  row: PriorityRecord;
}

export interface ProgramPrioritiesRemoval {
  id: string;
  index: number;
}

export interface ProgramPrioritiesRelocation {
  id: string;
  from: number;
  to: number;
}

export interface ProgramPrioritiesPatchEntry {
  id: string;
  patch: PriorityRecordPatch;
}

export interface ProgramScenariosDelta {
  removed: ProgramScenariosRemoval[];
  inserted: ProgramScenariosInsertion[];
  moved: ProgramScenariosRelocation[];
  modified: ProgramScenariosPatchEntry[];
}

export interface ProgramScenariosInsertion {
  index: number;
  row: Scenario;
}

export interface ProgramScenariosRemoval {
  id: string;
  index: number;
}

export interface ProgramScenariosRelocation {
  id: string;
  from: number;
  to: number;
}

export interface ProgramScenariosPatchEntry {
  id: string;
  patch: ScenarioPatch;
}

export interface ProgramOptionsDelta {
  removed: ProgramOptionsRemoval[];
  inserted: ProgramOptionsInsertion[];
  moved: ProgramOptionsRelocation[];
  modified: ProgramOptionsPatchEntry[];
}

export interface ProgramOptionsInsertion {
  index: number;
  row: OptionEvaluation;
}

export interface ProgramOptionsRemoval {
  id: string;
  index: number;
}

export interface ProgramOptionsRelocation {
  id: string;
  from: number;
  to: number;
}

export interface ProgramOptionsPatchEntry {
  id: string;
  patch: OptionEvaluationPatch;
}

export interface ProgramDecisionsDelta {
  removed: ProgramDecisionsRemoval[];
  inserted: ProgramDecisionsInsertion[];
  moved: ProgramDecisionsRelocation[];
  modified: ProgramDecisionsPatchEntry[];
}

export interface ProgramDecisionsInsertion {
  index: number;
  row: Decision;
}

export interface ProgramDecisionsRemoval {
  id: string;
  index: number;
}

export interface ProgramDecisionsRelocation {
  id: string;
  from: number;
  to: number;
}

export interface ProgramDecisionsPatchEntry {
  id: string;
  patch: DecisionPatch;
}

export interface ProgramValidationsDelta {
  removed: ProgramValidationsRemoval[];
  inserted: ProgramValidationsInsertion[];
  moved: ProgramValidationsRelocation[];
  modified: ProgramValidationsPatchEntry[];
}

export interface ProgramValidationsInsertion {
  index: number;
  row: ValidationRecord;
}

export interface ProgramValidationsRemoval {
  id: string;
  index: number;
}

export interface ProgramValidationsRelocation {
  id: string;
  from: number;
  to: number;
}

export interface ProgramValidationsPatchEntry {
  id: string;
  patch: ValidationRecordPatch;
}

export interface ProgramPerformanceDelta {
  removed: ProgramPerformanceRemoval[];
  inserted: ProgramPerformanceInsertion[];
  moved: ProgramPerformanceRelocation[];
  modified: ProgramPerformancePatchEntry[];
}

export interface ProgramPerformanceInsertion {
  index: number;
  row: PerformanceCriterion;
}

export interface ProgramPerformanceRemoval {
  id: string;
  index: number;
}

export interface ProgramPerformanceRelocation {
  id: string;
  from: number;
  to: number;
}

export interface ProgramPerformancePatchEntry {
  id: string;
  patch: PerformanceCriterionPatch;
}

export interface ProgramQualityDelta {
  removed: ProgramQualityRemoval[];
  inserted: ProgramQualityInsertion[];
  moved: ProgramQualityRelocation[];
  modified: ProgramQualityPatchEntry[];
}

export interface ProgramQualityInsertion {
  index: number;
  row: QualityRecord;
}

export interface ProgramQualityRemoval {
  id: string;
  index: number;
}

export interface ProgramQualityRelocation {
  id: string;
  from: number;
  to: number;
}

export interface ProgramQualityPatchEntry {
  id: string;
  patch: QualityRecordPatch;
}

export interface ProgramArtifactsDelta {
  removed: ProgramArtifactsRemoval[];
  inserted: ProgramArtifactsInsertion[];
  moved: ProgramArtifactsRelocation[];
  modified: ProgramArtifactsPatchEntry[];
}

export interface ProgramArtifactsInsertion {
  index: number;
  row: ArtifactRecord;
}

export interface ProgramArtifactsRemoval {
  id: string;
  index: number;
}

export interface ProgramArtifactsRelocation {
  id: string;
  from: number;
  to: number;
}

export interface ProgramArtifactsPatchEntry {
  id: string;
  patch: ArtifactRecordPatch;
}

export interface ProgramAssumptionsDelta {
  removed: ProgramAssumptionsRemoval[];
  inserted: ProgramAssumptionsInsertion[];
  moved: ProgramAssumptionsRelocation[];
  modified: ProgramAssumptionsPatchEntry[];
}

export interface ProgramAssumptionsInsertion {
  index: number;
  row: Assumption;
}

export interface ProgramAssumptionsRemoval {
  id: string;
  index: number;
}

export interface ProgramAssumptionsRelocation {
  id: string;
  from: number;
  to: number;
}

export interface ProgramAssumptionsPatchEntry {
  id: string;
  patch: AssumptionPatch;
}

export interface ProgramConstraintsDelta {
  removed: ProgramConstraintsRemoval[];
  inserted: ProgramConstraintsInsertion[];
  moved: ProgramConstraintsRelocation[];
  modified: ProgramConstraintsPatchEntry[];
}

export interface ProgramConstraintsInsertion {
  index: number;
  row: ConstraintRecord;
}

export interface ProgramConstraintsRemoval {
  id: string;
  index: number;
}

export interface ProgramConstraintsRelocation {
  id: string;
  from: number;
  to: number;
}

export interface ProgramConstraintsPatchEntry {
  id: string;
  patch: ConstraintRecordPatch;
}

export interface ProgramComplianceRecordsDelta {
  removed: ProgramComplianceRecordsRemoval[];
  inserted: ProgramComplianceRecordsInsertion[];
  moved: ProgramComplianceRecordsRelocation[];
  modified: ProgramComplianceRecordsPatchEntry[];
}

export interface ProgramComplianceRecordsInsertion {
  index: number;
  row: ComplianceRecord;
}

export interface ProgramComplianceRecordsRemoval {
  id: string;
  index: number;
}

export interface ProgramComplianceRecordsRelocation {
  id: string;
  from: number;
  to: number;
}

export interface ProgramComplianceRecordsPatchEntry {
  id: string;
  patch: ComplianceRecordPatch;
}

export interface ProgramApprovalsDelta {
  removed: ProgramApprovalsRemoval[];
  inserted: ProgramApprovalsInsertion[];
  moved: ProgramApprovalsRelocation[];
  modified: ProgramApprovalsPatchEntry[];
}

export interface ProgramApprovalsInsertion {
  index: number;
  row: ApprovalRecord;
}

export interface ProgramApprovalsRemoval {
  id: string;
  index: number;
}

export interface ProgramApprovalsRelocation {
  id: string;
  from: number;
  to: number;
}

export interface ProgramApprovalsPatchEntry {
  id: string;
  patch: ApprovalRecordPatch;
}

export interface ProgramMeetingsDelta {
  removed: ProgramMeetingsRemoval[];
  inserted: ProgramMeetingsInsertion[];
  moved: ProgramMeetingsRelocation[];
  modified: ProgramMeetingsPatchEntry[];
}

export interface ProgramMeetingsInsertion {
  index: number;
  row: MeetingRecord;
}

export interface ProgramMeetingsRemoval {
  id: string;
  index: number;
}

export interface ProgramMeetingsRelocation {
  id: string;
  from: number;
  to: number;
}

export interface ProgramMeetingsPatchEntry {
  id: string;
  patch: MeetingRecordPatch;
}

export interface ProgramChangesDelta {
  removed: ProgramChangesRemoval[];
  inserted: ProgramChangesInsertion[];
  moved: ProgramChangesRelocation[];
  modified: ProgramChangesPatchEntry[];
}

export interface ProgramChangesInsertion {
  index: number;
  row: ChangeRecord;
}

export interface ProgramChangesRemoval {
  id: string;
  index: number;
}

export interface ProgramChangesRelocation {
  id: string;
  from: number;
  to: number;
}

export interface ProgramChangesPatchEntry {
  id: string;
  patch: ChangeRecordPatch;
}

export interface ProgramCollaborationDelta {
  removed: ProgramCollaborationRemoval[];
  inserted: ProgramCollaborationInsertion[];
  moved: ProgramCollaborationRelocation[];
  modified: ProgramCollaborationPatchEntry[];
}

export interface ProgramCollaborationInsertion {
  index: number;
  row: CollaborationRecord;
}

export interface ProgramCollaborationRemoval {
  id: string;
  index: number;
}

export interface ProgramCollaborationRelocation {
  id: string;
  from: number;
  to: number;
}

export interface ProgramCollaborationPatchEntry {
  id: string;
  patch: CollaborationRecordPatch;
}

export interface ProgramAnalysesDelta {
  removed: ProgramAnalysesRemoval[];
  inserted: ProgramAnalysesInsertion[];
  moved: ProgramAnalysesRelocation[];
  modified: ProgramAnalysesPatchEntry[];
}

export interface ProgramAnalysesInsertion {
  index: number;
  row: AnalysisRecord;
}

export interface ProgramAnalysesRemoval {
  id: string;
  index: number;
}

export interface ProgramAnalysesRelocation {
  id: string;
  from: number;
  to: number;
}

export interface ProgramAnalysesPatchEntry {
  id: string;
  patch: AnalysisRecordPatch;
}

export interface ProgramReportsDelta {
  removed: ProgramReportsRemoval[];
  inserted: ProgramReportsInsertion[];
  moved: ProgramReportsRelocation[];
  modified: ProgramReportsPatchEntry[];
}

export interface ProgramReportsInsertion {
  index: number;
  row: ReportRecord;
}

export interface ProgramReportsRemoval {
  id: string;
  index: number;
}

export interface ProgramReportsRelocation {
  id: string;
  from: number;
  to: number;
}

export interface ProgramReportsPatchEntry {
  id: string;
  patch: ReportRecordPatch;
}

export interface ProgramSearchFiltersDelta {
  removed: ProgramSearchFiltersRemoval[];
  inserted: ProgramSearchFiltersInsertion[];
  moved: ProgramSearchFiltersRelocation[];
  modified: ProgramSearchFiltersPatchEntry[];
}

export interface ProgramSearchFiltersInsertion {
  index: number;
  row: SearchFilter;
}

export interface ProgramSearchFiltersRemoval {
  id: string;
  index: number;
}

export interface ProgramSearchFiltersRelocation {
  id: string;
  from: number;
  to: number;
}

export interface ProgramSearchFiltersPatchEntry {
  id: string;
  patch: SearchFilterPatch;
}

export interface ProgramStatusRecordsDelta {
  removed: ProgramStatusRecordsRemoval[];
  inserted: ProgramStatusRecordsInsertion[];
  moved: ProgramStatusRecordsRelocation[];
  modified: ProgramStatusRecordsPatchEntry[];
}

export interface ProgramStatusRecordsInsertion {
  index: number;
  row: StatusRecord;
}

export interface ProgramStatusRecordsRemoval {
  id: string;
  index: number;
}

export interface ProgramStatusRecordsRelocation {
  id: string;
  from: number;
  to: number;
}

export interface ProgramStatusRecordsPatchEntry {
  id: string;
  patch: StatusRecordPatch;
}

export interface ProgramWorkshopsDelta {
  removed: ProgramWorkshopsRemoval[];
  inserted: ProgramWorkshopsInsertion[];
  moved: ProgramWorkshopsRelocation[];
  modified: ProgramWorkshopsPatchEntry[];
}

export interface ProgramWorkshopsInsertion {
  index: number;
  row: Workshop;
}

export interface ProgramWorkshopsRemoval {
  id: string;
  index: number;
}

export interface ProgramWorkshopsRelocation {
  id: string;
  from: number;
  to: number;
}

export interface ProgramWorkshopsPatchEntry {
  id: string;
  patch: WorkshopPatch;
}

export interface ProgramSurveysDelta {
  removed: ProgramSurveysRemoval[];
  inserted: ProgramSurveysInsertion[];
  moved: ProgramSurveysRelocation[];
  modified: ProgramSurveysPatchEntry[];
}

export interface ProgramSurveysInsertion {
  index: number;
  row: Survey;
}

export interface ProgramSurveysRemoval {
  id: string;
  index: number;
}

export interface ProgramSurveysRelocation {
  id: string;
  from: number;
  to: number;
}

export interface ProgramSurveysPatchEntry {
  id: string;
  patch: SurveyPatch;
}

export interface ProgramIssuesDelta {
  removed: ProgramIssuesRemoval[];
  inserted: ProgramIssuesInsertion[];
  moved: ProgramIssuesRelocation[];
  modified: ProgramIssuesPatchEntry[];
}

export interface ProgramIssuesInsertion {
  index: number;
  row: Issue;
}

export interface ProgramIssuesRemoval {
  id: string;
  index: number;
}

export interface ProgramIssuesRelocation {
  id: string;
  from: number;
  to: number;
}

export interface ProgramIssuesPatchEntry {
  id: string;
  patch: IssuePatch;
}

export interface ProgramAuditEventsDelta {
  removed: ProgramAuditEventsRemoval[];
  inserted: ProgramAuditEventsInsertion[];
  moved: ProgramAuditEventsRelocation[];
  modified: ProgramAuditEventsPatchEntry[];
}

export interface ProgramAuditEventsInsertion {
  index: number;
  row: AuditEvent;
}

export interface ProgramAuditEventsRemoval {
  id: string;
  index: number;
}

export interface ProgramAuditEventsRelocation {
  id: string;
  from: number;
  to: number;
}

export interface ProgramAuditEventsPatchEntry {
  id: string;
  patch: AuditEventPatch;
}

export interface ProgramTemplatesDelta {
  removed: ProgramTemplatesRemoval[];
  inserted: ProgramTemplatesInsertion[];
  moved: ProgramTemplatesRelocation[];
  modified: ProgramTemplatesPatchEntry[];
}

export interface ProgramTemplatesInsertion {
  index: number;
  row: TemplateRecord;
}

export interface ProgramTemplatesRemoval {
  id: string;
  index: number;
}

export interface ProgramTemplatesRelocation {
  id: string;
  from: number;
  to: number;
}

export interface ProgramTemplatesPatchEntry {
  id: string;
  patch: TemplateRecordPatch;
}

export interface ProgramTracesDelta {
  removed: ProgramTracesRemoval[];
  inserted: ProgramTracesInsertion[];
  moved: ProgramTracesRelocation[];
  modified: ProgramTracesPatchEntry[];
}

export interface ProgramTracesInsertion {
  index: number;
  row: TraceLink;
}

export interface ProgramTracesRemoval {
  id: string;
  index: number;
}

export interface ProgramTracesRelocation {
  id: string;
  from: number;
  to: number;
}

export interface ProgramTracesPatchEntry {
  id: string;
  patch: TraceLinkPatch;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class architectProgramDiffGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const architectProgramDiffGuardReject = (at: string, why: string): never => {
  throw new architectProgramDiffGuardRefusal(at, why);
};

type architectProgramDiffGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type architectProgramDiffGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type architectProgramDiffGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const architectProgramDiffGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : architectProgramDiffGuardReject(at, "value is not an object");
export const architectProgramDiffGuardArray = (value: unknown, at: string, bounds: architectProgramDiffGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return architectProgramDiffGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) architectProgramDiffGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) architectProgramDiffGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const architectProgramDiffGuardString = (value: unknown, at: string, bounds: architectProgramDiffGuardTextBounds = {}): string => {
  if (typeof value !== "string") return architectProgramDiffGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) architectProgramDiffGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) architectProgramDiffGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) architectProgramDiffGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const architectProgramDiffGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : architectProgramDiffGuardReject(at, "value is not a boolean"));
export const architectProgramDiffGuardNumber = (value: unknown, at: string, bounds: architectProgramDiffGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return architectProgramDiffGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) architectProgramDiffGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) architectProgramDiffGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const architectProgramDiffGuardInteger = (value: unknown, at: string, bounds: architectProgramDiffGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? architectProgramDiffGuardNumber(value, at, bounds) : architectProgramDiffGuardReject(at, "value is not an integer");
export const architectProgramDiffGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : architectProgramDiffGuardReject(at, `value is not one of ${members.join(", ")}`);
export const architectProgramDiffGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : architectProgramDiffGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export type StakeholderPatch = Readonly<Record<string, unknown>>;

export function parseStakeholderPatch(value: unknown, at = "$"): StakeholderPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramStakeholdersPatchEntry(value: unknown, at = "$"): ProgramStakeholdersPatchEntry {
  const row = architectProgramDiffGuardObject(value, at);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseStakeholderPatch(row["patch"], `${at}.patch`),
  };
}

export type UserProfilePatch = Readonly<Record<string, unknown>>;

export function parseUserProfilePatch(value: unknown, at = "$"): UserProfilePatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramUsersPatchEntry(value: unknown, at = "$"): ProgramUsersPatchEntry {
  const row = architectProgramDiffGuardObject(value, at);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseUserProfilePatch(row["patch"], `${at}.patch`),
  };
}

export type ActivityPatch = Readonly<Record<string, unknown>>;

export function parseActivityPatch(value: unknown, at = "$"): ActivityPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramActivitiesPatchEntry(value: unknown, at = "$"): ProgramActivitiesPatchEntry {
  const row = architectProgramDiffGuardObject(value, at);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseActivityPatch(row["patch"], `${at}.patch`),
  };
}

export type FunctionPatch = Readonly<Record<string, unknown>>;

export function parseFunctionPatch(value: unknown, at = "$"): FunctionPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramFunctionsPatchEntry(value: unknown, at = "$"): ProgramFunctionsPatchEntry {
  const row = architectProgramDiffGuardObject(value, at);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseFunctionPatch(row["patch"], `${at}.patch`),
  };
}

export type ProgramElementPatch = Readonly<Record<string, unknown>>;

export function parseProgramElementPatch(value: unknown, at = "$"): ProgramElementPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramElementsPatchEntry(value: unknown, at = "$"): ProgramElementsPatchEntry {
  const row = architectProgramDiffGuardObject(value, at);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseProgramElementPatch(row["patch"], `${at}.patch`),
  };
}

export type QuantityRequirementPatch = Readonly<Record<string, unknown>>;

export function parseQuantityRequirementPatch(value: unknown, at = "$"): QuantityRequirementPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramQuantitiesPatchEntry(value: unknown, at = "$"): ProgramQuantitiesPatchEntry {
  const row = architectProgramDiffGuardObject(value, at);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseQuantityRequirementPatch(row["patch"], `${at}.patch`),
  };
}

export type RelationshipPatch = Readonly<Record<string, unknown>>;

export function parseRelationshipPatch(value: unknown, at = "$"): RelationshipPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramRelationshipsPatchEntry(value: unknown, at = "$"): ProgramRelationshipsPatchEntry {
  const row = architectProgramDiffGuardObject(value, at);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseRelationshipPatch(row["patch"], `${at}.patch`),
  };
}

export type AdjacencyPatch = Readonly<Record<string, unknown>>;

export function parseAdjacencyPatch(value: unknown, at = "$"): AdjacencyPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramAdjacenciesPatchEntry(value: unknown, at = "$"): ProgramAdjacenciesPatchEntry {
  const row = architectProgramDiffGuardObject(value, at);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseAdjacencyPatch(row["patch"], `${at}.patch`),
  };
}

export type ProcessPatch = Readonly<Record<string, unknown>>;

export function parseProcessPatch(value: unknown, at = "$"): ProcessPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramProcessesPatchEntry(value: unknown, at = "$"): ProgramProcessesPatchEntry {
  const row = architectProgramDiffGuardObject(value, at);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseProcessPatch(row["patch"], `${at}.patch`),
  };
}

export type FlowRequirementPatch = Readonly<Record<string, unknown>>;

export function parseFlowRequirementPatch(value: unknown, at = "$"): FlowRequirementPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramFlowsPatchEntry(value: unknown, at = "$"): ProgramFlowsPatchEntry {
  const row = architectProgramDiffGuardObject(value, at);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseFlowRequirementPatch(row["patch"], `${at}.patch`),
  };
}

export type AccessRulePatch = Readonly<Record<string, unknown>>;

export function parseAccessRulePatch(value: unknown, at = "$"): AccessRulePatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramAccessRulesPatchEntry(value: unknown, at = "$"): ProgramAccessRulesPatchEntry {
  const row = architectProgramDiffGuardObject(value, at);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseAccessRulePatch(row["patch"], `${at}.patch`),
  };
}

export type OperationalRequirementPatch = Readonly<Record<string, unknown>>;

export function parseOperationalRequirementPatch(value: unknown, at = "$"): OperationalRequirementPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramOperationsPatchEntry(value: unknown, at = "$"): ProgramOperationsPatchEntry {
  const row = architectProgramDiffGuardObject(value, at);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseOperationalRequirementPatch(row["patch"], `${at}.patch`),
  };
}

export type EquipmentPatch = Readonly<Record<string, unknown>>;

export function parseEquipmentPatch(value: unknown, at = "$"): EquipmentPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramEquipmentPatchEntry(value: unknown, at = "$"): ProgramEquipmentPatchEntry {
  const row = architectProgramDiffGuardObject(value, at);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseEquipmentPatch(row["patch"], `${at}.patch`),
  };
}

export type ResourcePatch = Readonly<Record<string, unknown>>;

export function parseResourcePatch(value: unknown, at = "$"): ResourcePatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramResourcesPatchEntry(value: unknown, at = "$"): ProgramResourcesPatchEntry {
  const row = architectProgramDiffGuardObject(value, at);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseResourcePatch(row["patch"], `${at}.patch`),
  };
}

export type StorageRequirementPatch = Readonly<Record<string, unknown>>;

export function parseStorageRequirementPatch(value: unknown, at = "$"): StorageRequirementPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramStoragePatchEntry(value: unknown, at = "$"): ProgramStoragePatchEntry {
  const row = architectProgramDiffGuardObject(value, at);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseStorageRequirementPatch(row["patch"], `${at}.patch`),
  };
}

export type EnvironmentalRequirementPatch = Readonly<Record<string, unknown>>;

export function parseEnvironmentalRequirementPatch(value: unknown, at = "$"): EnvironmentalRequirementPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramEnvironmentalPatchEntry(value: unknown, at = "$"): ProgramEnvironmentalPatchEntry {
  const row = architectProgramDiffGuardObject(value, at);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseEnvironmentalRequirementPatch(row["patch"], `${at}.patch`),
  };
}

export type HumanFactorRequirementPatch = Readonly<Record<string, unknown>>;

export function parseHumanFactorRequirementPatch(value: unknown, at = "$"): HumanFactorRequirementPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramHumanFactorsPatchEntry(value: unknown, at = "$"): ProgramHumanFactorsPatchEntry {
  const row = architectProgramDiffGuardObject(value, at);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseHumanFactorRequirementPatch(row["patch"], `${at}.patch`),
  };
}

export type AccessibilityRequirementPatch = Readonly<Record<string, unknown>>;

export function parseAccessibilityRequirementPatch(value: unknown, at = "$"): AccessibilityRequirementPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramAccessibilityPatchEntry(value: unknown, at = "$"): ProgramAccessibilityPatchEntry {
  const row = architectProgramDiffGuardObject(value, at);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseAccessibilityRequirementPatch(row["patch"], `${at}.patch`),
  };
}

export type PrivacyRequirementPatch = Readonly<Record<string, unknown>>;

export function parsePrivacyRequirementPatch(value: unknown, at = "$"): PrivacyRequirementPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramPrivacyPatchEntry(value: unknown, at = "$"): ProgramPrivacyPatchEntry {
  const row = architectProgramDiffGuardObject(value, at);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parsePrivacyRequirementPatch(row["patch"], `${at}.patch`),
  };
}

export type SafetyRequirementPatch = Readonly<Record<string, unknown>>;

export function parseSafetyRequirementPatch(value: unknown, at = "$"): SafetyRequirementPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramSafetyPatchEntry(value: unknown, at = "$"): ProgramSafetyPatchEntry {
  const row = architectProgramDiffGuardObject(value, at);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseSafetyRequirementPatch(row["patch"], `${at}.patch`),
  };
}

export type SecurityRequirementPatch = Readonly<Record<string, unknown>>;

export function parseSecurityRequirementPatch(value: unknown, at = "$"): SecurityRequirementPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramSecurityPatchEntry(value: unknown, at = "$"): ProgramSecurityPatchEntry {
  const row = architectProgramDiffGuardObject(value, at);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseSecurityRequirementPatch(row["patch"], `${at}.patch`),
  };
}

export type RegulatoryRequirementPatch = Readonly<Record<string, unknown>>;

export function parseRegulatoryRequirementPatch(value: unknown, at = "$"): RegulatoryRequirementPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramRegulatoryPatchEntry(value: unknown, at = "$"): ProgramRegulatoryPatchEntry {
  const row = architectProgramDiffGuardObject(value, at);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseRegulatoryRequirementPatch(row["patch"], `${at}.patch`),
  };
}

export type SiteContextPatch = Readonly<Record<string, unknown>>;

export function parseSiteContextPatch(value: unknown, at = "$"): SiteContextPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramSiteContextPatchEntry(value: unknown, at = "$"): ProgramSiteContextPatchEntry {
  const row = architectProgramDiffGuardObject(value, at);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseSiteContextPatch(row["patch"], `${at}.patch`),
  };
}

export type OrganizationalRequirementPatch = Readonly<Record<string, unknown>>;

export function parseOrganizationalRequirementPatch(value: unknown, at = "$"): OrganizationalRequirementPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramOrganizationalPatchEntry(value: unknown, at = "$"): ProgramOrganizationalPatchEntry {
  const row = architectProgramDiffGuardObject(value, at);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseOrganizationalRequirementPatch(row["patch"], `${at}.patch`),
  };
}

export type ServiceRequirementPatch = Readonly<Record<string, unknown>>;

export function parseServiceRequirementPatch(value: unknown, at = "$"): ServiceRequirementPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramServicesPatchEntry(value: unknown, at = "$"): ProgramServicesPatchEntry {
  const row = architectProgramDiffGuardObject(value, at);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseServiceRequirementPatch(row["patch"], `${at}.patch`),
  };
}

export type InfrastructureRequirementPatch = Readonly<Record<string, unknown>>;

export function parseInfrastructureRequirementPatch(value: unknown, at = "$"): InfrastructureRequirementPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramInfrastructurePatchEntry(value: unknown, at = "$"): ProgramInfrastructurePatchEntry {
  const row = architectProgramDiffGuardObject(value, at);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseInfrastructureRequirementPatch(row["patch"], `${at}.patch`),
  };
}

export type InformationRequirementPatch = Readonly<Record<string, unknown>>;

export function parseInformationRequirementPatch(value: unknown, at = "$"): InformationRequirementPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramInformationPatchEntry(value: unknown, at = "$"): ProgramInformationPatchEntry {
  const row = architectProgramDiffGuardObject(value, at);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseInformationRequirementPatch(row["patch"], `${at}.patch`),
  };
}

export type CommunicationRequirementPatch = Readonly<Record<string, unknown>>;

export function parseCommunicationRequirementPatch(value: unknown, at = "$"): CommunicationRequirementPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramCommunicationPatchEntry(value: unknown, at = "$"): ProgramCommunicationPatchEntry {
  const row = architectProgramDiffGuardObject(value, at);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseCommunicationRequirementPatch(row["patch"], `${at}.patch`),
  };
}

export type WayfindingRequirementPatch = Readonly<Record<string, unknown>>;

export function parseWayfindingRequirementPatch(value: unknown, at = "$"): WayfindingRequirementPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramWayfindingPatchEntry(value: unknown, at = "$"): ProgramWayfindingPatchEntry {
  const row = architectProgramDiffGuardObject(value, at);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseWayfindingRequirementPatch(row["patch"], `${at}.patch`),
  };
}

export type ScheduleRequirementPatch = Readonly<Record<string, unknown>>;

export function parseScheduleRequirementPatch(value: unknown, at = "$"): ScheduleRequirementPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramSchedulesPatchEntry(value: unknown, at = "$"): ProgramSchedulesPatchEntry {
  const row = architectProgramDiffGuardObject(value, at);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseScheduleRequirementPatch(row["patch"], `${at}.patch`),
  };
}

export type FlexibilityRequirementPatch = Readonly<Record<string, unknown>>;

export function parseFlexibilityRequirementPatch(value: unknown, at = "$"): FlexibilityRequirementPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramFlexibilityPatchEntry(value: unknown, at = "$"): ProgramFlexibilityPatchEntry {
  const row = architectProgramDiffGuardObject(value, at);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseFlexibilityRequirementPatch(row["patch"], `${at}.patch`),
  };
}

export type GrowthPlanPatch = Readonly<Record<string, unknown>>;

export function parseGrowthPlanPatch(value: unknown, at = "$"): GrowthPlanPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramGrowthPatchEntry(value: unknown, at = "$"): ProgramGrowthPatchEntry {
  const row = architectProgramDiffGuardObject(value, at);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseGrowthPlanPatch(row["patch"], `${at}.patch`),
  };
}

export type SustainabilityRequirementPatch = Readonly<Record<string, unknown>>;

export function parseSustainabilityRequirementPatch(value: unknown, at = "$"): SustainabilityRequirementPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramSustainabilityPatchEntry(value: unknown, at = "$"): ProgramSustainabilityPatchEntry {
  const row = architectProgramDiffGuardObject(value, at);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseSustainabilityRequirementPatch(row["patch"], `${at}.patch`),
  };
}

export type ResilienceRequirementPatch = Readonly<Record<string, unknown>>;

export function parseResilienceRequirementPatch(value: unknown, at = "$"): ResilienceRequirementPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramResiliencePatchEntry(value: unknown, at = "$"): ProgramResiliencePatchEntry {
  const row = architectProgramDiffGuardObject(value, at);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseResilienceRequirementPatch(row["patch"], `${at}.patch`),
  };
}

export type CostRequirementPatch = Readonly<Record<string, unknown>>;

export function parseCostRequirementPatch(value: unknown, at = "$"): CostRequirementPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramCostsPatchEntry(value: unknown, at = "$"): ProgramCostsPatchEntry {
  const row = architectProgramDiffGuardObject(value, at);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseCostRequirementPatch(row["patch"], `${at}.patch`),
  };
}

export type DeliveryConstraintPatch = Readonly<Record<string, unknown>>;

export function parseDeliveryConstraintPatch(value: unknown, at = "$"): DeliveryConstraintPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramDeliveryPatchEntry(value: unknown, at = "$"): ProgramDeliveryPatchEntry {
  const row = architectProgramDiffGuardObject(value, at);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseDeliveryConstraintPatch(row["patch"], `${at}.patch`),
  };
}

export type RiskPatch = Readonly<Record<string, unknown>>;

export function parseRiskPatch(value: unknown, at = "$"): RiskPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramRisksPatchEntry(value: unknown, at = "$"): ProgramRisksPatchEntry {
  const row = architectProgramDiffGuardObject(value, at);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseRiskPatch(row["patch"], `${at}.patch`),
  };
}

export type ConflictPatch = Readonly<Record<string, unknown>>;

export function parseConflictPatch(value: unknown, at = "$"): ConflictPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramConflictsPatchEntry(value: unknown, at = "$"): ProgramConflictsPatchEntry {
  const row = architectProgramDiffGuardObject(value, at);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseConflictPatch(row["patch"], `${at}.patch`),
  };
}

export type RequirementPatch = Readonly<Record<string, unknown>>;

export function parseRequirementPatch(value: unknown, at = "$"): RequirementPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramRequirementsPatchEntry(value: unknown, at = "$"): ProgramRequirementsPatchEntry {
  const row = architectProgramDiffGuardObject(value, at);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseRequirementPatch(row["patch"], `${at}.patch`),
  };
}

export type PriorityRecordPatch = Readonly<Record<string, unknown>>;

export function parsePriorityRecordPatch(value: unknown, at = "$"): PriorityRecordPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramPrioritiesPatchEntry(value: unknown, at = "$"): ProgramPrioritiesPatchEntry {
  const row = architectProgramDiffGuardObject(value, at);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parsePriorityRecordPatch(row["patch"], `${at}.patch`),
  };
}

export type ScenarioPatch = Readonly<Record<string, unknown>>;

export function parseScenarioPatch(value: unknown, at = "$"): ScenarioPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramScenariosPatchEntry(value: unknown, at = "$"): ProgramScenariosPatchEntry {
  const row = architectProgramDiffGuardObject(value, at);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseScenarioPatch(row["patch"], `${at}.patch`),
  };
}

export type OptionEvaluationPatch = Readonly<Record<string, unknown>>;

export function parseOptionEvaluationPatch(value: unknown, at = "$"): OptionEvaluationPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramOptionsPatchEntry(value: unknown, at = "$"): ProgramOptionsPatchEntry {
  const row = architectProgramDiffGuardObject(value, at);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseOptionEvaluationPatch(row["patch"], `${at}.patch`),
  };
}

export type DecisionPatch = Readonly<Record<string, unknown>>;

export function parseDecisionPatch(value: unknown, at = "$"): DecisionPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramDecisionsPatchEntry(value: unknown, at = "$"): ProgramDecisionsPatchEntry {
  const row = architectProgramDiffGuardObject(value, at);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseDecisionPatch(row["patch"], `${at}.patch`),
  };
}

export type ValidationRecordPatch = Readonly<Record<string, unknown>>;

export function parseValidationRecordPatch(value: unknown, at = "$"): ValidationRecordPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramValidationsPatchEntry(value: unknown, at = "$"): ProgramValidationsPatchEntry {
  const row = architectProgramDiffGuardObject(value, at);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseValidationRecordPatch(row["patch"], `${at}.patch`),
  };
}

export type PerformanceCriterionPatch = Readonly<Record<string, unknown>>;

export function parsePerformanceCriterionPatch(value: unknown, at = "$"): PerformanceCriterionPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramPerformancePatchEntry(value: unknown, at = "$"): ProgramPerformancePatchEntry {
  const row = architectProgramDiffGuardObject(value, at);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parsePerformanceCriterionPatch(row["patch"], `${at}.patch`),
  };
}

export type QualityRecordPatch = Readonly<Record<string, unknown>>;

export function parseQualityRecordPatch(value: unknown, at = "$"): QualityRecordPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramQualityPatchEntry(value: unknown, at = "$"): ProgramQualityPatchEntry {
  const row = architectProgramDiffGuardObject(value, at);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseQualityRecordPatch(row["patch"], `${at}.patch`),
  };
}

export type ArtifactRecordPatch = Readonly<Record<string, unknown>>;

export function parseArtifactRecordPatch(value: unknown, at = "$"): ArtifactRecordPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramArtifactsPatchEntry(value: unknown, at = "$"): ProgramArtifactsPatchEntry {
  const row = architectProgramDiffGuardObject(value, at);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseArtifactRecordPatch(row["patch"], `${at}.patch`),
  };
}

export type AssumptionPatch = Readonly<Record<string, unknown>>;

export function parseAssumptionPatch(value: unknown, at = "$"): AssumptionPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramAssumptionsPatchEntry(value: unknown, at = "$"): ProgramAssumptionsPatchEntry {
  const row = architectProgramDiffGuardObject(value, at);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseAssumptionPatch(row["patch"], `${at}.patch`),
  };
}

export type ConstraintRecordPatch = Readonly<Record<string, unknown>>;

export function parseConstraintRecordPatch(value: unknown, at = "$"): ConstraintRecordPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramConstraintsPatchEntry(value: unknown, at = "$"): ProgramConstraintsPatchEntry {
  const row = architectProgramDiffGuardObject(value, at);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseConstraintRecordPatch(row["patch"], `${at}.patch`),
  };
}

export type ComplianceRecordPatch = Readonly<Record<string, unknown>>;

export function parseComplianceRecordPatch(value: unknown, at = "$"): ComplianceRecordPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramComplianceRecordsPatchEntry(value: unknown, at = "$"): ProgramComplianceRecordsPatchEntry {
  const row = architectProgramDiffGuardObject(value, at);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseComplianceRecordPatch(row["patch"], `${at}.patch`),
  };
}

export type ApprovalRecordPatch = Readonly<Record<string, unknown>>;

export function parseApprovalRecordPatch(value: unknown, at = "$"): ApprovalRecordPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramApprovalsPatchEntry(value: unknown, at = "$"): ProgramApprovalsPatchEntry {
  const row = architectProgramDiffGuardObject(value, at);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseApprovalRecordPatch(row["patch"], `${at}.patch`),
  };
}

export type MeetingRecordPatch = Readonly<Record<string, unknown>>;

export function parseMeetingRecordPatch(value: unknown, at = "$"): MeetingRecordPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramMeetingsPatchEntry(value: unknown, at = "$"): ProgramMeetingsPatchEntry {
  const row = architectProgramDiffGuardObject(value, at);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseMeetingRecordPatch(row["patch"], `${at}.patch`),
  };
}

export type ChangeRecordPatch = Readonly<Record<string, unknown>>;

export function parseChangeRecordPatch(value: unknown, at = "$"): ChangeRecordPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramChangesPatchEntry(value: unknown, at = "$"): ProgramChangesPatchEntry {
  const row = architectProgramDiffGuardObject(value, at);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseChangeRecordPatch(row["patch"], `${at}.patch`),
  };
}

export type CollaborationRecordPatch = Readonly<Record<string, unknown>>;

export function parseCollaborationRecordPatch(value: unknown, at = "$"): CollaborationRecordPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramCollaborationPatchEntry(value: unknown, at = "$"): ProgramCollaborationPatchEntry {
  const row = architectProgramDiffGuardObject(value, at);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseCollaborationRecordPatch(row["patch"], `${at}.patch`),
  };
}

export type AnalysisRecordPatch = Readonly<Record<string, unknown>>;

export function parseAnalysisRecordPatch(value: unknown, at = "$"): AnalysisRecordPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramAnalysesPatchEntry(value: unknown, at = "$"): ProgramAnalysesPatchEntry {
  const row = architectProgramDiffGuardObject(value, at);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseAnalysisRecordPatch(row["patch"], `${at}.patch`),
  };
}

export type ReportRecordPatch = Readonly<Record<string, unknown>>;

export function parseReportRecordPatch(value: unknown, at = "$"): ReportRecordPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramReportsPatchEntry(value: unknown, at = "$"): ProgramReportsPatchEntry {
  const row = architectProgramDiffGuardObject(value, at);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseReportRecordPatch(row["patch"], `${at}.patch`),
  };
}

export type SearchFilterPatch = Readonly<Record<string, unknown>>;

export function parseSearchFilterPatch(value: unknown, at = "$"): SearchFilterPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramSearchFiltersPatchEntry(value: unknown, at = "$"): ProgramSearchFiltersPatchEntry {
  const row = architectProgramDiffGuardObject(value, at);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseSearchFilterPatch(row["patch"], `${at}.patch`),
  };
}

export type StatusRecordPatch = Readonly<Record<string, unknown>>;

export function parseStatusRecordPatch(value: unknown, at = "$"): StatusRecordPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramStatusRecordsPatchEntry(value: unknown, at = "$"): ProgramStatusRecordsPatchEntry {
  const row = architectProgramDiffGuardObject(value, at);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseStatusRecordPatch(row["patch"], `${at}.patch`),
  };
}

export type WorkshopPatch = Readonly<Record<string, unknown>>;

export function parseWorkshopPatch(value: unknown, at = "$"): WorkshopPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramWorkshopsPatchEntry(value: unknown, at = "$"): ProgramWorkshopsPatchEntry {
  const row = architectProgramDiffGuardObject(value, at);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseWorkshopPatch(row["patch"], `${at}.patch`),
  };
}

export type SurveyPatch = Readonly<Record<string, unknown>>;

export function parseSurveyPatch(value: unknown, at = "$"): SurveyPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramSurveysPatchEntry(value: unknown, at = "$"): ProgramSurveysPatchEntry {
  const row = architectProgramDiffGuardObject(value, at);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseSurveyPatch(row["patch"], `${at}.patch`),
  };
}

export type IssuePatch = Readonly<Record<string, unknown>>;

export function parseIssuePatch(value: unknown, at = "$"): IssuePatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramIssuesPatchEntry(value: unknown, at = "$"): ProgramIssuesPatchEntry {
  const row = architectProgramDiffGuardObject(value, at);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseIssuePatch(row["patch"], `${at}.patch`),
  };
}

export type AuditEventPatch = Readonly<Record<string, unknown>>;

export function parseAuditEventPatch(value: unknown, at = "$"): AuditEventPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramAuditEventsPatchEntry(value: unknown, at = "$"): ProgramAuditEventsPatchEntry {
  const row = architectProgramDiffGuardObject(value, at);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseAuditEventPatch(row["patch"], `${at}.patch`),
  };
}

export type TemplateRecordPatch = Readonly<Record<string, unknown>>;

export function parseTemplateRecordPatch(value: unknown, at = "$"): TemplateRecordPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramTemplatesPatchEntry(value: unknown, at = "$"): ProgramTemplatesPatchEntry {
  const row = architectProgramDiffGuardObject(value, at);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseTemplateRecordPatch(row["patch"], `${at}.patch`),
  };
}

export type TraceLinkPatch = Readonly<Record<string, unknown>>;

export function parseTraceLinkPatch(value: unknown, at = "$"): TraceLinkPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramTracesPatchEntry(value: unknown, at = "$"): ProgramTracesPatchEntry {
  const row = architectProgramDiffGuardObject(value, at);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseTraceLinkPatch(row["patch"], `${at}.patch`),
  };
}

export interface ProgramKnowledgeDelta {
  removed: ProgramKnowledgeRemoval[];
  inserted: ProgramKnowledgeInsertion[];
  moved: ProgramKnowledgeRelocation[];
  modified: ProgramKnowledgePatchEntry[];
}

export interface ProgramKnowledgeInsertion {
  index: number;
  row: KnowledgeRecord;
}

export interface ProgramKnowledgeRemoval {
  id: string;
  index: number;
}

export interface ProgramKnowledgeRelocation {
  id: string;
  from: number;
  to: number;
}

export interface ProgramKnowledgePatchEntry {
  id: string;
  patch: KnowledgeRecordPatch;
}

export interface ProgramBenchmarksDelta {
  removed: ProgramBenchmarksRemoval[];
  inserted: ProgramBenchmarksInsertion[];
  moved: ProgramBenchmarksRelocation[];
  modified: ProgramBenchmarksPatchEntry[];
}

export interface ProgramBenchmarksInsertion {
  index: number;
  row: BenchmarkRecord;
}

export interface ProgramBenchmarksRemoval {
  id: string;
  index: number;
}

export interface ProgramBenchmarksRelocation {
  id: string;
  from: number;
  to: number;
}

export interface ProgramBenchmarksPatchEntry {
  id: string;
  patch: BenchmarkRecordPatch;
}

export interface ProgramMetaEdit {
  set: ProgramMeta | null;
  patch: ProgramMetaPatch | null;
}

export interface ProjectDefinitionEdit {
  set: ProjectDefinition | null;
  patch: ProjectDefinitionPatch | null;
}

export interface GovernanceEdit {
  set: Governance | null;
  patch: GovernancePatch | null;
}

export type KnowledgeRecordPatch = Readonly<Record<string, unknown>>;

export function parseKnowledgeRecordPatch(value: unknown, at = "$"): KnowledgeRecordPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramKnowledgePatchEntry(value: unknown, at = "$"): ProgramKnowledgePatchEntry {
  const row = architectProgramDiffGuardObject(value, at);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseKnowledgeRecordPatch(row["patch"], `${at}.patch`),
  };
}

export type BenchmarkRecordPatch = Readonly<Record<string, unknown>>;

export function parseBenchmarkRecordPatch(value: unknown, at = "$"): BenchmarkRecordPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramBenchmarksPatchEntry(value: unknown, at = "$"): ProgramBenchmarksPatchEntry {
  const row = architectProgramDiffGuardObject(value, at);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseBenchmarkRecordPatch(row["patch"], `${at}.patch`),
  };
}

export type ProgramMetaPatch = Readonly<Record<string, unknown>>;

export function parseProgramMetaPatch(value: unknown, at = "$"): ProgramMetaPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export type ProjectDefinitionPatch = Readonly<Record<string, unknown>>;

export function parseProjectDefinitionPatch(value: unknown, at = "$"): ProjectDefinitionPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export type GovernancePatch = Readonly<Record<string, unknown>>;

export function parseGovernancePatch(value: unknown, at = "$"): GovernancePatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}
