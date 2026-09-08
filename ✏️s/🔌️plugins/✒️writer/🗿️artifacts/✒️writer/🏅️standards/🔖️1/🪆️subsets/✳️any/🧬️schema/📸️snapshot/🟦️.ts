/** 🧬️ Writer snapshot schema. */

export interface WriterSnapshot {
  /** @state artifact */
  schema: string;
  /** @state artifact */
  id: string;
  /** @state artifact */
  languageId: string;
  /** @state artifact */
  uri: string;
  /** @state artifact */
  text: string;
}

export interface WriterEditorSelection {
  start: number;
  end: number;
}

export interface WriterEditorSettings {
  showLineNumbers: boolean;
  fontPx: number;
  lineHeight: number;
  tabSize: number;
}

export interface WriterStringList {
  values: string[];
}

export interface WriterTextRangeEdit {
  start: number;
  end: number;
  insert: string;
}

export interface WriterTextDelta {
  replacement?: string;
  edits: WriterTextRangeEdit[];
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class writerWriterSnapshotGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const writerWriterSnapshotGuardReject = (at: string, why: string): never => {
  throw new writerWriterSnapshotGuardRefusal(at, why);
};

type writerWriterSnapshotGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type writerWriterSnapshotGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type writerWriterSnapshotGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const writerWriterSnapshotGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : writerWriterSnapshotGuardReject(at, "value is not an object");
export const writerWriterSnapshotGuardArray = (value: unknown, at: string, bounds: writerWriterSnapshotGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return writerWriterSnapshotGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) writerWriterSnapshotGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) writerWriterSnapshotGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const writerWriterSnapshotGuardString = (value: unknown, at: string, bounds: writerWriterSnapshotGuardTextBounds = {}): string => {
  if (typeof value !== "string") return writerWriterSnapshotGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) writerWriterSnapshotGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) writerWriterSnapshotGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) writerWriterSnapshotGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const writerWriterSnapshotGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : writerWriterSnapshotGuardReject(at, "value is not a boolean"));
export const writerWriterSnapshotGuardNumber = (value: unknown, at: string, bounds: writerWriterSnapshotGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return writerWriterSnapshotGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) writerWriterSnapshotGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) writerWriterSnapshotGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const writerWriterSnapshotGuardInteger = (value: unknown, at: string, bounds: writerWriterSnapshotGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? writerWriterSnapshotGuardNumber(value, at, bounds) : writerWriterSnapshotGuardReject(at, "value is not an integer");
export const writerWriterSnapshotGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : writerWriterSnapshotGuardReject(at, `value is not one of ${members.join(", ")}`);
export const writerWriterSnapshotGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : writerWriterSnapshotGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseWriterSnapshot(value: unknown, at = "$"): WriterSnapshot {
  const row = writerWriterSnapshotGuardObject(value, at);
  return {
    schema: writerWriterSnapshotGuardString(row["schema"], `${at}.schema`),
    id: writerWriterSnapshotGuardString(row["id"], `${at}.id`),
    languageId: writerWriterSnapshotGuardString(row["languageId"], `${at}.languageId`),
    uri: writerWriterSnapshotGuardString(row["uri"], `${at}.uri`),
    text: writerWriterSnapshotGuardString(row["text"], `${at}.text`),
  };
}
