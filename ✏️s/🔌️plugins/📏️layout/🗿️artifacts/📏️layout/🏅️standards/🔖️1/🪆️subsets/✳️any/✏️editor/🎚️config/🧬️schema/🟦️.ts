/** 🧬️ LayoutConfig */
export interface LayoutConfig {
  /** @state config */
  activePageId: string;
  /** @state config */
  selectedIds: string[];
  /** @state config */
  hoveredId?: string;
  /** @state config */
  dropPreview: LayoutDropPreviewState;
  /** @state config */
  engagementInput: string;
  /** @state config */
  camera: LayoutCamera;
  /** @state config */
  previewCamera: LayoutCamera;
  /** @state config */
}
export interface LayoutDropPreviewState { kind: string; x: number; y: number; }
export interface LayoutCamera { x: number; y: number; zoom: number; }

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class layoutLayoutConfigGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const layoutLayoutConfigGuardReject = (at: string, why: string): never => {
  throw new layoutLayoutConfigGuardRefusal(at, why);
};

type layoutLayoutConfigGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type layoutLayoutConfigGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type layoutLayoutConfigGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const layoutLayoutConfigGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : layoutLayoutConfigGuardReject(at, "value is not an object");
export const layoutLayoutConfigGuardArray = (value: unknown, at: string, bounds: layoutLayoutConfigGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return layoutLayoutConfigGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) layoutLayoutConfigGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) layoutLayoutConfigGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const layoutLayoutConfigGuardString = (value: unknown, at: string, bounds: layoutLayoutConfigGuardTextBounds = {}): string => {
  if (typeof value !== "string") return layoutLayoutConfigGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) layoutLayoutConfigGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) layoutLayoutConfigGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) layoutLayoutConfigGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const layoutLayoutConfigGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : layoutLayoutConfigGuardReject(at, "value is not a boolean"));
export const layoutLayoutConfigGuardNumber = (value: unknown, at: string, bounds: layoutLayoutConfigGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return layoutLayoutConfigGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) layoutLayoutConfigGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) layoutLayoutConfigGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const layoutLayoutConfigGuardInteger = (value: unknown, at: string, bounds: layoutLayoutConfigGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? layoutLayoutConfigGuardNumber(value, at, bounds) : layoutLayoutConfigGuardReject(at, "value is not an integer");
export const layoutLayoutConfigGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : layoutLayoutConfigGuardReject(at, `value is not one of ${members.join(", ")}`);
export const layoutLayoutConfigGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : layoutLayoutConfigGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseLayoutConfig(value: unknown, at = "$"): LayoutConfig {
  const row = layoutLayoutConfigGuardObject(value, at);
  return {
    activePageId: layoutLayoutConfigGuardString(row["activePageId"], `${at}.activePageId`),
    selectedIds: layoutLayoutConfigGuardArray(row["selectedIds"], `${at}.selectedIds`).map((item, index) => layoutLayoutConfigGuardString(item, `${at}.selectedIds[${index}]`)),
    hoveredId: row["hoveredId"] === undefined ? undefined : layoutLayoutConfigGuardString(row["hoveredId"], `${at}.hoveredId`),
    dropPreview: parseLayoutDropPreviewState(row["dropPreview"], `${at}.dropPreview`),
    engagementInput: layoutLayoutConfigGuardString(row["engagementInput"], `${at}.engagementInput`),
    camera: parseLayoutCamera(row["camera"], `${at}.camera`),
    previewCamera: parseLayoutCamera(row["previewCamera"], `${at}.previewCamera`),
  };
}

export function parseLayoutDropPreviewState(value: unknown, at = "$"): LayoutDropPreviewState {
  const row = layoutLayoutConfigGuardObject(value, at);
  return {
    kind: layoutLayoutConfigGuardString(row["kind"], `${at}.kind`),
    x: layoutLayoutConfigGuardNumber(row["x"], `${at}.x`),
    y: layoutLayoutConfigGuardNumber(row["y"], `${at}.y`),
  };
}

export function parseLayoutCamera(value: unknown, at = "$"): LayoutCamera {
  const row = layoutLayoutConfigGuardObject(value, at);
  return {
    x: layoutLayoutConfigGuardNumber(row["x"], `${at}.x`),
    y: layoutLayoutConfigGuardNumber(row["y"], `${at}.y`),
    zoom: layoutLayoutConfigGuardNumber(row["zoom"], `${at}.zoom`),
  };
}
