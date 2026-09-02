/** 🧭 `topology` — DAG shape of `fixture`'s widget/synapse graph, derived from `fixture`. */

export interface Procedural2dTopology {
  nodeCount: number;
  edgeCount: number;
  topoOrder: string[];
  depth: number;
  cycleFree: boolean;
}
