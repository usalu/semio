/** 💡️ Presentation inference schema — topology derived from the persisted tile order. */

export interface PresentationTopology {
  topoOrder: string[];
  depth: Record<string, number>;
  cycleFree: boolean;
  nodeCount: number;
}

export interface PresentationInference {
  /** @derived */
  topology: PresentationTopology;
}
