/** 🧭 `topology` — one named inference: graph-shape stats over the wires board's node/edge graph. */

export interface WiresTopology {
  nodeCount: number;
  edgeCount: number;
  componentCount: number;
  cycleFree: boolean;
}
