/** 🧬️ En1994 diff schema — sparse field delta. */

export interface En1994Diff {
  /** @state persistent */
  artifact?: En1994Artifact;
  /** @state persistent */
  annex?: string;
  /** @state persistent */
  mEdKnm?: number;
  /** @state persistent */
  vEdKn?: number;
  /** @state persistent */
  mPla?: number;
  /** @state persistent */
  mPlRd?: number;
  /** @state persistent */
  eta?: number;
  /** @state persistent */
  vLRd?: number;
  /** @state persistent */
  insulationThicknessMm?: number;
  /** @state persistent */
  fireRating?: string;
  /** @state persistent */
  deckType?: string;
  /** @state persistent */
  deltaSigmaMpa?: number;
  /** @state persistent */
  fatigueDetail?: string;
  /** @state persistent */
  dMm?: number;
  /** @state persistent */
  hScMm?: number;
  /** @state persistent */
  fCkMpa?: number;
  /** @state persistent */
  fUMpa?: number;
  /** @state persistent */
  eCmMpa?: number;
  /** @state persistent */
  vEdPerStudKn?: number;
  /** @state persistent */
  spanM?: number;
  /** @state persistent */
  fYMpa?: number;
  /** @state persistent */
  nCyclesStud?: number;
  /** @state persistent */
  deltaTauStudMpa?: number;
  /** @state shared-ui */
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
