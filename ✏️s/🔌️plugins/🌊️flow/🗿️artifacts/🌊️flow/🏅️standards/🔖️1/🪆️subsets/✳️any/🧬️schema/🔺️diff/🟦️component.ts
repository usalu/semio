/** 🧬️ Flow diff schema — sparse field delta. */

export interface FlowDiff {
  /** @state artifact */
  artifact?: FlowArtifact;
  /** @state artifact */
  schema?: string;
  /** @state artifact */
  camera?: CameraJson;
  /** @state artifact */
  widgets?: FlowWidgetsDelta;
  /** @state artifact */
  synapses?: FlowSynapsesDelta;
  /** @state artifact */
  layout?: FlowLayoutMapDelta;
  /** @state presence */
  selectedNodeIds?: FlowStringList;
  /** @state presence */
  selectedEdgeIds?: FlowStringList;
  /** @state presence */
  selectedHandleIds?: FlowStringList;
  /** @state presence */
  previewOffNodeIds?: FlowStringList;
  /** @state config */
  lodMode?: string;
  /** @state config */
  proximityDistance?: number;
  /** @state config */
  gridVisible?: boolean;
  /** @state config */
  gridSnapEnabled?: boolean;
  /** @state config */
  gridFactor?: number;
  /** @state config */
  catalogueSectionsJson?: string;
  /** @state config */
  automationEnabledJson?: string;
  /** @state config */
  contributionsJson?: string;
  /** @state config */
  generationJson?: string;
  /** @state config */
  locale?: string;
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

export interface FlowArtifact {
  schema: string;
  camera: CameraJson;
  widgets: Widget[];
  synapses: SynapseSpec[];
  layout: Record<string, WidgetLayout>;
  selectedNodeIds: string[];
  selectedEdgeIds: string[];
  selectedHandleIds: string[];
  previewOffNodeIds: string[];
  lodMode: string;
  proximityDistance: number;
  gridVisible: boolean;
  gridSnapEnabled: boolean;
  gridFactor: number;
  catalogueSectionsJson: string;
  automationEnabledJson: string;
  contributionsJson: string;
  generationJson: string;
  locale: string;
}

export interface FlowStringList {
  values: string[];
}

export interface FlowWidgetPatchEntry {
  id: string;
  patch: Widget;
}

export interface FlowWidgetsDelta {
  added: Widget[];
  removed: string[];
  patched: FlowWidgetPatchEntry[];
  reordered?: string[];
}

export interface FlowSynapsePatchEntry {
  id: string;
  patch: SynapseSpec;
}

export interface FlowSynapsesDelta {
  added: SynapseSpec[];
  removed: string[];
  patched: FlowSynapsePatchEntry[];
  reordered?: string[];
}

export interface FlowLayoutMapDelta {
  entries: Record<string, WidgetLayout | null>;
}
