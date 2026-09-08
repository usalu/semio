/** 🧬️ VCS diff schema — sparse field delta over the artifact. */

export interface VcsDiff {
  /** @state artifact */
  artifact?: VcsArtifact;
  /** @state artifact */
  schema?: string;
  /** @state artifact */
  title?: string;
  /** @state artifact */
  counter?: number;
  /** @state artifact */
  notes?: string;
  /** @state artifact */
  status?: string;
  /** @state artifact */
  tags?: VcsTagsDelta;
  /** @state presence */
  selectedCheckpointIds?: VcsStringList;
  /** @state config */
}

export interface VcsStringList {
  values: string[];
}

export interface VcsTagsDelta {
  added: string[];
  removed: string[];
}

export interface VcsArtifact {
  schema: string;
  title: string;
  counter: number;
  notes: string;
  status: string;
  tags: string[];
  selectedCheckpointIds: string[];
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class vcsVcsDiffGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const vcsVcsDiffGuardReject = (at: string, why: string): never => {
  throw new vcsVcsDiffGuardRefusal(at, why);
};

type vcsVcsDiffGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type vcsVcsDiffGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type vcsVcsDiffGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const vcsVcsDiffGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : vcsVcsDiffGuardReject(at, "value is not an object");
export const vcsVcsDiffGuardArray = (value: unknown, at: string, bounds: vcsVcsDiffGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return vcsVcsDiffGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) vcsVcsDiffGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) vcsVcsDiffGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const vcsVcsDiffGuardString = (value: unknown, at: string, bounds: vcsVcsDiffGuardTextBounds = {}): string => {
  if (typeof value !== "string") return vcsVcsDiffGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) vcsVcsDiffGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) vcsVcsDiffGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) vcsVcsDiffGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const vcsVcsDiffGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : vcsVcsDiffGuardReject(at, "value is not a boolean"));
export const vcsVcsDiffGuardNumber = (value: unknown, at: string, bounds: vcsVcsDiffGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return vcsVcsDiffGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) vcsVcsDiffGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) vcsVcsDiffGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const vcsVcsDiffGuardInteger = (value: unknown, at: string, bounds: vcsVcsDiffGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? vcsVcsDiffGuardNumber(value, at, bounds) : vcsVcsDiffGuardReject(at, "value is not an integer");
export const vcsVcsDiffGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : vcsVcsDiffGuardReject(at, `value is not one of ${members.join(", ")}`);
export const vcsVcsDiffGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : vcsVcsDiffGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseVcsDiff(value: unknown, at = "$"): VcsDiff {
  const row = vcsVcsDiffGuardObject(value, at);
  return {
    artifact: row["artifact"] === undefined ? undefined : vcsVcsDiffGuardObject(row["artifact"], `${at}.artifact`),
    schema: row["schema"] === undefined ? undefined : vcsVcsDiffGuardString(row["schema"], `${at}.schema`),
    title: row["title"] === undefined ? undefined : vcsVcsDiffGuardString(row["title"], `${at}.title`),
    counter: row["counter"] === undefined ? undefined : vcsVcsDiffGuardInteger(row["counter"], `${at}.counter`),
    notes: row["notes"] === undefined ? undefined : vcsVcsDiffGuardString(row["notes"], `${at}.notes`),
    status: row["status"] === undefined ? undefined : vcsVcsDiffGuardString(row["status"], `${at}.status`),
    tags: row["tags"] === undefined ? undefined : parseVcsTagsDelta(row["tags"], `${at}.tags`),
    selectedCheckpointIds: row["selectedCheckpointIds"] === undefined ? undefined : parseVcsStringList(row["selectedCheckpointIds"], `${at}.selectedCheckpointIds`),
  };
}

export function parseVcsStringList(value: unknown, at = "$"): VcsStringList {
  const row = vcsVcsDiffGuardObject(value, at);
  return {
    values: vcsVcsDiffGuardArray(row["values"], `${at}.values`).map((item, index) => vcsVcsDiffGuardString(item, `${at}.values[${index}]`)),
  };
}

export function parseVcsTagsDelta(value: unknown, at = "$"): VcsTagsDelta {
  const row = vcsVcsDiffGuardObject(value, at);
  return {
    added: vcsVcsDiffGuardArray(row["added"], `${at}.added`).map((item, index) => vcsVcsDiffGuardString(item, `${at}.added[${index}]`)),
    removed: vcsVcsDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => vcsVcsDiffGuardString(item, `${at}.removed[${index}]`)),
  };
}
