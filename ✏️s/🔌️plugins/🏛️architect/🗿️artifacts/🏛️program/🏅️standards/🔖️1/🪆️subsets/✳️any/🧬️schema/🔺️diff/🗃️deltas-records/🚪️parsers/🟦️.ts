/** 🚪️ The typed `parse<Export>` entry points projected from the schema for the types declared next to this folder.
 * @see ../🟦️.ts
 */
import { architectProgramDiffGuardClosed, architectProgramDiffGuardObject, architectProgramDiffGuardString } from "../../🟦️.ts";
import { parseAnalysisRecordPatch, parseApprovalRecordPatch, parseArtifactRecordPatch, parseAssumptionPatch, parseAuditEventPatch, parseBenchmarkRecordPatch, parseChangeRecordPatch, parseCollaborationRecordPatch, parseComplianceRecordPatch, parseConflictPatch, parseConstraintRecordPatch, parseDecisionPatch, parseIssuePatch, parseKnowledgeRecordPatch, parseMeetingRecordPatch, parseOptionEvaluationPatch, parsePerformanceCriterionPatch, parsePriorityRecordPatch, parseQualityRecordPatch, parseReportRecordPatch, parseRequirementPatch, parseRiskPatch, parseScenarioPatch, parseSearchFilterPatch, parseStatusRecordPatch, parseSurveyPatch, parseTemplateRecordPatch, parseTraceLinkPatch, parseValidationRecordPatch, parseWorkshopPatch } from "../../🩹️patch-types/🚪️parsers/🟦️.ts";
import type { ProgramAnalysesPatchEntry, ProgramApprovalsPatchEntry, ProgramArtifactsPatchEntry, ProgramAssumptionsPatchEntry, ProgramAuditEventsPatchEntry, ProgramBenchmarksPatchEntry, ProgramChangesPatchEntry, ProgramCollaborationPatchEntry, ProgramComplianceRecordsPatchEntry, ProgramConflictsPatchEntry, ProgramConstraintsPatchEntry, ProgramDecisionsPatchEntry, ProgramIssuesPatchEntry, ProgramKnowledgePatchEntry, ProgramMeetingsPatchEntry, ProgramOptionsPatchEntry, ProgramPerformancePatchEntry, ProgramPrioritiesPatchEntry, ProgramQualityPatchEntry, ProgramReportsPatchEntry, ProgramRequirementsPatchEntry, ProgramRisksPatchEntry, ProgramScenariosPatchEntry, ProgramSearchFiltersPatchEntry, ProgramStatusRecordsPatchEntry, ProgramSurveysPatchEntry, ProgramTemplatesPatchEntry, ProgramTracesPatchEntry, ProgramValidationsPatchEntry, ProgramWorkshopsPatchEntry } from "../🟦️.ts";

//#region 🚪️Parsers
export function parseProgramRisksPatchEntry(value: unknown, at = "$"): ProgramRisksPatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseRiskPatch(row["patch"], `${at}.patch`),
  };
}

export function parseProgramConflictsPatchEntry(value: unknown, at = "$"): ProgramConflictsPatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseConflictPatch(row["patch"], `${at}.patch`),
  };
}

export function parseProgramRequirementsPatchEntry(value: unknown, at = "$"): ProgramRequirementsPatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseRequirementPatch(row["patch"], `${at}.patch`),
  };
}

export function parseProgramPrioritiesPatchEntry(value: unknown, at = "$"): ProgramPrioritiesPatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parsePriorityRecordPatch(row["patch"], `${at}.patch`),
  };
}

export function parseProgramScenariosPatchEntry(value: unknown, at = "$"): ProgramScenariosPatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseScenarioPatch(row["patch"], `${at}.patch`),
  };
}

export function parseProgramOptionsPatchEntry(value: unknown, at = "$"): ProgramOptionsPatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseOptionEvaluationPatch(row["patch"], `${at}.patch`),
  };
}

export function parseProgramDecisionsPatchEntry(value: unknown, at = "$"): ProgramDecisionsPatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseDecisionPatch(row["patch"], `${at}.patch`),
  };
}

export function parseProgramValidationsPatchEntry(value: unknown, at = "$"): ProgramValidationsPatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseValidationRecordPatch(row["patch"], `${at}.patch`),
  };
}

export function parseProgramPerformancePatchEntry(value: unknown, at = "$"): ProgramPerformancePatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parsePerformanceCriterionPatch(row["patch"], `${at}.patch`),
  };
}

export function parseProgramQualityPatchEntry(value: unknown, at = "$"): ProgramQualityPatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseQualityRecordPatch(row["patch"], `${at}.patch`),
  };
}

export function parseProgramArtifactsPatchEntry(value: unknown, at = "$"): ProgramArtifactsPatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseArtifactRecordPatch(row["patch"], `${at}.patch`),
  };
}

export function parseProgramAssumptionsPatchEntry(value: unknown, at = "$"): ProgramAssumptionsPatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseAssumptionPatch(row["patch"], `${at}.patch`),
  };
}

export function parseProgramConstraintsPatchEntry(value: unknown, at = "$"): ProgramConstraintsPatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseConstraintRecordPatch(row["patch"], `${at}.patch`),
  };
}

export function parseProgramComplianceRecordsPatchEntry(value: unknown, at = "$"): ProgramComplianceRecordsPatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseComplianceRecordPatch(row["patch"], `${at}.patch`),
  };
}

export function parseProgramApprovalsPatchEntry(value: unknown, at = "$"): ProgramApprovalsPatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseApprovalRecordPatch(row["patch"], `${at}.patch`),
  };
}

export function parseProgramMeetingsPatchEntry(value: unknown, at = "$"): ProgramMeetingsPatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseMeetingRecordPatch(row["patch"], `${at}.patch`),
  };
}

export function parseProgramChangesPatchEntry(value: unknown, at = "$"): ProgramChangesPatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseChangeRecordPatch(row["patch"], `${at}.patch`),
  };
}

export function parseProgramCollaborationPatchEntry(value: unknown, at = "$"): ProgramCollaborationPatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseCollaborationRecordPatch(row["patch"], `${at}.patch`),
  };
}

export function parseProgramAnalysesPatchEntry(value: unknown, at = "$"): ProgramAnalysesPatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseAnalysisRecordPatch(row["patch"], `${at}.patch`),
  };
}

export function parseProgramReportsPatchEntry(value: unknown, at = "$"): ProgramReportsPatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseReportRecordPatch(row["patch"], `${at}.patch`),
  };
}

export function parseProgramSearchFiltersPatchEntry(value: unknown, at = "$"): ProgramSearchFiltersPatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseSearchFilterPatch(row["patch"], `${at}.patch`),
  };
}

export function parseProgramStatusRecordsPatchEntry(value: unknown, at = "$"): ProgramStatusRecordsPatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseStatusRecordPatch(row["patch"], `${at}.patch`),
  };
}

export function parseProgramWorkshopsPatchEntry(value: unknown, at = "$"): ProgramWorkshopsPatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseWorkshopPatch(row["patch"], `${at}.patch`),
  };
}

export function parseProgramSurveysPatchEntry(value: unknown, at = "$"): ProgramSurveysPatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseSurveyPatch(row["patch"], `${at}.patch`),
  };
}

export function parseProgramIssuesPatchEntry(value: unknown, at = "$"): ProgramIssuesPatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseIssuePatch(row["patch"], `${at}.patch`),
  };
}

export function parseProgramAuditEventsPatchEntry(value: unknown, at = "$"): ProgramAuditEventsPatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseAuditEventPatch(row["patch"], `${at}.patch`),
  };
}

export function parseProgramTemplatesPatchEntry(value: unknown, at = "$"): ProgramTemplatesPatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseTemplateRecordPatch(row["patch"], `${at}.patch`),
  };
}

export function parseProgramTracesPatchEntry(value: unknown, at = "$"): ProgramTracesPatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseTraceLinkPatch(row["patch"], `${at}.patch`),
  };
}

export function parseProgramKnowledgePatchEntry(value: unknown, at = "$"): ProgramKnowledgePatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseKnowledgeRecordPatch(row["patch"], `${at}.patch`),
  };
}

export function parseProgramBenchmarksPatchEntry(value: unknown, at = "$"): ProgramBenchmarksPatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseBenchmarkRecordPatch(row["patch"], `${at}.patch`),
  };
}
//#endregion 🚪️Parsers
