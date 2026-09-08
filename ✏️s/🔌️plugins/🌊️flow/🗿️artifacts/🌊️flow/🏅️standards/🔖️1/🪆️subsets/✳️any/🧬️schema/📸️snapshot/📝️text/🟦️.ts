/** 📝️ Text representation for `flow.flow.snapshot`. */
export type FlowSnapshotText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class flowFlowSnapshotTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const flowFlowSnapshotTextGuardReject = (at: string, why: string): never => {
  throw new flowFlowSnapshotTextGuardRefusal(at, why);
};

type flowFlowSnapshotTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type flowFlowSnapshotTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type flowFlowSnapshotTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const flowFlowSnapshotTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : flowFlowSnapshotTextGuardReject(at, "value is not an object");
export const flowFlowSnapshotTextGuardArray = (value: unknown, at: string, bounds: flowFlowSnapshotTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return flowFlowSnapshotTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) flowFlowSnapshotTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) flowFlowSnapshotTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const flowFlowSnapshotTextGuardString = (value: unknown, at: string, bounds: flowFlowSnapshotTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return flowFlowSnapshotTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) flowFlowSnapshotTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) flowFlowSnapshotTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) flowFlowSnapshotTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const flowFlowSnapshotTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : flowFlowSnapshotTextGuardReject(at, "value is not a boolean"));
export const flowFlowSnapshotTextGuardNumber = (value: unknown, at: string, bounds: flowFlowSnapshotTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return flowFlowSnapshotTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) flowFlowSnapshotTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) flowFlowSnapshotTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const flowFlowSnapshotTextGuardInteger = (value: unknown, at: string, bounds: flowFlowSnapshotTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? flowFlowSnapshotTextGuardNumber(value, at, bounds) : flowFlowSnapshotTextGuardReject(at, "value is not an integer");
export const flowFlowSnapshotTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : flowFlowSnapshotTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const flowFlowSnapshotTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : flowFlowSnapshotTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseFlowSnapshotText(value: unknown, at = "$"): FlowSnapshotText {
  return flowFlowSnapshotTextGuardObject(value, `${at}`);
}
