/** ↩️ procedural2d update-camera inverse — mirrors `inverse()` (…/🎛set-camera/↩️inverse/🦀️component.rs): unconditionally restores the captured BASE camera (the camera field always exists). */
import type { CameraJson, UpdateCamera } from "../🦠️mutation/🟦️component.ts";

export function inverse(_payload: UpdateCamera, baseCamera: CameraJson): UpdateCamera[] {
  return [{ camera: baseCamera }];
}
