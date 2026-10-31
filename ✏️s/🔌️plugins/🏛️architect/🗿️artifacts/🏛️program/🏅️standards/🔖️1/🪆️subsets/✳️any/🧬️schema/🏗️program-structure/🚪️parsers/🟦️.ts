/** 🚪️ The typed `parse<Export>` entry points projected from the schema for the types declared next to this folder.
 * @see ../🟦️.ts
 */
import { architectProgramArtifactGuardMember } from "../../🟦️.ts";
import type { AccessMode, ConnectionKind, EngagementLevel, FlowDirection, FlowKind, FunctionKind, InfluenceLevel, ProgramElementKind, RelationshipKind, UserCategory } from "../🟦️.ts";

//#region 🚪️Parsers
export function parseAccessMode(value: unknown, at = "$"): AccessMode {
  return architectProgramArtifactGuardMember(value, `${at}`, ["unrestricted", "cardControlled", "biometric", "keyed", "escortRequired", "timeRestricted", "roleBased", "emergencyOnly"] as const);
}

export function parseConnectionKind(value: unknown, at = "$"): ConnectionKind {
  return architectProgramArtifactGuardMember(value, `${at}`, ["direct", "indirect", "controlled", "sharedAccess", "none"] as const);
}

export function parseEngagementLevel(value: unknown, at = "$"): EngagementLevel {
  return architectProgramArtifactGuardMember(value, `${at}`, ["unaware", "resistant", "neutral", "supportive", "leading"] as const);
}

export function parseFlowDirection(value: unknown, at = "$"): FlowDirection {
  return architectProgramArtifactGuardMember(value, `${at}`, ["oneWay", "twoWay", "bidirectionalPeak", "restricted"] as const);
}

export function parseFlowKind(value: unknown, at = "$"): FlowKind {
  return architectProgramArtifactGuardMember(value, `${at}`, ["people", "material", "information", "service", "equipment", "waste", "emergency", "vehicle"] as const);
}

export function parseFunctionKind(value: unknown, at = "$"): FunctionKind {
  return architectProgramArtifactGuardMember(value, `${at}`, ["primary", "secondary", "support", "administrative", "service", "technical", "public", "private", "shared", "restricted", "temporary", "future", "operational", "circulation"] as const);
}

export function parseInfluenceLevel(value: unknown, at = "$"): InfluenceLevel {
  return architectProgramArtifactGuardMember(value, `${at}`, ["low", "medium", "high", "critical"] as const);
}

export function parseProgramElementKind(value: unknown, at = "$"): ProgramElementKind {
  return architectProgramArtifactGuardMember(value, `${at}`, ["building", "campus", "floor", "zone", "room", "suite", "department", "system", "circulation", "support", "outdoor", "furnitureGroup", "other"] as const);
}

export function parseRelationshipKind(value: unknown, at = "$"): RelationshipKind {
  return architectProgramArtifactGuardMember(value, `${at}`, ["contains", "serves", "supports", "dependsOn", "conflictsWith", "equivalentTo", "adjacentTo", "feeds", "receives", "controls", "monitors", "functional", "operational", "organizational", "user", "service", "information", "access", "security", "supervision", "communication", "dependency", "sequential", "sharedResource"] as const);
}

export function parseUserCategory(value: unknown, at = "$"): UserCategory {
  return architectProgramArtifactGuardMember(value, `${at}`, ["primary", "secondary", "occasional", "service", "visitor", "staff", "public"] as const);
}
//#endregion 🚪️Parsers
