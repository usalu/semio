/** 📝️ Text representation for `flow.flow.inference`. */
export type FlowInferenceText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class flowFlowInferenceTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const flowFlowInferenceTextGuardReject = (at: string, why: string): never => {
  throw new flowFlowInferenceTextGuardRefusal(at, why);
};

type flowFlowInferenceTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type flowFlowInferenceTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type flowFlowInferenceTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const flowFlowInferenceTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : flowFlowInferenceTextGuardReject(at, "value is not an object");
export const flowFlowInferenceTextGuardArray = (value: unknown, at: string, bounds: flowFlowInferenceTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return flowFlowInferenceTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) flowFlowInferenceTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) flowFlowInferenceTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const flowFlowInferenceTextGuardString = (value: unknown, at: string, bounds: flowFlowInferenceTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return flowFlowInferenceTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) flowFlowInferenceTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) flowFlowInferenceTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) flowFlowInferenceTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const flowFlowInferenceTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : flowFlowInferenceTextGuardReject(at, "value is not a boolean"));
export const flowFlowInferenceTextGuardNumber = (value: unknown, at: string, bounds: flowFlowInferenceTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return flowFlowInferenceTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) flowFlowInferenceTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) flowFlowInferenceTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const flowFlowInferenceTextGuardInteger = (value: unknown, at: string, bounds: flowFlowInferenceTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? flowFlowInferenceTextGuardNumber(value, at, bounds) : flowFlowInferenceTextGuardReject(at, "value is not an integer");
export const flowFlowInferenceTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : flowFlowInferenceTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const flowFlowInferenceTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : flowFlowInferenceTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseFlowInferenceText(value: unknown, at = "$"): FlowInferenceText {
  return flowFlowInferenceTextGuardObject(value, `${at}`);
}
