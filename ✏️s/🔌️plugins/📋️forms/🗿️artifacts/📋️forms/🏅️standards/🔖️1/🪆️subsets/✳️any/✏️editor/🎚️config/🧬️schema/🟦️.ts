/** 🧬️ FormsConfig */
export interface FormsConfig {
  /** @state config */
  currentStepIndex: number;
  /** @state config */
  tryValues: Record<string, string[]>;
  /** @state config */
  /** @state config */
  contributionsJson: string;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class formsFormsConfigGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const formsFormsConfigGuardReject = (at: string, why: string): never => {
  throw new formsFormsConfigGuardRefusal(at, why);
};

type formsFormsConfigGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type formsFormsConfigGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type formsFormsConfigGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const formsFormsConfigGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : formsFormsConfigGuardReject(at, "value is not an object");
export const formsFormsConfigGuardArray = (value: unknown, at: string, bounds: formsFormsConfigGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return formsFormsConfigGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) formsFormsConfigGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) formsFormsConfigGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const formsFormsConfigGuardString = (value: unknown, at: string, bounds: formsFormsConfigGuardTextBounds = {}): string => {
  if (typeof value !== "string") return formsFormsConfigGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) formsFormsConfigGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) formsFormsConfigGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) formsFormsConfigGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const formsFormsConfigGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : formsFormsConfigGuardReject(at, "value is not a boolean"));
export const formsFormsConfigGuardNumber = (value: unknown, at: string, bounds: formsFormsConfigGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return formsFormsConfigGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) formsFormsConfigGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) formsFormsConfigGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const formsFormsConfigGuardInteger = (value: unknown, at: string, bounds: formsFormsConfigGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? formsFormsConfigGuardNumber(value, at, bounds) : formsFormsConfigGuardReject(at, "value is not an integer");
export const formsFormsConfigGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : formsFormsConfigGuardReject(at, `value is not one of ${members.join(", ")}`);
export const formsFormsConfigGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : formsFormsConfigGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseFormsConfig(value: unknown, at = "$"): FormsConfig {
  const row = formsFormsConfigGuardObject(value, at);
  return {
    currentStepIndex: formsFormsConfigGuardInteger(row["currentStepIndex"], `${at}.currentStepIndex`, {"minimum": 0}),
    tryValues: formsFormsConfigGuardObject(row["tryValues"], `${at}.tryValues`),
    contributionsJson: formsFormsConfigGuardString(row["contributionsJson"], `${at}.contributionsJson`),
  };
}
