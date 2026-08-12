/** 🧬️ En1990 diff schema — sparse field delta. */

export interface En1990Diff {
  /** @state persistent */
  artifact?: En1990Artifact;
  /** @state persistent */
  gK?: number;
  /** @state persistent */
  qK?: En1990QkList;
  /** @state persistent */
  resistanceKn?: number;
  /** @state persistent */
  consequenceClass?: number;
  /** @state persistent */
  annex?: string;
  /** @state persistent */
  seismicAEdKn?: number;
  /** @state shared-ui */
  selectedCheckIndex?: number | null;
}

export interface En1990Artifact {
  gK: number;
  qK: qK[];
  resistanceKn: number;
  consequenceClass: number;
  annex: string;
  seismicAEdKn: number;
  selectedCheckIndex?: number | null;
}

export interface En1990QkEntry { category: string; value: number; }
export interface En1990QkList { values: En1990QkEntry[]; }
