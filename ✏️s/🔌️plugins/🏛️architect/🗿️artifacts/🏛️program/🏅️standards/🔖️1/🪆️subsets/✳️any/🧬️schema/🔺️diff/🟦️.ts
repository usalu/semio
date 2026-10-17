/** 🧬️ ProgramSnapshot diff schema — sparse field delta. */

import { architectProgramArtifactGuardExactObject, PROGRAM_ARTIFACT_FIELDS } from "../🟦️.ts";
import type { AccessibilityRequirementPatch, AccessRulePatch, ActivityPatch, AdjacencyPatch, AnalysisRecordPatch, ApprovalRecordPatch, ArtifactRecordPatch, AssumptionPatch, AuditEventPatch, BenchmarkRecordPatch, ChangeRecordPatch, CollaborationRecordPatch, CommunicationRequirementPatch, ComplianceRecordPatch, ConflictPatch, ConstraintRecordPatch, CostRequirementPatch, DecisionPatch, DeliveryConstraintPatch, EnvironmentalRequirementPatch, EquipmentPatch, FlexibilityRequirementPatch, FlowRequirementPatch, FunctionPatch, GovernancePatch, GrowthPlanPatch, HumanFactorRequirementPatch, InformationRequirementPatch, InfrastructureRequirementPatch, IssuePatch, KnowledgeRecordPatch, MeetingRecordPatch, OperationalRequirementPatch, OptionEvaluationPatch, OrganizationalRequirementPatch, PerformanceCriterionPatch, PriorityRecordPatch, PrivacyRequirementPatch, ProcessPatch, ProgramElementPatch, ProgramMetaPatch, ProjectDefinitionPatch, QualityRecordPatch, QuantityRequirementPatch, RegulatoryRequirementPatch, RelationshipPatch, ReportRecordPatch, RequirementPatch, ResilienceRequirementPatch, ResourcePatch, RiskPatch, SafetyRequirementPatch, ScenarioPatch, ScheduleRequirementPatch, SearchFilterPatch, SecurityRequirementPatch, ServiceRequirementPatch, SiteContextPatch, StakeholderPatch, StatusRecordPatch, StorageRequirementPatch, SurveyPatch, SustainabilityRequirementPatch, TemplateRecordPatch, TraceLinkPatch, UserProfilePatch, ValidationRecordPatch, WayfindingRequirementPatch, WorkshopPatch } from "./🩹️patch-types/🟦️.ts";
import type { ProgramAccessRulesDelta, ProgramAccessRulesPatchEntry, ProgramActivitiesDelta, ProgramActivitiesPatchEntry, ProgramAdjacenciesDelta, ProgramAdjacenciesPatchEntry, ProgramElementsDelta, ProgramElementsPatchEntry, ProgramFlowsDelta, ProgramFlowsPatchEntry, ProgramFunctionsDelta, ProgramFunctionsPatchEntry, ProgramProcessesDelta, ProgramProcessesPatchEntry, ProgramQuantitiesDelta, ProgramQuantitiesPatchEntry, ProgramRelationshipsDelta, ProgramRelationshipsPatchEntry, ProgramStakeholdersDelta, ProgramStakeholdersPatchEntry, ProgramUsersDelta, ProgramUsersPatchEntry } from "./🧱️deltas-program-structure/🟦️.ts";
import type { ProgramAccessibilityDelta, ProgramAccessibilityPatchEntry, ProgramCommunicationDelta, ProgramCommunicationPatchEntry, ProgramCostsDelta, ProgramCostsPatchEntry, ProgramDeliveryDelta, ProgramDeliveryPatchEntry, ProgramEnvironmentalDelta, ProgramEnvironmentalPatchEntry, ProgramEquipmentDelta, ProgramEquipmentPatchEntry, ProgramFlexibilityDelta, ProgramFlexibilityPatchEntry, ProgramGrowthDelta, ProgramGrowthPatchEntry, ProgramHumanFactorsDelta, ProgramHumanFactorsPatchEntry, ProgramInformationDelta, ProgramInformationPatchEntry, ProgramInfrastructureDelta, ProgramInfrastructurePatchEntry, ProgramOperationsDelta, ProgramOperationsPatchEntry, ProgramOrganizationalDelta, ProgramOrganizationalPatchEntry, ProgramPrivacyDelta, ProgramPrivacyPatchEntry, ProgramRegulatoryDelta, ProgramRegulatoryPatchEntry, ProgramResilienceDelta, ProgramResiliencePatchEntry, ProgramResourcesDelta, ProgramResourcesPatchEntry, ProgramSafetyDelta, ProgramSafetyPatchEntry, ProgramSchedulesDelta, ProgramSchedulesPatchEntry, ProgramSecurityDelta, ProgramSecurityPatchEntry, ProgramServicesDelta, ProgramServicesPatchEntry, ProgramSiteContextDelta, ProgramSiteContextPatchEntry, ProgramStorageDelta, ProgramStoragePatchEntry, ProgramSustainabilityDelta, ProgramSustainabilityPatchEntry, ProgramWayfindingDelta, ProgramWayfindingPatchEntry } from "./📋️deltas-requirements/🟦️.ts";
import type { GovernanceEdit, ProgramAnalysesDelta, ProgramAnalysesPatchEntry, ProgramApprovalsDelta, ProgramApprovalsPatchEntry, ProgramArtifactsDelta, ProgramArtifactsPatchEntry, ProgramAssumptionsDelta, ProgramAssumptionsPatchEntry, ProgramAuditEventsDelta, ProgramAuditEventsPatchEntry, ProgramBenchmarksDelta, ProgramBenchmarksPatchEntry, ProgramChangesDelta, ProgramChangesPatchEntry, ProgramCollaborationDelta, ProgramCollaborationPatchEntry, ProgramComplianceRecordsDelta, ProgramComplianceRecordsPatchEntry, ProgramConflictsDelta, ProgramConflictsPatchEntry, ProgramConstraintsDelta, ProgramConstraintsPatchEntry, ProgramDecisionsDelta, ProgramDecisionsPatchEntry, ProgramIssuesDelta, ProgramIssuesPatchEntry, ProgramKnowledgeDelta, ProgramKnowledgePatchEntry, ProgramMeetingsDelta, ProgramMeetingsPatchEntry, ProgramMetaEdit, ProgramOptionsDelta, ProgramOptionsPatchEntry, ProgramPerformanceDelta, ProgramPerformancePatchEntry, ProgramPrioritiesDelta, ProgramPrioritiesPatchEntry, ProgramQualityDelta, ProgramQualityPatchEntry, ProgramReportsDelta, ProgramReportsPatchEntry, ProgramRequirementsDelta, ProgramRequirementsPatchEntry, ProgramRisksDelta, ProgramRisksPatchEntry, ProgramScenariosDelta, ProgramScenariosPatchEntry, ProgramSearchFiltersDelta, ProgramSearchFiltersPatchEntry, ProgramStatusRecordsDelta, ProgramStatusRecordsPatchEntry, ProgramSurveysDelta, ProgramSurveysPatchEntry, ProgramTemplatesDelta, ProgramTemplatesPatchEntry, ProgramTracesDelta, ProgramTracesPatchEntry, ProgramValidationsDelta, ProgramValidationsPatchEntry, ProgramWorkshopsDelta, ProgramWorkshopsPatchEntry, ProjectDefinitionEdit } from "./🗃️deltas-records/🟦️.ts";

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
export const architectProgramDiffGuardClosed = (row: Readonly<Record<string, unknown>>, at: string, keys: readonly string[]): Readonly<Record<string, unknown>> => {
  for (const key of Object.keys(row)) if (!keys.includes(key)) architectProgramDiffGuardReject(`${at}.${key}`, "unknown property");
  return row;
};
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
export function parseStakeholderPatch(value: unknown, at = "$"): StakeholderPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramStakeholdersPatchEntry(value: unknown, at = "$"): ProgramStakeholdersPatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseStakeholderPatch(row["patch"], `${at}.patch`),
  };
}

export function parseUserProfilePatch(value: unknown, at = "$"): UserProfilePatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramUsersPatchEntry(value: unknown, at = "$"): ProgramUsersPatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseUserProfilePatch(row["patch"], `${at}.patch`),
  };
}

export function parseActivityPatch(value: unknown, at = "$"): ActivityPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramActivitiesPatchEntry(value: unknown, at = "$"): ProgramActivitiesPatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseActivityPatch(row["patch"], `${at}.patch`),
  };
}

export function parseFunctionPatch(value: unknown, at = "$"): FunctionPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramFunctionsPatchEntry(value: unknown, at = "$"): ProgramFunctionsPatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseFunctionPatch(row["patch"], `${at}.patch`),
  };
}

export function parseProgramElementPatch(value: unknown, at = "$"): ProgramElementPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramElementsPatchEntry(value: unknown, at = "$"): ProgramElementsPatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseProgramElementPatch(row["patch"], `${at}.patch`),
  };
}

export function parseQuantityRequirementPatch(value: unknown, at = "$"): QuantityRequirementPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramQuantitiesPatchEntry(value: unknown, at = "$"): ProgramQuantitiesPatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseQuantityRequirementPatch(row["patch"], `${at}.patch`),
  };
}

export function parseRelationshipPatch(value: unknown, at = "$"): RelationshipPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramRelationshipsPatchEntry(value: unknown, at = "$"): ProgramRelationshipsPatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseRelationshipPatch(row["patch"], `${at}.patch`),
  };
}

export function parseAdjacencyPatch(value: unknown, at = "$"): AdjacencyPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramAdjacenciesPatchEntry(value: unknown, at = "$"): ProgramAdjacenciesPatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseAdjacencyPatch(row["patch"], `${at}.patch`),
  };
}

export function parseProcessPatch(value: unknown, at = "$"): ProcessPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramProcessesPatchEntry(value: unknown, at = "$"): ProgramProcessesPatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseProcessPatch(row["patch"], `${at}.patch`),
  };
}

export function parseFlowRequirementPatch(value: unknown, at = "$"): FlowRequirementPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramFlowsPatchEntry(value: unknown, at = "$"): ProgramFlowsPatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseFlowRequirementPatch(row["patch"], `${at}.patch`),
  };
}

export function parseAccessRulePatch(value: unknown, at = "$"): AccessRulePatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramAccessRulesPatchEntry(value: unknown, at = "$"): ProgramAccessRulesPatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseAccessRulePatch(row["patch"], `${at}.patch`),
  };
}

export function parseOperationalRequirementPatch(value: unknown, at = "$"): OperationalRequirementPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramOperationsPatchEntry(value: unknown, at = "$"): ProgramOperationsPatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseOperationalRequirementPatch(row["patch"], `${at}.patch`),
  };
}

export function parseEquipmentPatch(value: unknown, at = "$"): EquipmentPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramEquipmentPatchEntry(value: unknown, at = "$"): ProgramEquipmentPatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseEquipmentPatch(row["patch"], `${at}.patch`),
  };
}

export function parseResourcePatch(value: unknown, at = "$"): ResourcePatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramResourcesPatchEntry(value: unknown, at = "$"): ProgramResourcesPatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseResourcePatch(row["patch"], `${at}.patch`),
  };
}

export function parseStorageRequirementPatch(value: unknown, at = "$"): StorageRequirementPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramStoragePatchEntry(value: unknown, at = "$"): ProgramStoragePatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseStorageRequirementPatch(row["patch"], `${at}.patch`),
  };
}

export function parseEnvironmentalRequirementPatch(value: unknown, at = "$"): EnvironmentalRequirementPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramEnvironmentalPatchEntry(value: unknown, at = "$"): ProgramEnvironmentalPatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseEnvironmentalRequirementPatch(row["patch"], `${at}.patch`),
  };
}

export function parseHumanFactorRequirementPatch(value: unknown, at = "$"): HumanFactorRequirementPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramHumanFactorsPatchEntry(value: unknown, at = "$"): ProgramHumanFactorsPatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseHumanFactorRequirementPatch(row["patch"], `${at}.patch`),
  };
}

export function parseAccessibilityRequirementPatch(value: unknown, at = "$"): AccessibilityRequirementPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramAccessibilityPatchEntry(value: unknown, at = "$"): ProgramAccessibilityPatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseAccessibilityRequirementPatch(row["patch"], `${at}.patch`),
  };
}

export function parsePrivacyRequirementPatch(value: unknown, at = "$"): PrivacyRequirementPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramPrivacyPatchEntry(value: unknown, at = "$"): ProgramPrivacyPatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parsePrivacyRequirementPatch(row["patch"], `${at}.patch`),
  };
}

export function parseSafetyRequirementPatch(value: unknown, at = "$"): SafetyRequirementPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramSafetyPatchEntry(value: unknown, at = "$"): ProgramSafetyPatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseSafetyRequirementPatch(row["patch"], `${at}.patch`),
  };
}

export function parseSecurityRequirementPatch(value: unknown, at = "$"): SecurityRequirementPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramSecurityPatchEntry(value: unknown, at = "$"): ProgramSecurityPatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseSecurityRequirementPatch(row["patch"], `${at}.patch`),
  };
}

export function parseRegulatoryRequirementPatch(value: unknown, at = "$"): RegulatoryRequirementPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramRegulatoryPatchEntry(value: unknown, at = "$"): ProgramRegulatoryPatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseRegulatoryRequirementPatch(row["patch"], `${at}.patch`),
  };
}

export function parseSiteContextPatch(value: unknown, at = "$"): SiteContextPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramSiteContextPatchEntry(value: unknown, at = "$"): ProgramSiteContextPatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseSiteContextPatch(row["patch"], `${at}.patch`),
  };
}

export function parseOrganizationalRequirementPatch(value: unknown, at = "$"): OrganizationalRequirementPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramOrganizationalPatchEntry(value: unknown, at = "$"): ProgramOrganizationalPatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseOrganizationalRequirementPatch(row["patch"], `${at}.patch`),
  };
}

export function parseServiceRequirementPatch(value: unknown, at = "$"): ServiceRequirementPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramServicesPatchEntry(value: unknown, at = "$"): ProgramServicesPatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseServiceRequirementPatch(row["patch"], `${at}.patch`),
  };
}

export function parseInfrastructureRequirementPatch(value: unknown, at = "$"): InfrastructureRequirementPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramInfrastructurePatchEntry(value: unknown, at = "$"): ProgramInfrastructurePatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseInfrastructureRequirementPatch(row["patch"], `${at}.patch`),
  };
}

export function parseInformationRequirementPatch(value: unknown, at = "$"): InformationRequirementPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramInformationPatchEntry(value: unknown, at = "$"): ProgramInformationPatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseInformationRequirementPatch(row["patch"], `${at}.patch`),
  };
}

export function parseCommunicationRequirementPatch(value: unknown, at = "$"): CommunicationRequirementPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramCommunicationPatchEntry(value: unknown, at = "$"): ProgramCommunicationPatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseCommunicationRequirementPatch(row["patch"], `${at}.patch`),
  };
}

export function parseWayfindingRequirementPatch(value: unknown, at = "$"): WayfindingRequirementPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramWayfindingPatchEntry(value: unknown, at = "$"): ProgramWayfindingPatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseWayfindingRequirementPatch(row["patch"], `${at}.patch`),
  };
}

export function parseScheduleRequirementPatch(value: unknown, at = "$"): ScheduleRequirementPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramSchedulesPatchEntry(value: unknown, at = "$"): ProgramSchedulesPatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseScheduleRequirementPatch(row["patch"], `${at}.patch`),
  };
}

export function parseFlexibilityRequirementPatch(value: unknown, at = "$"): FlexibilityRequirementPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramFlexibilityPatchEntry(value: unknown, at = "$"): ProgramFlexibilityPatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseFlexibilityRequirementPatch(row["patch"], `${at}.patch`),
  };
}

export function parseGrowthPlanPatch(value: unknown, at = "$"): GrowthPlanPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramGrowthPatchEntry(value: unknown, at = "$"): ProgramGrowthPatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseGrowthPlanPatch(row["patch"], `${at}.patch`),
  };
}

export function parseSustainabilityRequirementPatch(value: unknown, at = "$"): SustainabilityRequirementPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramSustainabilityPatchEntry(value: unknown, at = "$"): ProgramSustainabilityPatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseSustainabilityRequirementPatch(row["patch"], `${at}.patch`),
  };
}

export function parseResilienceRequirementPatch(value: unknown, at = "$"): ResilienceRequirementPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramResiliencePatchEntry(value: unknown, at = "$"): ProgramResiliencePatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseResilienceRequirementPatch(row["patch"], `${at}.patch`),
  };
}

export function parseCostRequirementPatch(value: unknown, at = "$"): CostRequirementPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramCostsPatchEntry(value: unknown, at = "$"): ProgramCostsPatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseCostRequirementPatch(row["patch"], `${at}.patch`),
  };
}

export function parseDeliveryConstraintPatch(value: unknown, at = "$"): DeliveryConstraintPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramDeliveryPatchEntry(value: unknown, at = "$"): ProgramDeliveryPatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseDeliveryConstraintPatch(row["patch"], `${at}.patch`),
  };
}

export function parseRiskPatch(value: unknown, at = "$"): RiskPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramRisksPatchEntry(value: unknown, at = "$"): ProgramRisksPatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseRiskPatch(row["patch"], `${at}.patch`),
  };
}

export function parseConflictPatch(value: unknown, at = "$"): ConflictPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramConflictsPatchEntry(value: unknown, at = "$"): ProgramConflictsPatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseConflictPatch(row["patch"], `${at}.patch`),
  };
}

export function parseRequirementPatch(value: unknown, at = "$"): RequirementPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramRequirementsPatchEntry(value: unknown, at = "$"): ProgramRequirementsPatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseRequirementPatch(row["patch"], `${at}.patch`),
  };
}

export function parsePriorityRecordPatch(value: unknown, at = "$"): PriorityRecordPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramPrioritiesPatchEntry(value: unknown, at = "$"): ProgramPrioritiesPatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parsePriorityRecordPatch(row["patch"], `${at}.patch`),
  };
}

export function parseScenarioPatch(value: unknown, at = "$"): ScenarioPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramScenariosPatchEntry(value: unknown, at = "$"): ProgramScenariosPatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseScenarioPatch(row["patch"], `${at}.patch`),
  };
}

export function parseOptionEvaluationPatch(value: unknown, at = "$"): OptionEvaluationPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramOptionsPatchEntry(value: unknown, at = "$"): ProgramOptionsPatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseOptionEvaluationPatch(row["patch"], `${at}.patch`),
  };
}

export function parseDecisionPatch(value: unknown, at = "$"): DecisionPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramDecisionsPatchEntry(value: unknown, at = "$"): ProgramDecisionsPatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseDecisionPatch(row["patch"], `${at}.patch`),
  };
}

export function parseValidationRecordPatch(value: unknown, at = "$"): ValidationRecordPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramValidationsPatchEntry(value: unknown, at = "$"): ProgramValidationsPatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseValidationRecordPatch(row["patch"], `${at}.patch`),
  };
}

export function parsePerformanceCriterionPatch(value: unknown, at = "$"): PerformanceCriterionPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramPerformancePatchEntry(value: unknown, at = "$"): ProgramPerformancePatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parsePerformanceCriterionPatch(row["patch"], `${at}.patch`),
  };
}

export function parseQualityRecordPatch(value: unknown, at = "$"): QualityRecordPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramQualityPatchEntry(value: unknown, at = "$"): ProgramQualityPatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseQualityRecordPatch(row["patch"], `${at}.patch`),
  };
}

export function parseArtifactRecordPatch(value: unknown, at = "$"): ArtifactRecordPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramArtifactsPatchEntry(value: unknown, at = "$"): ProgramArtifactsPatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseArtifactRecordPatch(row["patch"], `${at}.patch`),
  };
}

export function parseAssumptionPatch(value: unknown, at = "$"): AssumptionPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramAssumptionsPatchEntry(value: unknown, at = "$"): ProgramAssumptionsPatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseAssumptionPatch(row["patch"], `${at}.patch`),
  };
}

export function parseConstraintRecordPatch(value: unknown, at = "$"): ConstraintRecordPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramConstraintsPatchEntry(value: unknown, at = "$"): ProgramConstraintsPatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseConstraintRecordPatch(row["patch"], `${at}.patch`),
  };
}

export function parseComplianceRecordPatch(value: unknown, at = "$"): ComplianceRecordPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramComplianceRecordsPatchEntry(value: unknown, at = "$"): ProgramComplianceRecordsPatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseComplianceRecordPatch(row["patch"], `${at}.patch`),
  };
}

export function parseApprovalRecordPatch(value: unknown, at = "$"): ApprovalRecordPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramApprovalsPatchEntry(value: unknown, at = "$"): ProgramApprovalsPatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseApprovalRecordPatch(row["patch"], `${at}.patch`),
  };
}

export function parseMeetingRecordPatch(value: unknown, at = "$"): MeetingRecordPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramMeetingsPatchEntry(value: unknown, at = "$"): ProgramMeetingsPatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseMeetingRecordPatch(row["patch"], `${at}.patch`),
  };
}

export function parseChangeRecordPatch(value: unknown, at = "$"): ChangeRecordPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramChangesPatchEntry(value: unknown, at = "$"): ProgramChangesPatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseChangeRecordPatch(row["patch"], `${at}.patch`),
  };
}

export function parseCollaborationRecordPatch(value: unknown, at = "$"): CollaborationRecordPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramCollaborationPatchEntry(value: unknown, at = "$"): ProgramCollaborationPatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseCollaborationRecordPatch(row["patch"], `${at}.patch`),
  };
}

export function parseAnalysisRecordPatch(value: unknown, at = "$"): AnalysisRecordPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramAnalysesPatchEntry(value: unknown, at = "$"): ProgramAnalysesPatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseAnalysisRecordPatch(row["patch"], `${at}.patch`),
  };
}

export function parseReportRecordPatch(value: unknown, at = "$"): ReportRecordPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramReportsPatchEntry(value: unknown, at = "$"): ProgramReportsPatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseReportRecordPatch(row["patch"], `${at}.patch`),
  };
}

export function parseSearchFilterPatch(value: unknown, at = "$"): SearchFilterPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramSearchFiltersPatchEntry(value: unknown, at = "$"): ProgramSearchFiltersPatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseSearchFilterPatch(row["patch"], `${at}.patch`),
  };
}

export function parseStatusRecordPatch(value: unknown, at = "$"): StatusRecordPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramStatusRecordsPatchEntry(value: unknown, at = "$"): ProgramStatusRecordsPatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseStatusRecordPatch(row["patch"], `${at}.patch`),
  };
}

export function parseWorkshopPatch(value: unknown, at = "$"): WorkshopPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramWorkshopsPatchEntry(value: unknown, at = "$"): ProgramWorkshopsPatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseWorkshopPatch(row["patch"], `${at}.patch`),
  };
}

export function parseSurveyPatch(value: unknown, at = "$"): SurveyPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramSurveysPatchEntry(value: unknown, at = "$"): ProgramSurveysPatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseSurveyPatch(row["patch"], `${at}.patch`),
  };
}

export function parseIssuePatch(value: unknown, at = "$"): IssuePatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramIssuesPatchEntry(value: unknown, at = "$"): ProgramIssuesPatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseIssuePatch(row["patch"], `${at}.patch`),
  };
}

export function parseAuditEventPatch(value: unknown, at = "$"): AuditEventPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramAuditEventsPatchEntry(value: unknown, at = "$"): ProgramAuditEventsPatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseAuditEventPatch(row["patch"], `${at}.patch`),
  };
}

export function parseTemplateRecordPatch(value: unknown, at = "$"): TemplateRecordPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramTemplatesPatchEntry(value: unknown, at = "$"): ProgramTemplatesPatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseTemplateRecordPatch(row["patch"], `${at}.patch`),
  };
}

export function parseTraceLinkPatch(value: unknown, at = "$"): TraceLinkPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramTracesPatchEntry(value: unknown, at = "$"): ProgramTracesPatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseTraceLinkPatch(row["patch"], `${at}.patch`),
  };
}

export function parseKnowledgeRecordPatch(value: unknown, at = "$"): KnowledgeRecordPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramKnowledgePatchEntry(value: unknown, at = "$"): ProgramKnowledgePatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseKnowledgeRecordPatch(row["patch"], `${at}.patch`),
  };
}

export function parseBenchmarkRecordPatch(value: unknown, at = "$"): BenchmarkRecordPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProgramBenchmarksPatchEntry(value: unknown, at = "$"): ProgramBenchmarksPatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseBenchmarkRecordPatch(row["patch"], `${at}.patch`),
  };
}

export function parseProgramMetaPatch(value: unknown, at = "$"): ProgramMetaPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseProjectDefinitionPatch(value: unknown, at = "$"): ProjectDefinitionPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export function parseGovernancePatch(value: unknown, at = "$"): GovernancePatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}
