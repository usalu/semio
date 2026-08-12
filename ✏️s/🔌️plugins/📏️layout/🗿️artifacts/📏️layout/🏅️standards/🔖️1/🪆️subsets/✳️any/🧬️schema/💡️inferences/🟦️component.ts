/** 💡️ Layout inference schema — topology (parent-page/spread/page composition order). */

export interface LayoutTopology {
  topoOrder: string[];
  depth: Record<string, number>;
  cycleFree: boolean;
  nodeCount: number;
}

export interface LayoutInference {
  /** @state inferred */
  topology: LayoutTopology;
}
