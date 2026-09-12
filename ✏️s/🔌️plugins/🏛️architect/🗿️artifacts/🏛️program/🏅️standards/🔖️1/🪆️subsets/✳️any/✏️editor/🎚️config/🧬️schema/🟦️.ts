/** 🧬️ ArchitectConfig */
export interface ArchitectConfig {
  /** @state config */
  searchQuery: string;
  /** @state config */
  searchHistoryJson: string;
  /** @state config */
  lastResultJson: string;
  /** @state config */
  lastAnalysisJson: string;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class architectArchitectConfigGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const architectArchitectConfigGuardReject = (at: string, why: string): never => {
  throw new architectArchitectConfigGuardRefusal(at, why);
};

type architectArchitectConfigGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type architectArchitectConfigGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type architectArchitectConfigGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const architectArchitectConfigGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : architectArchitectConfigGuardReject(at, "value is not an object");
export const architectArchitectConfigGuardArray = (value: unknown, at: string, bounds: architectArchitectConfigGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return architectArchitectConfigGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) architectArchitectConfigGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) architectArchitectConfigGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const architectArchitectConfigGuardString = (value: unknown, at: string, bounds: architectArchitectConfigGuardTextBounds = {}): string => {
  if (typeof value !== "string") return architectArchitectConfigGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) architectArchitectConfigGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) architectArchitectConfigGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) architectArchitectConfigGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const architectArchitectConfigGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : architectArchitectConfigGuardReject(at, "value is not a boolean"));
export const architectArchitectConfigGuardNumber = (value: unknown, at: string, bounds: architectArchitectConfigGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return architectArchitectConfigGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) architectArchitectConfigGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) architectArchitectConfigGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const architectArchitectConfigGuardInteger = (value: unknown, at: string, bounds: architectArchitectConfigGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? architectArchitectConfigGuardNumber(value, at, bounds) : architectArchitectConfigGuardReject(at, "value is not an integer");
export const architectArchitectConfigGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : architectArchitectConfigGuardReject(at, `value is not one of ${members.join(", ")}`);
export const architectArchitectConfigGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : architectArchitectConfigGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseArchitectConfig(value: unknown, at = "$"): ArchitectConfig {
  const row = architectArchitectConfigGuardObject(value, at);
  return {
    searchQuery: architectArchitectConfigGuardString(row["searchQuery"], `${at}.searchQuery`),
    searchHistoryJson: architectArchitectConfigGuardString(row["searchHistoryJson"], `${at}.searchHistoryJson`),
    lastResultJson: architectArchitectConfigGuardString(row["lastResultJson"], `${at}.lastResultJson`),
    lastAnalysisJson: architectArchitectConfigGuardString(row["lastAnalysisJson"], `${at}.lastAnalysisJson`),
  };
}
