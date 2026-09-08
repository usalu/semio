/** 💡️ Layout inference schema — topology (parent-page/spread/page composition order). */

export interface LayoutTopology {
  topoOrder: string[];
  depth: Record<string, number>;
  cycleFree: boolean;
  nodeCount: number;
}

export interface LayoutInference {
  /** @derived */
  topology: LayoutTopology;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class layoutLayoutInferenceGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const layoutLayoutInferenceGuardReject = (at: string, why: string): never => {
  throw new layoutLayoutInferenceGuardRefusal(at, why);
};

type layoutLayoutInferenceGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type layoutLayoutInferenceGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type layoutLayoutInferenceGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const layoutLayoutInferenceGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : layoutLayoutInferenceGuardReject(at, "value is not an object");
export const layoutLayoutInferenceGuardArray = (value: unknown, at: string, bounds: layoutLayoutInferenceGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return layoutLayoutInferenceGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) layoutLayoutInferenceGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) layoutLayoutInferenceGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const layoutLayoutInferenceGuardString = (value: unknown, at: string, bounds: layoutLayoutInferenceGuardTextBounds = {}): string => {
  if (typeof value !== "string") return layoutLayoutInferenceGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) layoutLayoutInferenceGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) layoutLayoutInferenceGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) layoutLayoutInferenceGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const layoutLayoutInferenceGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : layoutLayoutInferenceGuardReject(at, "value is not a boolean"));
export const layoutLayoutInferenceGuardNumber = (value: unknown, at: string, bounds: layoutLayoutInferenceGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return layoutLayoutInferenceGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) layoutLayoutInferenceGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) layoutLayoutInferenceGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const layoutLayoutInferenceGuardInteger = (value: unknown, at: string, bounds: layoutLayoutInferenceGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? layoutLayoutInferenceGuardNumber(value, at, bounds) : layoutLayoutInferenceGuardReject(at, "value is not an integer");
export const layoutLayoutInferenceGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : layoutLayoutInferenceGuardReject(at, `value is not one of ${members.join(", ")}`);
export const layoutLayoutInferenceGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : layoutLayoutInferenceGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseLayoutInference(value: unknown, at = "$"): LayoutInference {
  const row = layoutLayoutInferenceGuardObject(value, at);
  return {
    topology: parseLayoutTopology(row["topology"], `${at}.topology`),
  };
}

export function parseLayoutTopology(value: unknown, at = "$"): LayoutTopology {
  const row = layoutLayoutInferenceGuardObject(value, at);
  return {
    topoOrder: layoutLayoutInferenceGuardArray(row["topoOrder"], `${at}.topoOrder`).map((item, index) => layoutLayoutInferenceGuardString(item, `${at}.topoOrder[${index}]`)),
    depth: layoutLayoutInferenceGuardObject(row["depth"], `${at}.depth`),
    cycleFree: layoutLayoutInferenceGuardBoolean(row["cycleFree"], `${at}.cycleFree`),
    nodeCount: layoutLayoutInferenceGuardInteger(row["nodeCount"], `${at}.nodeCount`),
  };
}
