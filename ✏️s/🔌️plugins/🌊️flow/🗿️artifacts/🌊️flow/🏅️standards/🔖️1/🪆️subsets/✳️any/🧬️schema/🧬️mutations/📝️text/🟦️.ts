/** 📝️ Text representation for `flow.flow.mutations`. */
export type FlowMutationsText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class flowFlowMutationsTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const flowFlowMutationsTextGuardReject = (at: string, why: string): never => {
  throw new flowFlowMutationsTextGuardRefusal(at, why);
};

type flowFlowMutationsTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type flowFlowMutationsTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type flowFlowMutationsTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const flowFlowMutationsTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : flowFlowMutationsTextGuardReject(at, "value is not an object");
export const flowFlowMutationsTextGuardArray = (value: unknown, at: string, bounds: flowFlowMutationsTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return flowFlowMutationsTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) flowFlowMutationsTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) flowFlowMutationsTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const flowFlowMutationsTextGuardString = (value: unknown, at: string, bounds: flowFlowMutationsTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return flowFlowMutationsTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) flowFlowMutationsTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) flowFlowMutationsTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) flowFlowMutationsTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const flowFlowMutationsTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : flowFlowMutationsTextGuardReject(at, "value is not a boolean"));
export const flowFlowMutationsTextGuardNumber = (value: unknown, at: string, bounds: flowFlowMutationsTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return flowFlowMutationsTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) flowFlowMutationsTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) flowFlowMutationsTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const flowFlowMutationsTextGuardInteger = (value: unknown, at: string, bounds: flowFlowMutationsTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? flowFlowMutationsTextGuardNumber(value, at, bounds) : flowFlowMutationsTextGuardReject(at, "value is not an integer");
export const flowFlowMutationsTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : flowFlowMutationsTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const flowFlowMutationsTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : flowFlowMutationsTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseFlowMutationsText(value: unknown, at = "$"): FlowMutationsText {
  return flowFlowMutationsTextGuardObject(value, `${at}`);
}
