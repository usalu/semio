/** 🧭 `topology` — one named inference: step/block dependency order derived from document order
 * plus `condition` var-reference edges between blocks. */

export interface FormsTopology {
  topoOrder: string[];
  depth: Record<string, number>;
  cycleFree: boolean;
  nodeCount: number;
}
