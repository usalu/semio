//#region 🧬️Configuration
/** 🧬️ Gis2dConfig */
export interface Gis2dConfig {
  /** @state config */
  layerVisibility: Record<string, boolean>;
  /** @state config */
  cameraJson: string;
  /** @state config */
  renderMode: string;
  /** @state config */
  vectorStyle: string;
  /** @state config */
  lodMode: string;
  /** @state config */
  layerStrokeScale: Record<string, number>;
  /** @state config */
}
//#endregion 🧬️Configuration

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class gisGis2dConfigGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const gisGis2dConfigGuardReject = (at: string, why: string): never => {
  throw new gisGis2dConfigGuardRefusal(at, why);
};

type gisGis2dConfigGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type gisGis2dConfigGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type gisGis2dConfigGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const gisGis2dConfigGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : gisGis2dConfigGuardReject(at, "value is not an object");
export const gisGis2dConfigGuardArray = (value: unknown, at: string, bounds: gisGis2dConfigGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return gisGis2dConfigGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) gisGis2dConfigGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) gisGis2dConfigGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const gisGis2dConfigGuardString = (value: unknown, at: string, bounds: gisGis2dConfigGuardTextBounds = {}): string => {
  if (typeof value !== "string") return gisGis2dConfigGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) gisGis2dConfigGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) gisGis2dConfigGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) gisGis2dConfigGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const gisGis2dConfigGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : gisGis2dConfigGuardReject(at, "value is not a boolean"));
export const gisGis2dConfigGuardNumber = (value: unknown, at: string, bounds: gisGis2dConfigGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return gisGis2dConfigGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) gisGis2dConfigGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) gisGis2dConfigGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const gisGis2dConfigGuardInteger = (value: unknown, at: string, bounds: gisGis2dConfigGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? gisGis2dConfigGuardNumber(value, at, bounds) : gisGis2dConfigGuardReject(at, "value is not an integer");
export const gisGis2dConfigGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : gisGis2dConfigGuardReject(at, `value is not one of ${members.join(", ")}`);
export const gisGis2dConfigGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : gisGis2dConfigGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseGis2dConfig(value: unknown, at = "$"): Gis2dConfig {
  const row = gisGis2dConfigGuardObject(value, at);
  return {
    layerVisibility: gisGis2dConfigGuardObject(row["layerVisibility"], `${at}.layerVisibility`),
    cameraJson: gisGis2dConfigGuardString(row["cameraJson"], `${at}.cameraJson`),
    renderMode: gisGis2dConfigGuardString(row["renderMode"], `${at}.renderMode`),
    vectorStyle: gisGis2dConfigGuardString(row["vectorStyle"], `${at}.vectorStyle`),
    lodMode: gisGis2dConfigGuardString(row["lodMode"], `${at}.lodMode`),
    layerStrokeScale: gisGis2dConfigGuardObject(row["layerStrokeScale"], `${at}.layerStrokeScale`),
  };
}
