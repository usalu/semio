/** 🧬️ Playground diff schema — sparse field delta over the artifact. */

export interface PlaygroundDiff {
  /** @state artifact */
  artifact?: PlaygroundArtifact;
  /** @state artifact */
  schema?: string;
}

export interface PlaygroundArtifact {
  schema: string;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class demonstratorPlaygroundDiffGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const demonstratorPlaygroundDiffGuardReject = (at: string, why: string): never => {
  throw new demonstratorPlaygroundDiffGuardRefusal(at, why);
};

type demonstratorPlaygroundDiffGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type demonstratorPlaygroundDiffGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type demonstratorPlaygroundDiffGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const demonstratorPlaygroundDiffGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : demonstratorPlaygroundDiffGuardReject(at, "value is not an object");
export const demonstratorPlaygroundDiffGuardArray = (value: unknown, at: string, bounds: demonstratorPlaygroundDiffGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return demonstratorPlaygroundDiffGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) demonstratorPlaygroundDiffGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) demonstratorPlaygroundDiffGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const demonstratorPlaygroundDiffGuardString = (value: unknown, at: string, bounds: demonstratorPlaygroundDiffGuardTextBounds = {}): string => {
  if (typeof value !== "string") return demonstratorPlaygroundDiffGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) demonstratorPlaygroundDiffGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) demonstratorPlaygroundDiffGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) demonstratorPlaygroundDiffGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const demonstratorPlaygroundDiffGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : demonstratorPlaygroundDiffGuardReject(at, "value is not a boolean"));
export const demonstratorPlaygroundDiffGuardNumber = (value: unknown, at: string, bounds: demonstratorPlaygroundDiffGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return demonstratorPlaygroundDiffGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) demonstratorPlaygroundDiffGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) demonstratorPlaygroundDiffGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const demonstratorPlaygroundDiffGuardInteger = (value: unknown, at: string, bounds: demonstratorPlaygroundDiffGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? demonstratorPlaygroundDiffGuardNumber(value, at, bounds) : demonstratorPlaygroundDiffGuardReject(at, "value is not an integer");
export const demonstratorPlaygroundDiffGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : demonstratorPlaygroundDiffGuardReject(at, `value is not one of ${members.join(", ")}`);
export const demonstratorPlaygroundDiffGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : demonstratorPlaygroundDiffGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parsePlaygroundDiff(value: unknown, at = "$"): PlaygroundDiff {
  const row = demonstratorPlaygroundDiffGuardObject(value, at);
  return {
    artifact: row["artifact"] === undefined ? undefined : demonstratorPlaygroundDiffGuardObject(row["artifact"], `${at}.artifact`),
    schema: row["schema"] === undefined ? undefined : demonstratorPlaygroundDiffGuardString(row["schema"], `${at}.schema`),
  };
}
