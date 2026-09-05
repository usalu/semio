/** ↩️ generation2d update-camera inverse — mirrors `inverse()` (…/🎛️set-camera/↩️inverse/🦀️.rs): unconditionally restores the captured BASE camera (the camera field always exists). */
import type { CameraJson, UpdateCamera } from "../🦠️mutation/🟦️.ts";

export function inverse(_payload: UpdateCamera, baseCamera: CameraJson): UpdateCamera[] {
  return [{ camera: baseCamera }];
}
