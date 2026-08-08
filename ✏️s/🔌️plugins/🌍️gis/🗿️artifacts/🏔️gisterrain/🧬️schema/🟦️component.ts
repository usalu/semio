/** 🧬️ GIS terrain artifact schema — every field with its state class. */

export interface GisTerrainArtifact {
  /** @state persistent */
  exaggeration: number;
  /** @state persistent */
  importedFeaturesJson: string;
  /** @state shared-ui */
  selectedIds: string[];
  /** @state local-ui */
  cameraJson: string;
  /** @state local-ui */
  locale: string;
}
