/** 📐️ Equation editor — Geometry window: typed twin of `🦀️.rs`'s view-model. Mirrors
 * the Rust `render(geometry: &EquationGeometry) -> UiNode` boundary — a flat point cloud, the
 * only input this window's render function reads (the convex-hull/centroid overlay is computed
 * Rust-side inside `geometry_layers_json` and never round-trips back through this typed view-model). */

/** ✏️ One point of the geometry playground's point cloud — mirrors Rust `EquationPoint`. */
export interface EquationPointViewModel {
  x: number;
  y: number;
}

/** ✏️ The Geometry window's typed view-model — mirrors the Rust `render()` boundary's input. */
export interface EquationGeometryViewModel {
  windowKindId: "math-geometry";
  bodyKey: "equation.play.geometry";
  points: EquationPointViewModel[];
}

export const MATH_PLAY_WINDOW_GEOMETRY = "math-geometry" as const;
export const MATH_PLAY_BODY_GEOMETRY = "equation.play.geometry" as const;
