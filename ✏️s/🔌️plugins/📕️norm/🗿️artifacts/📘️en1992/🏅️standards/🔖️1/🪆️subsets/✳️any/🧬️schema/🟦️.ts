/** 🧬️ En1992 artifact schema — every field with its state class. */

export interface En1992Artifact {
  /** @state artifact */
  annex: string;
  /** @state artifact */
  mEdKnm: number;
  /** @state artifact */
  vEdKn: number;
  /** @state artifact */
  fCk: number;
  /** @state artifact */
  bMm: number;
  /** @state artifact */
  dMm: number;
  /** @state artifact */
  aSMm2: number;
  /** @state artifact */
  fYk: number;
  /** @state artifact */
  rhoL: number;
  /** @state artifact */
  nEdKn: number;
  /** @state artifact */
  pKn: number;
  /** @state artifact */
  aCMm2: number;
  /** @state artifact */
  useFem: boolean;
  /** @state artifact */
  spanM: number;
  /** @state artifact */
  udlKnM: number;
  /** @state artifact */
  fireRating: string;
  /** @state artifact */
  providedAxisDistanceMm: number;
  /** @state artifact */
  bridgeSigmaCMpa: number;
  /** @state artifact */
  bridgeDeltaSigmaSMpa: number;
  /** @state artifact */
  tightnessClass: string;
  /** @state artifact */
  hdOverH: number;
  /** @state artifact */
  liquidSigmaSMpa: number;
  /** @state artifact */
  liquidRhoPEff: number;
  /** @state artifact */
  liquidFCtEffMpa: number;
  /** @state artifact */
  liquidESMpa: number;
  /** @state artifact */
  liquidSRMaxMm: number;
  /** @state artifact */
  anchorHEfMm: number;
  /** @state artifact */
  anchorCracked: boolean;
  /** @state artifact */
  anchorFUkMpa: number;
  /** @state artifact */
  anchorFYkMpa: number;
  /** @state artifact */
  anchorASMm2: number;
  /** @state artifact */
  anchorDMm: number;
  /** @state artifact */
  anchorC1Mm: number;
  /** @state artifact */
  anchorNEdKn: number;
  /** @state artifact */
  anchorVEdKn: number;
  /** @state presence */
  selectedCheckIndex?: number | null;
}
