/** 🧬️ Lowpoly diff schema — sparse field delta. */
import { parseArtifactChild, type ArtifactChild } from "../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🪆️child/🧬️schema/🟦️.ts";
import {
  parseLowpolyArtifact,
  parseLowpolyObject,
  parseLowpolyPaintLayer,
  parseLowpolyTransform,
  type LowpolyArtifact,
  type LowpolyObject,
  type LowpolyPaintLayer,
  type LowpolyTransform,
} from "../🟦️.ts";

export interface LowpolyDiff {
  /** @state artifact */
  artifact?: LowpolyArtifact | null;
  /** @state artifact */
  schema?: string | null;
  /** @state artifact */
  objects?: LowpolyObjectsDelta | null;
}

export interface LowpolyObjectsDelta {
  added: LowpolyObject[];
  removed: string[];
  patched: LowpolyObjectPatchEntry[];
  reordered: string[] | null;
}

export interface LowpolyObjectPatchEntry {
  id: string;
  patch: LowpolyObjectPatch;
  paintLayers: LowpolyPaintLayersDelta | null;
}

export interface LowpolyObjectPatch {
  name: string | null;
  smoothShading: boolean | null;
  transform: LowpolyTransform | null;
  mesh: ArtifactChild | null;
}

export interface LowpolyPaintLayersDelta {
  added: LowpolyIndexedPaintLayer[];
  removed: number[];
  patched: LowpolyIndexedPaintLayerPatch[];
  strokes: LowpolyPaintStrokeAt[];
}

export interface LowpolyIndexedPaintLayer {
  index: number;
  layer: LowpolyPaintLayer;
}

export interface LowpolyIndexedPaintLayerPatch {
  index: number;
  patch: LowpolyPaintLayerPatch;
}

export interface LowpolyPaintLayerPatch {
  name: string | null;
  visible: boolean | null;
  opacity: number | null;
  blendMode: string | null;
}

export interface LowpolyPaintStrokeAt {
  layerIndex: number;
  runs: PixelRun[];
}

export interface PixelRun {
  offset: number;
  bytes: string;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class lowpolyLowpolyDiffGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const lowpolyLowpolyDiffGuardReject = (at: string, why: string): never => {
  throw new lowpolyLowpolyDiffGuardRefusal(at, why);
};

type lowpolyLowpolyDiffGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type lowpolyLowpolyDiffGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type lowpolyLowpolyDiffGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const lowpolyLowpolyDiffGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : lowpolyLowpolyDiffGuardReject(at, "value is not an object");
export const lowpolyLowpolyDiffGuardArray = (value: unknown, at: string, bounds: lowpolyLowpolyDiffGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return lowpolyLowpolyDiffGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) lowpolyLowpolyDiffGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) lowpolyLowpolyDiffGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const lowpolyLowpolyDiffGuardString = (value: unknown, at: string, bounds: lowpolyLowpolyDiffGuardTextBounds = {}): string => {
  if (typeof value !== "string") return lowpolyLowpolyDiffGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) lowpolyLowpolyDiffGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) lowpolyLowpolyDiffGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) lowpolyLowpolyDiffGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const lowpolyLowpolyDiffGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : lowpolyLowpolyDiffGuardReject(at, "value is not a boolean"));
export const lowpolyLowpolyDiffGuardNumber = (value: unknown, at: string, bounds: lowpolyLowpolyDiffGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return lowpolyLowpolyDiffGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) lowpolyLowpolyDiffGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) lowpolyLowpolyDiffGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const lowpolyLowpolyDiffGuardInteger = (value: unknown, at: string, bounds: lowpolyLowpolyDiffGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? lowpolyLowpolyDiffGuardNumber(value, at, bounds) : lowpolyLowpolyDiffGuardReject(at, "value is not an integer");
export const lowpolyLowpolyDiffGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : lowpolyLowpolyDiffGuardReject(at, `value is not one of ${members.join(", ")}`);
export const lowpolyLowpolyDiffGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : lowpolyLowpolyDiffGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseLowpolyDiff(value: unknown, at = "$"): LowpolyDiff {
  const row = lowpolyLowpolyDiffGuardObject(value, at);
  return {
    artifact: row["artifact"] === null ? null : parseLowpolyArtifact(row["artifact"], `${at}.artifact`),
    schema: row["schema"] === null ? null : lowpolyLowpolyDiffGuardString(row["schema"], `${at}.schema`),
    objects: row["objects"] === null ? null : parseLowpolyObjectsDelta(row["objects"], `${at}.objects`),
  };
}

export function parseLowpolyObjectsDelta(value: unknown, at = "$"): LowpolyObjectsDelta {
  const row = lowpolyLowpolyDiffGuardObject(value, at);
  return {
    added: lowpolyLowpolyDiffGuardArray(row["added"], `${at}.added`).map((item, index) => parseLowpolyObject(item, `${at}.added[${index}]`)),
    removed: lowpolyLowpolyDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => lowpolyLowpolyDiffGuardString(item, `${at}.removed[${index}]`)),
    patched: lowpolyLowpolyDiffGuardArray(row["patched"], `${at}.patched`).map((item, index) => parseLowpolyObjectPatchEntry(item, `${at}.patched[${index}]`)),
    reordered: row["reordered"] === null ? null : lowpolyLowpolyDiffGuardArray(row["reordered"], `${at}.reordered`).map((item, index) => lowpolyLowpolyDiffGuardString(item, `${at}.reordered[${index}]`)),
  };
}

export function parseLowpolyObjectPatchEntry(value: unknown, at = "$"): LowpolyObjectPatchEntry {
  const row = lowpolyLowpolyDiffGuardObject(value, at);
  return {
    id: lowpolyLowpolyDiffGuardString(row["id"], `${at}.id`),
    patch: parseLowpolyObjectPatch(row["patch"], `${at}.patch`),
    paintLayers: row["paintLayers"] === null ? null : parseLowpolyPaintLayersDelta(row["paintLayers"], `${at}.paintLayers`),
  };
}

export function parseLowpolyObjectPatch(value: unknown, at = "$"): LowpolyObjectPatch {
  const row = lowpolyLowpolyDiffGuardObject(value, at);
  return {
    name: row["name"] === null ? null : lowpolyLowpolyDiffGuardString(row["name"], `${at}.name`),
    smoothShading: row["smoothShading"] === null ? null : lowpolyLowpolyDiffGuardBoolean(row["smoothShading"], `${at}.smoothShading`),
    transform: row["transform"] === null ? null : parseLowpolyTransform(row["transform"], `${at}.transform`),
    mesh: row["mesh"] === null ? null : parseArtifactChild(row["mesh"]),
  };
}

export function parseLowpolyPaintLayersDelta(value: unknown, at = "$"): LowpolyPaintLayersDelta {
  const row = lowpolyLowpolyDiffGuardObject(value, at);
  return {
    added: lowpolyLowpolyDiffGuardArray(row["added"], `${at}.added`).map((item, index) => parseLowpolyIndexedPaintLayer(item, `${at}.added[${index}]`)),
    removed: lowpolyLowpolyDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => lowpolyLowpolyDiffGuardInteger(item, `${at}.removed[${index}]`, {"minimum": 0})),
    patched: lowpolyLowpolyDiffGuardArray(row["patched"], `${at}.patched`).map((item, index) => parseLowpolyIndexedPaintLayerPatch(item, `${at}.patched[${index}]`)),
    strokes: lowpolyLowpolyDiffGuardArray(row["strokes"], `${at}.strokes`).map((item, index) => parseLowpolyPaintStrokeAt(item, `${at}.strokes[${index}]`)),
  };
}

export function parseLowpolyIndexedPaintLayer(value: unknown, at = "$"): LowpolyIndexedPaintLayer {
  const row = lowpolyLowpolyDiffGuardObject(value, at);
  return {
    index: lowpolyLowpolyDiffGuardInteger(row["index"], `${at}.index`, { minimum: 0 }),
    layer: parseLowpolyPaintLayer(row["layer"], `${at}.layer`),
  };
}

export function parseLowpolyIndexedPaintLayerPatch(value: unknown, at = "$"): LowpolyIndexedPaintLayerPatch {
  const row = lowpolyLowpolyDiffGuardObject(value, at);
  return {
    index: lowpolyLowpolyDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
    patch: parseLowpolyPaintLayerPatch(row["patch"], `${at}.patch`),
  };
}

export function parseLowpolyPaintLayerPatch(value: unknown, at = "$"): LowpolyPaintLayerPatch {
  const row = lowpolyLowpolyDiffGuardObject(value, at);
  return {
    name: row["name"] === null ? null : lowpolyLowpolyDiffGuardString(row["name"], `${at}.name`),
    visible: row["visible"] === null ? null : lowpolyLowpolyDiffGuardBoolean(row["visible"], `${at}.visible`),
    opacity: row["opacity"] === null ? null : lowpolyLowpolyDiffGuardNumber(row["opacity"], `${at}.opacity`),
    blendMode: row["blendMode"] === null ? null : lowpolyLowpolyDiffGuardString(row["blendMode"], `${at}.blendMode`),
  };
}

export function parseLowpolyPaintStrokeAt(value: unknown, at = "$"): LowpolyPaintStrokeAt {
  const row = lowpolyLowpolyDiffGuardObject(value, at);
  return {
    layerIndex: lowpolyLowpolyDiffGuardInteger(row["layerIndex"], `${at}.layerIndex`, {"minimum": 0}),
    runs: lowpolyLowpolyDiffGuardArray(row["runs"], `${at}.runs`).map((item, index) => parsePixelRun(item, `${at}.runs[${index}]`)),
  };
}

export function parsePixelRun(value: unknown, at = "$"): PixelRun {
  const row = lowpolyLowpolyDiffGuardObject(value, at);
  return {
    offset: lowpolyLowpolyDiffGuardInteger(row["offset"], `${at}.offset`, {"minimum": 0}),
    bytes: lowpolyLowpolyDiffGuardString(row["bytes"], `${at}.bytes`),
  };
}
