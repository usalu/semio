/** 🌐️ Lowpoly viewer — Model window: typed twin of `🦀️component.rs`'s view-model. Read-only mirror
 * of the shared `framework.window.mesh` scene payload `render()` produces — no mutation-shaped fields
 * (no selection, no gumball, no engagement session), matching the viewer's `ViewEmit`-only contract. */

/** 👁️ One instance placed in the world-3d scene — real per-object transform/label read straight off
 * `LowpolySnapshot.objects`. Geometry is always the shared fallback-box placeholder mesh id. */
export interface LowpolyViewModelInstance {
  id: string;
  meshId: "box";
  position: [number, number, number];
  rotation: [number, number, number, number];
  scale: [number, number, number];
  label: string;
  smoothShading: boolean;
}

/** 👁️ The Model window's typed view-model — the TS mirror of the Rust `render()` boundary's inputs
 * (a bare `LowpolySnapshot`, no runtime/config/utility state: a viewer has none of those). */
export interface LowpolyViewModelViewModel {
  windowKindId: "framework.window.mesh";
  bodyKey: "framework.window.mesh";
  instances: LowpolyViewModelInstance[];
}

export const LOWPOLY_VIEW_MODEL_WINDOW_KIND_ID = "framework.window.mesh" as const;
export const LOWPOLY_VIEW_MODEL_BODY_KEY = "framework.window.mesh" as const;
