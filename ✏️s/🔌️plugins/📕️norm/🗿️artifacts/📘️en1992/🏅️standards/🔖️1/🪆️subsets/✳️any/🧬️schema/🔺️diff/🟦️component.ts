/** 🧬️ En1992 diff schema — sparse field delta. */

export interface En1992Diff {
  /** @state artifact */
  artifact?: En1992Artifact;
  /** @state artifact */
  annex?: string;
  /** @state artifact */
  mEdKnm?: number;
  /** @state artifact */
  vEdKn?: number;
  /** @state artifact */
  fCk?: number;
  /** @state artifact */
  bMm?: number;
  /** @state artifact */
  dMm?: number;
  /** @state artifact */
  aSMm2?: number;
  /** @state artifact */
  fYk?: number;
  /** @state artifact */
  rhoL?: number;
  /** @state artifact */
  nEdKn?: number;
  /** @state artifact */
  pKn?: number;
  /** @state artifact */
  aCMm2?: number;
  /** @state artifact */
  useFem?: boolean;
  /** @state artifact */
  spanM?: number;
  /** @state artifact */
  udlKnM?: number;
  /** @state artifact */
  fireRating?: string;
  /** @state artifact */
  providedAxisDistanceMm?: number;
  /** @state artifact */
  bridgeSigmaCMpa?: number;
  /** @state artifact */
  bridgeDeltaSigmaSMpa?: number;
  /** @state artifact */
  tightnessClass?: string;
  /** @state artifact */
  hdOverH?: number;
  /** @state artifact */
  liquidSigmaSMpa?: number;
  /** @state artifact */
  liquidRhoPEff?: number;
  /** @state artifact */
  liquidFCtEffMpa?: number;
  /** @state artifact */
  liquidESMpa?: number;
  /** @state artifact */
  liquidSRMaxMm?: number;
  /** @state artifact */
  anchorHEfMm?: number;
  /** @state artifact */
  anchorCracked?: boolean;
  /** @state artifact */
  anchorFUkMpa?: number;
  /** @state artifact */
  anchorFYkMpa?: number;
  /** @state artifact */
  anchorASMm2?: number;
  /** @state artifact */
  anchorDMm?: number;
  /** @state artifact */
  anchorC1Mm?: number;
  /** @state artifact */
  anchorNEdKn?: number;
  /** @state artifact */
  anchorVEdKn?: number;
  /** @state presence */
  selectedCheckIndex?: number | null;
}

export interface En1992Artifact {
  annex: string;
  mEdKnm: number;
  vEdKn: number;
  fCk: number;
  bMm: number;
  dMm: number;
  aSMm2: number;
  fYk: number;
  rhoL: number;
  nEdKn: number;
  pKn: number;
  aCMm2: number;
  useFem: boolean;
  spanM: number;
  udlKnM: number;
  fireRating: string;
  providedAxisDistanceMm: number;
  bridgeSigmaCMpa: number;
  bridgeDeltaSigmaSMpa: number;
  tightnessClass: string;
  hdOverH: number;
  liquidSigmaSMpa: number;
  liquidRhoPEff: number;
  liquidFCtEffMpa: number;
  liquidESMpa: number;
  liquidSRMaxMm: number;
  anchorHEfMm: number;
  anchorCracked: boolean;
  anchorFUkMpa: number;
  anchorFYkMpa: number;
  anchorASMm2: number;
  anchorDMm: number;
  anchorC1Mm: number;
  anchorNEdKn: number;
  anchorVEdKn: number;
  selectedCheckIndex?: number | null;
}
