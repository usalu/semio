/** 💡️ Generation3d inference schema — topology (DAG shape of `fixture`'s widget/synapse graph). */

export interface Generation3dTopology {
  nodeCount: number;
  edgeCount: number;
  topoOrder: string[];
  depth: number;
  cycleFree: boolean;
}

export interface Generation3dInference {
  /** @derived */
  topology: Generation3dTopology;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class proceduralGeneration3dInferenceGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const proceduralGeneration3dInferenceGuardReject = (at: string, why: string): never => {
  throw new proceduralGeneration3dInferenceGuardRefusal(at, why);
};

type proceduralGeneration3dInferenceGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type proceduralGeneration3dInferenceGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type proceduralGeneration3dInferenceGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const proceduralGeneration3dInferenceGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : proceduralGeneration3dInferenceGuardReject(at, "value is not an object");
export const proceduralGeneration3dInferenceGuardArray = (value: unknown, at: string, bounds: proceduralGeneration3dInferenceGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return proceduralGeneration3dInferenceGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) proceduralGeneration3dInferenceGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) proceduralGeneration3dInferenceGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const proceduralGeneration3dInferenceGuardString = (value: unknown, at: string, bounds: proceduralGeneration3dInferenceGuardTextBounds = {}): string => {
  if (typeof value !== "string") return proceduralGeneration3dInferenceGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) proceduralGeneration3dInferenceGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) proceduralGeneration3dInferenceGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) proceduralGeneration3dInferenceGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const proceduralGeneration3dInferenceGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : proceduralGeneration3dInferenceGuardReject(at, "value is not a boolean"));
export const proceduralGeneration3dInferenceGuardNumber = (value: unknown, at: string, bounds: proceduralGeneration3dInferenceGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return proceduralGeneration3dInferenceGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) proceduralGeneration3dInferenceGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) proceduralGeneration3dInferenceGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const proceduralGeneration3dInferenceGuardInteger = (value: unknown, at: string, bounds: proceduralGeneration3dInferenceGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? proceduralGeneration3dInferenceGuardNumber(value, at, bounds) : proceduralGeneration3dInferenceGuardReject(at, "value is not an integer");
export const proceduralGeneration3dInferenceGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : proceduralGeneration3dInferenceGuardReject(at, `value is not one of ${members.join(", ")}`);
export const proceduralGeneration3dInferenceGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : proceduralGeneration3dInferenceGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseGeneration3dInference(value: unknown, at = "$"): Generation3dInference {
  const row = proceduralGeneration3dInferenceGuardObject(value, at);
  return {
    topology: parseGeneration3dTopology(row["topology"], `${at}.topology`),
  };
}

export function parseGeneration3dTopology(value: unknown, at = "$"): Generation3dTopology {
  const row = proceduralGeneration3dInferenceGuardObject(value, at);
  return {
    nodeCount: proceduralGeneration3dInferenceGuardInteger(row["nodeCount"], `${at}.nodeCount`, {"minimum": 0}),
    edgeCount: proceduralGeneration3dInferenceGuardInteger(row["edgeCount"], `${at}.edgeCount`, {"minimum": 0}),
    topoOrder: proceduralGeneration3dInferenceGuardArray(row["topoOrder"], `${at}.topoOrder`).map((item, index) => proceduralGeneration3dInferenceGuardString(item, `${at}.topoOrder[${index}]`)),
    depth: proceduralGeneration3dInferenceGuardInteger(row["depth"], `${at}.depth`, {"minimum": 0}),
    cycleFree: proceduralGeneration3dInferenceGuardBoolean(row["cycleFree"], `${at}.cycleFree`),
  };
}
