/** 🔧️ cad.mutation op-text facade. Real codec: `impl protocol::OpText for CadMutation` in the
 * sibling `🦀️.rs` (`parse_op`/`print_op`, callable natively from Rust today). Not wired
 * here: `world actor`'s WIT surface (🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧬️schema/
 * 📜️.wit) exports only `poll`/`jobs`/`checkpoint`/`describe` — the per-verb
 * `apply-mutations[-text]` export that could have carried this was deleted in the B1 world-collapse
 * in favor of one turn-loop entry point. Needs a new stateless codec-call WIT export plus a TS host
 * loader (jco-generated component bindings, see 🧫️fixtures/🔌️jcoprobe) before this can call real
 * Rust. See 📓️wasm-facade-wiring.md. */
export function parseDsl(text: string): unknown {
  throw new Error("cad.mutation parseDsl: no WASM codec-call export exists (world actor only exports poll/jobs/checkpoint/describe); wire once one is added");
}
export function printDsl(value: unknown): string {
  throw new Error("cad.mutation printDsl: no WASM codec-call export exists (world actor only exports poll/jobs/checkpoint/describe); wire once one is added");
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class cadCadMutationsTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const cadCadMutationsTextGuardReject = (at: string, why: string): never => {
  throw new cadCadMutationsTextGuardRefusal(at, why);
};

type cadCadMutationsTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type cadCadMutationsTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type cadCadMutationsTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const cadCadMutationsTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : cadCadMutationsTextGuardReject(at, "value is not an object");
export const cadCadMutationsTextGuardArray = (value: unknown, at: string, bounds: cadCadMutationsTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return cadCadMutationsTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) cadCadMutationsTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) cadCadMutationsTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const cadCadMutationsTextGuardString = (value: unknown, at: string, bounds: cadCadMutationsTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return cadCadMutationsTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) cadCadMutationsTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) cadCadMutationsTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) cadCadMutationsTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const cadCadMutationsTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : cadCadMutationsTextGuardReject(at, "value is not a boolean"));
export const cadCadMutationsTextGuardNumber = (value: unknown, at: string, bounds: cadCadMutationsTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return cadCadMutationsTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) cadCadMutationsTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) cadCadMutationsTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const cadCadMutationsTextGuardInteger = (value: unknown, at: string, bounds: cadCadMutationsTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? cadCadMutationsTextGuardNumber(value, at, bounds) : cadCadMutationsTextGuardReject(at, "value is not an integer");
export const cadCadMutationsTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : cadCadMutationsTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const cadCadMutationsTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : cadCadMutationsTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export type CadMutationsText = Readonly<Record<string, unknown>>;

export function parseCadMutationsText(value: unknown, at = "$"): CadMutationsText {
  return cadCadMutationsTextGuardObject(value, `${at}`);
}
