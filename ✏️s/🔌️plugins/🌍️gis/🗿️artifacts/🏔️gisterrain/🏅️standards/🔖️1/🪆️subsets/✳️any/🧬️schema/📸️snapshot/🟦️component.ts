/** 🧬️ GIS terrain snapshot schema — persistent fields only. */

export interface GisTerrainSnapshot {
  /** @state artifact */
  exaggeration: number;
  /** @state artifact */
  importedFeaturesJson: string;
}
