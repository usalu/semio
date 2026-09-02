/** 🧭 `topology` — a real topological sort over `steps` + `edges` (Kahn's algorithm): topoOrder is
 * a valid topological order when the graph is acyclic (else every step in persisted order, so the
 * result stays total), depth is each step's longest path length from any root, cycleFree is
 * whether every step was reachable by the algorithm. */

export interface SequenceTopology {
  topoOrder: string[];
  depth: Record<string, number>;
  cycleFree: boolean;
  nodeCount: number;
}
