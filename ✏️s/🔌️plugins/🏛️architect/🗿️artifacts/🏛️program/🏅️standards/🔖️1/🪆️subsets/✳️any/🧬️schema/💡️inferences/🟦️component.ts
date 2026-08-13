/** 💡️ Architect program inference schema — topology (hierarchy shape of elements via parentId). */

export interface ProgramTopology {
  nodeCount: number;
  rootCount: number;
  maxDepth: number;
  cycleFree: boolean;
  topoOrder: string[];
}

export interface ProgramInference {
  /** @derived */
  topology: ProgramTopology;
}
