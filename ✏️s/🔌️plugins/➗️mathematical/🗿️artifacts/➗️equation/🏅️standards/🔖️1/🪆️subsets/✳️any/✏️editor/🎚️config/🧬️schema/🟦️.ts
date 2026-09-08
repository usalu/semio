/** 🧬️ EquationCamera */
export interface EquationCamera {
  x: number;
  y: number;
  zoom: number;
}

/** 🧬️ EquationConfig */
export interface EquationConfig {
  /** @state config */
  camera: EquationCamera;
  /** @state config */
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class equationEquationConfigGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const equationEquationConfigGuardReject = (at: string, why: string): never => {
  throw new equationEquationConfigGuardRefusal(at, why);
};

type equationEquationConfigGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type equationEquationConfigGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type equationEquationConfigGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const equationEquationConfigGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : equationEquationConfigGuardReject(at, "value is not an object");
export const equationEquationConfigGuardArray = (value: unknown, at: string, bounds: equationEquationConfigGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return equationEquationConfigGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) equationEquationConfigGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) equationEquationConfigGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const equationEquationConfigGuardString = (value: unknown, at: string, bounds: equationEquationConfigGuardTextBounds = {}): string => {
  if (typeof value !== "string") return equationEquationConfigGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) equationEquationConfigGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) equationEquationConfigGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) equationEquationConfigGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const equationEquationConfigGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : equationEquationConfigGuardReject(at, "value is not a boolean"));
export const equationEquationConfigGuardNumber = (value: unknown, at: string, bounds: equationEquationConfigGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return equationEquationConfigGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) equationEquationConfigGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) equationEquationConfigGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const equationEquationConfigGuardInteger = (value: unknown, at: string, bounds: equationEquationConfigGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? equationEquationConfigGuardNumber(value, at, bounds) : equationEquationConfigGuardReject(at, "value is not an integer");
export const equationEquationConfigGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : equationEquationConfigGuardReject(at, `value is not one of ${members.join(", ")}`);
export const equationEquationConfigGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : equationEquationConfigGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseEquationConfig(value: unknown, at = "$"): EquationConfig {
  const row = equationEquationConfigGuardObject(value, at);
  return {
    camera: parseEquationCamera(row["camera"], `${at}.camera`),
  };
}

export function parseEquationCamera(value: unknown, at = "$"): EquationCamera {
  const row = equationEquationConfigGuardObject(value, at);
  return {
    x: equationEquationConfigGuardNumber(row["x"], `${at}.x`),
    y: equationEquationConfigGuardNumber(row["y"], `${at}.y`),
    zoom: equationEquationConfigGuardNumber(row["zoom"], `${at}.zoom`),
  };
}
