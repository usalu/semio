/** 🧬️ Shooting artifact schema — every field with its state class. */

import { parseArtifactChild, type ArtifactChild } from "../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🪆️child/🧬️schema/🟦️.ts";

export interface ShootingArtifact {
  /** @state artifact */
  schema: string;
  /** @state artifact */
  assets: ShootingAsset[];
  /** @state artifact */
  savedCameras: ShootingSavedCamera[];
  /** @state artifact */
  scene: ShootingSceneLighting;
  /** @state artifact */
  shots: ShootingShot[];
  /** @state artifact */
  activeShotId: string;
  /** @state artifact */
  activeAssetId: string;
  /** @state artifact @child kind=s.stdio.semio */
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
}

export interface ShootingSceneLighting {
  background: string;
  sun: ShootingSun;
  ambient: ShootingAmbient;
  shadow: ShootingShadow;
  material: ShootingMaterial;
  emblemBase64?: string;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class shootingShootingArtifactGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const shootingShootingArtifactGuardReject = (at: string, why: string): never => {
  throw new shootingShootingArtifactGuardRefusal(at, why);
};

type shootingShootingArtifactGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type shootingShootingArtifactGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type shootingShootingArtifactGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const shootingShootingArtifactGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : shootingShootingArtifactGuardReject(at, "value is not an object");
export const shootingShootingArtifactGuardArray = (value: unknown, at: string, bounds: shootingShootingArtifactGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return shootingShootingArtifactGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) shootingShootingArtifactGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) shootingShootingArtifactGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const shootingShootingArtifactGuardString = (value: unknown, at: string, bounds: shootingShootingArtifactGuardTextBounds = {}): string => {
  if (typeof value !== "string") return shootingShootingArtifactGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) shootingShootingArtifactGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) shootingShootingArtifactGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) shootingShootingArtifactGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const shootingShootingArtifactGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : shootingShootingArtifactGuardReject(at, "value is not a boolean"));
export const shootingShootingArtifactGuardNumber = (value: unknown, at: string, bounds: shootingShootingArtifactGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return shootingShootingArtifactGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) shootingShootingArtifactGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) shootingShootingArtifactGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const shootingShootingArtifactGuardInteger = (value: unknown, at: string, bounds: shootingShootingArtifactGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? shootingShootingArtifactGuardNumber(value, at, bounds) : shootingShootingArtifactGuardReject(at, "value is not an integer");
export const shootingShootingArtifactGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : shootingShootingArtifactGuardReject(at, `value is not one of ${members.join(", ")}`);
export const shootingShootingArtifactGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : shootingShootingArtifactGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseShootingArtifact(value: unknown, at = "$"): ShootingArtifact {
  const row = shootingShootingArtifactGuardObject(value, at);
  return {
    schema: shootingShootingArtifactGuardString(row["schema"], `${at}.schema`),
    assets: shootingShootingArtifactGuardArray(row["assets"], `${at}.assets`).map((item, index) => parseShootingAsset(item, `${at}.assets[${index}]`)),
    savedCameras: shootingShootingArtifactGuardArray(row["savedCameras"], `${at}.savedCameras`).map((item, index) => parseShootingSavedCamera(item, `${at}.savedCameras[${index}]`)),
    scene: parseShootingSceneLighting(row["scene"], `${at}.scene`),
    shots: shootingShootingArtifactGuardArray(row["shots"], `${at}.shots`).map((item, index) => parseShootingShot(item, `${at}.shots[${index}]`)),
    activeShotId: shootingShootingArtifactGuardString(row["activeShotId"], `${at}.activeShotId`),
    activeAssetId: shootingShootingArtifactGuardString(row["activeAssetId"], `${at}.activeAssetId`),
    emblem: row["emblem"] === undefined ? undefined : parseArtifactChild(row["emblem"], `${at}.emblem`),
  };
}

export function parseShootingCamera(value: unknown, at = "$"): ShootingCamera {
  const row = shootingShootingArtifactGuardObject(value, at);
  return {
    position: shootingShootingArtifactGuardArray(row["position"], `${at}.position`, {"minItems": 3, "maxItems": 3}).map((item, index) => shootingShootingArtifactGuardNumber(item, `${at}.position[${index}]`)),
    target: shootingShootingArtifactGuardArray(row["target"], `${at}.target`, {"minItems": 3, "maxItems": 3}).map((item, index) => shootingShootingArtifactGuardNumber(item, `${at}.target[${index}]`)),
    zoom: shootingShootingArtifactGuardNumber(row["zoom"], `${at}.zoom`),
    fov: shootingShootingArtifactGuardNumber(row["fov"], `${at}.fov`),
    up: row["up"] === undefined ? undefined : shootingShootingArtifactGuardArray(row["up"], `${at}.up`, {"minItems": 3, "maxItems": 3}).map((item, index) => shootingShootingArtifactGuardNumber(item, `${at}.up[${index}]`)),
    projection: row["projection"] === undefined ? undefined : shootingShootingArtifactGuardString(row["projection"], `${at}.projection`),
  };
}

export function parseShootingSavedCamera(value: unknown, at = "$"): ShootingSavedCamera {
  const row = shootingShootingArtifactGuardObject(value, at);
  return {
    id: shootingShootingArtifactGuardString(row["id"], `${at}.id`),
    label: shootingShootingArtifactGuardString(row["label"], `${at}.label`),
  };
}

export function parseShootingAsset(value: unknown, at = "$"): ShootingAsset {
  const row = shootingShootingArtifactGuardObject(value, at);
  return {
    id: shootingShootingArtifactGuardString(row["id"], `${at}.id`),
    name: shootingShootingArtifactGuardString(row["name"], `${at}.name`),
    url: shootingShootingArtifactGuardString(row["url"], `${at}.url`),
    format: shootingShootingArtifactGuardString(row["format"], `${at}.format`),
    origin: shootingShootingArtifactGuardArray(row["origin"], `${at}.origin`, {"minItems": 3, "maxItems": 3}).map((item, index) => shootingShootingArtifactGuardNumber(item, `${at}.origin[${index}]`)),
    orientation: row["orientation"] === undefined ? undefined : shootingShootingArtifactGuardArray(row["orientation"], `${at}.orientation`, {"minItems": 4, "maxItems": 4}).map((item, index) => shootingShootingArtifactGuardNumber(item, `${at}.orientation[${index}]`)),
    scale: row["scale"] === undefined ? undefined : shootingShootingArtifactGuardArray(row["scale"], `${at}.scale`, {"minItems": 3, "maxItems": 3}).map((item, index) => shootingShootingArtifactGuardNumber(item, `${at}.scale[${index}]`)),
  };
}

export function parseShootingShot(value: unknown, at = "$"): ShootingShot {
  const row = shootingShootingArtifactGuardObject(value, at);
  return {
    id: shootingShootingArtifactGuardString(row["id"], `${at}.id`),
    label: shootingShootingArtifactGuardString(row["label"], `${at}.label`),
    width: shootingShootingArtifactGuardInteger(row["width"], `${at}.width`, {"minimum": 0}),
    height: shootingShootingArtifactGuardInteger(row["height"], `${at}.height`, {"minimum": 0}),
    format: shootingShootingArtifactGuardString(row["format"], `${at}.format`),
    shape: shootingShootingArtifactGuardString(row["shape"], `${at}.shape`),
    background: row["background"] === undefined ? undefined : shootingShootingArtifactGuardString(row["background"], `${at}.background`),
    cameraId: row["cameraId"] === undefined ? undefined : shootingShootingArtifactGuardString(row["cameraId"], `${at}.cameraId`),
  };
}

export function parseShootingSun(value: unknown, at = "$"): ShootingSun {
  const row = shootingShootingArtifactGuardObject(value, at);
  return {
    enabled: shootingShootingArtifactGuardBoolean(row["enabled"], `${at}.enabled`),
    azimuth: shootingShootingArtifactGuardNumber(row["azimuth"], `${at}.azimuth`),
    elevation: shootingShootingArtifactGuardNumber(row["elevation"], `${at}.elevation`),
    intensity: shootingShootingArtifactGuardNumber(row["intensity"], `${at}.intensity`),
    color: shootingShootingArtifactGuardString(row["color"], `${at}.color`),
  };
}

export function parseShootingAmbient(value: unknown, at = "$"): ShootingAmbient {
  const row = shootingShootingArtifactGuardObject(value, at);
  return {
    intensity: shootingShootingArtifactGuardNumber(row["intensity"], `${at}.intensity`),
    color: shootingShootingArtifactGuardString(row["color"], `${at}.color`),
  };
}

export function parseShootingShadow(value: unknown, at = "$"): ShootingShadow {
  const row = shootingShootingArtifactGuardObject(value, at);
  return {
    enabled: shootingShootingArtifactGuardBoolean(row["enabled"], `${at}.enabled`),
    opacity: shootingShootingArtifactGuardNumber(row["opacity"], `${at}.opacity`),
    softness: shootingShootingArtifactGuardNumber(row["softness"], `${at}.softness`),
  };
}

export function parseShootingMaterial(value: unknown, at = "$"): ShootingMaterial {
  const row = shootingShootingArtifactGuardObject(value, at);
  return {
    color: shootingShootingArtifactGuardString(row["color"], `${at}.color`),
    metalness: shootingShootingArtifactGuardNumber(row["metalness"], `${at}.metalness`),
    roughness: shootingShootingArtifactGuardNumber(row["roughness"], `${at}.roughness`),
    emissive: shootingShootingArtifactGuardString(row["emissive"], `${at}.emissive`),
    emissiveIntensity: shootingShootingArtifactGuardNumber(row["emissiveIntensity"], `${at}.emissiveIntensity`),
  };
}

export function parseShootingSceneLighting(value: unknown, at = "$"): ShootingSceneLighting {
  const row = shootingShootingArtifactGuardObject(value, at);
  return {
    background: shootingShootingArtifactGuardString(row["background"], `${at}.background`),
    sun: parseShootingSun(row["sun"], `${at}.sun`),
    ambient: parseShootingAmbient(row["ambient"], `${at}.ambient`),
    shadow: parseShootingShadow(row["shadow"], `${at}.shadow`),
    material: parseShootingMaterial(row["material"], `${at}.material`),
    emblemBase64: row["emblemBase64"] === undefined ? undefined : shootingShootingArtifactGuardString(row["emblemBase64"], `${at}.emblemBase64`),
  };
}

export interface ShootingRetainedCommandStepContract {
  readonly maxRawBytes: number;
  readonly maxDecodedItems: number;
  readonly maxWorkUnitsPerStep: number;
  readonly maxOutputBytes: number;
  readonly maxStepMicros: number;
  readonly maxCheckpointBytes?: number;
  readonly maxInFlightPages?: number;
}

export function parseShootingRetainedCommandStepContract(value: unknown, at = "$"): ShootingRetainedCommandStepContract {
  const row = shootingShootingArtifactGuardObject(value, at);
  return {
    maxRawBytes: shootingShootingArtifactGuardInteger(row["maxRawBytes"], `${at}.maxRawBytes`, {"minimum": 1}),
    maxDecodedItems: shootingShootingArtifactGuardInteger(row["maxDecodedItems"], `${at}.maxDecodedItems`, {"minimum": 1}),
    maxWorkUnitsPerStep: shootingShootingArtifactGuardInteger(row["maxWorkUnitsPerStep"], `${at}.maxWorkUnitsPerStep`, {"minimum": 1}),
    maxOutputBytes: shootingShootingArtifactGuardInteger(row["maxOutputBytes"], `${at}.maxOutputBytes`, {"minimum": 1}),
    maxStepMicros: shootingShootingArtifactGuardInteger(row["maxStepMicros"], `${at}.maxStepMicros`, {"minimum": 1, "maximum": 7999}),
    maxCheckpointBytes: row["maxCheckpointBytes"] === undefined ? undefined : shootingShootingArtifactGuardInteger(row["maxCheckpointBytes"], `${at}.maxCheckpointBytes`, {"minimum": 1}),
    maxInFlightPages: row["maxInFlightPages"] === undefined ? undefined : shootingShootingArtifactGuardInteger(row["maxInFlightPages"], `${at}.maxInFlightPages`, {"minimum": 1}),
  };
}
