/** 💡️ Sequence inference schema — topology is a real Kahn's-algorithm topological sort over the
 * step DAG (steps + edges). */

export interface SequenceTopology {
  topoOrder: string[];
  depth: Record<string, number>;
  cycleFree: boolean;
  nodeCount: number;
}

export interface SequenceInference {
  /** @state inferred */
  topology: SequenceTopology;
}
