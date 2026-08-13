/** 🧬️ En1991 snapshot schema — persistent fields only. */

export interface En1991Snapshot {
  /** @state artifact */
  areaM2: number;
  /** @state artifact */
  category: string;
  /** @state artifact */
  annex: string;
  /** @state artifact */
  selfWeightMaterial: string;
  /** @state artifact */
  selfWeightThicknessM: number;
  /** @state artifact */
  assumedGKKnM2: number;
  /** @state artifact */
  fireCurve: string;
  /** @state artifact */
  fireResistanceMin: number;
  /** @state artifact */
  fireMemberCapacityC: number;
  /** @state artifact */
  snowZone: number;
  /** @state artifact */
  snowAltitudeM: number;
  /** @state artifact */
  enSKKnM2: number;
  /** @state artifact */
  windZone: number;
  /** @state artifact */
  enVBMS: number;
  /** @state artifact */
  deltaTK: number;
  /** @state artifact */
  constructionActivity: string;
  /** @state artifact */
  accidentalMassT: number;
  /** @state artifact */
  accidentalSpeedKmH: number;
  /** @state artifact */
  bridgeLane: number;
  /** @state artifact */
  bridgeSpanM: number;
  /** @state artifact */
  bridgeLaneWidthM: number;
  /** @state artifact */
  bridgeMomentResistanceKnm: number;
  /** @state artifact */
  craneClass: string;
  /** @state artifact */
  hoistClass: string;
  /** @state artifact */
  hoistingSpeedMS: number;
  /** @state artifact */
  siloBulkDensityKnM3: number;
  /** @state artifact */
  siloHeightM: number;
  /** @state artifact */
  siloHydraulicRadiusM: number;
  /** @state artifact */
  siloMu: number;
  /** @state artifact */
  siloK: number;
  /** @state artifact */
  cS: number;
  /** @state artifact */
  cD: number;
}
