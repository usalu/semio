/** 🧬️ Wires artifact schema — every field with its state class. */

export type DslValue = Record<string, unknown>;

export interface WiresArtifact {
  /** @state artifact */
  wiresFixture: DslValue;
  /** @state artifact */
  boardFixture: DslValue;
  /** @state artifact */
  dragNodeId?: string;
  /** @state artifact */
  dragLastX: number;
  /** @state artifact */
  dragLastY: number;
  /** @state config */
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class reasoningWiresArtifactGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const reasoningWiresArtifactGuardReject = (at: string, why: string): never => {
  throw new reasoningWiresArtifactGuardRefusal(at, why);
};

type reasoningWiresArtifactGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type reasoningWiresArtifactGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type reasoningWiresArtifactGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const reasoningWiresArtifactGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : reasoningWiresArtifactGuardReject(at, "value is not an object");
export const reasoningWiresArtifactGuardArray = (value: unknown, at: string, bounds: reasoningWiresArtifactGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return reasoningWiresArtifactGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) reasoningWiresArtifactGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) reasoningWiresArtifactGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const reasoningWiresArtifactGuardString = (value: unknown, at: string, bounds: reasoningWiresArtifactGuardTextBounds = {}): string => {
  if (typeof value !== "string") return reasoningWiresArtifactGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) reasoningWiresArtifactGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) reasoningWiresArtifactGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) reasoningWiresArtifactGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const reasoningWiresArtifactGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : reasoningWiresArtifactGuardReject(at, "value is not a boolean"));
export const reasoningWiresArtifactGuardNumber = (value: unknown, at: string, bounds: reasoningWiresArtifactGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return reasoningWiresArtifactGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) reasoningWiresArtifactGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) reasoningWiresArtifactGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const reasoningWiresArtifactGuardInteger = (value: unknown, at: string, bounds: reasoningWiresArtifactGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? reasoningWiresArtifactGuardNumber(value, at, bounds) : reasoningWiresArtifactGuardReject(at, "value is not an integer");
export const reasoningWiresArtifactGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : reasoningWiresArtifactGuardReject(at, `value is not one of ${members.join(", ")}`);
export const reasoningWiresArtifactGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : reasoningWiresArtifactGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseWiresArtifact(value: unknown, at = "$"): WiresArtifact {
  const row = reasoningWiresArtifactGuardObject(value, at);
  return {
    wiresFixture: reasoningWiresArtifactGuardObject(row["wiresFixture"], `${at}.wiresFixture`),
    boardFixture: reasoningWiresArtifactGuardObject(row["boardFixture"], `${at}.boardFixture`),
    dragNodeId: row["dragNodeId"] === undefined ? undefined : reasoningWiresArtifactGuardString(row["dragNodeId"], `${at}.dragNodeId`),
    dragLastX: reasoningWiresArtifactGuardNumber(row["dragLastX"], `${at}.dragLastX`),
    dragLastY: reasoningWiresArtifactGuardNumber(row["dragLastY"], `${at}.dragLastY`),
  };
}
