/** 🧬️ VCS artifact schema — every field with its state class. */

export interface VcsArtifact {
  /** @state artifact */
  schema: string;
  /** @state artifact */
  title: string;
  /** @state artifact */
  counter: number;
  /** @state artifact */
  notes: string;
  /** @state artifact */
  status: string;
  /** @state artifact */
  tags: string[];
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class vcsVcsArtifactGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const vcsVcsArtifactGuardReject = (at: string, why: string): never => {
  throw new vcsVcsArtifactGuardRefusal(at, why);
};

type vcsVcsArtifactGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type vcsVcsArtifactGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type vcsVcsArtifactGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const vcsVcsArtifactGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : vcsVcsArtifactGuardReject(at, "value is not an object");
export const vcsVcsArtifactGuardArray = (value: unknown, at: string, bounds: vcsVcsArtifactGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return vcsVcsArtifactGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) vcsVcsArtifactGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) vcsVcsArtifactGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const vcsVcsArtifactGuardString = (value: unknown, at: string, bounds: vcsVcsArtifactGuardTextBounds = {}): string => {
  if (typeof value !== "string") return vcsVcsArtifactGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) vcsVcsArtifactGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) vcsVcsArtifactGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) vcsVcsArtifactGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const vcsVcsArtifactGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : vcsVcsArtifactGuardReject(at, "value is not a boolean"));
export const vcsVcsArtifactGuardNumber = (value: unknown, at: string, bounds: vcsVcsArtifactGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return vcsVcsArtifactGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) vcsVcsArtifactGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) vcsVcsArtifactGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const vcsVcsArtifactGuardInteger = (value: unknown, at: string, bounds: vcsVcsArtifactGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? vcsVcsArtifactGuardNumber(value, at, bounds) : vcsVcsArtifactGuardReject(at, "value is not an integer");
export const vcsVcsArtifactGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : vcsVcsArtifactGuardReject(at, `value is not one of ${members.join(", ")}`);
export const vcsVcsArtifactGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : vcsVcsArtifactGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseVcsArtifact(value: unknown, at = "$"): VcsArtifact {
  const row = vcsVcsArtifactGuardObject(value, at);
  return {
    schema: vcsVcsArtifactGuardString(row["schema"], `${at}.schema`),
    title: vcsVcsArtifactGuardString(row["title"], `${at}.title`),
    counter: vcsVcsArtifactGuardInteger(row["counter"], `${at}.counter`),
    notes: vcsVcsArtifactGuardString(row["notes"], `${at}.notes`),
    status: vcsVcsArtifactGuardString(row["status"], `${at}.status`),
    tags: vcsVcsArtifactGuardArray(row["tags"], `${at}.tags`).map((item, index) => vcsVcsArtifactGuardString(item, `${at}.tags[${index}]`)),
  };
}
