/** 🚪️ The typed `parse<Export>` entry points projected from the schema for the types declared next to this folder.
 * @see ../🟦️.ts
 */
import { architectProgramArtifactGuardMember } from "../../🟦️.ts";
import type { CostBasis, EnvironmentalParameter, HumanFactorAspect, PrivacyKind, SafetyDomain, SecurityControlKind, StorageClass } from "../🟦️.ts";

//#region 🚪️Parsers
export function parseCostBasis(value: unknown, at = "$"): CostBasis {
  return architectProgramArtifactGuardMember(value, `${at}`, ["capital", "operational", "lifecycle", "replacement", "maintenance"] as const);
}

export function parseEnvironmentalParameter(value: unknown, at = "$"): EnvironmentalParameter {
  return architectProgramArtifactGuardMember(value, `${at}`, ["temperature", "humidity", "airQuality", "lighting", "acoustics", "ventilation", "radiation", "vibration", "pressure", "iaq"] as const);
}

export function parseHumanFactorAspect(value: unknown, at = "$"): HumanFactorAspect {
  return architectProgramArtifactGuardMember(value, `${at}`, ["ergonomics", "cognition", "sensory", "social", "cultural", "behavioral", "physical", "psychological", "fatigue", "stress"] as const);
}

export function parsePrivacyKind(value: unknown, at = "$"): PrivacyKind {
  return architectProgramArtifactGuardMember(value, `${at}`, ["public", "semiPublic", "semiPrivate", "private", "confidential", "restricted", "anonymous"] as const);
}

export function parseSafetyDomain(value: unknown, at = "$"): SafetyDomain {
  return architectProgramArtifactGuardMember(value, `${at}`, ["lifeSafety", "occupationalHealth", "fire", "structural", "electrical", "chemical", "radiation", "ergonomics", "biological", "environmental"] as const);
}

export function parseSecurityControlKind(value: unknown, at = "$"): SecurityControlKind {
  return architectProgramArtifactGuardMember(value, `${at}`, ["accessControl", "surveillance", "perimeter", "cyber", "personnel", "information", "physical", "procedural", "screening", "keyManagement"] as const);
}

export function parseStorageClass(value: unknown, at = "$"): StorageClass {
  return architectProgramArtifactGuardMember(value, `${at}`, ["general", "secure", "climateControlled", "hazardous", "archive", "mobile", "fixed", "shared", "coldChain", "flammable"] as const);
}
//#endregion 🚪️Parsers
