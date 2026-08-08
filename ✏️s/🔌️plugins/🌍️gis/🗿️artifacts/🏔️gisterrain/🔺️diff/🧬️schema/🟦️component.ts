/** 🧬️ GIS terrain diff schema — sparse field delta. */

export interface GisTerrainDiff {
  /** @state persistent */
  artifact?: GisTerrainArtifact;
  /** @state persistent */
  exaggeration?: number;
  /** @state persistent */
  importedFeaturesJson?: string;
  /** @state shared-ui */
  selectedIds?: GisTerrainStringList;
  /** @state local-ui */
  cameraJson?: string;
  /** @state local-ui */
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
