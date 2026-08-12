/** 🧭 `topology` — the honest vacuous topology: `PlaygroundSnapshot` carries no domain entities or
 * references yet, so this is always the empty graph (zero nodes, empty order, cycle-free). */

export interface PlaygroundTopology {
  topoOrder: string[];
  depth: Record<string, number>;
  cycleFree: boolean;
  nodeCount: number;
}
