/** 🧬️ EN 1999 diff schema. */

export interface En1999Diff {
  /** @state persistent */
  artifact?: En1999Artifact;
  /** @state persistent */
  nEdKn?: number;
  /** @state persistent */
  mEdKnm?: number;
  /** @state persistent */
  aMm2?: number;
  /** @state persistent */
  wElMm3?: number;
  /** @state persistent */
  alloy?: number;
  /** @state persistent */
  chi?: number;
  /** @state persistent */
  iTMm4?: number;
  /** @state persistent */
  lCrMm?: number;
  /** @state persistent */
  thetaC?: number;
  /** @state persistent */
  deltaSigmaEd?: number;
  /** @state persistent */
  deltaSigmaC?: number;
  /** @state persistent */
  fatigueM?: number;
  /** @state persistent */
  nCycles?: number;
  /** @state persistent */
  vWeldEdKn?: number;
  /** @state persistent */
  weldThroatMm?: number;
  /** @state persistent */
  weldLengthMm?: number;
  /** @state persistent */
  betaW?: number;
  /** @state persistent */
  sheetBMm?: number;
  /** @state persistent */
  sheetTMm?: number;
  /** @state persistent */
  sheetKSigma?: number;
  /** @state persistent */
  sheetWElMm3?: number;
  /** @state persistent */
  sheetMEdKnm?: number;
  /** @state persistent */
  shellTMm?: number;
  /** @state persistent */
  shellRMm?: number;
  /** @state persistent */
  sigmaEdShellMpa?: number;
  /** @state persistent */
  annex?: number;
  /** @state shared-ui */
  selectedCheckIndex?: number | null | null;
}
