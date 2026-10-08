/** 🧬️ Shooting diff schema — sparse typed delta over the artifact: ordered structural edits plus keyed field patches per list, a scene field patch and assignable scalars. */

import type { ArtifactChild } from "../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🪆️child/🧬️schema/🟦️.ts";
import { parseShootingCamera } from "../🟦️.ts";

export interface ShootingDiff {
  /** @state artifact */
  schema?: string;
  /** @state artifact */
  assets?: ShootingAssetsDelta;
  /** @state artifact */
  savedCameras?: ShootingSavedCamerasDelta;
  /** @state artifact */
  scene?: ShootingScenePatch;
  /** @state artifact */
  shots?: ShootingShotsDelta;
  /** @state artifact */
  activeShotId?: string;
  /** @state artifact */
  activeAssetId?: string;
  /** @state artifact @child kind=s.stdio.semio */
  emblem?: ShootingAssigned<ArtifactChild | null>;
}

/** An explicitly assigned optional value: assigning `null` stays distinct from leaving the slot untouched. */
export interface ShootingAssigned<T> {
  value: T;
}

export type ShootingEdit<T> = { edit: "add"; index: number; item: T } | { edit: "remove"; id: string } | { edit: "move"; id: string; index: number };

export interface ShootingAssetsDelta {
  edits: ShootingEdit<ShootingAsset>[];
  patched: ShootingAssetPatchEntry[];
}

export interface ShootingShotsDelta {
  edits: ShootingEdit<ShootingShot>[];
  patched: ShootingShotPatchEntry[];
}

export interface ShootingSavedCamerasDelta {
  edits: ShootingEdit<ShootingSavedCamera>[];
  patched: ShootingSavedCameraPatchEntry[];
}

export interface ShootingAssetPatchEntry {
  id: string;
  patch: ShootingAssetPatch;
}

export interface ShootingShotPatchEntry {
  id: string;
  patch: ShootingShotPatch;
}

export interface ShootingSavedCameraPatchEntry {
  id: string;
  patch: ShootingSavedCameraPatch;
}

export interface ShootingAssetPatch {
  name?: string;
  url?: string;
  format?: string;
  origin?: [number, number, number];
  orientation?: ShootingAssigned<[number, number, number, number] | null>;
  scale?: ShootingAssigned<[number, number, number] | null>;
}

export interface ShootingShotPatch {
  label?: string;
  width?: number;
  height?: number;
  format?: string;
  shape?: string;
  background?: ShootingAssigned<string | null>;
  cameraId?: ShootingAssigned<string | null>;
}

export interface ShootingSavedCameraPatch {
  label?: string;
  camera?: ShootingCamera;
}

export interface ShootingScenePatch {
  background?: string;
  sunEnabled?: boolean;
  sunAzimuth?: number;
  sunElevation?: number;
  sunIntensity?: number;
  sunColor?: string;
  ambientIntensity?: number;
  ambientColor?: string;
  shadowEnabled?: boolean;
  shadowOpacity?: number;
  shadowSoftness?: number;
  materialColor?: string;
  materialMetalness?: number;
  materialRoughness?: number;
  materialEmissive?: string;
  materialEmissiveIntensity?: number;
  materialStroke?: string;
}

export interface ShootingArtifact {
  schema: string;
  assets: ShootingAsset[];
  savedCameras: ShootingSavedCamera[];
  scene: ShootingSceneLighting;
  shots: ShootingShot[];
  activeShotId: string;
  activeAssetId: string;
  emblem?: ArtifactChild;
}

export interface ShootingCamera {
  position: [number, number, number];
  target: [number, number, number];
  zoom: number;
  fov: number;
  up?: [number, number, number];
  projection?: string;
}

export interface ShootingSavedCamera {
  id: string;
  label: string;
}

export interface ShootingAsset {
  id: string;
  name: string;
  url: string;
  format: string;
  origin: [number, number, number];
  orientation?: [number, number, number, number];
  scale?: [number, number, number];
}

export interface ShootingShot {
  id: string;
  label: string;
  width: number;
  height: number;
  format: string;
  shape: string;
  background?: string;
  cameraId?: string;
}

export interface ShootingSun {
  enabled: boolean;
  azimuth: number;
  elevation: number;
  intensity: number;
  color: string;
}

export interface ShootingAmbient {
  intensity: number;
  color: string;
}

export interface ShootingShadow {
  enabled: boolean;
  opacity: number;
  softness: number;
}

export interface ShootingMaterial {
  color: string;
  metalness: number;
  roughness: number;
  emissive: string;
  emissiveIntensity: number;
  stroke: string;
}

export interface ShootingSceneLighting {
  background: string;
  sun: ShootingSun;
  ambient: ShootingAmbient;
  shadow: ShootingShadow;
  material: ShootingMaterial;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class shootingShootingDiffGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const shootingShootingDiffGuardReject = (at: string, why: string): never => {
  throw new shootingShootingDiffGuardRefusal(at, why);
};

type shootingShootingDiffGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type shootingShootingDiffGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type shootingShootingDiffGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const shootingShootingDiffGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : shootingShootingDiffGuardReject(at, "value is not an object");
export const shootingShootingDiffGuardArray = (value: unknown, at: string, bounds: shootingShootingDiffGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return shootingShootingDiffGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) shootingShootingDiffGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) shootingShootingDiffGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const shootingShootingDiffGuardString = (value: unknown, at: string, bounds: shootingShootingDiffGuardTextBounds = {}): string => {
  if (typeof value !== "string") return shootingShootingDiffGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) shootingShootingDiffGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) shootingShootingDiffGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) shootingShootingDiffGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const shootingShootingDiffGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : shootingShootingDiffGuardReject(at, "value is not a boolean"));
export const shootingShootingDiffGuardNumber = (value: unknown, at: string, bounds: shootingShootingDiffGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return shootingShootingDiffGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) shootingShootingDiffGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) shootingShootingDiffGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const shootingShootingDiffGuardInteger = (value: unknown, at: string, bounds: shootingShootingDiffGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? shootingShootingDiffGuardNumber(value, at, bounds) : shootingShootingDiffGuardReject(at, "value is not an integer");
export const shootingShootingDiffGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : shootingShootingDiffGuardReject(at, `value is not one of ${members.join(", ")}`);
export const shootingShootingDiffGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : shootingShootingDiffGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseShootingAssigned<T>(value: unknown, at: string, parseValue: (item: unknown, where: string) => T): ShootingAssigned<T> {
  const row = shootingShootingDiffGuardObject(value, at);
  return { value: parseValue(row["value"], `${at}.value`) };
}

export function parseShootingEdit<T>(value: unknown, at: string, parseItem: (item: unknown, where: string) => T): ShootingEdit<T> {
  const row = shootingShootingDiffGuardObject(value, at);
  const edit = shootingShootingDiffGuardMember(row["edit"], `${at}.edit`, ["add", "remove", "move"] as const);
  if (edit === "add") return { edit, index: shootingShootingDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}), item: parseItem(row["item"], `${at}.item`) };
  if (edit === "remove") return { edit, id: shootingShootingDiffGuardString(row["id"], `${at}.id`) };
  return { edit, id: shootingShootingDiffGuardString(row["id"], `${at}.id`), index: shootingShootingDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}) };
}

export function parseShootingAssetPatch(value: unknown, at = "$"): ShootingAssetPatch {
  const row = shootingShootingDiffGuardObject(value, at);
  return {
    name: row["name"] === undefined ? undefined : shootingShootingDiffGuardString(row["name"], `${at}.name`),
    url: row["url"] === undefined ? undefined : shootingShootingDiffGuardString(row["url"], `${at}.url`),
    format: row["format"] === undefined ? undefined : shootingShootingDiffGuardString(row["format"], `${at}.format`),
    origin: row["origin"] === undefined ? undefined : shootingShootingDiffGuardArray(row["origin"], `${at}.origin`, {"minItems": 3, "maxItems": 3}).map((item, index) => shootingShootingDiffGuardNumber(item, `${at}.origin[${index}]`)),
    orientation: row["orientation"] === undefined ? undefined : parseShootingAssigned(row["orientation"], `${at}.orientation`, (item, where) => item === null ? null : shootingShootingDiffGuardArray(item, where, {"minItems": 4, "maxItems": 4}).map((member, index) => shootingShootingDiffGuardNumber(member, `${where}[${index}]`)) as [number, number, number, number]),
    scale: row["scale"] === undefined ? undefined : parseShootingAssigned(row["scale"], `${at}.scale`, (item, where) => item === null ? null : shootingShootingDiffGuardArray(item, where, {"minItems": 3, "maxItems": 3}).map((member, index) => shootingShootingDiffGuardNumber(member, `${where}[${index}]`)) as [number, number, number]),
  };
}

export function parseShootingShotPatch(value: unknown, at = "$"): ShootingShotPatch {
  const row = shootingShootingDiffGuardObject(value, at);
  return {
    label: row["label"] === undefined ? undefined : shootingShootingDiffGuardString(row["label"], `${at}.label`),
    width: row["width"] === undefined ? undefined : shootingShootingDiffGuardInteger(row["width"], `${at}.width`, {"minimum": 0}),
    height: row["height"] === undefined ? undefined : shootingShootingDiffGuardInteger(row["height"], `${at}.height`, {"minimum": 0}),
    format: row["format"] === undefined ? undefined : shootingShootingDiffGuardString(row["format"], `${at}.format`),
    shape: row["shape"] === undefined ? undefined : shootingShootingDiffGuardString(row["shape"], `${at}.shape`),
    background: row["background"] === undefined ? undefined : parseShootingAssigned(row["background"], `${at}.background`, (item, where) => item === null ? null : shootingShootingDiffGuardString(item, where)),
    cameraId: row["cameraId"] === undefined ? undefined : parseShootingAssigned(row["cameraId"], `${at}.cameraId`, (item, where) => item === null ? null : shootingShootingDiffGuardString(item, where)),
  };
}

export function parseShootingSavedCameraPatch(value: unknown, at = "$"): ShootingSavedCameraPatch {
  const row = shootingShootingDiffGuardObject(value, at);
  return {
    label: row["label"] === undefined ? undefined : shootingShootingDiffGuardString(row["label"], `${at}.label`),
    camera: row["camera"] === undefined ? undefined : parseShootingCamera(row["camera"], `${at}.camera`),
  };
}

export function parseShootingScenePatch(value: unknown, at = "$"): ShootingScenePatch {
  const row = shootingShootingDiffGuardObject(value, at);
  return {
    background: row["background"] === undefined ? undefined : shootingShootingDiffGuardString(row["background"], `${at}.background`),
    sunEnabled: row["sunEnabled"] === undefined ? undefined : shootingShootingDiffGuardBoolean(row["sunEnabled"], `${at}.sunEnabled`),
    sunAzimuth: row["sunAzimuth"] === undefined ? undefined : shootingShootingDiffGuardNumber(row["sunAzimuth"], `${at}.sunAzimuth`),
    sunElevation: row["sunElevation"] === undefined ? undefined : shootingShootingDiffGuardNumber(row["sunElevation"], `${at}.sunElevation`),
    sunIntensity: row["sunIntensity"] === undefined ? undefined : shootingShootingDiffGuardNumber(row["sunIntensity"], `${at}.sunIntensity`),
    sunColor: row["sunColor"] === undefined ? undefined : shootingShootingDiffGuardString(row["sunColor"], `${at}.sunColor`),
    ambientIntensity: row["ambientIntensity"] === undefined ? undefined : shootingShootingDiffGuardNumber(row["ambientIntensity"], `${at}.ambientIntensity`),
    ambientColor: row["ambientColor"] === undefined ? undefined : shootingShootingDiffGuardString(row["ambientColor"], `${at}.ambientColor`),
    shadowEnabled: row["shadowEnabled"] === undefined ? undefined : shootingShootingDiffGuardBoolean(row["shadowEnabled"], `${at}.shadowEnabled`),
    shadowOpacity: row["shadowOpacity"] === undefined ? undefined : shootingShootingDiffGuardNumber(row["shadowOpacity"], `${at}.shadowOpacity`),
    shadowSoftness: row["shadowSoftness"] === undefined ? undefined : shootingShootingDiffGuardNumber(row["shadowSoftness"], `${at}.shadowSoftness`),
    materialColor: row["materialColor"] === undefined ? undefined : shootingShootingDiffGuardString(row["materialColor"], `${at}.materialColor`),
    materialMetalness: row["materialMetalness"] === undefined ? undefined : shootingShootingDiffGuardNumber(row["materialMetalness"], `${at}.materialMetalness`),
    materialRoughness: row["materialRoughness"] === undefined ? undefined : shootingShootingDiffGuardNumber(row["materialRoughness"], `${at}.materialRoughness`),
    materialEmissive: row["materialEmissive"] === undefined ? undefined : shootingShootingDiffGuardString(row["materialEmissive"], `${at}.materialEmissive`),
    materialEmissiveIntensity: row["materialEmissiveIntensity"] === undefined ? undefined : shootingShootingDiffGuardNumber(row["materialEmissiveIntensity"], `${at}.materialEmissiveIntensity`),
    materialStroke: row["materialStroke"] === undefined ? undefined : shootingShootingDiffGuardString(row["materialStroke"], `${at}.materialStroke`),
  };
}

export function parseShootingAssetPatchEntry(value: unknown, at = "$"): ShootingAssetPatchEntry {
  const row = shootingShootingDiffGuardObject(value, at);
  return {
    id: shootingShootingDiffGuardString(row["id"], `${at}.id`),
    patch: parseShootingAssetPatch(row["patch"], `${at}.patch`),
  };
}

export function parseShootingShotPatchEntry(value: unknown, at = "$"): ShootingShotPatchEntry {
  const row = shootingShootingDiffGuardObject(value, at);
  return {
    id: shootingShootingDiffGuardString(row["id"], `${at}.id`),
    patch: parseShootingShotPatch(row["patch"], `${at}.patch`),
  };
}

export function parseShootingSavedCameraPatchEntry(value: unknown, at = "$"): ShootingSavedCameraPatchEntry {
  const row = shootingShootingDiffGuardObject(value, at);
  return {
    id: shootingShootingDiffGuardString(row["id"], `${at}.id`),
    patch: parseShootingSavedCameraPatch(row["patch"], `${at}.patch`),
  };
}
