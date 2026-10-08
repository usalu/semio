/** 🔺️ Sparse CAD document diff over composed child identities and native keyed collection deltas. */
import {
  cadContractArray,
  cadContractBoolean,
  cadContractExact,
  cadContractFixedNumbers,
  cadContractNumber,
  cadContractObject,
  cadContractString,
  parseCadChild,
  parseCadNode,
  parseCadReference,
  type ArtifactChild,
  type CadBrepChild,
  type CadNode,
  type CadReference,
} from "../🟦️.ts";

export interface CadNodePatch { label?: string | null }
export interface CadNodePatchEntry { id: string; patch: CadNodePatch }
export interface CadNodesDelta { added: CadNode[]; removed: string[]; patched: CadNodePatchEntry[]; reordered: string[] | null }
export interface CadModelSlot { child: ArtifactChild | null }
export interface CadDrawingsDelta { added: ArtifactChild[]; removed: string[]; reordered: string[] | null }
export interface CadBrepsDelta { added: CadBrepChild[]; removed: string[]; reordered: string[] | null }
export interface CadOrientationSet { value: [number, number, number, number] | null }
export interface CadScaleSet { value: number | null }
export interface CadOpacitySet { value: number | null }
export interface CadReferencePatch {
  sourceUrl?: string | null;
  mediaKind?: string | null;
  origin?: [number, number, number] | null;
  orientation?: CadOrientationSet | null;
  scale?: CadScaleSet | null;
  widthWorld?: number | null;
  hidden?: boolean | null;
  locked?: boolean | null;
  opacity?: CadOpacitySet | null;
}
export interface CadReferencePatchEntry { id: string; patch: CadReferencePatch }
export interface CadReferencesDelta { added: CadReference[]; removed: string[]; patched: CadReferencePatchEntry[]; reordered: string[] | null }

export interface CadDiff {
  /** @state artifact */ schema?: string | null;
  /** @state artifact */ id?: string | null;
  /** @state artifact @child kind=s.stdio.semio */ shapeModel?: CadModelSlot | null;
  /** @state artifact @child kind=s.stdio.semio */ buildingModel?: CadModelSlot | null;
  /** @state artifact @child kind=s.stdio.semio */ energyModel?: CadModelSlot | null;
  /** @state artifact @child kind=s.stdio.semio */ structureClassicModel?: CadModelSlot | null;
  /** @state artifact @child kind=s.stdio.semio many */ drawings?: CadDrawingsDelta | null;
  /** @state artifact @child kind=s.stdio.semio many */ breps?: CadBrepsDelta | null;
  /** @state artifact */ referencesByModelDefinitionId?: Record<string, CadReferencesDelta> | null;
  /** @state artifact */ nodes?: CadNodesDelta | null;
}

const strings = (value: unknown, at: string): string[] => cadContractArray(value, at).map((item, index) => cadContractString(item, `${at}[${index}]`));
const optionalStrings = (value: unknown, at: string): string[] | null => (value === null ? null : strings(value, at));

function parseNodePatch(value: unknown, at: string): CadNodePatch {
  const row = cadContractObject(value, at);
  cadContractExact(row, ["label"], [], at);
  return Object.hasOwn(row, "label") ? { label: row.label === null ? null : cadContractString(row.label, `${at}.label`) } : {};
}

function parseNodesDelta(value: unknown, at: string): CadNodesDelta {
  const row = cadContractObject(value, at);
  const keys = ["added", "removed", "patched", "reordered"];
  cadContractExact(row, keys, keys, at);
  return {
    added: cadContractArray(row.added, `${at}.added`).map((item, index) => parseCadNode(item, `${at}.added[${index}]`)),
    removed: strings(row.removed, `${at}.removed`),
    patched: cadContractArray(row.patched, `${at}.patched`).map((item, index) => {
      const path = `${at}.patched[${index}]`, patch = cadContractObject(item, path);
      cadContractExact(patch, ["id", "patch"], ["id", "patch"], path);
      return { id: cadContractString(patch.id, `${path}.id`), patch: parseNodePatch(patch.patch, `${path}.patch`) };
    }),
    reordered: optionalStrings(row.reordered, `${at}.reordered`),
  };
}

function parseModelSlot(value: unknown, at: string): CadModelSlot {
  const row = cadContractObject(value, at);
  cadContractExact(row, ["child"], ["child"], at);
  return { child: row.child === null ? null : parseCadChild(row.child, "model", `${at}.child`) };
}

function parseChildrenDelta<S extends "drawing" | "brep">(value: unknown, subset: S, at: string) {
  const row = cadContractObject(value, at);
  const keys = ["added", "removed", "reordered"];
  cadContractExact(row, keys, keys, at);
  return {
    added: cadContractArray(row.added, `${at}.added`).map((item, index) => parseCadChild(item, subset, `${at}.added[${index}]`)),
    removed: strings(row.removed, `${at}.removed`),
    reordered: optionalStrings(row.reordered, `${at}.reordered`),
  };
}

function parseSet<T>(value: unknown, at: string, parse: (input: unknown, path: string) => T): { value: T | null } {
  const row = cadContractObject(value, at);
  cadContractExact(row, ["value"], ["value"], at);
  return { value: row.value === null ? null : parse(row.value, `${at}.value`) };
}

function parseReferencePatch(value: unknown, at: string): CadReferencePatch {
  const row = cadContractObject(value, at);
  cadContractExact(row, ["sourceUrl", "mediaKind", "origin", "orientation", "scale", "widthWorld", "hidden", "locked", "opacity"], [], at);
  const result: CadReferencePatch = {};
  const optional = <T>(key: keyof CadReferencePatch, parse: (input: unknown, path: string) => T) => {
    if (Object.hasOwn(row, key)) (result as Record<string, unknown>)[key] = row[key] === null ? null : parse(row[key], `${at}.${key}`);
  };
  optional("sourceUrl", cadContractString);
  optional("mediaKind", cadContractString);
  optional("origin", (input, path) => cadContractFixedNumbers(input, 3, path) as [number, number, number]);
  optional("orientation", (input, path) => parseSet(input, path, (inner, innerPath) => cadContractFixedNumbers(inner, 4, innerPath) as [number, number, number, number]));
  optional("scale", (input, path) => parseSet(input, path, cadContractNumber));
  optional("widthWorld", cadContractNumber);
  optional("hidden", cadContractBoolean);
  optional("locked", cadContractBoolean);
  optional("opacity", (input, path) => parseSet(input, path, cadContractNumber));
  return result;
}

function parseReferencesDelta(value: unknown, at: string): CadReferencesDelta {
  const row = cadContractObject(value, at);
  const keys = ["added", "removed", "patched", "reordered"];
  cadContractExact(row, keys, keys, at);
  return {
    added: cadContractArray(row.added, `${at}.added`).map((item, index) => parseCadReference(item, `${at}.added[${index}]`)),
    removed: strings(row.removed, `${at}.removed`),
    patched: cadContractArray(row.patched, `${at}.patched`).map((item, index) => {
      const path = `${at}.patched[${index}]`, patch = cadContractObject(item, path);
      cadContractExact(patch, ["id", "patch"], ["id", "patch"], path);
      return { id: cadContractString(patch.id, `${path}.id`), patch: parseReferencePatch(patch.patch, `${path}.patch`) };
    }),
    reordered: optionalStrings(row.reordered, `${at}.reordered`),
  };
}

/** 🪪️ Parses only the sparse fields implemented by native CadDiff. */
export function parseCadDiff(value: unknown, at = "$"): CadDiff {
  const row = cadContractObject(value, at);
  const keys = ["schema", "id", "shapeModel", "buildingModel", "energyModel", "structureClassicModel", "drawings", "breps", "referencesByModelDefinitionId", "nodes"];
  cadContractExact(row, keys, [], at);
  const result: CadDiff = {};
  if (Object.hasOwn(row, "schema")) result.schema = row.schema === null ? null : cadContractString(row.schema, `${at}.schema`);
  if (Object.hasOwn(row, "id")) result.id = row.id === null ? null : cadContractString(row.id, `${at}.id`);
  for (const key of ["shapeModel", "buildingModel", "energyModel", "structureClassicModel"] as const) {
    if (Object.hasOwn(row, key)) result[key] = row[key] === null ? null : parseModelSlot(row[key], `${at}.${key}`);
  }
  if (Object.hasOwn(row, "drawings")) result.drawings = row.drawings === null ? null : parseChildrenDelta(row.drawings, "drawing", `${at}.drawings`);
  if (Object.hasOwn(row, "breps")) result.breps = row.breps === null ? null : parseChildrenDelta(row.breps, "brep", `${at}.breps`);
  if (Object.hasOwn(row, "referencesByModelDefinitionId")) {
    result.referencesByModelDefinitionId = row.referencesByModelDefinitionId === null ? null : Object.fromEntries(Object.entries(cadContractObject(row.referencesByModelDefinitionId, `${at}.referencesByModelDefinitionId`)).map(([model, delta]) => [model, parseReferencesDelta(delta, `${at}.referencesByModelDefinitionId.${model}`)]));
  }
  if (Object.hasOwn(row, "nodes")) result.nodes = row.nodes === null ? null : parseNodesDelta(row.nodes, `${at}.nodes`);
  return result;
}
