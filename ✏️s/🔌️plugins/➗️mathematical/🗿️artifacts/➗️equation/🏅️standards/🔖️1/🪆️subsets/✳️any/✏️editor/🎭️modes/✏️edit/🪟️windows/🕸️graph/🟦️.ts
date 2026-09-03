/** 🕸️ Equation editor — Graph window: typed twin of `🦀️.rs`'s view-model. Mirrors the
 * Rust `render(graph: &EquationGraph, camera: &EquationCamera) -> UiNode` boundary — the
 * editable node-graph document state plus the session-only viewport camera
 * (`crate::editor::equation::config::EquationConfig.camera`, never a document field). */

/** ✏️ One node of the graph playground — mirrors Rust `EquationNode`. */
export interface EquationNodeViewModel {
  id: string;
  label: string;
  x: number;
  y: number;
}

/** ✏️ One directed/undirected edge — mirrors Rust `EquationEdge`. */
export interface EquationEdgeViewModel {
  id: string;
  source: string;
  target: string;
}

/** ✏️ Node-graph viewport camera — mirrors Rust `EquationCamera`, session-only config state. */
export interface EquationCameraViewModel {
  x: number;
  y: number;
  zoom: number;
}

/** ✏️ The Graph window's typed view-model — mirrors the Rust `render()` boundary's two inputs. */
export interface EquationGraphViewModel {
  windowKindId: "math-graph";
  bodyKey: "equation.play.graph";
  directed: boolean;
  nodes: EquationNodeViewModel[];
  edges: EquationEdgeViewModel[];
  algorithm: string;
  algorithmSeed: string | null;
  camera: EquationCameraViewModel;
}

export const MATH_PLAY_WINDOW_GRAPH = "math-graph" as const;
export const MATH_PLAY_BODY_GRAPH = "equation.play.graph" as const;
