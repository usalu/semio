/** 🧬️ Flow artifact schema — every field with its state class. */

export interface FlowArtifact {
  /** @state persistent */
  schema: string;
  /** @state persistent */
  camera: CameraJson;
  /** @state persistent */
  widgets: Widget[];
  /** @state persistent */
  synapses: SynapseSpec[];
  /** @state persistent */
  layout: Record<string, WidgetLayout>;
  /** @state shared-ui */
  selectedNodeIds: string[];
  /** @state shared-ui */
  selectedEdgeIds: string[];
  /** @state shared-ui */
  selectedHandleIds: string[];
  /** @state shared-ui */
  previewOffNodeIds: string[];
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

export interface WidgetLayout {
  x: number;
  y: number;
}

export interface SynapseSpec {
  id: string;
  from: string;
  to: string;
  fromPort: string;
  toPort: string;
}

/** Widget payload as JSON text (opaque enum). */
export type Widget = string;
