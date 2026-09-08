/** 💡️ Equation inference schema — topology (graph topological order). */

export interface EquationTopology {
  topoOrder: string[];
  depth: Record<string, number>;
  cycleFree: boolean;
  nodeCount: number;
}

export interface EquationInference {
  /** @derived */
  topology: EquationTopology;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class equationEquationInferenceGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const equationEquationInferenceGuardReject = (at: string, why: string): never => {
  throw new equationEquationInferenceGuardRefusal(at, why);
};

type equationEquationInferenceGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type equationEquationInferenceGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type equationEquationInferenceGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const equationEquationInferenceGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : equationEquationInferenceGuardReject(at, "value is not an object");
export const equationEquationInferenceGuardArray = (value: unknown, at: string, bounds: equationEquationInferenceGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return equationEquationInferenceGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) equationEquationInferenceGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) equationEquationInferenceGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const equationEquationInferenceGuardString = (value: unknown, at: string, bounds: equationEquationInferenceGuardTextBounds = {}): string => {
  if (typeof value !== "string") return equationEquationInferenceGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) equationEquationInferenceGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) equationEquationInferenceGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) equationEquationInferenceGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const equationEquationInferenceGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : equationEquationInferenceGuardReject(at, "value is not a boolean"));
export const equationEquationInferenceGuardNumber = (value: unknown, at: string, bounds: equationEquationInferenceGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return equationEquationInferenceGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) equationEquationInferenceGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) equationEquationInferenceGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const equationEquationInferenceGuardInteger = (value: unknown, at: string, bounds: equationEquationInferenceGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? equationEquationInferenceGuardNumber(value, at, bounds) : equationEquationInferenceGuardReject(at, "value is not an integer");
export const equationEquationInferenceGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : equationEquationInferenceGuardReject(at, `value is not one of ${members.join(", ")}`);
export const equationEquationInferenceGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : equationEquationInferenceGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseEquationInference(value: unknown, at = "$"): EquationInference {
  const row = equationEquationInferenceGuardObject(value, at);
  return {
    topology: parseEquationTopology(row["topology"], `${at}.topology`),
  };
}

export function parseEquationTopology(value: unknown, at = "$"): EquationTopology {
  const row = equationEquationInferenceGuardObject(value, at);
  return {
    topoOrder: equationEquationInferenceGuardArray(row["topoOrder"], `${at}.topoOrder`).map((item, index) => equationEquationInferenceGuardString(item, `${at}.topoOrder[${index}]`)),
    depth: equationEquationInferenceGuardObject(row["depth"], `${at}.depth`),
    cycleFree: equationEquationInferenceGuardBoolean(row["cycleFree"], `${at}.cycleFree`),
    nodeCount: equationEquationInferenceGuardInteger(row["nodeCount"], `${at}.nodeCount`),
  };
}
