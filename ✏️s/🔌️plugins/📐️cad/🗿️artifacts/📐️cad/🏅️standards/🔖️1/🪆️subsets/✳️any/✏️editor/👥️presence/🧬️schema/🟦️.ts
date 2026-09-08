/** 🧬️ CadPresence */

export interface CadPresence {
  /** @state presence */
  selectedObjectIds: string[];
  /** @state presence */
  selectedNodeIds: string[];
  /** @state presence */
  hoveredObjectId?: string;
  /** @state presence */
  hoveredTargetObjectId?: string;
  /** @state presence */
  hoveredTargetMode?: string;
  /** @state presence */
  hoveredTargetId?: number;
  /** @state presence */
  activeObjectId?: string;
  /** @state presence */
  componentSelectionMode: string;
  /** @state presence */
  componentSelectionIds: number[];
  /** @state presence */
  componentSelectionTargetsMesh: boolean;
  /** @state presence */
  componentSelectionTargetsVertex: boolean;
  /** @state presence */
  componentSelectionTargetsEdge: boolean;
  /** @state presence */
  componentSelectionTargetsFace: boolean;
  /** @state presence */
  cameraPosition: number[];
  /** @state presence */
  cameraTarget: number[];
  /** @state presence */
  cameraZoom: number;
  /** @state presence */
  cameraFov: number;
  /** @state presence */
  activeUtilityId: string;
  /** @state presence */
  engagementStep: string;
  /** @state presence */
  engagementPane?: string;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class cadCadPresenceGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const cadCadPresenceGuardReject = (at: string, why: string): never => {
  throw new cadCadPresenceGuardRefusal(at, why);
};

type cadCadPresenceGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type cadCadPresenceGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type cadCadPresenceGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const cadCadPresenceGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : cadCadPresenceGuardReject(at, "value is not an object");
export const cadCadPresenceGuardArray = (value: unknown, at: string, bounds: cadCadPresenceGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return cadCadPresenceGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) cadCadPresenceGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) cadCadPresenceGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const cadCadPresenceGuardString = (value: unknown, at: string, bounds: cadCadPresenceGuardTextBounds = {}): string => {
  if (typeof value !== "string") return cadCadPresenceGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) cadCadPresenceGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) cadCadPresenceGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) cadCadPresenceGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const cadCadPresenceGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : cadCadPresenceGuardReject(at, "value is not a boolean"));
export const cadCadPresenceGuardNumber = (value: unknown, at: string, bounds: cadCadPresenceGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return cadCadPresenceGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) cadCadPresenceGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) cadCadPresenceGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const cadCadPresenceGuardInteger = (value: unknown, at: string, bounds: cadCadPresenceGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? cadCadPresenceGuardNumber(value, at, bounds) : cadCadPresenceGuardReject(at, "value is not an integer");
export const cadCadPresenceGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : cadCadPresenceGuardReject(at, `value is not one of ${members.join(", ")}`);
export const cadCadPresenceGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : cadCadPresenceGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseCadPresence(value: unknown, at = "$"): CadPresence {
  const row = cadCadPresenceGuardObject(value, at);
  return {
    selectedObjectIds: cadCadPresenceGuardArray(row["selectedObjectIds"], `${at}.selectedObjectIds`).map((item, index) => cadCadPresenceGuardString(item, `${at}.selectedObjectIds[${index}]`)),
    selectedNodeIds: cadCadPresenceGuardArray(row["selectedNodeIds"], `${at}.selectedNodeIds`).map((item, index) => cadCadPresenceGuardString(item, `${at}.selectedNodeIds[${index}]`)),
    hoveredObjectId: row["hoveredObjectId"] === undefined ? undefined : cadCadPresenceGuardString(row["hoveredObjectId"], `${at}.hoveredObjectId`),
    hoveredTargetObjectId: row["hoveredTargetObjectId"] === undefined ? undefined : cadCadPresenceGuardString(row["hoveredTargetObjectId"], `${at}.hoveredTargetObjectId`),
    hoveredTargetMode: row["hoveredTargetMode"] === undefined ? undefined : cadCadPresenceGuardString(row["hoveredTargetMode"], `${at}.hoveredTargetMode`),
    hoveredTargetId: row["hoveredTargetId"] === undefined ? undefined : cadCadPresenceGuardInteger(row["hoveredTargetId"], `${at}.hoveredTargetId`, {"minimum": 0}),
    activeObjectId: row["activeObjectId"] === undefined ? undefined : cadCadPresenceGuardString(row["activeObjectId"], `${at}.activeObjectId`),
    componentSelectionMode: cadCadPresenceGuardString(row["componentSelectionMode"], `${at}.componentSelectionMode`),
    componentSelectionIds: cadCadPresenceGuardArray(row["componentSelectionIds"], `${at}.componentSelectionIds`).map((item, index) => cadCadPresenceGuardInteger(item, `${at}.componentSelectionIds[${index}]`, {"minimum": 0})),
    componentSelectionTargetsMesh: cadCadPresenceGuardBoolean(row["componentSelectionTargetsMesh"], `${at}.componentSelectionTargetsMesh`),
    componentSelectionTargetsVertex: cadCadPresenceGuardBoolean(row["componentSelectionTargetsVertex"], `${at}.componentSelectionTargetsVertex`),
    componentSelectionTargetsEdge: cadCadPresenceGuardBoolean(row["componentSelectionTargetsEdge"], `${at}.componentSelectionTargetsEdge`),
    componentSelectionTargetsFace: cadCadPresenceGuardBoolean(row["componentSelectionTargetsFace"], `${at}.componentSelectionTargetsFace`),
    cameraPosition: cadCadPresenceGuardArray(row["cameraPosition"], `${at}.cameraPosition`, {"minItems": 3, "maxItems": 3}).map((item, index) => cadCadPresenceGuardNumber(item, `${at}.cameraPosition[${index}]`)),
    cameraTarget: cadCadPresenceGuardArray(row["cameraTarget"], `${at}.cameraTarget`, {"minItems": 3, "maxItems": 3}).map((item, index) => cadCadPresenceGuardNumber(item, `${at}.cameraTarget[${index}]`)),
    cameraZoom: cadCadPresenceGuardNumber(row["cameraZoom"], `${at}.cameraZoom`),
    cameraFov: cadCadPresenceGuardNumber(row["cameraFov"], `${at}.cameraFov`),
    activeUtilityId: cadCadPresenceGuardString(row["activeUtilityId"], `${at}.activeUtilityId`),
    engagementStep: cadCadPresenceGuardString(row["engagementStep"], `${at}.engagementStep`),
    engagementPane: row["engagementPane"] === undefined ? undefined : cadCadPresenceGuardString(row["engagementPane"], `${at}.engagementPane`),
  };
}
