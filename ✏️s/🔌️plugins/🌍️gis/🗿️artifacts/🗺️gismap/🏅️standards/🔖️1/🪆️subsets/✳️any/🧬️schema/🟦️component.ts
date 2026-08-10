/** 🧬️ GIS map artifact schema — every field with its state class. */

export interface GisMapArtifact {
  /** @state persistent */
  positions: GisMapFeature[];
  /** @state persistent */
  routes: GisMapFeature[];
  /** @state persistent */
  regions: GisMapFeature[];
  /** @state shared-ui */
  selectedIds: string[];
  /** @state shared-ui */
  featureSelectionJson: string;
  /** @state shared-ui */
  layerVisibility: Record<string, boolean>;
  /** @state shared-ui */
  layerStrokeScale: Record<string, number>;
  /** @state local-ui */
  cameraJson: string;
  /** @state local-ui */
  renderMode: string;
  /** @state local-ui */
  vectorStyle: string;
  /** @state local-ui */
  lodMode: string;
  /** @state local-ui */
  hoverJson: string;
  /** @state local-ui */
  selectionMethod: string;
  /** @state local-ui */
  selectionMode: string;
  /** @state local-ui */
  locale: string;
}

export interface GisMapFeature {
  id: string;
  data: Record<string, unknown>;
}
