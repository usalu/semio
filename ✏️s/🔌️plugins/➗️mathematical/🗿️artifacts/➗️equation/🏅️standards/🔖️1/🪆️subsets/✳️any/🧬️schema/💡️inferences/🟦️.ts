/** 💡️ Equation inference schema — topology (graph topological order). */

export interface EquationTopology {
  topoOrder: string[];
  depth: Record<string, number>;
  cycleFree: boolean;
  nodeCount: number;
}

export interface EquationInference {
  /** @derived */
  topology: EquationTopology;
}
