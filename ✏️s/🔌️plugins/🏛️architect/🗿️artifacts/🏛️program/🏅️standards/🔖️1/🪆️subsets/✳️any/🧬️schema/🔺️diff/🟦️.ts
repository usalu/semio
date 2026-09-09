/** 🧬️ ProgramSnapshot diff schema — sparse field delta. */

import { architectProgramArtifactGuardExactObject, parseArtifactChild, parseProgramArtifact, PROGRAM_ARTIFACT_FIELDS, type ArtifactChild } from "../🟦️.ts";
import type { AccessRule, AccessibilityRequirement, Activity, Adjacency, AnalysisRecord, ApprovalRecord, ArtifactRecord, Assumption, AuditEvent, ChangeRecord, CollaborationRecord, CommunicationRequirement, ComplianceRecord, Conflict, ConstraintRecord, CostRequirement, Decision, DeliveryConstraint, EnvironmentalRequirement, Equipment, FlexibilityRequirement, FlowRequirement, Function, Governance, GrowthPlan, HumanFactorRequirement, InformationRequirement, InfrastructureRequirement, Issue, MeetingRecord, OperationalRequirement, OptionEvaluation, OrganizationalRequirement, PerformanceCriterion, PriorityRecord, PrivacyRequirement, Process, ProgramArtifact, ProgramElement, ProgramMeta, ProjectDefinition, QualityRecord, QuantityRequirement, RegulatoryRequirement, Relationship, ReportRecord, Requirement, ResilienceRequirement, Resource, Risk, SafetyRequirement, Scenario, ScheduleRequirement, SearchFilter, SecurityRequirement, ServiceRequirement, SiteContext, Stakeholder, StatusRecord, StorageRequirement, Survey, SustainabilityRequirement, TemplateRecord, TraceLink, UserProfile, ValidationRecord, WayfindingRequirement, Workshop } from "../🟦️.ts";

export interface ProgramDiff {
  /** @state artifact */
  artifact: ProgramArtifact | null;
  /** @state artifact */
  schema: string | null;
  /** @state artifact */
  meta: ProgramMeta | null;
  /** @state artifact */
  project: ProjectDefinition | null;
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
  knowledge: ArtifactChild | null;
  /** @state artifact */
  benchmarks: ArtifactChild | null;
  /** @state artifact */
  traces: ProgramTracesDelta | null;
  /** @state artifact */
  governance: Governance | null;
}

export const PROGRAM_DIFF_FIELDS = ["artifact", ...PROGRAM_ARTIFACT_FIELDS] as const;

export function parseProgramDiff(value: unknown, at = "$"): ProgramDiff {
  const row = architectProgramArtifactGuardExactObject(value, at, PROGRAM_DIFF_FIELDS);
  if (row.artifact !== null) parseProgramArtifact(row.artifact, `${at}.artifact`);
  if (row.knowledge !== null) parseArtifactChild(row.knowledge);
  if (row.benchmarks !== null) parseArtifactChild(row.benchmarks);
  return row as unknown as ProgramDiff;
}

export interface ProgramStakeholdersDelta {
  added: Stakeholder[];
  removed: string[];
  patched: ProgramStakeholdersPatchEntry[];
  reordered: string[] | null;
}

export interface ProgramStakeholdersPatchEntry {
  id: string;
  patch: StakeholderPatch;
}

export interface ProgramUsersDelta {
  added: UserProfile[];
  removed: string[];
  patched: ProgramUsersPatchEntry[];
  reordered: string[] | null;
}

export interface ProgramUsersPatchEntry {
  id: string;
  patch: UserProfilePatch;
}

export interface ProgramActivitiesDelta {
  added: Activity[];
  removed: string[];
  patched: ProgramActivitiesPatchEntry[];
  reordered: string[] | null;
}

export interface ProgramActivitiesPatchEntry {
  id: string;
  patch: ActivityPatch;
}

export interface ProgramFunctionsDelta {
  added: Function[];
  removed: string[];
  patched: ProgramFunctionsPatchEntry[];
  reordered: string[] | null;
}

export interface ProgramFunctionsPatchEntry {
  id: string;
  patch: FunctionPatch;
}

export interface ProgramElementsDelta {
  added: ProgramElement[];
  removed: string[];
  patched: ProgramElementsPatchEntry[];
  reordered: string[] | null;
}

export interface ProgramElementsPatchEntry {
  id: string;
  patch: ProgramElementPatch;
}

export interface ProgramQuantitiesDelta {
  added: QuantityRequirement[];
  removed: string[];
  patched: ProgramQuantitiesPatchEntry[];
  reordered: string[] | null;
}

export interface ProgramQuantitiesPatchEntry {
  id: string;
  patch: QuantityRequirementPatch;
}

export interface ProgramRelationshipsDelta {
  added: Relationship[];
  removed: string[];
  patched: ProgramRelationshipsPatchEntry[];
  reordered: string[] | null;
}

export interface ProgramRelationshipsPatchEntry {
  id: string;
  patch: RelationshipPatch;
}

export interface ProgramAdjacenciesDelta {
  added: Adjacency[];
  removed: string[];
  patched: ProgramAdjacenciesPatchEntry[];
  reordered: string[] | null;
}

export interface ProgramAdjacenciesPatchEntry {
  id: string;
  patch: AdjacencyPatch;
}

export interface ProgramProcessesDelta {
  added: Process[];
  removed: string[];
  patched: ProgramProcessesPatchEntry[];
  reordered: string[] | null;
}

export interface ProgramProcessesPatchEntry {
  id: string;
  patch: ProcessPatch;
}

export interface ProgramFlowsDelta {
  added: FlowRequirement[];
  removed: string[];
  patched: ProgramFlowsPatchEntry[];
  reordered: string[] | null;
}

export interface ProgramFlowsPatchEntry {
  id: string;
  patch: FlowRequirementPatch;
}

export interface ProgramAccessRulesDelta {
  added: AccessRule[];
  removed: string[];
  patched: ProgramAccessRulesPatchEntry[];
  reordered: string[] | null;
}

export interface ProgramAccessRulesPatchEntry {
  id: string;
  patch: AccessRulePatch;
}

export interface ProgramOperationsDelta {
  added: OperationalRequirement[];
  removed: string[];
  patched: ProgramOperationsPatchEntry[];
  reordered: string[] | null;
}

export interface ProgramOperationsPatchEntry {
  id: string;
  patch: OperationalRequirementPatch;
}

export interface ProgramEquipmentDelta {
  added: Equipment[];
  removed: string[];
  patched: ProgramEquipmentPatchEntry[];
  reordered: string[] | null;
}

export interface ProgramEquipmentPatchEntry {
  id: string;
  patch: EquipmentPatch;
}

export interface ProgramResourcesDelta {
  added: Resource[];
  removed: string[];
  patched: ProgramResourcesPatchEntry[];
  reordered: string[] | null;
}

export interface ProgramResourcesPatchEntry {
  id: string;
  patch: ResourcePatch;
}

export interface ProgramStorageDelta {
  added: StorageRequirement[];
  removed: string[];
  patched: ProgramStoragePatchEntry[];
  reordered: string[] | null;
}

export interface ProgramStoragePatchEntry {
  id: string;
  patch: StorageRequirementPatch;
}

export interface ProgramEnvironmentalDelta {
  added: EnvironmentalRequirement[];
  removed: string[];
  patched: ProgramEnvironmentalPatchEntry[];
  reordered: string[] | null;
}

export interface ProgramEnvironmentalPatchEntry {
  id: string;
  patch: EnvironmentalRequirementPatch;
}

export interface ProgramHumanFactorsDelta {
  added: HumanFactorRequirement[];
  removed: string[];
  patched: ProgramHumanFactorsPatchEntry[];
  reordered: string[] | null;
}

export interface ProgramHumanFactorsPatchEntry {
  id: string;
  patch: HumanFactorRequirementPatch;
}

export interface ProgramAccessibilityDelta {
  added: AccessibilityRequirement[];
  removed: string[];
  patched: ProgramAccessibilityPatchEntry[];
  reordered: string[] | null;
}

export interface ProgramAccessibilityPatchEntry {
  id: string;
  patch: AccessibilityRequirementPatch;
}

export interface ProgramPrivacyDelta {
  added: PrivacyRequirement[];
  removed: string[];
  patched: ProgramPrivacyPatchEntry[];
  reordered: string[] | null;
}

export interface ProgramPrivacyPatchEntry {
  id: string;
  patch: PrivacyRequirementPatch;
}

export interface ProgramSafetyDelta {
  added: SafetyRequirement[];
  removed: string[];
  patched: ProgramSafetyPatchEntry[];
  reordered: string[] | null;
}

export interface ProgramSafetyPatchEntry {
  id: string;
  patch: SafetyRequirementPatch;
}

export interface ProgramSecurityDelta {
  added: SecurityRequirement[];
  removed: string[];
  patched: ProgramSecurityPatchEntry[];
  reordered: string[] | null;
}

export interface ProgramSecurityPatchEntry {
  id: string;
  patch: SecurityRequirementPatch;
}

export interface ProgramRegulatoryDelta {
  added: RegulatoryRequirement[];
  removed: string[];
  patched: ProgramRegulatoryPatchEntry[];
  reordered: string[] | null;
}

export interface ProgramRegulatoryPatchEntry {
  id: string;
  patch: RegulatoryRequirementPatch;
}

export interface ProgramSiteContextDelta {
  added: SiteContext[];
  removed: string[];
  patched: ProgramSiteContextPatchEntry[];
  reordered: string[] | null;
}

export interface ProgramSiteContextPatchEntry {
  id: string;
  patch: SiteContextPatch;
}

export interface ProgramOrganizationalDelta {
  added: OrganizationalRequirement[];
  removed: string[];
  patched: ProgramOrganizationalPatchEntry[];
  reordered: string[] | null;
}

export interface ProgramOrganizationalPatchEntry {
  id: string;
  patch: OrganizationalRequirementPatch;
}

export interface ProgramServicesDelta {
  added: ServiceRequirement[];
  removed: string[];
  patched: ProgramServicesPatchEntry[];
  reordered: string[] | null;
}

export interface ProgramServicesPatchEntry {
  id: string;
  patch: ServiceRequirementPatch;
}

export interface ProgramInfrastructureDelta {
  added: InfrastructureRequirement[];
  removed: string[];
  patched: ProgramInfrastructurePatchEntry[];
  reordered: string[] | null;
}

export interface ProgramInfrastructurePatchEntry {
  id: string;
  patch: InfrastructureRequirementPatch;
}

export interface ProgramInformationDelta {
  added: InformationRequirement[];
  removed: string[];
  patched: ProgramInformationPatchEntry[];
  reordered: string[] | null;
}

export interface ProgramInformationPatchEntry {
  id: string;
  patch: InformationRequirementPatch;
}

export interface ProgramCommunicationDelta {
  added: CommunicationRequirement[];
  removed: string[];
  patched: ProgramCommunicationPatchEntry[];
  reordered: string[] | null;
}

export interface ProgramCommunicationPatchEntry {
  id: string;
  patch: CommunicationRequirementPatch;
}

export interface ProgramWayfindingDelta {
  added: WayfindingRequirement[];
  removed: string[];
  patched: ProgramWayfindingPatchEntry[];
  reordered: string[] | null;
}

export interface ProgramWayfindingPatchEntry {
  id: string;
  patch: WayfindingRequirementPatch;
}

export interface ProgramSchedulesDelta {
  added: ScheduleRequirement[];
  removed: string[];
  patched: ProgramSchedulesPatchEntry[];
  reordered: string[] | null;
}

export interface ProgramSchedulesPatchEntry {
  id: string;
  patch: ScheduleRequirementPatch;
}

export interface ProgramFlexibilityDelta {
  added: FlexibilityRequirement[];
  removed: string[];
  patched: ProgramFlexibilityPatchEntry[];
  reordered: string[] | null;
}

export interface ProgramFlexibilityPatchEntry {
  id: string;
  patch: FlexibilityRequirementPatch;
}

export interface ProgramGrowthDelta {
  added: GrowthPlan[];
  removed: string[];
  patched: ProgramGrowthPatchEntry[];
  reordered: string[] | null;
}

export interface ProgramGrowthPatchEntry {
  id: string;
  patch: GrowthPlanPatch;
}

export interface ProgramSustainabilityDelta {
  added: SustainabilityRequirement[];
  removed: string[];
  patched: ProgramSustainabilityPatchEntry[];
  reordered: string[] | null;
}

export interface ProgramSustainabilityPatchEntry {
  id: string;
  patch: SustainabilityRequirementPatch;
}

export interface ProgramResilienceDelta {
  added: ResilienceRequirement[];
  removed: string[];
  patched: ProgramResiliencePatchEntry[];
  reordered: string[] | null;
}

export interface ProgramResiliencePatchEntry {
  id: string;
  patch: ResilienceRequirementPatch;
}

export interface ProgramCostsDelta {
  added: CostRequirement[];
  removed: string[];
  patched: ProgramCostsPatchEntry[];
  reordered: string[] | null;
}

export interface ProgramCostsPatchEntry {
  id: string;
  patch: CostRequirementPatch;
}

export interface ProgramDeliveryDelta {
  added: DeliveryConstraint[];
  removed: string[];
  patched: ProgramDeliveryPatchEntry[];
  reordered: string[] | null;
}

export interface ProgramDeliveryPatchEntry {
  id: string;
  patch: DeliveryConstraintPatch;
}

export interface ProgramRisksDelta {
  added: Risk[];
  removed: string[];
  patched: ProgramRisksPatchEntry[];
  reordered: string[] | null;
}

export interface ProgramRisksPatchEntry {
  id: string;
  patch: RiskPatch;
}

export interface ProgramConflictsDelta {
  added: Conflict[];
  removed: string[];
  patched: ProgramConflictsPatchEntry[];
  reordered: string[] | null;
}

export interface ProgramConflictsPatchEntry {
  id: string;
  patch: ConflictPatch;
}

export interface ProgramRequirementsDelta {
  added: Requirement[];
  removed: string[];
  patched: ProgramRequirementsPatchEntry[];
  reordered: string[] | null;
}

export interface ProgramRequirementsPatchEntry {
  id: string;
  patch: RequirementPatch;
}

export interface ProgramPrioritiesDelta {
  added: PriorityRecord[];
  removed: string[];
  patched: ProgramPrioritiesPatchEntry[];
  reordered: string[] | null;
}

export interface ProgramPrioritiesPatchEntry {
  id: string;
  patch: PriorityRecordPatch;
}

export interface ProgramScenariosDelta {
  added: Scenario[];
  removed: string[];
  patched: ProgramScenariosPatchEntry[];
  reordered: string[] | null;
}

export interface ProgramScenariosPatchEntry {
  id: string;
  patch: ScenarioPatch;
}

export interface ProgramOptionsDelta {
  added: OptionEvaluation[];
  removed: string[];
  patched: ProgramOptionsPatchEntry[];
  reordered: string[] | null;
}

export interface ProgramOptionsPatchEntry {
  id: string;
  patch: OptionEvaluationPatch;
}

export interface ProgramDecisionsDelta {
  added: Decision[];
  removed: string[];
  patched: ProgramDecisionsPatchEntry[];
  reordered: string[] | null;
}

export interface ProgramDecisionsPatchEntry {
  id: string;
  patch: DecisionPatch;
}

export interface ProgramValidationsDelta {
  added: ValidationRecord[];
  removed: string[];
  patched: ProgramValidationsPatchEntry[];
  reordered: string[] | null;
}

export interface ProgramValidationsPatchEntry {
  id: string;
  patch: ValidationRecordPatch;
}

export interface ProgramPerformanceDelta {
  added: PerformanceCriterion[];
  removed: string[];
  patched: ProgramPerformancePatchEntry[];
  reordered: string[] | null;
}

export interface ProgramPerformancePatchEntry {
  id: string;
  patch: PerformanceCriterionPatch;
}

export interface ProgramQualityDelta {
  added: QualityRecord[];
  removed: string[];
  patched: ProgramQualityPatchEntry[];
  reordered: string[] | null;
}

export interface ProgramQualityPatchEntry {
  id: string;
  patch: QualityRecordPatch;
}

export interface ProgramArtifactsDelta {
  added: ArtifactRecord[];
  removed: string[];
  patched: ProgramArtifactsPatchEntry[];
  reordered: string[] | null;
}

export interface ProgramArtifactsPatchEntry {
  id: string;
  patch: ArtifactRecordPatch;
}

export interface ProgramAssumptionsDelta {
  added: Assumption[];
  removed: string[];
  patched: ProgramAssumptionsPatchEntry[];
  reordered: string[] | null;
}

export interface ProgramAssumptionsPatchEntry {
  id: string;
  patch: AssumptionPatch;
}

export interface ProgramConstraintsDelta {
  added: ConstraintRecord[];
  removed: string[];
  patched: ProgramConstraintsPatchEntry[];
  reordered: string[] | null;
}

export interface ProgramConstraintsPatchEntry {
  id: string;
  patch: ConstraintRecordPatch;
}

export interface ProgramComplianceRecordsDelta {
  added: ComplianceRecord[];
  removed: string[];
  patched: ProgramComplianceRecordsPatchEntry[];
  reordered: string[] | null;
}

export interface ProgramComplianceRecordsPatchEntry {
  id: string;
  patch: ComplianceRecordPatch;
}

export interface ProgramApprovalsDelta {
  added: ApprovalRecord[];
  removed: string[];
  patched: ProgramApprovalsPatchEntry[];
  reordered: string[] | null;
}

export interface ProgramApprovalsPatchEntry {
  id: string;
  patch: ApprovalRecordPatch;
}

export interface ProgramMeetingsDelta {
  added: MeetingRecord[];
  removed: string[];
  patched: ProgramMeetingsPatchEntry[];
  reordered: string[] | null;
}

export interface ProgramMeetingsPatchEntry {
  id: string;
  patch: MeetingRecordPatch;
}

export interface ProgramChangesDelta {
  added: ChangeRecord[];
  removed: string[];
  patched: ProgramChangesPatchEntry[];
  reordered: string[] | null;
}

export interface ProgramChangesPatchEntry {
  id: string;
  patch: ChangeRecordPatch;
}

export interface ProgramCollaborationDelta {
  added: CollaborationRecord[];
  removed: string[];
  patched: ProgramCollaborationPatchEntry[];
  reordered: string[] | null;
}

export interface ProgramCollaborationPatchEntry {
  id: string;
  patch: CollaborationRecordPatch;
}

export interface ProgramAnalysesDelta {
  added: AnalysisRecord[];
  removed: string[];
  patched: ProgramAnalysesPatchEntry[];
  reordered: string[] | null;
}

export interface ProgramAnalysesPatchEntry {
  id: string;
  patch: AnalysisRecordPatch;
}

export interface ProgramReportsDelta {
  added: ReportRecord[];
  removed: string[];
  patched: ProgramReportsPatchEntry[];
  reordered: string[] | null;
}

export interface ProgramReportsPatchEntry {
  id: string;
  patch: ReportRecordPatch;
}

export interface ProgramSearchFiltersDelta {
  added: SearchFilter[];
  removed: string[];
  patched: ProgramSearchFiltersPatchEntry[];
  reordered: string[] | null;
}

export interface ProgramSearchFiltersPatchEntry {
  id: string;
  patch: SearchFilterPatch;
}

export interface ProgramStatusRecordsDelta {
  added: StatusRecord[];
  removed: string[];
  patched: ProgramStatusRecordsPatchEntry[];
  reordered: string[] | null;
}

export interface ProgramStatusRecordsPatchEntry {
  id: string;
  patch: StatusRecordPatch;
}

export interface ProgramWorkshopsDelta {
  added: Workshop[];
  removed: string[];
  patched: ProgramWorkshopsPatchEntry[];
  reordered: string[] | null;
}

export interface ProgramWorkshopsPatchEntry {
  id: string;
  patch: WorkshopPatch;
}

export interface ProgramSurveysDelta {
  added: Survey[];
  removed: string[];
  patched: ProgramSurveysPatchEntry[];
  reordered: string[] | null;
}

export interface ProgramSurveysPatchEntry {
  id: string;
  patch: SurveyPatch;
}

export interface ProgramIssuesDelta {
  added: Issue[];
  removed: string[];
  patched: ProgramIssuesPatchEntry[];
  reordered: string[] | null;
}

export interface ProgramIssuesPatchEntry {
  id: string;
  patch: IssuePatch;
}

export interface ProgramAuditEventsDelta {
  added: AuditEvent[];
  removed: string[];
  patched: ProgramAuditEventsPatchEntry[];
  reordered: string[] | null;
}

export interface ProgramAuditEventsPatchEntry {
  id: string;
  patch: AuditEventPatch;
}

export interface ProgramTemplatesDelta {
  added: TemplateRecord[];
  removed: string[];
  patched: ProgramTemplatesPatchEntry[];
  reordered: string[] | null;
}

export interface ProgramTemplatesPatchEntry {
  id: string;
  patch: TemplateRecordPatch;
}

export interface ProgramTracesDelta {
  added: TraceLink[];
  removed: string[];
  patched: ProgramTracesPatchEntry[];
  reordered: string[] | null;
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

export interface ProgramStringList {
  readonly values: readonly string[];
}

export function parseProgramStringList(value: unknown, at = "$"): ProgramStringList {
  const row = architectProgramDiffGuardObject(value, at);
  return {
    values: architectProgramDiffGuardArray(row["values"], `${at}.values`).map((item, index) => architectProgramDiffGuardString(item, `${at}.values[${index}]`)),
  };
}

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
