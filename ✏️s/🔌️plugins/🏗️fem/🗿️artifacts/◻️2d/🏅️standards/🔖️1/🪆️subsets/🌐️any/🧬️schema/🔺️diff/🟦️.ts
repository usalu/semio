import {parseFemNode,parseFemElement,parseFemRegion,parseFemMaterial,parseFemSection,parseFemSupport,parseFemLoad,parseFemLoadCase,parseFemCombination} from "../📸️snapshot/🟦️.ts";
/** 🧬️ Fem2d diff schema — sparse field delta. */

import type {FemNode,FemElement,FemMaterial,FemSection,FemSupport,FemLoad,FemLoadCase,FemRegion,FemCombination} from "../📸️snapshot/🟦️.ts";
export type {FemNode,FemDof,FemElement,FemMaterial,FemSection,FemSupport,FemLoad,FemLoadCase,FemRegion,FemCombinationTerm,FemCombination,FemAnalysisSettings} from "../📸️snapshot/🟦️.ts";


export interface Fem2dDiff {
  /** @state artifact */
  nodes?: Fem2dNodesDelta;
  /** @state artifact */
  elements?: Fem2dElementsDelta;
  /** @state artifact */
  regions?: Fem2dRegionsDelta;
  /** @state artifact */
  materials?: Fem2dMaterialsDelta;
  /** @state artifact */
  sections?: Fem2dSectionsDelta;
  /** @state artifact */
  supports?: Fem2dSupportsDelta;
  /** @state artifact */
  loadCases?: Fem2dLoadCasesDelta;
  /** @state artifact */
  combinations?: Fem2dCombinationsDelta;
  /** @state artifact */
  analysis?: Fem2dAnalysisPatch;
  /** @state config */
}

export interface Fem2dNodesDelta {
  added: FemNode[];
  removed: string[];
  patched: Fem2dNodesPatchEntry[];
  reordered?: string[];
}

export interface Fem2dNodesPatchEntry {
  id: string;
  item: FemNode;
}

export interface Fem2dElementsDelta {
  added: FemElement[];
  removed: string[];
  patched: Fem2dElementsPatchEntry[];
  reordered?: string[];
}

export interface Fem2dElementsPatchEntry {
  id: string;
  item: FemElement;
}

export interface Fem2dRegionsDelta {
  added: FemRegion[];
  removed: string[];
  patched: Fem2dRegionsPatchEntry[];
  reordered?: string[];
}

export interface Fem2dRegionsPatchEntry {
  id: string;
  item: FemRegion;
}

export interface Fem2dMaterialsDelta {
  added: FemMaterial[];
  removed: string[];
  patched: Fem2dMaterialsPatchEntry[];
  reordered?: string[];
}

export interface Fem2dMaterialsPatchEntry {
  id: string;
  item: FemMaterial;
}

export interface Fem2dSectionsDelta {
  added: FemSection[];
  removed: string[];
  patched: Fem2dSectionsPatchEntry[];
  reordered?: string[];
}

export interface Fem2dSectionsPatchEntry {
  id: string;
  item: FemSection;
}

export interface Fem2dSupportsDelta {
  added: FemSupport[];
  removed: string[];
  patched: Fem2dSupportsPatchEntry[];
  reordered?: string[];
}

export interface Fem2dSupportsPatchEntry {
  id: string;
  item: FemSupport;
}

export interface Fem2dLoadCasesDelta {
  added: FemLoadCase[];
  removed: string[];
  patched: Fem2dLoadCasesPatchEntry[];
  reordered?: string[];
}

export interface Fem2dLoadCasesPatchEntry {
  id: string;
  patch: Fem2dLoadCasePatch;
}

export interface Fem2dLoadCasePatch {
  name?: string;
  selfWeight?: boolean;
  loads?: Fem2dLoadsDelta;
}

export interface Fem2dLoadsDelta {
  added: FemLoad[];
  removed: string[];
  patched: Fem2dLoadsPatchEntry[];
  reordered?: string[];
}

export interface Fem2dLoadsPatchEntry {
  id: string;
  item: FemLoad;
}

export interface Fem2dAnalysisPatch {
  modalCount?: number;
  bucklingCount?: number;
  deformationScale?: number;
}

export interface Fem2dCombinationsDelta {
  added: FemCombination[];
  removed: string[];
  patched: Fem2dCombinationsPatchEntry[];
  reordered?: string[];
}

export interface Fem2dCombinationsPatchEntry {
  id: string;
  item: FemCombination;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class femFem2dDiffGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const femFem2dDiffGuardReject = (at: string, why: string): never => {
  throw new femFem2dDiffGuardRefusal(at, why);
};

type femFem2dDiffGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type femFem2dDiffGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type femFem2dDiffGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const femFem2dDiffGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : femFem2dDiffGuardReject(at, "value is not an object");
export const femFem2dDiffGuardArray = (value: unknown, at: string, bounds: femFem2dDiffGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return femFem2dDiffGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) femFem2dDiffGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) femFem2dDiffGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const femFem2dDiffGuardString = (value: unknown, at: string, bounds: femFem2dDiffGuardTextBounds = {}): string => {
  if (typeof value !== "string") return femFem2dDiffGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) femFem2dDiffGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) femFem2dDiffGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) femFem2dDiffGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const femFem2dDiffGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : femFem2dDiffGuardReject(at, "value is not a boolean"));
export const femFem2dDiffGuardNumber = (value: unknown, at: string, bounds: femFem2dDiffGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return femFem2dDiffGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) femFem2dDiffGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) femFem2dDiffGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const femFem2dDiffGuardInteger = (value: unknown, at: string, bounds: femFem2dDiffGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? femFem2dDiffGuardNumber(value, at, bounds) : femFem2dDiffGuardReject(at, "value is not an integer");
export const femFem2dDiffGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : femFem2dDiffGuardReject(at, `value is not one of ${members.join(", ")}`);
export const femFem2dDiffGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : femFem2dDiffGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

/** 🩹️ Admit the declared patch identity and complete typed replacement. */
function parseFemPatch<T>(value:unknown,at:string,parse:(value:unknown,at:string)=>T):{id:string;item:T}{const row=femFem2dDiffGuardObject(value,at);for(const key of Object.keys(row))if(key!=="id"&&key!=="item")throw Error(`${at}: unknown patch field`);return{id:femFem2dDiffGuardString(row.id,`${at}.id`),item:parse(row.item,`${at}.item`)}}

export function parseFem2dNodesDelta(value: unknown, at = "$"): Fem2dNodesDelta {
  const row = femFem2dDiffGuardObject(value, at);
  return {
    added: femFem2dDiffGuardArray(row["added"], `${at}.added`).map((item, index) => parseFemNode(item, `${at}.added[${index}]`)),
    removed: femFem2dDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => femFem2dDiffGuardString(item, `${at}.removed[${index}]`)),
    patched: femFem2dDiffGuardArray(row["patched"], `${at}.patched`).map((item, index) => parseFemPatch(item, `${at}.patched[${index}]`, parseFemNode)),
    reordered: row["reordered"] === undefined ? undefined : femFem2dDiffGuardArray(row["reordered"], `${at}.reordered`).map((item, index) => femFem2dDiffGuardString(item, `${at}.reordered[${index}]`)),
  };
}

export function parseFem2dElementsDelta(value: unknown, at = "$"): Fem2dElementsDelta {
  const row = femFem2dDiffGuardObject(value, at);
  return {
    added: femFem2dDiffGuardArray(row["added"], `${at}.added`).map((item, index) => parseFemElement(item, `${at}.added[${index}]`)),
    removed: femFem2dDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => femFem2dDiffGuardString(item, `${at}.removed[${index}]`)),
    patched: femFem2dDiffGuardArray(row["patched"], `${at}.patched`).map((item, index) => parseFemPatch(item, `${at}.patched[${index}]`, parseFemElement)),
    reordered: row["reordered"] === undefined ? undefined : femFem2dDiffGuardArray(row["reordered"], `${at}.reordered`).map((item, index) => femFem2dDiffGuardString(item, `${at}.reordered[${index}]`)),
  };
}

export function parseFem2dRegionsDelta(value: unknown, at = "$"): Fem2dRegionsDelta {
  const row = femFem2dDiffGuardObject(value, at);
  return {
    added: femFem2dDiffGuardArray(row["added"], `${at}.added`).map((item, index) => parseFemRegion(item, `${at}.added[${index}]`)),
    removed: femFem2dDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => femFem2dDiffGuardString(item, `${at}.removed[${index}]`)),
    patched: femFem2dDiffGuardArray(row["patched"], `${at}.patched`).map((item, index) => parseFemPatch(item, `${at}.patched[${index}]`, parseFemRegion)),
    reordered: row["reordered"] === undefined ? undefined : femFem2dDiffGuardArray(row["reordered"], `${at}.reordered`).map((item, index) => femFem2dDiffGuardString(item, `${at}.reordered[${index}]`)),
  };
}

export function parseFem2dMaterialsDelta(value: unknown, at = "$"): Fem2dMaterialsDelta {
  const row = femFem2dDiffGuardObject(value, at);
  return {
    added: femFem2dDiffGuardArray(row["added"], `${at}.added`).map((item, index) => parseFemMaterial(item, `${at}.added[${index}]`)),
    removed: femFem2dDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => femFem2dDiffGuardString(item, `${at}.removed[${index}]`)),
    patched: femFem2dDiffGuardArray(row["patched"], `${at}.patched`).map((item, index) => parseFemPatch(item, `${at}.patched[${index}]`, parseFemMaterial)),
    reordered: row["reordered"] === undefined ? undefined : femFem2dDiffGuardArray(row["reordered"], `${at}.reordered`).map((item, index) => femFem2dDiffGuardString(item, `${at}.reordered[${index}]`)),
  };
}

export function parseFem2dSectionsDelta(value: unknown, at = "$"): Fem2dSectionsDelta {
  const row = femFem2dDiffGuardObject(value, at);
  return {
    added: femFem2dDiffGuardArray(row["added"], `${at}.added`).map((item, index) => parseFemSection(item, `${at}.added[${index}]`)),
    removed: femFem2dDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => femFem2dDiffGuardString(item, `${at}.removed[${index}]`)),
    patched: femFem2dDiffGuardArray(row["patched"], `${at}.patched`).map((item, index) => parseFemPatch(item, `${at}.patched[${index}]`, parseFemSection)),
    reordered: row["reordered"] === undefined ? undefined : femFem2dDiffGuardArray(row["reordered"], `${at}.reordered`).map((item, index) => femFem2dDiffGuardString(item, `${at}.reordered[${index}]`)),
  };
}

export function parseFem2dSupportsDelta(value: unknown, at = "$"): Fem2dSupportsDelta {
  const row = femFem2dDiffGuardObject(value, at);
  return {
    added: femFem2dDiffGuardArray(row["added"], `${at}.added`).map((item, index) => parseFemSupport(item, `${at}.added[${index}]`)),
    removed: femFem2dDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => femFem2dDiffGuardString(item, `${at}.removed[${index}]`)),
    patched: femFem2dDiffGuardArray(row["patched"], `${at}.patched`).map((item, index) => parseFemPatch(item, `${at}.patched[${index}]`, parseFemSupport)),
    reordered: row["reordered"] === undefined ? undefined : femFem2dDiffGuardArray(row["reordered"], `${at}.reordered`).map((item, index) => femFem2dDiffGuardString(item, `${at}.reordered[${index}]`)),
  };
}

export function parseFem2dLoadCasesDelta(value: unknown, at = "$"): Fem2dLoadCasesDelta {
  const row = femFem2dDiffGuardObject(value, at);
  return {
    added: femFem2dDiffGuardArray(row["added"], `${at}.added`).map((item, index) => parseFemLoadCase(item, `${at}.added[${index}]`)),
    removed: femFem2dDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => femFem2dDiffGuardString(item, `${at}.removed[${index}]`)),
    patched: femFem2dDiffGuardArray(row["patched"], `${at}.patched`).map((item, index) => parseFem2dLoadCasesPatchEntry(item, `${at}.patched[${index}]`)),
    reordered: row["reordered"] === undefined ? undefined : femFem2dDiffGuardArray(row["reordered"], `${at}.reordered`).map((item, index) => femFem2dDiffGuardString(item, `${at}.reordered[${index}]`)),
  };
}

export function parseFem2dCombinationsDelta(value: unknown, at = "$"): Fem2dCombinationsDelta {
  const row = femFem2dDiffGuardObject(value, at);
  return {
    added: femFem2dDiffGuardArray(row["added"], `${at}.added`).map((item, index) => parseFemCombination(item, `${at}.added[${index}]`)),
    removed: femFem2dDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => femFem2dDiffGuardString(item, `${at}.removed[${index}]`)),
    patched: femFem2dDiffGuardArray(row["patched"], `${at}.patched`).map((item, index) => parseFemPatch(item, `${at}.patched[${index}]`, parseFemCombination)),
    reordered: row["reordered"] === undefined ? undefined : femFem2dDiffGuardArray(row["reordered"], `${at}.reordered`).map((item, index) => femFem2dDiffGuardString(item, `${at}.reordered[${index}]`)),
  };
}

export function parseFem2dLoadCasesPatchEntry(value: unknown, at = "$"): Fem2dLoadCasesPatchEntry {
  const row = femFem2dDiffGuardObject(value, at);
  return { id: femFem2dDiffGuardString(row["id"], `${at}.id`), patch: parseFem2dLoadCasePatch(row["patch"], `${at}.patch`) };
}

export function parseFem2dLoadCasePatch(value: unknown, at = "$"): Fem2dLoadCasePatch {
  const row = femFem2dDiffGuardObject(value, at);
  return {
    name: row["name"] === undefined || row["name"] === null ? undefined : femFem2dDiffGuardString(row["name"], `${at}.name`),
    selfWeight: row["selfWeight"] === undefined || row["selfWeight"] === null ? undefined : femFem2dDiffGuardBoolean(row["selfWeight"], `${at}.selfWeight`),
    loads: row["loads"] === undefined || row["loads"] === null ? undefined : parseFem2dLoadsDelta(row["loads"], `${at}.loads`),
  };
}

export function parseFem2dLoadsDelta(value: unknown, at = "$"): Fem2dLoadsDelta {
  const row = femFem2dDiffGuardObject(value, at);
  return {
    added: femFem2dDiffGuardArray(row["added"], `${at}.added`).map((item, index) => parseFemLoad(item, `${at}.added[${index}]`)),
    removed: femFem2dDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => femFem2dDiffGuardString(item, `${at}.removed[${index}]`)),
    patched: femFem2dDiffGuardArray(row["patched"], `${at}.patched`).map((item, index) => parseFemPatch(item, `${at}.patched[${index}]`, parseFemLoad)),
    reordered: row["reordered"] === undefined ? undefined : femFem2dDiffGuardArray(row["reordered"], `${at}.reordered`).map((item, index) => femFem2dDiffGuardString(item, `${at}.reordered[${index}]`)),
  };
}

export function parseFem2dAnalysisPatch(value: unknown, at = "$"): Fem2dAnalysisPatch {
  const row = femFem2dDiffGuardObject(value, at);
  return {
    modalCount: row["modalCount"] === undefined || row["modalCount"] === null ? undefined : femFem2dDiffGuardInteger(row["modalCount"], `${at}.modalCount`, { minimum: 1 }),
    bucklingCount: row["bucklingCount"] === undefined || row["bucklingCount"] === null ? undefined : femFem2dDiffGuardInteger(row["bucklingCount"], `${at}.bucklingCount`, { minimum: 1 }),
    deformationScale: row["deformationScale"] === undefined || row["deformationScale"] === null ? undefined : femFem2dDiffGuardNumber(row["deformationScale"], `${at}.deformationScale`),
  };
}
