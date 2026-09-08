/** 🧬️ LayoutPresence */
export interface LayoutPresence {
  /** @state presence */
  activePageId: string;
  /** @state presence */
  selectedIds: string[];
  /** @state presence */
  hoveredId?: string;
  /** @state presence */
  dropPreview: LayoutDropPreviewState;
  /** @state presence */
  camera: LayoutCamera;
  /** @state presence */
  previewCamera: LayoutCamera;
}
export interface LayoutDropPreviewState { kind: string; x: number; y: number; }
export interface LayoutCamera { x: number; y: number; zoom: number; }

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class layoutLayoutPresenceGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const layoutLayoutPresenceGuardReject = (at: string, why: string): never => {
  throw new layoutLayoutPresenceGuardRefusal(at, why);
};

type layoutLayoutPresenceGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type layoutLayoutPresenceGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type layoutLayoutPresenceGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const layoutLayoutPresenceGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : layoutLayoutPresenceGuardReject(at, "value is not an object");
export const layoutLayoutPresenceGuardArray = (value: unknown, at: string, bounds: layoutLayoutPresenceGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return layoutLayoutPresenceGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) layoutLayoutPresenceGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) layoutLayoutPresenceGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const layoutLayoutPresenceGuardString = (value: unknown, at: string, bounds: layoutLayoutPresenceGuardTextBounds = {}): string => {
  if (typeof value !== "string") return layoutLayoutPresenceGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) layoutLayoutPresenceGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) layoutLayoutPresenceGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) layoutLayoutPresenceGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const layoutLayoutPresenceGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : layoutLayoutPresenceGuardReject(at, "value is not a boolean"));
export const layoutLayoutPresenceGuardNumber = (value: unknown, at: string, bounds: layoutLayoutPresenceGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return layoutLayoutPresenceGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) layoutLayoutPresenceGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) layoutLayoutPresenceGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const layoutLayoutPresenceGuardInteger = (value: unknown, at: string, bounds: layoutLayoutPresenceGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? layoutLayoutPresenceGuardNumber(value, at, bounds) : layoutLayoutPresenceGuardReject(at, "value is not an integer");
export const layoutLayoutPresenceGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : layoutLayoutPresenceGuardReject(at, `value is not one of ${members.join(", ")}`);
export const layoutLayoutPresenceGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : layoutLayoutPresenceGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseLayoutPresence(value: unknown, at = "$"): LayoutPresence {
  const row = layoutLayoutPresenceGuardObject(value, at);
  return {
    activePageId: layoutLayoutPresenceGuardString(row["activePageId"], `${at}.activePageId`),
    selectedIds: layoutLayoutPresenceGuardArray(row["selectedIds"], `${at}.selectedIds`).map((item, index) => layoutLayoutPresenceGuardString(item, `${at}.selectedIds[${index}]`)),
    hoveredId: row["hoveredId"] === undefined ? undefined : layoutLayoutPresenceGuardString(row["hoveredId"], `${at}.hoveredId`),
    dropPreview: parseLayoutDropPreviewState(row["dropPreview"], `${at}.dropPreview`),
    camera: parseLayoutCamera(row["camera"], `${at}.camera`),
    previewCamera: parseLayoutCamera(row["previewCamera"], `${at}.previewCamera`),
  };
}

export function parseLayoutDropPreviewState(value: unknown, at = "$"): LayoutDropPreviewState {
  const row = layoutLayoutPresenceGuardObject(value, at);
  return {
    kind: layoutLayoutPresenceGuardString(row["kind"], `${at}.kind`),
    x: layoutLayoutPresenceGuardNumber(row["x"], `${at}.x`),
    y: layoutLayoutPresenceGuardNumber(row["y"], `${at}.y`),
  };
}

export function parseLayoutCamera(value: unknown, at = "$"): LayoutCamera {
  const row = layoutLayoutPresenceGuardObject(value, at);
  return {
    x: layoutLayoutPresenceGuardNumber(row["x"], `${at}.x`),
    y: layoutLayoutPresenceGuardNumber(row["y"], `${at}.y`),
    zoom: layoutLayoutPresenceGuardNumber(row["zoom"], `${at}.zoom`),
  };
}
