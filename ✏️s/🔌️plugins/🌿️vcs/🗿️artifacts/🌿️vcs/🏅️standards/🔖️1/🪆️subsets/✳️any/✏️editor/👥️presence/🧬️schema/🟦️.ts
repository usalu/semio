/** 🧬️ VcsDemoPresence — empty: the VCS play demo keeps all view state in `VcsDemoConfig`; history selection and locale are local config, not shareable live state. Mirrors `VcsDemoPresence` in `../🦀️.rs`. */
export interface VcsDemoPresence {}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class vcsVcsPresenceGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const vcsVcsPresenceGuardReject = (at: string, why: string): never => {
  throw new vcsVcsPresenceGuardRefusal(at, why);
};

type vcsVcsPresenceGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type vcsVcsPresenceGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type vcsVcsPresenceGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const vcsVcsPresenceGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : vcsVcsPresenceGuardReject(at, "value is not an object");
export const vcsVcsPresenceGuardArray = (value: unknown, at: string, bounds: vcsVcsPresenceGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return vcsVcsPresenceGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) vcsVcsPresenceGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) vcsVcsPresenceGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const vcsVcsPresenceGuardString = (value: unknown, at: string, bounds: vcsVcsPresenceGuardTextBounds = {}): string => {
  if (typeof value !== "string") return vcsVcsPresenceGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) vcsVcsPresenceGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) vcsVcsPresenceGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) vcsVcsPresenceGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const vcsVcsPresenceGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : vcsVcsPresenceGuardReject(at, "value is not a boolean"));
export const vcsVcsPresenceGuardNumber = (value: unknown, at: string, bounds: vcsVcsPresenceGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return vcsVcsPresenceGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) vcsVcsPresenceGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) vcsVcsPresenceGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const vcsVcsPresenceGuardInteger = (value: unknown, at: string, bounds: vcsVcsPresenceGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? vcsVcsPresenceGuardNumber(value, at, bounds) : vcsVcsPresenceGuardReject(at, "value is not an integer");
export const vcsVcsPresenceGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : vcsVcsPresenceGuardReject(at, `value is not one of ${members.join(", ")}`);
export const vcsVcsPresenceGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : vcsVcsPresenceGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseVcsDemoPresence(value: unknown, at = "$"): VcsDemoPresence {
  return vcsVcsPresenceGuardObject(value, `${at}`);
}
