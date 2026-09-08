/** 💡️ Presentation inference schema — topology derived from the persisted tile order. */

export interface PresentationTopology {
  topoOrder: string[];
  depth: Record<string, number>;
  cycleFree: boolean;
  nodeCount: number;
}

export interface PresentationInference {
  /** @derived */
  topology: PresentationTopology;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class animatePresentationInferenceGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const animatePresentationInferenceGuardReject = (at: string, why: string): never => {
  throw new animatePresentationInferenceGuardRefusal(at, why);
};

type animatePresentationInferenceGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type animatePresentationInferenceGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type animatePresentationInferenceGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const animatePresentationInferenceGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : animatePresentationInferenceGuardReject(at, "value is not an object");
export const animatePresentationInferenceGuardArray = (value: unknown, at: string, bounds: animatePresentationInferenceGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return animatePresentationInferenceGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) animatePresentationInferenceGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) animatePresentationInferenceGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const animatePresentationInferenceGuardString = (value: unknown, at: string, bounds: animatePresentationInferenceGuardTextBounds = {}): string => {
  if (typeof value !== "string") return animatePresentationInferenceGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) animatePresentationInferenceGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) animatePresentationInferenceGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) animatePresentationInferenceGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const animatePresentationInferenceGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : animatePresentationInferenceGuardReject(at, "value is not a boolean"));
export const animatePresentationInferenceGuardNumber = (value: unknown, at: string, bounds: animatePresentationInferenceGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return animatePresentationInferenceGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) animatePresentationInferenceGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) animatePresentationInferenceGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const animatePresentationInferenceGuardInteger = (value: unknown, at: string, bounds: animatePresentationInferenceGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? animatePresentationInferenceGuardNumber(value, at, bounds) : animatePresentationInferenceGuardReject(at, "value is not an integer");
export const animatePresentationInferenceGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : animatePresentationInferenceGuardReject(at, `value is not one of ${members.join(", ")}`);
export const animatePresentationInferenceGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : animatePresentationInferenceGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parsePresentationInference(value: unknown, at = "$"): PresentationInference {
  const row = animatePresentationInferenceGuardObject(value, at);
  return {
    topology: parsePresentationTopology(row["topology"], `${at}.topology`),
  };
}

export function parsePresentationTopology(value: unknown, at = "$"): PresentationTopology {
  const row = animatePresentationInferenceGuardObject(value, at);
  return {
    topoOrder: animatePresentationInferenceGuardArray(row["topoOrder"], `${at}.topoOrder`).map((item, index) => animatePresentationInferenceGuardString(item, `${at}.topoOrder[${index}]`)),
    depth: animatePresentationInferenceGuardObject(row["depth"], `${at}.depth`),
    cycleFree: animatePresentationInferenceGuardBoolean(row["cycleFree"], `${at}.cycleFree`),
    nodeCount: animatePresentationInferenceGuardInteger(row["nodeCount"], `${at}.nodeCount`),
  };
}
