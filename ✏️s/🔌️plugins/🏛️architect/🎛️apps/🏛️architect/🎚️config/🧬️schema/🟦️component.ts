/** 🧬️ ArchitectConfig */
export type AdjacencyKind = "required" | "preferred" | "optional" | "prohibited";

export interface ArchitectConfig {
  /** @state local-ui */
  selectedIds: string[];
  /** @state local-ui */
  activeRegister: string;
  /** @state local-ui */
  searchQuery: string;
  /** @state local-ui */
  searchHistoryJson: string;
  /** @state local-ui */
  activeReportJson: string;
  /** @state local-ui */
  lastResultJson: string;
  /** @state local-ui */
  lastAnalysisJson: string;
  /** @state local-ui */
  adjacencyKindFilter?: AdjacencyKind;
  /** @state local-ui */
  graphCameraX: number;
  /** @state local-ui */
  graphCameraY: number;
  /** @state local-ui */
  graphCameraZoom: number;
}
