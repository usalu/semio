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
  const row = femFem3dDiffGuardObject(value, at);
  return {
    removed: parseFemRemovals(row["removed"], `${at}.removed`),
    inserted: parseFemInsertions(row["inserted"], `${at}.inserted`, parseFemNode),
    moved: parseFemRelocations(row["moved"], `${at}.moved`),
    modified: parseFemModifications(row["modified"], `${at}.modified`, parseFemNode),
  };
}

export function parseFem3dElementsDelta(value: unknown, at = "$"): Fem3dElementsDelta {
  const row = femFem3dDiffGuardObject(value, at);
  return {
    removed: parseFemRemovals(row["removed"], `${at}.removed`),
    inserted: parseFemInsertions(row["inserted"], `${at}.inserted`, parseFemElement),
    moved: parseFemRelocations(row["moved"], `${at}.moved`),
    modified: parseFemModifications(row["modified"], `${at}.modified`, parseFemElement),
  };
}

export function parseFem3dMaterialsDelta(value: unknown, at = "$"): Fem3dMaterialsDelta {
  const row = femFem3dDiffGuardObject(value, at);
  return {
    removed: parseFemRemovals(row["removed"], `${at}.removed`),
    inserted: parseFemInsertions(row["inserted"], `${at}.inserted`, parseFemMaterial),
    moved: parseFemRelocations(row["moved"], `${at}.moved`),
    modified: parseFemModifications(row["modified"], `${at}.modified`, parseFemMaterial),
  };
}

export function parseFem3dSectionsDelta(value: unknown, at = "$"): Fem3dSectionsDelta {
  const row = femFem3dDiffGuardObject(value, at);
  return {
    removed: parseFemRemovals(row["removed"], `${at}.removed`),
    inserted: parseFemInsertions(row["inserted"], `${at}.inserted`, parseFemSection),
    moved: parseFemRelocations(row["moved"], `${at}.moved`),
    modified: parseFemModifications(row["modified"], `${at}.modified`, parseFemSection),
  };
}

export function parseFem3dSolidsDelta(value: unknown, at = "$"): Fem3dSolidsDelta {
  const row = femFem3dDiffGuardObject(value, at);
  return {
    removed: parseFemRemovals(row["removed"], `${at}.removed`),
    inserted: parseFemInsertions(row["inserted"], `${at}.inserted`, parseFemSolid),
    moved: parseFemRelocations(row["moved"], `${at}.moved`),
    modified: parseFemModifications(row["modified"], `${at}.modified`, parseFemSolid),
  };
}

export function parseFem3dSupportsDelta(value: unknown, at = "$"): Fem3dSupportsDelta {
  const row = femFem3dDiffGuardObject(value, at);
  return {
    removed: parseFemRemovals(row["removed"], `${at}.removed`),
    inserted: parseFemInsertions(row["inserted"], `${at}.inserted`, parseFemSupport),
    moved: parseFemRelocations(row["moved"], `${at}.moved`),
    modified: parseFemModifications(row["modified"], `${at}.modified`, parseFemSupport),
  };
}

export function parseFem3dLoadCasesDelta(value: unknown, at = "$"): Fem3dLoadCasesDelta {
  const row = femFem3dDiffGuardObject(value, at);
  return {
    removed: parseFemRemovals(row["removed"], `${at}.removed`),
    inserted: parseFemInsertions(row["inserted"], `${at}.inserted`, parseFemLoadCase),
    moved: parseFemRelocations(row["moved"], `${at}.moved`),
    modified: parseFemModifications(row["modified"], `${at}.modified`, parseFem3dLoadCasePatch),
  };
}

export function parseFem3dCombinationsDelta(value: unknown, at = "$"): Fem3dCombinationsDelta {
  const row = femFem3dDiffGuardObject(value, at);
  return {
    removed: parseFemRemovals(row["removed"], `${at}.removed`),
    inserted: parseFemInsertions(row["inserted"], `${at}.inserted`, parseFemCombination),
    moved: parseFemRelocations(row["moved"], `${at}.moved`),
    modified: parseFemModifications(row["modified"], `${at}.modified`, parseFemCombination),
  };
}

export function parseFem3dLoadCasePatch(value: unknown, at = "$"): Fem3dLoadCasePatch {
  const row = femFem3dDiffGuardObject(value, at);
  return {
    name: row["name"] === undefined || row["name"] === null ? undefined : femFem3dDiffGuardString(row["name"], `${at}.name`),
    selfWeight: row["selfWeight"] === undefined || row["selfWeight"] === null ? undefined : femFem3dDiffGuardBoolean(row["selfWeight"], `${at}.selfWeight`),
    loads: row["loads"] === undefined || row["loads"] === null ? undefined : parseFem3dLoadsDelta(row["loads"], `${at}.loads`),
  };
}

export function parseFem3dLoadsDelta(value: unknown, at = "$"): Fem3dLoadsDelta {
  const row = femFem3dDiffGuardObject(value, at);
  return {
    removed: parseFemRemovals(row["removed"], `${at}.removed`),
    inserted: parseFemInsertions(row["inserted"], `${at}.inserted`, parseFemLoad),
    moved: parseFemRelocations(row["moved"], `${at}.moved`),
    modified: parseFemModifications(row["modified"], `${at}.modified`, parseFemLoad),
  };
}

export function parseFem3dAnalysisPatch(value: unknown, at = "$"): Fem3dAnalysisPatch {
  const row = femFem3dDiffGuardObject(value, at);
  return {
    modalCount: row["modalCount"] === undefined || row["modalCount"] === null ? undefined : femFem3dDiffGuardInteger(row["modalCount"], `${at}.modalCount`, { minimum: 1 }),
    bucklingCount: row["bucklingCount"] === undefined || row["bucklingCount"] === null ? undefined : femFem3dDiffGuardInteger(row["bucklingCount"], `${at}.bucklingCount`, { minimum: 1 }),
    deformationScale: row["deformationScale"] === undefined || row["deformationScale"] === null ? undefined : femFem3dDiffGuardNumber(row["deformationScale"], `${at}.deformationScale`),
  };
}
