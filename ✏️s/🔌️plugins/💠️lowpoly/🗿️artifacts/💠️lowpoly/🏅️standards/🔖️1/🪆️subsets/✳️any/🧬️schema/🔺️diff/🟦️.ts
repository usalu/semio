/** 🧬️ Lowpoly diff schema — sparse field delta. */

export interface LowpolyDiff {
  /** @state artifact */
  artifact?: LowpolyArtifact;
  /** @state artifact */
  schema?: string;
  /** @state artifact */
  objects?: LowpolyObjectsDelta;
  /** @state presence */
  activeObjectId?: string | null;
  /** @state presence */
  selection?: LowpolySelection;
  /** @state presence */
  selectedObjectIds?: LowpolyStringList;
  /** @state presence */
  paintUtility?: string;
  /** @state presence */
  activePaintLayer?: number;
  /** @state presence */
  activeUtilityId?: string;
  /** @state config */
  showEdges?: boolean;
  /** @state config */
  sunEnabled?: boolean;
  /** @state config */
  sunAzimuth?: number;
  /** @state config */
  sunElevation?: number;
  /** @state config */
  sunIntensity?: number;
  /** @state config */
  sunColor?: string;
  /** @state config */
  worldCameraPositionX?: number;
  /** @state config */
  worldCameraPositionY?: number;
  /** @state config */
  worldCameraPositionZ?: number;
  /** @state config */
  worldCameraTargetX?: number;
  /** @state config */
  worldCameraTargetY?: number;
  /** @state config */
  worldCameraTargetZ?: number;
  /** @state config */
  worldCameraFov?: number;
  /** @state config */
  utilityParamsJson?: string;
  /** @state config */
  paintColorR?: number;
  /** @state config */
  paintColorG?: number;
  /** @state config */
  paintColorB?: number;
  /** @state config */
  paintColorA?: number;
  /** @state config */
  selectionMethod?: string;
  /** @state config */
  selectionModeDefault?: string;
  /** @state config */
  engagementInput?: string;
  /** @state config */
  /** @state artifact */
  hoveredObjectId?: string | null;
  /** @state artifact */
  hoveredTargetObjectId?: string | null;
  /** @state artifact */
  hoveredTargetMode?: string | null;
  /** @state artifact */
  hoveredTargetId?: number | null;
  /** @state artifact */
  strokeDragActive?: boolean;
  /** @state artifact */
  transformDragActive?: boolean;
  /** @state artifact */
  previewSeq?: number;
}

export interface LowpolyStringList {
  values: string[];
}

export interface LowpolyObjectsDelta {
  added: LowpolyObject[];
  removed: string[];
  patched: LowpolyObjectPatchEntry[];
  reordered?: string[];
}

export interface LowpolyObjectPatchEntry {
  id: string;
  patch: LowpolyObjectPatch;
  paintLayers?: LowpolyPaintLayersDelta;
}

export interface LowpolyObjectPatch {
  name?: string;
  smoothShading?: boolean;
  transform?: LowpolyTransform;
  /** Double-optional on the wire: absent = untouched, `null` = cleared, present = new handle. */
  mesh?: LowpolyMeshHandle | null;
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
  name?: string;
  visible?: boolean;
  opacity?: number;
  blendMode?: string;
}

export interface LowpolyPaintStrokeAt {
  layerIndex: number;
  runs: PixelRun[];
}

export interface PixelRun {
  offset: number;
  bytes: string;
}

export interface LowpolyArtifact {
  schema: string;
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
  mesh: LowpolyMeshHandle | null;
  paintLayers: LowpolyPaintLayer[];
}

export interface LowpolyMeshHandle {
  childId: string;
  target: ArtifactRef;
}

export interface ArtifactDialect {
  artifactKind: string;
  standard: string;
  subset: string;
}

export interface ArtifactRef {
  artifactId: string;
  dialect: ArtifactDialect;
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

export function parseLowpolyStringList(value: unknown, at = "$"): LowpolyStringList {
  const row = lowpolyLowpolyDiffGuardObject(value, at);
  return {
    values: lowpolyLowpolyDiffGuardArray(row["values"], `${at}.values`).map((item, index) => lowpolyLowpolyDiffGuardString(item, `${at}.values[${index}]`)),
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

export function parseLowpolyIndexedPaintLayerPatch(value: unknown, at = "$"): LowpolyIndexedPaintLayerPatch {
  const row = lowpolyLowpolyDiffGuardObject(value, at);
  return {
    index: lowpolyLowpolyDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
    patch: parseLowpolyPaintLayerPatch(row["patch"], `${at}.patch`),
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
