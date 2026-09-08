/** 🧬️ Gis3dPresence */
export interface Gis3dPresence {
  /** @state presence */
  cameraJson: string;
  /** @state presence */
  selectedIds: string[];
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class gisGis3dPresenceGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const gisGis3dPresenceGuardReject = (at: string, why: string): never => {
  throw new gisGis3dPresenceGuardRefusal(at, why);
};

type gisGis3dPresenceGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type gisGis3dPresenceGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type gisGis3dPresenceGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const gisGis3dPresenceGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : gisGis3dPresenceGuardReject(at, "value is not an object");
export const gisGis3dPresenceGuardArray = (value: unknown, at: string, bounds: gisGis3dPresenceGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return gisGis3dPresenceGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) gisGis3dPresenceGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) gisGis3dPresenceGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const gisGis3dPresenceGuardString = (value: unknown, at: string, bounds: gisGis3dPresenceGuardTextBounds = {}): string => {
  if (typeof value !== "string") return gisGis3dPresenceGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) gisGis3dPresenceGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) gisGis3dPresenceGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) gisGis3dPresenceGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const gisGis3dPresenceGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : gisGis3dPresenceGuardReject(at, "value is not a boolean"));
export const gisGis3dPresenceGuardNumber = (value: unknown, at: string, bounds: gisGis3dPresenceGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return gisGis3dPresenceGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) gisGis3dPresenceGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) gisGis3dPresenceGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const gisGis3dPresenceGuardInteger = (value: unknown, at: string, bounds: gisGis3dPresenceGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? gisGis3dPresenceGuardNumber(value, at, bounds) : gisGis3dPresenceGuardReject(at, "value is not an integer");
export const gisGis3dPresenceGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : gisGis3dPresenceGuardReject(at, `value is not one of ${members.join(", ")}`);
export const gisGis3dPresenceGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : gisGis3dPresenceGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseGis3dPresence(value: unknown, at = "$"): Gis3dPresence {
  const row = gisGis3dPresenceGuardObject(value, at);
  return {
    cameraJson: gisGis3dPresenceGuardString(row["cameraJson"], `${at}.cameraJson`),
    selectedIds: gisGis3dPresenceGuardArray(row["selectedIds"], `${at}.selectedIds`).map((item, index) => gisGis3dPresenceGuardString(item, `${at}.selectedIds[${index}]`)),
  };
}
