/** 🧬️ FlowConfig */
export interface FlowConfig {
  /** @state local-ui */
  selectedNodeIds: string[];
  /** @state local-ui */
  selectedEdgeIds: string[];
  /** @state local-ui */
  selectedHandleIds: string[];
  /** @state local-ui */
  previewOffNodeIds: string[];
  /** @state local-ui */
  camera: CameraJson;
  /** @state local-ui */
  lodMode: string;
  /** @state local-ui */
  proximityDistance: number;
  /** @state local-ui */
  gridVisible: boolean;
  /** @state local-ui */
  gridSnapEnabled: boolean;
  /** @state local-ui */
  gridFactor: number;
  /** @state local-ui */
  catalogueSectionsJson: string;
  /** @state local-ui */
  automationEnabledJson: string;
  /** @state local-ui */
  contributionsJson: string;
  /** @state local-ui */
  generationJson: string;
  /** @state local-ui */
  locale: string;
}

export interface CameraJson {
  x: number;
  y: number;
  zoom: number;
}
