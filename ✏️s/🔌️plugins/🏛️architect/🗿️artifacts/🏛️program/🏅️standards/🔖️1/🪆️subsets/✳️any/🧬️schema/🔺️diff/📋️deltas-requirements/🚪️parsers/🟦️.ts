/** 🚪️ The typed `parse<Export>` entry points projected from the schema for the types declared next to this folder.
 * @see ../🟦️.ts
 */
import { architectProgramDiffGuardClosed, architectProgramDiffGuardObject, architectProgramDiffGuardString } from "../../🟦️.ts";
import { parseAccessibilityRequirementPatch, parseCommunicationRequirementPatch, parseCostRequirementPatch, parseDeliveryConstraintPatch, parseEnvironmentalRequirementPatch, parseEquipmentPatch, parseFlexibilityRequirementPatch, parseGrowthPlanPatch, parseHumanFactorRequirementPatch, parseInformationRequirementPatch, parseInfrastructureRequirementPatch, parseOperationalRequirementPatch, parseOrganizationalRequirementPatch, parsePrivacyRequirementPatch, parseRegulatoryRequirementPatch, parseResilienceRequirementPatch, parseResourcePatch, parseSafetyRequirementPatch, parseScheduleRequirementPatch, parseSecurityRequirementPatch, parseServiceRequirementPatch, parseSiteContextPatch, parseStorageRequirementPatch, parseSustainabilityRequirementPatch, parseWayfindingRequirementPatch } from "../../🩹️patch-types/🚪️parsers/🟦️.ts";
import type { ProgramAccessibilityPatchEntry, ProgramCommunicationPatchEntry, ProgramCostsPatchEntry, ProgramDeliveryPatchEntry, ProgramEnvironmentalPatchEntry, ProgramEquipmentPatchEntry, ProgramFlexibilityPatchEntry, ProgramGrowthPatchEntry, ProgramHumanFactorsPatchEntry, ProgramInformationPatchEntry, ProgramInfrastructurePatchEntry, ProgramOperationsPatchEntry, ProgramOrganizationalPatchEntry, ProgramPrivacyPatchEntry, ProgramRegulatoryPatchEntry, ProgramResiliencePatchEntry, ProgramResourcesPatchEntry, ProgramSafetyPatchEntry, ProgramSchedulesPatchEntry, ProgramSecurityPatchEntry, ProgramServicesPatchEntry, ProgramSiteContextPatchEntry, ProgramStoragePatchEntry, ProgramSustainabilityPatchEntry, ProgramWayfindingPatchEntry } from "../🟦️.ts";

//#region 🚪️Parsers
export function parseProgramOperationsPatchEntry(value: unknown, at = "$"): ProgramOperationsPatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseOperationalRequirementPatch(row["patch"], `${at}.patch`),
  };
}

export function parseProgramEquipmentPatchEntry(value: unknown, at = "$"): ProgramEquipmentPatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseEquipmentPatch(row["patch"], `${at}.patch`),
  };
}

export function parseProgramResourcesPatchEntry(value: unknown, at = "$"): ProgramResourcesPatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseResourcePatch(row["patch"], `${at}.patch`),
  };
}

export function parseProgramStoragePatchEntry(value: unknown, at = "$"): ProgramStoragePatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseStorageRequirementPatch(row["patch"], `${at}.patch`),
  };
}

export function parseProgramEnvironmentalPatchEntry(value: unknown, at = "$"): ProgramEnvironmentalPatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseEnvironmentalRequirementPatch(row["patch"], `${at}.patch`),
  };
}

export function parseProgramHumanFactorsPatchEntry(value: unknown, at = "$"): ProgramHumanFactorsPatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseHumanFactorRequirementPatch(row["patch"], `${at}.patch`),
  };
}

export function parseProgramAccessibilityPatchEntry(value: unknown, at = "$"): ProgramAccessibilityPatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseAccessibilityRequirementPatch(row["patch"], `${at}.patch`),
  };
}

export function parseProgramPrivacyPatchEntry(value: unknown, at = "$"): ProgramPrivacyPatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parsePrivacyRequirementPatch(row["patch"], `${at}.patch`),
  };
}

export function parseProgramSafetyPatchEntry(value: unknown, at = "$"): ProgramSafetyPatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseSafetyRequirementPatch(row["patch"], `${at}.patch`),
  };
}

export function parseProgramSecurityPatchEntry(value: unknown, at = "$"): ProgramSecurityPatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseSecurityRequirementPatch(row["patch"], `${at}.patch`),
  };
}

export function parseProgramRegulatoryPatchEntry(value: unknown, at = "$"): ProgramRegulatoryPatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseRegulatoryRequirementPatch(row["patch"], `${at}.patch`),
  };
}

export function parseProgramSiteContextPatchEntry(value: unknown, at = "$"): ProgramSiteContextPatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseSiteContextPatch(row["patch"], `${at}.patch`),
  };
}

export function parseProgramOrganizationalPatchEntry(value: unknown, at = "$"): ProgramOrganizationalPatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseOrganizationalRequirementPatch(row["patch"], `${at}.patch`),
  };
}

export function parseProgramServicesPatchEntry(value: unknown, at = "$"): ProgramServicesPatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseServiceRequirementPatch(row["patch"], `${at}.patch`),
  };
}

export function parseProgramInfrastructurePatchEntry(value: unknown, at = "$"): ProgramInfrastructurePatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseInfrastructureRequirementPatch(row["patch"], `${at}.patch`),
  };
}

export function parseProgramInformationPatchEntry(value: unknown, at = "$"): ProgramInformationPatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseInformationRequirementPatch(row["patch"], `${at}.patch`),
  };
}

export function parseProgramCommunicationPatchEntry(value: unknown, at = "$"): ProgramCommunicationPatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseCommunicationRequirementPatch(row["patch"], `${at}.patch`),
  };
}

export function parseProgramWayfindingPatchEntry(value: unknown, at = "$"): ProgramWayfindingPatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseWayfindingRequirementPatch(row["patch"], `${at}.patch`),
  };
}

export function parseProgramSchedulesPatchEntry(value: unknown, at = "$"): ProgramSchedulesPatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseScheduleRequirementPatch(row["patch"], `${at}.patch`),
  };
}

export function parseProgramFlexibilityPatchEntry(value: unknown, at = "$"): ProgramFlexibilityPatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseFlexibilityRequirementPatch(row["patch"], `${at}.patch`),
  };
}

export function parseProgramGrowthPatchEntry(value: unknown, at = "$"): ProgramGrowthPatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseGrowthPlanPatch(row["patch"], `${at}.patch`),
  };
}

export function parseProgramSustainabilityPatchEntry(value: unknown, at = "$"): ProgramSustainabilityPatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseSustainabilityRequirementPatch(row["patch"], `${at}.patch`),
  };
}

export function parseProgramResiliencePatchEntry(value: unknown, at = "$"): ProgramResiliencePatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseResilienceRequirementPatch(row["patch"], `${at}.patch`),
  };
}

export function parseProgramCostsPatchEntry(value: unknown, at = "$"): ProgramCostsPatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseCostRequirementPatch(row["patch"], `${at}.patch`),
  };
}

export function parseProgramDeliveryPatchEntry(value: unknown, at = "$"): ProgramDeliveryPatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseDeliveryConstraintPatch(row["patch"], `${at}.patch`),
  };
}
//#endregion 🚪️Parsers
