/** 📸️ Forms snapshot schema — artifact-lane fields only. Mirrors Rust `FormsSnapshot`
 * (sibling `🦀️.rs`): ticket 26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM replaced the old
 * inline `steps: FormStep[]` field with two fixed composed CHILD slots (`structure`/`results`) —
 * this facet no longer defines its own document tree, it composes stdio's `value`/`table` subsets
 * instead. */

export interface FormsSnapshot {
  /** @state artifact */
  schema: string;
  /** @state artifact */
  id: string;
  /** @state artifact */
  version: string;
  /** @state artifact */
  title?: string;
  /** @state artifact */
  structure: ArtifactChildHandle;
  /** @state artifact */
  results: ArtifactChildHandle;
}

export interface ArtifactDialect {
  artifactKind: string;
  standard: string;
  subset: string;
}

export interface ArtifactRef {
  artifactId: string;
  dialect: ArtifactDialect;
}
/** 🌉️ Mirrors `store::ArtifactChild<S>` — `childId`/`target` only; `local_owner` and
 *  `PhantomData<S>` are `#[serde(skip)]`. */
export interface ArtifactChildHandle {
  childId: string;
  target: ArtifactRef;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class formsFormsSnapshotGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const formsFormsSnapshotGuardReject = (at: string, why: string): never => {
  throw new formsFormsSnapshotGuardRefusal(at, why);
};

type formsFormsSnapshotGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type formsFormsSnapshotGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type formsFormsSnapshotGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const formsFormsSnapshotGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : formsFormsSnapshotGuardReject(at, "value is not an object");
export const formsFormsSnapshotGuardArray = (value: unknown, at: string, bounds: formsFormsSnapshotGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return formsFormsSnapshotGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) formsFormsSnapshotGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) formsFormsSnapshotGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const formsFormsSnapshotGuardString = (value: unknown, at: string, bounds: formsFormsSnapshotGuardTextBounds = {}): string => {
  if (typeof value !== "string") return formsFormsSnapshotGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) formsFormsSnapshotGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) formsFormsSnapshotGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) formsFormsSnapshotGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const formsFormsSnapshotGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : formsFormsSnapshotGuardReject(at, "value is not a boolean"));
export const formsFormsSnapshotGuardNumber = (value: unknown, at: string, bounds: formsFormsSnapshotGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return formsFormsSnapshotGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) formsFormsSnapshotGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) formsFormsSnapshotGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const formsFormsSnapshotGuardInteger = (value: unknown, at: string, bounds: formsFormsSnapshotGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? formsFormsSnapshotGuardNumber(value, at, bounds) : formsFormsSnapshotGuardReject(at, "value is not an integer");
export const formsFormsSnapshotGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : formsFormsSnapshotGuardReject(at, `value is not one of ${members.join(", ")}`);
export const formsFormsSnapshotGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : formsFormsSnapshotGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseFormsSnapshot(value: unknown, at = "$"): FormsSnapshot {
  const row = formsFormsSnapshotGuardObject(value, at);
  return {
    schema: formsFormsSnapshotGuardString(row["schema"], `${at}.schema`),
    value: row["value"] === undefined ? undefined : formsFormsSnapshotGuardString(row["value"], `${at}.value`),
  };
}
