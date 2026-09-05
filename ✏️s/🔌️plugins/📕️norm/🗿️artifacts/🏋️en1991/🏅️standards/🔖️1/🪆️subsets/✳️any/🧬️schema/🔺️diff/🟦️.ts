/** 🧬️ En1991 diff schema — sparse field delta. */

export interface En1991Diff {
  /** @state artifact */
  artifact?: En1991Artifact;
  /** @state artifact */
  areaM2?: number;
  /** @state artifact */
  category?: string;
  /** @state artifact */
  annex?: string;
  /** @state artifact */
  selfWeightMaterial?: string;
  /** @state artifact */
  selfWeightThicknessM?: number;
  /** @state artifact */
  assumedGKKnM2?: number;
  /** @state artifact */
  fireCurve?: string;
  /** @state artifact */
  fireResistanceMin?: number;
  /** @state artifact */
  fireMemberCapacityC?: number;
  /** @state artifact */
  snowZone?: number;
  /** @state artifact */
  snowAltitudeM?: number;
  /** @state artifact */
  enSKKnM2?: number;
  /** @state artifact */
  windZone?: number;
  /** @state artifact */
  enVBMS?: number;
  /** @state artifact */
  deltaTK?: number;
  /** @state artifact */
  constructionActivity?: string;
  /** @state artifact */
  accidentalMassT?: number;
  /** @state artifact */
  accidentalSpeedKmH?: number;
  /** @state artifact */
  bridgeLane?: number;
  /** @state artifact */
  bridgeSpanM?: number;
  /** @state artifact */
  bridgeLaneWidthM?: number;
  /** @state artifact */
  bridgeMomentResistanceKnm?: number;
  /** @state artifact */
  craneClass?: string;
  /** @state artifact */
  hoistClass?: string;
  /** @state artifact */
  hoistingSpeedMS?: number;
  /** @state artifact */
  siloBulkDensityKnM3?: number;
  /** @state artifact */
  siloHeightM?: number;
  /** @state artifact */
  siloHydraulicRadiusM?: number;
  /** @state artifact */
  siloMu?: number;
  /** @state artifact */
  siloK?: number;
  /** @state artifact */
  cS?: number;
  /** @state artifact */
  cD?: number;
  /** @state presence */
  selectedCheckIndex?: number | null;
}

export interface En1991Artifact {
  areaM2: number;
  category: string;
  annex: string;
  selfWeightMaterial: string;
  selfWeightThicknessM: number;
  assumedGKKnM2: number;
  fireCurve: string;
  fireResistanceMin: number;
  fireMemberCapacityC: number;
  snowZone: number;
  snowAltitudeM: number;
  enSKKnM2: number;
  windZone: number;
  enVBMS: number;
  deltaTK: number;
  constructionActivity: string;
  accidentalMassT: number;
  accidentalSpeedKmH: number;
  bridgeLane: number;
  bridgeSpanM: number;
  bridgeLaneWidthM: number;
  bridgeMomentResistanceKnm: number;
  craneClass: string;
  hoistClass: string;
  hoistingSpeedMS: number;
  siloBulkDensityKnM3: number;
  siloHeightM: number;
  siloHydraulicRadiusM: number;
  siloMu: number;
  siloK: number;
  cS: number;
  cD: number;
  selectedCheckIndex?: number | null;
}
