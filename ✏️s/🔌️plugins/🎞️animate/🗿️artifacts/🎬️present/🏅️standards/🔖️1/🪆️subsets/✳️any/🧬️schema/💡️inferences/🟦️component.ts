/** 💡️ Present inference schema — topology derived from the persisted tile order. */

export interface PresentTopology {
  topoOrder: string[];
  depth: Record<string, number>;
  cycleFree: boolean;
  nodeCount: number;
}

export interface PresentInference {
  /** @state inferred */
  topology: PresentTopology;
}
