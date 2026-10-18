/** 🧬️ ProgramSnapshot diff schema — sparse field delta. */

import { architectProgramArtifactGuardExactObject, PROGRAM_ARTIFACT_FIELDS } from "../🟦️.ts";
import type { ProgramDiff } from "./🧾️program-diff/🟦️.ts";

export type { ProgramDiff };

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

export { parseAccessRulePatch, parseAccessibilityRequirementPatch, parseActivityPatch, parseAdjacencyPatch, parseAnalysisRecordPatch, parseApprovalRecordPatch, parseArtifactRecordPatch, parseAssumptionPatch, parseAuditEventPatch, parseBenchmarkRecordPatch, parseChangeRecordPatch, parseCollaborationRecordPatch, parseCommunicationRequirementPatch, parseComplianceRecordPatch, parseConflictPatch, parseConstraintRecordPatch, parseCostRequirementPatch, parseDecisionPatch, parseDeliveryConstraintPatch, parseEnvironmentalRequirementPatch, parseEquipmentPatch, parseFlexibilityRequirementPatch, parseFlowRequirementPatch, parseFunctionPatch, parseGovernancePatch, parseGrowthPlanPatch, parseHumanFactorRequirementPatch, parseInformationRequirementPatch, parseInfrastructureRequirementPatch, parseIssuePatch, parseKnowledgeRecordPatch, parseMeetingRecordPatch, parseOperationalRequirementPatch, parseOptionEvaluationPatch, parseOrganizationalRequirementPatch, parsePerformanceCriterionPatch, parsePriorityRecordPatch, parsePrivacyRequirementPatch, parseProcessPatch, parseProgramElementPatch, parseProgramMetaPatch, parseProjectDefinitionPatch, parseQualityRecordPatch, parseQuantityRequirementPatch, parseRegulatoryRequirementPatch, parseRelationshipPatch, parseReportRecordPatch, parseRequirementPatch, parseResilienceRequirementPatch, parseResourcePatch, parseRiskPatch, parseSafetyRequirementPatch, parseScenarioPatch, parseScheduleRequirementPatch, parseSearchFilterPatch, parseSecurityRequirementPatch, parseServiceRequirementPatch, parseSiteContextPatch, parseStakeholderPatch, parseStatusRecordPatch, parseStorageRequirementPatch, parseSurveyPatch, parseSustainabilityRequirementPatch, parseTemplateRecordPatch, parseTraceLinkPatch, parseUserProfilePatch, parseValidationRecordPatch, parseWayfindingRequirementPatch, parseWorkshopPatch } from "./🩹️patch-types/🚪️parsers/🟦️.ts";
export { parseProgramAccessRulesPatchEntry, parseProgramActivitiesPatchEntry, parseProgramAdjacenciesPatchEntry, parseProgramElementsPatchEntry, parseProgramFlowsPatchEntry, parseProgramFunctionsPatchEntry, parseProgramProcessesPatchEntry, parseProgramQuantitiesPatchEntry, parseProgramRelationshipsPatchEntry, parseProgramStakeholdersPatchEntry, parseProgramUsersPatchEntry } from "./🧱️deltas-program-structure/🚪️parsers/🟦️.ts";
export { parseProgramAccessibilityPatchEntry, parseProgramCommunicationPatchEntry, parseProgramCostsPatchEntry, parseProgramDeliveryPatchEntry, parseProgramEnvironmentalPatchEntry, parseProgramEquipmentPatchEntry, parseProgramFlexibilityPatchEntry, parseProgramGrowthPatchEntry, parseProgramHumanFactorsPatchEntry, parseProgramInformationPatchEntry, parseProgramInfrastructurePatchEntry, parseProgramOperationsPatchEntry, parseProgramOrganizationalPatchEntry, parseProgramPrivacyPatchEntry, parseProgramRegulatoryPatchEntry, parseProgramResiliencePatchEntry, parseProgramResourcesPatchEntry, parseProgramSafetyPatchEntry, parseProgramSchedulesPatchEntry, parseProgramSecurityPatchEntry, parseProgramServicesPatchEntry, parseProgramSiteContextPatchEntry, parseProgramStoragePatchEntry, parseProgramSustainabilityPatchEntry, parseProgramWayfindingPatchEntry } from "./📋️deltas-requirements/🚪️parsers/🟦️.ts";
export { parseProgramAnalysesPatchEntry, parseProgramApprovalsPatchEntry, parseProgramArtifactsPatchEntry, parseProgramAssumptionsPatchEntry, parseProgramAuditEventsPatchEntry, parseProgramBenchmarksPatchEntry, parseProgramChangesPatchEntry, parseProgramCollaborationPatchEntry, parseProgramComplianceRecordsPatchEntry, parseProgramConflictsPatchEntry, parseProgramConstraintsPatchEntry, parseProgramDecisionsPatchEntry, parseProgramIssuesPatchEntry, parseProgramKnowledgePatchEntry, parseProgramMeetingsPatchEntry, parseProgramOptionsPatchEntry, parseProgramPerformancePatchEntry, parseProgramPrioritiesPatchEntry, parseProgramQualityPatchEntry, parseProgramReportsPatchEntry, parseProgramRequirementsPatchEntry, parseProgramRisksPatchEntry, parseProgramScenariosPatchEntry, parseProgramSearchFiltersPatchEntry, parseProgramStatusRecordsPatchEntry, parseProgramSurveysPatchEntry, parseProgramTemplatesPatchEntry, parseProgramTracesPatchEntry, parseProgramValidationsPatchEntry, parseProgramWorkshopsPatchEntry } from "./🗃️deltas-records/🚪️parsers/🟦️.ts";
