/** 🧬️ Process3dPresence */
export interface Process3dPresence {
  /** 🛰️ @state presence */
  engagementInput: string;
  /** 📷️ @state presence */
  cameraPosition: number[];
  /** 🎥️ @state presence */
  cameraTarget: number[];
  /** 📸️ @state presence */
  cameraFov: number;
  /** 🟢️ @state presence */
  activeUtilityId: string;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class process3dPresenceGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const process3dPresenceGuardReject = (at: string, why: string): never => {
  throw new process3dPresenceGuardRefusal(at, why);
};

type process3dPresenceGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type process3dPresenceGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type process3dPresenceGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const process3dPresenceGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : process3dPresenceGuardReject(at, "value is not an object");
export const process3dPresenceGuardClosed = (row: Readonly<Record<string, unknown>>, at: string, keys: readonly string[]): Readonly<Record<string, unknown>> => {
  for (const key of Object.keys(row)) if (!keys.includes(key)) process3dPresenceGuardReject(`${at}.${key}`, "unknown property");
  return row;
};
export const process3dPresenceGuardArray = (value: unknown, at: string, bounds: process3dPresenceGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return process3dPresenceGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) process3dPresenceGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) process3dPresenceGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const process3dPresenceGuardString = (value: unknown, at: string, bounds: process3dPresenceGuardTextBounds = {}): string => {
  if (typeof value !== "string") return process3dPresenceGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) process3dPresenceGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) process3dPresenceGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) process3dPresenceGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const process3dPresenceGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : process3dPresenceGuardReject(at, "value is not a boolean"));
export const process3dPresenceGuardNumber = (value: unknown, at: string, bounds: process3dPresenceGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return process3dPresenceGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) process3dPresenceGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) process3dPresenceGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const process3dPresenceGuardInteger = (value: unknown, at: string, bounds: process3dPresenceGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? process3dPresenceGuardNumber(value, at, bounds) : process3dPresenceGuardReject(at, "value is not an integer");
export const process3dPresenceGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : process3dPresenceGuardReject(at, `value is not one of ${members.join(", ")}`);
export const process3dPresenceGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : process3dPresenceGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseProcess3dPresence(value: unknown, at = "$"): Process3dPresence {
  const row = process3dPresenceGuardClosed(process3dPresenceGuardObject(value, at), at, ["engagementInput", "cameraPosition", "cameraTarget", "cameraFov", "activeUtilityId"]);
  return {
    engagementInput: process3dPresenceGuardString(row["engagementInput"], `${at}.engagementInput`),
    cameraPosition: ((items0) => [process3dPresenceGuardNumber(items0[0], `${at}.cameraPosition[0]`), process3dPresenceGuardNumber(items0[1], `${at}.cameraPosition[1]`), process3dPresenceGuardNumber(items0[2], `${at}.cameraPosition[2]`)])(process3dPresenceGuardArray(row["cameraPosition"], `${at}.cameraPosition`, {"minItems": 3, "maxItems": 3})),
    cameraTarget: ((items0) => [process3dPresenceGuardNumber(items0[0], `${at}.cameraTarget[0]`), process3dPresenceGuardNumber(items0[1], `${at}.cameraTarget[1]`), process3dPresenceGuardNumber(items0[2], `${at}.cameraTarget[2]`)])(process3dPresenceGuardArray(row["cameraTarget"], `${at}.cameraTarget`, {"minItems": 3, "maxItems": 3})),
    cameraFov: process3dPresenceGuardNumber(row["cameraFov"], `${at}.cameraFov`),
    activeUtilityId: process3dPresenceGuardString(row["activeUtilityId"], `${at}.activeUtilityId`),
  };
}
