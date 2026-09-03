/** 💡️ Generation3d inference schema — topology (DAG shape of `fixture`'s widget/synapse graph). */

export interface Generation3dTopology {
  nodeCount: number;
  edgeCount: number;
  topoOrder: string[];
  depth: number;
  cycleFree: boolean;
}

export interface Generation3dInference {
  /** @derived */
  topology: Generation3dTopology;
}
