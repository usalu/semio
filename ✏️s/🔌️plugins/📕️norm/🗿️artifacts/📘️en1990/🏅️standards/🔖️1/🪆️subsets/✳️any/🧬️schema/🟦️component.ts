/** 🧬️ En1990 artifact schema — every field with its state class. */

export interface En1990Artifact {
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
  /** @state presence */
  selectedCheckIndex?: number | null;
}
