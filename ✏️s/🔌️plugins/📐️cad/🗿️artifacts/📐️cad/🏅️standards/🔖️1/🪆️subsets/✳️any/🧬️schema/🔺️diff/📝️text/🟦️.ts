import type { CadDiff } from "../🟦️.ts";

/** 🔺️ cad.diff text facade. `CadDiff` never implements `store::ArtifactDsl` anywhere in the cad
 * plugin — the sibling `🦀️.rs` only implements `apply_to_artifact`/`MutationDiff`; this
 * facet's `📖️.grammar.semio` is registered for LSP/tooling only, the same explicitly
 * documented case as `🗒️note/…/🚪️io/🔺️diff/📝️text/🦀️.rs`. There is nothing on the Rust
 * side to wire this to — parse/print for cad diff text do not exist, by design, not by omission. */
export function parseDsl(text: string): CadDiff {
  throw new Error("cad.diff has no text codec: CadDiff never implements ArtifactDsl (grammar is registered for tooling only, never parsed)");
}
export function printDsl(value: CadDiff): string {
  throw new Error("cad.diff has no text codec: CadDiff never implements ArtifactDsl (grammar is registered for tooling only, never parsed)");
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class cadCadDiffTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const cadCadDiffTextGuardReject = (at: string, why: string): never => {
  throw new cadCadDiffTextGuardRefusal(at, why);
};

type cadCadDiffTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type cadCadDiffTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type cadCadDiffTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const cadCadDiffTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : cadCadDiffTextGuardReject(at, "value is not an object");
export const cadCadDiffTextGuardArray = (value: unknown, at: string, bounds: cadCadDiffTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return cadCadDiffTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) cadCadDiffTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) cadCadDiffTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const cadCadDiffTextGuardString = (value: unknown, at: string, bounds: cadCadDiffTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return cadCadDiffTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) cadCadDiffTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) cadCadDiffTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) cadCadDiffTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const cadCadDiffTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : cadCadDiffTextGuardReject(at, "value is not a boolean"));
export const cadCadDiffTextGuardNumber = (value: unknown, at: string, bounds: cadCadDiffTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return cadCadDiffTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) cadCadDiffTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) cadCadDiffTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const cadCadDiffTextGuardInteger = (value: unknown, at: string, bounds: cadCadDiffTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? cadCadDiffTextGuardNumber(value, at, bounds) : cadCadDiffTextGuardReject(at, "value is not an integer");
export const cadCadDiffTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : cadCadDiffTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const cadCadDiffTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : cadCadDiffTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export type CadDiffText = Readonly<Record<string, unknown>>;

export function parseCadDiffText(value: unknown, at = "$"): CadDiffText {
  return cadCadDiffTextGuardObject(value, `${at}`);
}
