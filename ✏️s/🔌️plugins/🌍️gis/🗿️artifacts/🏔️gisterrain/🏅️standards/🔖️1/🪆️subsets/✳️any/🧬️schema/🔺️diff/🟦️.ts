/** 🧬️ GIS terrain diff schema — sparse field delta. */

export interface GisTerrainDiff {
  /** @state artifact */
  artifact?: GisTerrainArtifact;
  /** @state artifact */
  exaggeration?: number;
  /** @state artifact */
  importedFeaturesJson?: string;
  /** @state presence */
  selectedIds?: GisTerrainStringList;
  /** @state config */
  cameraJson?: string;
  /** @state config */
  locale?: string;
}

export interface GisTerrainArtifact {
  exaggeration: number;
  importedFeaturesJson: string;
  selectedIds: string[];
  cameraJson: string;
  locale: string;
}

export interface GisTerrainStringList {
  values: string[];
}
