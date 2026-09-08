/** 🧬️ Writer artifact schema. */

export interface WriterArtifact {
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
  /** @state presence */
  editorSelection?: WriterEditorSelection;
  /** @state presence */
  editorSettings: WriterEditorSettings;
  /** @state config */
  formatSignal: number;
  /** @state config */
  lintSignal: number;
  /** @state config */
  revision: number;
  /** @state config */
  engagementInput: string;
  /** @state config */
  cameraX: number;
  /** @state config */
  cameraY: number;
  /** @state config */
  cameraZoom: number;
  /** @state config */
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
export class writerWriterArtifactGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const writerWriterArtifactGuardReject = (at: string, why: string): never => {
  throw new writerWriterArtifactGuardRefusal(at, why);
};

type writerWriterArtifactGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type writerWriterArtifactGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type writerWriterArtifactGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const writerWriterArtifactGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : writerWriterArtifactGuardReject(at, "value is not an object");
export const writerWriterArtifactGuardArray = (value: unknown, at: string, bounds: writerWriterArtifactGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return writerWriterArtifactGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) writerWriterArtifactGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) writerWriterArtifactGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const writerWriterArtifactGuardString = (value: unknown, at: string, bounds: writerWriterArtifactGuardTextBounds = {}): string => {
  if (typeof value !== "string") return writerWriterArtifactGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) writerWriterArtifactGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) writerWriterArtifactGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) writerWriterArtifactGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const writerWriterArtifactGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : writerWriterArtifactGuardReject(at, "value is not a boolean"));
export const writerWriterArtifactGuardNumber = (value: unknown, at: string, bounds: writerWriterArtifactGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return writerWriterArtifactGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) writerWriterArtifactGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) writerWriterArtifactGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const writerWriterArtifactGuardInteger = (value: unknown, at: string, bounds: writerWriterArtifactGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? writerWriterArtifactGuardNumber(value, at, bounds) : writerWriterArtifactGuardReject(at, "value is not an integer");
export const writerWriterArtifactGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : writerWriterArtifactGuardReject(at, `value is not one of ${members.join(", ")}`);
export const writerWriterArtifactGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : writerWriterArtifactGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseWriterArtifact(value: unknown, at = "$"): WriterArtifact {
  const row = writerWriterArtifactGuardObject(value, at);
  return {
    schema: writerWriterArtifactGuardString(row["schema"], `${at}.schema`),
    id: writerWriterArtifactGuardString(row["id"], `${at}.id`),
    languageId: writerWriterArtifactGuardString(row["languageId"], `${at}.languageId`),
    uri: writerWriterArtifactGuardString(row["uri"], `${at}.uri`),
    text: writerWriterArtifactGuardString(row["text"], `${at}.text`),
    editorSelection: row["editorSelection"] === undefined ? undefined : parseWriterEditorSelection(row["editorSelection"], `${at}.editorSelection`),
    editorSettings: parseWriterEditorSettings(row["editorSettings"], `${at}.editorSettings`),
    formatSignal: writerWriterArtifactGuardInteger(row["formatSignal"], `${at}.formatSignal`, {"minimum": 0}),
    lintSignal: writerWriterArtifactGuardInteger(row["lintSignal"], `${at}.lintSignal`, {"minimum": 0}),
    revision: writerWriterArtifactGuardInteger(row["revision"], `${at}.revision`, {"minimum": 0}),
    engagementInput: writerWriterArtifactGuardString(row["engagementInput"], `${at}.engagementInput`),
    cameraX: writerWriterArtifactGuardNumber(row["cameraX"], `${at}.cameraX`),
    cameraY: writerWriterArtifactGuardNumber(row["cameraY"], `${at}.cameraY`),
    cameraZoom: writerWriterArtifactGuardNumber(row["cameraZoom"], `${at}.cameraZoom`),
  };
}

export function parseWriterEditorSelection(value: unknown, at = "$"): WriterEditorSelection {
  const row = writerWriterArtifactGuardObject(value, at);
  return {
    start: writerWriterArtifactGuardInteger(row["start"], `${at}.start`, {"minimum": 0}),
    end: writerWriterArtifactGuardInteger(row["end"], `${at}.end`, {"minimum": 0}),
  };
}

export function parseWriterEditorSettings(value: unknown, at = "$"): WriterEditorSettings {
  const row = writerWriterArtifactGuardObject(value, at);
  return {
    showLineNumbers: writerWriterArtifactGuardBoolean(row["showLineNumbers"], `${at}.showLineNumbers`),
    fontPx: writerWriterArtifactGuardInteger(row["fontPx"], `${at}.fontPx`, {"minimum": 0}),
    lineHeight: writerWriterArtifactGuardInteger(row["lineHeight"], `${at}.lineHeight`, {"minimum": 0}),
    tabSize: writerWriterArtifactGuardInteger(row["tabSize"], `${at}.tabSize`, {"minimum": 0}),
  };
}

export function parseWriterTextRangeEdit(value: unknown, at = "$"): WriterTextRangeEdit {
  const row = writerWriterArtifactGuardObject(value, at);
  return {
    start: writerWriterArtifactGuardInteger(row["start"], `${at}.start`, {"minimum": 0}),
    end: writerWriterArtifactGuardInteger(row["end"], `${at}.end`, {"minimum": 0}),
    insert: writerWriterArtifactGuardString(row["insert"], `${at}.insert`),
  };
}

export function parseWriterTextDelta(value: unknown, at = "$"): WriterTextDelta {
  const row = writerWriterArtifactGuardObject(value, at);
  return {
    replacement: row["replacement"] === undefined ? undefined : writerWriterArtifactGuardString(row["replacement"], `${at}.replacement`),
    edits: writerWriterArtifactGuardArray(row["edits"], `${at}.edits`).map((item, index) => parseWriterTextRangeEdit(item, `${at}.edits[${index}]`)),
  };
}
