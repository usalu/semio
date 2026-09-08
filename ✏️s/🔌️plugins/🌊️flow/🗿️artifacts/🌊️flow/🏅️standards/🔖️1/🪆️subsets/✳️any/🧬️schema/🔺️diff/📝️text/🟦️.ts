/** 📝️ Text representation for `flow.flow.diff`. */
export type FlowDiffText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class flowFlowDiffTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const flowFlowDiffTextGuardReject = (at: string, why: string): never => {
  throw new flowFlowDiffTextGuardRefusal(at, why);
};

type flowFlowDiffTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type flowFlowDiffTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type flowFlowDiffTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const flowFlowDiffTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : flowFlowDiffTextGuardReject(at, "value is not an object");
export const flowFlowDiffTextGuardArray = (value: unknown, at: string, bounds: flowFlowDiffTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return flowFlowDiffTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) flowFlowDiffTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) flowFlowDiffTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const flowFlowDiffTextGuardString = (value: unknown, at: string, bounds: flowFlowDiffTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return flowFlowDiffTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) flowFlowDiffTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) flowFlowDiffTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) flowFlowDiffTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const flowFlowDiffTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : flowFlowDiffTextGuardReject(at, "value is not a boolean"));
export const flowFlowDiffTextGuardNumber = (value: unknown, at: string, bounds: flowFlowDiffTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return flowFlowDiffTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) flowFlowDiffTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) flowFlowDiffTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const flowFlowDiffTextGuardInteger = (value: unknown, at: string, bounds: flowFlowDiffTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? flowFlowDiffTextGuardNumber(value, at, bounds) : flowFlowDiffTextGuardReject(at, "value is not an integer");
export const flowFlowDiffTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : flowFlowDiffTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const flowFlowDiffTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : flowFlowDiffTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseFlowDiffText(value: unknown, at = "$"): FlowDiffText {
  return flowFlowDiffTextGuardObject(value, `${at}`);
}
