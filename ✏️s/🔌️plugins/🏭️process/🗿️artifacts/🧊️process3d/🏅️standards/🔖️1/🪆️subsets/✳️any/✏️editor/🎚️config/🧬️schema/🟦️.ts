/** 🧬️ Process3dConfig */
export interface Process3dConfig {
  /** @state config */
  engagementInput: string;
  /** @state config */
  cameraPosition: number[];
  /** @state config */
  cameraTarget: number[];
  /** @state config */
  cameraFov: number;
  /** @state config */
  sunEnabled: boolean;
  /** @state config */
  sunAzimuth: number;
  /** @state config */
  sunElevation: number;
  /** @state config */
  sunIntensity: number;
  /** @state config */
  sunColor: string;
  /** @state config */
  contributionsJson: string;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class process3dConfigGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const process3dConfigGuardReject = (at: string, why: string): never => {
  throw new process3dConfigGuardRefusal(at, why);
};

type process3dConfigGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type process3dConfigGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type process3dConfigGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const process3dConfigGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : process3dConfigGuardReject(at, "value is not an object");
export const process3dConfigGuardArray = (value: unknown, at: string, bounds: process3dConfigGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return process3dConfigGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) process3dConfigGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) process3dConfigGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const process3dConfigGuardString = (value: unknown, at: string, bounds: process3dConfigGuardTextBounds = {}): string => {
  if (typeof value !== "string") return process3dConfigGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) process3dConfigGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) process3dConfigGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) process3dConfigGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const process3dConfigGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : process3dConfigGuardReject(at, "value is not a boolean"));
export const process3dConfigGuardNumber = (value: unknown, at: string, bounds: process3dConfigGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return process3dConfigGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) process3dConfigGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) process3dConfigGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const process3dConfigGuardInteger = (value: unknown, at: string, bounds: process3dConfigGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? process3dConfigGuardNumber(value, at, bounds) : process3dConfigGuardReject(at, "value is not an integer");
export const process3dConfigGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : process3dConfigGuardReject(at, `value is not one of ${members.join(", ")}`);
export const process3dConfigGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : process3dConfigGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseProcess3dConfig(value: unknown, at = "$"): Process3dConfig {
  const row = process3dConfigGuardObject(value, at);
  return {
    engagementInput: process3dConfigGuardString(row["engagementInput"], `${at}.engagementInput`),
    cameraPosition: process3dConfigGuardArray(row["cameraPosition"], `${at}.cameraPosition`, {"minItems": 3, "maxItems": 3}).map((item, index) => process3dConfigGuardNumber(item, `${at}.cameraPosition[${index}]`)),
    cameraTarget: process3dConfigGuardArray(row["cameraTarget"], `${at}.cameraTarget`, {"minItems": 3, "maxItems": 3}).map((item, index) => process3dConfigGuardNumber(item, `${at}.cameraTarget[${index}]`)),
    cameraFov: process3dConfigGuardNumber(row["cameraFov"], `${at}.cameraFov`),
    sunEnabled: process3dConfigGuardBoolean(row["sunEnabled"], `${at}.sunEnabled`),
    sunAzimuth: process3dConfigGuardNumber(row["sunAzimuth"], `${at}.sunAzimuth`),
    sunElevation: process3dConfigGuardNumber(row["sunElevation"], `${at}.sunElevation`),
    sunIntensity: process3dConfigGuardNumber(row["sunIntensity"], `${at}.sunIntensity`),
    sunColor: process3dConfigGuardString(row["sunColor"], `${at}.sunColor`),
    contributionsJson: process3dConfigGuardString(row["contributionsJson"], `${at}.contributionsJson`),
  };
}
