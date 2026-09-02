/** 🎥️ Shooting viewer — Scene window: typed twin of `🦀️.rs`'s view-model. Read-only mirror
 * of the world-3d scene payload `render()` produces — no camera/fit mutation-shaped fields (a viewer
 * has no persisted per-session camera), matching the viewer's `ViewEmit`-only contract. */

export interface ShootingViewSceneViewModel {
  windowKindId: "shooting-view-scene";
  bodyKey: "shooting.view.scene";
  surfaceId: "shooting.view.scene3d/scene";
}

export const SHOOTING_VIEW_SCENE_WINDOW_KIND_ID = "shooting-view-scene" as const;
export const SHOOTING_VIEW_SCENE_BODY_KEY = "shooting.view.scene" as const;
export const SHOOTING_VIEW_SCENE_SURFACE_ID = "shooting.view.scene3d/scene" as const;
