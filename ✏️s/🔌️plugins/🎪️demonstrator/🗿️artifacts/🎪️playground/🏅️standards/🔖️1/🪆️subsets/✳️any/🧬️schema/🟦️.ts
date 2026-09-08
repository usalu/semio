/** 🧬️ Playground artifact schema — every field with its state class. */

export interface PlaygroundArtifact {
  /** @state artifact */
  schema: string;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class demonstratorPlaygroundArtifactGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const demonstratorPlaygroundArtifactGuardReject = (at: string, why: string): never => {
  throw new demonstratorPlaygroundArtifactGuardRefusal(at, why);
};

type demonstratorPlaygroundArtifactGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type demonstratorPlaygroundArtifactGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type demonstratorPlaygroundArtifactGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const demonstratorPlaygroundArtifactGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : demonstratorPlaygroundArtifactGuardReject(at, "value is not an object");
export const demonstratorPlaygroundArtifactGuardArray = (value: unknown, at: string, bounds: demonstratorPlaygroundArtifactGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return demonstratorPlaygroundArtifactGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) demonstratorPlaygroundArtifactGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) demonstratorPlaygroundArtifactGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const demonstratorPlaygroundArtifactGuardString = (value: unknown, at: string, bounds: demonstratorPlaygroundArtifactGuardTextBounds = {}): string => {
  if (typeof value !== "string") return demonstratorPlaygroundArtifactGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) demonstratorPlaygroundArtifactGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) demonstratorPlaygroundArtifactGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) demonstratorPlaygroundArtifactGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const demonstratorPlaygroundArtifactGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : demonstratorPlaygroundArtifactGuardReject(at, "value is not a boolean"));
export const demonstratorPlaygroundArtifactGuardNumber = (value: unknown, at: string, bounds: demonstratorPlaygroundArtifactGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return demonstratorPlaygroundArtifactGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) demonstratorPlaygroundArtifactGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) demonstratorPlaygroundArtifactGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const demonstratorPlaygroundArtifactGuardInteger = (value: unknown, at: string, bounds: demonstratorPlaygroundArtifactGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? demonstratorPlaygroundArtifactGuardNumber(value, at, bounds) : demonstratorPlaygroundArtifactGuardReject(at, "value is not an integer");
export const demonstratorPlaygroundArtifactGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : demonstratorPlaygroundArtifactGuardReject(at, `value is not one of ${members.join(", ")}`);
export const demonstratorPlaygroundArtifactGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : demonstratorPlaygroundArtifactGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parsePlaygroundArtifact(value: unknown, at = "$"): PlaygroundArtifact {
  const row = demonstratorPlaygroundArtifactGuardObject(value, at);
  return {
    schema: demonstratorPlaygroundArtifactGuardString(row["schema"], `${at}.schema`),
  };
}
