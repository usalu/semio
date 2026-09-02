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
  locale?: string;
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
