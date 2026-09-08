/** 💡️ Sequence inference schema — topology is a real Kahn's-algorithm topological sort over the
 * step DAG (steps + edges). */

export interface SequenceTopology {
  topoOrder: string[];
  depth: Record<string, number>;
  cycleFree: boolean;
  nodeCount: number;
}

export interface SequenceInference {
  /** @derived */
  topology: SequenceTopology;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class sequenceSequenceInferenceGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const sequenceSequenceInferenceGuardReject = (at: string, why: string): never => {
  throw new sequenceSequenceInferenceGuardRefusal(at, why);
};

type sequenceSequenceInferenceGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type sequenceSequenceInferenceGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type sequenceSequenceInferenceGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const sequenceSequenceInferenceGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : sequenceSequenceInferenceGuardReject(at, "value is not an object");
export const sequenceSequenceInferenceGuardArray = (value: unknown, at: string, bounds: sequenceSequenceInferenceGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return sequenceSequenceInferenceGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) sequenceSequenceInferenceGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) sequenceSequenceInferenceGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const sequenceSequenceInferenceGuardString = (value: unknown, at: string, bounds: sequenceSequenceInferenceGuardTextBounds = {}): string => {
  if (typeof value !== "string") return sequenceSequenceInferenceGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) sequenceSequenceInferenceGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) sequenceSequenceInferenceGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) sequenceSequenceInferenceGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const sequenceSequenceInferenceGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : sequenceSequenceInferenceGuardReject(at, "value is not a boolean"));
export const sequenceSequenceInferenceGuardNumber = (value: unknown, at: string, bounds: sequenceSequenceInferenceGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return sequenceSequenceInferenceGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) sequenceSequenceInferenceGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) sequenceSequenceInferenceGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const sequenceSequenceInferenceGuardInteger = (value: unknown, at: string, bounds: sequenceSequenceInferenceGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? sequenceSequenceInferenceGuardNumber(value, at, bounds) : sequenceSequenceInferenceGuardReject(at, "value is not an integer");
export const sequenceSequenceInferenceGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : sequenceSequenceInferenceGuardReject(at, `value is not one of ${members.join(", ")}`);
export const sequenceSequenceInferenceGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : sequenceSequenceInferenceGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseSequenceInference(value: unknown, at = "$"): SequenceInference {
  const row = sequenceSequenceInferenceGuardObject(value, at);
  return {
    topology: parseSequenceTopology(row["topology"], `${at}.topology`),
  };
}

export function parseSequenceTopology(value: unknown, at = "$"): SequenceTopology {
  const row = sequenceSequenceInferenceGuardObject(value, at);
  return {
    topoOrder: sequenceSequenceInferenceGuardArray(row["topoOrder"], `${at}.topoOrder`).map((item, index) => sequenceSequenceInferenceGuardString(item, `${at}.topoOrder[${index}]`)),
    depth: sequenceSequenceInferenceGuardObject(row["depth"], `${at}.depth`),
    cycleFree: sequenceSequenceInferenceGuardBoolean(row["cycleFree"], `${at}.cycleFree`),
    nodeCount: sequenceSequenceInferenceGuardInteger(row["nodeCount"], `${at}.nodeCount`),
  };
}
