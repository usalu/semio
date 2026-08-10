/** 🧬️ En1993 artifact schema — every field with its state class. */

export interface En1993Artifact {
  /** @state persistent */
  annex: string;
  /** @state persistent */
  nEdKn: number;
  /** @state persistent */
  mEdKnm: number;
  /** @state persistent */
  vEdKn: number;
  /** @state persistent */
  aMm2: number;
  /** @state persistent */
  aVMm2: number;
  /** @state persistent */
  wPlMm3: number;
  /** @state persistent */
  fYMpa: number;
  /** @state persistent */
  fUMpa: number;
  /** @state persistent */
  chi: number;
  /** @state persistent */
  aNetMm2: number;
  /** @state persistent */
  tensionNEdKn: number;
  /** @state persistent */
  fireThicknessMm: number;
  /** @state persistent */
  fireRating: string;
  /** @state persistent */
  fireMassivity: number;
  /** @state persistent */
  fireMu0: number;
  /** @state persistent */
  fireDesignTemperatureC: number;
  /** @state persistent */
  cfBBarMm: number;
  /** @state persistent */
  cfTMm: number;
  /** @state persistent */
  cfKSigma: number;
  /** @state persistent */
  cfPsi: number;
  /** @state persistent */
  cfNEdKn: number;
  /** @state persistent */
  cfGrossResistanceKn: number;
  /** @state persistent */
  stainlessMEdKnm: number;
  /** @state persistent */
  stainlessWPlMm3: number;
  /** @state persistent */
  stainlessFYMpa: number;
  /** @state persistent */
  platedLambdaP: number;
  /** @state persistent */
  platedSigmaEdMpa: number;
  /** @state persistent */
  siloTMm: number;
  /** @state persistent */
  siloRMm: number;
  /** @state persistent */
  shellSigmaXEdMpa: number;
  /** @state persistent */
  siloK: number;
  /** @state persistent */
  siloGammaKnM3: number;
  /** @state persistent */
  siloDepthM: number;
  /** @state persistent */
  boltFEdKn: number;
  /** @state persistent */
  boltNBolts: number;
  /** @state persistent */
  boltASMm2: number;
  /** @state persistent */
  boltE1Mm: number;
  /** @state persistent */
  boltE2Mm: number;
  /** @state persistent */
  boltD0Mm: number;
  /** @state persistent */
  boltDMm: number;
  /** @state persistent */
  boltTMm: number;
  /** @state persistent */
  boltFUMpa: number;
  /** @state persistent */
  boltFUbMpa: number;
  /** @state persistent */
  weldAMm: number;
  /** @state persistent */
  weldLMm: number;
  /** @state persistent */
  weldFUMpa: number;
  /** @state persistent */
  weldSteelGrade: string;
  /** @state persistent */
  weldFEdKn: number;
  /** @state persistent */
  deltaSigmaMpa: number;
  /** @state persistent */
  fatigueCategory: number;
  /** @state persistent */
  fatigueMethod: string;
  /** @state persistent */
  t10SteelSubgrade: string;
  /** @state persistent */
  t10ActualThicknessMm: number;
  /** @state persistent */
  t10TEdC: number;
  /** @state persistent */
  tensionComponentFUkKn: number;
  /** @state persistent */
  tensionComponentFKKn: number;
  /** @state persistent */
  tensionComponentNEdKn: number;
  /** @state persistent */
  hssWElMm3: number;
  /** @state persistent */
  hssFYMpa: number;
  /** @state persistent */
  hssSectionClass: number;
  /** @state persistent */
  hssMEdKnm: number;
  /** @state persistent */
  bridgeLambda: number;
  /** @state persistent */
  bridgePhi2: number;
  /** @state persistent */
  bridgeDeltaSigmaPMpa: number;
  /** @state persistent */
  towerWindFactor: number;
  /** @state persistent */
  towerNEdKn: number;
  /** @state persistent */
  pileSigmaMpa: number;
  /** @state persistent */
  pileKRed: number;
  /** @state persistent */
  pileNEdKn: number;
  /** @state persistent */
  craneFZEdKn: number;
  /** @state persistent */
  craneWheelContactLengthMm: number;
  /** @state persistent */
  craneDispersionMm: number;
  /** @state persistent */
  craneTWMm: number;
  /** @state shared-ui */
  selectedCheckIndex?: number | null;
}
