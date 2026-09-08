/** @emoji 🧬️ Forms artifact schema — artifact-, presence- and config-lane fields. */
export interface FormsArtifact {
  schema: string;
  id: string;
  version: string;
  title?: string | null;
  steps: FormStep[];
  selectedIds: string[];
  currentStepIndex: number;
  tryValues: Record<string, string[]>;
  contributionsJson: string;
}

export interface FormStep {
  id: string;
  title: string;
  description?: string;
  blocks: FormQuestion[];
}

export interface FormQuestion {
  id: string;
  label: string;
  kind: string;
  [key: string]: unknown;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class formsFormsArtifactGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const formsFormsArtifactGuardReject = (at: string, why: string): never => {
  throw new formsFormsArtifactGuardRefusal(at, why);
};

type formsFormsArtifactGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type formsFormsArtifactGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type formsFormsArtifactGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const formsFormsArtifactGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : formsFormsArtifactGuardReject(at, "value is not an object");
export const formsFormsArtifactGuardArray = (value: unknown, at: string, bounds: formsFormsArtifactGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return formsFormsArtifactGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) formsFormsArtifactGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) formsFormsArtifactGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const formsFormsArtifactGuardString = (value: unknown, at: string, bounds: formsFormsArtifactGuardTextBounds = {}): string => {
  if (typeof value !== "string") return formsFormsArtifactGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) formsFormsArtifactGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) formsFormsArtifactGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) formsFormsArtifactGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const formsFormsArtifactGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : formsFormsArtifactGuardReject(at, "value is not a boolean"));
export const formsFormsArtifactGuardNumber = (value: unknown, at: string, bounds: formsFormsArtifactGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return formsFormsArtifactGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) formsFormsArtifactGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) formsFormsArtifactGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const formsFormsArtifactGuardInteger = (value: unknown, at: string, bounds: formsFormsArtifactGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? formsFormsArtifactGuardNumber(value, at, bounds) : formsFormsArtifactGuardReject(at, "value is not an integer");
export const formsFormsArtifactGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : formsFormsArtifactGuardReject(at, `value is not one of ${members.join(", ")}`);
export const formsFormsArtifactGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : formsFormsArtifactGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseFormStep(value: unknown, at = "$"): FormStep {
  const row = formsFormsArtifactGuardObject(value, at);
  return {
    id: formsFormsArtifactGuardString(row["id"], `${at}.id`),
    title: formsFormsArtifactGuardString(row["title"], `${at}.title`),
    description: row["description"] === undefined ? undefined : formsFormsArtifactGuardString(row["description"], `${at}.description`),
    blocks: formsFormsArtifactGuardArray(row["blocks"], `${at}.blocks`).map((item, index) => parseFormQuestion(item, `${at}.blocks[${index}]`)),
  };
}

export function parseFormQuestion(value: unknown, at = "$"): FormQuestion {
  const row = formsFormsArtifactGuardObject(value, at);
  return {
    id: formsFormsArtifactGuardString(row["id"], `${at}.id`),
    label: formsFormsArtifactGuardString(row["label"], `${at}.label`),
    kind: formsFormsArtifactGuardString(row["kind"], `${at}.kind`),
  };
}

export interface FormsStringList {
  readonly values: readonly string[];
}

export function parseFormsStringList(value: unknown, at = "$"): FormsStringList {
  const row = formsFormsArtifactGuardObject(value, at);
  return {
    values: formsFormsArtifactGuardArray(row["values"], `${at}.values`).map((item, index) => formsFormsArtifactGuardString(item, `${at}.values[${index}]`)),
  };
}
