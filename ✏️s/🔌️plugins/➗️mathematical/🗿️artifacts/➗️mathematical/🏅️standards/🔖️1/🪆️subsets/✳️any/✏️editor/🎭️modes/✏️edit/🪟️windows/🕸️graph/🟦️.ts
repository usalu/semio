/** 🕸️ Mathematical editor — Graph window: typed twin of `🦀️.rs`'s view-model. Mirrors the
 * Rust `render(graph: &MathematicalGraph, camera: &MathematicalCamera) -> UiNode` boundary — the
 * editable node-graph document state plus the session-only viewport camera
 * (`crate::editor::mathematical::config::MathematicalConfig.camera`, never a document field). */

/** ✏️ One node of the graph playground — mirrors Rust `MathematicalNode`. */
export interface MathematicalNodeViewModel {
  id: string;
  label: string;
  x: number;
  y: number;
}

/** ✏️ One directed/undirected edge — mirrors Rust `MathematicalEdge`. */
export interface MathematicalEdgeViewModel {
  id: string;
  source: string;
  target: string;
}

/** ✏️ Node-graph viewport camera — mirrors Rust `MathematicalCamera`, session-only config state. */
export interface MathematicalCameraViewModel {
  x: number;
  y: number;
  zoom: number;
}

/** ✏️ The Graph window's typed view-model — mirrors the Rust `render()` boundary's two inputs. */
export interface MathematicalGraphViewModel {
  windowKindId: "math-graph";
  bodyKey: "mathematical.play.graph";
  directed: boolean;
  nodes: MathematicalNodeViewModel[];
  edges: MathematicalEdgeViewModel[];
  algorithm: string;
  algorithmSeed: string | null;
  camera: MathematicalCameraViewModel;
}

export const MATH_PLAY_WINDOW_GRAPH = "math-graph" as const;
export const MATH_PLAY_BODY_GRAPH = "mathematical.play.graph" as const;
