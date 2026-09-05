/** 🔺️ generation2d update-camera diff — mirrors `diff()` (…/🎛️set-camera/🔺️diff/🦀️.rs), a scalar-facet write on the fixture's camera. */
import type { CameraJson, UpdateCamera } from "../🦠️mutation/🟦️.ts";

export interface UpdateCameraDiff {
  camera: CameraJson;
}

export function diff(payload: UpdateCamera): UpdateCameraDiff {
  return { camera: payload.camera };
}
