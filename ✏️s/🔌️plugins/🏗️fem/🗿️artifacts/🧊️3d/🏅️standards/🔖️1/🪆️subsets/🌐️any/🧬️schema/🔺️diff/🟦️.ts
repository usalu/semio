/** 🩹️ Tagged sparse load fields retain the current variant as a precondition. */
export type Fem3dLoadPatch =
  | { kind: "nodal"; nodeId?: string; dof?: FemDof; value?: Binary64 }
  | { kind: "memberUdl"; elementId?: string; wx?: Binary64; wy?: Binary64; wz?: Binary64 }
  | { kind: "area"; solidId?: string; pressure?: Binary64 };
import { parseMapDelta, type MapDelta } from "../../../../../../../../../../../🧰️framework/🔨️modules/📡️replication/🎮️mutation/🗂️map/🧬️schema/🟦️.ts";
export type Fem3dCombinationTermsDelta = MapDelta<Binary64>;
/** 🩹️ Combination edits own a name or explicitly keyed term changes. */
export interface Fem3dCombinationPatch { name?: string; terms?: Fem3dCombinationTermsDelta; }

/** 🩹️ Sparse element fields preserve independent edits. */
export interface Fem3dElementPatch {
  kind?: "bar" | "frame";
  start?: string;
  end?: string;
  materialId?: string;
  sectionId?: string;
  roll?: Binary64;
}

/** 🩹️ Sparse support fields preserve independent edits. */
export interface Fem3dSupportPatch {
  nodeId?: string;
  fixed?: FemDof[];
  tx?: boolean;
  ty?: boolean;
  tz?: boolean;
  rx?: boolean;
  ry?: boolean;
  rz?: boolean;
}

/** 🩹️ Sparse solid fields preserve authored field ownership. */
export type Fem3dSolidPatch = Partial<Omit<FemSolid, "id">>;

/** 🩹️ Sparse section fields preserve stable identity. */
export interface Fem3dSectionPatch {
  name?: string;
  area?: Binary64;
  iy?: Binary64;
  iz?: Binary64;
  j?: Binary64;
}

/** 🩹️ Sparse node fields preserve stable identity. */
export interface Fem3dNodePatch {
  x?: Binary64;
  y?: Binary64;
  z?: Binary64;
}

import { type Binary64, parseBinary64, binary64Value } from "../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🔢️ieee754/🟦️.ts";
/** 🧱️ Sparse material field changes; stable identity belongs to the modification key. */
export interface Fem3dMaterialPatch {
  name?: string;
  e?: Binary64;
  nu?: Binary64;
  rho?: Binary64;
  g?: Binary64;
}

import {parseFemCount,parseFemFixedDofs,parseFemNode,parseFemElement,parseFemSolid,parseFemMaterial,parseFemSection,parseFemSupport,parseFemLoad,parseFemLoadCase,parseFemCombination} from "../📸️snapshot/🟦️.ts";
/** 🧬️ Fem3d diff schema — sparse field delta. */

import type {FemDof,FemNode,FemElement,FemMaterial,FemSection,FemSupport,FemLoad,FemLoadCase,FemSolid,FemCombination} from "../📸️snapshot/🟦️.ts";
export type {FemNode,FemDof,FemAxis,FemElement,FemMaterial,FemSection,FemSupport,FemLoad,FemLoadCase,FemSolid,FemCombination,FemAnalysisSettings} from "../📸️snapshot/🟦️.ts";


export interface Fem3dDiff {
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
  analysis?: Fem3dAnalysisPatch;
}

export interface Fem3dNodeRemoval {
  id: string;
  index: number;
}

export interface Fem3dNodeInsertion {
  index: number;
  row: FemNode;
}

export interface Fem3dNodeRelocation {
  id: string;
  from: number;
  to: number;
}

export interface Fem3dNodesModification {
  id: string;
  patch: Fem3dNodePatch;
}

export interface Fem3dNodesDelta {
  removed: Fem3dNodeRemoval[];
  inserted: Fem3dNodeInsertion[];
  moved: Fem3dNodeRelocation[];
  modified: Fem3dNodesModification[];
}

export interface Fem3dElementRemoval {
  id: string;
  index: number;
}

export interface Fem3dElementInsertion {
  index: number;
  row: FemElement;
}

export interface Fem3dElementRelocation {
  id: string;
  from: number;
  to: number;
}

export interface Fem3dElementsModification {
  id: string;
  patch: Fem3dElementPatch;
}

export interface Fem3dElementsDelta {
  removed: Fem3dElementRemoval[];
  inserted: Fem3dElementInsertion[];
  moved: Fem3dElementRelocation[];
  modified: Fem3dElementsModification[];
}

export interface Fem3dMaterialRemoval {
  id: string;
  index: number;
}

export interface Fem3dMaterialInsertion {
  index: number;
  row: FemMaterial;
}

export interface Fem3dMaterialRelocation {
  id: string;
  from: number;
  to: number;
}

export interface Fem3dMaterialsModification {
  id: string;
  patch: Fem3dMaterialPatch;
}

export interface Fem3dMaterialsDelta {
  removed: Fem3dMaterialRemoval[];
  inserted: Fem3dMaterialInsertion[];
  moved: Fem3dMaterialRelocation[];
  modified: Fem3dMaterialsModification[];
}

export interface Fem3dSectionRemoval {
  id: string;
  index: number;
}

export interface Fem3dSectionInsertion {
  index: number;
  row: FemSection;
}

export interface Fem3dSectionRelocation {
  id: string;
  from: number;
  to: number;
}

export interface Fem3dSectionsModification {
  id: string;
  patch: Fem3dSectionPatch;
}

export interface Fem3dSectionsDelta {
  removed: Fem3dSectionRemoval[];
  inserted: Fem3dSectionInsertion[];
  moved: Fem3dSectionRelocation[];
  modified: Fem3dSectionsModification[];
}

export interface Fem3dSolidRemoval {
  id: string;
  index: number;
}

export interface Fem3dSolidInsertion {
  index: number;
  row: FemSolid;
}

export interface Fem3dSolidRelocation {
  id: string;
  from: number;
  to: number;
}

export interface Fem3dSolidsModification {
  id: string;
  patch: Fem3dSolidPatch;
}

export interface Fem3dSolidsDelta {
  removed: Fem3dSolidRemoval[];
  inserted: Fem3dSolidInsertion[];
  moved: Fem3dSolidRelocation[];
  modified: Fem3dSolidsModification[];
}

export interface Fem3dSupportRemoval {
  id: string;
  index: number;
}

export interface Fem3dSupportInsertion {
  index: number;
  row: FemSupport;
}

export interface Fem3dSupportRelocation {
  id: string;
  from: number;
  to: number;
}

export interface Fem3dSupportsModification {
  id: string;
  patch: Fem3dSupportPatch;
}

export interface Fem3dSupportsDelta {
  removed: Fem3dSupportRemoval[];
  inserted: Fem3dSupportInsertion[];
  moved: Fem3dSupportRelocation[];
  modified: Fem3dSupportsModification[];
}

export interface Fem3dLoadCaseRemoval {
  id: string;
  index: number;
}

export interface Fem3dLoadCaseInsertion {
  index: number;
  row: FemLoadCase;
}

export interface Fem3dLoadCaseRelocation {
  id: string;
  from: number;
  to: number;
}

export interface Fem3dLoadCasesModification {
  id: string;
  patch: Fem3dLoadCasePatch;
}

export interface Fem3dLoadCasesDelta {
  removed: Fem3dLoadCaseRemoval[];
  inserted: Fem3dLoadCaseInsertion[];
  moved: Fem3dLoadCaseRelocation[];
  modified: Fem3dLoadCasesModification[];
}

export interface Fem3dLoadCasePatch {
  name?: string;
  selfWeight?: boolean;
  loads?: Fem3dLoadsDelta;
}

export interface Fem3dLoadRemoval {
  id: string;
  index: number;
}

export interface Fem3dLoadInsertion {
  index: number;
  row: FemLoad;
}

export interface Fem3dLoadRelocation {
  id: string;
  from: number;
  to: number;
}

export interface Fem3dLoadsModification {
  id: string;
  patch: Fem3dLoadPatch;
}

export interface Fem3dLoadsDelta {
  removed: Fem3dLoadRemoval[];
  inserted: Fem3dLoadInsertion[];
  moved: Fem3dLoadRelocation[];
  modified: Fem3dLoadsModification[];
}

export interface Fem3dAnalysisPatch {
  modalCount?: number;
  bucklingCount?: number;
  deformationScale?: number;
}

export interface Fem3dCombinationRemoval {
  id: string;
  index: number;
}

export interface Fem3dCombinationInsertion {
  index: number;
  row: FemCombination;
}

export interface Fem3dCombinationRelocation {
  id: string;
  from: number;
  to: number;
}

export interface Fem3dCombinationsModification {
  id: string;
  patch: Fem3dCombinationPatch;
}

export interface Fem3dCombinationsDelta {
  removed: Fem3dCombinationRemoval[];
  inserted: Fem3dCombinationInsertion[];
  moved: Fem3dCombinationRelocation[];
  modified: Fem3dCombinationsModification[];
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
export const femFem3dDiffGuardClosed = (row: Readonly<Record<string, unknown>>, at: string, keys: readonly string[]): Readonly<Record<string, unknown>> => {
  for (const key of Object.keys(row)) if (!keys.includes(key)) femFem3dDiffGuardReject(`${at}.${key}`, "unknown property");
  return row;
};
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

/** 🩹️ Admit the positional row shapes: removal `{id, index}`, insertion `{index, row}`, relocation `{id, from, to}` and the keyed modification `{id, patch}`. */
function parseFemRows<T>(value:unknown,at:string,keys:readonly string[],parse:(row:Readonly<Record<string,unknown>>,at:string)=>T):T[]{return femFem3dDiffGuardArray(value,at).map((item,index)=>{const here=`${at}[${index}]`,row=femFem3dDiffGuardObject(item,here);for(const key of Object.keys(row))if(!keys.includes(key))throw Error(`${here}: unknown field`);return parse(row,here)})}
const parseFemRemovals=(value:unknown,at:string)=>parseFemRows(value,at,["id","index"],(row,here)=>({id:femFem3dDiffGuardString(row.id,`${here}.id`),index:femFem3dDiffGuardInteger(row.index,`${here}.index`,{minimum:0})}));
const parseFemRelocations=(value:unknown,at:string)=>parseFemRows(value,at,["id","from","to"],(row,here)=>({id:femFem3dDiffGuardString(row.id,`${here}.id`),from:femFem3dDiffGuardInteger(row.from,`${here}.from`,{minimum:0}),to:femFem3dDiffGuardInteger(row.to,`${here}.to`,{minimum:0})}));
const parseFemInsertions=<T>(value:unknown,at:string,parse:(value:unknown,at:string)=>T)=>parseFemRows(value,at,["index","row"],(row,here)=>({index:femFem3dDiffGuardInteger(row.index,`${here}.index`,{minimum:0}),row:parse(row.row,`${here}.row`)}));
const parseFemModifications=<T>(value:unknown,at:string,parse:(value:unknown,at:string)=>T)=>parseFemRows(value,at,["id","patch"],(row,here)=>({id:femFem3dDiffGuardString(row.id,`${here}.id`),patch:parse(row.patch,`${here}.patch`)}));

export function parseFem3dNodesDelta(value: unknown, at = "$"): Fem3dNodesDelta {
  const row = femFem3dDiffGuardClosed(femFem3dDiffGuardObject(value, at), at, ["removed", "inserted", "moved", "modified"]);
  return {
    removed: femFem3dDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => parseFem3dNodeRemoval(item, `${at}.removed[${index}]`)),
    inserted: femFem3dDiffGuardArray(row["inserted"], `${at}.inserted`).map((item, index) => parseFem3dNodeInsertion(item, `${at}.inserted[${index}]`)),
    moved: femFem3dDiffGuardArray(row["moved"], `${at}.moved`).map((item, index) => parseFem3dNodeRelocation(item, `${at}.moved[${index}]`)),
    modified: femFem3dDiffGuardArray(row["modified"], `${at}.modified`).map((item, index) => parseFem3dNodesModification(item, `${at}.modified[${index}]`)),
  };
}

export function parseFem3dElementsDelta(value: unknown, at = "$"): Fem3dElementsDelta {
  const row = femFem3dDiffGuardClosed(femFem3dDiffGuardObject(value, at), at, ["removed", "inserted", "moved", "modified"]);
  return {
    removed: femFem3dDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => parseFem3dElementRemoval(item, `${at}.removed[${index}]`)),
    inserted: femFem3dDiffGuardArray(row["inserted"], `${at}.inserted`).map((item, index) => parseFem3dElementInsertion(item, `${at}.inserted[${index}]`)),
    moved: femFem3dDiffGuardArray(row["moved"], `${at}.moved`).map((item, index) => parseFem3dElementRelocation(item, `${at}.moved[${index}]`)),
    modified: femFem3dDiffGuardArray(row["modified"], `${at}.modified`).map((item, index) => parseFem3dElementsModification(item, `${at}.modified[${index}]`)),
  };
}

export function parseFem3dMaterialsDelta(value: unknown, at = "$"): Fem3dMaterialsDelta {
  const row = femFem3dDiffGuardClosed(femFem3dDiffGuardObject(value, at), at, ["removed", "inserted", "moved", "modified"]);
  return {
    removed: femFem3dDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => parseFem3dMaterialRemoval(item, `${at}.removed[${index}]`)),
    inserted: femFem3dDiffGuardArray(row["inserted"], `${at}.inserted`).map((item, index) => parseFem3dMaterialInsertion(item, `${at}.inserted[${index}]`)),
    moved: femFem3dDiffGuardArray(row["moved"], `${at}.moved`).map((item, index) => parseFem3dMaterialRelocation(item, `${at}.moved[${index}]`)),
    modified: femFem3dDiffGuardArray(row["modified"], `${at}.modified`).map((item, index) => parseFem3dMaterialsModification(item, `${at}.modified[${index}]`)),
  };
}

export function parseFem3dSectionsDelta(value: unknown, at = "$"): Fem3dSectionsDelta {
  const row = femFem3dDiffGuardClosed(femFem3dDiffGuardObject(value, at), at, ["removed", "inserted", "moved", "modified"]);
  return {
    removed: femFem3dDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => parseFem3dSectionRemoval(item, `${at}.removed[${index}]`)),
    inserted: femFem3dDiffGuardArray(row["inserted"], `${at}.inserted`).map((item, index) => parseFem3dSectionInsertion(item, `${at}.inserted[${index}]`)),
    moved: femFem3dDiffGuardArray(row["moved"], `${at}.moved`).map((item, index) => parseFem3dSectionRelocation(item, `${at}.moved[${index}]`)),
    modified: femFem3dDiffGuardArray(row["modified"], `${at}.modified`).map((item, index) => parseFem3dSectionsModification(item, `${at}.modified[${index}]`)),
  };
}

export function parseFem3dSolidsDelta(value: unknown, at = "$"): Fem3dSolidsDelta {
  const row = femFem3dDiffGuardClosed(femFem3dDiffGuardObject(value, at), at, ["removed", "inserted", "moved", "modified"]);
  return {
    removed: femFem3dDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => parseFem3dSolidRemoval(item, `${at}.removed[${index}]`)),
    inserted: femFem3dDiffGuardArray(row["inserted"], `${at}.inserted`).map((item, index) => parseFem3dSolidInsertion(item, `${at}.inserted[${index}]`)),
    moved: femFem3dDiffGuardArray(row["moved"], `${at}.moved`).map((item, index) => parseFem3dSolidRelocation(item, `${at}.moved[${index}]`)),
    modified: femFem3dDiffGuardArray(row["modified"], `${at}.modified`).map((item, index) => parseFem3dSolidsModification(item, `${at}.modified[${index}]`)),
  };
}

export function parseFem3dSupportsDelta(value: unknown, at = "$"): Fem3dSupportsDelta {
  const row = femFem3dDiffGuardClosed(femFem3dDiffGuardObject(value, at), at, ["removed", "inserted", "moved", "modified"]);
  return {
    removed: femFem3dDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => parseFem3dSupportRemoval(item, `${at}.removed[${index}]`)),
    inserted: femFem3dDiffGuardArray(row["inserted"], `${at}.inserted`).map((item, index) => parseFem3dSupportInsertion(item, `${at}.inserted[${index}]`)),
    moved: femFem3dDiffGuardArray(row["moved"], `${at}.moved`).map((item, index) => parseFem3dSupportRelocation(item, `${at}.moved[${index}]`)),
    modified: femFem3dDiffGuardArray(row["modified"], `${at}.modified`).map((item, index) => parseFem3dSupportsModification(item, `${at}.modified[${index}]`)),
  };
}

export function parseFem3dLoadCasesDelta(value: unknown, at = "$"): Fem3dLoadCasesDelta {
  const row = femFem3dDiffGuardClosed(femFem3dDiffGuardObject(value, at), at, ["removed", "inserted", "moved", "modified"]);
  return {
    removed: femFem3dDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => parseFem3dLoadCaseRemoval(item, `${at}.removed[${index}]`)),
    inserted: femFem3dDiffGuardArray(row["inserted"], `${at}.inserted`).map((item, index) => parseFem3dLoadCaseInsertion(item, `${at}.inserted[${index}]`)),
    moved: femFem3dDiffGuardArray(row["moved"], `${at}.moved`).map((item, index) => parseFem3dLoadCaseRelocation(item, `${at}.moved[${index}]`)),
    modified: femFem3dDiffGuardArray(row["modified"], `${at}.modified`).map((item, index) => parseFem3dLoadCasesModification(item, `${at}.modified[${index}]`)),
  };
}

export function parseFem3dCombinationsDelta(value: unknown, at = "$"): Fem3dCombinationsDelta {
  const row = femFem3dDiffGuardClosed(femFem3dDiffGuardObject(value, at), at, ["removed", "inserted", "moved", "modified"]);
  return {
    removed: femFem3dDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => parseFem3dCombinationRemoval(item, `${at}.removed[${index}]`)),
    inserted: femFem3dDiffGuardArray(row["inserted"], `${at}.inserted`).map((item, index) => parseFem3dCombinationInsertion(item, `${at}.inserted[${index}]`)),
    moved: femFem3dDiffGuardArray(row["moved"], `${at}.moved`).map((item, index) => parseFem3dCombinationRelocation(item, `${at}.moved[${index}]`)),
    modified: femFem3dDiffGuardArray(row["modified"], `${at}.modified`).map((item, index) => parseFem3dCombinationsModification(item, `${at}.modified[${index}]`)),
  };
}

export function parseFem3dLoadCasePatch(value: unknown, at = "$"): Fem3dLoadCasePatch {
  const row = femFem3dDiffGuardClosed(femFem3dDiffGuardObject(value, at), at, ["name", "selfWeight", "loads"]);
  return {
    name: row["name"] === undefined ? undefined : femFem3dDiffGuardString(row["name"], `${at}.name`),
    selfWeight: row["selfWeight"] === undefined ? undefined : femFem3dDiffGuardBoolean(row["selfWeight"], `${at}.selfWeight`),
    loads: row["loads"] === undefined ? undefined : parseFem3dLoadsDelta(row["loads"], `${at}.loads`),
  };
}

export function parseFem3dLoadsDelta(value: unknown, at = "$"): Fem3dLoadsDelta {
  const row = femFem3dDiffGuardClosed(femFem3dDiffGuardObject(value, at), at, ["removed", "inserted", "moved", "modified"]);
  return {
    removed: femFem3dDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => parseFem3dLoadRemoval(item, `${at}.removed[${index}]`)),
    inserted: femFem3dDiffGuardArray(row["inserted"], `${at}.inserted`).map((item, index) => parseFem3dLoadInsertion(item, `${at}.inserted[${index}]`)),
    moved: femFem3dDiffGuardArray(row["moved"], `${at}.moved`).map((item, index) => parseFem3dLoadRelocation(item, `${at}.moved[${index}]`)),
    modified: femFem3dDiffGuardArray(row["modified"], `${at}.modified`).map((item, index) => parseFem3dLoadsModification(item, `${at}.modified[${index}]`)),
  };
}

export function parseFem3dAnalysisPatch(value: unknown, at = "$"): Fem3dAnalysisPatch {
  const row = femFem3dDiffGuardClosed(femFem3dDiffGuardObject(value, at), at, ["modalCount", "bucklingCount", "deformationScale"]);
  return {
    modalCount: row["modalCount"] === undefined ? undefined : femFem3dDiffGuardInteger(row["modalCount"], `${at}.modalCount`, {"minimum": 1}),
    bucklingCount: row["bucklingCount"] === undefined ? undefined : femFem3dDiffGuardInteger(row["bucklingCount"], `${at}.bucklingCount`, {"minimum": 1}),
    deformationScale: row["deformationScale"] === undefined ? undefined : femFem3dDiffGuardNumber(row["deformationScale"], `${at}.deformationScale`),
  };
}

export function parseFem3dCombinationInsertion(value: unknown, at = "$"): Fem3dCombinationInsertion {
  const row = femFem3dDiffGuardClosed(femFem3dDiffGuardObject(value, at), at, ["index", "row"]);
  return {
    index: femFem3dDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
    row: parseFemCombination(row["row"], `${at}.row`),
  };
}

export function parseFem3dCombinationRelocation(value: unknown, at = "$"): Fem3dCombinationRelocation {
  const row = femFem3dDiffGuardClosed(femFem3dDiffGuardObject(value, at), at, ["id", "from", "to"]);
  return {
    id: femFem3dDiffGuardString(row["id"], `${at}.id`),
    from: femFem3dDiffGuardInteger(row["from"], `${at}.from`, {"minimum": 0}),
    to: femFem3dDiffGuardInteger(row["to"], `${at}.to`, {"minimum": 0}),
  };
}

export function parseFem3dCombinationRemoval(value: unknown, at = "$"): Fem3dCombinationRemoval {
  const row = femFem3dDiffGuardClosed(femFem3dDiffGuardObject(value, at), at, ["id", "index"]);
  return {
    id: femFem3dDiffGuardString(row["id"], `${at}.id`),
    index: femFem3dDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
  };
}

export function parseFem3dCombinationsModification(value: unknown, at = "$"): Fem3dCombinationsModification {
  const row = femFem3dDiffGuardClosed(femFem3dDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: femFem3dDiffGuardString(row["id"], `${at}.id`),
    patch: parseFem3dCombinationPatch(row["patch"], `${at}.patch`),
  };
}

export function parseFem3dElementInsertion(value: unknown, at = "$"): Fem3dElementInsertion {
  const row = femFem3dDiffGuardClosed(femFem3dDiffGuardObject(value, at), at, ["index", "row"]);
  return {
    index: femFem3dDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
    row: parseFemElement(row["row"], `${at}.row`),
  };
}

export function parseFem3dElementRelocation(value: unknown, at = "$"): Fem3dElementRelocation {
  const row = femFem3dDiffGuardClosed(femFem3dDiffGuardObject(value, at), at, ["id", "from", "to"]);
  return {
    id: femFem3dDiffGuardString(row["id"], `${at}.id`),
    from: femFem3dDiffGuardInteger(row["from"], `${at}.from`, {"minimum": 0}),
    to: femFem3dDiffGuardInteger(row["to"], `${at}.to`, {"minimum": 0}),
  };
}

export function parseFem3dElementRemoval(value: unknown, at = "$"): Fem3dElementRemoval {
  const row = femFem3dDiffGuardClosed(femFem3dDiffGuardObject(value, at), at, ["id", "index"]);
  return {
    id: femFem3dDiffGuardString(row["id"], `${at}.id`),
    index: femFem3dDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
  };
}

export function parseFem3dElementsModification(value: unknown, at = "$"): Fem3dElementsModification {
  const row = femFem3dDiffGuardClosed(femFem3dDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: femFem3dDiffGuardString(row["id"], `${at}.id`),
    patch: parseFem3dElementPatch(row["patch"], `${at}.patch`),
  };
}

export function parseFem3dLoadCaseInsertion(value: unknown, at = "$"): Fem3dLoadCaseInsertion {
  const row = femFem3dDiffGuardClosed(femFem3dDiffGuardObject(value, at), at, ["index", "row"]);
  return {
    index: femFem3dDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
    row: parseFemLoadCase(row["row"], `${at}.row`),
  };
}

export function parseFem3dLoadCaseRelocation(value: unknown, at = "$"): Fem3dLoadCaseRelocation {
  const row = femFem3dDiffGuardClosed(femFem3dDiffGuardObject(value, at), at, ["id", "from", "to"]);
  return {
    id: femFem3dDiffGuardString(row["id"], `${at}.id`),
    from: femFem3dDiffGuardInteger(row["from"], `${at}.from`, {"minimum": 0}),
    to: femFem3dDiffGuardInteger(row["to"], `${at}.to`, {"minimum": 0}),
  };
}

export function parseFem3dLoadCaseRemoval(value: unknown, at = "$"): Fem3dLoadCaseRemoval {
  const row = femFem3dDiffGuardClosed(femFem3dDiffGuardObject(value, at), at, ["id", "index"]);
  return {
    id: femFem3dDiffGuardString(row["id"], `${at}.id`),
    index: femFem3dDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
  };
}

export function parseFem3dLoadCasesModification(value: unknown, at = "$"): Fem3dLoadCasesModification {
  const row = femFem3dDiffGuardClosed(femFem3dDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: femFem3dDiffGuardString(row["id"], `${at}.id`),
    patch: parseFem3dLoadCasePatch(row["patch"], `${at}.patch`),
  };
}

export function parseFem3dLoadInsertion(value: unknown, at = "$"): Fem3dLoadInsertion {
  const row = femFem3dDiffGuardClosed(femFem3dDiffGuardObject(value, at), at, ["index", "row"]);
  return {
    index: femFem3dDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
    row: parseFemLoad(row["row"], `${at}.row`),
  };
}

export function parseFem3dLoadRelocation(value: unknown, at = "$"): Fem3dLoadRelocation {
  const row = femFem3dDiffGuardClosed(femFem3dDiffGuardObject(value, at), at, ["id", "from", "to"]);
  return {
    id: femFem3dDiffGuardString(row["id"], `${at}.id`),
    from: femFem3dDiffGuardInteger(row["from"], `${at}.from`, {"minimum": 0}),
    to: femFem3dDiffGuardInteger(row["to"], `${at}.to`, {"minimum": 0}),
  };
}

export function parseFem3dLoadRemoval(value: unknown, at = "$"): Fem3dLoadRemoval {
  const row = femFem3dDiffGuardClosed(femFem3dDiffGuardObject(value, at), at, ["id", "index"]);
  return {
    id: femFem3dDiffGuardString(row["id"], `${at}.id`),
    index: femFem3dDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
  };
}

export function parseFem3dLoadsModification(value: unknown, at = "$"): Fem3dLoadsModification {
  const row = femFem3dDiffGuardClosed(femFem3dDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: femFem3dDiffGuardString(row["id"], `${at}.id`),
    patch: parseFem3dLoadPatch(row["patch"], `${at}.patch`),
  };
}

export function parseFem3dMaterialInsertion(value: unknown, at = "$"): Fem3dMaterialInsertion {
  const row = femFem3dDiffGuardClosed(femFem3dDiffGuardObject(value, at), at, ["index", "row"]);
  return {
    index: femFem3dDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
    row: parseFemMaterial(row["row"], `${at}.row`),
  };
}

export function parseFem3dMaterialRelocation(value: unknown, at = "$"): Fem3dMaterialRelocation {
  const row = femFem3dDiffGuardClosed(femFem3dDiffGuardObject(value, at), at, ["id", "from", "to"]);
  return {
    id: femFem3dDiffGuardString(row["id"], `${at}.id`),
    from: femFem3dDiffGuardInteger(row["from"], `${at}.from`, {"minimum": 0}),
    to: femFem3dDiffGuardInteger(row["to"], `${at}.to`, {"minimum": 0}),
  };
}

export function parseFem3dMaterialRemoval(value: unknown, at = "$"): Fem3dMaterialRemoval {
  const row = femFem3dDiffGuardClosed(femFem3dDiffGuardObject(value, at), at, ["id", "index"]);
  return {
    id: femFem3dDiffGuardString(row["id"], `${at}.id`),
    index: femFem3dDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
  };
}

export function parseFem3dMaterialsModification(value: unknown, at = "$"): Fem3dMaterialsModification {
  const row = femFem3dDiffGuardClosed(femFem3dDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: femFem3dDiffGuardString(row["id"], `${at}.id`),
    patch: parseFem3dMaterialPatch(row["patch"], `${at}.patch`),
  };
}

export function parseFem3dNodeInsertion(value: unknown, at = "$"): Fem3dNodeInsertion {
  const row = femFem3dDiffGuardClosed(femFem3dDiffGuardObject(value, at), at, ["index", "row"]);
  return {
    index: femFem3dDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
    row: parseFemNode(row["row"], `${at}.row`),
  };
}

export function parseFem3dNodeRelocation(value: unknown, at = "$"): Fem3dNodeRelocation {
  const row = femFem3dDiffGuardClosed(femFem3dDiffGuardObject(value, at), at, ["id", "from", "to"]);
  return {
    id: femFem3dDiffGuardString(row["id"], `${at}.id`),
    from: femFem3dDiffGuardInteger(row["from"], `${at}.from`, {"minimum": 0}),
    to: femFem3dDiffGuardInteger(row["to"], `${at}.to`, {"minimum": 0}),
  };
}

export function parseFem3dNodeRemoval(value: unknown, at = "$"): Fem3dNodeRemoval {
  const row = femFem3dDiffGuardClosed(femFem3dDiffGuardObject(value, at), at, ["id", "index"]);
  return {
    id: femFem3dDiffGuardString(row["id"], `${at}.id`),
    index: femFem3dDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
  };
}

export function parseFem3dNodesModification(value: unknown, at = "$"): Fem3dNodesModification {
  const row = femFem3dDiffGuardClosed(femFem3dDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: femFem3dDiffGuardString(row["id"], `${at}.id`),
    patch: parseFem3dNodeDeltaPatch(row["patch"], `${at}.patch`),
  };
}

export function parseFem3dSectionInsertion(value: unknown, at = "$"): Fem3dSectionInsertion {
  const row = femFem3dDiffGuardClosed(femFem3dDiffGuardObject(value, at), at, ["index", "row"]);
  return {
    index: femFem3dDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
    row: parseFemSection(row["row"], `${at}.row`),
  };
}

export function parseFem3dSectionRelocation(value: unknown, at = "$"): Fem3dSectionRelocation {
  const row = femFem3dDiffGuardClosed(femFem3dDiffGuardObject(value, at), at, ["id", "from", "to"]);
  return {
    id: femFem3dDiffGuardString(row["id"], `${at}.id`),
    from: femFem3dDiffGuardInteger(row["from"], `${at}.from`, {"minimum": 0}),
    to: femFem3dDiffGuardInteger(row["to"], `${at}.to`, {"minimum": 0}),
  };
}

export function parseFem3dSectionRemoval(value: unknown, at = "$"): Fem3dSectionRemoval {
  const row = femFem3dDiffGuardClosed(femFem3dDiffGuardObject(value, at), at, ["id", "index"]);
  return {
    id: femFem3dDiffGuardString(row["id"], `${at}.id`),
    index: femFem3dDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
  };
}

export function parseFem3dSectionsModification(value: unknown, at = "$"): Fem3dSectionsModification {
  const row = femFem3dDiffGuardClosed(femFem3dDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: femFem3dDiffGuardString(row["id"], `${at}.id`),
    patch: parseFem3dSectionPatch(row["patch"], `${at}.patch`),
  };
}

export function parseFem3dSolidInsertion(value: unknown, at = "$"): Fem3dSolidInsertion {
  const row = femFem3dDiffGuardClosed(femFem3dDiffGuardObject(value, at), at, ["index", "row"]);
  return {
    index: femFem3dDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
    row: parseFemSolid(row["row"], `${at}.row`),
  };
}

export function parseFem3dSolidRelocation(value: unknown, at = "$"): Fem3dSolidRelocation {
  const row = femFem3dDiffGuardClosed(femFem3dDiffGuardObject(value, at), at, ["id", "from", "to"]);
  return {
    id: femFem3dDiffGuardString(row["id"], `${at}.id`),
    from: femFem3dDiffGuardInteger(row["from"], `${at}.from`, {"minimum": 0}),
    to: femFem3dDiffGuardInteger(row["to"], `${at}.to`, {"minimum": 0}),
  };
}

export function parseFem3dSolidRemoval(value: unknown, at = "$"): Fem3dSolidRemoval {
  const row = femFem3dDiffGuardClosed(femFem3dDiffGuardObject(value, at), at, ["id", "index"]);
  return {
    id: femFem3dDiffGuardString(row["id"], `${at}.id`),
    index: femFem3dDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
  };
}

export function parseFem3dSolidsModification(value: unknown, at = "$"): Fem3dSolidsModification {
  const row = femFem3dDiffGuardClosed(femFem3dDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: femFem3dDiffGuardString(row["id"], `${at}.id`),
    patch: parseFem3dSolidPatch(row["patch"], `${at}.patch`),
  };
}

export function parseFem3dSupportInsertion(value: unknown, at = "$"): Fem3dSupportInsertion {
  const row = femFem3dDiffGuardClosed(femFem3dDiffGuardObject(value, at), at, ["index", "row"]);
  return {
    index: femFem3dDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
    row: parseFemSupport(row["row"], `${at}.row`),
  };
}

export function parseFem3dSupportRelocation(value: unknown, at = "$"): Fem3dSupportRelocation {
  const row = femFem3dDiffGuardClosed(femFem3dDiffGuardObject(value, at), at, ["id", "from", "to"]);
  return {
    id: femFem3dDiffGuardString(row["id"], `${at}.id`),
    from: femFem3dDiffGuardInteger(row["from"], `${at}.from`, {"minimum": 0}),
    to: femFem3dDiffGuardInteger(row["to"], `${at}.to`, {"minimum": 0}),
  };
}

export function parseFem3dSupportRemoval(value: unknown, at = "$"): Fem3dSupportRemoval {
  const row = femFem3dDiffGuardClosed(femFem3dDiffGuardObject(value, at), at, ["id", "index"]);
  return {
    id: femFem3dDiffGuardString(row["id"], `${at}.id`),
    index: femFem3dDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
  };
}

export function parseFem3dSupportsModification(value: unknown, at = "$"): Fem3dSupportsModification {
  const row = femFem3dDiffGuardClosed(femFem3dDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: femFem3dDiffGuardString(row["id"], `${at}.id`),
    patch: parseFem3dSupportPatch(row["patch"], `${at}.patch`),
  };
}

/** 🩹️ Admits sparse material changes without accepting a replacement identity. */
export function parseFem3dMaterialPatch(value: unknown, at = "$"): Fem3dMaterialPatch {
  const row = femFem3dDiffGuardObject(value, at);
  const patch: Fem3dMaterialPatch = {};
  for (const key of Object.keys(row)) {
    if (!["name", "e", "nu", "rho", "g"].includes(key)) return femFem3dDiffGuardReject(`${at}.${key}`, "unknown material field");
    if (key === "name") {
      patch.name = femFem3dDiffGuardString(row[key], `${at}.${key}`);
    } else {
      const word = parseBinary64(row[key]);
      const number = binary64Value(word);
      if (!Number.isFinite(number) || (key === "nu" ? number <= -1 || number >= 0.5 : number <= 0)) return femFem3dDiffGuardReject(`${at}.${key}`, "invalid material value");
      patch[key as Exclude<keyof Fem3dMaterialPatch, "name">] = word;
    }
  }
  return patch;
}

/** 🩹️ Admits only supplied node fields with their native scalar identity. */
function parseFem3dNodeDeltaPatch(value: unknown, at = "$"): Fem3dNodePatch {
  const row = femFem3dDiffGuardObject(value, at);
  const patch: Fem3dNodePatch = {};
  for (const key of Object.keys(row)) {
    if (!["x", "y", "z"].includes(key)) return femFem3dDiffGuardReject(`${at}.${key}`, "unknown node field");
    const word = parseBinary64(row[key]);
    patch[key as keyof Fem3dNodePatch] = word;
  }
  return patch;
}

/** 🛂️ Validates finite coordinates for a supplied editing intent. */
export function parseFem3dNodePatch(value: unknown, at = "$"): Fem3dNodePatch {
  const patch = parseFem3dNodeDeltaPatch(value, at);
  for (const [key, word] of Object.entries(patch)) {
    if (!Number.isFinite(binary64Value(word))) return femFem3dDiffGuardReject(`${at}.${key}`, "invalid node value");
  }
  return patch;
}
/** 🩹️ Admits only supplied section fields with their native scalar identity. */
export function parseFem3dSectionPatch(value: unknown, at = "$"): Fem3dSectionPatch {
  const row = femFem3dDiffGuardObject(value, at);
  const patch: Fem3dSectionPatch = {};
  for (const key of Object.keys(row)) {
    if (!["name", "area", "iy", "iz", "j"].includes(key)) return femFem3dDiffGuardReject(`${at}.${key}`, "unknown section field");
    if (key === "name") { patch.name = femFem3dDiffGuardString(row[key], `${at}.${key}`); continue; }
    const word = parseBinary64(row[key]);
    const number = binary64Value(word);
    if (!Number.isFinite(number) || number <= 0) return femFem3dDiffGuardReject(`${at}.${key}`, "invalid section value");
    patch[key as Exclude<keyof Fem3dSectionPatch, "name">] = word;
  }
  return patch;
}
/** 🩹️ Admits supplied geometry fields without a replacement identity. */
export function parseFem3dSolidPatch(value: unknown, at = "$"): Fem3dSolidPatch {
  const row = femFem3dDiffGuardObject(value, at);
  const patch: Fem3dSolidPatch = {};
  const point = (value: unknown, here: string): [Binary64, Binary64] => {
    const coordinates = femFem3dDiffGuardArray(value, here);
    if (coordinates.length !== 2) return femFem3dDiffGuardReject(here, "two coordinates required");
    return coordinates.map((value, index) => {
      const word = parseBinary64(value, `${here}[${index}]`);
      if (!Number.isFinite(binary64Value(word))) return femFem3dDiffGuardReject(here, "finite coordinate required");
      return word;
    }) as [Binary64, Binary64];
  };
  const ring = (value: unknown, here: string) => {
    const points = femFem3dDiffGuardArray(value, here);
    if (points.length < 3) return femFem3dDiffGuardReject(here, "at least three points required");
    return points.map((value, index) => point(value, `${here}[${index}]`));
  };
  for (const key of Object.keys(row)) {
    if (!["name", "outline", "holes", "axis", "baseZ", "height", "layers", "meshSize", "materialId"].includes(key)) return femFem3dDiffGuardReject(`${at}.${key}`, "unknown geometry field");
    if (key === "name" || key === "materialId") patch[key] = femFem3dDiffGuardString(row[key], `${at}.${key}`);
    else if (key === "outline") patch.outline = ring(row[key], `${at}.${key}`);
    else if (key === "holes") patch.holes = femFem3dDiffGuardArray(row[key], `${at}.${key}`).map((value, index) => ring(value, `${at}.${key}[${index}]`));
    else if (key === "layers") {
      const count = parseFemCount(row[key], `${at}.${key}`);
      if (count === 0n) return GUARDReject(`${at}.${key}`, "positive layer count required");
      patch.layers = count;
    }
    else if (key === "axis") patch.axis = GUARDMember(row[key], `${at}.${key}`, ["x", "y", "z"] as const);
    else {
      const word = parseBinary64(row[key], `${at}.${key}`);
      const number = binary64Value(word);
      if (!Number.isFinite(number) || (key !== "baseZ" && number <= 0)) return femFem3dDiffGuardReject(`${at}.${key}`, "positive finite geometry property required");
      patch[key as "baseZ" | "height" | "meshSize"] = word;
    }
  }
  return patch;
}

/** 🩹️ Admits supplied support fields without stable identity. */
export function parseFem3dSupportPatch(value: unknown, at = "$"): Fem3dSupportPatch {
  const row = femFem3dDiffGuardObject(value, at);
  const patch: Fem3dSupportPatch = {};
  if (Object.hasOwn(row, "fixed") && ["tx", "ty", "tz", "rx", "ry", "rz"].some(key => Object.hasOwn(row, key))) return femFem3dDiffGuardReject(at, "whole fixed set and membership setters are exclusive");
  for (const key of Object.keys(row)) {
    if (!["nodeId", "fixed", "tx", "ty", "tz", "rx", "ry", "rz"].includes(key)) return femFem3dDiffGuardReject(`${at}.${key}`, "unknown support field");
    if (key === "nodeId") { patch.nodeId = femFem3dDiffGuardString(row[key], `${at}.${key}`); }
    if (key === "fixed") { patch.fixed = parseFemFixedDofs(row[key], `${at}.${key}`); }
    if (key === "tx") { if (typeof row[key] !== "boolean") return femFem3dDiffGuardReject(`${at}.${key}`, "boolean required"); patch.tx = row[key] as boolean; }
    if (key === "ty") { if (typeof row[key] !== "boolean") return femFem3dDiffGuardReject(`${at}.${key}`, "boolean required"); patch.ty = row[key] as boolean; }
    if (key === "tz") { if (typeof row[key] !== "boolean") return femFem3dDiffGuardReject(`${at}.${key}`, "boolean required"); patch.tz = row[key] as boolean; }
    if (key === "rx") { if (typeof row[key] !== "boolean") return femFem3dDiffGuardReject(`${at}.${key}`, "boolean required"); patch.rx = row[key] as boolean; }
    if (key === "ry") { if (typeof row[key] !== "boolean") return femFem3dDiffGuardReject(`${at}.${key}`, "boolean required"); patch.ry = row[key] as boolean; }
    if (key === "rz") { if (typeof row[key] !== "boolean") return femFem3dDiffGuardReject(`${at}.${key}`, "boolean required"); patch.rz = row[key] as boolean; }
  }
  return patch;
}

/** 🩹️ Admits supplied element fields without stable identity. */
export function parseFem3dElementPatch(value: unknown, at = "$"): Fem3dElementPatch {
  const row = femFem3dDiffGuardObject(value, at);
  const patch: Fem3dElementPatch = {};
  if (row.kind === "bar" && Object.hasOwn(row, "roll")) return femFem3dDiffGuardReject(at, "bar elements cannot carry roll");
  for (const key of Object.keys(row)) {
    if (!["kind", "start", "end", "materialId", "sectionId", "roll"].includes(key)) return femFem3dDiffGuardReject(`${at}.${key}`, "unknown element field");
    if (key === "kind") { patch.kind = femFem3dDiffGuardMember(row[key], `${at}.${key}`, ["bar", "frame"] as const); }
    if (key === "start") { patch.start = femFem3dDiffGuardString(row[key], `${at}.${key}`); }
    if (key === "end") { patch.end = femFem3dDiffGuardString(row[key], `${at}.${key}`); }
    if (key === "materialId") { patch.materialId = femFem3dDiffGuardString(row[key], `${at}.${key}`); }
    if (key === "sectionId") { patch.sectionId = femFem3dDiffGuardString(row[key], `${at}.${key}`); }
    if (key === "roll") { const word = parseBinary64(row[key], `${at}.${key}`); if (!Number.isFinite(binary64Value(word))) return femFem3dDiffGuardReject(`${at}.${key}`, "finite roll required"); patch.roll = word; }
  }
  return patch;
}

/** 🛡️ Admits a tagged field patch without identity, nulls or unknown fields. */
export function parseFem3dLoadPatch(value: unknown, at = "$"): Fem3dLoadPatch {
  const row = femFem3dDiffGuardObject(value, at);
  const kind = femFem3dDiffGuardMember(row.kind, `${at}.kind`, ["nodal", "memberUdl", "area"] as const);
  const fields: Record<string, readonly string[]> = { nodal: ["nodeId", "dof", "value"], memberUdl: ["elementId", "wx", "wy", "wz"], area: ["solidId", "pressure"] };
  const patch: Record<string, unknown> = { kind };
  for (const key of Object.keys(row)) {
    if (key === "kind") continue;
    if (!fields[kind].includes(key)) return femFem3dDiffGuardReject(`${at}.${key}`, "unknown load field");
    if (["nodeId", "elementId", "solidId"].includes(key)) patch[key] = femFem3dDiffGuardString(row[key], `${at}.${key}`);
    else if (key === "dof") patch[key] = femFem3dDiffGuardMember(row[key], `${at}.${key}`, ["Tx", "Ty", "Tz", "Rx", "Ry", "Rz"] as const);
    else { const word = parseBinary64(row[key], `${at}.${key}`); if (!Number.isFinite(binary64Value(word))) return femFem3dDiffGuardReject(`${at}.${key}`, "finite value required"); patch[key] = word; }
  }
  return patch as unknown as Fem3dLoadPatch;
}
/** ⚖️ Admits only authored name and keyed term fields. */
export function parseFem3dCombinationPatch(value: unknown, at = "$"): Fem3dCombinationPatch {
  const row = femFem3dDiffGuardObject(value, at);
  const patch: Fem3dCombinationPatch = {};
  for (const key of Object.keys(row)) {
    if (key === "name") patch.name = femFem3dDiffGuardString(row[key], `${at}.name`);
    else if (key === "terms") patch.terms = parseFem3dCombinationTermsDelta(row[key], `${at}.terms`);
    else return femFem3dDiffGuardReject(`${at}.${key}`, "unknown combination field");
  }
  return patch;
}
/** 🗂️ Uses the shared keyed map wire and finite owned numeric payloads. */
export function parseFem3dCombinationTermsDelta(value: unknown, at = "$"): Fem3dCombinationTermsDelta {
  return parseMapDelta(value, raw => { const word = parseBinary64(raw, at); if (!Number.isFinite(binary64Value(word))) return femFem3dDiffGuardReject(at, "finite factor required"); return word; });
}
