/** 🧬️ Vdi3805 snapshot schema — persistent fields only. */

export interface Vdi3805Snapshot {
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
}
