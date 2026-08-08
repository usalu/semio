/** 🧬️ EN 1996 artifact schema. */

export interface En1996Artifact {
  /** @state persistent */
  mEdKnm: number;
  /** @state persistent */
  nEdKn: number;
  /** @state persistent */
  vEdKn: number;
  /** @state persistent */
  hEdKn: number;
  /** @state persistent */
  zMm3: number;
  /** @state persistent */
  areaMm2: number;
  /** @state persistent */
  shearAreaMm2: number;
  /** @state persistent */
  fKMpa: number;
  /** @state persistent */
  fVkMpa: number;
  /** @state persistent */
  annex: number;
  /** @state persistent */
  masonryClass: number;
  /** @state persistent */
  designSituation: number;
  /** @state persistent */
  mu: number;
  /** @state persistent */
  wallThicknessMm: number;
  /** @state persistent */
  fireResistanceMin: number;
  /** @state persistent */
  unit: number;
  /** @state persistent */
  exposure: number;
  /** @state persistent */
  mortar: number;
  /** @state persistent */
  bedJointThicknessMm: number;
  /** @state persistent */
  storeys: number;
  /** @state persistent */
  hEfMm: number;
  /** @state persistent */
  tEfMm: number;
  /** @state shared-ui */
  selectedCheckIndex?: number | null;
}
