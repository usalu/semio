/** 💡️ Wires inference schema — topology (node/edge/component counts + cycle-freedom) over the board graph. */

export interface WiresTopology {
  nodeCount: number;
  edgeCount: number;
  componentCount: number;
  cycleFree: boolean;
}

export interface WiresInference {
  /** @state inferred */
  topology: WiresTopology;
}
