/** 🧬️ GIS map artifact schema — every field with its state class. */

export interface GisMapArtifact {
  /** @state artifact */
  positions: GisMapFeature[];
  /** @state artifact */
  routes: GisMapFeature[];
  /** @state artifact */
  regions: GisMapFeature[];
  /** @state presence */
  selectedIds: string[];
  /** @state presence */
  featureSelectionJson: string;
  /** @state presence */
  layerVisibility: Record<string, boolean>;
  /** @state presence */
  layerStrokeScale: Record<string, number>;
  /** @state config */
  cameraJson: string;
  /** @state config */
  renderMode: string;
  /** @state config */
  vectorStyle: string;
  /** @state config */
  lodMode: string;
  /** @state config */
  hoverJson: string;
  /** @state config */
  selectionMethod: string;
  /** @state config */
  selectionMode: string;
  /** @state config */
  locale: string;
}

export interface GisMapFeature {
  id: string;
  data: Record<string, unknown>;
}
