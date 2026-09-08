/** 🧬️ ArchitectPresence */
export type AdjacencyKind = "required" | "preferred" | "optional" | "prohibited";

export interface ArchitectPresence {
  /** @state presence */
  activeRegister: string;
  /** @state presence */
  adjacencyKindFilter?: AdjacencyKind;
  /** @state presence */
  graphCameraX: number;
  /** @state presence */
  graphCameraY: number;
  /** @state presence */
  graphCameraZoom: number;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class architectArchitectPresenceGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const architectArchitectPresenceGuardReject = (at: string, why: string): never => {
  throw new architectArchitectPresenceGuardRefusal(at, why);
};

type architectArchitectPresenceGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type architectArchitectPresenceGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type architectArchitectPresenceGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const architectArchitectPresenceGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : architectArchitectPresenceGuardReject(at, "value is not an object");
export const architectArchitectPresenceGuardArray = (value: unknown, at: string, bounds: architectArchitectPresenceGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return architectArchitectPresenceGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) architectArchitectPresenceGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) architectArchitectPresenceGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const architectArchitectPresenceGuardString = (value: unknown, at: string, bounds: architectArchitectPresenceGuardTextBounds = {}): string => {
  if (typeof value !== "string") return architectArchitectPresenceGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) architectArchitectPresenceGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) architectArchitectPresenceGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) architectArchitectPresenceGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const architectArchitectPresenceGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : architectArchitectPresenceGuardReject(at, "value is not a boolean"));
export const architectArchitectPresenceGuardNumber = (value: unknown, at: string, bounds: architectArchitectPresenceGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return architectArchitectPresenceGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) architectArchitectPresenceGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) architectArchitectPresenceGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const architectArchitectPresenceGuardInteger = (value: unknown, at: string, bounds: architectArchitectPresenceGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? architectArchitectPresenceGuardNumber(value, at, bounds) : architectArchitectPresenceGuardReject(at, "value is not an integer");
export const architectArchitectPresenceGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : architectArchitectPresenceGuardReject(at, `value is not one of ${members.join(", ")}`);
export const architectArchitectPresenceGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : architectArchitectPresenceGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseArchitectPresence(value: unknown, at = "$"): ArchitectPresence {
  const row = architectArchitectPresenceGuardObject(value, at);
  return {
    activeRegister: architectArchitectPresenceGuardString(row["activeRegister"], `${at}.activeRegister`),
    adjacencyKindFilter: row["adjacencyKindFilter"] === undefined ? undefined : architectArchitectPresenceGuardMember(row["adjacencyKindFilter"], `${at}.adjacencyKindFilter`, ["required", "preferred", "optional", "prohibited"] as const),
    graphCameraX: architectArchitectPresenceGuardNumber(row["graphCameraX"], `${at}.graphCameraX`),
    graphCameraY: architectArchitectPresenceGuardNumber(row["graphCameraY"], `${at}.graphCameraY`),
    graphCameraZoom: architectArchitectPresenceGuardNumber(row["graphCameraZoom"], `${at}.graphCameraZoom`),
  };
}
