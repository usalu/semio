/** 🧬️ EN 1995 diff schema. */

export interface En1995Diff {
  /** @state artifact */
  artifact?: En1995Artifact;
  /** @state artifact */
  annex?: number;
  /** @state artifact */
  mEdKnm?: number;
  /** @state artifact */
  nEdKn?: number;
  /** @state artifact */
  vEdKn?: number;
  /** @state artifact */
  wMm3?: number;
  /** @state artifact */
  aMm2?: number;
  /** @state artifact */
  bMm?: number;
  /** @state artifact */
  hMm?: number;
  /** @state artifact */
  fMK?: number;
  /** @state artifact */
  fC0K?: number;
  /** @state artifact */
  serviceClass?: number;
  /** @state artifact */
  loadDuration?: number;
  /** @state artifact */
  mCritKnm?: number;
  /** @state artifact */
  fEdKn?: number;
  /** @state artifact */
  aEfMm2?: number;
  /** @state artifact */
  fVK?: number;
  /** @state artifact */
  fireDurationMin?: number;
  /** @state artifact */
  sectionDepthMm?: number;
  /** @state artifact */
  aVertMS2?: number;
  /** @state artifact */
  nCyclesBridge?: number;
  /** @state presence */
  selectedCheckIndex?: number | null | null;
}
