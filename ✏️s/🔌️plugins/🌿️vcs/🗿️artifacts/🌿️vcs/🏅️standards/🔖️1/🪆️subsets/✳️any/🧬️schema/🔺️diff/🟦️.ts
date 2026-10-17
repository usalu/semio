/** 🧬️ VCS diff schema — sparse field delta over the artifact. */

export interface VcsDiff {
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
}

export interface VcsTagRemoval {
  id: string;
  index: number;
}

export interface VcsTagInsertion {
  index: number;
  row: string;
}

export interface VcsTagRelocation {
  id: string;
  from: number;
  to: number;
}

export interface VcsTagsDelta {
  removed: VcsTagRemoval[];
  inserted: VcsTagInsertion[];
  moved: VcsTagRelocation[];
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
export const vcsVcsDiffGuardClosed = (row: Readonly<Record<string, unknown>>, at: string, keys: readonly string[]): Readonly<Record<string, unknown>> => {
  for (const key of Object.keys(row)) if (!keys.includes(key)) vcsVcsDiffGuardReject(`${at}.${key}`, "unknown property");
  return row;
};
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
  const row = vcsVcsDiffGuardClosed(vcsVcsDiffGuardObject(value, at), at, ["schema", "title", "counter", "notes", "status", "tags"]);
  return {
    schema: row["schema"] === undefined ? undefined : vcsVcsDiffGuardString(row["schema"], `${at}.schema`),
    title: row["title"] === undefined ? undefined : vcsVcsDiffGuardString(row["title"], `${at}.title`),
    counter: row["counter"] === undefined ? undefined : vcsVcsDiffGuardInteger(row["counter"], `${at}.counter`),
    notes: row["notes"] === undefined ? undefined : vcsVcsDiffGuardString(row["notes"], `${at}.notes`),
    status: row["status"] === undefined ? undefined : vcsVcsDiffGuardString(row["status"], `${at}.status`),
    tags: row["tags"] === undefined ? undefined : parseVcsTagsDelta(row["tags"], `${at}.tags`),
  };
}

function vcsVcsDiffGuardCount(value: unknown, at: string): number {
  if (typeof value !== "number" || !Number.isInteger(value) || value < 0 || value > 4294967295) throw new vcsVcsDiffGuardRefusal(at, "uint32 required");
  return value;
}

export function parseVcsTagsDelta(value: unknown, at = "$"): VcsTagsDelta {
  const row = vcsVcsDiffGuardClosed(vcsVcsDiffGuardObject(value, at), at, ["removed", "inserted", "moved"]);
  return {
    removed: vcsVcsDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => parseVcsTagRemoval(item, `${at}.removed[${index}]`)),
    inserted: vcsVcsDiffGuardArray(row["inserted"], `${at}.inserted`).map((item, index) => parseVcsTagInsertion(item, `${at}.inserted[${index}]`)),
    moved: vcsVcsDiffGuardArray(row["moved"], `${at}.moved`).map((item, index) => parseVcsTagRelocation(item, `${at}.moved[${index}]`)),
  };
}

export function parseVcsTagInsertion(value: unknown, at = "$"): VcsTagInsertion {
  const row = vcsVcsDiffGuardClosed(vcsVcsDiffGuardObject(value, at), at, ["index", "row"]);
  return {
    index: vcsVcsDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0, "maximum": 4294967295}),
    row: vcsVcsDiffGuardString(row["row"], `${at}.row`),
  };
}

export function parseVcsTagRelocation(value: unknown, at = "$"): VcsTagRelocation {
  const row = vcsVcsDiffGuardClosed(vcsVcsDiffGuardObject(value, at), at, ["id", "from", "to"]);
  return {
    id: vcsVcsDiffGuardString(row["id"], `${at}.id`),
    from: vcsVcsDiffGuardInteger(row["from"], `${at}.from`, {"minimum": 0, "maximum": 4294967295}),
    to: vcsVcsDiffGuardInteger(row["to"], `${at}.to`, {"minimum": 0, "maximum": 4294967295}),
  };
}

export function parseVcsTagRemoval(value: unknown, at = "$"): VcsTagRemoval {
  const row = vcsVcsDiffGuardClosed(vcsVcsDiffGuardObject(value, at), at, ["id", "index"]);
  return {
    id: vcsVcsDiffGuardString(row["id"], `${at}.id`),
    index: vcsVcsDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0, "maximum": 4294967295}),
  };
}
