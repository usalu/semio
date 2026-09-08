/** 🧬️ Gis2dPresence */
export interface Gis2dPresence {
  /** @state presence */
  cameraJson: string;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class gisGis2dPresenceGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const gisGis2dPresenceGuardReject = (at: string, why: string): never => {
  throw new gisGis2dPresenceGuardRefusal(at, why);
};

type gisGis2dPresenceGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type gisGis2dPresenceGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type gisGis2dPresenceGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const gisGis2dPresenceGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : gisGis2dPresenceGuardReject(at, "value is not an object");
export const gisGis2dPresenceGuardArray = (value: unknown, at: string, bounds: gisGis2dPresenceGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return gisGis2dPresenceGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) gisGis2dPresenceGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) gisGis2dPresenceGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const gisGis2dPresenceGuardString = (value: unknown, at: string, bounds: gisGis2dPresenceGuardTextBounds = {}): string => {
  if (typeof value !== "string") return gisGis2dPresenceGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) gisGis2dPresenceGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) gisGis2dPresenceGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) gisGis2dPresenceGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const gisGis2dPresenceGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : gisGis2dPresenceGuardReject(at, "value is not a boolean"));
export const gisGis2dPresenceGuardNumber = (value: unknown, at: string, bounds: gisGis2dPresenceGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return gisGis2dPresenceGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) gisGis2dPresenceGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) gisGis2dPresenceGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const gisGis2dPresenceGuardInteger = (value: unknown, at: string, bounds: gisGis2dPresenceGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? gisGis2dPresenceGuardNumber(value, at, bounds) : gisGis2dPresenceGuardReject(at, "value is not an integer");
export const gisGis2dPresenceGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : gisGis2dPresenceGuardReject(at, `value is not one of ${members.join(", ")}`);
export const gisGis2dPresenceGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : gisGis2dPresenceGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseGis2dPresence(value: unknown, at = "$"): Gis2dPresence {
  const row = gisGis2dPresenceGuardObject(value, at);
  return {
    cameraJson: gisGis2dPresenceGuardString(row["cameraJson"], `${at}.cameraJson`),
  };
}
