/** 🧬️ Gis2dConfig */
export interface Gis2dConfig {
  /** @state local-ui */
  selectedIds: string[];
  /** @state local-ui */
  layerVisibility: Record<string, boolean>;
  /** @state local-ui */
  cameraJson: string;
  /** @state local-ui */
  renderMode: string;
  /** @state local-ui */
  vectorStyle: string;
  /** @state local-ui */
  lodMode: string;
  /** @state local-ui */
  featureSelectionJson: string;
  /** @state local-ui */
  hoverJson: string;
  /** @state local-ui */
  selectionMethod: string;
  /** @state local-ui */
  selectionMode: string;
  /** @state local-ui */
  layerStrokeScale: Record<string, number>;
  /** @state local-ui */
  locale: string;
}
