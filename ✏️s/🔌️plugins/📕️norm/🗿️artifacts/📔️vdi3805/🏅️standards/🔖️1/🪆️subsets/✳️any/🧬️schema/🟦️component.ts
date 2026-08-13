/** 🧬️ Vdi3805 artifact schema — every field with its state class. */

export interface Vdi3805Artifact {
  /** @state artifact */
  manufacturerFile: string;
  /** @state artifact */
  catalog: string;
  /** @state artifact */
  editionProfile: Record<string, string>;
  /** @state artifact */
  correctionAsOf: string;
  /** @state artifact */
  strictMode: boolean;
  /** @state artifact */
  index: string;
  /** @state artifact */
  geometry: Record<string, string>;
  /** @state artifact */
  curves: Record<string, string>;
  /** @state artifact */
  limits: string;
  /** @state presence */
  selectedCheckIndex?: number | null;
}
