/** 🧬️ GIS map diff schema — sparse field delta. */

export interface GisMapDiff {
  /** @state persistent */
  artifact?: GisMapArtifact;
  /** @state persistent */
  positions?: GisMapFeaturesDelta;
  /** @state persistent */
  routes?: GisMapFeaturesDelta;
  /** @state persistent */
  regions?: GisMapFeaturesDelta;
  /** @state shared-ui */
  selectedIds?: GisMapStringList;
  /** @state shared-ui */
  featureSelectionJson?: string;
  /** @state shared-ui */
  layerVisibility?: GisMapBoolMapDelta;
  /** @state shared-ui */
  layerStrokeScale?: GisMapNumberMapDelta;
  /** @state local-ui */
  cameraJson?: string;
  /** @state local-ui */
  renderMode?: string;
  /** @state local-ui */
  vectorStyle?: string;
  /** @state local-ui */
  lodMode?: string;
  /** @state local-ui */
  hoverJson?: string;
  /** @state local-ui */
  selectionMethod?: string;
  /** @state local-ui */
  selectionMode?: string;
  /** @state local-ui */
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
