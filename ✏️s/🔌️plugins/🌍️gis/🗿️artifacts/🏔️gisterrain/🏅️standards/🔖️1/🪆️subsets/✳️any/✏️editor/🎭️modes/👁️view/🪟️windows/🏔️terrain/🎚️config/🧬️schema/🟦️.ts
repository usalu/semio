import { parseSchemaRecord } from "../../../../../../../../../../../../../../../../🧰️framework/🔨️modules/🧬️schema/🧾️record/🟦️.ts";

/** 🧬️ GisTerrainWindowConfig */
export interface GisTerrainWindowConfig {
  /** @state config @owner window */
  cameraJson: string;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class gisGisTerrainWindowConfigGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const gisGisTerrainWindowConfigGuardReject = (at: string, why: string): never => {
  throw new gisGisTerrainWindowConfigGuardRefusal(at, why);
};

type gisGisTerrainWindowConfigGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type gisGisTerrainWindowConfigGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type gisGisTerrainWindowConfigGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const gisGisTerrainWindowConfigGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : gisGisTerrainWindowConfigGuardReject(at, "value is not an object");
export const gisGisTerrainWindowConfigGuardArray = (value: unknown, at: string, bounds: gisGisTerrainWindowConfigGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return gisGisTerrainWindowConfigGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) gisGisTerrainWindowConfigGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) gisGisTerrainWindowConfigGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const gisGisTerrainWindowConfigGuardString = (value: unknown, at: string, bounds: gisGisTerrainWindowConfigGuardTextBounds = {}): string => {
  if (typeof value !== "string") return gisGisTerrainWindowConfigGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) gisGisTerrainWindowConfigGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) gisGisTerrainWindowConfigGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) gisGisTerrainWindowConfigGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const gisGisTerrainWindowConfigGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : gisGisTerrainWindowConfigGuardReject(at, "value is not a boolean"));
export const gisGisTerrainWindowConfigGuardNumber = (value: unknown, at: string, bounds: gisGisTerrainWindowConfigGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return gisGisTerrainWindowConfigGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) gisGisTerrainWindowConfigGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) gisGisTerrainWindowConfigGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const gisGisTerrainWindowConfigGuardInteger = (value: unknown, at: string, bounds: gisGisTerrainWindowConfigGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? gisGisTerrainWindowConfigGuardNumber(value, at, bounds) : gisGisTerrainWindowConfigGuardReject(at, "value is not an integer");
export const gisGisTerrainWindowConfigGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : gisGisTerrainWindowConfigGuardReject(at, `value is not one of ${members.join(", ")}`);
export const gisGisTerrainWindowConfigGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : gisGisTerrainWindowConfigGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseGisTerrainWindowConfig(value: unknown, at = "$"): GisTerrainWindowConfig {
  const row = parseSchemaRecord(value, ["cameraJson"], at);
  return {
    cameraJson: gisGisTerrainWindowConfigGuardString(row["cameraJson"], `${at}.cameraJson`),
  };
}
