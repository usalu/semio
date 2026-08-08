/** 🧬️ Vdi3805 snapshot schema — persistent fields only. */

export interface Vdi3805Snapshot {
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
}
