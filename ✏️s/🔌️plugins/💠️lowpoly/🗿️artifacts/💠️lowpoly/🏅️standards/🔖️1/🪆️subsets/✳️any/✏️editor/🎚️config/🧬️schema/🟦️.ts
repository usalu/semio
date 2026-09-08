/** 🧬️ LowpolyConfig */
export interface LowpolyConfig {
  /** @state config */
  activeObjectId: string;
  /** @state config */
  selectionMode: string;
  /** @state config */
  selectionIds: number[];
  /** @state config */
  selectionTargetsMesh: boolean;
  /** @state config */
  selectionTargetsVertex: boolean;
  /** @state config */
  selectionTargetsEdge: boolean;
  /** @state config */
  selectionTargetsFace: boolean;
  /** @state config */
  selectionKeys: string[];
  /** @state config */
  paintUtility: string;
  /** @state config */
  activePaintLayer: number;
  /** @state config */
  selectionMethod: string;
  /** @state config */
  selectionModeDefault: string;
  /** @state config */
  selectedObjectIds: string[];
  /** @state config */
  hoveredObjectId?: string;
  /** @state config */
  hoveredTargetObjectId?: string;
  /** @state config */
  hoveredTargetMode?: string;
  /** @state config */
  hoveredTargetId?: number;
  /** @state config */
  utilityParamsJson: string;
  /** @state config */
  paintColorR: number;
  /** @state config */
  paintColorG: number;
  /** @state config */
  paintColorB: number;
  /** @state config */
  paintColorA: number;
  /** @state config */
  worldCameraPosition: number[];
  /** @state config */
  worldCameraTarget: number[];
  /** @state config */
  worldCameraFov: number;
  /** @state config */
  engagementInput: string;
  /** @state config */
  showEdges: boolean;
  /** @state config */
  sunEnabled: boolean;
  /** @state config */
  sunAzimuth: number;
  /** @state config */
  sunElevation: number;
  /** @state config */
  sunIntensity: number;
  /** @state config */
  sunColor: string;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class lowpolyLowpolyConfigGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const lowpolyLowpolyConfigGuardReject = (at: string, why: string): never => {
  throw new lowpolyLowpolyConfigGuardRefusal(at, why);
};

type lowpolyLowpolyConfigGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type lowpolyLowpolyConfigGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type lowpolyLowpolyConfigGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const lowpolyLowpolyConfigGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : lowpolyLowpolyConfigGuardReject(at, "value is not an object");
export const lowpolyLowpolyConfigGuardArray = (value: unknown, at: string, bounds: lowpolyLowpolyConfigGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return lowpolyLowpolyConfigGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) lowpolyLowpolyConfigGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) lowpolyLowpolyConfigGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const lowpolyLowpolyConfigGuardString = (value: unknown, at: string, bounds: lowpolyLowpolyConfigGuardTextBounds = {}): string => {
  if (typeof value !== "string") return lowpolyLowpolyConfigGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) lowpolyLowpolyConfigGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) lowpolyLowpolyConfigGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) lowpolyLowpolyConfigGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const lowpolyLowpolyConfigGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : lowpolyLowpolyConfigGuardReject(at, "value is not a boolean"));
export const lowpolyLowpolyConfigGuardNumber = (value: unknown, at: string, bounds: lowpolyLowpolyConfigGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return lowpolyLowpolyConfigGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) lowpolyLowpolyConfigGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) lowpolyLowpolyConfigGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const lowpolyLowpolyConfigGuardInteger = (value: unknown, at: string, bounds: lowpolyLowpolyConfigGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? lowpolyLowpolyConfigGuardNumber(value, at, bounds) : lowpolyLowpolyConfigGuardReject(at, "value is not an integer");
export const lowpolyLowpolyConfigGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : lowpolyLowpolyConfigGuardReject(at, `value is not one of ${members.join(", ")}`);
export const lowpolyLowpolyConfigGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : lowpolyLowpolyConfigGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseLowpolyConfig(value: unknown, at = "$"): LowpolyConfig {
  const row = lowpolyLowpolyConfigGuardObject(value, at);
  return {
    activeObjectId: lowpolyLowpolyConfigGuardString(row["activeObjectId"], `${at}.activeObjectId`),
    selectionMode: lowpolyLowpolyConfigGuardString(row["selectionMode"], `${at}.selectionMode`),
    selectionIds: lowpolyLowpolyConfigGuardArray(row["selectionIds"], `${at}.selectionIds`).map((item, index) => lowpolyLowpolyConfigGuardInteger(item, `${at}.selectionIds[${index}]`, {"minimum": 0})),
    selectionTargetsMesh: lowpolyLowpolyConfigGuardBoolean(row["selectionTargetsMesh"], `${at}.selectionTargetsMesh`),
    selectionTargetsVertex: lowpolyLowpolyConfigGuardBoolean(row["selectionTargetsVertex"], `${at}.selectionTargetsVertex`),
    selectionTargetsEdge: lowpolyLowpolyConfigGuardBoolean(row["selectionTargetsEdge"], `${at}.selectionTargetsEdge`),
    selectionTargetsFace: lowpolyLowpolyConfigGuardBoolean(row["selectionTargetsFace"], `${at}.selectionTargetsFace`),
    selectionKeys: lowpolyLowpolyConfigGuardArray(row["selectionKeys"], `${at}.selectionKeys`).map((item, index) => lowpolyLowpolyConfigGuardString(item, `${at}.selectionKeys[${index}]`)),
    paintUtility: lowpolyLowpolyConfigGuardString(row["paintUtility"], `${at}.paintUtility`),
    activePaintLayer: lowpolyLowpolyConfigGuardInteger(row["activePaintLayer"], `${at}.activePaintLayer`, {"minimum": 0}),
    selectionMethod: lowpolyLowpolyConfigGuardString(row["selectionMethod"], `${at}.selectionMethod`),
    selectionModeDefault: lowpolyLowpolyConfigGuardString(row["selectionModeDefault"], `${at}.selectionModeDefault`),
    selectedObjectIds: lowpolyLowpolyConfigGuardArray(row["selectedObjectIds"], `${at}.selectedObjectIds`).map((item, index) => lowpolyLowpolyConfigGuardString(item, `${at}.selectedObjectIds[${index}]`)),
    hoveredObjectId: row["hoveredObjectId"] === undefined ? undefined : lowpolyLowpolyConfigGuardString(row["hoveredObjectId"], `${at}.hoveredObjectId`),
    hoveredTargetObjectId: row["hoveredTargetObjectId"] === undefined ? undefined : lowpolyLowpolyConfigGuardString(row["hoveredTargetObjectId"], `${at}.hoveredTargetObjectId`),
    hoveredTargetMode: row["hoveredTargetMode"] === undefined ? undefined : lowpolyLowpolyConfigGuardString(row["hoveredTargetMode"], `${at}.hoveredTargetMode`),
    hoveredTargetId: row["hoveredTargetId"] === undefined ? undefined : lowpolyLowpolyConfigGuardInteger(row["hoveredTargetId"], `${at}.hoveredTargetId`, {"minimum": 0}),
    utilityParamsJson: lowpolyLowpolyConfigGuardString(row["utilityParamsJson"], `${at}.utilityParamsJson`),
    paintColorR: lowpolyLowpolyConfigGuardInteger(row["paintColorR"], `${at}.paintColorR`, {"minimum": 0, "maximum": 255}),
    paintColorG: lowpolyLowpolyConfigGuardInteger(row["paintColorG"], `${at}.paintColorG`, {"minimum": 0, "maximum": 255}),
    paintColorB: lowpolyLowpolyConfigGuardInteger(row["paintColorB"], `${at}.paintColorB`, {"minimum": 0, "maximum": 255}),
    paintColorA: lowpolyLowpolyConfigGuardInteger(row["paintColorA"], `${at}.paintColorA`, {"minimum": 0, "maximum": 255}),
    worldCameraPosition: lowpolyLowpolyConfigGuardArray(row["worldCameraPosition"], `${at}.worldCameraPosition`, {"minItems": 3, "maxItems": 3}).map((item, index) => lowpolyLowpolyConfigGuardNumber(item, `${at}.worldCameraPosition[${index}]`)),
    worldCameraTarget: lowpolyLowpolyConfigGuardArray(row["worldCameraTarget"], `${at}.worldCameraTarget`, {"minItems": 3, "maxItems": 3}).map((item, index) => lowpolyLowpolyConfigGuardNumber(item, `${at}.worldCameraTarget[${index}]`)),
    worldCameraFov: lowpolyLowpolyConfigGuardNumber(row["worldCameraFov"], `${at}.worldCameraFov`),
    engagementInput: lowpolyLowpolyConfigGuardString(row["engagementInput"], `${at}.engagementInput`),
    showEdges: lowpolyLowpolyConfigGuardBoolean(row["showEdges"], `${at}.showEdges`),
    sunEnabled: lowpolyLowpolyConfigGuardBoolean(row["sunEnabled"], `${at}.sunEnabled`),
    sunAzimuth: lowpolyLowpolyConfigGuardNumber(row["sunAzimuth"], `${at}.sunAzimuth`),
    sunElevation: lowpolyLowpolyConfigGuardNumber(row["sunElevation"], `${at}.sunElevation`),
    sunIntensity: lowpolyLowpolyConfigGuardNumber(row["sunIntensity"], `${at}.sunIntensity`),
    sunColor: lowpolyLowpolyConfigGuardString(row["sunColor"], `${at}.sunColor`),
  };
}
