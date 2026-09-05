/** 🧬️ EN 1996 artifact schema. */

export interface En1996Artifact {
  /** @state artifact */
  mEdKnm: number;
  /** @state artifact */
  nEdKn: number;
  /** @state artifact */
  vEdKn: number;
  /** @state artifact */
  hEdKn: number;
  /** @state artifact */
  zMm3: number;
  /** @state artifact */
  areaMm2: number;
  /** @state artifact */
  shearAreaMm2: number;
  /** @state artifact */
  fKMpa: number;
  /** @state artifact */
  fVkMpa: number;
  /** @state artifact */
  annex: number;
  /** @state artifact */
  masonryClass: number;
  /** @state artifact */
  designSituation: number;
  /** @state artifact */
  mu: number;
  /** @state artifact */
  wallThicknessMm: number;
  /** @state artifact */
  fireResistanceMin: number;
  /** @state artifact */
  unit: number;
  /** @state artifact */
  exposure: number;
  /** @state artifact */
  mortar: number;
  /** @state artifact */
  bedJointThicknessMm: number;
  /** @state artifact */
  storeys: number;
  /** @state artifact */
  hEfMm: number;
  /** @state artifact */
  tEfMm: number;
  /** @state presence */
  selectedCheckIndex?: number | null;
}
