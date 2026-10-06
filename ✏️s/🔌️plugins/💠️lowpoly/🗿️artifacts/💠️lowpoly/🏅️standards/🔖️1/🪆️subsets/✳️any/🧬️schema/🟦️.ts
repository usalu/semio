/** 🧬️ Lowpoly artifact schema — every field with its state class. */
import { parseArtifactChild, type ArtifactChild } from "../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🪆️child/🧬️schema/🟦️.ts";
import { type Binary32 } from "../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🔢️ieee754/🟦️.ts";
import { parseBinary32 } from "../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🔢️ieee754/🟦️.ts";
import {parseLowpolyMeshState,type LowpolyMeshState}from"./🕸️mesh/🟦️.ts";
export *from"./🕸️mesh/🟦️.ts";

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
  position: [Binary32, Binary32, Binary32];
  rotation: [Binary32, Binary32, Binary32];
  scale: [Binary32, Binary32, Binary32];
}

export interface LowpolyPaintLayer {
  name: string;
  visible: boolean;
  opacity: Binary32;
  blendMode: string;
  pixels: Uint8Array;
}

export interface LowpolyObject {
  id: string;
  name: string;
  transform: LowpolyTransform;
  smoothShading: boolean;
  /** `null` when the object owns no mesh yet — confirmed against the `create-object` mutation fixture. */
  mesh: ArtifactChild | null;
  paintLayers: LowpolyPaintLayer[];
  /** 📄️ Independent authored source text remains literal even when no managed mesh exists. */
  meshContent: string;
  /** 🕸️ Complete managed mesh state is independent of source and child handles. */
  meshState: LowpolyMeshState|null;
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
    meshContent: lowpolyLowpolyArtifactGuardString(row["meshContent"], `${at}.meshContent`),
    meshState: row["meshState"]===null?null:parseLowpolyMeshState(row["meshState"]),
  };
}

/** 📐️ Exact three-component native binary32 vector. */
export function parseLowpolyVector(value:unknown,at="$"):[Binary32,Binary32,Binary32]{const row=lowpolyLowpolyArtifactGuardArray(value,at,{minItems:3,maxItems:3});return[parseBinary32(row[0]),parseBinary32(row[1]),parseBinary32(row[2])];}
/** 🎨️ Intrinsic persisted pixel octets, independent of any wire encoding. */
export function parseLowpolyPixels(value:unknown):Uint8Array{if(!(value instanceof Uint8Array))throw Error("Lowpoly pixels require owned octets");return value.slice();}
export function parseLowpolyTransform(value: unknown, at = "$"): LowpolyTransform {
  const row = lowpolyLowpolyArtifactGuardObject(value, at);
  return {
    position: parseLowpolyVector(row["position"], `${at}.position`),
    rotation: parseLowpolyVector(row["rotation"], `${at}.rotation`),
    scale: parseLowpolyVector(row["scale"], `${at}.scale`),
  };
}

export function parseLowpolyPaintLayer(value: unknown, at = "$"): LowpolyPaintLayer {
  const row = lowpolyLowpolyArtifactGuardObject(value, at);
  return {
    name: lowpolyLowpolyArtifactGuardString(row["name"], `${at}.name`),
    visible: lowpolyLowpolyArtifactGuardBoolean(row["visible"], `${at}.visible`),
    opacity: parseBinary32(row["opacity"]),
    blendMode: lowpolyLowpolyArtifactGuardString(row["blendMode"], `${at}.blendMode`),
    pixels: parseLowpolyPixels(row["pixels"]),
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
