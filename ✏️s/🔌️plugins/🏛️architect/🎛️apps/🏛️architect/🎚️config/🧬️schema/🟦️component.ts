/** 🧬️ ArchitectConfig */
export type AdjacencyKind = "required" | "preferred" | "optional" | "prohibited";

export interface ArchitectConfig {
  /** @state config */
  selectedIds: string[];
  /** @state config */
  activeRegister: string;
  /** @state config */
  searchQuery: string;
  /** @state config */
  searchHistoryJson: string;
  /** @state config */
  activeReportJson: string;
  /** @state config */
  lastResultJson: string;
  /** @state config */
  lastAnalysisJson: string;
  /** @state config */
  adjacencyKindFilter?: AdjacencyKind;
  /** @state config */
  graphCameraX: number;
  /** @state config */
  graphCameraY: number;
  /** @state config */
  graphCameraZoom: number;
}
