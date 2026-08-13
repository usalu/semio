/** 🧬️ En1990 diff schema — sparse field delta. */

export interface En1990Diff {
  /** @state artifact */
  artifact?: En1990Artifact;
  /** @state artifact */
  gK?: number;
  /** @state artifact */
  qK?: En1990QkList;
  /** @state artifact */
  resistanceKn?: number;
  /** @state artifact */
  consequenceClass?: number;
  /** @state artifact */
  annex?: string;
  /** @state artifact */
  seismicAEdKn?: number;
  /** @state presence */
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
