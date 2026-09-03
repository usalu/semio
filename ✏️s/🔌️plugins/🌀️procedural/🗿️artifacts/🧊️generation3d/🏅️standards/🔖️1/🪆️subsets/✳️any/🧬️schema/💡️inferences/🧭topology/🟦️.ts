/** 🧭 `topology` — DAG shape of `fixture`'s widget/synapse graph, derived from `fixture`. */

export interface Generation3dTopology {
  nodeCount: number;
  edgeCount: number;
  topoOrder: string[];
  depth: number;
  cycleFree: boolean;
}
