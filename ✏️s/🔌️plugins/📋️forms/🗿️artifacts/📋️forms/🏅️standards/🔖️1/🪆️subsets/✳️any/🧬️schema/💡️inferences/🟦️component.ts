/** 💡️ Forms inference schema — topology (step/block dependency order). */

export interface FormsTopology {
  topoOrder: string[];
  depth: Record<string, number>;
  cycleFree: boolean;
  nodeCount: number;
}

export interface FormsInference {
  /** @derived */
  topology: FormsTopology;
}
