/** 🧭 `topology` — one named inference: the hierarchy shape of `elements` (via `parentId`). */

export interface ProgramTopology {
  nodeCount: number;
  rootCount: number;
  maxDepth: number;
  cycleFree: boolean;
  topoOrder: string[];
}
