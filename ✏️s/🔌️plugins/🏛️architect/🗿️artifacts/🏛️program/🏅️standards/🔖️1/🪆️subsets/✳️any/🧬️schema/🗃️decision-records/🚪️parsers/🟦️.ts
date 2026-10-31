/** 🚪️ The typed `parse<Export>` entry points projected from the schema for the types declared next to this folder.
 * @see ../🟦️.ts
 */
import { architectProgramArtifactGuardMember } from "../../🟦️.ts";
import type { AnalysisKind, AuditAction, ConflictKind, ReportKind, RequirementKind } from "../🟦️.ts";

//#region 🚪️Parsers
export function parseAnalysisKind(value: unknown, at = "$"): AnalysisKind {
  return architectProgramArtifactGuardMember(value, `${at}`, ["gap", "conflict", "dependency", "capacity", "demand", "utilization", "workflow", "risk", "cost", "scenario", "sensitivity", "impact", "trend", "requirementComparison", "requirementClustering", "requirementFiltering", "requirementSorting", "requirementScoring", "requirementWeighting", "relationshipAnalysis"] as const);
}

export function parseAuditAction(value: unknown, at = "$"): AuditAction {
  return architectProgramArtifactGuardMember(value, `${at}`, ["created", "updated", "deleted", "reviewed", "approved", "rejected", "exported", "imported", "merged", "archived"] as const);
}

export function parseConflictKind(value: unknown, at = "$"): ConflictKind {
  return architectProgramArtifactGuardMember(value, `${at}`, ["adjacency", "capacity", "schedule", "budget", "regulatory", "operational", "environmental", "security", "priority"] as const);
}

export function parseReportKind(value: unknown, at = "$"): ReportKind {
  return architectProgramArtifactGuardMember(value, `${at}`, ["executiveSummary", "programOverview", "stakeholderSummary", "requirementsMatrix", "adjacencyMatrix", "gapAnalysis", "riskRegister", "decisionLog", "validationSummary", "recommendation", "userSummary", "functionalSummary", "capacitySummary", "workflowSummary", "complianceSummary", "costSummary", "scheduleSummary", "changeSummary", "openIssueSummary", "prioritySummary", "scenarioSummary"] as const);
}

export function parseRequirementKind(value: unknown, at = "$"): RequirementKind {
  return architectProgramArtifactGuardMember(value, `${at}`, ["functional", "spatial", "performance", "regulatory", "operational", "technical", "aesthetic", "sustainability"] as const);
}
//#endregion 🚪️Parsers
