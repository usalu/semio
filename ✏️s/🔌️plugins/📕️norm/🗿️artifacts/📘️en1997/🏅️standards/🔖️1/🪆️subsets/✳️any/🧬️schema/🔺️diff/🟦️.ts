/** 🧬️ EN 1997 diff schema. */

import type { En1997Artifact } from "../🟦️.ts";

export interface En1997Diff {
  /** @state artifact */
  artifact?: En1997Artifact;
  /** @state artifact */
  vEdKn?: number;
  /** @state artifact */
  hEdKn?: number;
  /** @state artifact */
  footingAreaM2?: number;
  /** @state artifact */
  phiDeg?: number;
  /** @state artifact */
  cKpa?: number;
  /** @state artifact */
  gammaKnM3?: number;
  /** @state artifact */
  bM?: number;
  /** @state artifact */
  dFM?: number;
  /** @state artifact */
  eSMpa?: number;
  /** @state artifact */
  nu?: number;
  /** @state artifact */
  designApproach?: number;
  /** @state artifact */
  annex?: number;
  /** @state artifact */
  settlementLimitMm?: number;
  /** @state artifact */
  nPileEdKn?: number;
  /** @state artifact */
  alphaS?: number;
  /** @state artifact */
  pileDM?: number;
  /** @state artifact */
  qSKpa?: number;
  /** @state artifact */
  pileLM?: number;
  /** @state artifact */
  qBKpa?: number;
  /** @state artifact */
  pileBaseAreaM2?: number;
  /** @state artifact */
  pileNProfiles?: number;
  /** @state artifact */
  zInvestigatedM?: number;
  /** @state presence */
  selectedCheckIndex?: number | null;
}
