/** 💡️ Generation2d inference schema — topology (DAG shape of `fixture`'s widget/synapse graph). */

export interface Generation2dTopology {
  nodeCount: number;
  edgeCount: number;
  topoOrder: string[];
  depth: number;
  cycleFree: boolean;
}

export interface Generation2dInference {
  /** @derived */
  topology: Generation2dTopology;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class proceduralGeneration2dInferenceGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const proceduralGeneration2dInferenceGuardReject = (at: string, why: string): never => {
  throw new proceduralGeneration2dInferenceGuardRefusal(at, why);
};

type proceduralGeneration2dInferenceGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type proceduralGeneration2dInferenceGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type proceduralGeneration2dInferenceGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const proceduralGeneration2dInferenceGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : proceduralGeneration2dInferenceGuardReject(at, "value is not an object");
export const proceduralGeneration2dInferenceGuardArray = (value: unknown, at: string, bounds: proceduralGeneration2dInferenceGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return proceduralGeneration2dInferenceGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) proceduralGeneration2dInferenceGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) proceduralGeneration2dInferenceGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const proceduralGeneration2dInferenceGuardString = (value: unknown, at: string, bounds: proceduralGeneration2dInferenceGuardTextBounds = {}): string => {
  if (typeof value !== "string") return proceduralGeneration2dInferenceGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) proceduralGeneration2dInferenceGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) proceduralGeneration2dInferenceGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) proceduralGeneration2dInferenceGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const proceduralGeneration2dInferenceGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : proceduralGeneration2dInferenceGuardReject(at, "value is not a boolean"));
export const proceduralGeneration2dInferenceGuardNumber = (value: unknown, at: string, bounds: proceduralGeneration2dInferenceGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return proceduralGeneration2dInferenceGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) proceduralGeneration2dInferenceGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) proceduralGeneration2dInferenceGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const proceduralGeneration2dInferenceGuardInteger = (value: unknown, at: string, bounds: proceduralGeneration2dInferenceGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? proceduralGeneration2dInferenceGuardNumber(value, at, bounds) : proceduralGeneration2dInferenceGuardReject(at, "value is not an integer");
export const proceduralGeneration2dInferenceGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : proceduralGeneration2dInferenceGuardReject(at, `value is not one of ${members.join(", ")}`);
export const proceduralGeneration2dInferenceGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : proceduralGeneration2dInferenceGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseGeneration2dInference(value: unknown, at = "$"): Generation2dInference {
  const row = proceduralGeneration2dInferenceGuardObject(value, at);
  return {
    topology: parseGeneration2dTopology(row["topology"], `${at}.topology`),
  };
}

export function parseGeneration2dTopology(value: unknown, at = "$"): Generation2dTopology {
  const row = proceduralGeneration2dInferenceGuardObject(value, at);
  return {
    nodeCount: proceduralGeneration2dInferenceGuardInteger(row["nodeCount"], `${at}.nodeCount`, {"minimum": 0}),
    edgeCount: proceduralGeneration2dInferenceGuardInteger(row["edgeCount"], `${at}.edgeCount`, {"minimum": 0}),
    topoOrder: proceduralGeneration2dInferenceGuardArray(row["topoOrder"], `${at}.topoOrder`).map((item, index) => proceduralGeneration2dInferenceGuardString(item, `${at}.topoOrder[${index}]`)),
    depth: proceduralGeneration2dInferenceGuardInteger(row["depth"], `${at}.depth`, {"minimum": 0}),
    cycleFree: proceduralGeneration2dInferenceGuardBoolean(row["cycleFree"], `${at}.cycleFree`),
  };
}
