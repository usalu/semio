/** 🪚️ Process 3D editor — Workpiece window: typed twin of `🦀️.rs`'s view-model. Mirrors
 * the window's `render(fixture: &Process3dSnapshot, config: &Process3dConfig)` boundary — the
 * world-3d scene payload (camera + sun) plus the mutation-capable active-utility state a
 * mutation-capable surface carries (absent from the viewer's read-only twin, see
 * `👁️viewer/…/🟦️.ts`). */

/** ✏️ The live camera state driving `world3d_camera_json` — mirrors Rust `Process3dConfig`'s
 * flattened `camera_position`/`camera_target`/`camera_fov` fields. */
export interface Process3dCameraState {
  position: [number, number, number];
  target: [number, number, number];
  fov: number;
}

/** ✏️ The live sun state driving `world3d_scene`'s `WorldSunConfig` — mirrors Rust
 * `Process3dConfig`'s flattened `sun_*` fields. */
export interface Process3dSunState {
  enabled: boolean;
  azimuth: number;
  elevation: number;
  intensity: number;
  color: string;
}

/** ✏️ The Workpiece window's typed view-model — mirrors the Rust `render()` boundary's inputs
 * (the processed-mesh preview itself is derived server-side from `fixture`, not carried here). */
export interface Process3dWorkpieceViewModel {
  windowKindId: "process-workpiece";
  bodyKey: "process.play.main";
  surfaceId: "process.play";
  camera: Process3dCameraState;
  sun: Process3dSunState;
  activeUtilityId: string;
}

export const PROCESS_3D_PLAY_WINDOW_MAIN = "process-workpiece" as const;
export const PROCESS_3D_PLAY_BODY_MAIN = "process.play.main" as const;
export const PROCESS_3D_PLAY_SURFACE_MAIN = "process.play" as const;
