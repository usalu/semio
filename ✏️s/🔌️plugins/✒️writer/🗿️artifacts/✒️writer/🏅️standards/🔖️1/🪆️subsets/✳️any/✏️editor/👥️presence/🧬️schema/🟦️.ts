/** 🧬️ WriterPresence */
export interface WriterPresence {
  /** @state presence */
  editorSelection?: WriterEditorSelection;
  /** @state presence */
  camera: WriterCamera;
}

export interface WriterEditorSelection {
  start: number;
  end: number;
}

export interface WriterCamera {
  x: number;
  y: number;
  zoom: number;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class writerWriterPresenceGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const writerWriterPresenceGuardReject = (at: string, why: string): never => {
  throw new writerWriterPresenceGuardRefusal(at, why);
};

type writerWriterPresenceGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type writerWriterPresenceGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type writerWriterPresenceGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const writerWriterPresenceGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : writerWriterPresenceGuardReject(at, "value is not an object");
export const writerWriterPresenceGuardArray = (value: unknown, at: string, bounds: writerWriterPresenceGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return writerWriterPresenceGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) writerWriterPresenceGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) writerWriterPresenceGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const writerWriterPresenceGuardString = (value: unknown, at: string, bounds: writerWriterPresenceGuardTextBounds = {}): string => {
  if (typeof value !== "string") return writerWriterPresenceGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) writerWriterPresenceGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) writerWriterPresenceGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) writerWriterPresenceGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const writerWriterPresenceGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : writerWriterPresenceGuardReject(at, "value is not a boolean"));
export const writerWriterPresenceGuardNumber = (value: unknown, at: string, bounds: writerWriterPresenceGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return writerWriterPresenceGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) writerWriterPresenceGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) writerWriterPresenceGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const writerWriterPresenceGuardInteger = (value: unknown, at: string, bounds: writerWriterPresenceGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? writerWriterPresenceGuardNumber(value, at, bounds) : writerWriterPresenceGuardReject(at, "value is not an integer");
export const writerWriterPresenceGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : writerWriterPresenceGuardReject(at, `value is not one of ${members.join(", ")}`);
export const writerWriterPresenceGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : writerWriterPresenceGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseWriterPresence(value: unknown, at = "$"): WriterPresence {
  const row = writerWriterPresenceGuardObject(value, at);
  return {
    editorSelection: row["editorSelection"] === undefined ? undefined : parseWriterEditorSelection(row["editorSelection"], `${at}.editorSelection`),
    camera: parseWriterCamera(row["camera"], `${at}.camera`),
  };
}

export function parseWriterEditorSelection(value: unknown, at = "$"): WriterEditorSelection {
  const row = writerWriterPresenceGuardObject(value, at);
  return {
    start: writerWriterPresenceGuardInteger(row["start"], `${at}.start`, {"minimum": 0}),
    end: writerWriterPresenceGuardInteger(row["end"], `${at}.end`, {"minimum": 0}),
  };
}

export function parseWriterCamera(value: unknown, at = "$"): WriterCamera {
  const row = writerWriterPresenceGuardObject(value, at);
  return {
    x: writerWriterPresenceGuardNumber(row["x"], `${at}.x`),
    y: writerWriterPresenceGuardNumber(row["y"], `${at}.y`),
    zoom: writerWriterPresenceGuardNumber(row["zoom"], `${at}.zoom`),
  };
}
