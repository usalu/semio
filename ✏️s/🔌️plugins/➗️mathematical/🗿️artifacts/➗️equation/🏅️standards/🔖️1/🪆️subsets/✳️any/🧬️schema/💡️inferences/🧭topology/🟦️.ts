/** 🧭 `topology` — one named inference: the graph playground's topological order derived from
 * each edge's `source`/`target`. */

export interface EquationTopology {
  topoOrder: string[];
  depth: Record<string, number>;
  cycleFree: boolean;
  nodeCount: number;
}
