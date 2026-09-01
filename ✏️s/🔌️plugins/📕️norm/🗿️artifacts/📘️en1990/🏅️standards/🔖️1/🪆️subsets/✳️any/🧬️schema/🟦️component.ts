/** 🧬️ En1990 artifact schema — every field with its state class. */

export interface En1990Artifact {
  /** @state artifact */
  gK: number;
  /** @state artifact */
  qK: ArtifactChildHandle;
  /** @state artifact */
  resistanceKn: number;
  /** @state artifact */
  consequenceClass: number;
  /** @state artifact */
  annex: string;
  /** @state artifact */
  seismicAEdKn: number;
  /** @state presence */
  selectedCheckIndex?: number | null;
}

/** 🌉️ Opaque mirror of `store::os_io::ArtifactRef` — a cross-cutting framework identity type, out of
 *  this facet's own domain. */
export interface ArtifactDialect {
  artifactKind: string;
  standard: string;
  subset: string;
}

export interface ArtifactRef {
  artifactId: string;
  dialect: ArtifactDialect;
}

/** 🌉️ Mirrors `store::ArtifactChild<S>` (`#[serde(rename_all = "camelCase")]`, `child_id`/`target`
 *  fields only — the `local_owner`/`PhantomData<S>` fields are `#[serde(skip)]`). */
export interface ArtifactChildHandle {
  childId: string;
  target: ArtifactRef;
}
