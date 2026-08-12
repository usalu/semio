/** 💡️ Playground inference schema — topology is always the vacuous empty graph today (no domain
 * entities exist yet on `PlaygroundSnapshot`). */

export interface PlaygroundTopology {
  topoOrder: string[];
  depth: Record<string, number>;
  cycleFree: boolean;
  nodeCount: number;
}

export interface PlaygroundInference {
  /** @state inferred */
  topology: PlaygroundTopology;
}
