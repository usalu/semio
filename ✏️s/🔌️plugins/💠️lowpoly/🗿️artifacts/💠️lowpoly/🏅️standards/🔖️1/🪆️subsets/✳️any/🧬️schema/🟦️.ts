/** 🧬️ Lowpoly artifact schema — every field with its state class. */
import { parseArtifactChild, type ArtifactChild } from "../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🪆️child/🧬️schema/🟦️.ts";

export interface LowpolyArtifact {
  /** @state artifact */
  schema: string;
  /** @state artifact */
  objects: LowpolyObject[];
}

export interface LowpolySelectionTargets {
  mesh: boolean;
  vertex: boolean;
  edge: boolean;
  face: boolean;
}

export interface LowpolySelection {
  targets: LowpolySelectionTargets;
  keys: string[];
  mode: string;
  ids: number[];
}

export interface LowpolyTransform {
  position: [number, number, number];
  rotation: [number, number, number];
  scale: [number, number, number];
}

export interface LowpolyPaintLayer {
  name: string;
  visible: boolean;
  opacity: number;
  blendMode: string;
  pixels: string;
}

export interface LowpolyObject {
  id: string;
  name: string;
  transform: LowpolyTransform;
  smoothShading: boolean;
  /** `null` when the object owns no mesh yet — confirmed against the `create-object` mutation fixture. */
  mesh: ArtifactChild | null;
  paintLayers: LowpolyPaintLayer[];
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class lowpolyLowpolyArtifactGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const lowpolyLowpolyArtifactGuardReject = (at: string, why: string): never => {
  throw new lowpolyLowpolyArtifactGuardRefusal(at, why);
};

type lowpolyLowpolyArtifactGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type lowpolyLowpolyArtifactGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type lowpolyLowpolyArtifactGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const lowpolyLowpolyArtifactGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : lowpolyLowpolyArtifactGuardReject(at, "value is not an object");
export const lowpolyLowpolyArtifactGuardArray = (value: unknown, at: string, bounds: lowpolyLowpolyArtifactGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return lowpolyLowpolyArtifactGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) lowpolyLowpolyArtifactGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) lowpolyLowpolyArtifactGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const lowpolyLowpolyArtifactGuardString = (value: unknown, at: string, bounds: lowpolyLowpolyArtifactGuardTextBounds = {}): string => {
  if (typeof value !== "string") return lowpolyLowpolyArtifactGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) lowpolyLowpolyArtifactGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) lowpolyLowpolyArtifactGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) lowpolyLowpolyArtifactGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const lowpolyLowpolyArtifactGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : lowpolyLowpolyArtifactGuardReject(at, "value is not a boolean"));
export const lowpolyLowpolyArtifactGuardNumber = (value: unknown, at: string, bounds: lowpolyLowpolyArtifactGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return lowpolyLowpolyArtifactGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) lowpolyLowpolyArtifactGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) lowpolyLowpolyArtifactGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const lowpolyLowpolyArtifactGuardInteger = (value: unknown, at: string, bounds: lowpolyLowpolyArtifactGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? lowpolyLowpolyArtifactGuardNumber(value, at, bounds) : lowpolyLowpolyArtifactGuardReject(at, "value is not an integer");
export const lowpolyLowpolyArtifactGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : lowpolyLowpolyArtifactGuardReject(at, `value is not one of ${members.join(", ")}`);
export const lowpolyLowpolyArtifactGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : lowpolyLowpolyArtifactGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseLowpolyArtifact(value: unknown, at = "$"): LowpolyArtifact {
  const row = lowpolyLowpolyArtifactGuardObject(value, at);
  return {
    schema: lowpolyLowpolyArtifactGuardString(row["schema"], `${at}.schema`),
    objects: lowpolyLowpolyArtifactGuardArray(row["objects"], `${at}.objects`).map((item, index) => parseLowpolyObject(item, `${at}.objects[${index}]`)),
  };
}

export function parseLowpolyObject(value: unknown, at = "$"): LowpolyObject {
  const row = lowpolyLowpolyArtifactGuardObject(value, at);
  return {
    id: lowpolyLowpolyArtifactGuardString(row["id"], `${at}.id`),
    name: lowpolyLowpolyArtifactGuardString(row["name"], `${at}.name`),
    transform: parseLowpolyTransform(row["transform"], `${at}.transform`),
    smoothShading: lowpolyLowpolyArtifactGuardBoolean(row["smoothShading"], `${at}.smoothShading`),
    mesh: row["mesh"] === null ? null : parseArtifactChild(row["mesh"]),
    paintLayers: lowpolyLowpolyArtifactGuardArray(row["paintLayers"], `${at}.paintLayers`).map((item, index) => parseLowpolyPaintLayer(item, `${at}.paintLayers[${index}]`)),
  };
}

export function parseLowpolyTransform(value: unknown, at = "$"): LowpolyTransform {
  const row = lowpolyLowpolyArtifactGuardObject(value, at);
  return {
    position: lowpolyLowpolyArtifactGuardArray(row["position"], `${at}.position`, {"minItems": 3, "maxItems": 3}).map((item, index) => lowpolyLowpolyArtifactGuardNumber(item, `${at}.position[${index}]`)),
    rotation: lowpolyLowpolyArtifactGuardArray(row["rotation"], `${at}.rotation`, {"minItems": 3, "maxItems": 3}).map((item, index) => lowpolyLowpolyArtifactGuardNumber(item, `${at}.rotation[${index}]`)),
    scale: lowpolyLowpolyArtifactGuardArray(row["scale"], `${at}.scale`, {"minItems": 3, "maxItems": 3}).map((item, index) => lowpolyLowpolyArtifactGuardNumber(item, `${at}.scale[${index}]`)),
  };
}

export function parseLowpolyPaintLayer(value: unknown, at = "$"): LowpolyPaintLayer {
  const row = lowpolyLowpolyArtifactGuardObject(value, at);
  return {
    name: lowpolyLowpolyArtifactGuardString(row["name"], `${at}.name`),
    visible: lowpolyLowpolyArtifactGuardBoolean(row["visible"], `${at}.visible`),
    opacity: lowpolyLowpolyArtifactGuardNumber(row["opacity"], `${at}.opacity`),
    blendMode: lowpolyLowpolyArtifactGuardString(row["blendMode"], `${at}.blendMode`),
    pixels: lowpolyLowpolyArtifactGuardString(row["pixels"], `${at}.pixels`),
  };
}

export function parseLowpolySelection(value: unknown, at = "$"): LowpolySelection {
  const row = lowpolyLowpolyArtifactGuardObject(value, at);
  return {
    targets: parseLowpolySelectionTargets(row["targets"], `${at}.targets`),
    keys: lowpolyLowpolyArtifactGuardArray(row["keys"], `${at}.keys`).map((item, index) => lowpolyLowpolyArtifactGuardString(item, `${at}.keys[${index}]`)),
    mode: lowpolyLowpolyArtifactGuardString(row["mode"], `${at}.mode`),
    ids: lowpolyLowpolyArtifactGuardArray(row["ids"], `${at}.ids`).map((item, index) => lowpolyLowpolyArtifactGuardInteger(item, `${at}.ids[${index}]`, {"minimum": 0})),
  };
}

export function parseLowpolySelectionTargets(value: unknown, at = "$"): LowpolySelectionTargets {
  const row = lowpolyLowpolyArtifactGuardObject(value, at);
  return {
    mesh: lowpolyLowpolyArtifactGuardBoolean(row["mesh"], `${at}.mesh`),
    vertex: lowpolyLowpolyArtifactGuardBoolean(row["vertex"], `${at}.vertex`),
    edge: lowpolyLowpolyArtifactGuardBoolean(row["edge"], `${at}.edge`),
    face: lowpolyLowpolyArtifactGuardBoolean(row["face"], `${at}.face`),
  };
}
