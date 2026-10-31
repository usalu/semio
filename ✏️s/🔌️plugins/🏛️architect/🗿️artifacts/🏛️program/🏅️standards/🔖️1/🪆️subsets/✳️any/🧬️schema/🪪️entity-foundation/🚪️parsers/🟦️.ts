/** 🚪️ The typed `parse<Export>` entry points projected from the schema for the types declared next to this folder.
 * @see ../🟦️.ts
 */
import { parseTextField, parseTimestampMeta } from "../../🟦️.ts";
import { architectProgramArtifactGuardArray, architectProgramArtifactGuardClosed, architectProgramArtifactGuardMember, architectProgramArtifactGuardObject, architectProgramArtifactGuardString } from "../../🟦️.ts";
import type { AccessLevel, DeliveryPhase, EntityHeader, IssueSeverity, LifecycleStatus, Ownership, Priority, RiskLevel, SeparationKind, TaggedNote, TextField, TimestampMeta, TraceKind, ValidationStatus } from "../🟦️.ts";

//#region 🚪️Parsers
export function parseAccessLevel(value: unknown, at = "$"): AccessLevel {
  return architectProgramArtifactGuardMember(value, `${at}`, ["public", "restricted", "controlled", "private", "secure", "emergencyOnly"] as const);
}

export function parseDeliveryPhase(value: unknown, at = "$"): DeliveryPhase {
  return architectProgramArtifactGuardMember(value, `${at}`, ["concept", "schematic", "designDevelopment", "constructionDocuments", "procurement", "construction", "commissioning", "occupancy"] as const);
}

export function parseEntityHeader(value: unknown, at = "$"): EntityHeader {
  const row = architectProgramArtifactGuardClosed(architectProgramArtifactGuardObject(value, at), at, ["id", "name", "description", "status", "priority", "ownership", "tags", "notes", "timestamps"]);
  return {
    id: architectProgramArtifactGuardString(row["id"], `${at}.id`),
    name: architectProgramArtifactGuardString(row["name"], `${at}.name`),
    description: row["description"] === undefined ? undefined : parseTextField(row["description"], `${at}.description`),
    status: parseLifecycleStatus(row["status"], `${at}.status`),
    priority: parsePriority(row["priority"], `${at}.priority`),
    ownership: parseOwnership(row["ownership"], `${at}.ownership`),
    tags: row["tags"] === undefined ? [] : architectProgramArtifactGuardArray(row["tags"], `${at}.tags`).map((item, index) => architectProgramArtifactGuardString(item, `${at}.tags[${index}]`)),
    notes: row["notes"] === undefined ? [] : architectProgramArtifactGuardArray(row["notes"], `${at}.notes`).map((item, index) => parseTaggedNote(item, `${at}.notes[${index}]`)),
    timestamps: parseTimestampMeta(row["timestamps"], `${at}.timestamps`),
  };
}

export function parseIssueSeverity(value: unknown, at = "$"): IssueSeverity {
  return architectProgramArtifactGuardMember(value, `${at}`, ["cosmetic", "minor", "major", "critical", "blocker"] as const);
}

export function parseLifecycleStatus(value: unknown, at = "$"): LifecycleStatus {
  return architectProgramArtifactGuardMember(value, `${at}`, ["draft", "proposed", "underReview", "validated", "approved", "rejected", "deferred", "superseded", "archived", "open", "closed", "atRisk", "blocked", "inProgress", "complete"] as const);
}

export function parseOwnership(value: unknown, at = "$"): Ownership {
  const row = architectProgramArtifactGuardClosed(architectProgramArtifactGuardObject(value, at), at, ["ownerId", "authorityId", "consultantIds", "participantIds"]);
  return {
    ownerId: (row["ownerId"] === null ? null : architectProgramArtifactGuardString(row["ownerId"], `${at}.ownerId`)),
    authorityId: (row["authorityId"] === null ? null : architectProgramArtifactGuardString(row["authorityId"], `${at}.authorityId`)),
    consultantIds: architectProgramArtifactGuardArray(row["consultantIds"], `${at}.consultantIds`).map((item, index) => architectProgramArtifactGuardString(item, `${at}.consultantIds[${index}]`)),
    participantIds: architectProgramArtifactGuardArray(row["participantIds"], `${at}.participantIds`).map((item, index) => architectProgramArtifactGuardString(item, `${at}.participantIds[${index}]`)),
  };
}

export function parsePriority(value: unknown, at = "$"): Priority {
  return architectProgramArtifactGuardMember(value, `${at}`, ["mandatory", "essential", "preferred", "optional", "deferred", "prohibited"] as const);
}

export function parseRiskLevel(value: unknown, at = "$"): RiskLevel {
  return architectProgramArtifactGuardMember(value, `${at}`, ["negligible", "low", "medium", "high", "critical"] as const);
}

export function parseSeparationKind(value: unknown, at = "$"): SeparationKind {
  return architectProgramArtifactGuardMember(value, `${at}`, ["acoustic", "visual", "security", "olfactory", "thermal", "fire", "hygienic", "circulation", "operational", "infectionControl"] as const);
}

export function parseTaggedNote(value: unknown, at = "$"): TaggedNote {
  const row = architectProgramArtifactGuardClosed(architectProgramArtifactGuardObject(value, at), at, ["tag", "text"]);
  return {
    tag: architectProgramArtifactGuardString(row["tag"], `${at}.tag`),
    text: architectProgramArtifactGuardString(row["text"], `${at}.text`),
  };
}



export function parseTraceKind(value: unknown, at = "$"): TraceKind {
  return architectProgramArtifactGuardMember(value, `${at}`, ["objectiveToRequirement", "stakeholderToRequirement", "userToActivity", "activityToFunction", "functionToProgramElement", "requirementToDecision", "requirementToRisk", "requirementToStandard", "requirementToValidation", "requirementToApproval", "requirementToChange", "equipmentToActivity", "processToResource", "constraintToImpact", "scenarioToDecision", "issueToAction", "actionToOwner", "decisionToOutcome", "versionToChange", "fullAuditTrail"] as const);
}

export function parseValidationStatus(value: unknown, at = "$"): ValidationStatus {
  return architectProgramArtifactGuardMember(value, `${at}`, ["pending", "passed", "failed", "waived", "deferred"] as const);
}
//#endregion 🚪️Parsers
