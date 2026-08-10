/** 🧬️ En1992 snapshot schema — persistent fields only. */

export interface En1992Snapshot {
  /** @state persistent */
  annex: string;
  /** @state persistent */
  mEdKnm: number;
  /** @state persistent */
  vEdKn: number;
  /** @state persistent */
  fCk: number;
  /** @state persistent */
  bMm: number;
  /** @state persistent */
  dMm: number;
  /** @state persistent */
  aSMm2: number;
  /** @state persistent */
  fYk: number;
  /** @state persistent */
  rhoL: number;
  /** @state persistent */
  nEdKn: number;
  /** @state persistent */
  pKn: number;
  /** @state persistent */
  aCMm2: number;
  /** @state persistent */
  useFem: boolean;
  /** @state persistent */
  spanM: number;
  /** @state persistent */
  udlKnM: number;
  /** @state persistent */
  fireRating: string;
  /** @state persistent */
  providedAxisDistanceMm: number;
  /** @state persistent */
  bridgeSigmaCMpa: number;
  /** @state persistent */
  bridgeDeltaSigmaSMpa: number;
  /** @state persistent */
  tightnessClass: string;
  /** @state persistent */
  hdOverH: number;
  /** @state persistent */
  liquidSigmaSMpa: number;
  /** @state persistent */
  liquidRhoPEff: number;
  /** @state persistent */
  liquidFCtEffMpa: number;
  /** @state persistent */
  liquidESMpa: number;
  /** @state persistent */
  liquidSRMaxMm: number;
  /** @state persistent */
  anchorHEfMm: number;
  /** @state persistent */
  anchorCracked: boolean;
  /** @state persistent */
  anchorFUkMpa: number;
  /** @state persistent */
  anchorFYkMpa: number;
  /** @state persistent */
  anchorASMm2: number;
  /** @state persistent */
  anchorDMm: number;
  /** @state persistent */
  anchorC1Mm: number;
  /** @state persistent */
  anchorNEdKn: number;
  /** @state persistent */
  anchorVEdKn: number;
}
