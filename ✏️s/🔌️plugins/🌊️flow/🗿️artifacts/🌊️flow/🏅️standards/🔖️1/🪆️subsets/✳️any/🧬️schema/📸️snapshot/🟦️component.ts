/** 🧬️ Flow snapshot schema — persistent fields only. */

export interface FlowSnapshot {
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
