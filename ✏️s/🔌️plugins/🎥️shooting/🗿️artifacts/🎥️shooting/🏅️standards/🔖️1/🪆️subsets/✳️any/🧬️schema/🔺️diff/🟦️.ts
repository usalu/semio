/** 🧬️ Shooting diff schema — sparse field delta over the artifact. */

export interface ShootingDiff {
  /** @state artifact */
  artifact?: ShootingArtifact;
  /** @state artifact */
  schema?: string;
  /** @state artifact */
  assets?: ShootingAssetsDelta;
  /** @state artifact */
  savedCameras?: ShootingSavedCamerasDelta;
  /** @state artifact */
  scene?: ShootingSceneLighting;
  /** @state artifact */
  shots?: ShootingShotsDelta;
  /** @state artifact */
  activeShotId?: string;
  /** @state artifact */
  activeAssetId?: string;
  /** @state presence */
  selectedShotIds?: ShootingStringList;
  /** @state presence */
  activeUtilityId?: string;
  /** @state config */
  defaultShotFormat?: string;
  /** @state config */
  defaultShotShape?: string;
  /** @state config */
  defaultAssetFormat?: string;
  /** @state config */
  centerModel?: boolean;
  /** @state config */
  fitRevision?: number;
  /** @state config */
  cameraDraftLabel?: string;
  /** @state config */
  camera?: ShootingCamera;
  /** @state config */
}

export interface ShootingStringList {
  values: string[];
}

export interface ShootingAssetsDelta {
  added: ShootingAsset[];
  removed: string[];
  patched: ShootingAssetPatchEntry[];
  reordered?: string[];
}

export interface ShootingShotsDelta {
  added: ShootingShot[];
  removed: string[];
  patched: ShootingShotPatchEntry[];
  reordered?: string[];
}

export interface ShootingSavedCamerasDelta {
  added: ShootingSavedCamera[];
  removed: string[];
  patched: ShootingSavedCameraPatchEntry[];
  reordered?: string[];
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
  origin?: [number, number, number];
  orientation?: [number, number, number, number];
  scale?: [number, number, number];
}

export interface ShootingShotPatch {
  label?: string;
  width?: number;
  height?: number;
  format?: string;
  shape?: string;
}

export interface ShootingSavedCameraPatch {
  label?: string;
  camera?: ShootingCamera;
}

export interface ShootingArtifact {
  schema: string;
  assets: ShootingAsset[];
  savedCameras: ShootingSavedCamera[];
  scene: ShootingSceneLighting;
  shots: ShootingShot[];
  activeShotId: string;
  activeAssetId: string;
  selectedShotIds: string[];
  activeUtilityId: string;
  defaultShotFormat: string;
  defaultShotShape: string;
  defaultAssetFormat: string;
  centerModel: boolean;
  fitRevision: number;
  cameraDraftLabel: string;
  camera: ShootingCamera;
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
  camera: ShootingCamera;
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

export function parseShootingStringList(value: unknown, at = "$"): ShootingStringList {
  const row = shootingShootingDiffGuardObject(value, at);
  return {
    values: shootingShootingDiffGuardArray(row["values"], `${at}.values`).map((item, index) => shootingShootingDiffGuardString(item, `${at}.values[${index}]`)),
  };
}

export function parseShootingAssetPatch(value: unknown, at = "$"): ShootingAssetPatch {
  const row = shootingShootingDiffGuardObject(value, at);
  return {
    name: row["name"] === undefined ? undefined : shootingShootingDiffGuardString(row["name"], `${at}.name`),
    url: row["url"] === undefined ? undefined : shootingShootingDiffGuardString(row["url"], `${at}.url`),
    origin: row["origin"] === undefined ? undefined : shootingShootingDiffGuardArray(row["origin"], `${at}.origin`, {"minItems": 3, "maxItems": 3}).map((item, index) => shootingShootingDiffGuardNumber(item, `${at}.origin[${index}]`)),
    orientation: row["orientation"] === undefined ? undefined : shootingShootingDiffGuardArray(row["orientation"], `${at}.orientation`, {"minItems": 4, "maxItems": 4}).map((item, index) => shootingShootingDiffGuardNumber(item, `${at}.orientation[${index}]`)),
    scale: row["scale"] === undefined ? undefined : shootingShootingDiffGuardArray(row["scale"], `${at}.scale`, {"minItems": 3, "maxItems": 3}).map((item, index) => shootingShootingDiffGuardNumber(item, `${at}.scale[${index}]`)),
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
