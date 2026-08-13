/** 🧬️ ArchitectPresence */
export type AdjacencyKind = "required" | "preferred" | "optional" | "prohibited";

export interface ArchitectPresence {
  /** @state presence */
  selectedIds: string[];
  /** @state presence */
  activeRegister: string;
  /** @state presence */
  adjacencyKindFilter?: AdjacencyKind;
  /** @state presence */
  graphCameraX: number;
  /** @state presence */
  graphCameraY: number;
  /** @state presence */
  graphCameraZoom: number;
}
