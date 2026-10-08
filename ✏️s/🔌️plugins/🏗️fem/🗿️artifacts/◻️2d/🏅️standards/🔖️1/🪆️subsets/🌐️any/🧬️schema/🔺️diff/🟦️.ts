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
  const row = femFem2dDiffGuardObject(value, at);
  return {
    removed: parseFemRemovals(row["removed"], `${at}.removed`),
    inserted: parseFemInsertions(row["inserted"], `${at}.inserted`, parseFemNode),
    moved: parseFemRelocations(row["moved"], `${at}.moved`),
    modified: parseFemModifications(row["modified"], `${at}.modified`, parseFemNode),
  };
}

export function parseFem2dElementsDelta(value: unknown, at = "$"): Fem2dElementsDelta {
  const row = femFem2dDiffGuardObject(value, at);
  return {
    removed: parseFemRemovals(row["removed"], `${at}.removed`),
    inserted: parseFemInsertions(row["inserted"], `${at}.inserted`, parseFemElement),
    moved: parseFemRelocations(row["moved"], `${at}.moved`),
    modified: parseFemModifications(row["modified"], `${at}.modified`, parseFemElement),
  };
}

export function parseFem2dRegionsDelta(value: unknown, at = "$"): Fem2dRegionsDelta {
  const row = femFem2dDiffGuardObject(value, at);
  return {
    removed: parseFemRemovals(row["removed"], `${at}.removed`),
    inserted: parseFemInsertions(row["inserted"], `${at}.inserted`, parseFemRegion),
    moved: parseFemRelocations(row["moved"], `${at}.moved`),
    modified: parseFemModifications(row["modified"], `${at}.modified`, parseFemRegion),
  };
}

export function parseFem2dMaterialsDelta(value: unknown, at = "$"): Fem2dMaterialsDelta {
  const row = femFem2dDiffGuardObject(value, at);
  return {
    removed: parseFemRemovals(row["removed"], `${at}.removed`),
    inserted: parseFemInsertions(row["inserted"], `${at}.inserted`, parseFemMaterial),
    moved: parseFemRelocations(row["moved"], `${at}.moved`),
    modified: parseFemModifications(row["modified"], `${at}.modified`, parseFemMaterial),
  };
}

export function parseFem2dSectionsDelta(value: unknown, at = "$"): Fem2dSectionsDelta {
  const row = femFem2dDiffGuardObject(value, at);
  return {
    removed: parseFemRemovals(row["removed"], `${at}.removed`),
    inserted: parseFemInsertions(row["inserted"], `${at}.inserted`, parseFemSection),
    moved: parseFemRelocations(row["moved"], `${at}.moved`),
    modified: parseFemModifications(row["modified"], `${at}.modified`, parseFemSection),
  };
}

export function parseFem2dSupportsDelta(value: unknown, at = "$"): Fem2dSupportsDelta {
  const row = femFem2dDiffGuardObject(value, at);
  return {
    removed: parseFemRemovals(row["removed"], `${at}.removed`),
    inserted: parseFemInsertions(row["inserted"], `${at}.inserted`, parseFemSupport),
    moved: parseFemRelocations(row["moved"], `${at}.moved`),
    modified: parseFemModifications(row["modified"], `${at}.modified`, parseFemSupport),
  };
}

export function parseFem2dLoadCasesDelta(value: unknown, at = "$"): Fem2dLoadCasesDelta {
  const row = femFem2dDiffGuardObject(value, at);
  return {
    removed: parseFemRemovals(row["removed"], `${at}.removed`),
    inserted: parseFemInsertions(row["inserted"], `${at}.inserted`, parseFemLoadCase),
    moved: parseFemRelocations(row["moved"], `${at}.moved`),
    modified: parseFemModifications(row["modified"], `${at}.modified`, parseFem2dLoadCasePatch),
  };
}

export function parseFem2dCombinationsDelta(value: unknown, at = "$"): Fem2dCombinationsDelta {
  const row = femFem2dDiffGuardObject(value, at);
  return {
    removed: parseFemRemovals(row["removed"], `${at}.removed`),
    inserted: parseFemInsertions(row["inserted"], `${at}.inserted`, parseFemCombination),
    moved: parseFemRelocations(row["moved"], `${at}.moved`),
    modified: parseFemModifications(row["modified"], `${at}.modified`, parseFemCombination),
  };
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
    removed: parseFemRemovals(row["removed"], `${at}.removed`),
    inserted: parseFemInsertions(row["inserted"], `${at}.inserted`, parseFemLoad),
    moved: parseFemRelocations(row["moved"], `${at}.moved`),
    modified: parseFemModifications(row["modified"], `${at}.modified`, parseFemLoad),
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
