/** 🔺️ Forms diff schema — sparse field delta over the artifact. Mirrors Rust `FormsDiff` (sibling
 * `🦀️.rs`): `structure`/`results` are single-`Option` composed child-handle swaps
 * (ticket 26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM), never a whole-snapshot `artifact` replace —
 * that dead slot was removed. `FormsStepsDelta`/`FormsStepPatch` stay declared (the "DeltaHelpers"
 * region of the same Rust file) even though `FormsDiff` itself no longer carries a `steps` field:
 * every mutation triad still builds its change as a `FormsStepsDelta` internally, applied against
 * the working-scene steps, before regenerating `structure`/`results`. */

export interface FormsDiff {
  /** @state artifact */
  schema?: string;
  /** @state artifact */
  id?: string;
  /** @state artifact */
  version?: string;
  /** @state artifact */
  title?: string | null;
  /** @state artifact */
  structure?: ArtifactChildHandle;
  /** @state artifact */
  results?: ArtifactChildHandle;
  /** @state presence */
  selectedIds?: FormsStringList;
  /** @state config */
  currentStepIndex?: number;
  /** @state config */
  tryValues?: Record<string, string[]>;
  /** @state config */
  /** @state config */
  contributionsJson?: string;
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

export type DslValue = Record<string, unknown>;

export interface FormQuestionOption {
  value: string;
  label: string;
}

export interface FormVectorField {
  key: string;
  label?: string;
  value?: number;
}

export type FormExpr =
  | { kind: 'const'; value: DslValue }
  | { kind: 'var'; name: string }
  | { kind: 'eq'; left: FormExpr; right: FormExpr }
  | { kind: 'and'; items: FormExpr[] }
  | { kind: 'or'; items: FormExpr[] }
  | { kind: 'truthy'; expr: FormExpr };

export interface FormQuestion {
  id: string;
  label: string;
  kind: string;
  description?: string;
  required?: boolean;
  placeholder?: string;
  default?: DslValue;
  min?: number;
  max?: number;
  step?: number;
  unit?: string;
  text?: string;
  options?: FormQuestionOption[];
  fields?: FormVectorField[];
  schema?: string;
  src?: string;
  accept?: string;
  fixtureSlug?: string;
  params?: DslValue;
  condition?: FormExpr;
}

export interface FormStep {
  id: string;
  title: string;
  description?: string;
  blocks: FormQuestion[];
}

/** 📋 String-list wrapper so optional list diffs stay scalar across formats. */
export interface FormsStringList {
  values: string[];
}

/** 🧩 Identified-collection delta for `steps`, built internally by every mutation triad. */
export interface FormsStepsDelta {
  added: FormStep[];
  removed: string[];
  patched: FormsStepPatchEntry[];
  reordered?: string[];
}

/** 🩹 One patched step entry. */
export interface FormsStepPatchEntry {
  id: string;
  patch: FormsStepPatch;
}

/** 🩹 Partial step replacement — `blocks`, when set, is the step's FULL new `blocks` list. */
export interface FormsStepPatch {
  title?: string;
  description?: string | null;
  blocks?: FormQuestion[];
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class formsFormsDiffGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const formsFormsDiffGuardReject = (at: string, why: string): never => {
  throw new formsFormsDiffGuardRefusal(at, why);
};

type formsFormsDiffGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type formsFormsDiffGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type formsFormsDiffGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const formsFormsDiffGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : formsFormsDiffGuardReject(at, "value is not an object");
export const formsFormsDiffGuardArray = (value: unknown, at: string, bounds: formsFormsDiffGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return formsFormsDiffGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) formsFormsDiffGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) formsFormsDiffGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const formsFormsDiffGuardString = (value: unknown, at: string, bounds: formsFormsDiffGuardTextBounds = {}): string => {
  if (typeof value !== "string") return formsFormsDiffGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) formsFormsDiffGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) formsFormsDiffGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) formsFormsDiffGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const formsFormsDiffGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : formsFormsDiffGuardReject(at, "value is not a boolean"));
export const formsFormsDiffGuardNumber = (value: unknown, at: string, bounds: formsFormsDiffGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return formsFormsDiffGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) formsFormsDiffGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) formsFormsDiffGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const formsFormsDiffGuardInteger = (value: unknown, at: string, bounds: formsFormsDiffGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? formsFormsDiffGuardNumber(value, at, bounds) : formsFormsDiffGuardReject(at, "value is not an integer");
export const formsFormsDiffGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : formsFormsDiffGuardReject(at, `value is not one of ${members.join(", ")}`);
export const formsFormsDiffGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : formsFormsDiffGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseFormsDiff(value: unknown, at = "$"): FormsDiff {
  const row = formsFormsDiffGuardObject(value, at);
  return {
    schema: row["schema"] === undefined ? undefined : formsFormsDiffGuardString(row["schema"], `${at}.schema`),
    value: row["value"] === undefined ? undefined : formsFormsDiffGuardString(row["value"], `${at}.value`),
  };
}
