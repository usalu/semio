/** 🧬️ En1994 diff schema — sparse field delta. */

export interface En1994Diff {
  /** @state artifact */
  artifact?: En1994Artifact;
  /** @state artifact */
  annex?: string;
  /** @state artifact */
  mEdKnm?: number;
  /** @state artifact */
  vEdKn?: number;
  /** @state artifact */
  mPla?: number;
  /** @state artifact */
  mPlRd?: number;
  /** @state artifact */
  eta?: number;
  /** @state artifact */
  vLRd?: number;
  /** @state artifact */
  insulationThicknessMm?: number;
  /** @state artifact */
  fireRating?: string;
  /** @state artifact */
  deckType?: string;
  /** @state artifact */
  deltaSigmaMpa?: number;
  /** @state artifact */
  fatigueDetail?: string;
  /** @state artifact */
  dMm?: number;
  /** @state artifact */
  hScMm?: number;
  /** @state artifact */
  fCkMpa?: number;
  /** @state artifact */
  fUMpa?: number;
  /** @state artifact */
  eCmMpa?: number;
  /** @state artifact */
  vEdPerStudKn?: number;
  /** @state artifact */
  spanM?: number;
  /** @state artifact */
  fYMpa?: number;
  /** @state artifact */
  nCyclesStud?: number;
  /** @state artifact */
  deltaTauStudMpa?: number;
  /** @state presence */
  selectedCheckIndex?: number | null;
}

export interface En1994Artifact {
  annex: string;
  mEdKnm: number;
  vEdKn: number;
  mPla: number;
  mPlRd: number;
  eta: number;
  vLRd: number;
  insulationThicknessMm: number;
  fireRating: string;
  deckType: string;
  deltaSigmaMpa: number;
  fatigueDetail: string;
  dMm: number;
  hScMm: number;
  fCkMpa: number;
  fUMpa: number;
  eCmMpa: number;
  vEdPerStudKn: number;
  spanM: number;
  fYMpa: number;
  nCyclesStud: number;
  deltaTauStudMpa: number;
  selectedCheckIndex?: number | null;
}
