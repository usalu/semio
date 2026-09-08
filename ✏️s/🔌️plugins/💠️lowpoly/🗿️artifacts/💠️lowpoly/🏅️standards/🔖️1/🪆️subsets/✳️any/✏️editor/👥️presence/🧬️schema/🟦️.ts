/** 🧬️ LowpolyPresence */
export interface LowpolyPresence {
  /** @state presence */
  selectionMode: string;
  /** @state presence */
  selectionIds: number[];
  /** @state presence */
  selectionTargetsMesh: boolean;
  /** @state presence */
  selectionTargetsVertex: boolean;
  /** @state presence */
  selectionTargetsEdge: boolean;
  /** @state presence */
  selectionTargetsFace: boolean;
  /** @state presence */
  selectedObjectIds: string[];
  /** @state presence */
  hoveredObjectId?: string;
  /** @state presence */
  hoveredTargetObjectId?: string;
  /** @state presence */
  hoveredTargetMode?: string;
  /** @state presence */
  hoveredTargetId?: number;
  /** @state presence */
  worldCameraPosition: number[];
  /** @state presence */
  worldCameraTarget: number[];
  /** @state presence */
  worldCameraFov: number;
  /** @state presence */
  activeUtilityId: string;
  /** @state presence */
  paintUtility: string;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class lowpolyLowpolyPresenceGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const lowpolyLowpolyPresenceGuardReject = (at: string, why: string): never => {
  throw new lowpolyLowpolyPresenceGuardRefusal(at, why);
};

type lowpolyLowpolyPresenceGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type lowpolyLowpolyPresenceGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type lowpolyLowpolyPresenceGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const lowpolyLowpolyPresenceGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : lowpolyLowpolyPresenceGuardReject(at, "value is not an object");
export const lowpolyLowpolyPresenceGuardArray = (value: unknown, at: string, bounds: lowpolyLowpolyPresenceGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return lowpolyLowpolyPresenceGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) lowpolyLowpolyPresenceGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) lowpolyLowpolyPresenceGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const lowpolyLowpolyPresenceGuardString = (value: unknown, at: string, bounds: lowpolyLowpolyPresenceGuardTextBounds = {}): string => {
  if (typeof value !== "string") return lowpolyLowpolyPresenceGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) lowpolyLowpolyPresenceGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) lowpolyLowpolyPresenceGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) lowpolyLowpolyPresenceGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const lowpolyLowpolyPresenceGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : lowpolyLowpolyPresenceGuardReject(at, "value is not a boolean"));
export const lowpolyLowpolyPresenceGuardNumber = (value: unknown, at: string, bounds: lowpolyLowpolyPresenceGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return lowpolyLowpolyPresenceGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) lowpolyLowpolyPresenceGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) lowpolyLowpolyPresenceGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const lowpolyLowpolyPresenceGuardInteger = (value: unknown, at: string, bounds: lowpolyLowpolyPresenceGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? lowpolyLowpolyPresenceGuardNumber(value, at, bounds) : lowpolyLowpolyPresenceGuardReject(at, "value is not an integer");
export const lowpolyLowpolyPresenceGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : lowpolyLowpolyPresenceGuardReject(at, `value is not one of ${members.join(", ")}`);
export const lowpolyLowpolyPresenceGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : lowpolyLowpolyPresenceGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseLowpolyPresence(value: unknown, at = "$"): LowpolyPresence {
  const row = lowpolyLowpolyPresenceGuardObject(value, at);
  return {
    selectionMode: lowpolyLowpolyPresenceGuardString(row["selectionMode"], `${at}.selectionMode`),
    selectionIds: lowpolyLowpolyPresenceGuardArray(row["selectionIds"], `${at}.selectionIds`).map((item, index) => lowpolyLowpolyPresenceGuardInteger(item, `${at}.selectionIds[${index}]`, {"minimum": 0})),
    selectionTargetsMesh: lowpolyLowpolyPresenceGuardBoolean(row["selectionTargetsMesh"], `${at}.selectionTargetsMesh`),
    selectionTargetsVertex: lowpolyLowpolyPresenceGuardBoolean(row["selectionTargetsVertex"], `${at}.selectionTargetsVertex`),
    selectionTargetsEdge: lowpolyLowpolyPresenceGuardBoolean(row["selectionTargetsEdge"], `${at}.selectionTargetsEdge`),
    selectionTargetsFace: lowpolyLowpolyPresenceGuardBoolean(row["selectionTargetsFace"], `${at}.selectionTargetsFace`),
    selectedObjectIds: lowpolyLowpolyPresenceGuardArray(row["selectedObjectIds"], `${at}.selectedObjectIds`).map((item, index) => lowpolyLowpolyPresenceGuardString(item, `${at}.selectedObjectIds[${index}]`)),
    hoveredObjectId: row["hoveredObjectId"] === undefined ? undefined : lowpolyLowpolyPresenceGuardString(row["hoveredObjectId"], `${at}.hoveredObjectId`),
    hoveredTargetObjectId: row["hoveredTargetObjectId"] === undefined ? undefined : lowpolyLowpolyPresenceGuardString(row["hoveredTargetObjectId"], `${at}.hoveredTargetObjectId`),
    hoveredTargetMode: row["hoveredTargetMode"] === undefined ? undefined : lowpolyLowpolyPresenceGuardString(row["hoveredTargetMode"], `${at}.hoveredTargetMode`),
    hoveredTargetId: row["hoveredTargetId"] === undefined ? undefined : lowpolyLowpolyPresenceGuardInteger(row["hoveredTargetId"], `${at}.hoveredTargetId`, {"minimum": 0}),
    worldCameraPosition: lowpolyLowpolyPresenceGuardArray(row["worldCameraPosition"], `${at}.worldCameraPosition`, {"minItems": 3, "maxItems": 3}).map((item, index) => lowpolyLowpolyPresenceGuardNumber(item, `${at}.worldCameraPosition[${index}]`)),
    worldCameraTarget: lowpolyLowpolyPresenceGuardArray(row["worldCameraTarget"], `${at}.worldCameraTarget`, {"minItems": 3, "maxItems": 3}).map((item, index) => lowpolyLowpolyPresenceGuardNumber(item, `${at}.worldCameraTarget[${index}]`)),
    worldCameraFov: lowpolyLowpolyPresenceGuardNumber(row["worldCameraFov"], `${at}.worldCameraFov`),
    activeUtilityId: lowpolyLowpolyPresenceGuardString(row["activeUtilityId"], `${at}.activeUtilityId`),
    paintUtility: lowpolyLowpolyPresenceGuardString(row["paintUtility"], `${at}.paintUtility`),
  };
}
