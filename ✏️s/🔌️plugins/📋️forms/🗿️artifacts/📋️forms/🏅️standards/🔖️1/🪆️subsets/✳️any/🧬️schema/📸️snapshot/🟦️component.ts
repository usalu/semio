/** 📸️ Forms snapshot schema — artifact-lane fields only. Mirrors Rust `FormsSnapshot`
 * (sibling `🦀️component.rs`): ticket 26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM replaced the old
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
