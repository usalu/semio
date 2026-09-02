/** 💡️ Procedural2d inference schema — topology (DAG shape of `fixture`'s widget/synapse graph). */

export interface Procedural2dTopology {
  nodeCount: number;
  edgeCount: number;
  topoOrder: string[];
  depth: number;
  cycleFree: boolean;
}

export interface Procedural2dInference {
  /** @derived */
  topology: Procedural2dTopology;
}
