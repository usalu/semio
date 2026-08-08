/** 🧬️ En1993 diff schema — sparse field delta. */

export interface En1993Diff {
  /** @state persistent */
  artifact?: En1993Artifact;
  /** @state persistent */
  annex?: string;
  /** @state persistent */
  nEdKn?: number;
  /** @state persistent */
  mEdKnm?: number;
  /** @state persistent */
  vEdKn?: number;
  /** @state persistent */
  aMm2?: number;
  /** @state persistent */
  aVMm2?: number;
  /** @state persistent */
  wPlMm3?: number;
  /** @state persistent */
  fYMpa?: number;
  /** @state persistent */
  fUMpa?: number;
  /** @state persistent */
  chi?: number;
  /** @state persistent */
  aNetMm2?: number;
  /** @state persistent */
  tensionNEdKn?: number;
  /** @state persistent */
  fireThicknessMm?: number;
  /** @state persistent */
  fireRating?: string;
  /** @state persistent */
  fireMassivity?: number;
  /** @state persistent */
  fireMu0?: number;
  /** @state persistent */
  fireDesignTemperatureC?: number;
  /** @state persistent */
  cfBBarMm?: number;
  /** @state persistent */
  cfTMm?: number;
  /** @state persistent */
  cfKSigma?: number;
  /** @state persistent */
  cfPsi?: number;
  /** @state persistent */
  cfNEdKn?: number;
  /** @state persistent */
  cfGrossResistanceKn?: number;
  /** @state persistent */
  stainlessMEdKnm?: number;
  /** @state persistent */
  stainlessWPlMm3?: number;
  /** @state persistent */
  stainlessFYMpa?: number;
  /** @state persistent */
  platedLambdaP?: number;
  /** @state persistent */
  platedSigmaEdMpa?: number;
  /** @state persistent */
  siloTMm?: number;
  /** @state persistent */
  siloRMm?: number;
  /** @state persistent */
  shellSigmaXEdMpa?: number;
  /** @state persistent */
  siloK?: number;
  /** @state persistent */
  siloGammaKnM3?: number;
  /** @state persistent */
  siloDepthM?: number;
  /** @state persistent */
  boltFEdKn?: number;
  /** @state persistent */
  boltNBolts?: number;
  /** @state persistent */
  boltASMm2?: number;
  /** @state persistent */
  boltE1Mm?: number;
  /** @state persistent */
  boltE2Mm?: number;
  /** @state persistent */
  boltD0Mm?: number;
  /** @state persistent */
  boltDMm?: number;
  /** @state persistent */
  boltTMm?: number;
  /** @state persistent */
  boltFUMpa?: number;
  /** @state persistent */
  boltFUbMpa?: number;
  /** @state persistent */
  weldAMm?: number;
  /** @state persistent */
  weldLMm?: number;
  /** @state persistent */
  weldFUMpa?: number;
  /** @state persistent */
  weldSteelGrade?: string;
  /** @state persistent */
  weldFEdKn?: number;
  /** @state persistent */
  deltaSigmaMpa?: number;
  /** @state persistent */
  fatigueCategory?: number;
  /** @state persistent */
  fatigueMethod?: string;
  /** @state persistent */
  t10SteelSubgrade?: string;
  /** @state persistent */
  t10ActualThicknessMm?: number;
  /** @state persistent */
  t10TEdC?: number;
  /** @state persistent */
  tensionComponentFUkKn?: number;
  /** @state persistent */
  tensionComponentFKKn?: number;
  /** @state persistent */
  tensionComponentNEdKn?: number;
  /** @state persistent */
  hssWElMm3?: number;
  /** @state persistent */
  hssFYMpa?: number;
  /** @state persistent */
  hssSectionClass?: number;
  /** @state persistent */
  hssMEdKnm?: number;
  /** @state persistent */
  bridgeLambda?: number;
  /** @state persistent */
  bridgePhi2?: number;
  /** @state persistent */
  bridgeDeltaSigmaPMpa?: number;
  /** @state persistent */
  towerWindFactor?: number;
  /** @state persistent */
  towerNEdKn?: number;
  /** @state persistent */
  pileSigmaMpa?: number;
  /** @state persistent */
  pileKRed?: number;
  /** @state persistent */
  pileNEdKn?: number;
  /** @state persistent */
  craneFZEdKn?: number;
  /** @state persistent */
  craneWheelContactLengthMm?: number;
  /** @state persistent */
  craneDispersionMm?: number;
  /** @state persistent */
  craneTWMm?: number;
  /** @state shared-ui */
  selectedCheckIndex?: number | null;
}

export interface En1993Artifact {
  annex: string;
  nEdKn: number;
  mEdKnm: number;
  vEdKn: number;
  aMm2: number;
  aVMm2: number;
  wPlMm3: number;
  fYMpa: number;
  fUMpa: number;
  chi: number;
  aNetMm2: number;
  tensionNEdKn: number;
  fireThicknessMm: number;
  fireRating: string;
  fireMassivity: number;
  fireMu0: number;
  fireDesignTemperatureC: number;
  cfBBarMm: number;
  cfTMm: number;
  cfKSigma: number;
  cfPsi: number;
  cfNEdKn: number;
  cfGrossResistanceKn: number;
  stainlessMEdKnm: number;
  stainlessWPlMm3: number;
  stainlessFYMpa: number;
  platedLambdaP: number;
  platedSigmaEdMpa: number;
  siloTMm: number;
  siloRMm: number;
  shellSigmaXEdMpa: number;
  siloK: number;
  siloGammaKnM3: number;
  siloDepthM: number;
  boltFEdKn: number;
  boltNBolts: number;
  boltASMm2: number;
  boltE1Mm: number;
  boltE2Mm: number;
  boltD0Mm: number;
  boltDMm: number;
  boltTMm: number;
  boltFUMpa: number;
  boltFUbMpa: number;
  weldAMm: number;
  weldLMm: number;
  weldFUMpa: number;
  weldSteelGrade: string;
  weldFEdKn: number;
  deltaSigmaMpa: number;
  fatigueCategory: number;
  fatigueMethod: string;
  t10SteelSubgrade: string;
  t10ActualThicknessMm: number;
  t10TEdC: number;
  tensionComponentFUkKn: number;
  tensionComponentFKKn: number;
  tensionComponentNEdKn: number;
  hssWElMm3: number;
  hssFYMpa: number;
  hssSectionClass: number;
  hssMEdKnm: number;
  bridgeLambda: number;
  bridgePhi2: number;
  bridgeDeltaSigmaPMpa: number;
  towerWindFactor: number;
  towerNEdKn: number;
  pileSigmaMpa: number;
  pileKRed: number;
  pileNEdKn: number;
  craneFZEdKn: number;
  craneWheelContactLengthMm: number;
  craneDispersionMm: number;
  craneTWMm: number;
  selectedCheckIndex?: number | null;
}