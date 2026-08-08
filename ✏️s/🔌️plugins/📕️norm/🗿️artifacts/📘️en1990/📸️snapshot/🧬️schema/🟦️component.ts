/** 🧬️ En1990 snapshot schema — persistent fields only. */

export interface En1990Snapshot {
  /** @state persistent */
  gK: number;
  /** @state persistent */
  qK: qK[];
  /** @state persistent */
  resistanceKn: number;
  /** @state persistent */
  consequenceClass: number;
  /** @state persistent */
  annex: string;
  /** @state persistent */
  seismicAEdKn: number;
}
