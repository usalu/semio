/** 🧬️ CAD document composed from four model slots and a drawing collection. */
import { parseArtifactChild, type ArtifactChild } from "../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🪆️child/🧬️schema/🟦️.ts";
export type { ArtifactChild } from "../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🪆️child/🧬️schema/🟦️.ts";

export interface CadReference {
  id: string;
  sourceUrl: string;
  mediaKind: string;
  origin: [number, number, number];
  orientation: [number, number, number, number] | null;
  scale: number | null;
  widthWorld: number;
  hidden: boolean;
  locked: boolean;
  opacity: number | null;
}

export type CadReferenceList = CadReference[];
export interface CadNode { id: string; label: string; kind: string }

export interface CadArtifact {
  /** @state artifact */ schema: string;
  /** @state artifact */ id: string;
  /** @state artifact @child kind=s.stdio.semio */ shapeModel?: ArtifactChild;
  /** @state artifact @child kind=s.stdio.semio */ buildingModel?: ArtifactChild;
  /** @state artifact @child kind=s.stdio.semio */ energyModel?: ArtifactChild;
  /** @state artifact @child kind=s.stdio.semio */ structureClassicModel?: ArtifactChild;
  /** @state artifact @child kind=s.stdio.semio many */ drawings: ArtifactChild[];
  /** @state artifact */ referencesByModelDefinitionId: Record<string, CadReferenceList>;
  /** @state artifact */ nodes: CadNode[];
}

function object(value: unknown, at: string): Record<string, unknown> {
  if (value === null || typeof value !== "object" || Array.isArray(value)) throw new Error(`${at}: value must be an object`);
  return value as Record<string, unknown>;
}

function exact(row: Record<string, unknown>, allowed: readonly string[], required: readonly string[], at: string): void {
  const keys = Object.keys(row);
  const foreign = keys.find((key) => !allowed.includes(key));
  if (foreign !== undefined) throw new Error(`${at}: unknown field ${foreign}`);
  const missing = required.find((key) => !Object.hasOwn(row, key));
  if (missing !== undefined) throw new Error(`${at}: missing field ${missing}`);
}

function string(value: unknown, at: string): string {
  if (typeof value !== "string") throw new Error(`${at}: value must be a string`);
  return value;
}

function number(value: unknown, at: string): number {
  if (typeof value !== "number" || !Number.isFinite(value)) throw new Error(`${at}: value must be a finite number`);
  return value;
}

function boolean(value: unknown, at: string): boolean {
  if (typeof value !== "boolean") throw new Error(`${at}: value must be a boolean`);
  return value;
}

function array(value: unknown, at: string): unknown[] {
  if (!Array.isArray(value)) throw new Error(`${at}: value must be an array`);
  return value;
}

function fixedNumbers<T extends number>(value: unknown, size: T, at: string): number[] {
  const values = array(value, at).map((item, index) => number(item, `${at}[${index}]`));
  if (values.length !== size) throw new Error(`${at}: expected ${size} numbers`);
  return values;
}

/** 🪪️ Parses one exact model or drawing identity, including CAD's identity and subtype laws. */
export function parseCadChild(value: unknown, subset: "model" | "drawing", at = "$"): ArtifactChild {
  const child = parseArtifactChild(value);
  if (child.childId !== child.target.artifactId) throw new Error(`${at}: childId must equal target.artifactId`);
  const dialect = child.target.dialect;
  if (dialect.artifactKind !== "s.stdio.semio" || dialect.standard !== "v1" || dialect.subset !== subset) throw new Error(`${at}: expected s.stdio.semio@v1/${subset}`);
  return child;
}

export function parseCadNode(value: unknown, at = "$"): CadNode {
  const row = object(value, at);
  exact(row, ["id", "label", "kind"], ["id", "label", "kind"], at);
  return { id: string(row.id, `${at}.id`), label: string(row.label, `${at}.label`), kind: string(row.kind, `${at}.kind`) };
}

export function parseCadReference(value: unknown, at = "$"): CadReference {
  const row = object(value, at);
  const keys = ["id", "sourceUrl", "mediaKind", "origin", "orientation", "scale", "widthWorld", "hidden", "locked", "opacity"];
  exact(row, keys, keys, at);
  const nullableNumber = (input: unknown, path: string): number | null => input === null ? null : number(input, path);
  return {
    id: string(row.id, `${at}.id`),
    sourceUrl: string(row.sourceUrl, `${at}.sourceUrl`),
    mediaKind: string(row.mediaKind, `${at}.mediaKind`),
    origin: fixedNumbers(row.origin, 3, `${at}.origin`) as [number, number, number],
    orientation: row.orientation === null ? null : fixedNumbers(row.orientation, 4, `${at}.orientation`) as [number, number, number, number],
    scale: nullableNumber(row.scale, `${at}.scale`),
    widthWorld: number(row.widthWorld, `${at}.widthWorld`),
    hidden: boolean(row.hidden, `${at}.hidden`),
    locked: boolean(row.locked, `${at}.locked`),
    opacity: nullableNumber(row.opacity, `${at}.opacity`),
  };
}

export function parseCadReferences(value: unknown, at = "$"): Record<string, CadReferenceList> {
  const row = object(value, at);
  return Object.fromEntries(Object.entries(row).map(([key, list]) => [key, array(list, `${at}.${key}`).map((item, index) => parseCadReference(item, `${at}.${key}[${index}]`))]));
}

/** 🪪️ Parses only persisted CAD fields and rejects former flat geometry and UI selection state. */
export function parseCadArtifact(value: unknown, at = "$"): CadArtifact {
  const row = object(value, at);
  const allowed = ["schema", "id", "shapeModel", "buildingModel", "energyModel", "structureClassicModel", "drawings", "referencesByModelDefinitionId", "nodes"];
  exact(row, allowed, ["schema", "id", "drawings", "referencesByModelDefinitionId", "nodes"], at);
  const result: CadArtifact = {
    schema: string(row.schema, `${at}.schema`),
    id: string(row.id, `${at}.id`),
    drawings: array(row.drawings, `${at}.drawings`).map((item, index) => parseCadChild(item, "drawing", `${at}.drawings[${index}]`)),
    referencesByModelDefinitionId: parseCadReferences(row.referencesByModelDefinitionId, `${at}.referencesByModelDefinitionId`),
    nodes: array(row.nodes, `${at}.nodes`).map((item, index) => parseCadNode(item, `${at}.nodes[${index}]`)),
  };
  for (const key of ["shapeModel", "buildingModel", "energyModel", "structureClassicModel"] as const) {
    if (Object.hasOwn(row, key)) result[key] = parseCadChild(row[key], "model", `${at}.${key}`);
  }
  return result;
}

export const cadContractObject = object;
export const cadContractExact = exact;
export const cadContractString = string;
export const cadContractArray = array;
