/** 💡️ Mathematical inference schema — topology (graph topological order). */

export interface MathematicalTopology {
  topoOrder: string[];
  depth: Record<string, number>;
  cycleFree: boolean;
  nodeCount: number;
}

export interface MathematicalInference {
  /** @state inferred */
  topology: MathematicalTopology;
}
