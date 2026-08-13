/** 🧬️ EN 1998 artifact schema. */

export interface En1998Artifact {
  /** @state artifact */
  seismicZone: number;
  /** @state artifact */
  groundType: number;
  /** @state artifact */
  importanceClass: number;
  /** @state artifact */
  structuralSystem: number;
  /** @state artifact */
  t1S: number;
  /** @state artifact */
  massT: number;
  /** @state artifact */
  vRdKn: number;
  /** @state artifact */
  driftMm: number;
  /** @state artifact */
  heightM: number;
  /** @state artifact */
  multipleResistingSystems: boolean;
  /** @state artifact */
  annex: number;
  /** @state artifact */
  enAGr: number;
  /** @state artifact */
  enGroundType: number;
  /** @state artifact */
  enSpectrumType: number;
  /** @state artifact */
  periodRatio: number;
  /** @state artifact */
  bridgeVRdKn: number;
  /** @state artifact */
  bearingDEdMm: number;
  /** @state artifact */
  bearingDRdMm: number;
  /** @state artifact */
  retrofitKnowledgeLevel: number;
  /** @state artifact */
  retrofitLimitState: number;
  /** @state artifact */
  retrofitEDKn: number;
  /** @state artifact */
  retrofitRKKn: number;
  /** @state artifact */
  retrofitGammaEl: number;
  /** @state artifact */
  siloHeightM: number;
  /** @state artifact */
  siloRadiusM: number;
  /** @state artifact */
  siloNRdKn: number;
  /** @state artifact */
  siloVEdKn: number;
  /** @state artifact */
  siloVRdKn: number;
  /** @state artifact */
  siloQNominal: number;
  /** @state artifact */
  tankHeightM: number;
  /** @state artifact */
  tankRadiusM: number;
  /** @state artifact */
  tankMassT: number;
  /** @state artifact */
  tankVRdKn: number;
  /** @state artifact */
  towerMEdKnm: number;
  /** @state artifact */
  towerMRdKnm: number;
  /** @state artifact */
  towerIsChimney: boolean;
  /** @state artifact */
  towerQNominal: number;
  /** @state artifact */
  towerMassT: number;
  /** @state artifact */
  foundationAreaM2: number;
  /** @state artifact */
  foundationPRdKpa: number;
  /** @state artifact */
  foundationHEdKn: number;
  /** @state artifact */
  foundationHRdKn: number;
  /** @state artifact */
  kFoundation: number;
  /** @state artifact */
  kSoil: number;
  /** @state artifact */
  wallHeightM: number;
  /** @state artifact */
  wallPhiDeg: number;
  /** @state artifact */
  wallSoilGammaKnM3: number;
  /** @state artifact */
  wallR: number;
  /** @state artifact */
  wallHRdKn: number;
  /** @state presence */
  selectedCheckIndex?: number | null;
}
