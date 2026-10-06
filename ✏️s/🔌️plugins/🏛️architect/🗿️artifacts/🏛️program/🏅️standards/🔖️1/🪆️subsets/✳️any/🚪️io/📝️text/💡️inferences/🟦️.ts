/** 📝️ Text representation for `architect.program.inference`. */
export type ProgramInferenceText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class architectProgramInferenceTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const architectProgramInferenceTextGuardReject = (at: string, why: string): never => {
  throw new architectProgramInferenceTextGuardRefusal(at, why);
};

type architectProgramInferenceTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type architectProgramInferenceTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type architectProgramInferenceTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const architectProgramInferenceTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : architectProgramInferenceTextGuardReject(at, "value is not an object");
export const architectProgramInferenceTextGuardArray = (value: unknown, at: string, bounds: architectProgramInferenceTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return architectProgramInferenceTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) architectProgramInferenceTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) architectProgramInferenceTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const architectProgramInferenceTextGuardString = (value: unknown, at: string, bounds: architectProgramInferenceTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return architectProgramInferenceTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) architectProgramInferenceTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) architectProgramInferenceTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) architectProgramInferenceTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const architectProgramInferenceTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : architectProgramInferenceTextGuardReject(at, "value is not a boolean"));
export const architectProgramInferenceTextGuardNumber = (value: unknown, at: string, bounds: architectProgramInferenceTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return architectProgramInferenceTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) architectProgramInferenceTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) architectProgramInferenceTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const architectProgramInferenceTextGuardInteger = (value: unknown, at: string, bounds: architectProgramInferenceTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? architectProgramInferenceTextGuardNumber(value, at, bounds) : architectProgramInferenceTextGuardReject(at, "value is not an integer");
export const architectProgramInferenceTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : architectProgramInferenceTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const architectProgramInferenceTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : architectProgramInferenceTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseProgramInferenceText(value: unknown, at = "$"): ProgramInferenceText {
  return architectProgramInferenceTextGuardObject(value, `${at}`);
}
