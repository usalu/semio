/** 🧬️ VcsDemoConfig */
export interface VcsDemoConfig {
  /** @state config */
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class vcsVcsConfigGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const vcsVcsConfigGuardReject = (at: string, why: string): never => {
  throw new vcsVcsConfigGuardRefusal(at, why);
};

type vcsVcsConfigGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type vcsVcsConfigGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type vcsVcsConfigGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const vcsVcsConfigGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : vcsVcsConfigGuardReject(at, "value is not an object");
export const vcsVcsConfigGuardArray = (value: unknown, at: string, bounds: vcsVcsConfigGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return vcsVcsConfigGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) vcsVcsConfigGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) vcsVcsConfigGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const vcsVcsConfigGuardString = (value: unknown, at: string, bounds: vcsVcsConfigGuardTextBounds = {}): string => {
  if (typeof value !== "string") return vcsVcsConfigGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) vcsVcsConfigGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) vcsVcsConfigGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) vcsVcsConfigGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const vcsVcsConfigGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : vcsVcsConfigGuardReject(at, "value is not a boolean"));
export const vcsVcsConfigGuardNumber = (value: unknown, at: string, bounds: vcsVcsConfigGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return vcsVcsConfigGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) vcsVcsConfigGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) vcsVcsConfigGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const vcsVcsConfigGuardInteger = (value: unknown, at: string, bounds: vcsVcsConfigGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? vcsVcsConfigGuardNumber(value, at, bounds) : vcsVcsConfigGuardReject(at, "value is not an integer");
export const vcsVcsConfigGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : vcsVcsConfigGuardReject(at, `value is not one of ${members.join(", ")}`);
export const vcsVcsConfigGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : vcsVcsConfigGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseVcsDemoConfig(value: unknown, at = "$"): VcsDemoConfig {
  return vcsVcsConfigGuardObject(value, `${at}`);
}
