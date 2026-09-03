/** 💡️ Generation2d inference schema — topology (DAG shape of `fixture`'s widget/synapse graph). */

export interface Generation2dTopology {
  nodeCount: number;
  edgeCount: number;
  topoOrder: string[];
  depth: number;
  cycleFree: boolean;
}

export interface Generation2dInference {
  /** @derived */
  topology: Generation2dTopology;
}
