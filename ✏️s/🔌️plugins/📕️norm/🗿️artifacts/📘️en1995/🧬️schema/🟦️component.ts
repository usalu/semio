/** 🧬️ EN 1995 artifact schema. */

export interface En1995Artifact {
  /** @state persistent */
  annex: number;
  /** @state persistent */
  mEdKnm: number;
  /** @state persistent */
  nEdKn: number;
  /** @state persistent */
  vEdKn: number;
  /** @state persistent */
  wMm3: number;
  /** @state persistent */
  aMm2: number;
  /** @state persistent */
  bMm: number;
  /** @state persistent */
  hMm: number;
  /** @state persistent */
  fMK: number;
  /** @state persistent */
  fC0K: number;
  /** @state persistent */
  serviceClass: number;
  /** @state persistent */
  loadDuration: number;
  /** @state persistent */
  mCritKnm: number;
  /** @state persistent */
  fEdKn: number;
  /** @state persistent */
  aEfMm2: number;
  /** @state persistent */
  fVK: number;
  /** @state persistent */
  fireDurationMin: number;
  /** @state persistent */
  sectionDepthMm: number;
  /** @state persistent */
  aVertMS2: number;
  /** @state persistent */
  nCyclesBridge: number;
  /** @state shared-ui */
  selectedCheckIndex?: number | null;
}
