/** 🧬️ VCS snapshot schema — artifact-lane fields only. */

export interface VcsSnapshot {
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
export class vcsVcsSnapshotGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const vcsVcsSnapshotGuardReject = (at: string, why: string): never => {
  throw new vcsVcsSnapshotGuardRefusal(at, why);
};

type vcsVcsSnapshotGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type vcsVcsSnapshotGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type vcsVcsSnapshotGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const vcsVcsSnapshotGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : vcsVcsSnapshotGuardReject(at, "value is not an object");
export const vcsVcsSnapshotGuardArray = (value: unknown, at: string, bounds: vcsVcsSnapshotGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return vcsVcsSnapshotGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) vcsVcsSnapshotGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) vcsVcsSnapshotGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const vcsVcsSnapshotGuardString = (value: unknown, at: string, bounds: vcsVcsSnapshotGuardTextBounds = {}): string => {
  if (typeof value !== "string") return vcsVcsSnapshotGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) vcsVcsSnapshotGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) vcsVcsSnapshotGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) vcsVcsSnapshotGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const vcsVcsSnapshotGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : vcsVcsSnapshotGuardReject(at, "value is not a boolean"));
export const vcsVcsSnapshotGuardNumber = (value: unknown, at: string, bounds: vcsVcsSnapshotGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return vcsVcsSnapshotGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) vcsVcsSnapshotGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) vcsVcsSnapshotGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const vcsVcsSnapshotGuardInteger = (value: unknown, at: string, bounds: vcsVcsSnapshotGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? vcsVcsSnapshotGuardNumber(value, at, bounds) : vcsVcsSnapshotGuardReject(at, "value is not an integer");
export const vcsVcsSnapshotGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : vcsVcsSnapshotGuardReject(at, `value is not one of ${members.join(", ")}`);
export const vcsVcsSnapshotGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : vcsVcsSnapshotGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseVcsSnapshot(value: unknown, at = "$"): VcsSnapshot {
  const row = vcsVcsSnapshotGuardObject(value, at);
  return {
    schema: vcsVcsSnapshotGuardString(row["schema"], `${at}.schema`),
    title: vcsVcsSnapshotGuardString(row["title"], `${at}.title`),
    counter: vcsVcsSnapshotGuardInteger(row["counter"], `${at}.counter`),
    notes: vcsVcsSnapshotGuardString(row["notes"], `${at}.notes`),
    status: vcsVcsSnapshotGuardString(row["status"], `${at}.status`),
    tags: vcsVcsSnapshotGuardArray(row["tags"], `${at}.tags`).map((item, index) => vcsVcsSnapshotGuardString(item, `${at}.tags[${index}]`)),
  };
}
