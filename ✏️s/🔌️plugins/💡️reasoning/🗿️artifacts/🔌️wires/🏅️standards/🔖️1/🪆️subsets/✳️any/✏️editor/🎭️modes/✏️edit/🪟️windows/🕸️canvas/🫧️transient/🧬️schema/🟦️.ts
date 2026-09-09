/** 🧬️ WiresCanvasTransient */
export interface WiresCanvasTransient {
  /** @state transient */
  dragNodeId?: string;
  /** @state transient */
  dragStartX: number;
  /** @state transient */
  dragStartY: number;
  /** @state transient */
  dragLastX: number;
  /** @state transient */
  dragLastY: number;
  /** @state transient */
  dragZoom: number;
  /** @state transient */
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class reasoningWiresCanvasTransientGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const reasoningWiresCanvasTransientGuardReject = (at: string, why: string): never => {
  throw new reasoningWiresCanvasTransientGuardRefusal(at, why);
};

type reasoningWiresCanvasTransientGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type reasoningWiresCanvasTransientGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type reasoningWiresCanvasTransientGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const reasoningWiresCanvasTransientGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : reasoningWiresCanvasTransientGuardReject(at, "value is not an object");
export const reasoningWiresCanvasTransientGuardArray = (value: unknown, at: string, bounds: reasoningWiresCanvasTransientGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return reasoningWiresCanvasTransientGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) reasoningWiresCanvasTransientGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) reasoningWiresCanvasTransientGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const reasoningWiresCanvasTransientGuardString = (value: unknown, at: string, bounds: reasoningWiresCanvasTransientGuardTextBounds = {}): string => {
  if (typeof value !== "string") return reasoningWiresCanvasTransientGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) reasoningWiresCanvasTransientGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) reasoningWiresCanvasTransientGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) reasoningWiresCanvasTransientGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const reasoningWiresCanvasTransientGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : reasoningWiresCanvasTransientGuardReject(at, "value is not a boolean"));
export const reasoningWiresCanvasTransientGuardNumber = (value: unknown, at: string, bounds: reasoningWiresCanvasTransientGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return reasoningWiresCanvasTransientGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) reasoningWiresCanvasTransientGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) reasoningWiresCanvasTransientGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const reasoningWiresCanvasTransientGuardInteger = (value: unknown, at: string, bounds: reasoningWiresCanvasTransientGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? reasoningWiresCanvasTransientGuardNumber(value, at, bounds) : reasoningWiresCanvasTransientGuardReject(at, "value is not an integer");
export const reasoningWiresCanvasTransientGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : reasoningWiresCanvasTransientGuardReject(at, `value is not one of ${members.join(", ")}`);
export const reasoningWiresCanvasTransientGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : reasoningWiresCanvasTransientGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseWiresCanvasTransient(value: unknown, at = "$"): WiresCanvasTransient {
  const row = reasoningWiresCanvasTransientGuardObject(value, at);
  return {
    dragNodeId: row["dragNodeId"] === undefined ? undefined : reasoningWiresCanvasTransientGuardString(row["dragNodeId"], `${at}.dragNodeId`),
    dragStartX: reasoningWiresCanvasTransientGuardNumber(row["dragStartX"], `${at}.dragStartX`),
    dragStartY: reasoningWiresCanvasTransientGuardNumber(row["dragStartY"], `${at}.dragStartY`),
    dragLastX: reasoningWiresCanvasTransientGuardNumber(row["dragLastX"], `${at}.dragLastX`),
    dragLastY: reasoningWiresCanvasTransientGuardNumber(row["dragLastY"], `${at}.dragLastY`),
    dragZoom: reasoningWiresCanvasTransientGuardNumber(row["dragZoom"], `${at}.dragZoom`, { minimum: Number.MIN_VALUE }),
  };
}
