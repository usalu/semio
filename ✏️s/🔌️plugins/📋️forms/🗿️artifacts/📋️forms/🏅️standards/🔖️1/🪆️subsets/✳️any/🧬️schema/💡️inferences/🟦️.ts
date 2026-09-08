/** 💡️ Forms inference schema — topology (step/block dependency order). */

export interface FormsTopology {
  topoOrder: string[];
  depth: Record<string, number>;
  cycleFree: boolean;
  nodeCount: number;
}

export interface FormsInference {
  /** @derived */
  topology: FormsTopology;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class formsFormsInferenceGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const formsFormsInferenceGuardReject = (at: string, why: string): never => {
  throw new formsFormsInferenceGuardRefusal(at, why);
};

type formsFormsInferenceGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type formsFormsInferenceGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type formsFormsInferenceGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const formsFormsInferenceGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : formsFormsInferenceGuardReject(at, "value is not an object");
export const formsFormsInferenceGuardArray = (value: unknown, at: string, bounds: formsFormsInferenceGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return formsFormsInferenceGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) formsFormsInferenceGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) formsFormsInferenceGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const formsFormsInferenceGuardString = (value: unknown, at: string, bounds: formsFormsInferenceGuardTextBounds = {}): string => {
  if (typeof value !== "string") return formsFormsInferenceGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) formsFormsInferenceGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) formsFormsInferenceGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) formsFormsInferenceGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const formsFormsInferenceGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : formsFormsInferenceGuardReject(at, "value is not a boolean"));
export const formsFormsInferenceGuardNumber = (value: unknown, at: string, bounds: formsFormsInferenceGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return formsFormsInferenceGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) formsFormsInferenceGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) formsFormsInferenceGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const formsFormsInferenceGuardInteger = (value: unknown, at: string, bounds: formsFormsInferenceGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? formsFormsInferenceGuardNumber(value, at, bounds) : formsFormsInferenceGuardReject(at, "value is not an integer");
export const formsFormsInferenceGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : formsFormsInferenceGuardReject(at, `value is not one of ${members.join(", ")}`);
export const formsFormsInferenceGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : formsFormsInferenceGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseFormsInference(value: unknown, at = "$"): FormsInference {
  const row = formsFormsInferenceGuardObject(value, at);
  return {
    topology: parseFormsTopology(row["topology"], `${at}.topology`),
  };
}

export function parseFormsTopology(value: unknown, at = "$"): FormsTopology {
  const row = formsFormsInferenceGuardObject(value, at);
  return {
    topoOrder: formsFormsInferenceGuardArray(row["topoOrder"], `${at}.topoOrder`).map((item, index) => formsFormsInferenceGuardString(item, `${at}.topoOrder[${index}]`)),
    depth: formsFormsInferenceGuardObject(row["depth"], `${at}.depth`),
    cycleFree: formsFormsInferenceGuardBoolean(row["cycleFree"], `${at}.cycleFree`),
    nodeCount: formsFormsInferenceGuardInteger(row["nodeCount"], `${at}.nodeCount`),
  };
}
