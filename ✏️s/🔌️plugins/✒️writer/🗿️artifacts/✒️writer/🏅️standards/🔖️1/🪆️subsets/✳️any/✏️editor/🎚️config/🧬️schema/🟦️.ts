/** 🧬️ WriterConfig */
export interface WriterConfig {
  /** @state config */
  editorSelection?: WriterEditorSelection;
  /** @state config */
  formatSignal: number;
  /** @state config */
  lintSignal: number;
  /** @state config */
  revision: number;
  /** @state config */
  editorSettings: WriterEditorSettings;
  /** @state config */
  engagementInput: string;
  /** @state config */
  camera: WriterCamera;
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

export interface WriterCamera {
  x: number;
  y: number;
  zoom: number;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class writerWriterConfigGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const writerWriterConfigGuardReject = (at: string, why: string): never => {
  throw new writerWriterConfigGuardRefusal(at, why);
};

type writerWriterConfigGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type writerWriterConfigGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type writerWriterConfigGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const writerWriterConfigGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : writerWriterConfigGuardReject(at, "value is not an object");
export const writerWriterConfigGuardArray = (value: unknown, at: string, bounds: writerWriterConfigGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return writerWriterConfigGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) writerWriterConfigGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) writerWriterConfigGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const writerWriterConfigGuardString = (value: unknown, at: string, bounds: writerWriterConfigGuardTextBounds = {}): string => {
  if (typeof value !== "string") return writerWriterConfigGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) writerWriterConfigGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) writerWriterConfigGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) writerWriterConfigGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const writerWriterConfigGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : writerWriterConfigGuardReject(at, "value is not a boolean"));
export const writerWriterConfigGuardNumber = (value: unknown, at: string, bounds: writerWriterConfigGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return writerWriterConfigGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) writerWriterConfigGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) writerWriterConfigGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const writerWriterConfigGuardInteger = (value: unknown, at: string, bounds: writerWriterConfigGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? writerWriterConfigGuardNumber(value, at, bounds) : writerWriterConfigGuardReject(at, "value is not an integer");
export const writerWriterConfigGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : writerWriterConfigGuardReject(at, `value is not one of ${members.join(", ")}`);
export const writerWriterConfigGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : writerWriterConfigGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseWriterEditorSelection(value: unknown, at = "$"): WriterEditorSelection {
  const row = writerWriterConfigGuardObject(value, at);
  return {
    start: writerWriterConfigGuardInteger(row["start"], `${at}.start`, {"minimum": 0}),
    end: writerWriterConfigGuardInteger(row["end"], `${at}.end`, {"minimum": 0}),
  };
}

export function parseWriterCamera(value: unknown, at = "$"): WriterCamera {
  const row = writerWriterConfigGuardObject(value, at);
  return {
    x: writerWriterConfigGuardNumber(row["x"], `${at}.x`),
    y: writerWriterConfigGuardNumber(row["y"], `${at}.y`),
    zoom: writerWriterConfigGuardNumber(row["zoom"], `${at}.zoom`),
  };
}
