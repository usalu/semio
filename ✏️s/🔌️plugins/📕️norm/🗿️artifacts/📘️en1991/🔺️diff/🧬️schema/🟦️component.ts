/** 🧬️ En1991 diff schema — sparse field delta. */

export interface En1991Diff {
  /** @state persistent */
  artifact?: En1991Artifact;
  /** @state persistent */
  areaM2?: number;
  /** @state persistent */
  category?: string;
  /** @state persistent */
  annex?: string;
  /** @state persistent */
  selfWeightMaterial?: string;
  /** @state persistent */
  selfWeightThicknessM?: number;
  /** @state persistent */
  assumedGKKnM2?: number;
  /** @state persistent */
  fireCurve?: string;
  /** @state persistent */
  fireResistanceMin?: number;
  /** @state persistent */
  fireMemberCapacityC?: number;
  /** @state persistent */
  snowZone?: number;
  /** @state persistent */
  snowAltitudeM?: number;
  /** @state persistent */
  enSKKnM2?: number;
  /** @state persistent */
  windZone?: number;
  /** @state persistent */
  enVBMS?: number;
  /** @state persistent */
  deltaTK?: number;
  /** @state persistent */
  constructionActivity?: string;
  /** @state persistent */
  accidentalMassT?: number;
  /** @state persistent */
  accidentalSpeedKmH?: number;
  /** @state persistent */
  bridgeLane?: number;
  /** @state persistent */
  bridgeSpanM?: number;
  /** @state persistent */
  bridgeLaneWidthM?: number;
  /** @state persistent */
  bridgeMomentResistanceKnm?: number;
  /** @state persistent */
  craneClass?: string;
  /** @state persistent */
  hoistClass?: string;
  /** @state persistent */
  hoistingSpeedMS?: number;
  /** @state persistent */
  siloBulkDensityKnM3?: number;
  /** @state persistent */
  siloHeightM?: number;
  /** @state persistent */
  siloHydraulicRadiusM?: number;
  /** @state persistent */
  siloMu?: number;
  /** @state persistent */
  siloK?: number;
  /** @state persistent */
  cS?: number;
  /** @state persistent */
  cD?: number;
  /** @state shared-ui */
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