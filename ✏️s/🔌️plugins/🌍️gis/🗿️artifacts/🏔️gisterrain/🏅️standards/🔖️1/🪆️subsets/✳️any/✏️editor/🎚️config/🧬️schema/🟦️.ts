/** 🧬️ Gis3dConfig */
export interface Gis3dConfig {
  /** @state config */
  cameraJson: string;
  /** @state config */
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class gisGis3dConfigGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const gisGis3dConfigGuardReject = (at: string, why: string): never => {
  throw new gisGis3dConfigGuardRefusal(at, why);
};

type gisGis3dConfigGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type gisGis3dConfigGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type gisGis3dConfigGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const gisGis3dConfigGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : gisGis3dConfigGuardReject(at, "value is not an object");
export const gisGis3dConfigGuardArray = (value: unknown, at: string, bounds: gisGis3dConfigGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return gisGis3dConfigGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) gisGis3dConfigGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) gisGis3dConfigGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const gisGis3dConfigGuardString = (value: unknown, at: string, bounds: gisGis3dConfigGuardTextBounds = {}): string => {
  if (typeof value !== "string") return gisGis3dConfigGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) gisGis3dConfigGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) gisGis3dConfigGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) gisGis3dConfigGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const gisGis3dConfigGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : gisGis3dConfigGuardReject(at, "value is not a boolean"));
export const gisGis3dConfigGuardNumber = (value: unknown, at: string, bounds: gisGis3dConfigGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return gisGis3dConfigGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) gisGis3dConfigGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) gisGis3dConfigGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const gisGis3dConfigGuardInteger = (value: unknown, at: string, bounds: gisGis3dConfigGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? gisGis3dConfigGuardNumber(value, at, bounds) : gisGis3dConfigGuardReject(at, "value is not an integer");
export const gisGis3dConfigGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : gisGis3dConfigGuardReject(at, `value is not one of ${members.join(", ")}`);
export const gisGis3dConfigGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : gisGis3dConfigGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseGis3dConfig(value: unknown, at = "$"): Gis3dConfig {
  const row = gisGis3dConfigGuardObject(value, at);
  return {
    cameraJson: gisGis3dConfigGuardString(row["cameraJson"], `${at}.cameraJson`),
  };
}
