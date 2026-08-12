/** 🧭 `topology` — one named inference: execution-order topology stats over the widget/synapse graph. */

export interface FlowTopology {
  topoOrder: string[];
  depth: Record<string, number>;
  cycleFree: boolean;
  nodeCount: number;
}
