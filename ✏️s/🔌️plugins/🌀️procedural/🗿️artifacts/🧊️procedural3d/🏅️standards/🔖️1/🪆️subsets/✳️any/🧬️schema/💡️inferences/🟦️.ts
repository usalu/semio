/** 💡️ Procedural3d inference schema — topology (DAG shape of `fixture`'s widget/synapse graph). */

export interface Procedural3dTopology {
  nodeCount: number;
  edgeCount: number;
  topoOrder: string[];
  depth: number;
  cycleFree: boolean;
}

export interface Procedural3dInference {
  /** @derived */
  topology: Procedural3dTopology;
}
