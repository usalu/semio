/** 🧭 `topology` — one named inference: the tile filmstrip's persisted order recast as a trivial
 * topology (topoOrder == tile ids in order, depth == each tile's index, always cycleFree). */

export interface PresentationTopology {
  topoOrder: string[];
  depth: Record<string, number>;
  cycleFree: boolean;
  nodeCount: number;
}
