/** 🧬️ Flow artifact schema — every field with its state class. */

export interface FlowArtifact {
  /** @state artifact */
  schema: string;
  /** @state artifact */
  camera: CameraJson;
  /** @state artifact */
  widgets: Widget[];
  /** @state artifact */
  synapses: SynapseSpec[];
  /** @state artifact */
  layout: Record<string, WidgetLayout>;
  /** @state presence */
  selectedNodeIds: string[];
  /** @state presence */
  selectedEdgeIds: string[];
  /** @state presence */
  selectedHandleIds: string[];
  /** @state presence */
  previewOffNodeIds: string[];
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
