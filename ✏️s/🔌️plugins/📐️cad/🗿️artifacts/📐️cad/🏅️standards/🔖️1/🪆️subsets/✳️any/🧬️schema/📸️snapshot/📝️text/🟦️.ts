import type { CadSnapshot } from "../🟦️.ts";

/** 🗣️ cad.cad snapshot text facade. Real codec: `parse_dsl`/`print_dsl` in the sibling
 * `🦀️.rs`, wrapping `store::ArtifactDsl for CadSnapshot` (callable natively from Rust
 * today). Not wired here: `world actor`'s WIT surface (🧰️framework/🛍️products/💻️os/🔨️modules/
 * 🔌️plugin/🧬️schema/📜️.wit) exports only `poll`/`jobs`/`checkpoint`/`describe` — the
 * `read/load-app-document-text` export that could have carried this was deleted in the B1
 * world-collapse in favor of one turn-loop entry point. Needs a new stateless codec-call WIT export
 * plus a TS host loader (jco-generated component bindings, see 🧫️fixtures/🔌️jcoprobe). See
 * 📓️wasm-facade-wiring.md. */
export function parseDsl(text: string): CadSnapshot {
  throw new Error("cad.cad parseDsl: no WASM codec-call export exists (world actor only exports poll/jobs/checkpoint/describe); wire once one is added");
}
export function printDsl(value: CadSnapshot): string {
  throw new Error("cad.cad printDsl: no WASM codec-call export exists (world actor only exports poll/jobs/checkpoint/describe); wire once one is added");
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class cadCadSnapshotTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const cadCadSnapshotTextGuardReject = (at: string, why: string): never => {
  throw new cadCadSnapshotTextGuardRefusal(at, why);
};

type cadCadSnapshotTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type cadCadSnapshotTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type cadCadSnapshotTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const cadCadSnapshotTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : cadCadSnapshotTextGuardReject(at, "value is not an object");
export const cadCadSnapshotTextGuardArray = (value: unknown, at: string, bounds: cadCadSnapshotTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return cadCadSnapshotTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) cadCadSnapshotTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) cadCadSnapshotTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const cadCadSnapshotTextGuardString = (value: unknown, at: string, bounds: cadCadSnapshotTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return cadCadSnapshotTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) cadCadSnapshotTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) cadCadSnapshotTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) cadCadSnapshotTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const cadCadSnapshotTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : cadCadSnapshotTextGuardReject(at, "value is not a boolean"));
export const cadCadSnapshotTextGuardNumber = (value: unknown, at: string, bounds: cadCadSnapshotTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return cadCadSnapshotTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) cadCadSnapshotTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) cadCadSnapshotTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const cadCadSnapshotTextGuardInteger = (value: unknown, at: string, bounds: cadCadSnapshotTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? cadCadSnapshotTextGuardNumber(value, at, bounds) : cadCadSnapshotTextGuardReject(at, "value is not an integer");
export const cadCadSnapshotTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : cadCadSnapshotTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const cadCadSnapshotTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : cadCadSnapshotTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export type CadSnapshotText = Readonly<Record<string, unknown>>;

export function parseCadSnapshotText(value: unknown, at = "$"): CadSnapshotText {
  return cadCadSnapshotTextGuardObject(value, `${at}`);
}
