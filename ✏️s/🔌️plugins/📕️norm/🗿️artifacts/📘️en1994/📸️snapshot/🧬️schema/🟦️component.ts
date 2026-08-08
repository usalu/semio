/** 🧬️ En1994 snapshot schema — persistent fields only. */

export interface En1994Snapshot {
  /** @state persistent */
  annex: string;
  /** @state persistent */
  mEdKnm: number;
  /** @state persistent */
  vEdKn: number;
  /** @state persistent */
  mPla: number;
  /** @state persistent */
  mPlRd: number;
  /** @state persistent */
  eta: number;
  /** @state persistent */
  vLRd: number;
  /** @state persistent */
  insulationThicknessMm: number;
  /** @state persistent */
  fireRating: string;
  /** @state persistent */
  deckType: string;
  /** @state persistent */
  deltaSigmaMpa: number;
  /** @state persistent */
  fatigueDetail: string;
  /** @state persistent */
  dMm: number;
  /** @state persistent */
  hScMm: number;
  /** @state persistent */
  fCkMpa: number;
  /** @state persistent */
  fUMpa: number;
  /** @state persistent */
  eCmMpa: number;
  /** @state persistent */
  vEdPerStudKn: number;
  /** @state persistent */
  spanM: number;
  /** @state persistent */
  fYMpa: number;
  /** @state persistent */
  nCyclesStud: number;
  /** @state persistent */
  deltaTauStudMpa: number;
}
