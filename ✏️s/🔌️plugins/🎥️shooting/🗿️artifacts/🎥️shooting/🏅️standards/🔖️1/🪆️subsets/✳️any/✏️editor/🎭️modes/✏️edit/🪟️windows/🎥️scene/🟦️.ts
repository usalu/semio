/** 🎥️ Shooting editor — Scene window: typed twin of `🦀️.rs`'s view-model. Mirrors the
 * window's `render(snapshot: &ShootingSnapshot, cfg: &ShootingConfig)` boundary — the world-3d scene
 * payload plus the live viewport camera / center-model fit state a mutation-capable surface carries
 * (both absent entirely from the viewer's read-only twin, see `👁️viewer/…/🟦️.ts`). */

/** ✏️ The session-only, mutation-capable viewport camera — mirrors Rust `ShootingCamera`. */
export interface ShootingCamera {
  position: [number, number, number];
  target: [number, number, number];
  zoom: number;
  fov: number;
  up?: [number, number, number];
  projection?: string;
}

/** ✏️ The Scene window's typed view-model — mirrors the Rust `render()` boundary's inputs. */
export interface ShootingSceneViewModel {
  windowKindId: "shooting-scene";
  bodyKey: "shooting.play.scene";
  surfaceId: "shooting.play.scene3d/scene";
  camera: ShootingCamera;
  centerModel: boolean;
  fitRevision: number;
}

export const SHOOTING_PLAY_SCENE_WINDOW_KIND_ID = "shooting-scene" as const;
export const SHOOTING_PLAY_SCENE_BODY_KEY = "shooting.play.scene" as const;
export const SHOOTING_PLAY_SCENE_SURFACE_ID = "shooting.play.scene3d/scene" as const;
