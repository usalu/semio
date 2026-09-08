/** 💡️ Wires inference schema — topology (node/edge/component counts + cycle-freedom) over the board graph. */

export interface WiresTopology {
  nodeCount: number;
  edgeCount: number;
  componentCount: number;
  cycleFree: boolean;
}

export interface WiresInference {
  /** @derived */
  topology: WiresTopology;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class reasoningWiresInferenceGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const reasoningWiresInferenceGuardReject = (at: string, why: string): never => {
  throw new reasoningWiresInferenceGuardRefusal(at, why);
};

type reasoningWiresInferenceGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type reasoningWiresInferenceGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type reasoningWiresInferenceGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const reasoningWiresInferenceGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : reasoningWiresInferenceGuardReject(at, "value is not an object");
export const reasoningWiresInferenceGuardArray = (value: unknown, at: string, bounds: reasoningWiresInferenceGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return reasoningWiresInferenceGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) reasoningWiresInferenceGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) reasoningWiresInferenceGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const reasoningWiresInferenceGuardString = (value: unknown, at: string, bounds: reasoningWiresInferenceGuardTextBounds = {}): string => {
  if (typeof value !== "string") return reasoningWiresInferenceGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) reasoningWiresInferenceGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) reasoningWiresInferenceGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) reasoningWiresInferenceGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const reasoningWiresInferenceGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : reasoningWiresInferenceGuardReject(at, "value is not a boolean"));
export const reasoningWiresInferenceGuardNumber = (value: unknown, at: string, bounds: reasoningWiresInferenceGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return reasoningWiresInferenceGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) reasoningWiresInferenceGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) reasoningWiresInferenceGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const reasoningWiresInferenceGuardInteger = (value: unknown, at: string, bounds: reasoningWiresInferenceGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? reasoningWiresInferenceGuardNumber(value, at, bounds) : reasoningWiresInferenceGuardReject(at, "value is not an integer");
export const reasoningWiresInferenceGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : reasoningWiresInferenceGuardReject(at, `value is not one of ${members.join(", ")}`);
export const reasoningWiresInferenceGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : reasoningWiresInferenceGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseWiresInference(value: unknown, at = "$"): WiresInference {
  const row = reasoningWiresInferenceGuardObject(value, at);
  return {
    topology: parseWiresTopology(row["topology"], `${at}.topology`),
  };
}

export function parseWiresTopology(value: unknown, at = "$"): WiresTopology {
  const row = reasoningWiresInferenceGuardObject(value, at);
  return {
    nodeCount: reasoningWiresInferenceGuardInteger(row["nodeCount"], `${at}.nodeCount`, {"minimum": 0}),
    edgeCount: reasoningWiresInferenceGuardInteger(row["edgeCount"], `${at}.edgeCount`, {"minimum": 0}),
    componentCount: reasoningWiresInferenceGuardInteger(row["componentCount"], `${at}.componentCount`, {"minimum": 0}),
    cycleFree: reasoningWiresInferenceGuardBoolean(row["cycleFree"], `${at}.cycleFree`),
  };
}
