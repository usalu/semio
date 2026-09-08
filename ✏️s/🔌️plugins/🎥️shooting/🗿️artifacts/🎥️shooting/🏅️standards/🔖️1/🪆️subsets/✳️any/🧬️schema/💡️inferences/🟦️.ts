/** 💡️ Shooting inference schema — topology derived from the shot→camera reference graph. */

export interface ShootingTopology {
  topoOrder: string[];
  depth: Record<string, number>;
  cycleFree: boolean;
  nodeCount: number;
}

export interface ShootingInference {
  /** @derived */
  topology: ShootingTopology;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class shootingShootingInferenceGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const shootingShootingInferenceGuardReject = (at: string, why: string): never => {
  throw new shootingShootingInferenceGuardRefusal(at, why);
};

type shootingShootingInferenceGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type shootingShootingInferenceGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type shootingShootingInferenceGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const shootingShootingInferenceGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : shootingShootingInferenceGuardReject(at, "value is not an object");
export const shootingShootingInferenceGuardArray = (value: unknown, at: string, bounds: shootingShootingInferenceGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return shootingShootingInferenceGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) shootingShootingInferenceGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) shootingShootingInferenceGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const shootingShootingInferenceGuardString = (value: unknown, at: string, bounds: shootingShootingInferenceGuardTextBounds = {}): string => {
  if (typeof value !== "string") return shootingShootingInferenceGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) shootingShootingInferenceGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) shootingShootingInferenceGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) shootingShootingInferenceGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const shootingShootingInferenceGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : shootingShootingInferenceGuardReject(at, "value is not a boolean"));
export const shootingShootingInferenceGuardNumber = (value: unknown, at: string, bounds: shootingShootingInferenceGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return shootingShootingInferenceGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) shootingShootingInferenceGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) shootingShootingInferenceGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const shootingShootingInferenceGuardInteger = (value: unknown, at: string, bounds: shootingShootingInferenceGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? shootingShootingInferenceGuardNumber(value, at, bounds) : shootingShootingInferenceGuardReject(at, "value is not an integer");
export const shootingShootingInferenceGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : shootingShootingInferenceGuardReject(at, `value is not one of ${members.join(", ")}`);
export const shootingShootingInferenceGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : shootingShootingInferenceGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseShootingInference(value: unknown, at = "$"): ShootingInference {
  const row = shootingShootingInferenceGuardObject(value, at);
  return {
    topology: parseShootingTopology(row["topology"], `${at}.topology`),
  };
}

export function parseShootingTopology(value: unknown, at = "$"): ShootingTopology {
  const row = shootingShootingInferenceGuardObject(value, at);
  return {
    topoOrder: shootingShootingInferenceGuardArray(row["topoOrder"], `${at}.topoOrder`).map((item, index) => shootingShootingInferenceGuardString(item, `${at}.topoOrder[${index}]`)),
    depth: shootingShootingInferenceGuardObject(row["depth"], `${at}.depth`),
    cycleFree: shootingShootingInferenceGuardBoolean(row["cycleFree"], `${at}.cycleFree`),
    nodeCount: shootingShootingInferenceGuardInteger(row["nodeCount"], `${at}.nodeCount`),
  };
}
