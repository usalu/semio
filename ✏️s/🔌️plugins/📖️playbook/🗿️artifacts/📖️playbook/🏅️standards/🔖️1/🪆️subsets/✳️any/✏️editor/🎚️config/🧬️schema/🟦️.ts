/** 🧬️ PlaybookConfig */
export interface PlaybookConfig {
  /** @state config */
  /** @state config */
  contributionsJson: string;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class playbookPlaybookConfigGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const playbookPlaybookConfigGuardReject = (at: string, why: string): never => {
  throw new playbookPlaybookConfigGuardRefusal(at, why);
};

type playbookPlaybookConfigGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type playbookPlaybookConfigGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type playbookPlaybookConfigGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const playbookPlaybookConfigGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : playbookPlaybookConfigGuardReject(at, "value is not an object");
export const playbookPlaybookConfigGuardArray = (value: unknown, at: string, bounds: playbookPlaybookConfigGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return playbookPlaybookConfigGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) playbookPlaybookConfigGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) playbookPlaybookConfigGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const playbookPlaybookConfigGuardString = (value: unknown, at: string, bounds: playbookPlaybookConfigGuardTextBounds = {}): string => {
  if (typeof value !== "string") return playbookPlaybookConfigGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) playbookPlaybookConfigGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) playbookPlaybookConfigGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) playbookPlaybookConfigGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const playbookPlaybookConfigGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : playbookPlaybookConfigGuardReject(at, "value is not a boolean"));
export const playbookPlaybookConfigGuardNumber = (value: unknown, at: string, bounds: playbookPlaybookConfigGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return playbookPlaybookConfigGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) playbookPlaybookConfigGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) playbookPlaybookConfigGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const playbookPlaybookConfigGuardInteger = (value: unknown, at: string, bounds: playbookPlaybookConfigGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? playbookPlaybookConfigGuardNumber(value, at, bounds) : playbookPlaybookConfigGuardReject(at, "value is not an integer");
export const playbookPlaybookConfigGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : playbookPlaybookConfigGuardReject(at, `value is not one of ${members.join(", ")}`);
export const playbookPlaybookConfigGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : playbookPlaybookConfigGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parsePlaybookConfig(value: unknown, at = "$"): PlaybookConfig {
  const row = playbookPlaybookConfigGuardObject(value, at);
  return {
    contributionsJson: playbookPlaybookConfigGuardString(row["contributionsJson"], `${at}.contributionsJson`),
  };
}
