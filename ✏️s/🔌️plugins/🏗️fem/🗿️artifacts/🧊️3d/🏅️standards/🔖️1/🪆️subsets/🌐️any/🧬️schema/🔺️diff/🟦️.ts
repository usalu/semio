import {parseFemNode,parseFemElement,parseFemSolid,parseFemMaterial,parseFemSection,parseFemSupport,parseFemLoadCase,parseFemCombination} from "../📸️snapshot/🟦️.ts";
/** 🧬️ Fem3d diff schema — sparse field delta. */

import type {FemNode,FemDof,FemAxis,FemElement,FemMaterial,FemSection,FemSupport,FemLoad,FemLoadCase,FemSolid,FemCombination,FemAnalysisSettings} from "../📸️snapshot/🟦️.ts";
export type {FemNode,FemDof,FemAxis,FemElement,FemMaterial,FemSection,FemSupport,FemLoad,FemLoadCase,FemSolid,FemCombination,FemAnalysisSettings} from "../📸️snapshot/🟦️.ts";
import type {Fem3dArtifact} from "../🟦️.ts";
export type {Fem3dArtifact} from "../🟦️.ts";


export interface Fem3dDiff {
  /** @state artifact */
  artifact?: Fem3dArtifact;
  /** @state artifact */
  nodes?: Fem3dNodesDelta;
  /** @state artifact */
  elements?: Fem3dElementsDelta;
  /** @state artifact */
  materials?: Fem3dMaterialsDelta;
  /** @state artifact */
  sections?: Fem3dSectionsDelta;
  /** @state artifact */
  solids?: Fem3dSolidsDelta;
  /** @state artifact */
  supports?: Fem3dSupportsDelta;
  /** @state artifact */
  loadCases?: Fem3dLoadCasesDelta;
  /** @state artifact */
  combinations?: Fem3dCombinationsDelta;
  /** @state artifact */
  analysis?: FemAnalysisSettings;
}

export interface Fem3dNodesDelta {
  added: FemNode[];
  removed: string[];
  patched: Fem3dNodesPatchEntry[];
  reordered?: string[];
}

export interface Fem3dNodesPatchEntry {
  id: string;
  item: FemNode;
}

export interface Fem3dElementsDelta {
  added: FemElement[];
  removed: string[];
  patched: Fem3dElementsPatchEntry[];
  reordered?: string[];
}

export interface Fem3dElementsPatchEntry {
  id: string;
  item: FemElement;
}

export interface Fem3dMaterialsDelta {
  added: FemMaterial[];
  removed: string[];
  patched: Fem3dMaterialsPatchEntry[];
  reordered?: string[];
}

export interface Fem3dMaterialsPatchEntry {
  id: string;
  item: FemMaterial;
}

export interface Fem3dSectionsDelta {
  added: FemSection[];
  removed: string[];
  patched: Fem3dSectionsPatchEntry[];
  reordered?: string[];
}

export interface Fem3dSectionsPatchEntry {
  id: string;
  item: FemSection;
}

export interface Fem3dSolidsDelta {
  added: FemSolid[];
  removed: string[];
  patched: Fem3dSolidsPatchEntry[];
  reordered?: string[];
}

export interface Fem3dSolidsPatchEntry {
  id: string;
  item: FemSolid;
}

export interface Fem3dSupportsDelta {
  added: FemSupport[];
  removed: string[];
  patched: Fem3dSupportsPatchEntry[];
  reordered?: string[];
}

export interface Fem3dSupportsPatchEntry {
  id: string;
  item: FemSupport;
}

export interface Fem3dLoadCasesDelta {
  added: FemLoadCase[];
  removed: string[];
  patched: Fem3dLoadCasesPatchEntry[];
  reordered?: string[];
}

export interface Fem3dLoadCasesPatchEntry {
  id: string;
  item: FemLoadCase;
}

export interface Fem3dCombinationsDelta {
  added: FemCombination[];
  removed: string[];
  patched: Fem3dCombinationsPatchEntry[];
  reordered?: string[];
}

export interface Fem3dCombinationsPatchEntry {
  id: string;
  item: FemCombination;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class femFem3dDiffGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const femFem3dDiffGuardReject = (at: string, why: string): never => {
  throw new femFem3dDiffGuardRefusal(at, why);
};

type femFem3dDiffGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type femFem3dDiffGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type femFem3dDiffGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const femFem3dDiffGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : femFem3dDiffGuardReject(at, "value is not an object");
export const femFem3dDiffGuardArray = (value: unknown, at: string, bounds: femFem3dDiffGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return femFem3dDiffGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) femFem3dDiffGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) femFem3dDiffGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const femFem3dDiffGuardString = (value: unknown, at: string, bounds: femFem3dDiffGuardTextBounds = {}): string => {
  if (typeof value !== "string") return femFem3dDiffGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) femFem3dDiffGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) femFem3dDiffGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) femFem3dDiffGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const femFem3dDiffGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : femFem3dDiffGuardReject(at, "value is not a boolean"));
export const femFem3dDiffGuardNumber = (value: unknown, at: string, bounds: femFem3dDiffGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return femFem3dDiffGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) femFem3dDiffGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) femFem3dDiffGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const femFem3dDiffGuardInteger = (value: unknown, at: string, bounds: femFem3dDiffGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? femFem3dDiffGuardNumber(value, at, bounds) : femFem3dDiffGuardReject(at, "value is not an integer");
export const femFem3dDiffGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : femFem3dDiffGuardReject(at, `value is not one of ${members.join(", ")}`);
export const femFem3dDiffGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : femFem3dDiffGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

/** 🩹️ Admit the declared patch identity and complete typed replacement. */
function parseFemPatch<T>(value:unknown,at:string,parse:(value:unknown,at:string)=>T):{id:string;item:T}{const row=femFem3dDiffGuardObject(value,at);for(const key of Object.keys(row))if(key!=="id"&&key!=="item")throw Error(`${at}: unknown patch field`);return{id:femFem3dDiffGuardString(row.id,`${at}.id`),item:parse(row.item,`${at}.item`)}}

export function parseFem3dNodesDelta(value: unknown, at = "$"): Fem3dNodesDelta {
  const row = femFem3dDiffGuardObject(value, at);
  return {
    added: femFem3dDiffGuardArray(row["added"], `${at}.added`).map((item, index) => parseFemNode(item, `${at}.added[${index}]`)),
    removed: femFem3dDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => femFem3dDiffGuardString(item, `${at}.removed[${index}]`)),
    patched: femFem3dDiffGuardArray(row["patched"], `${at}.patched`).map((item, index) => parseFemPatch(item, `${at}.patched[${index}]`, parseFemNode)),
    reordered: row["reordered"] === undefined ? undefined : femFem3dDiffGuardArray(row["reordered"], `${at}.reordered`).map((item, index) => femFem3dDiffGuardString(item, `${at}.reordered[${index}]`)),
  };
}

export function parseFem3dElementsDelta(value: unknown, at = "$"): Fem3dElementsDelta {
  const row = femFem3dDiffGuardObject(value, at);
  return {
    added: femFem3dDiffGuardArray(row["added"], `${at}.added`).map((item, index) => parseFemElement(item, `${at}.added[${index}]`)),
    removed: femFem3dDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => femFem3dDiffGuardString(item, `${at}.removed[${index}]`)),
    patched: femFem3dDiffGuardArray(row["patched"], `${at}.patched`).map((item, index) => parseFemPatch(item, `${at}.patched[${index}]`, parseFemElement)),
    reordered: row["reordered"] === undefined ? undefined : femFem3dDiffGuardArray(row["reordered"], `${at}.reordered`).map((item, index) => femFem3dDiffGuardString(item, `${at}.reordered[${index}]`)),
  };
}

export function parseFem3dMaterialsDelta(value: unknown, at = "$"): Fem3dMaterialsDelta {
  const row = femFem3dDiffGuardObject(value, at);
  return {
    added: femFem3dDiffGuardArray(row["added"], `${at}.added`).map((item, index) => parseFemMaterial(item, `${at}.added[${index}]`)),
    removed: femFem3dDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => femFem3dDiffGuardString(item, `${at}.removed[${index}]`)),
    patched: femFem3dDiffGuardArray(row["patched"], `${at}.patched`).map((item, index) => parseFemPatch(item, `${at}.patched[${index}]`, parseFemMaterial)),
    reordered: row["reordered"] === undefined ? undefined : femFem3dDiffGuardArray(row["reordered"], `${at}.reordered`).map((item, index) => femFem3dDiffGuardString(item, `${at}.reordered[${index}]`)),
  };
}

export function parseFem3dSectionsDelta(value: unknown, at = "$"): Fem3dSectionsDelta {
  const row = femFem3dDiffGuardObject(value, at);
  return {
    added: femFem3dDiffGuardArray(row["added"], `${at}.added`).map((item, index) => parseFemSection(item, `${at}.added[${index}]`)),
    removed: femFem3dDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => femFem3dDiffGuardString(item, `${at}.removed[${index}]`)),
    patched: femFem3dDiffGuardArray(row["patched"], `${at}.patched`).map((item, index) => parseFemPatch(item, `${at}.patched[${index}]`, parseFemSection)),
    reordered: row["reordered"] === undefined ? undefined : femFem3dDiffGuardArray(row["reordered"], `${at}.reordered`).map((item, index) => femFem3dDiffGuardString(item, `${at}.reordered[${index}]`)),
  };
}

export function parseFem3dSolidsDelta(value: unknown, at = "$"): Fem3dSolidsDelta {
  const row = femFem3dDiffGuardObject(value, at);
  return {
    added: femFem3dDiffGuardArray(row["added"], `${at}.added`).map((item, index) => parseFemSolid(item, `${at}.added[${index}]`)),
    removed: femFem3dDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => femFem3dDiffGuardString(item, `${at}.removed[${index}]`)),
    patched: femFem3dDiffGuardArray(row["patched"], `${at}.patched`).map((item, index) => parseFemPatch(item, `${at}.patched[${index}]`, parseFemSolid)),
    reordered: row["reordered"] === undefined ? undefined : femFem3dDiffGuardArray(row["reordered"], `${at}.reordered`).map((item, index) => femFem3dDiffGuardString(item, `${at}.reordered[${index}]`)),
  };
}

export function parseFem3dSupportsDelta(value: unknown, at = "$"): Fem3dSupportsDelta {
  const row = femFem3dDiffGuardObject(value, at);
  return {
    added: femFem3dDiffGuardArray(row["added"], `${at}.added`).map((item, index) => parseFemSupport(item, `${at}.added[${index}]`)),
    removed: femFem3dDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => femFem3dDiffGuardString(item, `${at}.removed[${index}]`)),
    patched: femFem3dDiffGuardArray(row["patched"], `${at}.patched`).map((item, index) => parseFemPatch(item, `${at}.patched[${index}]`, parseFemSupport)),
    reordered: row["reordered"] === undefined ? undefined : femFem3dDiffGuardArray(row["reordered"], `${at}.reordered`).map((item, index) => femFem3dDiffGuardString(item, `${at}.reordered[${index}]`)),
  };
}

export function parseFem3dLoadCasesDelta(value: unknown, at = "$"): Fem3dLoadCasesDelta {
  const row = femFem3dDiffGuardObject(value, at);
  return {
    added: femFem3dDiffGuardArray(row["added"], `${at}.added`).map((item, index) => parseFemLoadCase(item, `${at}.added[${index}]`)),
    removed: femFem3dDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => femFem3dDiffGuardString(item, `${at}.removed[${index}]`)),
    patched: femFem3dDiffGuardArray(row["patched"], `${at}.patched`).map((item, index) => parseFemPatch(item, `${at}.patched[${index}]`, parseFemLoadCase)),
    reordered: row["reordered"] === undefined ? undefined : femFem3dDiffGuardArray(row["reordered"], `${at}.reordered`).map((item, index) => femFem3dDiffGuardString(item, `${at}.reordered[${index}]`)),
  };
}

export function parseFem3dCombinationsDelta(value: unknown, at = "$"): Fem3dCombinationsDelta {
  const row = femFem3dDiffGuardObject(value, at);
  return {
    added: femFem3dDiffGuardArray(row["added"], `${at}.added`).map((item, index) => parseFemCombination(item, `${at}.added[${index}]`)),
    removed: femFem3dDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => femFem3dDiffGuardString(item, `${at}.removed[${index}]`)),
    patched: femFem3dDiffGuardArray(row["patched"], `${at}.patched`).map((item, index) => parseFemPatch(item, `${at}.patched[${index}]`, parseFemCombination)),
    reordered: row["reordered"] === undefined ? undefined : femFem3dDiffGuardArray(row["reordered"], `${at}.reordered`).map((item, index) => femFem3dDiffGuardString(item, `${at}.reordered[${index}]`)),
  };
}
