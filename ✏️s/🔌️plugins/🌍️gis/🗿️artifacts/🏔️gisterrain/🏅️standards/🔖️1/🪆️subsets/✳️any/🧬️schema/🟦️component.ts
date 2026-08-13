/** 🧬️ GIS terrain artifact schema — every field with its state class. */

export interface GisTerrainArtifact {
  /** @state artifact */
  exaggeration: number;
  /** @state artifact */
  importedFeaturesJson: string;
  /** @state presence */
  selectedIds: string[];
  /** @state config */
  cameraJson: string;
  /** @state config */
  locale: string;
}
