import {parseFemNode,parseFemElement,parseFemSolid,parseFemMaterial,parseFemSection,parseFemSupport,parseFemLoad,parseFemLoadCase,parseFemCombination} from "../📸️snapshot/🟦️.ts";
/** 🧬️ Fem3d diff schema — sparse field delta. */

import type {FemNode,FemElement,FemMaterial,FemSection,FemSupport,FemLoad,FemLoadCase,FemSolid,FemCombination} from "../📸️snapshot/🟦️.ts";
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
  patch: FemNode;
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
  patch: FemElement;
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
  patch: FemMaterial;
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
  patch: FemSection;
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
  patch: FemSolid;
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
  patch: FemSupport;
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
  patch: FemLoad;
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
  patch: FemCombination;
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
    patch: parseFemCombination(row["patch"], `${at}.patch`),
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
    patch: parseFemElement(row["patch"], `${at}.patch`),
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
    patch: parseFemLoad(row["patch"], `${at}.patch`),
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
    patch: parseFemMaterial(row["patch"], `${at}.patch`),
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
    patch: parseFemNode(row["patch"], `${at}.patch`),
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
    patch: parseFemSection(row["patch"], `${at}.patch`),
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
    patch: parseFemSolid(row["patch"], `${at}.patch`),
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
    patch: parseFemSupport(row["patch"], `${at}.patch`),
  };
}
