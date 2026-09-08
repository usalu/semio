/** 💡️ Playground inference schema — topology is always the vacuous empty graph today (no domain
 * entities exist yet on `PlaygroundSnapshot`). */

export interface PlaygroundTopology {
  topoOrder: string[];
  depth: Record<string, number>;
  cycleFree: boolean;
  nodeCount: number;
}

export interface PlaygroundInference {
  /** @derived */
  topology: PlaygroundTopology;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class demonstratorPlaygroundInferenceGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const demonstratorPlaygroundInferenceGuardReject = (at: string, why: string): never => {
  throw new demonstratorPlaygroundInferenceGuardRefusal(at, why);
};

type demonstratorPlaygroundInferenceGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type demonstratorPlaygroundInferenceGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type demonstratorPlaygroundInferenceGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const demonstratorPlaygroundInferenceGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : demonstratorPlaygroundInferenceGuardReject(at, "value is not an object");
export const demonstratorPlaygroundInferenceGuardArray = (value: unknown, at: string, bounds: demonstratorPlaygroundInferenceGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return demonstratorPlaygroundInferenceGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) demonstratorPlaygroundInferenceGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) demonstratorPlaygroundInferenceGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const demonstratorPlaygroundInferenceGuardString = (value: unknown, at: string, bounds: demonstratorPlaygroundInferenceGuardTextBounds = {}): string => {
  if (typeof value !== "string") return demonstratorPlaygroundInferenceGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) demonstratorPlaygroundInferenceGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) demonstratorPlaygroundInferenceGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) demonstratorPlaygroundInferenceGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const demonstratorPlaygroundInferenceGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : demonstratorPlaygroundInferenceGuardReject(at, "value is not a boolean"));
export const demonstratorPlaygroundInferenceGuardNumber = (value: unknown, at: string, bounds: demonstratorPlaygroundInferenceGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return demonstratorPlaygroundInferenceGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) demonstratorPlaygroundInferenceGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) demonstratorPlaygroundInferenceGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const demonstratorPlaygroundInferenceGuardInteger = (value: unknown, at: string, bounds: demonstratorPlaygroundInferenceGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? demonstratorPlaygroundInferenceGuardNumber(value, at, bounds) : demonstratorPlaygroundInferenceGuardReject(at, "value is not an integer");
export const demonstratorPlaygroundInferenceGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : demonstratorPlaygroundInferenceGuardReject(at, `value is not one of ${members.join(", ")}`);
export const demonstratorPlaygroundInferenceGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : demonstratorPlaygroundInferenceGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parsePlaygroundInference(value: unknown, at = "$"): PlaygroundInference {
  const row = demonstratorPlaygroundInferenceGuardObject(value, at);
  return {
    topology: parsePlaygroundTopology(row["topology"], `${at}.topology`),
  };
}

export function parsePlaygroundTopology(value: unknown, at = "$"): PlaygroundTopology {
  const row = demonstratorPlaygroundInferenceGuardObject(value, at);
  return {
    topoOrder: demonstratorPlaygroundInferenceGuardArray(row["topoOrder"], `${at}.topoOrder`).map((item, index) => demonstratorPlaygroundInferenceGuardString(item, `${at}.topoOrder[${index}]`)),
    depth: demonstratorPlaygroundInferenceGuardObject(row["depth"], `${at}.depth`),
    cycleFree: demonstratorPlaygroundInferenceGuardBoolean(row["cycleFree"], `${at}.cycleFree`),
    nodeCount: demonstratorPlaygroundInferenceGuardInteger(row["nodeCount"], `${at}.nodeCount`),
  };
}
