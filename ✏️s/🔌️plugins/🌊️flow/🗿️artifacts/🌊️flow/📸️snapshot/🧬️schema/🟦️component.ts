/** 🧬️ Flow snapshot schema — persistent fields only. */

export interface FlowSnapshot {
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
