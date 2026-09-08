/** 💡️ Flow inference schema — topology (topological order + depth + cycle-freedom) over widgets/synapses. */

export interface FlowTopology {
  topoOrder: string[];
  depth: Record<string, number>;
  cycleFree: boolean;
  nodeCount: number;
}

export interface FlowInference {
  /** @derived */
  topology: FlowTopology;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class flowFlowInferenceGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const flowFlowInferenceGuardReject = (at: string, why: string): never => {
  throw new flowFlowInferenceGuardRefusal(at, why);
};

type flowFlowInferenceGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type flowFlowInferenceGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type flowFlowInferenceGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const flowFlowInferenceGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : flowFlowInferenceGuardReject(at, "value is not an object");
export const flowFlowInferenceGuardArray = (value: unknown, at: string, bounds: flowFlowInferenceGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return flowFlowInferenceGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) flowFlowInferenceGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) flowFlowInferenceGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const flowFlowInferenceGuardString = (value: unknown, at: string, bounds: flowFlowInferenceGuardTextBounds = {}): string => {
  if (typeof value !== "string") return flowFlowInferenceGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) flowFlowInferenceGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) flowFlowInferenceGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) flowFlowInferenceGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const flowFlowInferenceGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : flowFlowInferenceGuardReject(at, "value is not a boolean"));
export const flowFlowInferenceGuardNumber = (value: unknown, at: string, bounds: flowFlowInferenceGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return flowFlowInferenceGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) flowFlowInferenceGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) flowFlowInferenceGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const flowFlowInferenceGuardInteger = (value: unknown, at: string, bounds: flowFlowInferenceGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? flowFlowInferenceGuardNumber(value, at, bounds) : flowFlowInferenceGuardReject(at, "value is not an integer");
export const flowFlowInferenceGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : flowFlowInferenceGuardReject(at, `value is not one of ${members.join(", ")}`);
export const flowFlowInferenceGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : flowFlowInferenceGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseFlowInference(value: unknown, at = "$"): FlowInference {
  const row = flowFlowInferenceGuardObject(value, at);
  return {
    topology: parseFlowTopology(row["topology"], `${at}.topology`),
  };
}

export function parseFlowTopology(value: unknown, at = "$"): FlowTopology {
  const row = flowFlowInferenceGuardObject(value, at);
  return {
    topoOrder: flowFlowInferenceGuardArray(row["topoOrder"], `${at}.topoOrder`).map((item, index) => flowFlowInferenceGuardString(item, `${at}.topoOrder[${index}]`)),
    depth: flowFlowInferenceGuardObject(row["depth"], `${at}.depth`),
    cycleFree: flowFlowInferenceGuardBoolean(row["cycleFree"], `${at}.cycleFree`),
    nodeCount: flowFlowInferenceGuardInteger(row["nodeCount"], `${at}.nodeCount`, {"minimum": 0}),
  };
}
