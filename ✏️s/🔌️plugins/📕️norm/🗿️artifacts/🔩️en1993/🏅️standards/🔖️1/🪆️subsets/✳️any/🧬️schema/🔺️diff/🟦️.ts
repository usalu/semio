/** 🧬️ En1993 diff schema — sparse field delta. */

export interface En1993Diff {
  /** @state artifact */
  artifact?: En1993Artifact;
  /** @state artifact */
  annex?: string;
  /** @state artifact */
  nEdKn?: number;
  /** @state artifact */
  mEdKnm?: number;
  /** @state artifact */
  vEdKn?: number;
  /** @state artifact */
  aMm2?: number;
  /** @state artifact */
  aVMm2?: number;
  /** @state artifact */
  wPlMm3?: number;
  /** @state artifact */
  fYMpa?: number;
  /** @state artifact */
  fUMpa?: number;
  /** @state artifact */
  chi?: number;
  /** @state artifact */
  aNetMm2?: number;
  /** @state artifact */
  tensionNEdKn?: number;
  /** @state artifact */
  fireThicknessMm?: number;
  /** @state artifact */
  fireRating?: string;
  /** @state artifact */
  fireMassivity?: number;
  /** @state artifact */
  fireMu0?: number;
  /** @state artifact */
  fireDesignTemperatureC?: number;
  /** @state artifact */
  cfBBarMm?: number;
  /** @state artifact */
  cfTMm?: number;
  /** @state artifact */
  cfKSigma?: number;
  /** @state artifact */
  cfPsi?: number;
  /** @state artifact */
  cfNEdKn?: number;
  /** @state artifact */
  cfGrossResistanceKn?: number;
  /** @state artifact */
  stainlessMEdKnm?: number;
  /** @state artifact */
  stainlessWPlMm3?: number;
  /** @state artifact */
  stainlessFYMpa?: number;
  /** @state artifact */
  platedLambdaP?: number;
  /** @state artifact */
  platedSigmaEdMpa?: number;
  /** @state artifact */
  siloTMm?: number;
  /** @state artifact */
  siloRMm?: number;
  /** @state artifact */
  shellSigmaXEdMpa?: number;
  /** @state artifact */
  siloK?: number;
  /** @state artifact */
  siloGammaKnM3?: number;
  /** @state artifact */
  siloDepthM?: number;
  /** @state artifact */
  boltFEdKn?: number;
  /** @state artifact */
  boltNBolts?: number;
  /** @state artifact */
  boltASMm2?: number;
  /** @state artifact */
  boltE1Mm?: number;
  /** @state artifact */
  boltE2Mm?: number;
  /** @state artifact */
  boltD0Mm?: number;
  /** @state artifact */
  boltDMm?: number;
  /** @state artifact */
  boltTMm?: number;
  /** @state artifact */
  boltFUMpa?: number;
  /** @state artifact */
  boltFUbMpa?: number;
  /** @state artifact */
  weldAMm?: number;
  /** @state artifact */
  weldLMm?: number;
  /** @state artifact */
  weldFUMpa?: number;
  /** @state artifact */
  weldSteelGrade?: string;
  /** @state artifact */
  weldFEdKn?: number;
  /** @state artifact */
  deltaSigmaMpa?: number;
  /** @state artifact */
  fatigueCategory?: number;
  /** @state artifact */
  fatigueMethod?: string;
  /** @state artifact */
  t10SteelSubgrade?: string;
  /** @state artifact */
  t10ActualThicknessMm?: number;
  /** @state artifact */
  t10TEdC?: number;
  /** @state artifact */
  tensionComponentFUkKn?: number;
  /** @state artifact */
  tensionComponentFKKn?: number;
  /** @state artifact */
  tensionComponentNEdKn?: number;
  /** @state artifact */
  hssWElMm3?: number;
  /** @state artifact */
  hssFYMpa?: number;
  /** @state artifact */
  hssSectionClass?: number;
  /** @state artifact */
  hssMEdKnm?: number;
  /** @state artifact */
  bridgeLambda?: number;
  /** @state artifact */
  bridgePhi2?: number;
  /** @state artifact */
  bridgeDeltaSigmaPMpa?: number;
  /** @state artifact */
  towerWindFactor?: number;
  /** @state artifact */
  towerNEdKn?: number;
  /** @state artifact */
  pileSigmaMpa?: number;
  /** @state artifact */
  pileKRed?: number;
  /** @state artifact */
  pileNEdKn?: number;
  /** @state artifact */
  craneFZEdKn?: number;
  /** @state artifact */
  craneWheelContactLengthMm?: number;
  /** @state artifact */
  craneDispersionMm?: number;
  /** @state artifact */
  craneTWMm?: number;
  /** @state presence */
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
