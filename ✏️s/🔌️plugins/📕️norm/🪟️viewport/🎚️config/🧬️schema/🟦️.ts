/** 🧬️ NormViewportWindowConfig */
export interface NormViewportWindowConfig {
  /** 👉️ @state windowConfig */
  cameraPreset?: string;
  /** 👉️ @state windowConfig */
  hiddenLayers: string[];
  /** 👉️ @state windowConfig */
  colourMap?: string;
  /** 👉️ @state windowConfig */
  unitSystem?: string;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class normNormViewportWindowConfigGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const normNormViewportWindowConfigGuardReject = (at: string, why: string): never => {
  throw new normNormViewportWindowConfigGuardRefusal(at, why);
};

type normNormViewportWindowConfigGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type normNormViewportWindowConfigGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type normNormViewportWindowConfigGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const normNormViewportWindowConfigGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : normNormViewportWindowConfigGuardReject(at, "value is not an object");
export const normNormViewportWindowConfigGuardClosed = (row: Readonly<Record<string, unknown>>, at: string, keys: readonly string[]): Readonly<Record<string, unknown>> => {
  for (const key of Object.keys(row)) if (!keys.includes(key)) normNormViewportWindowConfigGuardReject(`${at}.${key}`, "unknown property");
  return row;
};
export const normNormViewportWindowConfigGuardArray = (value: unknown, at: string, bounds: normNormViewportWindowConfigGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return normNormViewportWindowConfigGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) normNormViewportWindowConfigGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) normNormViewportWindowConfigGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const normNormViewportWindowConfigGuardString = (value: unknown, at: string, bounds: normNormViewportWindowConfigGuardTextBounds = {}): string => {
  if (typeof value !== "string") return normNormViewportWindowConfigGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) normNormViewportWindowConfigGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) normNormViewportWindowConfigGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) normNormViewportWindowConfigGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const normNormViewportWindowConfigGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : normNormViewportWindowConfigGuardReject(at, "value is not a boolean"));
export const normNormViewportWindowConfigGuardNumber = (value: unknown, at: string, bounds: normNormViewportWindowConfigGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return normNormViewportWindowConfigGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) normNormViewportWindowConfigGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) normNormViewportWindowConfigGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const normNormViewportWindowConfigGuardInteger = (value: unknown, at: string, bounds: normNormViewportWindowConfigGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? normNormViewportWindowConfigGuardNumber(value, at, bounds) : normNormViewportWindowConfigGuardReject(at, "value is not an integer");
export const normNormViewportWindowConfigGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : normNormViewportWindowConfigGuardReject(at, `value is not one of ${members.join(", ")}`);
export const normNormViewportWindowConfigGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : normNormViewportWindowConfigGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseNormViewportWindowConfig(value: unknown, at = "$"): NormViewportWindowConfig {
  const row = normNormViewportWindowConfigGuardClosed(normNormViewportWindowConfigGuardObject(value, at), at, ["cameraPreset", "hiddenLayers", "colourMap", "unitSystem"]);
  const text = (key: string): string | undefined => (row[key] === undefined ? undefined : normNormViewportWindowConfigGuardString(row[key], `${at}.${key}`));
  return {
    cameraPreset: text("cameraPreset"),
    hiddenLayers: row["hiddenLayers"] === undefined ? [] : normNormViewportWindowConfigGuardArray(row["hiddenLayers"], `${at}.hiddenLayers`).map((layer, index) => normNormViewportWindowConfigGuardString(layer, `${at}.hiddenLayers[${index}]`)),
    colourMap: text("colourMap"),
    unitSystem: text("unitSystem"),
  };
}
