/** 🧬️ Generation2d diff schema — sparse field delta. */

export interface Generation2dDiff {
  /** @state artifact */
  artifact?: Generation2dArtifact;
  /** @state artifact */
  hostSnapshot?: FlowHostSnapshot;
  /** @state artifact */
  generation?: GenerationPlayState;
}

export type Generation2dStringList = { values: string[] };
export interface Generation2dArtifact { /* see artifact facet */ }

export type CameraJson = { x: number; y: number; zoom: number };
export type WidgetLayout = { x: number; y: number };
export type SynapseSpec = { id: string; from: string; to: string; fromPort: string; toPort: string };
/** 🎛️ One flow widget, internally tagged by `kind` — the flow `Widget` enum's wire form (`tag = "kind"`, camelCase). */
export type Widget =
  | { kind: "neuron"; id: string; neuronKind: string; params: Record<string, unknown>; inputPorts: string[]; outputPorts: string[]; preview: boolean }
  | { kind: "inputSlider"; id: string; label: string; value: number; min: number; max: number; step: number }
  | { kind: "inputNote"; id: string; text: string }
  | { kind: "inputImage"; id: string; src: string }
  | { kind: "variable"; id: string; name: string; schema: string }
  | { kind: "outputPreview"; id: string; preview: Record<string, unknown>; expanded: string[] }
  | { kind: "outputAction"; id: string; action: string }
  | { kind: "outputExport"; id: string; format: string }
  | { kind: "cluster"; id: string; name: string; tree: FlowTree; flow: FlowUi };
export type FlowTree = { neurons: FlowNeuron[]; synapses: SynapseSpec[] };
export type FlowNeuron = { id: string; kind: string; params: Record<string, unknown>; tree: FlowTree | null };
export type FlowUi = { camera: CameraJson; nodes: Record<string, FlowNodeGui>; previews: FlowPreviewGui[] };
export type FlowNodeGui = { layout: WidgetLayout; chrome: NodeChrome };
export type NodeChrome =
  | { kind: "plain"; preview: boolean }
  | { kind: "slider"; label: string; min: number; max: number; step: number; value: number }
  | { kind: "note"; text: string }
  | { kind: "image"; src: string }
  | { kind: "variable"; name: string; schema: string };
export type FlowPreviewGui = { id: string; source: FlowChannelRef | null; mode: string; preview: Record<string, unknown>; expanded: string[]; layout: WidgetLayout | null };
export type FlowChannelRef = { neuron: string; channel: string };
export type FlowHostSnapshot = {
  schema: string;
  camera: CameraJson;
  widgets: Widget[];
  synapses: SynapseSpec[];
  layout: Record<string, WidgetLayout>;
};
export type FormGeneration = { id: string; name: string; values: Record<string, unknown> };
export type GenerationPlayState = {
  generations: FormGeneration[];
  selectedGenerationId?: string;
  previewText?: string;
};
