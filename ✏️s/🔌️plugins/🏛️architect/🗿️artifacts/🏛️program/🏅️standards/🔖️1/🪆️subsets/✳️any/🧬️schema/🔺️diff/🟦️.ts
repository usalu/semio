/** 🧬️ ProgramSnapshot diff schema — sparse field delta. */
import type { ProgramAccessibilityDelta, ProgramCommunicationDelta, ProgramCostsDelta, ProgramDeliveryDelta, ProgramEnvironmentalDelta, ProgramEquipmentDelta, ProgramFlexibilityDelta, ProgramGrowthDelta, ProgramHumanFactorsDelta, ProgramInformationDelta, ProgramInfrastructureDelta, ProgramOperationsDelta, ProgramOrganizationalDelta, ProgramPrivacyDelta, ProgramRegulatoryDelta, ProgramResilienceDelta, ProgramResourcesDelta, ProgramSafetyDelta, ProgramSchedulesDelta, ProgramSecurityDelta, ProgramServicesDelta, ProgramSiteContextDelta, ProgramStorageDelta, ProgramSustainabilityDelta, ProgramWayfindingDelta } from "./📋️deltas-requirements/🟦️.ts";
import type { GovernanceEdit, ProgramAnalysesDelta, ProgramApprovalsDelta, ProgramArtifactsDelta, ProgramAssumptionsDelta, ProgramAuditEventsDelta, ProgramBenchmarksDelta, ProgramChangesDelta, ProgramCollaborationDelta, ProgramComplianceRecordsDelta, ProgramConflictsDelta, ProgramConstraintsDelta, ProgramDecisionsDelta, ProgramIssuesDelta, ProgramKnowledgeDelta, ProgramMeetingsDelta, ProgramMetaEdit, ProgramOptionsDelta, ProgramPerformanceDelta, ProgramPrioritiesDelta, ProgramQualityDelta, ProgramReportsDelta, ProgramRequirementsDelta, ProgramRisksDelta, ProgramScenariosDelta, ProgramSearchFiltersDelta, ProgramStatusRecordsDelta, ProgramSurveysDelta, ProgramTemplatesDelta, ProgramTracesDelta, ProgramValidationsDelta, ProgramWorkshopsDelta, ProjectDefinitionEdit } from "./🗃️deltas-records/🟦️.ts";
import type { ProgramAccessRulesDelta, ProgramActivitiesDelta, ProgramAdjacenciesDelta, ProgramElementsDelta, ProgramFlowsDelta, ProgramFunctionsDelta, ProgramProcessesDelta, ProgramQuantitiesDelta, ProgramRelationshipsDelta, ProgramStakeholdersDelta, ProgramUsersDelta } from "./🧱️deltas-program-structure/🟦️.ts";
import { parseAccessRule, parseAccessibilityRequirement, parseActivity, parseAdjacency, parseAnalysisRecord, parseApprovalRecord, parseArtifactRecord, parseAssumption, parseAuditEvent, parseChangeRecord, parseCollaborationRecord, parseCommunicationRequirement, parseComplianceRecord, parseConflict, parseConstraintRecord, parseCostRequirement, parseDecision, parseDeliveryConstraint, parseEnvironmentalRequirement, parseEquipment, parseFlexibilityRequirement, parseFlowRequirement, parseFunction, parseGovernance, parseGrowthPlan, parseHumanFactorRequirement, parseInformationRequirement, parseInfrastructureRequirement, parseIssue, parseMeetingRecord, parseOperationalRequirement, parseOptionEvaluation, parseOrganizationalRequirement, parsePerformanceCriterion, parsePriorityRecord, parsePrivacyRequirement, parseProcess, parseProgramElement, parseProgramMeta, parseProjectDefinition, parseQualityRecord, parseQuantityRequirement, parseRegulatoryRequirement, parseRelationship, parseReportRecord, parseRequirement, parseResilienceRequirement, parseResource, parseRisk, parseSafetyRequirement, parseScenario, parseScheduleRequirement, parseSearchFilter, parseSecurityRequirement, parseServiceRequirement, parseSiteContext, parseStakeholder, parseStatusRecord, parseStorageRequirement, parseSurvey, parseSustainabilityRequirement, parseTemplateRecord, parseTraceLink, parseUserProfile, parseValidationRecord, parseWayfindingRequirement, parseWorkshop } from "../🟦️.ts";

import { architectProgramArtifactGuardExactObject, PROGRAM_ARTIFACT_FIELDS } from "../🟦️.ts";
import type { ProgramDiff } from "./🧾️program-diff/🟦️.ts";
export type { ProgramDiff } from "./🧾️program-diff/🟦️.ts";

export type { ProgramAccessibilityDelta, ProgramCommunicationDelta, ProgramCostsDelta, ProgramDeliveryDelta, ProgramEnvironmentalDelta, ProgramEquipmentDelta, ProgramFlexibilityDelta, ProgramGrowthDelta, ProgramHumanFactorsDelta, ProgramInformationDelta, ProgramInfrastructureDelta, ProgramOperationsDelta, ProgramOrganizationalDelta, ProgramPrivacyDelta, ProgramRegulatoryDelta, ProgramResilienceDelta, ProgramResourcesDelta, ProgramSafetyDelta, ProgramSchedulesDelta, ProgramSecurityDelta, ProgramServicesDelta, ProgramSiteContextDelta, ProgramStorageDelta, ProgramSustainabilityDelta, ProgramWayfindingDelta } from "./📋️deltas-requirements/🟦️.ts";
export type { GovernanceEdit, ProgramAnalysesDelta, ProgramApprovalsDelta, ProgramArtifactsDelta, ProgramAssumptionsDelta, ProgramAuditEventsDelta, ProgramBenchmarksDelta, ProgramChangesDelta, ProgramCollaborationDelta, ProgramComplianceRecordsDelta, ProgramConflictsDelta, ProgramConstraintsDelta, ProgramDecisionsDelta, ProgramIssuesDelta, ProgramKnowledgeDelta, ProgramMeetingsDelta, ProgramMetaEdit, ProgramOptionsDelta, ProgramPerformanceDelta, ProgramPrioritiesDelta, ProgramQualityDelta, ProgramReportsDelta, ProgramRequirementsDelta, ProgramRisksDelta, ProgramScenariosDelta, ProgramSearchFiltersDelta, ProgramStatusRecordsDelta, ProgramSurveysDelta, ProgramTemplatesDelta, ProgramTracesDelta, ProgramValidationsDelta, ProgramWorkshopsDelta, ProjectDefinitionEdit } from "./🗃️deltas-records/🟦️.ts";
export type { ProgramAccessRulesDelta, ProgramActivitiesDelta, ProgramAdjacenciesDelta, ProgramElementsDelta, ProgramFlowsDelta, ProgramFunctionsDelta, ProgramProcessesDelta, ProgramQuantitiesDelta, ProgramRelationshipsDelta, ProgramStakeholdersDelta, ProgramUsersDelta } from "./🧱️deltas-program-structure/🟦️.ts";

import type { ProgramBenchmarksInsertion, ProgramKnowledgeInsertion } from "./🗃️deltas-records/🟦️.ts";
export type { ProgramBenchmarksInsertion, ProgramKnowledgeInsertion } from "./🗃️deltas-records/🟦️.ts";

import { parseBenchmarkRecord, parseKnowledgeRecord } from "../🟦️.ts";
export { parseBenchmarkRecord, parseKnowledgeRecord } from "../🟦️.ts";
import type { BenchmarkRecord, KnowledgeRecord } from "../🟦️.ts";
export type { BenchmarkRecord, KnowledgeRecord } from "../🟦️.ts";

export const PROGRAM_DIFF_FIELDS = PROGRAM_ARTIFACT_FIELDS.filter((field) => field !== "knowledgePayload" && field !== "benchmarksPayload");

export function parseProgramDiff(value: unknown, at = "$"): ProgramDiff {
  const row = architectProgramArtifactGuardExactObject(value, at, PROGRAM_DIFF_FIELDS);
  return row as unknown as ProgramDiff;
}

//#region 🚪️Parsers
/** 🛑️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class architectProgramDiffGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const architectProgramDiffGuardReject = (at: string, why: string): never => {
  throw new architectProgramDiffGuardRefusal(at, why);
};

type architectProgramDiffGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type architectProgramDiffGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number; readonly exclusiveMinimum?: number; readonly exclusiveMaximum?: number };
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
  if (bounds.exclusiveMinimum !== undefined && value <= bounds.exclusiveMinimum) architectProgramDiffGuardReject(at, `number is not above ${bounds.exclusiveMinimum}`);
  if (bounds.exclusiveMaximum !== undefined && value >= bounds.exclusiveMaximum) architectProgramDiffGuardReject(at, `number is not below ${bounds.exclusiveMaximum}`);
  return value;
};
export const architectProgramDiffGuardInteger = (value: unknown, at: string, bounds: architectProgramDiffGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? architectProgramDiffGuardNumber(value, at, bounds) : architectProgramDiffGuardReject(at, "value is not an integer");
export const architectProgramDiffGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : architectProgramDiffGuardReject(at, `value is not one of ${members.join(", ")}`);
export const architectProgramDiffGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : architectProgramDiffGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export type AccessRulePatch = Readonly<Record<string, unknown>>;

export function parseAccessRulePatch(value: unknown, at = "$"): AccessRulePatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export type AccessibilityRequirementPatch = Readonly<Record<string, unknown>>;

export function parseAccessibilityRequirementPatch(value: unknown, at = "$"): AccessibilityRequirementPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export type ActivityPatch = Readonly<Record<string, unknown>>;

export function parseActivityPatch(value: unknown, at = "$"): ActivityPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export type AdjacencyPatch = Readonly<Record<string, unknown>>;

export function parseAdjacencyPatch(value: unknown, at = "$"): AdjacencyPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export type AnalysisRecordPatch = Readonly<Record<string, unknown>>;

export function parseAnalysisRecordPatch(value: unknown, at = "$"): AnalysisRecordPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export type ApprovalRecordPatch = Readonly<Record<string, unknown>>;

export function parseApprovalRecordPatch(value: unknown, at = "$"): ApprovalRecordPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export type ArtifactRecordPatch = Readonly<Record<string, unknown>>;

export function parseArtifactRecordPatch(value: unknown, at = "$"): ArtifactRecordPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export type AssumptionPatch = Readonly<Record<string, unknown>>;

export function parseAssumptionPatch(value: unknown, at = "$"): AssumptionPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export type AuditEventPatch = Readonly<Record<string, unknown>>;

export function parseAuditEventPatch(value: unknown, at = "$"): AuditEventPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}





export type BenchmarkRecordPatch = Readonly<Record<string, unknown>>;

export function parseBenchmarkRecordPatch(value: unknown, at = "$"): BenchmarkRecordPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export type ChangeRecordPatch = Readonly<Record<string, unknown>>;

export function parseChangeRecordPatch(value: unknown, at = "$"): ChangeRecordPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export type CollaborationRecordPatch = Readonly<Record<string, unknown>>;

export function parseCollaborationRecordPatch(value: unknown, at = "$"): CollaborationRecordPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export type CommunicationRequirementPatch = Readonly<Record<string, unknown>>;

export function parseCommunicationRequirementPatch(value: unknown, at = "$"): CommunicationRequirementPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export type ComplianceRecordPatch = Readonly<Record<string, unknown>>;

export function parseComplianceRecordPatch(value: unknown, at = "$"): ComplianceRecordPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export type ConflictPatch = Readonly<Record<string, unknown>>;

export function parseConflictPatch(value: unknown, at = "$"): ConflictPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export type ConstraintRecordPatch = Readonly<Record<string, unknown>>;

export function parseConstraintRecordPatch(value: unknown, at = "$"): ConstraintRecordPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export type CostRequirementPatch = Readonly<Record<string, unknown>>;

export function parseCostRequirementPatch(value: unknown, at = "$"): CostRequirementPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export type DecisionPatch = Readonly<Record<string, unknown>>;

export function parseDecisionPatch(value: unknown, at = "$"): DecisionPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export type DeliveryConstraintPatch = Readonly<Record<string, unknown>>;

export function parseDeliveryConstraintPatch(value: unknown, at = "$"): DeliveryConstraintPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export type EnvironmentalRequirementPatch = Readonly<Record<string, unknown>>;

export function parseEnvironmentalRequirementPatch(value: unknown, at = "$"): EnvironmentalRequirementPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export type EquipmentPatch = Readonly<Record<string, unknown>>;

export function parseEquipmentPatch(value: unknown, at = "$"): EquipmentPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export type FlexibilityRequirementPatch = Readonly<Record<string, unknown>>;

export function parseFlexibilityRequirementPatch(value: unknown, at = "$"): FlexibilityRequirementPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export type FlowRequirementPatch = Readonly<Record<string, unknown>>;

export function parseFlowRequirementPatch(value: unknown, at = "$"): FlowRequirementPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export type FunctionPatch = Readonly<Record<string, unknown>>;

export function parseFunctionPatch(value: unknown, at = "$"): FunctionPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}



export function parseGovernanceEdit(value: unknown, at = "$"): GovernanceEdit {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["set", "patch"]);
  return {
    set: (row["set"] === null ? null : parseGovernance(row["set"], `${at}.set`)),
    patch: (row["patch"] === null ? null : parseGovernancePatch(row["patch"], `${at}.patch`)),
  };
}

export type GovernancePatch = Readonly<Record<string, unknown>>;

export function parseGovernancePatch(value: unknown, at = "$"): GovernancePatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export type GrowthPlanPatch = Readonly<Record<string, unknown>>;

export function parseGrowthPlanPatch(value: unknown, at = "$"): GrowthPlanPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export type HumanFactorRequirementPatch = Readonly<Record<string, unknown>>;

export function parseHumanFactorRequirementPatch(value: unknown, at = "$"): HumanFactorRequirementPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export type InformationRequirementPatch = Readonly<Record<string, unknown>>;

export function parseInformationRequirementPatch(value: unknown, at = "$"): InformationRequirementPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export type InfrastructureRequirementPatch = Readonly<Record<string, unknown>>;

export function parseInfrastructureRequirementPatch(value: unknown, at = "$"): InfrastructureRequirementPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export type IssuePatch = Readonly<Record<string, unknown>>;

export function parseIssuePatch(value: unknown, at = "$"): IssuePatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}





export type KnowledgeRecordPatch = Readonly<Record<string, unknown>>;

export function parseKnowledgeRecordPatch(value: unknown, at = "$"): KnowledgeRecordPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export type LifecycleStatus = "draft" | "proposed" | "underReview" | "validated" | "approved" | "rejected" | "deferred" | "superseded" | "archived" | "open" | "closed" | "atRisk" | "blocked" | "inProgress" | "complete";

export function parseLifecycleStatus(value: unknown, at = "$"): LifecycleStatus {
  return architectProgramDiffGuardMember(value, `${at}`, ["draft", "proposed", "underReview", "validated", "approved", "rejected", "deferred", "superseded", "archived", "open", "closed", "atRisk", "blocked", "inProgress", "complete"] as const);
}

export type MeetingRecordPatch = Readonly<Record<string, unknown>>;

export function parseMeetingRecordPatch(value: unknown, at = "$"): MeetingRecordPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export type OperationalRequirementPatch = Readonly<Record<string, unknown>>;

export function parseOperationalRequirementPatch(value: unknown, at = "$"): OperationalRequirementPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export type OptionEvaluationPatch = Readonly<Record<string, unknown>>;

export function parseOptionEvaluationPatch(value: unknown, at = "$"): OptionEvaluationPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export type OrganizationalRequirementPatch = Readonly<Record<string, unknown>>;

export function parseOrganizationalRequirementPatch(value: unknown, at = "$"): OrganizationalRequirementPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export interface Ownership {
  readonly ownerId: string | null;
  readonly authorityId: string | null;
  readonly consultantIds: readonly string[];
  readonly participantIds: readonly string[];
}

export function parseOwnership(value: unknown, at = "$"): Ownership {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["ownerId", "authorityId", "consultantIds", "participantIds"]);
  return {
    ownerId: (row["ownerId"] === null ? null : architectProgramDiffGuardString(row["ownerId"], `${at}.ownerId`)),
    authorityId: (row["authorityId"] === null ? null : architectProgramDiffGuardString(row["authorityId"], `${at}.authorityId`)),
    consultantIds: architectProgramDiffGuardArray(row["consultantIds"], `${at}.consultantIds`).map((item, index) => architectProgramDiffGuardString(item, `${at}.consultantIds[${index}]`)),
    participantIds: architectProgramDiffGuardArray(row["participantIds"], `${at}.participantIds`).map((item, index) => architectProgramDiffGuardString(item, `${at}.participantIds[${index}]`)),
  };
}

export type PerformanceCriterionPatch = Readonly<Record<string, unknown>>;

export function parsePerformanceCriterionPatch(value: unknown, at = "$"): PerformanceCriterionPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export type Priority = "mandatory" | "essential" | "preferred" | "optional" | "deferred" | "prohibited";

export function parsePriority(value: unknown, at = "$"): Priority {
  return architectProgramDiffGuardMember(value, `${at}`, ["mandatory", "essential", "preferred", "optional", "deferred", "prohibited"] as const);
}

export type PriorityRecordPatch = Readonly<Record<string, unknown>>;

export function parsePriorityRecordPatch(value: unknown, at = "$"): PriorityRecordPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export type PrivacyRequirementPatch = Readonly<Record<string, unknown>>;

export function parsePrivacyRequirementPatch(value: unknown, at = "$"): PrivacyRequirementPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export type ProcessPatch = Readonly<Record<string, unknown>>;

export function parseProcessPatch(value: unknown, at = "$"): ProcessPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}



export function parseProgramAccessRulesDelta(value: unknown, at = "$"): ProgramAccessRulesDelta {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["removed", "inserted", "moved", "modified"]);
  return {
    removed: architectProgramDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => parseProgramAccessRulesRemoval(item, `${at}.removed[${index}]`)),
    inserted: architectProgramDiffGuardArray(row["inserted"], `${at}.inserted`).map((item, index) => parseProgramAccessRulesInsertion(item, `${at}.inserted[${index}]`)),
    moved: architectProgramDiffGuardArray(row["moved"], `${at}.moved`).map((item, index) => parseProgramAccessRulesRelocation(item, `${at}.moved[${index}]`)),
    modified: architectProgramDiffGuardArray(row["modified"], `${at}.modified`).map((item, index) => parseProgramAccessRulesPatchEntry(item, `${at}.modified[${index}]`)),
  };
}

export interface ProgramAccessRulesInsertion {
  readonly index: number;
  readonly row: import("../🟦️.ts").AccessRule;
}

export function parseProgramAccessRulesInsertion(value: unknown, at = "$"): ProgramAccessRulesInsertion {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["index", "row"]);
  return {
    index: architectProgramDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
    row: parseAccessRule(row["row"], `${at}.row`),
  };
}

export interface ProgramAccessRulesPatchEntry {
  readonly id: string;
  readonly patch: AccessRulePatch;
}

export function parseProgramAccessRulesPatchEntry(value: unknown, at = "$"): ProgramAccessRulesPatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseAccessRulePatch(row["patch"], `${at}.patch`),
  };
}

export interface ProgramAccessRulesRelocation {
  readonly id: string;
  readonly from: number;
  readonly to: number;
}

export function parseProgramAccessRulesRelocation(value: unknown, at = "$"): ProgramAccessRulesRelocation {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "from", "to"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    from: architectProgramDiffGuardInteger(row["from"], `${at}.from`, {"minimum": 0}),
    to: architectProgramDiffGuardInteger(row["to"], `${at}.to`, {"minimum": 0}),
  };
}

export interface ProgramAccessRulesRemoval {
  readonly id: string;
  readonly index: number;
}

export function parseProgramAccessRulesRemoval(value: unknown, at = "$"): ProgramAccessRulesRemoval {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "index"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    index: architectProgramDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
  };
}



export function parseProgramAccessibilityDelta(value: unknown, at = "$"): ProgramAccessibilityDelta {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["removed", "inserted", "moved", "modified"]);
  return {
    removed: architectProgramDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => parseProgramAccessibilityRemoval(item, `${at}.removed[${index}]`)),
    inserted: architectProgramDiffGuardArray(row["inserted"], `${at}.inserted`).map((item, index) => parseProgramAccessibilityInsertion(item, `${at}.inserted[${index}]`)),
    moved: architectProgramDiffGuardArray(row["moved"], `${at}.moved`).map((item, index) => parseProgramAccessibilityRelocation(item, `${at}.moved[${index}]`)),
    modified: architectProgramDiffGuardArray(row["modified"], `${at}.modified`).map((item, index) => parseProgramAccessibilityPatchEntry(item, `${at}.modified[${index}]`)),
  };
}

export interface ProgramAccessibilityInsertion {
  readonly index: number;
  readonly row: import("../🟦️.ts").AccessibilityRequirement;
}

export function parseProgramAccessibilityInsertion(value: unknown, at = "$"): ProgramAccessibilityInsertion {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["index", "row"]);
  return {
    index: architectProgramDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
    row: parseAccessibilityRequirement(row["row"], `${at}.row`),
  };
}

export interface ProgramAccessibilityPatchEntry {
  readonly id: string;
  readonly patch: AccessibilityRequirementPatch;
}

export function parseProgramAccessibilityPatchEntry(value: unknown, at = "$"): ProgramAccessibilityPatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseAccessibilityRequirementPatch(row["patch"], `${at}.patch`),
  };
}

export interface ProgramAccessibilityRelocation {
  readonly id: string;
  readonly from: number;
  readonly to: number;
}

export function parseProgramAccessibilityRelocation(value: unknown, at = "$"): ProgramAccessibilityRelocation {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "from", "to"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    from: architectProgramDiffGuardInteger(row["from"], `${at}.from`, {"minimum": 0}),
    to: architectProgramDiffGuardInteger(row["to"], `${at}.to`, {"minimum": 0}),
  };
}

export interface ProgramAccessibilityRemoval {
  readonly id: string;
  readonly index: number;
}

export function parseProgramAccessibilityRemoval(value: unknown, at = "$"): ProgramAccessibilityRemoval {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "index"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    index: architectProgramDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
  };
}



export function parseProgramActivitiesDelta(value: unknown, at = "$"): ProgramActivitiesDelta {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["removed", "inserted", "moved", "modified"]);
  return {
    removed: architectProgramDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => parseProgramActivitiesRemoval(item, `${at}.removed[${index}]`)),
    inserted: architectProgramDiffGuardArray(row["inserted"], `${at}.inserted`).map((item, index) => parseProgramActivitiesInsertion(item, `${at}.inserted[${index}]`)),
    moved: architectProgramDiffGuardArray(row["moved"], `${at}.moved`).map((item, index) => parseProgramActivitiesRelocation(item, `${at}.moved[${index}]`)),
    modified: architectProgramDiffGuardArray(row["modified"], `${at}.modified`).map((item, index) => parseProgramActivitiesPatchEntry(item, `${at}.modified[${index}]`)),
  };
}

export interface ProgramActivitiesInsertion {
  readonly index: number;
  readonly row: import("../🟦️.ts").Activity;
}

export function parseProgramActivitiesInsertion(value: unknown, at = "$"): ProgramActivitiesInsertion {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["index", "row"]);
  return {
    index: architectProgramDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
    row: parseActivity(row["row"], `${at}.row`),
  };
}

export interface ProgramActivitiesPatchEntry {
  readonly id: string;
  readonly patch: ActivityPatch;
}

export function parseProgramActivitiesPatchEntry(value: unknown, at = "$"): ProgramActivitiesPatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseActivityPatch(row["patch"], `${at}.patch`),
  };
}

export interface ProgramActivitiesRelocation {
  readonly id: string;
  readonly from: number;
  readonly to: number;
}

export function parseProgramActivitiesRelocation(value: unknown, at = "$"): ProgramActivitiesRelocation {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "from", "to"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    from: architectProgramDiffGuardInteger(row["from"], `${at}.from`, {"minimum": 0}),
    to: architectProgramDiffGuardInteger(row["to"], `${at}.to`, {"minimum": 0}),
  };
}

export interface ProgramActivitiesRemoval {
  readonly id: string;
  readonly index: number;
}

export function parseProgramActivitiesRemoval(value: unknown, at = "$"): ProgramActivitiesRemoval {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "index"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    index: architectProgramDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
  };
}



export function parseProgramAdjacenciesDelta(value: unknown, at = "$"): ProgramAdjacenciesDelta {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["removed", "inserted", "moved", "modified"]);
  return {
    removed: architectProgramDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => parseProgramAdjacenciesRemoval(item, `${at}.removed[${index}]`)),
    inserted: architectProgramDiffGuardArray(row["inserted"], `${at}.inserted`).map((item, index) => parseProgramAdjacenciesInsertion(item, `${at}.inserted[${index}]`)),
    moved: architectProgramDiffGuardArray(row["moved"], `${at}.moved`).map((item, index) => parseProgramAdjacenciesRelocation(item, `${at}.moved[${index}]`)),
    modified: architectProgramDiffGuardArray(row["modified"], `${at}.modified`).map((item, index) => parseProgramAdjacenciesPatchEntry(item, `${at}.modified[${index}]`)),
  };
}

export interface ProgramAdjacenciesInsertion {
  readonly index: number;
  readonly row: import("../🟦️.ts").Adjacency;
}

export function parseProgramAdjacenciesInsertion(value: unknown, at = "$"): ProgramAdjacenciesInsertion {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["index", "row"]);
  return {
    index: architectProgramDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
    row: parseAdjacency(row["row"], `${at}.row`),
  };
}

export interface ProgramAdjacenciesPatchEntry {
  readonly id: string;
  readonly patch: AdjacencyPatch;
}

export function parseProgramAdjacenciesPatchEntry(value: unknown, at = "$"): ProgramAdjacenciesPatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseAdjacencyPatch(row["patch"], `${at}.patch`),
  };
}

export interface ProgramAdjacenciesRelocation {
  readonly id: string;
  readonly from: number;
  readonly to: number;
}

export function parseProgramAdjacenciesRelocation(value: unknown, at = "$"): ProgramAdjacenciesRelocation {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "from", "to"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    from: architectProgramDiffGuardInteger(row["from"], `${at}.from`, {"minimum": 0}),
    to: architectProgramDiffGuardInteger(row["to"], `${at}.to`, {"minimum": 0}),
  };
}

export interface ProgramAdjacenciesRemoval {
  readonly id: string;
  readonly index: number;
}

export function parseProgramAdjacenciesRemoval(value: unknown, at = "$"): ProgramAdjacenciesRemoval {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "index"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    index: architectProgramDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
  };
}



export function parseProgramAnalysesDelta(value: unknown, at = "$"): ProgramAnalysesDelta {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["removed", "inserted", "moved", "modified"]);
  return {
    removed: architectProgramDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => parseProgramAnalysesRemoval(item, `${at}.removed[${index}]`)),
    inserted: architectProgramDiffGuardArray(row["inserted"], `${at}.inserted`).map((item, index) => parseProgramAnalysesInsertion(item, `${at}.inserted[${index}]`)),
    moved: architectProgramDiffGuardArray(row["moved"], `${at}.moved`).map((item, index) => parseProgramAnalysesRelocation(item, `${at}.moved[${index}]`)),
    modified: architectProgramDiffGuardArray(row["modified"], `${at}.modified`).map((item, index) => parseProgramAnalysesPatchEntry(item, `${at}.modified[${index}]`)),
  };
}

export interface ProgramAnalysesInsertion {
  readonly index: number;
  readonly row: import("../🟦️.ts").AnalysisRecord;
}

export function parseProgramAnalysesInsertion(value: unknown, at = "$"): ProgramAnalysesInsertion {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["index", "row"]);
  return {
    index: architectProgramDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
    row: parseAnalysisRecord(row["row"], `${at}.row`),
  };
}

export interface ProgramAnalysesPatchEntry {
  readonly id: string;
  readonly patch: AnalysisRecordPatch;
}

export function parseProgramAnalysesPatchEntry(value: unknown, at = "$"): ProgramAnalysesPatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseAnalysisRecordPatch(row["patch"], `${at}.patch`),
  };
}

export interface ProgramAnalysesRelocation {
  readonly id: string;
  readonly from: number;
  readonly to: number;
}

export function parseProgramAnalysesRelocation(value: unknown, at = "$"): ProgramAnalysesRelocation {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "from", "to"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    from: architectProgramDiffGuardInteger(row["from"], `${at}.from`, {"minimum": 0}),
    to: architectProgramDiffGuardInteger(row["to"], `${at}.to`, {"minimum": 0}),
  };
}

export interface ProgramAnalysesRemoval {
  readonly id: string;
  readonly index: number;
}

export function parseProgramAnalysesRemoval(value: unknown, at = "$"): ProgramAnalysesRemoval {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "index"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    index: architectProgramDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
  };
}



export function parseProgramApprovalsDelta(value: unknown, at = "$"): ProgramApprovalsDelta {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["removed", "inserted", "moved", "modified"]);
  return {
    removed: architectProgramDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => parseProgramApprovalsRemoval(item, `${at}.removed[${index}]`)),
    inserted: architectProgramDiffGuardArray(row["inserted"], `${at}.inserted`).map((item, index) => parseProgramApprovalsInsertion(item, `${at}.inserted[${index}]`)),
    moved: architectProgramDiffGuardArray(row["moved"], `${at}.moved`).map((item, index) => parseProgramApprovalsRelocation(item, `${at}.moved[${index}]`)),
    modified: architectProgramDiffGuardArray(row["modified"], `${at}.modified`).map((item, index) => parseProgramApprovalsPatchEntry(item, `${at}.modified[${index}]`)),
  };
}

export interface ProgramApprovalsInsertion {
  readonly index: number;
  readonly row: import("../🟦️.ts").ApprovalRecord;
}

export function parseProgramApprovalsInsertion(value: unknown, at = "$"): ProgramApprovalsInsertion {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["index", "row"]);
  return {
    index: architectProgramDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
    row: parseApprovalRecord(row["row"], `${at}.row`),
  };
}

export interface ProgramApprovalsPatchEntry {
  readonly id: string;
  readonly patch: ApprovalRecordPatch;
}

export function parseProgramApprovalsPatchEntry(value: unknown, at = "$"): ProgramApprovalsPatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseApprovalRecordPatch(row["patch"], `${at}.patch`),
  };
}

export interface ProgramApprovalsRelocation {
  readonly id: string;
  readonly from: number;
  readonly to: number;
}

export function parseProgramApprovalsRelocation(value: unknown, at = "$"): ProgramApprovalsRelocation {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "from", "to"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    from: architectProgramDiffGuardInteger(row["from"], `${at}.from`, {"minimum": 0}),
    to: architectProgramDiffGuardInteger(row["to"], `${at}.to`, {"minimum": 0}),
  };
}

export interface ProgramApprovalsRemoval {
  readonly id: string;
  readonly index: number;
}

export function parseProgramApprovalsRemoval(value: unknown, at = "$"): ProgramApprovalsRemoval {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "index"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    index: architectProgramDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
  };
}



export function parseProgramArtifactsDelta(value: unknown, at = "$"): ProgramArtifactsDelta {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["removed", "inserted", "moved", "modified"]);
  return {
    removed: architectProgramDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => parseProgramArtifactsRemoval(item, `${at}.removed[${index}]`)),
    inserted: architectProgramDiffGuardArray(row["inserted"], `${at}.inserted`).map((item, index) => parseProgramArtifactsInsertion(item, `${at}.inserted[${index}]`)),
    moved: architectProgramDiffGuardArray(row["moved"], `${at}.moved`).map((item, index) => parseProgramArtifactsRelocation(item, `${at}.moved[${index}]`)),
    modified: architectProgramDiffGuardArray(row["modified"], `${at}.modified`).map((item, index) => parseProgramArtifactsPatchEntry(item, `${at}.modified[${index}]`)),
  };
}

export interface ProgramArtifactsInsertion {
  readonly index: number;
  readonly row: import("../🟦️.ts").ArtifactRecord;
}

export function parseProgramArtifactsInsertion(value: unknown, at = "$"): ProgramArtifactsInsertion {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["index", "row"]);
  return {
    index: architectProgramDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
    row: parseArtifactRecord(row["row"], `${at}.row`),
  };
}

export interface ProgramArtifactsPatchEntry {
  readonly id: string;
  readonly patch: ArtifactRecordPatch;
}

export function parseProgramArtifactsPatchEntry(value: unknown, at = "$"): ProgramArtifactsPatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseArtifactRecordPatch(row["patch"], `${at}.patch`),
  };
}

export interface ProgramArtifactsRelocation {
  readonly id: string;
  readonly from: number;
  readonly to: number;
}

export function parseProgramArtifactsRelocation(value: unknown, at = "$"): ProgramArtifactsRelocation {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "from", "to"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    from: architectProgramDiffGuardInteger(row["from"], `${at}.from`, {"minimum": 0}),
    to: architectProgramDiffGuardInteger(row["to"], `${at}.to`, {"minimum": 0}),
  };
}

export interface ProgramArtifactsRemoval {
  readonly id: string;
  readonly index: number;
}

export function parseProgramArtifactsRemoval(value: unknown, at = "$"): ProgramArtifactsRemoval {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "index"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    index: architectProgramDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
  };
}



export function parseProgramAssumptionsDelta(value: unknown, at = "$"): ProgramAssumptionsDelta {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["removed", "inserted", "moved", "modified"]);
  return {
    removed: architectProgramDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => parseProgramAssumptionsRemoval(item, `${at}.removed[${index}]`)),
    inserted: architectProgramDiffGuardArray(row["inserted"], `${at}.inserted`).map((item, index) => parseProgramAssumptionsInsertion(item, `${at}.inserted[${index}]`)),
    moved: architectProgramDiffGuardArray(row["moved"], `${at}.moved`).map((item, index) => parseProgramAssumptionsRelocation(item, `${at}.moved[${index}]`)),
    modified: architectProgramDiffGuardArray(row["modified"], `${at}.modified`).map((item, index) => parseProgramAssumptionsPatchEntry(item, `${at}.modified[${index}]`)),
  };
}

export interface ProgramAssumptionsInsertion {
  readonly index: number;
  readonly row: import("../🟦️.ts").Assumption;
}

export function parseProgramAssumptionsInsertion(value: unknown, at = "$"): ProgramAssumptionsInsertion {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["index", "row"]);
  return {
    index: architectProgramDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
    row: parseAssumption(row["row"], `${at}.row`),
  };
}

export interface ProgramAssumptionsPatchEntry {
  readonly id: string;
  readonly patch: AssumptionPatch;
}

export function parseProgramAssumptionsPatchEntry(value: unknown, at = "$"): ProgramAssumptionsPatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseAssumptionPatch(row["patch"], `${at}.patch`),
  };
}

export interface ProgramAssumptionsRelocation {
  readonly id: string;
  readonly from: number;
  readonly to: number;
}

export function parseProgramAssumptionsRelocation(value: unknown, at = "$"): ProgramAssumptionsRelocation {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "from", "to"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    from: architectProgramDiffGuardInteger(row["from"], `${at}.from`, {"minimum": 0}),
    to: architectProgramDiffGuardInteger(row["to"], `${at}.to`, {"minimum": 0}),
  };
}

export interface ProgramAssumptionsRemoval {
  readonly id: string;
  readonly index: number;
}

export function parseProgramAssumptionsRemoval(value: unknown, at = "$"): ProgramAssumptionsRemoval {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "index"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    index: architectProgramDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
  };
}



export function parseProgramAuditEventsDelta(value: unknown, at = "$"): ProgramAuditEventsDelta {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["removed", "inserted", "moved", "modified"]);
  return {
    removed: architectProgramDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => parseProgramAuditEventsRemoval(item, `${at}.removed[${index}]`)),
    inserted: architectProgramDiffGuardArray(row["inserted"], `${at}.inserted`).map((item, index) => parseProgramAuditEventsInsertion(item, `${at}.inserted[${index}]`)),
    moved: architectProgramDiffGuardArray(row["moved"], `${at}.moved`).map((item, index) => parseProgramAuditEventsRelocation(item, `${at}.moved[${index}]`)),
    modified: architectProgramDiffGuardArray(row["modified"], `${at}.modified`).map((item, index) => parseProgramAuditEventsPatchEntry(item, `${at}.modified[${index}]`)),
  };
}

export interface ProgramAuditEventsInsertion {
  readonly index: number;
  readonly row: import("../🟦️.ts").AuditEvent;
}

export function parseProgramAuditEventsInsertion(value: unknown, at = "$"): ProgramAuditEventsInsertion {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["index", "row"]);
  return {
    index: architectProgramDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
    row: parseAuditEvent(row["row"], `${at}.row`),
  };
}

export interface ProgramAuditEventsPatchEntry {
  readonly id: string;
  readonly patch: AuditEventPatch;
}

export function parseProgramAuditEventsPatchEntry(value: unknown, at = "$"): ProgramAuditEventsPatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseAuditEventPatch(row["patch"], `${at}.patch`),
  };
}

export interface ProgramAuditEventsRelocation {
  readonly id: string;
  readonly from: number;
  readonly to: number;
}

export function parseProgramAuditEventsRelocation(value: unknown, at = "$"): ProgramAuditEventsRelocation {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "from", "to"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    from: architectProgramDiffGuardInteger(row["from"], `${at}.from`, {"minimum": 0}),
    to: architectProgramDiffGuardInteger(row["to"], `${at}.to`, {"minimum": 0}),
  };
}

export interface ProgramAuditEventsRemoval {
  readonly id: string;
  readonly index: number;
}

export function parseProgramAuditEventsRemoval(value: unknown, at = "$"): ProgramAuditEventsRemoval {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "index"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    index: architectProgramDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
  };
}



export function parseProgramBenchmarksDelta(value: unknown, at = "$"): ProgramBenchmarksDelta {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["removed", "inserted", "moved", "modified"]);
  return {
    removed: architectProgramDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => parseProgramBenchmarksRemoval(item, `${at}.removed[${index}]`)),
    inserted: architectProgramDiffGuardArray(row["inserted"], `${at}.inserted`).map((item, index) => parseProgramBenchmarksInsertion(item, `${at}.inserted[${index}]`)),
    moved: architectProgramDiffGuardArray(row["moved"], `${at}.moved`).map((item, index) => parseProgramBenchmarksRelocation(item, `${at}.moved[${index}]`)),
    modified: architectProgramDiffGuardArray(row["modified"], `${at}.modified`).map((item, index) => parseProgramBenchmarksPatchEntry(item, `${at}.modified[${index}]`)),
  };
}



export function parseProgramBenchmarksInsertion(value: unknown, at = "$"): ProgramBenchmarksInsertion {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["index", "row"]);
  return {
    index: architectProgramDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
    row: parseBenchmarkRecord(row["row"], `${at}.row`),
  };
}

export interface ProgramBenchmarksPatchEntry {
  readonly id: string;
  readonly patch: BenchmarkRecordPatch;
}

export function parseProgramBenchmarksPatchEntry(value: unknown, at = "$"): ProgramBenchmarksPatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseBenchmarkRecordPatch(row["patch"], `${at}.patch`),
  };
}

export interface ProgramBenchmarksRelocation {
  readonly id: string;
  readonly from: number;
  readonly to: number;
}

export function parseProgramBenchmarksRelocation(value: unknown, at = "$"): ProgramBenchmarksRelocation {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "from", "to"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    from: architectProgramDiffGuardInteger(row["from"], `${at}.from`, {"minimum": 0}),
    to: architectProgramDiffGuardInteger(row["to"], `${at}.to`, {"minimum": 0}),
  };
}

export interface ProgramBenchmarksRemoval {
  readonly id: string;
  readonly index: number;
}

export function parseProgramBenchmarksRemoval(value: unknown, at = "$"): ProgramBenchmarksRemoval {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "index"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    index: architectProgramDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
  };
}



export function parseProgramChangesDelta(value: unknown, at = "$"): ProgramChangesDelta {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["removed", "inserted", "moved", "modified"]);
  return {
    removed: architectProgramDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => parseProgramChangesRemoval(item, `${at}.removed[${index}]`)),
    inserted: architectProgramDiffGuardArray(row["inserted"], `${at}.inserted`).map((item, index) => parseProgramChangesInsertion(item, `${at}.inserted[${index}]`)),
    moved: architectProgramDiffGuardArray(row["moved"], `${at}.moved`).map((item, index) => parseProgramChangesRelocation(item, `${at}.moved[${index}]`)),
    modified: architectProgramDiffGuardArray(row["modified"], `${at}.modified`).map((item, index) => parseProgramChangesPatchEntry(item, `${at}.modified[${index}]`)),
  };
}

export interface ProgramChangesInsertion {
  readonly index: number;
  readonly row: import("../🟦️.ts").ChangeRecord;
}

export function parseProgramChangesInsertion(value: unknown, at = "$"): ProgramChangesInsertion {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["index", "row"]);
  return {
    index: architectProgramDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
    row: parseChangeRecord(row["row"], `${at}.row`),
  };
}

export interface ProgramChangesPatchEntry {
  readonly id: string;
  readonly patch: ChangeRecordPatch;
}

export function parseProgramChangesPatchEntry(value: unknown, at = "$"): ProgramChangesPatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseChangeRecordPatch(row["patch"], `${at}.patch`),
  };
}

export interface ProgramChangesRelocation {
  readonly id: string;
  readonly from: number;
  readonly to: number;
}

export function parseProgramChangesRelocation(value: unknown, at = "$"): ProgramChangesRelocation {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "from", "to"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    from: architectProgramDiffGuardInteger(row["from"], `${at}.from`, {"minimum": 0}),
    to: architectProgramDiffGuardInteger(row["to"], `${at}.to`, {"minimum": 0}),
  };
}

export interface ProgramChangesRemoval {
  readonly id: string;
  readonly index: number;
}

export function parseProgramChangesRemoval(value: unknown, at = "$"): ProgramChangesRemoval {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "index"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    index: architectProgramDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
  };
}



export function parseProgramCollaborationDelta(value: unknown, at = "$"): ProgramCollaborationDelta {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["removed", "inserted", "moved", "modified"]);
  return {
    removed: architectProgramDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => parseProgramCollaborationRemoval(item, `${at}.removed[${index}]`)),
    inserted: architectProgramDiffGuardArray(row["inserted"], `${at}.inserted`).map((item, index) => parseProgramCollaborationInsertion(item, `${at}.inserted[${index}]`)),
    moved: architectProgramDiffGuardArray(row["moved"], `${at}.moved`).map((item, index) => parseProgramCollaborationRelocation(item, `${at}.moved[${index}]`)),
    modified: architectProgramDiffGuardArray(row["modified"], `${at}.modified`).map((item, index) => parseProgramCollaborationPatchEntry(item, `${at}.modified[${index}]`)),
  };
}

export interface ProgramCollaborationInsertion {
  readonly index: number;
  readonly row: import("../🟦️.ts").CollaborationRecord;
}

export function parseProgramCollaborationInsertion(value: unknown, at = "$"): ProgramCollaborationInsertion {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["index", "row"]);
  return {
    index: architectProgramDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
    row: parseCollaborationRecord(row["row"], `${at}.row`),
  };
}

export interface ProgramCollaborationPatchEntry {
  readonly id: string;
  readonly patch: CollaborationRecordPatch;
}

export function parseProgramCollaborationPatchEntry(value: unknown, at = "$"): ProgramCollaborationPatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseCollaborationRecordPatch(row["patch"], `${at}.patch`),
  };
}

export interface ProgramCollaborationRelocation {
  readonly id: string;
  readonly from: number;
  readonly to: number;
}

export function parseProgramCollaborationRelocation(value: unknown, at = "$"): ProgramCollaborationRelocation {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "from", "to"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    from: architectProgramDiffGuardInteger(row["from"], `${at}.from`, {"minimum": 0}),
    to: architectProgramDiffGuardInteger(row["to"], `${at}.to`, {"minimum": 0}),
  };
}

export interface ProgramCollaborationRemoval {
  readonly id: string;
  readonly index: number;
}

export function parseProgramCollaborationRemoval(value: unknown, at = "$"): ProgramCollaborationRemoval {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "index"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    index: architectProgramDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
  };
}



export function parseProgramCommunicationDelta(value: unknown, at = "$"): ProgramCommunicationDelta {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["removed", "inserted", "moved", "modified"]);
  return {
    removed: architectProgramDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => parseProgramCommunicationRemoval(item, `${at}.removed[${index}]`)),
    inserted: architectProgramDiffGuardArray(row["inserted"], `${at}.inserted`).map((item, index) => parseProgramCommunicationInsertion(item, `${at}.inserted[${index}]`)),
    moved: architectProgramDiffGuardArray(row["moved"], `${at}.moved`).map((item, index) => parseProgramCommunicationRelocation(item, `${at}.moved[${index}]`)),
    modified: architectProgramDiffGuardArray(row["modified"], `${at}.modified`).map((item, index) => parseProgramCommunicationPatchEntry(item, `${at}.modified[${index}]`)),
  };
}

export interface ProgramCommunicationInsertion {
  readonly index: number;
  readonly row: import("../🟦️.ts").CommunicationRequirement;
}

export function parseProgramCommunicationInsertion(value: unknown, at = "$"): ProgramCommunicationInsertion {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["index", "row"]);
  return {
    index: architectProgramDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
    row: parseCommunicationRequirement(row["row"], `${at}.row`),
  };
}

export interface ProgramCommunicationPatchEntry {
  readonly id: string;
  readonly patch: CommunicationRequirementPatch;
}

export function parseProgramCommunicationPatchEntry(value: unknown, at = "$"): ProgramCommunicationPatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseCommunicationRequirementPatch(row["patch"], `${at}.patch`),
  };
}

export interface ProgramCommunicationRelocation {
  readonly id: string;
  readonly from: number;
  readonly to: number;
}

export function parseProgramCommunicationRelocation(value: unknown, at = "$"): ProgramCommunicationRelocation {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "from", "to"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    from: architectProgramDiffGuardInteger(row["from"], `${at}.from`, {"minimum": 0}),
    to: architectProgramDiffGuardInteger(row["to"], `${at}.to`, {"minimum": 0}),
  };
}

export interface ProgramCommunicationRemoval {
  readonly id: string;
  readonly index: number;
}

export function parseProgramCommunicationRemoval(value: unknown, at = "$"): ProgramCommunicationRemoval {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "index"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    index: architectProgramDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
  };
}



export function parseProgramComplianceRecordsDelta(value: unknown, at = "$"): ProgramComplianceRecordsDelta {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["removed", "inserted", "moved", "modified"]);
  return {
    removed: architectProgramDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => parseProgramComplianceRecordsRemoval(item, `${at}.removed[${index}]`)),
    inserted: architectProgramDiffGuardArray(row["inserted"], `${at}.inserted`).map((item, index) => parseProgramComplianceRecordsInsertion(item, `${at}.inserted[${index}]`)),
    moved: architectProgramDiffGuardArray(row["moved"], `${at}.moved`).map((item, index) => parseProgramComplianceRecordsRelocation(item, `${at}.moved[${index}]`)),
    modified: architectProgramDiffGuardArray(row["modified"], `${at}.modified`).map((item, index) => parseProgramComplianceRecordsPatchEntry(item, `${at}.modified[${index}]`)),
  };
}

export interface ProgramComplianceRecordsInsertion {
  readonly index: number;
  readonly row: import("../🟦️.ts").ComplianceRecord;
}

export function parseProgramComplianceRecordsInsertion(value: unknown, at = "$"): ProgramComplianceRecordsInsertion {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["index", "row"]);
  return {
    index: architectProgramDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
    row: parseComplianceRecord(row["row"], `${at}.row`),
  };
}

export interface ProgramComplianceRecordsPatchEntry {
  readonly id: string;
  readonly patch: ComplianceRecordPatch;
}

export function parseProgramComplianceRecordsPatchEntry(value: unknown, at = "$"): ProgramComplianceRecordsPatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseComplianceRecordPatch(row["patch"], `${at}.patch`),
  };
}

export interface ProgramComplianceRecordsRelocation {
  readonly id: string;
  readonly from: number;
  readonly to: number;
}

export function parseProgramComplianceRecordsRelocation(value: unknown, at = "$"): ProgramComplianceRecordsRelocation {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "from", "to"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    from: architectProgramDiffGuardInteger(row["from"], `${at}.from`, {"minimum": 0}),
    to: architectProgramDiffGuardInteger(row["to"], `${at}.to`, {"minimum": 0}),
  };
}

export interface ProgramComplianceRecordsRemoval {
  readonly id: string;
  readonly index: number;
}

export function parseProgramComplianceRecordsRemoval(value: unknown, at = "$"): ProgramComplianceRecordsRemoval {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "index"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    index: architectProgramDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
  };
}



export function parseProgramConflictsDelta(value: unknown, at = "$"): ProgramConflictsDelta {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["removed", "inserted", "moved", "modified"]);
  return {
    removed: architectProgramDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => parseProgramConflictsRemoval(item, `${at}.removed[${index}]`)),
    inserted: architectProgramDiffGuardArray(row["inserted"], `${at}.inserted`).map((item, index) => parseProgramConflictsInsertion(item, `${at}.inserted[${index}]`)),
    moved: architectProgramDiffGuardArray(row["moved"], `${at}.moved`).map((item, index) => parseProgramConflictsRelocation(item, `${at}.moved[${index}]`)),
    modified: architectProgramDiffGuardArray(row["modified"], `${at}.modified`).map((item, index) => parseProgramConflictsPatchEntry(item, `${at}.modified[${index}]`)),
  };
}

export interface ProgramConflictsInsertion {
  readonly index: number;
  readonly row: import("../🟦️.ts").Conflict;
}

export function parseProgramConflictsInsertion(value: unknown, at = "$"): ProgramConflictsInsertion {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["index", "row"]);
  return {
    index: architectProgramDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
    row: parseConflict(row["row"], `${at}.row`),
  };
}

export interface ProgramConflictsPatchEntry {
  readonly id: string;
  readonly patch: ConflictPatch;
}

export function parseProgramConflictsPatchEntry(value: unknown, at = "$"): ProgramConflictsPatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseConflictPatch(row["patch"], `${at}.patch`),
  };
}

export interface ProgramConflictsRelocation {
  readonly id: string;
  readonly from: number;
  readonly to: number;
}

export function parseProgramConflictsRelocation(value: unknown, at = "$"): ProgramConflictsRelocation {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "from", "to"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    from: architectProgramDiffGuardInteger(row["from"], `${at}.from`, {"minimum": 0}),
    to: architectProgramDiffGuardInteger(row["to"], `${at}.to`, {"minimum": 0}),
  };
}

export interface ProgramConflictsRemoval {
  readonly id: string;
  readonly index: number;
}

export function parseProgramConflictsRemoval(value: unknown, at = "$"): ProgramConflictsRemoval {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "index"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    index: architectProgramDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
  };
}



export function parseProgramConstraintsDelta(value: unknown, at = "$"): ProgramConstraintsDelta {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["removed", "inserted", "moved", "modified"]);
  return {
    removed: architectProgramDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => parseProgramConstraintsRemoval(item, `${at}.removed[${index}]`)),
    inserted: architectProgramDiffGuardArray(row["inserted"], `${at}.inserted`).map((item, index) => parseProgramConstraintsInsertion(item, `${at}.inserted[${index}]`)),
    moved: architectProgramDiffGuardArray(row["moved"], `${at}.moved`).map((item, index) => parseProgramConstraintsRelocation(item, `${at}.moved[${index}]`)),
    modified: architectProgramDiffGuardArray(row["modified"], `${at}.modified`).map((item, index) => parseProgramConstraintsPatchEntry(item, `${at}.modified[${index}]`)),
  };
}

export interface ProgramConstraintsInsertion {
  readonly index: number;
  readonly row: import("../🟦️.ts").ConstraintRecord;
}

export function parseProgramConstraintsInsertion(value: unknown, at = "$"): ProgramConstraintsInsertion {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["index", "row"]);
  return {
    index: architectProgramDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
    row: parseConstraintRecord(row["row"], `${at}.row`),
  };
}

export interface ProgramConstraintsPatchEntry {
  readonly id: string;
  readonly patch: ConstraintRecordPatch;
}

export function parseProgramConstraintsPatchEntry(value: unknown, at = "$"): ProgramConstraintsPatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseConstraintRecordPatch(row["patch"], `${at}.patch`),
  };
}

export interface ProgramConstraintsRelocation {
  readonly id: string;
  readonly from: number;
  readonly to: number;
}

export function parseProgramConstraintsRelocation(value: unknown, at = "$"): ProgramConstraintsRelocation {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "from", "to"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    from: architectProgramDiffGuardInteger(row["from"], `${at}.from`, {"minimum": 0}),
    to: architectProgramDiffGuardInteger(row["to"], `${at}.to`, {"minimum": 0}),
  };
}

export interface ProgramConstraintsRemoval {
  readonly id: string;
  readonly index: number;
}

export function parseProgramConstraintsRemoval(value: unknown, at = "$"): ProgramConstraintsRemoval {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "index"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    index: architectProgramDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
  };
}



export function parseProgramCostsDelta(value: unknown, at = "$"): ProgramCostsDelta {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["removed", "inserted", "moved", "modified"]);
  return {
    removed: architectProgramDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => parseProgramCostsRemoval(item, `${at}.removed[${index}]`)),
    inserted: architectProgramDiffGuardArray(row["inserted"], `${at}.inserted`).map((item, index) => parseProgramCostsInsertion(item, `${at}.inserted[${index}]`)),
    moved: architectProgramDiffGuardArray(row["moved"], `${at}.moved`).map((item, index) => parseProgramCostsRelocation(item, `${at}.moved[${index}]`)),
    modified: architectProgramDiffGuardArray(row["modified"], `${at}.modified`).map((item, index) => parseProgramCostsPatchEntry(item, `${at}.modified[${index}]`)),
  };
}

export interface ProgramCostsInsertion {
  readonly index: number;
  readonly row: import("../🟦️.ts").CostRequirement;
}

export function parseProgramCostsInsertion(value: unknown, at = "$"): ProgramCostsInsertion {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["index", "row"]);
  return {
    index: architectProgramDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
    row: parseCostRequirement(row["row"], `${at}.row`),
  };
}

export interface ProgramCostsPatchEntry {
  readonly id: string;
  readonly patch: CostRequirementPatch;
}

export function parseProgramCostsPatchEntry(value: unknown, at = "$"): ProgramCostsPatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseCostRequirementPatch(row["patch"], `${at}.patch`),
  };
}

export interface ProgramCostsRelocation {
  readonly id: string;
  readonly from: number;
  readonly to: number;
}

export function parseProgramCostsRelocation(value: unknown, at = "$"): ProgramCostsRelocation {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "from", "to"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    from: architectProgramDiffGuardInteger(row["from"], `${at}.from`, {"minimum": 0}),
    to: architectProgramDiffGuardInteger(row["to"], `${at}.to`, {"minimum": 0}),
  };
}

export interface ProgramCostsRemoval {
  readonly id: string;
  readonly index: number;
}

export function parseProgramCostsRemoval(value: unknown, at = "$"): ProgramCostsRemoval {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "index"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    index: architectProgramDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
  };
}



export function parseProgramDecisionsDelta(value: unknown, at = "$"): ProgramDecisionsDelta {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["removed", "inserted", "moved", "modified"]);
  return {
    removed: architectProgramDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => parseProgramDecisionsRemoval(item, `${at}.removed[${index}]`)),
    inserted: architectProgramDiffGuardArray(row["inserted"], `${at}.inserted`).map((item, index) => parseProgramDecisionsInsertion(item, `${at}.inserted[${index}]`)),
    moved: architectProgramDiffGuardArray(row["moved"], `${at}.moved`).map((item, index) => parseProgramDecisionsRelocation(item, `${at}.moved[${index}]`)),
    modified: architectProgramDiffGuardArray(row["modified"], `${at}.modified`).map((item, index) => parseProgramDecisionsPatchEntry(item, `${at}.modified[${index}]`)),
  };
}

export interface ProgramDecisionsInsertion {
  readonly index: number;
  readonly row: import("../🟦️.ts").Decision;
}

export function parseProgramDecisionsInsertion(value: unknown, at = "$"): ProgramDecisionsInsertion {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["index", "row"]);
  return {
    index: architectProgramDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
    row: parseDecision(row["row"], `${at}.row`),
  };
}

export interface ProgramDecisionsPatchEntry {
  readonly id: string;
  readonly patch: DecisionPatch;
}

export function parseProgramDecisionsPatchEntry(value: unknown, at = "$"): ProgramDecisionsPatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseDecisionPatch(row["patch"], `${at}.patch`),
  };
}

export interface ProgramDecisionsRelocation {
  readonly id: string;
  readonly from: number;
  readonly to: number;
}

export function parseProgramDecisionsRelocation(value: unknown, at = "$"): ProgramDecisionsRelocation {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "from", "to"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    from: architectProgramDiffGuardInteger(row["from"], `${at}.from`, {"minimum": 0}),
    to: architectProgramDiffGuardInteger(row["to"], `${at}.to`, {"minimum": 0}),
  };
}

export interface ProgramDecisionsRemoval {
  readonly id: string;
  readonly index: number;
}

export function parseProgramDecisionsRemoval(value: unknown, at = "$"): ProgramDecisionsRemoval {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "index"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    index: architectProgramDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
  };
}



export function parseProgramDeliveryDelta(value: unknown, at = "$"): ProgramDeliveryDelta {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["removed", "inserted", "moved", "modified"]);
  return {
    removed: architectProgramDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => parseProgramDeliveryRemoval(item, `${at}.removed[${index}]`)),
    inserted: architectProgramDiffGuardArray(row["inserted"], `${at}.inserted`).map((item, index) => parseProgramDeliveryInsertion(item, `${at}.inserted[${index}]`)),
    moved: architectProgramDiffGuardArray(row["moved"], `${at}.moved`).map((item, index) => parseProgramDeliveryRelocation(item, `${at}.moved[${index}]`)),
    modified: architectProgramDiffGuardArray(row["modified"], `${at}.modified`).map((item, index) => parseProgramDeliveryPatchEntry(item, `${at}.modified[${index}]`)),
  };
}

export interface ProgramDeliveryInsertion {
  readonly index: number;
  readonly row: import("../🟦️.ts").DeliveryConstraint;
}

export function parseProgramDeliveryInsertion(value: unknown, at = "$"): ProgramDeliveryInsertion {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["index", "row"]);
  return {
    index: architectProgramDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
    row: parseDeliveryConstraint(row["row"], `${at}.row`),
  };
}

export interface ProgramDeliveryPatchEntry {
  readonly id: string;
  readonly patch: DeliveryConstraintPatch;
}

export function parseProgramDeliveryPatchEntry(value: unknown, at = "$"): ProgramDeliveryPatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseDeliveryConstraintPatch(row["patch"], `${at}.patch`),
  };
}

export interface ProgramDeliveryRelocation {
  readonly id: string;
  readonly from: number;
  readonly to: number;
}

export function parseProgramDeliveryRelocation(value: unknown, at = "$"): ProgramDeliveryRelocation {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "from", "to"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    from: architectProgramDiffGuardInteger(row["from"], `${at}.from`, {"minimum": 0}),
    to: architectProgramDiffGuardInteger(row["to"], `${at}.to`, {"minimum": 0}),
  };
}

export interface ProgramDeliveryRemoval {
  readonly id: string;
  readonly index: number;
}

export function parseProgramDeliveryRemoval(value: unknown, at = "$"): ProgramDeliveryRemoval {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "index"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    index: architectProgramDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
  };
}

export type ProgramElementPatch = Readonly<Record<string, unknown>>;

export function parseProgramElementPatch(value: unknown, at = "$"): ProgramElementPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}



export function parseProgramElementsDelta(value: unknown, at = "$"): ProgramElementsDelta {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["removed", "inserted", "moved", "modified"]);
  return {
    removed: architectProgramDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => parseProgramElementsRemoval(item, `${at}.removed[${index}]`)),
    inserted: architectProgramDiffGuardArray(row["inserted"], `${at}.inserted`).map((item, index) => parseProgramElementsInsertion(item, `${at}.inserted[${index}]`)),
    moved: architectProgramDiffGuardArray(row["moved"], `${at}.moved`).map((item, index) => parseProgramElementsRelocation(item, `${at}.moved[${index}]`)),
    modified: architectProgramDiffGuardArray(row["modified"], `${at}.modified`).map((item, index) => parseProgramElementsPatchEntry(item, `${at}.modified[${index}]`)),
  };
}

export interface ProgramElementsInsertion {
  readonly index: number;
  readonly row: import("../🟦️.ts").ProgramElement;
}

export function parseProgramElementsInsertion(value: unknown, at = "$"): ProgramElementsInsertion {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["index", "row"]);
  return {
    index: architectProgramDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
    row: parseProgramElement(row["row"], `${at}.row`),
  };
}

export interface ProgramElementsPatchEntry {
  readonly id: string;
  readonly patch: ProgramElementPatch;
}

export function parseProgramElementsPatchEntry(value: unknown, at = "$"): ProgramElementsPatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseProgramElementPatch(row["patch"], `${at}.patch`),
  };
}

export interface ProgramElementsRelocation {
  readonly id: string;
  readonly from: number;
  readonly to: number;
}

export function parseProgramElementsRelocation(value: unknown, at = "$"): ProgramElementsRelocation {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "from", "to"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    from: architectProgramDiffGuardInteger(row["from"], `${at}.from`, {"minimum": 0}),
    to: architectProgramDiffGuardInteger(row["to"], `${at}.to`, {"minimum": 0}),
  };
}

export interface ProgramElementsRemoval {
  readonly id: string;
  readonly index: number;
}

export function parseProgramElementsRemoval(value: unknown, at = "$"): ProgramElementsRemoval {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "index"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    index: architectProgramDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
  };
}



export function parseProgramEnvironmentalDelta(value: unknown, at = "$"): ProgramEnvironmentalDelta {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["removed", "inserted", "moved", "modified"]);
  return {
    removed: architectProgramDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => parseProgramEnvironmentalRemoval(item, `${at}.removed[${index}]`)),
    inserted: architectProgramDiffGuardArray(row["inserted"], `${at}.inserted`).map((item, index) => parseProgramEnvironmentalInsertion(item, `${at}.inserted[${index}]`)),
    moved: architectProgramDiffGuardArray(row["moved"], `${at}.moved`).map((item, index) => parseProgramEnvironmentalRelocation(item, `${at}.moved[${index}]`)),
    modified: architectProgramDiffGuardArray(row["modified"], `${at}.modified`).map((item, index) => parseProgramEnvironmentalPatchEntry(item, `${at}.modified[${index}]`)),
  };
}

export interface ProgramEnvironmentalInsertion {
  readonly index: number;
  readonly row: import("../🟦️.ts").EnvironmentalRequirement;
}

export function parseProgramEnvironmentalInsertion(value: unknown, at = "$"): ProgramEnvironmentalInsertion {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["index", "row"]);
  return {
    index: architectProgramDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
    row: parseEnvironmentalRequirement(row["row"], `${at}.row`),
  };
}

export interface ProgramEnvironmentalPatchEntry {
  readonly id: string;
  readonly patch: EnvironmentalRequirementPatch;
}

export function parseProgramEnvironmentalPatchEntry(value: unknown, at = "$"): ProgramEnvironmentalPatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseEnvironmentalRequirementPatch(row["patch"], `${at}.patch`),
  };
}

export interface ProgramEnvironmentalRelocation {
  readonly id: string;
  readonly from: number;
  readonly to: number;
}

export function parseProgramEnvironmentalRelocation(value: unknown, at = "$"): ProgramEnvironmentalRelocation {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "from", "to"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    from: architectProgramDiffGuardInteger(row["from"], `${at}.from`, {"minimum": 0}),
    to: architectProgramDiffGuardInteger(row["to"], `${at}.to`, {"minimum": 0}),
  };
}

export interface ProgramEnvironmentalRemoval {
  readonly id: string;
  readonly index: number;
}

export function parseProgramEnvironmentalRemoval(value: unknown, at = "$"): ProgramEnvironmentalRemoval {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "index"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    index: architectProgramDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
  };
}



export function parseProgramEquipmentDelta(value: unknown, at = "$"): ProgramEquipmentDelta {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["removed", "inserted", "moved", "modified"]);
  return {
    removed: architectProgramDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => parseProgramEquipmentRemoval(item, `${at}.removed[${index}]`)),
    inserted: architectProgramDiffGuardArray(row["inserted"], `${at}.inserted`).map((item, index) => parseProgramEquipmentInsertion(item, `${at}.inserted[${index}]`)),
    moved: architectProgramDiffGuardArray(row["moved"], `${at}.moved`).map((item, index) => parseProgramEquipmentRelocation(item, `${at}.moved[${index}]`)),
    modified: architectProgramDiffGuardArray(row["modified"], `${at}.modified`).map((item, index) => parseProgramEquipmentPatchEntry(item, `${at}.modified[${index}]`)),
  };
}

export interface ProgramEquipmentInsertion {
  readonly index: number;
  readonly row: import("../🟦️.ts").Equipment;
}

export function parseProgramEquipmentInsertion(value: unknown, at = "$"): ProgramEquipmentInsertion {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["index", "row"]);
  return {
    index: architectProgramDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
    row: parseEquipment(row["row"], `${at}.row`),
  };
}

export interface ProgramEquipmentPatchEntry {
  readonly id: string;
  readonly patch: EquipmentPatch;
}

export function parseProgramEquipmentPatchEntry(value: unknown, at = "$"): ProgramEquipmentPatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseEquipmentPatch(row["patch"], `${at}.patch`),
  };
}

export interface ProgramEquipmentRelocation {
  readonly id: string;
  readonly from: number;
  readonly to: number;
}

export function parseProgramEquipmentRelocation(value: unknown, at = "$"): ProgramEquipmentRelocation {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "from", "to"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    from: architectProgramDiffGuardInteger(row["from"], `${at}.from`, {"minimum": 0}),
    to: architectProgramDiffGuardInteger(row["to"], `${at}.to`, {"minimum": 0}),
  };
}

export interface ProgramEquipmentRemoval {
  readonly id: string;
  readonly index: number;
}

export function parseProgramEquipmentRemoval(value: unknown, at = "$"): ProgramEquipmentRemoval {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "index"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    index: architectProgramDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
  };
}



export function parseProgramFlexibilityDelta(value: unknown, at = "$"): ProgramFlexibilityDelta {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["removed", "inserted", "moved", "modified"]);
  return {
    removed: architectProgramDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => parseProgramFlexibilityRemoval(item, `${at}.removed[${index}]`)),
    inserted: architectProgramDiffGuardArray(row["inserted"], `${at}.inserted`).map((item, index) => parseProgramFlexibilityInsertion(item, `${at}.inserted[${index}]`)),
    moved: architectProgramDiffGuardArray(row["moved"], `${at}.moved`).map((item, index) => parseProgramFlexibilityRelocation(item, `${at}.moved[${index}]`)),
    modified: architectProgramDiffGuardArray(row["modified"], `${at}.modified`).map((item, index) => parseProgramFlexibilityPatchEntry(item, `${at}.modified[${index}]`)),
  };
}

export interface ProgramFlexibilityInsertion {
  readonly index: number;
  readonly row: import("../🟦️.ts").FlexibilityRequirement;
}

export function parseProgramFlexibilityInsertion(value: unknown, at = "$"): ProgramFlexibilityInsertion {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["index", "row"]);
  return {
    index: architectProgramDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
    row: parseFlexibilityRequirement(row["row"], `${at}.row`),
  };
}

export interface ProgramFlexibilityPatchEntry {
  readonly id: string;
  readonly patch: FlexibilityRequirementPatch;
}

export function parseProgramFlexibilityPatchEntry(value: unknown, at = "$"): ProgramFlexibilityPatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseFlexibilityRequirementPatch(row["patch"], `${at}.patch`),
  };
}

export interface ProgramFlexibilityRelocation {
  readonly id: string;
  readonly from: number;
  readonly to: number;
}

export function parseProgramFlexibilityRelocation(value: unknown, at = "$"): ProgramFlexibilityRelocation {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "from", "to"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    from: architectProgramDiffGuardInteger(row["from"], `${at}.from`, {"minimum": 0}),
    to: architectProgramDiffGuardInteger(row["to"], `${at}.to`, {"minimum": 0}),
  };
}

export interface ProgramFlexibilityRemoval {
  readonly id: string;
  readonly index: number;
}

export function parseProgramFlexibilityRemoval(value: unknown, at = "$"): ProgramFlexibilityRemoval {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "index"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    index: architectProgramDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
  };
}



export function parseProgramFlowsDelta(value: unknown, at = "$"): ProgramFlowsDelta {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["removed", "inserted", "moved", "modified"]);
  return {
    removed: architectProgramDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => parseProgramFlowsRemoval(item, `${at}.removed[${index}]`)),
    inserted: architectProgramDiffGuardArray(row["inserted"], `${at}.inserted`).map((item, index) => parseProgramFlowsInsertion(item, `${at}.inserted[${index}]`)),
    moved: architectProgramDiffGuardArray(row["moved"], `${at}.moved`).map((item, index) => parseProgramFlowsRelocation(item, `${at}.moved[${index}]`)),
    modified: architectProgramDiffGuardArray(row["modified"], `${at}.modified`).map((item, index) => parseProgramFlowsPatchEntry(item, `${at}.modified[${index}]`)),
  };
}

export interface ProgramFlowsInsertion {
  readonly index: number;
  readonly row: import("../🟦️.ts").FlowRequirement;
}

export function parseProgramFlowsInsertion(value: unknown, at = "$"): ProgramFlowsInsertion {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["index", "row"]);
  return {
    index: architectProgramDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
    row: parseFlowRequirement(row["row"], `${at}.row`),
  };
}

export interface ProgramFlowsPatchEntry {
  readonly id: string;
  readonly patch: FlowRequirementPatch;
}

export function parseProgramFlowsPatchEntry(value: unknown, at = "$"): ProgramFlowsPatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseFlowRequirementPatch(row["patch"], `${at}.patch`),
  };
}

export interface ProgramFlowsRelocation {
  readonly id: string;
  readonly from: number;
  readonly to: number;
}

export function parseProgramFlowsRelocation(value: unknown, at = "$"): ProgramFlowsRelocation {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "from", "to"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    from: architectProgramDiffGuardInteger(row["from"], `${at}.from`, {"minimum": 0}),
    to: architectProgramDiffGuardInteger(row["to"], `${at}.to`, {"minimum": 0}),
  };
}

export interface ProgramFlowsRemoval {
  readonly id: string;
  readonly index: number;
}

export function parseProgramFlowsRemoval(value: unknown, at = "$"): ProgramFlowsRemoval {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "index"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    index: architectProgramDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
  };
}



export function parseProgramFunctionsDelta(value: unknown, at = "$"): ProgramFunctionsDelta {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["removed", "inserted", "moved", "modified"]);
  return {
    removed: architectProgramDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => parseProgramFunctionsRemoval(item, `${at}.removed[${index}]`)),
    inserted: architectProgramDiffGuardArray(row["inserted"], `${at}.inserted`).map((item, index) => parseProgramFunctionsInsertion(item, `${at}.inserted[${index}]`)),
    moved: architectProgramDiffGuardArray(row["moved"], `${at}.moved`).map((item, index) => parseProgramFunctionsRelocation(item, `${at}.moved[${index}]`)),
    modified: architectProgramDiffGuardArray(row["modified"], `${at}.modified`).map((item, index) => parseProgramFunctionsPatchEntry(item, `${at}.modified[${index}]`)),
  };
}

export interface ProgramFunctionsInsertion {
  readonly index: number;
  readonly row: import("../🟦️.ts").Function;
}

export function parseProgramFunctionsInsertion(value: unknown, at = "$"): ProgramFunctionsInsertion {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["index", "row"]);
  return {
    index: architectProgramDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
    row: parseFunction(row["row"], `${at}.row`),
  };
}

export interface ProgramFunctionsPatchEntry {
  readonly id: string;
  readonly patch: FunctionPatch;
}

export function parseProgramFunctionsPatchEntry(value: unknown, at = "$"): ProgramFunctionsPatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseFunctionPatch(row["patch"], `${at}.patch`),
  };
}

export interface ProgramFunctionsRelocation {
  readonly id: string;
  readonly from: number;
  readonly to: number;
}

export function parseProgramFunctionsRelocation(value: unknown, at = "$"): ProgramFunctionsRelocation {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "from", "to"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    from: architectProgramDiffGuardInteger(row["from"], `${at}.from`, {"minimum": 0}),
    to: architectProgramDiffGuardInteger(row["to"], `${at}.to`, {"minimum": 0}),
  };
}

export interface ProgramFunctionsRemoval {
  readonly id: string;
  readonly index: number;
}

export function parseProgramFunctionsRemoval(value: unknown, at = "$"): ProgramFunctionsRemoval {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "index"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    index: architectProgramDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
  };
}



export function parseProgramGrowthDelta(value: unknown, at = "$"): ProgramGrowthDelta {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["removed", "inserted", "moved", "modified"]);
  return {
    removed: architectProgramDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => parseProgramGrowthRemoval(item, `${at}.removed[${index}]`)),
    inserted: architectProgramDiffGuardArray(row["inserted"], `${at}.inserted`).map((item, index) => parseProgramGrowthInsertion(item, `${at}.inserted[${index}]`)),
    moved: architectProgramDiffGuardArray(row["moved"], `${at}.moved`).map((item, index) => parseProgramGrowthRelocation(item, `${at}.moved[${index}]`)),
    modified: architectProgramDiffGuardArray(row["modified"], `${at}.modified`).map((item, index) => parseProgramGrowthPatchEntry(item, `${at}.modified[${index}]`)),
  };
}

export interface ProgramGrowthInsertion {
  readonly index: number;
  readonly row: import("../🟦️.ts").GrowthPlan;
}

export function parseProgramGrowthInsertion(value: unknown, at = "$"): ProgramGrowthInsertion {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["index", "row"]);
  return {
    index: architectProgramDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
    row: parseGrowthPlan(row["row"], `${at}.row`),
  };
}

export interface ProgramGrowthPatchEntry {
  readonly id: string;
  readonly patch: GrowthPlanPatch;
}

export function parseProgramGrowthPatchEntry(value: unknown, at = "$"): ProgramGrowthPatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseGrowthPlanPatch(row["patch"], `${at}.patch`),
  };
}

export interface ProgramGrowthRelocation {
  readonly id: string;
  readonly from: number;
  readonly to: number;
}

export function parseProgramGrowthRelocation(value: unknown, at = "$"): ProgramGrowthRelocation {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "from", "to"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    from: architectProgramDiffGuardInteger(row["from"], `${at}.from`, {"minimum": 0}),
    to: architectProgramDiffGuardInteger(row["to"], `${at}.to`, {"minimum": 0}),
  };
}

export interface ProgramGrowthRemoval {
  readonly id: string;
  readonly index: number;
}

export function parseProgramGrowthRemoval(value: unknown, at = "$"): ProgramGrowthRemoval {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "index"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    index: architectProgramDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
  };
}



export function parseProgramHumanFactorsDelta(value: unknown, at = "$"): ProgramHumanFactorsDelta {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["removed", "inserted", "moved", "modified"]);
  return {
    removed: architectProgramDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => parseProgramHumanFactorsRemoval(item, `${at}.removed[${index}]`)),
    inserted: architectProgramDiffGuardArray(row["inserted"], `${at}.inserted`).map((item, index) => parseProgramHumanFactorsInsertion(item, `${at}.inserted[${index}]`)),
    moved: architectProgramDiffGuardArray(row["moved"], `${at}.moved`).map((item, index) => parseProgramHumanFactorsRelocation(item, `${at}.moved[${index}]`)),
    modified: architectProgramDiffGuardArray(row["modified"], `${at}.modified`).map((item, index) => parseProgramHumanFactorsPatchEntry(item, `${at}.modified[${index}]`)),
  };
}

export interface ProgramHumanFactorsInsertion {
  readonly index: number;
  readonly row: import("../🟦️.ts").HumanFactorRequirement;
}

export function parseProgramHumanFactorsInsertion(value: unknown, at = "$"): ProgramHumanFactorsInsertion {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["index", "row"]);
  return {
    index: architectProgramDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
    row: parseHumanFactorRequirement(row["row"], `${at}.row`),
  };
}

export interface ProgramHumanFactorsPatchEntry {
  readonly id: string;
  readonly patch: HumanFactorRequirementPatch;
}

export function parseProgramHumanFactorsPatchEntry(value: unknown, at = "$"): ProgramHumanFactorsPatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseHumanFactorRequirementPatch(row["patch"], `${at}.patch`),
  };
}

export interface ProgramHumanFactorsRelocation {
  readonly id: string;
  readonly from: number;
  readonly to: number;
}

export function parseProgramHumanFactorsRelocation(value: unknown, at = "$"): ProgramHumanFactorsRelocation {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "from", "to"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    from: architectProgramDiffGuardInteger(row["from"], `${at}.from`, {"minimum": 0}),
    to: architectProgramDiffGuardInteger(row["to"], `${at}.to`, {"minimum": 0}),
  };
}

export interface ProgramHumanFactorsRemoval {
  readonly id: string;
  readonly index: number;
}

export function parseProgramHumanFactorsRemoval(value: unknown, at = "$"): ProgramHumanFactorsRemoval {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "index"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    index: architectProgramDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
  };
}



export function parseProgramInformationDelta(value: unknown, at = "$"): ProgramInformationDelta {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["removed", "inserted", "moved", "modified"]);
  return {
    removed: architectProgramDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => parseProgramInformationRemoval(item, `${at}.removed[${index}]`)),
    inserted: architectProgramDiffGuardArray(row["inserted"], `${at}.inserted`).map((item, index) => parseProgramInformationInsertion(item, `${at}.inserted[${index}]`)),
    moved: architectProgramDiffGuardArray(row["moved"], `${at}.moved`).map((item, index) => parseProgramInformationRelocation(item, `${at}.moved[${index}]`)),
    modified: architectProgramDiffGuardArray(row["modified"], `${at}.modified`).map((item, index) => parseProgramInformationPatchEntry(item, `${at}.modified[${index}]`)),
  };
}

export interface ProgramInformationInsertion {
  readonly index: number;
  readonly row: import("../🟦️.ts").InformationRequirement;
}

export function parseProgramInformationInsertion(value: unknown, at = "$"): ProgramInformationInsertion {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["index", "row"]);
  return {
    index: architectProgramDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
    row: parseInformationRequirement(row["row"], `${at}.row`),
  };
}

export interface ProgramInformationPatchEntry {
  readonly id: string;
  readonly patch: InformationRequirementPatch;
}

export function parseProgramInformationPatchEntry(value: unknown, at = "$"): ProgramInformationPatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseInformationRequirementPatch(row["patch"], `${at}.patch`),
  };
}

export interface ProgramInformationRelocation {
  readonly id: string;
  readonly from: number;
  readonly to: number;
}

export function parseProgramInformationRelocation(value: unknown, at = "$"): ProgramInformationRelocation {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "from", "to"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    from: architectProgramDiffGuardInteger(row["from"], `${at}.from`, {"minimum": 0}),
    to: architectProgramDiffGuardInteger(row["to"], `${at}.to`, {"minimum": 0}),
  };
}

export interface ProgramInformationRemoval {
  readonly id: string;
  readonly index: number;
}

export function parseProgramInformationRemoval(value: unknown, at = "$"): ProgramInformationRemoval {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "index"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    index: architectProgramDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
  };
}



export function parseProgramInfrastructureDelta(value: unknown, at = "$"): ProgramInfrastructureDelta {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["removed", "inserted", "moved", "modified"]);
  return {
    removed: architectProgramDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => parseProgramInfrastructureRemoval(item, `${at}.removed[${index}]`)),
    inserted: architectProgramDiffGuardArray(row["inserted"], `${at}.inserted`).map((item, index) => parseProgramInfrastructureInsertion(item, `${at}.inserted[${index}]`)),
    moved: architectProgramDiffGuardArray(row["moved"], `${at}.moved`).map((item, index) => parseProgramInfrastructureRelocation(item, `${at}.moved[${index}]`)),
    modified: architectProgramDiffGuardArray(row["modified"], `${at}.modified`).map((item, index) => parseProgramInfrastructurePatchEntry(item, `${at}.modified[${index}]`)),
  };
}

export interface ProgramInfrastructureInsertion {
  readonly index: number;
  readonly row: import("../🟦️.ts").InfrastructureRequirement;
}

export function parseProgramInfrastructureInsertion(value: unknown, at = "$"): ProgramInfrastructureInsertion {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["index", "row"]);
  return {
    index: architectProgramDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
    row: parseInfrastructureRequirement(row["row"], `${at}.row`),
  };
}

export interface ProgramInfrastructurePatchEntry {
  readonly id: string;
  readonly patch: InfrastructureRequirementPatch;
}

export function parseProgramInfrastructurePatchEntry(value: unknown, at = "$"): ProgramInfrastructurePatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseInfrastructureRequirementPatch(row["patch"], `${at}.patch`),
  };
}

export interface ProgramInfrastructureRelocation {
  readonly id: string;
  readonly from: number;
  readonly to: number;
}

export function parseProgramInfrastructureRelocation(value: unknown, at = "$"): ProgramInfrastructureRelocation {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "from", "to"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    from: architectProgramDiffGuardInteger(row["from"], `${at}.from`, {"minimum": 0}),
    to: architectProgramDiffGuardInteger(row["to"], `${at}.to`, {"minimum": 0}),
  };
}

export interface ProgramInfrastructureRemoval {
  readonly id: string;
  readonly index: number;
}

export function parseProgramInfrastructureRemoval(value: unknown, at = "$"): ProgramInfrastructureRemoval {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "index"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    index: architectProgramDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
  };
}



export function parseProgramIssuesDelta(value: unknown, at = "$"): ProgramIssuesDelta {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["removed", "inserted", "moved", "modified"]);
  return {
    removed: architectProgramDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => parseProgramIssuesRemoval(item, `${at}.removed[${index}]`)),
    inserted: architectProgramDiffGuardArray(row["inserted"], `${at}.inserted`).map((item, index) => parseProgramIssuesInsertion(item, `${at}.inserted[${index}]`)),
    moved: architectProgramDiffGuardArray(row["moved"], `${at}.moved`).map((item, index) => parseProgramIssuesRelocation(item, `${at}.moved[${index}]`)),
    modified: architectProgramDiffGuardArray(row["modified"], `${at}.modified`).map((item, index) => parseProgramIssuesPatchEntry(item, `${at}.modified[${index}]`)),
  };
}

export interface ProgramIssuesInsertion {
  readonly index: number;
  readonly row: import("../🟦️.ts").Issue;
}

export function parseProgramIssuesInsertion(value: unknown, at = "$"): ProgramIssuesInsertion {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["index", "row"]);
  return {
    index: architectProgramDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
    row: parseIssue(row["row"], `${at}.row`),
  };
}

export interface ProgramIssuesPatchEntry {
  readonly id: string;
  readonly patch: IssuePatch;
}

export function parseProgramIssuesPatchEntry(value: unknown, at = "$"): ProgramIssuesPatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseIssuePatch(row["patch"], `${at}.patch`),
  };
}

export interface ProgramIssuesRelocation {
  readonly id: string;
  readonly from: number;
  readonly to: number;
}

export function parseProgramIssuesRelocation(value: unknown, at = "$"): ProgramIssuesRelocation {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "from", "to"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    from: architectProgramDiffGuardInteger(row["from"], `${at}.from`, {"minimum": 0}),
    to: architectProgramDiffGuardInteger(row["to"], `${at}.to`, {"minimum": 0}),
  };
}

export interface ProgramIssuesRemoval {
  readonly id: string;
  readonly index: number;
}

export function parseProgramIssuesRemoval(value: unknown, at = "$"): ProgramIssuesRemoval {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "index"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    index: architectProgramDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
  };
}



export function parseProgramKnowledgeDelta(value: unknown, at = "$"): ProgramKnowledgeDelta {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["removed", "inserted", "moved", "modified"]);
  return {
    removed: architectProgramDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => parseProgramKnowledgeRemoval(item, `${at}.removed[${index}]`)),
    inserted: architectProgramDiffGuardArray(row["inserted"], `${at}.inserted`).map((item, index) => parseProgramKnowledgeInsertion(item, `${at}.inserted[${index}]`)),
    moved: architectProgramDiffGuardArray(row["moved"], `${at}.moved`).map((item, index) => parseProgramKnowledgeRelocation(item, `${at}.moved[${index}]`)),
    modified: architectProgramDiffGuardArray(row["modified"], `${at}.modified`).map((item, index) => parseProgramKnowledgePatchEntry(item, `${at}.modified[${index}]`)),
  };
}



export function parseProgramKnowledgeInsertion(value: unknown, at = "$"): ProgramKnowledgeInsertion {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["index", "row"]);
  return {
    index: architectProgramDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
    row: parseKnowledgeRecord(row["row"], `${at}.row`),
  };
}

export interface ProgramKnowledgePatchEntry {
  readonly id: string;
  readonly patch: KnowledgeRecordPatch;
}

export function parseProgramKnowledgePatchEntry(value: unknown, at = "$"): ProgramKnowledgePatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseKnowledgeRecordPatch(row["patch"], `${at}.patch`),
  };
}

export interface ProgramKnowledgeRelocation {
  readonly id: string;
  readonly from: number;
  readonly to: number;
}

export function parseProgramKnowledgeRelocation(value: unknown, at = "$"): ProgramKnowledgeRelocation {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "from", "to"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    from: architectProgramDiffGuardInteger(row["from"], `${at}.from`, {"minimum": 0}),
    to: architectProgramDiffGuardInteger(row["to"], `${at}.to`, {"minimum": 0}),
  };
}

export interface ProgramKnowledgeRemoval {
  readonly id: string;
  readonly index: number;
}

export function parseProgramKnowledgeRemoval(value: unknown, at = "$"): ProgramKnowledgeRemoval {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "index"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    index: architectProgramDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
  };
}



export function parseProgramMeetingsDelta(value: unknown, at = "$"): ProgramMeetingsDelta {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["removed", "inserted", "moved", "modified"]);
  return {
    removed: architectProgramDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => parseProgramMeetingsRemoval(item, `${at}.removed[${index}]`)),
    inserted: architectProgramDiffGuardArray(row["inserted"], `${at}.inserted`).map((item, index) => parseProgramMeetingsInsertion(item, `${at}.inserted[${index}]`)),
    moved: architectProgramDiffGuardArray(row["moved"], `${at}.moved`).map((item, index) => parseProgramMeetingsRelocation(item, `${at}.moved[${index}]`)),
    modified: architectProgramDiffGuardArray(row["modified"], `${at}.modified`).map((item, index) => parseProgramMeetingsPatchEntry(item, `${at}.modified[${index}]`)),
  };
}

export interface ProgramMeetingsInsertion {
  readonly index: number;
  readonly row: import("../🟦️.ts").MeetingRecord;
}

export function parseProgramMeetingsInsertion(value: unknown, at = "$"): ProgramMeetingsInsertion {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["index", "row"]);
  return {
    index: architectProgramDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
    row: parseMeetingRecord(row["row"], `${at}.row`),
  };
}

export interface ProgramMeetingsPatchEntry {
  readonly id: string;
  readonly patch: MeetingRecordPatch;
}

export function parseProgramMeetingsPatchEntry(value: unknown, at = "$"): ProgramMeetingsPatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseMeetingRecordPatch(row["patch"], `${at}.patch`),
  };
}

export interface ProgramMeetingsRelocation {
  readonly id: string;
  readonly from: number;
  readonly to: number;
}

export function parseProgramMeetingsRelocation(value: unknown, at = "$"): ProgramMeetingsRelocation {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "from", "to"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    from: architectProgramDiffGuardInteger(row["from"], `${at}.from`, {"minimum": 0}),
    to: architectProgramDiffGuardInteger(row["to"], `${at}.to`, {"minimum": 0}),
  };
}

export interface ProgramMeetingsRemoval {
  readonly id: string;
  readonly index: number;
}

export function parseProgramMeetingsRemoval(value: unknown, at = "$"): ProgramMeetingsRemoval {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "index"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    index: architectProgramDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
  };
}



export function parseProgramMetaEdit(value: unknown, at = "$"): ProgramMetaEdit {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["set", "patch"]);
  return {
    set: (row["set"] === null ? null : parseProgramMeta(row["set"], `${at}.set`)),
    patch: (row["patch"] === null ? null : parseProgramMetaPatch(row["patch"], `${at}.patch`)),
  };
}

export type ProgramMetaPatch = Readonly<Record<string, unknown>>;

export function parseProgramMetaPatch(value: unknown, at = "$"): ProgramMetaPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}



export function parseProgramOperationsDelta(value: unknown, at = "$"): ProgramOperationsDelta {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["removed", "inserted", "moved", "modified"]);
  return {
    removed: architectProgramDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => parseProgramOperationsRemoval(item, `${at}.removed[${index}]`)),
    inserted: architectProgramDiffGuardArray(row["inserted"], `${at}.inserted`).map((item, index) => parseProgramOperationsInsertion(item, `${at}.inserted[${index}]`)),
    moved: architectProgramDiffGuardArray(row["moved"], `${at}.moved`).map((item, index) => parseProgramOperationsRelocation(item, `${at}.moved[${index}]`)),
    modified: architectProgramDiffGuardArray(row["modified"], `${at}.modified`).map((item, index) => parseProgramOperationsPatchEntry(item, `${at}.modified[${index}]`)),
  };
}

export interface ProgramOperationsInsertion {
  readonly index: number;
  readonly row: import("../🟦️.ts").OperationalRequirement;
}

export function parseProgramOperationsInsertion(value: unknown, at = "$"): ProgramOperationsInsertion {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["index", "row"]);
  return {
    index: architectProgramDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
    row: parseOperationalRequirement(row["row"], `${at}.row`),
  };
}

export interface ProgramOperationsPatchEntry {
  readonly id: string;
  readonly patch: OperationalRequirementPatch;
}

export function parseProgramOperationsPatchEntry(value: unknown, at = "$"): ProgramOperationsPatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseOperationalRequirementPatch(row["patch"], `${at}.patch`),
  };
}

export interface ProgramOperationsRelocation {
  readonly id: string;
  readonly from: number;
  readonly to: number;
}

export function parseProgramOperationsRelocation(value: unknown, at = "$"): ProgramOperationsRelocation {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "from", "to"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    from: architectProgramDiffGuardInteger(row["from"], `${at}.from`, {"minimum": 0}),
    to: architectProgramDiffGuardInteger(row["to"], `${at}.to`, {"minimum": 0}),
  };
}

export interface ProgramOperationsRemoval {
  readonly id: string;
  readonly index: number;
}

export function parseProgramOperationsRemoval(value: unknown, at = "$"): ProgramOperationsRemoval {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "index"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    index: architectProgramDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
  };
}



export function parseProgramOptionsDelta(value: unknown, at = "$"): ProgramOptionsDelta {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["removed", "inserted", "moved", "modified"]);
  return {
    removed: architectProgramDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => parseProgramOptionsRemoval(item, `${at}.removed[${index}]`)),
    inserted: architectProgramDiffGuardArray(row["inserted"], `${at}.inserted`).map((item, index) => parseProgramOptionsInsertion(item, `${at}.inserted[${index}]`)),
    moved: architectProgramDiffGuardArray(row["moved"], `${at}.moved`).map((item, index) => parseProgramOptionsRelocation(item, `${at}.moved[${index}]`)),
    modified: architectProgramDiffGuardArray(row["modified"], `${at}.modified`).map((item, index) => parseProgramOptionsPatchEntry(item, `${at}.modified[${index}]`)),
  };
}

export interface ProgramOptionsInsertion {
  readonly index: number;
  readonly row: import("../🟦️.ts").OptionEvaluation;
}

export function parseProgramOptionsInsertion(value: unknown, at = "$"): ProgramOptionsInsertion {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["index", "row"]);
  return {
    index: architectProgramDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
    row: parseOptionEvaluation(row["row"], `${at}.row`),
  };
}

export interface ProgramOptionsPatchEntry {
  readonly id: string;
  readonly patch: OptionEvaluationPatch;
}

export function parseProgramOptionsPatchEntry(value: unknown, at = "$"): ProgramOptionsPatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseOptionEvaluationPatch(row["patch"], `${at}.patch`),
  };
}

export interface ProgramOptionsRelocation {
  readonly id: string;
  readonly from: number;
  readonly to: number;
}

export function parseProgramOptionsRelocation(value: unknown, at = "$"): ProgramOptionsRelocation {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "from", "to"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    from: architectProgramDiffGuardInteger(row["from"], `${at}.from`, {"minimum": 0}),
    to: architectProgramDiffGuardInteger(row["to"], `${at}.to`, {"minimum": 0}),
  };
}

export interface ProgramOptionsRemoval {
  readonly id: string;
  readonly index: number;
}

export function parseProgramOptionsRemoval(value: unknown, at = "$"): ProgramOptionsRemoval {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "index"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    index: architectProgramDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
  };
}



export function parseProgramOrganizationalDelta(value: unknown, at = "$"): ProgramOrganizationalDelta {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["removed", "inserted", "moved", "modified"]);
  return {
    removed: architectProgramDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => parseProgramOrganizationalRemoval(item, `${at}.removed[${index}]`)),
    inserted: architectProgramDiffGuardArray(row["inserted"], `${at}.inserted`).map((item, index) => parseProgramOrganizationalInsertion(item, `${at}.inserted[${index}]`)),
    moved: architectProgramDiffGuardArray(row["moved"], `${at}.moved`).map((item, index) => parseProgramOrganizationalRelocation(item, `${at}.moved[${index}]`)),
    modified: architectProgramDiffGuardArray(row["modified"], `${at}.modified`).map((item, index) => parseProgramOrganizationalPatchEntry(item, `${at}.modified[${index}]`)),
  };
}

export interface ProgramOrganizationalInsertion {
  readonly index: number;
  readonly row: import("../🟦️.ts").OrganizationalRequirement;
}

export function parseProgramOrganizationalInsertion(value: unknown, at = "$"): ProgramOrganizationalInsertion {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["index", "row"]);
  return {
    index: architectProgramDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
    row: parseOrganizationalRequirement(row["row"], `${at}.row`),
  };
}

export interface ProgramOrganizationalPatchEntry {
  readonly id: string;
  readonly patch: OrganizationalRequirementPatch;
}

export function parseProgramOrganizationalPatchEntry(value: unknown, at = "$"): ProgramOrganizationalPatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseOrganizationalRequirementPatch(row["patch"], `${at}.patch`),
  };
}

export interface ProgramOrganizationalRelocation {
  readonly id: string;
  readonly from: number;
  readonly to: number;
}

export function parseProgramOrganizationalRelocation(value: unknown, at = "$"): ProgramOrganizationalRelocation {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "from", "to"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    from: architectProgramDiffGuardInteger(row["from"], `${at}.from`, {"minimum": 0}),
    to: architectProgramDiffGuardInteger(row["to"], `${at}.to`, {"minimum": 0}),
  };
}

export interface ProgramOrganizationalRemoval {
  readonly id: string;
  readonly index: number;
}

export function parseProgramOrganizationalRemoval(value: unknown, at = "$"): ProgramOrganizationalRemoval {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "index"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    index: architectProgramDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
  };
}



export function parseProgramPerformanceDelta(value: unknown, at = "$"): ProgramPerformanceDelta {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["removed", "inserted", "moved", "modified"]);
  return {
    removed: architectProgramDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => parseProgramPerformanceRemoval(item, `${at}.removed[${index}]`)),
    inserted: architectProgramDiffGuardArray(row["inserted"], `${at}.inserted`).map((item, index) => parseProgramPerformanceInsertion(item, `${at}.inserted[${index}]`)),
    moved: architectProgramDiffGuardArray(row["moved"], `${at}.moved`).map((item, index) => parseProgramPerformanceRelocation(item, `${at}.moved[${index}]`)),
    modified: architectProgramDiffGuardArray(row["modified"], `${at}.modified`).map((item, index) => parseProgramPerformancePatchEntry(item, `${at}.modified[${index}]`)),
  };
}

export interface ProgramPerformanceInsertion {
  readonly index: number;
  readonly row: import("../🟦️.ts").PerformanceCriterion;
}

export function parseProgramPerformanceInsertion(value: unknown, at = "$"): ProgramPerformanceInsertion {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["index", "row"]);
  return {
    index: architectProgramDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
    row: parsePerformanceCriterion(row["row"], `${at}.row`),
  };
}

export interface ProgramPerformancePatchEntry {
  readonly id: string;
  readonly patch: PerformanceCriterionPatch;
}

export function parseProgramPerformancePatchEntry(value: unknown, at = "$"): ProgramPerformancePatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parsePerformanceCriterionPatch(row["patch"], `${at}.patch`),
  };
}

export interface ProgramPerformanceRelocation {
  readonly id: string;
  readonly from: number;
  readonly to: number;
}

export function parseProgramPerformanceRelocation(value: unknown, at = "$"): ProgramPerformanceRelocation {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "from", "to"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    from: architectProgramDiffGuardInteger(row["from"], `${at}.from`, {"minimum": 0}),
    to: architectProgramDiffGuardInteger(row["to"], `${at}.to`, {"minimum": 0}),
  };
}

export interface ProgramPerformanceRemoval {
  readonly id: string;
  readonly index: number;
}

export function parseProgramPerformanceRemoval(value: unknown, at = "$"): ProgramPerformanceRemoval {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "index"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    index: architectProgramDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
  };
}



export function parseProgramPrioritiesDelta(value: unknown, at = "$"): ProgramPrioritiesDelta {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["removed", "inserted", "moved", "modified"]);
  return {
    removed: architectProgramDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => parseProgramPrioritiesRemoval(item, `${at}.removed[${index}]`)),
    inserted: architectProgramDiffGuardArray(row["inserted"], `${at}.inserted`).map((item, index) => parseProgramPrioritiesInsertion(item, `${at}.inserted[${index}]`)),
    moved: architectProgramDiffGuardArray(row["moved"], `${at}.moved`).map((item, index) => parseProgramPrioritiesRelocation(item, `${at}.moved[${index}]`)),
    modified: architectProgramDiffGuardArray(row["modified"], `${at}.modified`).map((item, index) => parseProgramPrioritiesPatchEntry(item, `${at}.modified[${index}]`)),
  };
}

export interface ProgramPrioritiesInsertion {
  readonly index: number;
  readonly row: import("../🟦️.ts").PriorityRecord;
}

export function parseProgramPrioritiesInsertion(value: unknown, at = "$"): ProgramPrioritiesInsertion {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["index", "row"]);
  return {
    index: architectProgramDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
    row: parsePriorityRecord(row["row"], `${at}.row`),
  };
}

export interface ProgramPrioritiesPatchEntry {
  readonly id: string;
  readonly patch: PriorityRecordPatch;
}

export function parseProgramPrioritiesPatchEntry(value: unknown, at = "$"): ProgramPrioritiesPatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parsePriorityRecordPatch(row["patch"], `${at}.patch`),
  };
}

export interface ProgramPrioritiesRelocation {
  readonly id: string;
  readonly from: number;
  readonly to: number;
}

export function parseProgramPrioritiesRelocation(value: unknown, at = "$"): ProgramPrioritiesRelocation {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "from", "to"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    from: architectProgramDiffGuardInteger(row["from"], `${at}.from`, {"minimum": 0}),
    to: architectProgramDiffGuardInteger(row["to"], `${at}.to`, {"minimum": 0}),
  };
}

export interface ProgramPrioritiesRemoval {
  readonly id: string;
  readonly index: number;
}

export function parseProgramPrioritiesRemoval(value: unknown, at = "$"): ProgramPrioritiesRemoval {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "index"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    index: architectProgramDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
  };
}



export function parseProgramPrivacyDelta(value: unknown, at = "$"): ProgramPrivacyDelta {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["removed", "inserted", "moved", "modified"]);
  return {
    removed: architectProgramDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => parseProgramPrivacyRemoval(item, `${at}.removed[${index}]`)),
    inserted: architectProgramDiffGuardArray(row["inserted"], `${at}.inserted`).map((item, index) => parseProgramPrivacyInsertion(item, `${at}.inserted[${index}]`)),
    moved: architectProgramDiffGuardArray(row["moved"], `${at}.moved`).map((item, index) => parseProgramPrivacyRelocation(item, `${at}.moved[${index}]`)),
    modified: architectProgramDiffGuardArray(row["modified"], `${at}.modified`).map((item, index) => parseProgramPrivacyPatchEntry(item, `${at}.modified[${index}]`)),
  };
}

export interface ProgramPrivacyInsertion {
  readonly index: number;
  readonly row: import("../🟦️.ts").PrivacyRequirement;
}

export function parseProgramPrivacyInsertion(value: unknown, at = "$"): ProgramPrivacyInsertion {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["index", "row"]);
  return {
    index: architectProgramDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
    row: parsePrivacyRequirement(row["row"], `${at}.row`),
  };
}

export interface ProgramPrivacyPatchEntry {
  readonly id: string;
  readonly patch: PrivacyRequirementPatch;
}

export function parseProgramPrivacyPatchEntry(value: unknown, at = "$"): ProgramPrivacyPatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parsePrivacyRequirementPatch(row["patch"], `${at}.patch`),
  };
}

export interface ProgramPrivacyRelocation {
  readonly id: string;
  readonly from: number;
  readonly to: number;
}

export function parseProgramPrivacyRelocation(value: unknown, at = "$"): ProgramPrivacyRelocation {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "from", "to"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    from: architectProgramDiffGuardInteger(row["from"], `${at}.from`, {"minimum": 0}),
    to: architectProgramDiffGuardInteger(row["to"], `${at}.to`, {"minimum": 0}),
  };
}

export interface ProgramPrivacyRemoval {
  readonly id: string;
  readonly index: number;
}

export function parseProgramPrivacyRemoval(value: unknown, at = "$"): ProgramPrivacyRemoval {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "index"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    index: architectProgramDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
  };
}



export function parseProgramProcessesDelta(value: unknown, at = "$"): ProgramProcessesDelta {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["removed", "inserted", "moved", "modified"]);
  return {
    removed: architectProgramDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => parseProgramProcessesRemoval(item, `${at}.removed[${index}]`)),
    inserted: architectProgramDiffGuardArray(row["inserted"], `${at}.inserted`).map((item, index) => parseProgramProcessesInsertion(item, `${at}.inserted[${index}]`)),
    moved: architectProgramDiffGuardArray(row["moved"], `${at}.moved`).map((item, index) => parseProgramProcessesRelocation(item, `${at}.moved[${index}]`)),
    modified: architectProgramDiffGuardArray(row["modified"], `${at}.modified`).map((item, index) => parseProgramProcessesPatchEntry(item, `${at}.modified[${index}]`)),
  };
}

export interface ProgramProcessesInsertion {
  readonly index: number;
  readonly row: import("../🟦️.ts").Process;
}

export function parseProgramProcessesInsertion(value: unknown, at = "$"): ProgramProcessesInsertion {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["index", "row"]);
  return {
    index: architectProgramDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
    row: parseProcess(row["row"], `${at}.row`),
  };
}

export interface ProgramProcessesPatchEntry {
  readonly id: string;
  readonly patch: ProcessPatch;
}

export function parseProgramProcessesPatchEntry(value: unknown, at = "$"): ProgramProcessesPatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseProcessPatch(row["patch"], `${at}.patch`),
  };
}

export interface ProgramProcessesRelocation {
  readonly id: string;
  readonly from: number;
  readonly to: number;
}

export function parseProgramProcessesRelocation(value: unknown, at = "$"): ProgramProcessesRelocation {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "from", "to"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    from: architectProgramDiffGuardInteger(row["from"], `${at}.from`, {"minimum": 0}),
    to: architectProgramDiffGuardInteger(row["to"], `${at}.to`, {"minimum": 0}),
  };
}

export interface ProgramProcessesRemoval {
  readonly id: string;
  readonly index: number;
}

export function parseProgramProcessesRemoval(value: unknown, at = "$"): ProgramProcessesRemoval {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "index"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    index: architectProgramDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
  };
}



export function parseProgramQualityDelta(value: unknown, at = "$"): ProgramQualityDelta {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["removed", "inserted", "moved", "modified"]);
  return {
    removed: architectProgramDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => parseProgramQualityRemoval(item, `${at}.removed[${index}]`)),
    inserted: architectProgramDiffGuardArray(row["inserted"], `${at}.inserted`).map((item, index) => parseProgramQualityInsertion(item, `${at}.inserted[${index}]`)),
    moved: architectProgramDiffGuardArray(row["moved"], `${at}.moved`).map((item, index) => parseProgramQualityRelocation(item, `${at}.moved[${index}]`)),
    modified: architectProgramDiffGuardArray(row["modified"], `${at}.modified`).map((item, index) => parseProgramQualityPatchEntry(item, `${at}.modified[${index}]`)),
  };
}

export interface ProgramQualityInsertion {
  readonly index: number;
  readonly row: import("../🟦️.ts").QualityRecord;
}

export function parseProgramQualityInsertion(value: unknown, at = "$"): ProgramQualityInsertion {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["index", "row"]);
  return {
    index: architectProgramDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
    row: parseQualityRecord(row["row"], `${at}.row`),
  };
}

export interface ProgramQualityPatchEntry {
  readonly id: string;
  readonly patch: QualityRecordPatch;
}

export function parseProgramQualityPatchEntry(value: unknown, at = "$"): ProgramQualityPatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseQualityRecordPatch(row["patch"], `${at}.patch`),
  };
}

export interface ProgramQualityRelocation {
  readonly id: string;
  readonly from: number;
  readonly to: number;
}

export function parseProgramQualityRelocation(value: unknown, at = "$"): ProgramQualityRelocation {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "from", "to"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    from: architectProgramDiffGuardInteger(row["from"], `${at}.from`, {"minimum": 0}),
    to: architectProgramDiffGuardInteger(row["to"], `${at}.to`, {"minimum": 0}),
  };
}

export interface ProgramQualityRemoval {
  readonly id: string;
  readonly index: number;
}

export function parseProgramQualityRemoval(value: unknown, at = "$"): ProgramQualityRemoval {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "index"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    index: architectProgramDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
  };
}



export function parseProgramQuantitiesDelta(value: unknown, at = "$"): ProgramQuantitiesDelta {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["removed", "inserted", "moved", "modified"]);
  return {
    removed: architectProgramDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => parseProgramQuantitiesRemoval(item, `${at}.removed[${index}]`)),
    inserted: architectProgramDiffGuardArray(row["inserted"], `${at}.inserted`).map((item, index) => parseProgramQuantitiesInsertion(item, `${at}.inserted[${index}]`)),
    moved: architectProgramDiffGuardArray(row["moved"], `${at}.moved`).map((item, index) => parseProgramQuantitiesRelocation(item, `${at}.moved[${index}]`)),
    modified: architectProgramDiffGuardArray(row["modified"], `${at}.modified`).map((item, index) => parseProgramQuantitiesPatchEntry(item, `${at}.modified[${index}]`)),
  };
}

export interface ProgramQuantitiesInsertion {
  readonly index: number;
  readonly row: import("../🟦️.ts").QuantityRequirement;
}

export function parseProgramQuantitiesInsertion(value: unknown, at = "$"): ProgramQuantitiesInsertion {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["index", "row"]);
  return {
    index: architectProgramDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
    row: parseQuantityRequirement(row["row"], `${at}.row`),
  };
}

export interface ProgramQuantitiesPatchEntry {
  readonly id: string;
  readonly patch: QuantityRequirementPatch;
}

export function parseProgramQuantitiesPatchEntry(value: unknown, at = "$"): ProgramQuantitiesPatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseQuantityRequirementPatch(row["patch"], `${at}.patch`),
  };
}

export interface ProgramQuantitiesRelocation {
  readonly id: string;
  readonly from: number;
  readonly to: number;
}

export function parseProgramQuantitiesRelocation(value: unknown, at = "$"): ProgramQuantitiesRelocation {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "from", "to"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    from: architectProgramDiffGuardInteger(row["from"], `${at}.from`, {"minimum": 0}),
    to: architectProgramDiffGuardInteger(row["to"], `${at}.to`, {"minimum": 0}),
  };
}

export interface ProgramQuantitiesRemoval {
  readonly id: string;
  readonly index: number;
}

export function parseProgramQuantitiesRemoval(value: unknown, at = "$"): ProgramQuantitiesRemoval {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "index"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    index: architectProgramDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
  };
}



export function parseProgramRegulatoryDelta(value: unknown, at = "$"): ProgramRegulatoryDelta {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["removed", "inserted", "moved", "modified"]);
  return {
    removed: architectProgramDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => parseProgramRegulatoryRemoval(item, `${at}.removed[${index}]`)),
    inserted: architectProgramDiffGuardArray(row["inserted"], `${at}.inserted`).map((item, index) => parseProgramRegulatoryInsertion(item, `${at}.inserted[${index}]`)),
    moved: architectProgramDiffGuardArray(row["moved"], `${at}.moved`).map((item, index) => parseProgramRegulatoryRelocation(item, `${at}.moved[${index}]`)),
    modified: architectProgramDiffGuardArray(row["modified"], `${at}.modified`).map((item, index) => parseProgramRegulatoryPatchEntry(item, `${at}.modified[${index}]`)),
  };
}

export interface ProgramRegulatoryInsertion {
  readonly index: number;
  readonly row: import("../🟦️.ts").RegulatoryRequirement;
}

export function parseProgramRegulatoryInsertion(value: unknown, at = "$"): ProgramRegulatoryInsertion {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["index", "row"]);
  return {
    index: architectProgramDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
    row: parseRegulatoryRequirement(row["row"], `${at}.row`),
  };
}

export interface ProgramRegulatoryPatchEntry {
  readonly id: string;
  readonly patch: RegulatoryRequirementPatch;
}

export function parseProgramRegulatoryPatchEntry(value: unknown, at = "$"): ProgramRegulatoryPatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseRegulatoryRequirementPatch(row["patch"], `${at}.patch`),
  };
}

export interface ProgramRegulatoryRelocation {
  readonly id: string;
  readonly from: number;
  readonly to: number;
}

export function parseProgramRegulatoryRelocation(value: unknown, at = "$"): ProgramRegulatoryRelocation {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "from", "to"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    from: architectProgramDiffGuardInteger(row["from"], `${at}.from`, {"minimum": 0}),
    to: architectProgramDiffGuardInteger(row["to"], `${at}.to`, {"minimum": 0}),
  };
}

export interface ProgramRegulatoryRemoval {
  readonly id: string;
  readonly index: number;
}

export function parseProgramRegulatoryRemoval(value: unknown, at = "$"): ProgramRegulatoryRemoval {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "index"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    index: architectProgramDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
  };
}



export function parseProgramRelationshipsDelta(value: unknown, at = "$"): ProgramRelationshipsDelta {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["removed", "inserted", "moved", "modified"]);
  return {
    removed: architectProgramDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => parseProgramRelationshipsRemoval(item, `${at}.removed[${index}]`)),
    inserted: architectProgramDiffGuardArray(row["inserted"], `${at}.inserted`).map((item, index) => parseProgramRelationshipsInsertion(item, `${at}.inserted[${index}]`)),
    moved: architectProgramDiffGuardArray(row["moved"], `${at}.moved`).map((item, index) => parseProgramRelationshipsRelocation(item, `${at}.moved[${index}]`)),
    modified: architectProgramDiffGuardArray(row["modified"], `${at}.modified`).map((item, index) => parseProgramRelationshipsPatchEntry(item, `${at}.modified[${index}]`)),
  };
}

export interface ProgramRelationshipsInsertion {
  readonly index: number;
  readonly row: import("../🟦️.ts").Relationship;
}

export function parseProgramRelationshipsInsertion(value: unknown, at = "$"): ProgramRelationshipsInsertion {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["index", "row"]);
  return {
    index: architectProgramDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
    row: parseRelationship(row["row"], `${at}.row`),
  };
}

export interface ProgramRelationshipsPatchEntry {
  readonly id: string;
  readonly patch: RelationshipPatch;
}

export function parseProgramRelationshipsPatchEntry(value: unknown, at = "$"): ProgramRelationshipsPatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseRelationshipPatch(row["patch"], `${at}.patch`),
  };
}

export interface ProgramRelationshipsRelocation {
  readonly id: string;
  readonly from: number;
  readonly to: number;
}

export function parseProgramRelationshipsRelocation(value: unknown, at = "$"): ProgramRelationshipsRelocation {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "from", "to"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    from: architectProgramDiffGuardInteger(row["from"], `${at}.from`, {"minimum": 0}),
    to: architectProgramDiffGuardInteger(row["to"], `${at}.to`, {"minimum": 0}),
  };
}

export interface ProgramRelationshipsRemoval {
  readonly id: string;
  readonly index: number;
}

export function parseProgramRelationshipsRemoval(value: unknown, at = "$"): ProgramRelationshipsRemoval {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "index"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    index: architectProgramDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
  };
}



export function parseProgramReportsDelta(value: unknown, at = "$"): ProgramReportsDelta {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["removed", "inserted", "moved", "modified"]);
  return {
    removed: architectProgramDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => parseProgramReportsRemoval(item, `${at}.removed[${index}]`)),
    inserted: architectProgramDiffGuardArray(row["inserted"], `${at}.inserted`).map((item, index) => parseProgramReportsInsertion(item, `${at}.inserted[${index}]`)),
    moved: architectProgramDiffGuardArray(row["moved"], `${at}.moved`).map((item, index) => parseProgramReportsRelocation(item, `${at}.moved[${index}]`)),
    modified: architectProgramDiffGuardArray(row["modified"], `${at}.modified`).map((item, index) => parseProgramReportsPatchEntry(item, `${at}.modified[${index}]`)),
  };
}

export interface ProgramReportsInsertion {
  readonly index: number;
  readonly row: import("../🟦️.ts").ReportRecord;
}

export function parseProgramReportsInsertion(value: unknown, at = "$"): ProgramReportsInsertion {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["index", "row"]);
  return {
    index: architectProgramDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
    row: parseReportRecord(row["row"], `${at}.row`),
  };
}

export interface ProgramReportsPatchEntry {
  readonly id: string;
  readonly patch: ReportRecordPatch;
}

export function parseProgramReportsPatchEntry(value: unknown, at = "$"): ProgramReportsPatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseReportRecordPatch(row["patch"], `${at}.patch`),
  };
}

export interface ProgramReportsRelocation {
  readonly id: string;
  readonly from: number;
  readonly to: number;
}

export function parseProgramReportsRelocation(value: unknown, at = "$"): ProgramReportsRelocation {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "from", "to"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    from: architectProgramDiffGuardInteger(row["from"], `${at}.from`, {"minimum": 0}),
    to: architectProgramDiffGuardInteger(row["to"], `${at}.to`, {"minimum": 0}),
  };
}

export interface ProgramReportsRemoval {
  readonly id: string;
  readonly index: number;
}

export function parseProgramReportsRemoval(value: unknown, at = "$"): ProgramReportsRemoval {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "index"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    index: architectProgramDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
  };
}



export function parseProgramRequirementsDelta(value: unknown, at = "$"): ProgramRequirementsDelta {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["removed", "inserted", "moved", "modified"]);
  return {
    removed: architectProgramDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => parseProgramRequirementsRemoval(item, `${at}.removed[${index}]`)),
    inserted: architectProgramDiffGuardArray(row["inserted"], `${at}.inserted`).map((item, index) => parseProgramRequirementsInsertion(item, `${at}.inserted[${index}]`)),
    moved: architectProgramDiffGuardArray(row["moved"], `${at}.moved`).map((item, index) => parseProgramRequirementsRelocation(item, `${at}.moved[${index}]`)),
    modified: architectProgramDiffGuardArray(row["modified"], `${at}.modified`).map((item, index) => parseProgramRequirementsPatchEntry(item, `${at}.modified[${index}]`)),
  };
}

export interface ProgramRequirementsInsertion {
  readonly index: number;
  readonly row: import("../🟦️.ts").Requirement;
}

export function parseProgramRequirementsInsertion(value: unknown, at = "$"): ProgramRequirementsInsertion {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["index", "row"]);
  return {
    index: architectProgramDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
    row: parseRequirement(row["row"], `${at}.row`),
  };
}

export interface ProgramRequirementsPatchEntry {
  readonly id: string;
  readonly patch: RequirementPatch;
}

export function parseProgramRequirementsPatchEntry(value: unknown, at = "$"): ProgramRequirementsPatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseRequirementPatch(row["patch"], `${at}.patch`),
  };
}

export interface ProgramRequirementsRelocation {
  readonly id: string;
  readonly from: number;
  readonly to: number;
}

export function parseProgramRequirementsRelocation(value: unknown, at = "$"): ProgramRequirementsRelocation {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "from", "to"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    from: architectProgramDiffGuardInteger(row["from"], `${at}.from`, {"minimum": 0}),
    to: architectProgramDiffGuardInteger(row["to"], `${at}.to`, {"minimum": 0}),
  };
}

export interface ProgramRequirementsRemoval {
  readonly id: string;
  readonly index: number;
}

export function parseProgramRequirementsRemoval(value: unknown, at = "$"): ProgramRequirementsRemoval {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "index"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    index: architectProgramDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
  };
}



export function parseProgramResilienceDelta(value: unknown, at = "$"): ProgramResilienceDelta {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["removed", "inserted", "moved", "modified"]);
  return {
    removed: architectProgramDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => parseProgramResilienceRemoval(item, `${at}.removed[${index}]`)),
    inserted: architectProgramDiffGuardArray(row["inserted"], `${at}.inserted`).map((item, index) => parseProgramResilienceInsertion(item, `${at}.inserted[${index}]`)),
    moved: architectProgramDiffGuardArray(row["moved"], `${at}.moved`).map((item, index) => parseProgramResilienceRelocation(item, `${at}.moved[${index}]`)),
    modified: architectProgramDiffGuardArray(row["modified"], `${at}.modified`).map((item, index) => parseProgramResiliencePatchEntry(item, `${at}.modified[${index}]`)),
  };
}

export interface ProgramResilienceInsertion {
  readonly index: number;
  readonly row: import("../🟦️.ts").ResilienceRequirement;
}

export function parseProgramResilienceInsertion(value: unknown, at = "$"): ProgramResilienceInsertion {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["index", "row"]);
  return {
    index: architectProgramDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
    row: parseResilienceRequirement(row["row"], `${at}.row`),
  };
}

export interface ProgramResiliencePatchEntry {
  readonly id: string;
  readonly patch: ResilienceRequirementPatch;
}

export function parseProgramResiliencePatchEntry(value: unknown, at = "$"): ProgramResiliencePatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseResilienceRequirementPatch(row["patch"], `${at}.patch`),
  };
}

export interface ProgramResilienceRelocation {
  readonly id: string;
  readonly from: number;
  readonly to: number;
}

export function parseProgramResilienceRelocation(value: unknown, at = "$"): ProgramResilienceRelocation {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "from", "to"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    from: architectProgramDiffGuardInteger(row["from"], `${at}.from`, {"minimum": 0}),
    to: architectProgramDiffGuardInteger(row["to"], `${at}.to`, {"minimum": 0}),
  };
}

export interface ProgramResilienceRemoval {
  readonly id: string;
  readonly index: number;
}

export function parseProgramResilienceRemoval(value: unknown, at = "$"): ProgramResilienceRemoval {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "index"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    index: architectProgramDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
  };
}



export function parseProgramResourcesDelta(value: unknown, at = "$"): ProgramResourcesDelta {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["removed", "inserted", "moved", "modified"]);
  return {
    removed: architectProgramDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => parseProgramResourcesRemoval(item, `${at}.removed[${index}]`)),
    inserted: architectProgramDiffGuardArray(row["inserted"], `${at}.inserted`).map((item, index) => parseProgramResourcesInsertion(item, `${at}.inserted[${index}]`)),
    moved: architectProgramDiffGuardArray(row["moved"], `${at}.moved`).map((item, index) => parseProgramResourcesRelocation(item, `${at}.moved[${index}]`)),
    modified: architectProgramDiffGuardArray(row["modified"], `${at}.modified`).map((item, index) => parseProgramResourcesPatchEntry(item, `${at}.modified[${index}]`)),
  };
}

export interface ProgramResourcesInsertion {
  readonly index: number;
  readonly row: import("../🟦️.ts").Resource;
}

export function parseProgramResourcesInsertion(value: unknown, at = "$"): ProgramResourcesInsertion {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["index", "row"]);
  return {
    index: architectProgramDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
    row: parseResource(row["row"], `${at}.row`),
  };
}

export interface ProgramResourcesPatchEntry {
  readonly id: string;
  readonly patch: ResourcePatch;
}

export function parseProgramResourcesPatchEntry(value: unknown, at = "$"): ProgramResourcesPatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseResourcePatch(row["patch"], `${at}.patch`),
  };
}

export interface ProgramResourcesRelocation {
  readonly id: string;
  readonly from: number;
  readonly to: number;
}

export function parseProgramResourcesRelocation(value: unknown, at = "$"): ProgramResourcesRelocation {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "from", "to"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    from: architectProgramDiffGuardInteger(row["from"], `${at}.from`, {"minimum": 0}),
    to: architectProgramDiffGuardInteger(row["to"], `${at}.to`, {"minimum": 0}),
  };
}

export interface ProgramResourcesRemoval {
  readonly id: string;
  readonly index: number;
}

export function parseProgramResourcesRemoval(value: unknown, at = "$"): ProgramResourcesRemoval {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "index"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    index: architectProgramDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
  };
}



export function parseProgramRisksDelta(value: unknown, at = "$"): ProgramRisksDelta {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["removed", "inserted", "moved", "modified"]);
  return {
    removed: architectProgramDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => parseProgramRisksRemoval(item, `${at}.removed[${index}]`)),
    inserted: architectProgramDiffGuardArray(row["inserted"], `${at}.inserted`).map((item, index) => parseProgramRisksInsertion(item, `${at}.inserted[${index}]`)),
    moved: architectProgramDiffGuardArray(row["moved"], `${at}.moved`).map((item, index) => parseProgramRisksRelocation(item, `${at}.moved[${index}]`)),
    modified: architectProgramDiffGuardArray(row["modified"], `${at}.modified`).map((item, index) => parseProgramRisksPatchEntry(item, `${at}.modified[${index}]`)),
  };
}

export interface ProgramRisksInsertion {
  readonly index: number;
  readonly row: import("../🟦️.ts").Risk;
}

export function parseProgramRisksInsertion(value: unknown, at = "$"): ProgramRisksInsertion {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["index", "row"]);
  return {
    index: architectProgramDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
    row: parseRisk(row["row"], `${at}.row`),
  };
}

export interface ProgramRisksPatchEntry {
  readonly id: string;
  readonly patch: RiskPatch;
}

export function parseProgramRisksPatchEntry(value: unknown, at = "$"): ProgramRisksPatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseRiskPatch(row["patch"], `${at}.patch`),
  };
}

export interface ProgramRisksRelocation {
  readonly id: string;
  readonly from: number;
  readonly to: number;
}

export function parseProgramRisksRelocation(value: unknown, at = "$"): ProgramRisksRelocation {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "from", "to"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    from: architectProgramDiffGuardInteger(row["from"], `${at}.from`, {"minimum": 0}),
    to: architectProgramDiffGuardInteger(row["to"], `${at}.to`, {"minimum": 0}),
  };
}

export interface ProgramRisksRemoval {
  readonly id: string;
  readonly index: number;
}

export function parseProgramRisksRemoval(value: unknown, at = "$"): ProgramRisksRemoval {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "index"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    index: architectProgramDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
  };
}



export function parseProgramSafetyDelta(value: unknown, at = "$"): ProgramSafetyDelta {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["removed", "inserted", "moved", "modified"]);
  return {
    removed: architectProgramDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => parseProgramSafetyRemoval(item, `${at}.removed[${index}]`)),
    inserted: architectProgramDiffGuardArray(row["inserted"], `${at}.inserted`).map((item, index) => parseProgramSafetyInsertion(item, `${at}.inserted[${index}]`)),
    moved: architectProgramDiffGuardArray(row["moved"], `${at}.moved`).map((item, index) => parseProgramSafetyRelocation(item, `${at}.moved[${index}]`)),
    modified: architectProgramDiffGuardArray(row["modified"], `${at}.modified`).map((item, index) => parseProgramSafetyPatchEntry(item, `${at}.modified[${index}]`)),
  };
}

export interface ProgramSafetyInsertion {
  readonly index: number;
  readonly row: import("../🟦️.ts").SafetyRequirement;
}

export function parseProgramSafetyInsertion(value: unknown, at = "$"): ProgramSafetyInsertion {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["index", "row"]);
  return {
    index: architectProgramDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
    row: parseSafetyRequirement(row["row"], `${at}.row`),
  };
}

export interface ProgramSafetyPatchEntry {
  readonly id: string;
  readonly patch: SafetyRequirementPatch;
}

export function parseProgramSafetyPatchEntry(value: unknown, at = "$"): ProgramSafetyPatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseSafetyRequirementPatch(row["patch"], `${at}.patch`),
  };
}

export interface ProgramSafetyRelocation {
  readonly id: string;
  readonly from: number;
  readonly to: number;
}

export function parseProgramSafetyRelocation(value: unknown, at = "$"): ProgramSafetyRelocation {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "from", "to"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    from: architectProgramDiffGuardInteger(row["from"], `${at}.from`, {"minimum": 0}),
    to: architectProgramDiffGuardInteger(row["to"], `${at}.to`, {"minimum": 0}),
  };
}

export interface ProgramSafetyRemoval {
  readonly id: string;
  readonly index: number;
}

export function parseProgramSafetyRemoval(value: unknown, at = "$"): ProgramSafetyRemoval {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "index"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    index: architectProgramDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
  };
}



export function parseProgramScenariosDelta(value: unknown, at = "$"): ProgramScenariosDelta {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["removed", "inserted", "moved", "modified"]);
  return {
    removed: architectProgramDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => parseProgramScenariosRemoval(item, `${at}.removed[${index}]`)),
    inserted: architectProgramDiffGuardArray(row["inserted"], `${at}.inserted`).map((item, index) => parseProgramScenariosInsertion(item, `${at}.inserted[${index}]`)),
    moved: architectProgramDiffGuardArray(row["moved"], `${at}.moved`).map((item, index) => parseProgramScenariosRelocation(item, `${at}.moved[${index}]`)),
    modified: architectProgramDiffGuardArray(row["modified"], `${at}.modified`).map((item, index) => parseProgramScenariosPatchEntry(item, `${at}.modified[${index}]`)),
  };
}

export interface ProgramScenariosInsertion {
  readonly index: number;
  readonly row: import("../🟦️.ts").Scenario;
}

export function parseProgramScenariosInsertion(value: unknown, at = "$"): ProgramScenariosInsertion {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["index", "row"]);
  return {
    index: architectProgramDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
    row: parseScenario(row["row"], `${at}.row`),
  };
}

export interface ProgramScenariosPatchEntry {
  readonly id: string;
  readonly patch: ScenarioPatch;
}

export function parseProgramScenariosPatchEntry(value: unknown, at = "$"): ProgramScenariosPatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseScenarioPatch(row["patch"], `${at}.patch`),
  };
}

export interface ProgramScenariosRelocation {
  readonly id: string;
  readonly from: number;
  readonly to: number;
}

export function parseProgramScenariosRelocation(value: unknown, at = "$"): ProgramScenariosRelocation {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "from", "to"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    from: architectProgramDiffGuardInteger(row["from"], `${at}.from`, {"minimum": 0}),
    to: architectProgramDiffGuardInteger(row["to"], `${at}.to`, {"minimum": 0}),
  };
}

export interface ProgramScenariosRemoval {
  readonly id: string;
  readonly index: number;
}

export function parseProgramScenariosRemoval(value: unknown, at = "$"): ProgramScenariosRemoval {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "index"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    index: architectProgramDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
  };
}



export function parseProgramSchedulesDelta(value: unknown, at = "$"): ProgramSchedulesDelta {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["removed", "inserted", "moved", "modified"]);
  return {
    removed: architectProgramDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => parseProgramSchedulesRemoval(item, `${at}.removed[${index}]`)),
    inserted: architectProgramDiffGuardArray(row["inserted"], `${at}.inserted`).map((item, index) => parseProgramSchedulesInsertion(item, `${at}.inserted[${index}]`)),
    moved: architectProgramDiffGuardArray(row["moved"], `${at}.moved`).map((item, index) => parseProgramSchedulesRelocation(item, `${at}.moved[${index}]`)),
    modified: architectProgramDiffGuardArray(row["modified"], `${at}.modified`).map((item, index) => parseProgramSchedulesPatchEntry(item, `${at}.modified[${index}]`)),
  };
}

export interface ProgramSchedulesInsertion {
  readonly index: number;
  readonly row: import("../🟦️.ts").ScheduleRequirement;
}

export function parseProgramSchedulesInsertion(value: unknown, at = "$"): ProgramSchedulesInsertion {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["index", "row"]);
  return {
    index: architectProgramDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
    row: parseScheduleRequirement(row["row"], `${at}.row`),
  };
}

export interface ProgramSchedulesPatchEntry {
  readonly id: string;
  readonly patch: ScheduleRequirementPatch;
}

export function parseProgramSchedulesPatchEntry(value: unknown, at = "$"): ProgramSchedulesPatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseScheduleRequirementPatch(row["patch"], `${at}.patch`),
  };
}

export interface ProgramSchedulesRelocation {
  readonly id: string;
  readonly from: number;
  readonly to: number;
}

export function parseProgramSchedulesRelocation(value: unknown, at = "$"): ProgramSchedulesRelocation {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "from", "to"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    from: architectProgramDiffGuardInteger(row["from"], `${at}.from`, {"minimum": 0}),
    to: architectProgramDiffGuardInteger(row["to"], `${at}.to`, {"minimum": 0}),
  };
}

export interface ProgramSchedulesRemoval {
  readonly id: string;
  readonly index: number;
}

export function parseProgramSchedulesRemoval(value: unknown, at = "$"): ProgramSchedulesRemoval {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "index"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    index: architectProgramDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
  };
}



export function parseProgramSearchFiltersDelta(value: unknown, at = "$"): ProgramSearchFiltersDelta {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["removed", "inserted", "moved", "modified"]);
  return {
    removed: architectProgramDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => parseProgramSearchFiltersRemoval(item, `${at}.removed[${index}]`)),
    inserted: architectProgramDiffGuardArray(row["inserted"], `${at}.inserted`).map((item, index) => parseProgramSearchFiltersInsertion(item, `${at}.inserted[${index}]`)),
    moved: architectProgramDiffGuardArray(row["moved"], `${at}.moved`).map((item, index) => parseProgramSearchFiltersRelocation(item, `${at}.moved[${index}]`)),
    modified: architectProgramDiffGuardArray(row["modified"], `${at}.modified`).map((item, index) => parseProgramSearchFiltersPatchEntry(item, `${at}.modified[${index}]`)),
  };
}

export interface ProgramSearchFiltersInsertion {
  readonly index: number;
  readonly row: import("../🟦️.ts").SearchFilter;
}

export function parseProgramSearchFiltersInsertion(value: unknown, at = "$"): ProgramSearchFiltersInsertion {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["index", "row"]);
  return {
    index: architectProgramDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
    row: parseSearchFilter(row["row"], `${at}.row`),
  };
}

export interface ProgramSearchFiltersPatchEntry {
  readonly id: string;
  readonly patch: SearchFilterPatch;
}

export function parseProgramSearchFiltersPatchEntry(value: unknown, at = "$"): ProgramSearchFiltersPatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseSearchFilterPatch(row["patch"], `${at}.patch`),
  };
}

export interface ProgramSearchFiltersRelocation {
  readonly id: string;
  readonly from: number;
  readonly to: number;
}

export function parseProgramSearchFiltersRelocation(value: unknown, at = "$"): ProgramSearchFiltersRelocation {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "from", "to"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    from: architectProgramDiffGuardInteger(row["from"], `${at}.from`, {"minimum": 0}),
    to: architectProgramDiffGuardInteger(row["to"], `${at}.to`, {"minimum": 0}),
  };
}

export interface ProgramSearchFiltersRemoval {
  readonly id: string;
  readonly index: number;
}

export function parseProgramSearchFiltersRemoval(value: unknown, at = "$"): ProgramSearchFiltersRemoval {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "index"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    index: architectProgramDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
  };
}



export function parseProgramSecurityDelta(value: unknown, at = "$"): ProgramSecurityDelta {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["removed", "inserted", "moved", "modified"]);
  return {
    removed: architectProgramDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => parseProgramSecurityRemoval(item, `${at}.removed[${index}]`)),
    inserted: architectProgramDiffGuardArray(row["inserted"], `${at}.inserted`).map((item, index) => parseProgramSecurityInsertion(item, `${at}.inserted[${index}]`)),
    moved: architectProgramDiffGuardArray(row["moved"], `${at}.moved`).map((item, index) => parseProgramSecurityRelocation(item, `${at}.moved[${index}]`)),
    modified: architectProgramDiffGuardArray(row["modified"], `${at}.modified`).map((item, index) => parseProgramSecurityPatchEntry(item, `${at}.modified[${index}]`)),
  };
}

export interface ProgramSecurityInsertion {
  readonly index: number;
  readonly row: import("../🟦️.ts").SecurityRequirement;
}

export function parseProgramSecurityInsertion(value: unknown, at = "$"): ProgramSecurityInsertion {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["index", "row"]);
  return {
    index: architectProgramDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
    row: parseSecurityRequirement(row["row"], `${at}.row`),
  };
}

export interface ProgramSecurityPatchEntry {
  readonly id: string;
  readonly patch: SecurityRequirementPatch;
}

export function parseProgramSecurityPatchEntry(value: unknown, at = "$"): ProgramSecurityPatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseSecurityRequirementPatch(row["patch"], `${at}.patch`),
  };
}

export interface ProgramSecurityRelocation {
  readonly id: string;
  readonly from: number;
  readonly to: number;
}

export function parseProgramSecurityRelocation(value: unknown, at = "$"): ProgramSecurityRelocation {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "from", "to"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    from: architectProgramDiffGuardInteger(row["from"], `${at}.from`, {"minimum": 0}),
    to: architectProgramDiffGuardInteger(row["to"], `${at}.to`, {"minimum": 0}),
  };
}

export interface ProgramSecurityRemoval {
  readonly id: string;
  readonly index: number;
}

export function parseProgramSecurityRemoval(value: unknown, at = "$"): ProgramSecurityRemoval {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "index"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    index: architectProgramDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
  };
}



export function parseProgramServicesDelta(value: unknown, at = "$"): ProgramServicesDelta {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["removed", "inserted", "moved", "modified"]);
  return {
    removed: architectProgramDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => parseProgramServicesRemoval(item, `${at}.removed[${index}]`)),
    inserted: architectProgramDiffGuardArray(row["inserted"], `${at}.inserted`).map((item, index) => parseProgramServicesInsertion(item, `${at}.inserted[${index}]`)),
    moved: architectProgramDiffGuardArray(row["moved"], `${at}.moved`).map((item, index) => parseProgramServicesRelocation(item, `${at}.moved[${index}]`)),
    modified: architectProgramDiffGuardArray(row["modified"], `${at}.modified`).map((item, index) => parseProgramServicesPatchEntry(item, `${at}.modified[${index}]`)),
  };
}

export interface ProgramServicesInsertion {
  readonly index: number;
  readonly row: import("../🟦️.ts").ServiceRequirement;
}

export function parseProgramServicesInsertion(value: unknown, at = "$"): ProgramServicesInsertion {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["index", "row"]);
  return {
    index: architectProgramDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
    row: parseServiceRequirement(row["row"], `${at}.row`),
  };
}

export interface ProgramServicesPatchEntry {
  readonly id: string;
  readonly patch: ServiceRequirementPatch;
}

export function parseProgramServicesPatchEntry(value: unknown, at = "$"): ProgramServicesPatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseServiceRequirementPatch(row["patch"], `${at}.patch`),
  };
}

export interface ProgramServicesRelocation {
  readonly id: string;
  readonly from: number;
  readonly to: number;
}

export function parseProgramServicesRelocation(value: unknown, at = "$"): ProgramServicesRelocation {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "from", "to"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    from: architectProgramDiffGuardInteger(row["from"], `${at}.from`, {"minimum": 0}),
    to: architectProgramDiffGuardInteger(row["to"], `${at}.to`, {"minimum": 0}),
  };
}

export interface ProgramServicesRemoval {
  readonly id: string;
  readonly index: number;
}

export function parseProgramServicesRemoval(value: unknown, at = "$"): ProgramServicesRemoval {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "index"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    index: architectProgramDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
  };
}



export function parseProgramSiteContextDelta(value: unknown, at = "$"): ProgramSiteContextDelta {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["removed", "inserted", "moved", "modified"]);
  return {
    removed: architectProgramDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => parseProgramSiteContextRemoval(item, `${at}.removed[${index}]`)),
    inserted: architectProgramDiffGuardArray(row["inserted"], `${at}.inserted`).map((item, index) => parseProgramSiteContextInsertion(item, `${at}.inserted[${index}]`)),
    moved: architectProgramDiffGuardArray(row["moved"], `${at}.moved`).map((item, index) => parseProgramSiteContextRelocation(item, `${at}.moved[${index}]`)),
    modified: architectProgramDiffGuardArray(row["modified"], `${at}.modified`).map((item, index) => parseProgramSiteContextPatchEntry(item, `${at}.modified[${index}]`)),
  };
}

export interface ProgramSiteContextInsertion {
  readonly index: number;
  readonly row: import("../🟦️.ts").SiteContext;
}

export function parseProgramSiteContextInsertion(value: unknown, at = "$"): ProgramSiteContextInsertion {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["index", "row"]);
  return {
    index: architectProgramDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
    row: parseSiteContext(row["row"], `${at}.row`),
  };
}

export interface ProgramSiteContextPatchEntry {
  readonly id: string;
  readonly patch: SiteContextPatch;
}

export function parseProgramSiteContextPatchEntry(value: unknown, at = "$"): ProgramSiteContextPatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseSiteContextPatch(row["patch"], `${at}.patch`),
  };
}

export interface ProgramSiteContextRelocation {
  readonly id: string;
  readonly from: number;
  readonly to: number;
}

export function parseProgramSiteContextRelocation(value: unknown, at = "$"): ProgramSiteContextRelocation {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "from", "to"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    from: architectProgramDiffGuardInteger(row["from"], `${at}.from`, {"minimum": 0}),
    to: architectProgramDiffGuardInteger(row["to"], `${at}.to`, {"minimum": 0}),
  };
}

export interface ProgramSiteContextRemoval {
  readonly id: string;
  readonly index: number;
}

export function parseProgramSiteContextRemoval(value: unknown, at = "$"): ProgramSiteContextRemoval {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "index"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    index: architectProgramDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
  };
}



export function parseProgramStakeholdersDelta(value: unknown, at = "$"): ProgramStakeholdersDelta {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["removed", "inserted", "moved", "modified"]);
  return {
    removed: architectProgramDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => parseProgramStakeholdersRemoval(item, `${at}.removed[${index}]`)),
    inserted: architectProgramDiffGuardArray(row["inserted"], `${at}.inserted`).map((item, index) => parseProgramStakeholdersInsertion(item, `${at}.inserted[${index}]`)),
    moved: architectProgramDiffGuardArray(row["moved"], `${at}.moved`).map((item, index) => parseProgramStakeholdersRelocation(item, `${at}.moved[${index}]`)),
    modified: architectProgramDiffGuardArray(row["modified"], `${at}.modified`).map((item, index) => parseProgramStakeholdersPatchEntry(item, `${at}.modified[${index}]`)),
  };
}

export interface ProgramStakeholdersInsertion {
  readonly index: number;
  readonly row: import("../🟦️.ts").Stakeholder;
}

export function parseProgramStakeholdersInsertion(value: unknown, at = "$"): ProgramStakeholdersInsertion {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["index", "row"]);
  return {
    index: architectProgramDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
    row: parseStakeholder(row["row"], `${at}.row`),
  };
}

export interface ProgramStakeholdersPatchEntry {
  readonly id: string;
  readonly patch: StakeholderPatch;
}

export function parseProgramStakeholdersPatchEntry(value: unknown, at = "$"): ProgramStakeholdersPatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseStakeholderPatch(row["patch"], `${at}.patch`),
  };
}

export interface ProgramStakeholdersRelocation {
  readonly id: string;
  readonly from: number;
  readonly to: number;
}

export function parseProgramStakeholdersRelocation(value: unknown, at = "$"): ProgramStakeholdersRelocation {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "from", "to"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    from: architectProgramDiffGuardInteger(row["from"], `${at}.from`, {"minimum": 0}),
    to: architectProgramDiffGuardInteger(row["to"], `${at}.to`, {"minimum": 0}),
  };
}

export interface ProgramStakeholdersRemoval {
  readonly id: string;
  readonly index: number;
}

export function parseProgramStakeholdersRemoval(value: unknown, at = "$"): ProgramStakeholdersRemoval {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "index"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    index: architectProgramDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
  };
}



export function parseProgramStatusRecordsDelta(value: unknown, at = "$"): ProgramStatusRecordsDelta {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["removed", "inserted", "moved", "modified"]);
  return {
    removed: architectProgramDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => parseProgramStatusRecordsRemoval(item, `${at}.removed[${index}]`)),
    inserted: architectProgramDiffGuardArray(row["inserted"], `${at}.inserted`).map((item, index) => parseProgramStatusRecordsInsertion(item, `${at}.inserted[${index}]`)),
    moved: architectProgramDiffGuardArray(row["moved"], `${at}.moved`).map((item, index) => parseProgramStatusRecordsRelocation(item, `${at}.moved[${index}]`)),
    modified: architectProgramDiffGuardArray(row["modified"], `${at}.modified`).map((item, index) => parseProgramStatusRecordsPatchEntry(item, `${at}.modified[${index}]`)),
  };
}

export interface ProgramStatusRecordsInsertion {
  readonly index: number;
  readonly row: import("../🟦️.ts").StatusRecord;
}

export function parseProgramStatusRecordsInsertion(value: unknown, at = "$"): ProgramStatusRecordsInsertion {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["index", "row"]);
  return {
    index: architectProgramDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
    row: parseStatusRecord(row["row"], `${at}.row`),
  };
}

export interface ProgramStatusRecordsPatchEntry {
  readonly id: string;
  readonly patch: StatusRecordPatch;
}

export function parseProgramStatusRecordsPatchEntry(value: unknown, at = "$"): ProgramStatusRecordsPatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseStatusRecordPatch(row["patch"], `${at}.patch`),
  };
}

export interface ProgramStatusRecordsRelocation {
  readonly id: string;
  readonly from: number;
  readonly to: number;
}

export function parseProgramStatusRecordsRelocation(value: unknown, at = "$"): ProgramStatusRecordsRelocation {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "from", "to"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    from: architectProgramDiffGuardInteger(row["from"], `${at}.from`, {"minimum": 0}),
    to: architectProgramDiffGuardInteger(row["to"], `${at}.to`, {"minimum": 0}),
  };
}

export interface ProgramStatusRecordsRemoval {
  readonly id: string;
  readonly index: number;
}

export function parseProgramStatusRecordsRemoval(value: unknown, at = "$"): ProgramStatusRecordsRemoval {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "index"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    index: architectProgramDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
  };
}



export function parseProgramStorageDelta(value: unknown, at = "$"): ProgramStorageDelta {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["removed", "inserted", "moved", "modified"]);
  return {
    removed: architectProgramDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => parseProgramStorageRemoval(item, `${at}.removed[${index}]`)),
    inserted: architectProgramDiffGuardArray(row["inserted"], `${at}.inserted`).map((item, index) => parseProgramStorageInsertion(item, `${at}.inserted[${index}]`)),
    moved: architectProgramDiffGuardArray(row["moved"], `${at}.moved`).map((item, index) => parseProgramStorageRelocation(item, `${at}.moved[${index}]`)),
    modified: architectProgramDiffGuardArray(row["modified"], `${at}.modified`).map((item, index) => parseProgramStoragePatchEntry(item, `${at}.modified[${index}]`)),
  };
}

export interface ProgramStorageInsertion {
  readonly index: number;
  readonly row: import("../🟦️.ts").StorageRequirement;
}

export function parseProgramStorageInsertion(value: unknown, at = "$"): ProgramStorageInsertion {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["index", "row"]);
  return {
    index: architectProgramDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
    row: parseStorageRequirement(row["row"], `${at}.row`),
  };
}

export interface ProgramStoragePatchEntry {
  readonly id: string;
  readonly patch: StorageRequirementPatch;
}

export function parseProgramStoragePatchEntry(value: unknown, at = "$"): ProgramStoragePatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseStorageRequirementPatch(row["patch"], `${at}.patch`),
  };
}

export interface ProgramStorageRelocation {
  readonly id: string;
  readonly from: number;
  readonly to: number;
}

export function parseProgramStorageRelocation(value: unknown, at = "$"): ProgramStorageRelocation {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "from", "to"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    from: architectProgramDiffGuardInteger(row["from"], `${at}.from`, {"minimum": 0}),
    to: architectProgramDiffGuardInteger(row["to"], `${at}.to`, {"minimum": 0}),
  };
}

export interface ProgramStorageRemoval {
  readonly id: string;
  readonly index: number;
}

export function parseProgramStorageRemoval(value: unknown, at = "$"): ProgramStorageRemoval {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "index"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    index: architectProgramDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
  };
}



export function parseProgramSurveysDelta(value: unknown, at = "$"): ProgramSurveysDelta {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["removed", "inserted", "moved", "modified"]);
  return {
    removed: architectProgramDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => parseProgramSurveysRemoval(item, `${at}.removed[${index}]`)),
    inserted: architectProgramDiffGuardArray(row["inserted"], `${at}.inserted`).map((item, index) => parseProgramSurveysInsertion(item, `${at}.inserted[${index}]`)),
    moved: architectProgramDiffGuardArray(row["moved"], `${at}.moved`).map((item, index) => parseProgramSurveysRelocation(item, `${at}.moved[${index}]`)),
    modified: architectProgramDiffGuardArray(row["modified"], `${at}.modified`).map((item, index) => parseProgramSurveysPatchEntry(item, `${at}.modified[${index}]`)),
  };
}

export interface ProgramSurveysInsertion {
  readonly index: number;
  readonly row: import("../🟦️.ts").Survey;
}

export function parseProgramSurveysInsertion(value: unknown, at = "$"): ProgramSurveysInsertion {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["index", "row"]);
  return {
    index: architectProgramDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
    row: parseSurvey(row["row"], `${at}.row`),
  };
}

export interface ProgramSurveysPatchEntry {
  readonly id: string;
  readonly patch: SurveyPatch;
}

export function parseProgramSurveysPatchEntry(value: unknown, at = "$"): ProgramSurveysPatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseSurveyPatch(row["patch"], `${at}.patch`),
  };
}

export interface ProgramSurveysRelocation {
  readonly id: string;
  readonly from: number;
  readonly to: number;
}

export function parseProgramSurveysRelocation(value: unknown, at = "$"): ProgramSurveysRelocation {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "from", "to"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    from: architectProgramDiffGuardInteger(row["from"], `${at}.from`, {"minimum": 0}),
    to: architectProgramDiffGuardInteger(row["to"], `${at}.to`, {"minimum": 0}),
  };
}

export interface ProgramSurveysRemoval {
  readonly id: string;
  readonly index: number;
}

export function parseProgramSurveysRemoval(value: unknown, at = "$"): ProgramSurveysRemoval {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "index"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    index: architectProgramDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
  };
}



export function parseProgramSustainabilityDelta(value: unknown, at = "$"): ProgramSustainabilityDelta {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["removed", "inserted", "moved", "modified"]);
  return {
    removed: architectProgramDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => parseProgramSustainabilityRemoval(item, `${at}.removed[${index}]`)),
    inserted: architectProgramDiffGuardArray(row["inserted"], `${at}.inserted`).map((item, index) => parseProgramSustainabilityInsertion(item, `${at}.inserted[${index}]`)),
    moved: architectProgramDiffGuardArray(row["moved"], `${at}.moved`).map((item, index) => parseProgramSustainabilityRelocation(item, `${at}.moved[${index}]`)),
    modified: architectProgramDiffGuardArray(row["modified"], `${at}.modified`).map((item, index) => parseProgramSustainabilityPatchEntry(item, `${at}.modified[${index}]`)),
  };
}

export interface ProgramSustainabilityInsertion {
  readonly index: number;
  readonly row: import("../🟦️.ts").SustainabilityRequirement;
}

export function parseProgramSustainabilityInsertion(value: unknown, at = "$"): ProgramSustainabilityInsertion {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["index", "row"]);
  return {
    index: architectProgramDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
    row: parseSustainabilityRequirement(row["row"], `${at}.row`),
  };
}

export interface ProgramSustainabilityPatchEntry {
  readonly id: string;
  readonly patch: SustainabilityRequirementPatch;
}

export function parseProgramSustainabilityPatchEntry(value: unknown, at = "$"): ProgramSustainabilityPatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseSustainabilityRequirementPatch(row["patch"], `${at}.patch`),
  };
}

export interface ProgramSustainabilityRelocation {
  readonly id: string;
  readonly from: number;
  readonly to: number;
}

export function parseProgramSustainabilityRelocation(value: unknown, at = "$"): ProgramSustainabilityRelocation {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "from", "to"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    from: architectProgramDiffGuardInteger(row["from"], `${at}.from`, {"minimum": 0}),
    to: architectProgramDiffGuardInteger(row["to"], `${at}.to`, {"minimum": 0}),
  };
}

export interface ProgramSustainabilityRemoval {
  readonly id: string;
  readonly index: number;
}

export function parseProgramSustainabilityRemoval(value: unknown, at = "$"): ProgramSustainabilityRemoval {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "index"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    index: architectProgramDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
  };
}



export function parseProgramTemplatesDelta(value: unknown, at = "$"): ProgramTemplatesDelta {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["removed", "inserted", "moved", "modified"]);
  return {
    removed: architectProgramDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => parseProgramTemplatesRemoval(item, `${at}.removed[${index}]`)),
    inserted: architectProgramDiffGuardArray(row["inserted"], `${at}.inserted`).map((item, index) => parseProgramTemplatesInsertion(item, `${at}.inserted[${index}]`)),
    moved: architectProgramDiffGuardArray(row["moved"], `${at}.moved`).map((item, index) => parseProgramTemplatesRelocation(item, `${at}.moved[${index}]`)),
    modified: architectProgramDiffGuardArray(row["modified"], `${at}.modified`).map((item, index) => parseProgramTemplatesPatchEntry(item, `${at}.modified[${index}]`)),
  };
}

export interface ProgramTemplatesInsertion {
  readonly index: number;
  readonly row: import("../🟦️.ts").TemplateRecord;
}

export function parseProgramTemplatesInsertion(value: unknown, at = "$"): ProgramTemplatesInsertion {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["index", "row"]);
  return {
    index: architectProgramDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
    row: parseTemplateRecord(row["row"], `${at}.row`),
  };
}

export interface ProgramTemplatesPatchEntry {
  readonly id: string;
  readonly patch: TemplateRecordPatch;
}

export function parseProgramTemplatesPatchEntry(value: unknown, at = "$"): ProgramTemplatesPatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseTemplateRecordPatch(row["patch"], `${at}.patch`),
  };
}

export interface ProgramTemplatesRelocation {
  readonly id: string;
  readonly from: number;
  readonly to: number;
}

export function parseProgramTemplatesRelocation(value: unknown, at = "$"): ProgramTemplatesRelocation {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "from", "to"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    from: architectProgramDiffGuardInteger(row["from"], `${at}.from`, {"minimum": 0}),
    to: architectProgramDiffGuardInteger(row["to"], `${at}.to`, {"minimum": 0}),
  };
}

export interface ProgramTemplatesRemoval {
  readonly id: string;
  readonly index: number;
}

export function parseProgramTemplatesRemoval(value: unknown, at = "$"): ProgramTemplatesRemoval {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "index"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    index: architectProgramDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
  };
}



export function parseProgramTracesDelta(value: unknown, at = "$"): ProgramTracesDelta {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["removed", "inserted", "moved", "modified"]);
  return {
    removed: architectProgramDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => parseProgramTracesRemoval(item, `${at}.removed[${index}]`)),
    inserted: architectProgramDiffGuardArray(row["inserted"], `${at}.inserted`).map((item, index) => parseProgramTracesInsertion(item, `${at}.inserted[${index}]`)),
    moved: architectProgramDiffGuardArray(row["moved"], `${at}.moved`).map((item, index) => parseProgramTracesRelocation(item, `${at}.moved[${index}]`)),
    modified: architectProgramDiffGuardArray(row["modified"], `${at}.modified`).map((item, index) => parseProgramTracesPatchEntry(item, `${at}.modified[${index}]`)),
  };
}

export interface ProgramTracesInsertion {
  readonly index: number;
  readonly row: import("../🟦️.ts").TraceLink;
}

export function parseProgramTracesInsertion(value: unknown, at = "$"): ProgramTracesInsertion {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["index", "row"]);
  return {
    index: architectProgramDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
    row: parseTraceLink(row["row"], `${at}.row`),
  };
}

export interface ProgramTracesPatchEntry {
  readonly id: string;
  readonly patch: TraceLinkPatch;
}

export function parseProgramTracesPatchEntry(value: unknown, at = "$"): ProgramTracesPatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseTraceLinkPatch(row["patch"], `${at}.patch`),
  };
}

export interface ProgramTracesRelocation {
  readonly id: string;
  readonly from: number;
  readonly to: number;
}

export function parseProgramTracesRelocation(value: unknown, at = "$"): ProgramTracesRelocation {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "from", "to"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    from: architectProgramDiffGuardInteger(row["from"], `${at}.from`, {"minimum": 0}),
    to: architectProgramDiffGuardInteger(row["to"], `${at}.to`, {"minimum": 0}),
  };
}

export interface ProgramTracesRemoval {
  readonly id: string;
  readonly index: number;
}

export function parseProgramTracesRemoval(value: unknown, at = "$"): ProgramTracesRemoval {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "index"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    index: architectProgramDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
  };
}



export function parseProgramUsersDelta(value: unknown, at = "$"): ProgramUsersDelta {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["removed", "inserted", "moved", "modified"]);
  return {
    removed: architectProgramDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => parseProgramUsersRemoval(item, `${at}.removed[${index}]`)),
    inserted: architectProgramDiffGuardArray(row["inserted"], `${at}.inserted`).map((item, index) => parseProgramUsersInsertion(item, `${at}.inserted[${index}]`)),
    moved: architectProgramDiffGuardArray(row["moved"], `${at}.moved`).map((item, index) => parseProgramUsersRelocation(item, `${at}.moved[${index}]`)),
    modified: architectProgramDiffGuardArray(row["modified"], `${at}.modified`).map((item, index) => parseProgramUsersPatchEntry(item, `${at}.modified[${index}]`)),
  };
}

export interface ProgramUsersInsertion {
  readonly index: number;
  readonly row: import("../🟦️.ts").UserProfile;
}

export function parseProgramUsersInsertion(value: unknown, at = "$"): ProgramUsersInsertion {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["index", "row"]);
  return {
    index: architectProgramDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
    row: parseUserProfile(row["row"], `${at}.row`),
  };
}

export interface ProgramUsersPatchEntry {
  readonly id: string;
  readonly patch: UserProfilePatch;
}

export function parseProgramUsersPatchEntry(value: unknown, at = "$"): ProgramUsersPatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseUserProfilePatch(row["patch"], `${at}.patch`),
  };
}

export interface ProgramUsersRelocation {
  readonly id: string;
  readonly from: number;
  readonly to: number;
}

export function parseProgramUsersRelocation(value: unknown, at = "$"): ProgramUsersRelocation {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "from", "to"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    from: architectProgramDiffGuardInteger(row["from"], `${at}.from`, {"minimum": 0}),
    to: architectProgramDiffGuardInteger(row["to"], `${at}.to`, {"minimum": 0}),
  };
}

export interface ProgramUsersRemoval {
  readonly id: string;
  readonly index: number;
}

export function parseProgramUsersRemoval(value: unknown, at = "$"): ProgramUsersRemoval {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "index"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    index: architectProgramDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
  };
}



export function parseProgramValidationsDelta(value: unknown, at = "$"): ProgramValidationsDelta {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["removed", "inserted", "moved", "modified"]);
  return {
    removed: architectProgramDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => parseProgramValidationsRemoval(item, `${at}.removed[${index}]`)),
    inserted: architectProgramDiffGuardArray(row["inserted"], `${at}.inserted`).map((item, index) => parseProgramValidationsInsertion(item, `${at}.inserted[${index}]`)),
    moved: architectProgramDiffGuardArray(row["moved"], `${at}.moved`).map((item, index) => parseProgramValidationsRelocation(item, `${at}.moved[${index}]`)),
    modified: architectProgramDiffGuardArray(row["modified"], `${at}.modified`).map((item, index) => parseProgramValidationsPatchEntry(item, `${at}.modified[${index}]`)),
  };
}

export interface ProgramValidationsInsertion {
  readonly index: number;
  readonly row: import("../🟦️.ts").ValidationRecord;
}

export function parseProgramValidationsInsertion(value: unknown, at = "$"): ProgramValidationsInsertion {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["index", "row"]);
  return {
    index: architectProgramDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
    row: parseValidationRecord(row["row"], `${at}.row`),
  };
}

export interface ProgramValidationsPatchEntry {
  readonly id: string;
  readonly patch: ValidationRecordPatch;
}

export function parseProgramValidationsPatchEntry(value: unknown, at = "$"): ProgramValidationsPatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseValidationRecordPatch(row["patch"], `${at}.patch`),
  };
}

export interface ProgramValidationsRelocation {
  readonly id: string;
  readonly from: number;
  readonly to: number;
}

export function parseProgramValidationsRelocation(value: unknown, at = "$"): ProgramValidationsRelocation {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "from", "to"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    from: architectProgramDiffGuardInteger(row["from"], `${at}.from`, {"minimum": 0}),
    to: architectProgramDiffGuardInteger(row["to"], `${at}.to`, {"minimum": 0}),
  };
}

export interface ProgramValidationsRemoval {
  readonly id: string;
  readonly index: number;
}

export function parseProgramValidationsRemoval(value: unknown, at = "$"): ProgramValidationsRemoval {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "index"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    index: architectProgramDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
  };
}



export function parseProgramWayfindingDelta(value: unknown, at = "$"): ProgramWayfindingDelta {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["removed", "inserted", "moved", "modified"]);
  return {
    removed: architectProgramDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => parseProgramWayfindingRemoval(item, `${at}.removed[${index}]`)),
    inserted: architectProgramDiffGuardArray(row["inserted"], `${at}.inserted`).map((item, index) => parseProgramWayfindingInsertion(item, `${at}.inserted[${index}]`)),
    moved: architectProgramDiffGuardArray(row["moved"], `${at}.moved`).map((item, index) => parseProgramWayfindingRelocation(item, `${at}.moved[${index}]`)),
    modified: architectProgramDiffGuardArray(row["modified"], `${at}.modified`).map((item, index) => parseProgramWayfindingPatchEntry(item, `${at}.modified[${index}]`)),
  };
}

export interface ProgramWayfindingInsertion {
  readonly index: number;
  readonly row: import("../🟦️.ts").WayfindingRequirement;
}

export function parseProgramWayfindingInsertion(value: unknown, at = "$"): ProgramWayfindingInsertion {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["index", "row"]);
  return {
    index: architectProgramDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
    row: parseWayfindingRequirement(row["row"], `${at}.row`),
  };
}

export interface ProgramWayfindingPatchEntry {
  readonly id: string;
  readonly patch: WayfindingRequirementPatch;
}

export function parseProgramWayfindingPatchEntry(value: unknown, at = "$"): ProgramWayfindingPatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseWayfindingRequirementPatch(row["patch"], `${at}.patch`),
  };
}

export interface ProgramWayfindingRelocation {
  readonly id: string;
  readonly from: number;
  readonly to: number;
}

export function parseProgramWayfindingRelocation(value: unknown, at = "$"): ProgramWayfindingRelocation {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "from", "to"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    from: architectProgramDiffGuardInteger(row["from"], `${at}.from`, {"minimum": 0}),
    to: architectProgramDiffGuardInteger(row["to"], `${at}.to`, {"minimum": 0}),
  };
}

export interface ProgramWayfindingRemoval {
  readonly id: string;
  readonly index: number;
}

export function parseProgramWayfindingRemoval(value: unknown, at = "$"): ProgramWayfindingRemoval {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "index"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    index: architectProgramDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
  };
}



export function parseProgramWorkshopsDelta(value: unknown, at = "$"): ProgramWorkshopsDelta {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["removed", "inserted", "moved", "modified"]);
  return {
    removed: architectProgramDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => parseProgramWorkshopsRemoval(item, `${at}.removed[${index}]`)),
    inserted: architectProgramDiffGuardArray(row["inserted"], `${at}.inserted`).map((item, index) => parseProgramWorkshopsInsertion(item, `${at}.inserted[${index}]`)),
    moved: architectProgramDiffGuardArray(row["moved"], `${at}.moved`).map((item, index) => parseProgramWorkshopsRelocation(item, `${at}.moved[${index}]`)),
    modified: architectProgramDiffGuardArray(row["modified"], `${at}.modified`).map((item, index) => parseProgramWorkshopsPatchEntry(item, `${at}.modified[${index}]`)),
  };
}

export interface ProgramWorkshopsInsertion {
  readonly index: number;
  readonly row: import("../🟦️.ts").Workshop;
}

export function parseProgramWorkshopsInsertion(value: unknown, at = "$"): ProgramWorkshopsInsertion {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["index", "row"]);
  return {
    index: architectProgramDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
    row: parseWorkshop(row["row"], `${at}.row`),
  };
}

export interface ProgramWorkshopsPatchEntry {
  readonly id: string;
  readonly patch: WorkshopPatch;
}

export function parseProgramWorkshopsPatchEntry(value: unknown, at = "$"): ProgramWorkshopsPatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseWorkshopPatch(row["patch"], `${at}.patch`),
  };
}

export interface ProgramWorkshopsRelocation {
  readonly id: string;
  readonly from: number;
  readonly to: number;
}

export function parseProgramWorkshopsRelocation(value: unknown, at = "$"): ProgramWorkshopsRelocation {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "from", "to"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    from: architectProgramDiffGuardInteger(row["from"], `${at}.from`, {"minimum": 0}),
    to: architectProgramDiffGuardInteger(row["to"], `${at}.to`, {"minimum": 0}),
  };
}

export interface ProgramWorkshopsRemoval {
  readonly id: string;
  readonly index: number;
}

export function parseProgramWorkshopsRemoval(value: unknown, at = "$"): ProgramWorkshopsRemoval {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "index"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    index: architectProgramDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
  };
}



export function parseProjectDefinitionEdit(value: unknown, at = "$"): ProjectDefinitionEdit {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["set", "patch"]);
  return {
    set: (row["set"] === null ? null : parseProjectDefinition(row["set"], `${at}.set`)),
    patch: (row["patch"] === null ? null : parseProjectDefinitionPatch(row["patch"], `${at}.patch`)),
  };
}

export type ProjectDefinitionPatch = Readonly<Record<string, unknown>>;

export function parseProjectDefinitionPatch(value: unknown, at = "$"): ProjectDefinitionPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export type QualityRecordPatch = Readonly<Record<string, unknown>>;

export function parseQualityRecordPatch(value: unknown, at = "$"): QualityRecordPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export type QuantityRequirementPatch = Readonly<Record<string, unknown>>;

export function parseQuantityRequirementPatch(value: unknown, at = "$"): QuantityRequirementPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export type RegulatoryRequirementPatch = Readonly<Record<string, unknown>>;

export function parseRegulatoryRequirementPatch(value: unknown, at = "$"): RegulatoryRequirementPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export type RelationshipPatch = Readonly<Record<string, unknown>>;

export function parseRelationshipPatch(value: unknown, at = "$"): RelationshipPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export type ReportRecordPatch = Readonly<Record<string, unknown>>;

export function parseReportRecordPatch(value: unknown, at = "$"): ReportRecordPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export type RequirementPatch = Readonly<Record<string, unknown>>;

export function parseRequirementPatch(value: unknown, at = "$"): RequirementPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export type ResilienceRequirementPatch = Readonly<Record<string, unknown>>;

export function parseResilienceRequirementPatch(value: unknown, at = "$"): ResilienceRequirementPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export type ResourcePatch = Readonly<Record<string, unknown>>;

export function parseResourcePatch(value: unknown, at = "$"): ResourcePatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export type RiskPatch = Readonly<Record<string, unknown>>;

export function parseRiskPatch(value: unknown, at = "$"): RiskPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export type SafetyRequirementPatch = Readonly<Record<string, unknown>>;

export function parseSafetyRequirementPatch(value: unknown, at = "$"): SafetyRequirementPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export type ScenarioPatch = Readonly<Record<string, unknown>>;

export function parseScenarioPatch(value: unknown, at = "$"): ScenarioPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export type ScheduleRequirementPatch = Readonly<Record<string, unknown>>;

export function parseScheduleRequirementPatch(value: unknown, at = "$"): ScheduleRequirementPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export type SearchFilterPatch = Readonly<Record<string, unknown>>;

export function parseSearchFilterPatch(value: unknown, at = "$"): SearchFilterPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export type SecurityRequirementPatch = Readonly<Record<string, unknown>>;

export function parseSecurityRequirementPatch(value: unknown, at = "$"): SecurityRequirementPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export type ServiceRequirementPatch = Readonly<Record<string, unknown>>;

export function parseServiceRequirementPatch(value: unknown, at = "$"): ServiceRequirementPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export type SiteContextPatch = Readonly<Record<string, unknown>>;

export function parseSiteContextPatch(value: unknown, at = "$"): SiteContextPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export type StakeholderPatch = Readonly<Record<string, unknown>>;

export function parseStakeholderPatch(value: unknown, at = "$"): StakeholderPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export type StatusRecordPatch = Readonly<Record<string, unknown>>;

export function parseStatusRecordPatch(value: unknown, at = "$"): StatusRecordPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export type StorageRequirementPatch = Readonly<Record<string, unknown>>;

export function parseStorageRequirementPatch(value: unknown, at = "$"): StorageRequirementPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export type SurveyPatch = Readonly<Record<string, unknown>>;

export function parseSurveyPatch(value: unknown, at = "$"): SurveyPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export type SustainabilityRequirementPatch = Readonly<Record<string, unknown>>;

export function parseSustainabilityRequirementPatch(value: unknown, at = "$"): SustainabilityRequirementPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export interface TaggedNote {
  readonly tag: string;
  readonly text: string;
}

export function parseTaggedNote(value: unknown, at = "$"): TaggedNote {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["tag", "text"]);
  return {
    tag: architectProgramDiffGuardString(row["tag"], `${at}.tag`),
    text: architectProgramDiffGuardString(row["text"], `${at}.text`),
  };
}

export type TemplateRecordPatch = Readonly<Record<string, unknown>>;

export function parseTemplateRecordPatch(value: unknown, at = "$"): TemplateRecordPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export interface TextField {
  readonly text: string;
  readonly format?: string;
}

export function parseTextField(value: unknown, at = "$"): TextField {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["text", "format"]);
  return {
    text: architectProgramDiffGuardString(row["text"], `${at}.text`),
    format: row["format"] === undefined ? undefined : architectProgramDiffGuardString(row["format"], `${at}.format`),
  };
}

export interface TimestampMeta {
  readonly created: string;
  readonly updated: string;
  readonly createdBy?: string;
  readonly updatedBy?: string;
}

export function parseTimestampMeta(value: unknown, at = "$"): TimestampMeta {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["created", "updated", "createdBy", "updatedBy"]);
  return {
    created: architectProgramDiffGuardString(row["created"], `${at}.created`),
    updated: architectProgramDiffGuardString(row["updated"], `${at}.updated`),
    createdBy: row["createdBy"] === undefined ? undefined : architectProgramDiffGuardString(row["createdBy"], `${at}.createdBy`),
    updatedBy: row["updatedBy"] === undefined ? undefined : architectProgramDiffGuardString(row["updatedBy"], `${at}.updatedBy`),
  };
}

export type TraceLinkPatch = Readonly<Record<string, unknown>>;

export function parseTraceLinkPatch(value: unknown, at = "$"): TraceLinkPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export type UserProfilePatch = Readonly<Record<string, unknown>>;

export function parseUserProfilePatch(value: unknown, at = "$"): UserProfilePatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export type ValidationRecordPatch = Readonly<Record<string, unknown>>;

export function parseValidationRecordPatch(value: unknown, at = "$"): ValidationRecordPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export type ValidationStatus = "pending" | "passed" | "failed" | "waived" | "deferred";

export function parseValidationStatus(value: unknown, at = "$"): ValidationStatus {
  return architectProgramDiffGuardMember(value, `${at}`, ["pending", "passed", "failed", "waived", "deferred"] as const);
}

export type WayfindingRequirementPatch = Readonly<Record<string, unknown>>;

export function parseWayfindingRequirementPatch(value: unknown, at = "$"): WayfindingRequirementPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}

export type WorkshopPatch = Readonly<Record<string, unknown>>;

export function parseWorkshopPatch(value: unknown, at = "$"): WorkshopPatch {
  return architectProgramDiffGuardObject(value, `${at}`);
}
