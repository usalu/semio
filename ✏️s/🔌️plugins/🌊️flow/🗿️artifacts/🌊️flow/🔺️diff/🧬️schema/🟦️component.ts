/** 🧬️ Flow diff schema — sparse field delta. */

export interface FlowDiff {
  /** @state persistent */
  artifact?: FlowArtifact;
  /** @state persistent */
  schema?: string;
  /** @state persistent */
  camera?: CameraJson;
  /** @state persistent */
  widgets?: FlowWidgetsDelta;
  /** @state persistent */
  synapses?: FlowSynapsesDelta;
  /** @state persistent */
  layout?: FlowLayoutMapDelta;
  /** @state shared-ui */
  selectedNodeIds?: FlowStringList;
  /** @state shared-ui */
  selectedEdgeIds?: FlowStringList;
  /** @state shared-ui */
  selectedHandleIds?: FlowStringList;
  /** @state shared-ui */
  previewOffNodeIds?: FlowStringList;
  /** @state local-ui */
  lodMode?: string;
  /** @state local-ui */
  proximityDistance?: number;
  /** @state local-ui */
  gridVisible?: boolean;
  /** @state local-ui */
  gridSnapEnabled?: boolean;
  /** @state local-ui */
  gridFactor?: number;
  /** @state local-ui */
  catalogueSectionsJson?: string;
  /** @state local-ui */
  automationEnabledJson?: string;
  /** @state local-ui */
  contributionsJson?: string;
  /** @state local-ui */
  generationJson?: string;
  /** @state local-ui */
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
