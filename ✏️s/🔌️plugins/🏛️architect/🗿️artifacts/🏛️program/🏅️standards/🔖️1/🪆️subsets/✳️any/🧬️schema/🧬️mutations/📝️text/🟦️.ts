/** 📝️ Text representation for `architect.program.mutations`. */
export type ProgramMutationsText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class architectProgramMutationsTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const architectProgramMutationsTextGuardReject = (at: string, why: string): never => {
  throw new architectProgramMutationsTextGuardRefusal(at, why);
};

type architectProgramMutationsTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type architectProgramMutationsTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type architectProgramMutationsTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const architectProgramMutationsTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : architectProgramMutationsTextGuardReject(at, "value is not an object");
export const architectProgramMutationsTextGuardArray = (value: unknown, at: string, bounds: architectProgramMutationsTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return architectProgramMutationsTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) architectProgramMutationsTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) architectProgramMutationsTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const architectProgramMutationsTextGuardString = (value: unknown, at: string, bounds: architectProgramMutationsTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return architectProgramMutationsTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) architectProgramMutationsTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) architectProgramMutationsTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) architectProgramMutationsTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const architectProgramMutationsTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : architectProgramMutationsTextGuardReject(at, "value is not a boolean"));
export const architectProgramMutationsTextGuardNumber = (value: unknown, at: string, bounds: architectProgramMutationsTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return architectProgramMutationsTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) architectProgramMutationsTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) architectProgramMutationsTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const architectProgramMutationsTextGuardInteger = (value: unknown, at: string, bounds: architectProgramMutationsTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? architectProgramMutationsTextGuardNumber(value, at, bounds) : architectProgramMutationsTextGuardReject(at, "value is not an integer");
export const architectProgramMutationsTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : architectProgramMutationsTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const architectProgramMutationsTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : architectProgramMutationsTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseProgramMutationsText(value: unknown, at = "$"): ProgramMutationsText {
  return architectProgramMutationsTextGuardObject(value, `${at}`);
}
