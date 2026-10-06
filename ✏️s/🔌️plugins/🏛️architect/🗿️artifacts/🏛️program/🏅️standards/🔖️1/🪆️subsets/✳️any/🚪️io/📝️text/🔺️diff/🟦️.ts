/** 📝️ Text representation for `architect.program.diff`. */
export type ProgramDiffText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class architectProgramDiffTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const architectProgramDiffTextGuardReject = (at: string, why: string): never => {
  throw new architectProgramDiffTextGuardRefusal(at, why);
};

type architectProgramDiffTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type architectProgramDiffTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type architectProgramDiffTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const architectProgramDiffTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : architectProgramDiffTextGuardReject(at, "value is not an object");
export const architectProgramDiffTextGuardArray = (value: unknown, at: string, bounds: architectProgramDiffTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return architectProgramDiffTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) architectProgramDiffTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) architectProgramDiffTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const architectProgramDiffTextGuardString = (value: unknown, at: string, bounds: architectProgramDiffTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return architectProgramDiffTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) architectProgramDiffTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) architectProgramDiffTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) architectProgramDiffTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const architectProgramDiffTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : architectProgramDiffTextGuardReject(at, "value is not a boolean"));
export const architectProgramDiffTextGuardNumber = (value: unknown, at: string, bounds: architectProgramDiffTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return architectProgramDiffTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) architectProgramDiffTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) architectProgramDiffTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const architectProgramDiffTextGuardInteger = (value: unknown, at: string, bounds: architectProgramDiffTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? architectProgramDiffTextGuardNumber(value, at, bounds) : architectProgramDiffTextGuardReject(at, "value is not an integer");
export const architectProgramDiffTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : architectProgramDiffTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const architectProgramDiffTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : architectProgramDiffTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseProgramDiffText(value: unknown, at = "$"): ProgramDiffText {
  return architectProgramDiffTextGuardObject(value, `${at}`);
}
