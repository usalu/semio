/** 🧬️ EN 1998 diff schema. */

export interface En1998Diff {
  /** @state persistent */
  artifact?: En1998Artifact;
  /** @state persistent */
  seismicZone?: number;
  /** @state persistent */
  groundType?: number;
  /** @state persistent */
  importanceClass?: number;
  /** @state persistent */
  structuralSystem?: number;
  /** @state persistent */
  t1S?: number;
  /** @state persistent */
  massT?: number;
  /** @state persistent */
  vRdKn?: number;
  /** @state persistent */
  driftMm?: number;
  /** @state persistent */
  heightM?: number;
  /** @state persistent */
  multipleResistingSystems?: boolean;
  /** @state persistent */
  annex?: number;
  /** @state persistent */
  enAGr?: number;
  /** @state persistent */
  enGroundType?: number;
  /** @state persistent */
  enSpectrumType?: number;
  /** @state persistent */
  periodRatio?: number;
  /** @state persistent */
  bridgeVRdKn?: number;
  /** @state persistent */
  bearingDEdMm?: number;
  /** @state persistent */
  bearingDRdMm?: number;
  /** @state persistent */
  retrofitKnowledgeLevel?: number;
  /** @state persistent */
  retrofitLimitState?: number;
  /** @state persistent */
  retrofitEDKn?: number;
  /** @state persistent */
  retrofitRKKn?: number;
  /** @state persistent */
  retrofitGammaEl?: number;
  /** @state persistent */
  siloHeightM?: number;
  /** @state persistent */
  siloRadiusM?: number;
  /** @state persistent */
  siloNRdKn?: number;
  /** @state persistent */
  siloVEdKn?: number;
  /** @state persistent */
  siloVRdKn?: number;
  /** @state persistent */
  siloQNominal?: number;
  /** @state persistent */
  tankHeightM?: number;
  /** @state persistent */
  tankRadiusM?: number;
  /** @state persistent */
  tankMassT?: number;
  /** @state persistent */
  tankVRdKn?: number;
  /** @state persistent */
  towerMEdKnm?: number;
  /** @state persistent */
  towerMRdKnm?: number;
  /** @state persistent */
  towerIsChimney?: boolean;
  /** @state persistent */
  towerQNominal?: number;
  /** @state persistent */
  towerMassT?: number;
  /** @state persistent */
  foundationAreaM2?: number;
  /** @state persistent */
  foundationPRdKpa?: number;
  /** @state persistent */
  foundationHEdKn?: number;
  /** @state persistent */
  foundationHRdKn?: number;
  /** @state persistent */
  kFoundation?: number;
  /** @state persistent */
  kSoil?: number;
  /** @state persistent */
  wallHeightM?: number;
  /** @state persistent */
  wallPhiDeg?: number;
  /** @state persistent */
  wallSoilGammaKnM3?: number;
  /** @state persistent */
  wallR?: number;
  /** @state persistent */
  wallHRdKn?: number;
  /** @state shared-ui */
  selectedCheckIndex?: number | null | null;
}
