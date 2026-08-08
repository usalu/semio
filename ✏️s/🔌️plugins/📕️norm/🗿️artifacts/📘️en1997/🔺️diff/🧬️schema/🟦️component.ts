/** 🧬️ EN 1997 diff schema. */

export interface En1997Diff {
  /** @state persistent */
  artifact?: En1997Artifact;
  /** @state persistent */
  vEdKn?: number;
  /** @state persistent */
  hEdKn?: number;
  /** @state persistent */
  footingAreaM2?: number;
  /** @state persistent */
  phiDeg?: number;
  /** @state persistent */
  cKpa?: number;
  /** @state persistent */
  gammaKnM3?: number;
  /** @state persistent */
  bM?: number;
  /** @state persistent */
  dFM?: number;
  /** @state persistent */
  eSMpa?: number;
  /** @state persistent */
  nu?: number;
  /** @state persistent */
  designApproach?: number;
  /** @state persistent */
  annex?: number;
  /** @state persistent */
  settlementLimitMm?: number;
  /** @state persistent */
  nPileEdKn?: number;
  /** @state persistent */
  alphaS?: number;
  /** @state persistent */
  pileDM?: number;
  /** @state persistent */
  qSKpa?: number;
  /** @state persistent */
  pileLM?: number;
  /** @state persistent */
  qBKpa?: number;
  /** @state persistent */
  pileBaseAreaM2?: number;
  /** @state persistent */
  pileNProfiles?: number;
  /** @state persistent */
  zInvestigatedM?: number;
  /** @state shared-ui */
  selectedCheckIndex?: number | null | null;
}
