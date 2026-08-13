/** 🧬️ GIS map diff schema — sparse field delta. */

export interface GisMapDiff {
  /** @state artifact */
  artifact?: GisMapArtifact;
  /** @state artifact */
  positions?: GisMapFeaturesDelta;
  /** @state artifact */
  routes?: GisMapFeaturesDelta;
  /** @state artifact */
  regions?: GisMapFeaturesDelta;
  /** @state presence */
  selectedIds?: GisMapStringList;
  /** @state presence */
  featureSelectionJson?: string;
  /** @state presence */
  layerVisibility?: GisMapBoolMapDelta;
  /** @state presence */
  layerStrokeScale?: GisMapNumberMapDelta;
  /** @state config */
  cameraJson?: string;
  /** @state config */
  renderMode?: string;
  /** @state config */
  vectorStyle?: string;
  /** @state config */
  lodMode?: string;
  /** @state config */
  hoverJson?: string;
  /** @state config */
  selectionMethod?: string;
  /** @state config */
  selectionMode?: string;
  /** @state config */
  locale?: string;
}

export interface GisMapArtifact {
  positions: GisMapFeature[];
  routes: GisMapFeature[];
  regions: GisMapFeature[];
  selectedIds: string[];
  featureSelectionJson: string;
  layerVisibility: Record<string, boolean>;
  layerStrokeScale: Record<string, number>;
  cameraJson: string;
  renderMode: string;
  vectorStyle: string;
  lodMode: string;
  hoverJson: string;
  selectionMethod: string;
  selectionMode: string;
  locale: string;
}

export interface GisMapFeature {
  id: string;
  data: Record<string, unknown>;
}

export interface GisMapStringList { values: string[]; }
export interface GisMapBoolMapDelta { entries: Record<string, boolean | null>; }
export interface GisMapNumberMapDelta { entries: Record<string, number | null>; }
export interface GisMapFeaturesDelta {
  added: GisMapFeature[];
  removed: string[];
  patched: GisMapFeaturePatchEntry[];
  reordered?: string[];
}
export interface GisMapFeaturePatchEntry {
  id: string;
  patch: GisMapFeaturePatch;
}
export interface GisMapFeaturePatch {
  data?: Record<string, unknown>;
}
