/** 🧬️ EN 1999 diff schema. */

import type { En1999Artifact } from "../🟦️component.ts";

export interface En1999Diff {
  /** @state artifact */
  artifact?: En1999Artifact;
  /** @state artifact */
  nEdKn?: number;
  /** @state artifact */
  mEdKnm?: number;
  /** @state artifact */
  aMm2?: number;
  /** @state artifact */
  wElMm3?: number;
  /** @state artifact */
  alloy?: number;
  /** @state artifact */
  chi?: number;
  /** @state artifact */
  iTMm4?: number;
  /** @state artifact */
  lCrMm?: number;
  /** @state artifact */
  thetaC?: number;
  /** @state artifact */
  deltaSigmaEd?: number;
  /** @state artifact */
  deltaSigmaC?: number;
  /** @state artifact */
  fatigueM?: number;
  /** @state artifact */
  nCycles?: number;
  /** @state artifact */
  vWeldEdKn?: number;
  /** @state artifact */
  weldThroatMm?: number;
  /** @state artifact */
  weldLengthMm?: number;
  /** @state artifact */
  betaW?: number;
  /** @state artifact */
  sheetBMm?: number;
  /** @state artifact */
  sheetTMm?: number;
  /** @state artifact */
  sheetKSigma?: number;
  /** @state artifact */
  sheetWElMm3?: number;
  /** @state artifact */
  sheetMEdKnm?: number;
  /** @state artifact */
  shellTMm?: number;
  /** @state artifact */
  shellRMm?: number;
  /** @state artifact */
  sigmaEdShellMpa?: number;
  /** @state artifact */
  annex?: number;
  /** @state presence */
  selectedCheckIndex?: number | null;
}
