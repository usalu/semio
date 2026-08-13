/** 🧬️ En1990 snapshot schema — persistent fields only. */

export interface En1990Snapshot {
  /** @state artifact */
  gK: number;
  /** @state artifact */
  qK: qK[];
  /** @state artifact */
  resistanceKn: number;
  /** @state artifact */
  consequenceClass: number;
  /** @state artifact */
  annex: string;
  /** @state artifact */
  seismicAEdKn: number;
}
