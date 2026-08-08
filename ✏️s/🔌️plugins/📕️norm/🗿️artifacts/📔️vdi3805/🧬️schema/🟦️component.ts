/** 🧬️ Vdi3805 artifact schema — every field with its state class. */

export interface Vdi3805Artifact {
  /** @state persistent */
  manufacturerFile: string;
  /** @state persistent */
  catalog: string;
  /** @state persistent */
  editionProfile: Record<string, string>;
  /** @state persistent */
  correctionAsOf: string;
  /** @state persistent */
  strictMode: boolean;
  /** @state persistent */
  index: string;
  /** @state persistent */
  geometry: Record<string, string>;
  /** @state persistent */
  curves: Record<string, string>;
  /** @state persistent */
  limits: string;
  /** @state shared-ui */
  selectedCheckIndex?: number | null;
}
