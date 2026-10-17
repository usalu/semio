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

export interface Fem2dNodeRemoval {
  id: string;
  index: number;
}

export interface Fem2dNodeInsertion {
  index: number;
  row: FemNode;
}

export interface Fem2dNodeRelocation {
  id: string;
  from: number;
  to: number;
}

export interface Fem2dNodesModification {
  id: string;
  patch: FemNode;
}

export interface Fem2dNodesDelta {
  removed: Fem2dNodeRemoval[];
  inserted: Fem2dNodeInsertion[];
  moved: Fem2dNodeRelocation[];
  modified: Fem2dNodesModification[];
}

export interface Fem2dElementRemoval {
  id: string;
  index: number;
}

export interface Fem2dElementInsertion {
  index: number;
  row: FemElement;
}

export interface Fem2dElementRelocation {
  id: string;
  from: number;
  to: number;
}

export interface Fem2dElementsModification {
  id: string;
  patch: FemElement;
}

export interface Fem2dElementsDelta {
  removed: Fem2dElementRemoval[];
  inserted: Fem2dElementInsertion[];
  moved: Fem2dElementRelocation[];
  modified: Fem2dElementsModification[];
}

export interface Fem2dRegionRemoval {
  id: string;
  index: number;
}

export interface Fem2dRegionInsertion {
  index: number;
  row: FemRegion;
}

export interface Fem2dRegionRelocation {
  id: string;
  from: number;
  to: number;
}

export interface Fem2dRegionsModification {
  id: string;
  patch: FemRegion;
}

export interface Fem2dRegionsDelta {
  removed: Fem2dRegionRemoval[];
  inserted: Fem2dRegionInsertion[];
  moved: Fem2dRegionRelocation[];
  modified: Fem2dRegionsModification[];
}

export interface Fem2dMaterialRemoval {
  id: string;
  index: number;
}

export interface Fem2dMaterialInsertion {
  index: number;
  row: FemMaterial;
}

export interface Fem2dMaterialRelocation {
  id: string;
  from: number;
  to: number;
}

export interface Fem2dMaterialsModification {
  id: string;
  patch: FemMaterial;
}

export interface Fem2dMaterialsDelta {
  removed: Fem2dMaterialRemoval[];
  inserted: Fem2dMaterialInsertion[];
  moved: Fem2dMaterialRelocation[];
  modified: Fem2dMaterialsModification[];
}

export interface Fem2dSectionRemoval {
  id: string;
  index: number;
}

export interface Fem2dSectionInsertion {
  index: number;
  row: FemSection;
}

export interface Fem2dSectionRelocation {
  id: string;
  from: number;
  to: number;
}

export interface Fem2dSectionsModification {
  id: string;
  patch: FemSection;
}

export interface Fem2dSectionsDelta {
  removed: Fem2dSectionRemoval[];
  inserted: Fem2dSectionInsertion[];
  moved: Fem2dSectionRelocation[];
  modified: Fem2dSectionsModification[];
}

export interface Fem2dSupportRemoval {
  id: string;
  index: number;
}

export interface Fem2dSupportInsertion {
  index: number;
  row: FemSupport;
}

export interface Fem2dSupportRelocation {
  id: string;
  from: number;
  to: number;
}

export interface Fem2dSupportsModification {
  id: string;
  patch: FemSupport;
}

export interface Fem2dSupportsDelta {
  removed: Fem2dSupportRemoval[];
  inserted: Fem2dSupportInsertion[];
  moved: Fem2dSupportRelocation[];
  modified: Fem2dSupportsModification[];
}

export interface Fem2dLoadCaseRemoval {
  id: string;
  index: number;
}

export interface Fem2dLoadCaseInsertion {
  index: number;
  row: FemLoadCase;
}

export interface Fem2dLoadCaseRelocation {
  id: string;
  from: number;
  to: number;
}

export interface Fem2dLoadCasesModification {
  id: string;
  patch: Fem2dLoadCasePatch;
}

export interface Fem2dLoadCasesDelta {
  removed: Fem2dLoadCaseRemoval[];
  inserted: Fem2dLoadCaseInsertion[];
  moved: Fem2dLoadCaseRelocation[];
  modified: Fem2dLoadCasesModification[];
}

export interface Fem2dLoadCasePatch {
  name?: string;
  selfWeight?: boolean;
  loads?: Fem2dLoadsDelta;
}

export interface Fem2dLoadRemoval {
  id: string;
  index: number;
}

export interface Fem2dLoadInsertion {
  index: number;
  row: FemLoad;
}

export interface Fem2dLoadRelocation {
  id: string;
  from: number;
  to: number;
}

export interface Fem2dLoadsModification {
  id: string;
  patch: FemLoad;
}

export interface Fem2dLoadsDelta {
  removed: Fem2dLoadRemoval[];
  inserted: Fem2dLoadInsertion[];
  moved: Fem2dLoadRelocation[];
  modified: Fem2dLoadsModification[];
}

export interface Fem2dAnalysisPatch {
  modalCount?: number;
  bucklingCount?: number;
  deformationScale?: number;
}

export interface Fem2dCombinationRemoval {
  id: string;
  index: number;
}

export interface Fem2dCombinationInsertion {
  index: number;
  row: FemCombination;
}

export interface Fem2dCombinationRelocation {
  id: string;
  from: number;
  to: number;
}

export interface Fem2dCombinationsModification {
  id: string;
  patch: FemCombination;
}

export interface Fem2dCombinationsDelta {
  removed: Fem2dCombinationRemoval[];
  inserted: Fem2dCombinationInsertion[];
  moved: Fem2dCombinationRelocation[];
  modified: Fem2dCombinationsModification[];
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
export const femFem2dDiffGuardClosed = (row: Readonly<Record<string, unknown>>, at: string, keys: readonly string[]): Readonly<Record<string, unknown>> => {
  for (const key of Object.keys(row)) if (!keys.includes(key)) femFem2dDiffGuardReject(`${at}.${key}`, "unknown property");
  return row;
};
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

/** 🩹️ Admit the positional row shapes: removal `{id, index}`, insertion `{index, row}`, relocation `{id, from, to}` and the keyed modification `{id, patch}`. */
function parseFemRows<T>(value:unknown,at:string,keys:readonly string[],parse:(row:Readonly<Record<string,unknown>>,at:string)=>T):T[]{return femFem2dDiffGuardArray(value,at).map((item,index)=>{const here=`${at}[${index}]`,row=femFem2dDiffGuardObject(item,here);for(const key of Object.keys(row))if(!keys.includes(key))throw Error(`${here}: unknown field`);return parse(row,here)})}
const parseFemRemovals=(value:unknown,at:string)=>parseFemRows(value,at,["id","index"],(row,here)=>({id:femFem2dDiffGuardString(row.id,`${here}.id`),index:femFem2dDiffGuardInteger(row.index,`${here}.index`,{minimum:0})}));
const parseFemRelocations=(value:unknown,at:string)=>parseFemRows(value,at,["id","from","to"],(row,here)=>({id:femFem2dDiffGuardString(row.id,`${here}.id`),from:femFem2dDiffGuardInteger(row.from,`${here}.from`,{minimum:0}),to:femFem2dDiffGuardInteger(row.to,`${here}.to`,{minimum:0})}));
const parseFemInsertions=<T>(value:unknown,at:string,parse:(value:unknown,at:string)=>T)=>parseFemRows(value,at,["index","row"],(row,here)=>({index:femFem2dDiffGuardInteger(row.index,`${here}.index`,{minimum:0}),row:parse(row.row,`${here}.row`)}));
const parseFemModifications=<T>(value:unknown,at:string,parse:(value:unknown,at:string)=>T)=>parseFemRows(value,at,["id","patch"],(row,here)=>({id:femFem2dDiffGuardString(row.id,`${here}.id`),patch:parse(row.patch,`${here}.patch`)}));

export function parseFem2dNodesDelta(value: unknown, at = "$"): Fem2dNodesDelta {
  const row = femFem2dDiffGuardClosed(femFem2dDiffGuardObject(value, at), at, ["removed", "inserted", "moved", "modified"]);
  return {
    removed: femFem2dDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => parseFem2dNodeRemoval(item, `${at}.removed[${index}]`)),
    inserted: femFem2dDiffGuardArray(row["inserted"], `${at}.inserted`).map((item, index) => parseFem2dNodeInsertion(item, `${at}.inserted[${index}]`)),
    moved: femFem2dDiffGuardArray(row["moved"], `${at}.moved`).map((item, index) => parseFem2dNodeRelocation(item, `${at}.moved[${index}]`)),
    modified: femFem2dDiffGuardArray(row["modified"], `${at}.modified`).map((item, index) => parseFem2dNodesModification(item, `${at}.modified[${index}]`)),
  };
}

export function parseFem2dElementsDelta(value: unknown, at = "$"): Fem2dElementsDelta {
  const row = femFem2dDiffGuardClosed(femFem2dDiffGuardObject(value, at), at, ["removed", "inserted", "moved", "modified"]);
  return {
    removed: femFem2dDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => parseFem2dElementRemoval(item, `${at}.removed[${index}]`)),
    inserted: femFem2dDiffGuardArray(row["inserted"], `${at}.inserted`).map((item, index) => parseFem2dElementInsertion(item, `${at}.inserted[${index}]`)),
    moved: femFem2dDiffGuardArray(row["moved"], `${at}.moved`).map((item, index) => parseFem2dElementRelocation(item, `${at}.moved[${index}]`)),
    modified: femFem2dDiffGuardArray(row["modified"], `${at}.modified`).map((item, index) => parseFem2dElementsModification(item, `${at}.modified[${index}]`)),
  };
}

export function parseFem2dRegionsDelta(value: unknown, at = "$"): Fem2dRegionsDelta {
  const row = femFem2dDiffGuardClosed(femFem2dDiffGuardObject(value, at), at, ["removed", "inserted", "moved", "modified"]);
  return {
    removed: femFem2dDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => parseFem2dRegionRemoval(item, `${at}.removed[${index}]`)),
    inserted: femFem2dDiffGuardArray(row["inserted"], `${at}.inserted`).map((item, index) => parseFem2dRegionInsertion(item, `${at}.inserted[${index}]`)),
    moved: femFem2dDiffGuardArray(row["moved"], `${at}.moved`).map((item, index) => parseFem2dRegionRelocation(item, `${at}.moved[${index}]`)),
    modified: femFem2dDiffGuardArray(row["modified"], `${at}.modified`).map((item, index) => parseFem2dRegionsModification(item, `${at}.modified[${index}]`)),
  };
}

export function parseFem2dMaterialsDelta(value: unknown, at = "$"): Fem2dMaterialsDelta {
  const row = femFem2dDiffGuardClosed(femFem2dDiffGuardObject(value, at), at, ["removed", "inserted", "moved", "modified"]);
  return {
    removed: femFem2dDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => parseFem2dMaterialRemoval(item, `${at}.removed[${index}]`)),
    inserted: femFem2dDiffGuardArray(row["inserted"], `${at}.inserted`).map((item, index) => parseFem2dMaterialInsertion(item, `${at}.inserted[${index}]`)),
    moved: femFem2dDiffGuardArray(row["moved"], `${at}.moved`).map((item, index) => parseFem2dMaterialRelocation(item, `${at}.moved[${index}]`)),
    modified: femFem2dDiffGuardArray(row["modified"], `${at}.modified`).map((item, index) => parseFem2dMaterialsModification(item, `${at}.modified[${index}]`)),
  };
}

export function parseFem2dSectionsDelta(value: unknown, at = "$"): Fem2dSectionsDelta {
  const row = femFem2dDiffGuardClosed(femFem2dDiffGuardObject(value, at), at, ["removed", "inserted", "moved", "modified"]);
  return {
    removed: femFem2dDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => parseFem2dSectionRemoval(item, `${at}.removed[${index}]`)),
    inserted: femFem2dDiffGuardArray(row["inserted"], `${at}.inserted`).map((item, index) => parseFem2dSectionInsertion(item, `${at}.inserted[${index}]`)),
    moved: femFem2dDiffGuardArray(row["moved"], `${at}.moved`).map((item, index) => parseFem2dSectionRelocation(item, `${at}.moved[${index}]`)),
    modified: femFem2dDiffGuardArray(row["modified"], `${at}.modified`).map((item, index) => parseFem2dSectionsModification(item, `${at}.modified[${index}]`)),
  };
}

export function parseFem2dSupportsDelta(value: unknown, at = "$"): Fem2dSupportsDelta {
  const row = femFem2dDiffGuardClosed(femFem2dDiffGuardObject(value, at), at, ["removed", "inserted", "moved", "modified"]);
  return {
    removed: femFem2dDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => parseFem2dSupportRemoval(item, `${at}.removed[${index}]`)),
    inserted: femFem2dDiffGuardArray(row["inserted"], `${at}.inserted`).map((item, index) => parseFem2dSupportInsertion(item, `${at}.inserted[${index}]`)),
    moved: femFem2dDiffGuardArray(row["moved"], `${at}.moved`).map((item, index) => parseFem2dSupportRelocation(item, `${at}.moved[${index}]`)),
    modified: femFem2dDiffGuardArray(row["modified"], `${at}.modified`).map((item, index) => parseFem2dSupportsModification(item, `${at}.modified[${index}]`)),
  };
}

export function parseFem2dLoadCasesDelta(value: unknown, at = "$"): Fem2dLoadCasesDelta {
  const row = femFem2dDiffGuardClosed(femFem2dDiffGuardObject(value, at), at, ["removed", "inserted", "moved", "modified"]);
  return {
    removed: femFem2dDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => parseFem2dLoadCaseRemoval(item, `${at}.removed[${index}]`)),
    inserted: femFem2dDiffGuardArray(row["inserted"], `${at}.inserted`).map((item, index) => parseFem2dLoadCaseInsertion(item, `${at}.inserted[${index}]`)),
    moved: femFem2dDiffGuardArray(row["moved"], `${at}.moved`).map((item, index) => parseFem2dLoadCaseRelocation(item, `${at}.moved[${index}]`)),
    modified: femFem2dDiffGuardArray(row["modified"], `${at}.modified`).map((item, index) => parseFem2dLoadCasesModification(item, `${at}.modified[${index}]`)),
  };
}

export function parseFem2dCombinationsDelta(value: unknown, at = "$"): Fem2dCombinationsDelta {
  const row = femFem2dDiffGuardClosed(femFem2dDiffGuardObject(value, at), at, ["removed", "inserted", "moved", "modified"]);
  return {
    removed: femFem2dDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => parseFem2dCombinationRemoval(item, `${at}.removed[${index}]`)),
    inserted: femFem2dDiffGuardArray(row["inserted"], `${at}.inserted`).map((item, index) => parseFem2dCombinationInsertion(item, `${at}.inserted[${index}]`)),
    moved: femFem2dDiffGuardArray(row["moved"], `${at}.moved`).map((item, index) => parseFem2dCombinationRelocation(item, `${at}.moved[${index}]`)),
    modified: femFem2dDiffGuardArray(row["modified"], `${at}.modified`).map((item, index) => parseFem2dCombinationsModification(item, `${at}.modified[${index}]`)),
  };
}

export function parseFem2dLoadCasePatch(value: unknown, at = "$"): Fem2dLoadCasePatch {
  const row = femFem2dDiffGuardClosed(femFem2dDiffGuardObject(value, at), at, ["name", "selfWeight", "loads"]);
  return {
    name: row["name"] === undefined ? undefined : femFem2dDiffGuardString(row["name"], `${at}.name`),
    selfWeight: row["selfWeight"] === undefined ? undefined : femFem2dDiffGuardBoolean(row["selfWeight"], `${at}.selfWeight`),
    loads: row["loads"] === undefined ? undefined : parseFem2dLoadsDelta(row["loads"], `${at}.loads`),
  };
}

export function parseFem2dLoadsDelta(value: unknown, at = "$"): Fem2dLoadsDelta {
  const row = femFem2dDiffGuardClosed(femFem2dDiffGuardObject(value, at), at, ["removed", "inserted", "moved", "modified"]);
  return {
    removed: femFem2dDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => parseFem2dLoadRemoval(item, `${at}.removed[${index}]`)),
    inserted: femFem2dDiffGuardArray(row["inserted"], `${at}.inserted`).map((item, index) => parseFem2dLoadInsertion(item, `${at}.inserted[${index}]`)),
    moved: femFem2dDiffGuardArray(row["moved"], `${at}.moved`).map((item, index) => parseFem2dLoadRelocation(item, `${at}.moved[${index}]`)),
    modified: femFem2dDiffGuardArray(row["modified"], `${at}.modified`).map((item, index) => parseFem2dLoadsModification(item, `${at}.modified[${index}]`)),
  };
}

export function parseFem2dAnalysisPatch(value: unknown, at = "$"): Fem2dAnalysisPatch {
  const row = femFem2dDiffGuardClosed(femFem2dDiffGuardObject(value, at), at, ["modalCount", "bucklingCount", "deformationScale"]);
  return {
    modalCount: row["modalCount"] === undefined ? undefined : femFem2dDiffGuardInteger(row["modalCount"], `${at}.modalCount`, {"minimum": 1}),
    bucklingCount: row["bucklingCount"] === undefined ? undefined : femFem2dDiffGuardInteger(row["bucklingCount"], `${at}.bucklingCount`, {"minimum": 1}),
    deformationScale: row["deformationScale"] === undefined ? undefined : femFem2dDiffGuardNumber(row["deformationScale"], `${at}.deformationScale`),
  };
}

export function parseFem2dCombinationInsertion(value: unknown, at = "$"): Fem2dCombinationInsertion {
  const row = femFem2dDiffGuardClosed(femFem2dDiffGuardObject(value, at), at, ["index", "row"]);
  return {
    index: femFem2dDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
    row: parseFemCombination(row["row"], `${at}.row`),
  };
}

export function parseFem2dCombinationRelocation(value: unknown, at = "$"): Fem2dCombinationRelocation {
  const row = femFem2dDiffGuardClosed(femFem2dDiffGuardObject(value, at), at, ["id", "from", "to"]);
  return {
    id: femFem2dDiffGuardString(row["id"], `${at}.id`),
    from: femFem2dDiffGuardInteger(row["from"], `${at}.from`, {"minimum": 0}),
    to: femFem2dDiffGuardInteger(row["to"], `${at}.to`, {"minimum": 0}),
  };
}

export function parseFem2dCombinationRemoval(value: unknown, at = "$"): Fem2dCombinationRemoval {
  const row = femFem2dDiffGuardClosed(femFem2dDiffGuardObject(value, at), at, ["id", "index"]);
  return {
    id: femFem2dDiffGuardString(row["id"], `${at}.id`),
    index: femFem2dDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
  };
}

export function parseFem2dCombinationsModification(value: unknown, at = "$"): Fem2dCombinationsModification {
  const row = femFem2dDiffGuardClosed(femFem2dDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: femFem2dDiffGuardString(row["id"], `${at}.id`),
    patch: parseFemCombination(row["patch"], `${at}.patch`),
  };
}

export function parseFem2dElementInsertion(value: unknown, at = "$"): Fem2dElementInsertion {
  const row = femFem2dDiffGuardClosed(femFem2dDiffGuardObject(value, at), at, ["index", "row"]);
  return {
    index: femFem2dDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
    row: parseFemElement(row["row"], `${at}.row`),
  };
}

export function parseFem2dElementRelocation(value: unknown, at = "$"): Fem2dElementRelocation {
  const row = femFem2dDiffGuardClosed(femFem2dDiffGuardObject(value, at), at, ["id", "from", "to"]);
  return {
    id: femFem2dDiffGuardString(row["id"], `${at}.id`),
    from: femFem2dDiffGuardInteger(row["from"], `${at}.from`, {"minimum": 0}),
    to: femFem2dDiffGuardInteger(row["to"], `${at}.to`, {"minimum": 0}),
  };
}

export function parseFem2dElementRemoval(value: unknown, at = "$"): Fem2dElementRemoval {
  const row = femFem2dDiffGuardClosed(femFem2dDiffGuardObject(value, at), at, ["id", "index"]);
  return {
    id: femFem2dDiffGuardString(row["id"], `${at}.id`),
    index: femFem2dDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
  };
}

export function parseFem2dElementsModification(value: unknown, at = "$"): Fem2dElementsModification {
  const row = femFem2dDiffGuardClosed(femFem2dDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: femFem2dDiffGuardString(row["id"], `${at}.id`),
    patch: parseFemElement(row["patch"], `${at}.patch`),
  };
}

export function parseFem2dLoadCaseInsertion(value: unknown, at = "$"): Fem2dLoadCaseInsertion {
  const row = femFem2dDiffGuardClosed(femFem2dDiffGuardObject(value, at), at, ["index", "row"]);
  return {
    index: femFem2dDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
    row: parseFemLoadCase(row["row"], `${at}.row`),
  };
}

export function parseFem2dLoadCaseRelocation(value: unknown, at = "$"): Fem2dLoadCaseRelocation {
  const row = femFem2dDiffGuardClosed(femFem2dDiffGuardObject(value, at), at, ["id", "from", "to"]);
  return {
    id: femFem2dDiffGuardString(row["id"], `${at}.id`),
    from: femFem2dDiffGuardInteger(row["from"], `${at}.from`, {"minimum": 0}),
    to: femFem2dDiffGuardInteger(row["to"], `${at}.to`, {"minimum": 0}),
  };
}

export function parseFem2dLoadCaseRemoval(value: unknown, at = "$"): Fem2dLoadCaseRemoval {
  const row = femFem2dDiffGuardClosed(femFem2dDiffGuardObject(value, at), at, ["id", "index"]);
  return {
    id: femFem2dDiffGuardString(row["id"], `${at}.id`),
    index: femFem2dDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
  };
}

export function parseFem2dLoadCasesModification(value: unknown, at = "$"): Fem2dLoadCasesModification {
  const row = femFem2dDiffGuardClosed(femFem2dDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: femFem2dDiffGuardString(row["id"], `${at}.id`),
    patch: parseFem2dLoadCasePatch(row["patch"], `${at}.patch`),
  };
}

export function parseFem2dLoadInsertion(value: unknown, at = "$"): Fem2dLoadInsertion {
  const row = femFem2dDiffGuardClosed(femFem2dDiffGuardObject(value, at), at, ["index", "row"]);
  return {
    index: femFem2dDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
    row: parseFemLoad(row["row"], `${at}.row`),
  };
}

export function parseFem2dLoadRelocation(value: unknown, at = "$"): Fem2dLoadRelocation {
  const row = femFem2dDiffGuardClosed(femFem2dDiffGuardObject(value, at), at, ["id", "from", "to"]);
  return {
    id: femFem2dDiffGuardString(row["id"], `${at}.id`),
    from: femFem2dDiffGuardInteger(row["from"], `${at}.from`, {"minimum": 0}),
    to: femFem2dDiffGuardInteger(row["to"], `${at}.to`, {"minimum": 0}),
  };
}

export function parseFem2dLoadRemoval(value: unknown, at = "$"): Fem2dLoadRemoval {
  const row = femFem2dDiffGuardClosed(femFem2dDiffGuardObject(value, at), at, ["id", "index"]);
  return {
    id: femFem2dDiffGuardString(row["id"], `${at}.id`),
    index: femFem2dDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
  };
}

export function parseFem2dLoadsModification(value: unknown, at = "$"): Fem2dLoadsModification {
  const row = femFem2dDiffGuardClosed(femFem2dDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: femFem2dDiffGuardString(row["id"], `${at}.id`),
    patch: parseFemLoad(row["patch"], `${at}.patch`),
  };
}

export function parseFem2dMaterialInsertion(value: unknown, at = "$"): Fem2dMaterialInsertion {
  const row = femFem2dDiffGuardClosed(femFem2dDiffGuardObject(value, at), at, ["index", "row"]);
  return {
    index: femFem2dDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
    row: parseFemMaterial(row["row"], `${at}.row`),
  };
}

export function parseFem2dMaterialRelocation(value: unknown, at = "$"): Fem2dMaterialRelocation {
  const row = femFem2dDiffGuardClosed(femFem2dDiffGuardObject(value, at), at, ["id", "from", "to"]);
  return {
    id: femFem2dDiffGuardString(row["id"], `${at}.id`),
    from: femFem2dDiffGuardInteger(row["from"], `${at}.from`, {"minimum": 0}),
    to: femFem2dDiffGuardInteger(row["to"], `${at}.to`, {"minimum": 0}),
  };
}

export function parseFem2dMaterialRemoval(value: unknown, at = "$"): Fem2dMaterialRemoval {
  const row = femFem2dDiffGuardClosed(femFem2dDiffGuardObject(value, at), at, ["id", "index"]);
  return {
    id: femFem2dDiffGuardString(row["id"], `${at}.id`),
    index: femFem2dDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
  };
}

export function parseFem2dMaterialsModification(value: unknown, at = "$"): Fem2dMaterialsModification {
  const row = femFem2dDiffGuardClosed(femFem2dDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: femFem2dDiffGuardString(row["id"], `${at}.id`),
    patch: parseFemMaterial(row["patch"], `${at}.patch`),
  };
}

export function parseFem2dNodeInsertion(value: unknown, at = "$"): Fem2dNodeInsertion {
  const row = femFem2dDiffGuardClosed(femFem2dDiffGuardObject(value, at), at, ["index", "row"]);
  return {
    index: femFem2dDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
    row: parseFemNode(row["row"], `${at}.row`),
  };
}

export function parseFem2dNodeRelocation(value: unknown, at = "$"): Fem2dNodeRelocation {
  const row = femFem2dDiffGuardClosed(femFem2dDiffGuardObject(value, at), at, ["id", "from", "to"]);
  return {
    id: femFem2dDiffGuardString(row["id"], `${at}.id`),
    from: femFem2dDiffGuardInteger(row["from"], `${at}.from`, {"minimum": 0}),
    to: femFem2dDiffGuardInteger(row["to"], `${at}.to`, {"minimum": 0}),
  };
}

export function parseFem2dNodeRemoval(value: unknown, at = "$"): Fem2dNodeRemoval {
  const row = femFem2dDiffGuardClosed(femFem2dDiffGuardObject(value, at), at, ["id", "index"]);
  return {
    id: femFem2dDiffGuardString(row["id"], `${at}.id`),
    index: femFem2dDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
  };
}

export function parseFem2dNodesModification(value: unknown, at = "$"): Fem2dNodesModification {
  const row = femFem2dDiffGuardClosed(femFem2dDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: femFem2dDiffGuardString(row["id"], `${at}.id`),
    patch: parseFemNode(row["patch"], `${at}.patch`),
  };
}

export function parseFem2dRegionInsertion(value: unknown, at = "$"): Fem2dRegionInsertion {
  const row = femFem2dDiffGuardClosed(femFem2dDiffGuardObject(value, at), at, ["index", "row"]);
  return {
    index: femFem2dDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
    row: parseFemRegion(row["row"], `${at}.row`),
  };
}

export function parseFem2dRegionRelocation(value: unknown, at = "$"): Fem2dRegionRelocation {
  const row = femFem2dDiffGuardClosed(femFem2dDiffGuardObject(value, at), at, ["id", "from", "to"]);
  return {
    id: femFem2dDiffGuardString(row["id"], `${at}.id`),
    from: femFem2dDiffGuardInteger(row["from"], `${at}.from`, {"minimum": 0}),
    to: femFem2dDiffGuardInteger(row["to"], `${at}.to`, {"minimum": 0}),
  };
}

export function parseFem2dRegionRemoval(value: unknown, at = "$"): Fem2dRegionRemoval {
  const row = femFem2dDiffGuardClosed(femFem2dDiffGuardObject(value, at), at, ["id", "index"]);
  return {
    id: femFem2dDiffGuardString(row["id"], `${at}.id`),
    index: femFem2dDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
  };
}

export function parseFem2dRegionsModification(value: unknown, at = "$"): Fem2dRegionsModification {
  const row = femFem2dDiffGuardClosed(femFem2dDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: femFem2dDiffGuardString(row["id"], `${at}.id`),
    patch: parseFemRegion(row["patch"], `${at}.patch`),
  };
}

export function parseFem2dSectionInsertion(value: unknown, at = "$"): Fem2dSectionInsertion {
  const row = femFem2dDiffGuardClosed(femFem2dDiffGuardObject(value, at), at, ["index", "row"]);
  return {
    index: femFem2dDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
    row: parseFemSection(row["row"], `${at}.row`),
  };
}

export function parseFem2dSectionRelocation(value: unknown, at = "$"): Fem2dSectionRelocation {
  const row = femFem2dDiffGuardClosed(femFem2dDiffGuardObject(value, at), at, ["id", "from", "to"]);
  return {
    id: femFem2dDiffGuardString(row["id"], `${at}.id`),
    from: femFem2dDiffGuardInteger(row["from"], `${at}.from`, {"minimum": 0}),
    to: femFem2dDiffGuardInteger(row["to"], `${at}.to`, {"minimum": 0}),
  };
}

export function parseFem2dSectionRemoval(value: unknown, at = "$"): Fem2dSectionRemoval {
  const row = femFem2dDiffGuardClosed(femFem2dDiffGuardObject(value, at), at, ["id", "index"]);
  return {
    id: femFem2dDiffGuardString(row["id"], `${at}.id`),
    index: femFem2dDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
  };
}

export function parseFem2dSectionsModification(value: unknown, at = "$"): Fem2dSectionsModification {
  const row = femFem2dDiffGuardClosed(femFem2dDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: femFem2dDiffGuardString(row["id"], `${at}.id`),
    patch: parseFemSection(row["patch"], `${at}.patch`),
  };
}

export function parseFem2dSupportInsertion(value: unknown, at = "$"): Fem2dSupportInsertion {
  const row = femFem2dDiffGuardClosed(femFem2dDiffGuardObject(value, at), at, ["index", "row"]);
  return {
    index: femFem2dDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
    row: parseFemSupport(row["row"], `${at}.row`),
  };
}

export function parseFem2dSupportRelocation(value: unknown, at = "$"): Fem2dSupportRelocation {
  const row = femFem2dDiffGuardClosed(femFem2dDiffGuardObject(value, at), at, ["id", "from", "to"]);
  return {
    id: femFem2dDiffGuardString(row["id"], `${at}.id`),
    from: femFem2dDiffGuardInteger(row["from"], `${at}.from`, {"minimum": 0}),
    to: femFem2dDiffGuardInteger(row["to"], `${at}.to`, {"minimum": 0}),
  };
}

export function parseFem2dSupportRemoval(value: unknown, at = "$"): Fem2dSupportRemoval {
  const row = femFem2dDiffGuardClosed(femFem2dDiffGuardObject(value, at), at, ["id", "index"]);
  return {
    id: femFem2dDiffGuardString(row["id"], `${at}.id`),
    index: femFem2dDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
  };
}

export function parseFem2dSupportsModification(value: unknown, at = "$"): Fem2dSupportsModification {
  const row = femFem2dDiffGuardClosed(femFem2dDiffGuardObject(value, at), at, ["id", "patch"]);
  return {
    id: femFem2dDiffGuardString(row["id"], `${at}.id`),
    patch: parseFemSupport(row["patch"], `${at}.patch`),
  };
}
