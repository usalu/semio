/** ↩️ procedural3d update-camera/↩️inverse — mirror of the self-inverse pre-state camera restore. */
import type { UpdateCamera, CameraJson } from "../🦠️mutation/🟦️component.ts";

export function inverse(_payload: UpdateCamera, baseCamera: CameraJson): UpdateCamera[] {
  return [{ camera: baseCamera }];
}
