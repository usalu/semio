/** 🧬️ GIS terrain snapshot schema — persistent fields only. */

export interface GisTerrainSnapshot {
  /** @state persistent */
  exaggeration: number;
  /** @state persistent */
  importedFeaturesJson: string;
}
