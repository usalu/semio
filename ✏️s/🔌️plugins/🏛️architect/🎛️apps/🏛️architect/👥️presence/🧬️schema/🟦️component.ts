/** 🧬️ ArchitectPresence */
export type AdjacencyKind = "required" | "preferred" | "optional" | "prohibited";

export interface ArchitectPresence {
  /** @state shared-ui */
  selectedIds: string[];
  /** @state shared-ui */
  activeRegister: string;
  /** @state shared-ui */
  adjacencyKindFilter?: AdjacencyKind;
  /** @state shared-ui */
  graphCameraX: number;
  /** @state shared-ui */
  graphCameraY: number;
  /** @state shared-ui */
  graphCameraZoom: number;
}
