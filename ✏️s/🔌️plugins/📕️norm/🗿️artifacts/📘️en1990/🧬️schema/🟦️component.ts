/** 🧬️ En1990 artifact schema — every field with its state class. */

export interface En1990Artifact {
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
  /** @state shared-ui */
  selectedCheckIndex?: number | null;
}
