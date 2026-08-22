/** 🧬️ FlowConfig */
export interface FlowConfig {
  /** @state config */
  previewOffNodeIds: string[];
  /** @state config */
  camera: CameraJson;
  /** @state config */
  lodMode: string;
  /** @state config */
  proximityDistance: number;
  /** @state config */
  gridVisible: boolean;
  /** @state config */
  gridSnapEnabled: boolean;
  /** @state config */
  gridFactor: number;
  /** @state config */
  catalogueSectionsJson: string;
  /** @state config */
  automationEnabledJson: string;
  /** @state config */
  contributionsJson: string;
  /** @state config */
  generationJson: string;
  /** @state config */
  duplicateWidgetProgressJson: string;
  /** @state config */
  locale: string;
}

export interface CameraJson {
  x: number;
  y: number;
  zoom: number;
}
