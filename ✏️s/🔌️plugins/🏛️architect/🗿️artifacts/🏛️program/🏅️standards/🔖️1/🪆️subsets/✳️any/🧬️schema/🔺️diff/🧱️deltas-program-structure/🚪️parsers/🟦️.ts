/** 🚪️ The typed `parse<Export>` entry points projected from the schema for the types declared next to this folder.
 * @see ../🟦️.ts
 */
import { architectProgramDiffGuardClosed, architectProgramDiffGuardObject, architectProgramDiffGuardString } from "../../🟦️.ts";
import { parseAccessRulePatch, parseActivityPatch, parseAdjacencyPatch, parseFlowRequirementPatch, parseFunctionPatch, parseProcessPatch, parseProgramElementPatch, parseQuantityRequirementPatch, parseRelationshipPatch, parseStakeholderPatch, parseUserProfilePatch } from "../../🩹️patch-types/🚪️parsers/🟦️.ts";
import type { ProgramAccessRulesPatchEntry, ProgramActivitiesPatchEntry, ProgramAdjacenciesPatchEntry, ProgramElementsPatchEntry, ProgramFlowsPatchEntry, ProgramFunctionsPatchEntry, ProgramProcessesPatchEntry, ProgramQuantitiesPatchEntry, ProgramRelationshipsPatchEntry, ProgramStakeholdersPatchEntry, ProgramUsersPatchEntry } from "../🟦️.ts";

//#region 🚪️Parsers
export function parseProgramStakeholdersPatchEntry(value: unknown, at = "$"): ProgramStakeholdersPatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseStakeholderPatch(row["patch"], `${at}.patch`),
  };
}

export function parseProgramUsersPatchEntry(value: unknown, at = "$"): ProgramUsersPatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseUserProfilePatch(row["patch"], `${at}.patch`),
  };
}

export function parseProgramActivitiesPatchEntry(value: unknown, at = "$"): ProgramActivitiesPatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseActivityPatch(row["patch"], `${at}.patch`),
  };
}

export function parseProgramFunctionsPatchEntry(value: unknown, at = "$"): ProgramFunctionsPatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseFunctionPatch(row["patch"], `${at}.patch`),
  };
}

export function parseProgramElementsPatchEntry(value: unknown, at = "$"): ProgramElementsPatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseProgramElementPatch(row["patch"], `${at}.patch`),
  };
}

export function parseProgramQuantitiesPatchEntry(value: unknown, at = "$"): ProgramQuantitiesPatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseQuantityRequirementPatch(row["patch"], `${at}.patch`),
  };
}

export function parseProgramRelationshipsPatchEntry(value: unknown, at = "$"): ProgramRelationshipsPatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseRelationshipPatch(row["patch"], `${at}.patch`),
  };
}

export function parseProgramAdjacenciesPatchEntry(value: unknown, at = "$"): ProgramAdjacenciesPatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseAdjacencyPatch(row["patch"], `${at}.patch`),
  };
}

export function parseProgramProcessesPatchEntry(value: unknown, at = "$"): ProgramProcessesPatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseProcessPatch(row["patch"], `${at}.patch`),
  };
}

export function parseProgramFlowsPatchEntry(value: unknown, at = "$"): ProgramFlowsPatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseFlowRequirementPatch(row["patch"], `${at}.patch`),
  };
}

export function parseProgramAccessRulesPatchEntry(value: unknown, at = "$"): ProgramAccessRulesPatchEntry {
  const row = architectProgramDiffGuardClosed(architectProgramDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: architectProgramDiffGuardString(row["id"], `${at}.id`),
    patch: parseAccessRulePatch(row["patch"], `${at}.patch`),
  };
}
//#endregion 🚪️Parsers
