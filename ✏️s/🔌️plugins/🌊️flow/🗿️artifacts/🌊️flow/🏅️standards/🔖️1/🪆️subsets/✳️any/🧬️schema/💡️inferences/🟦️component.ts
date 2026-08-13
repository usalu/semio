/** 💡️ Flow inference schema — topology (topological order + depth + cycle-freedom) over widgets/synapses. */

export interface FlowTopology {
  topoOrder: string[];
  depth: Record<string, number>;
  cycleFree: boolean;
  nodeCount: number;
}

export interface FlowInference {
  /** @derived */
  topology: FlowTopology;
}
