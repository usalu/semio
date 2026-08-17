/** 📐️ Mathematical editor — Geometry window: typed twin of `🦀️component.rs`'s view-model. Mirrors
 * the Rust `render(geometry: &MathematicalGeometry) -> UiNode` boundary — a flat point cloud, the
 * only input this window's render function reads (the convex-hull/centroid overlay is computed
 * Rust-side inside `geometry_layers_json` and never round-trips back through this typed view-model). */

/** ✏️ One point of the geometry playground's point cloud — mirrors Rust `MathematicalPoint`. */
export interface MathematicalPointViewModel {
  x: number;
  y: number;
}

/** ✏️ The Geometry window's typed view-model — mirrors the Rust `render()` boundary's input. */
export interface MathematicalGeometryViewModel {
  windowKindId: "math-geometry";
  bodyKey: "mathematical.play.geometry";
  points: MathematicalPointViewModel[];
}

export const MATH_PLAY_WINDOW_GEOMETRY = "math-geometry" as const;
export const MATH_PLAY_BODY_GEOMETRY = "mathematical.play.geometry" as const;
