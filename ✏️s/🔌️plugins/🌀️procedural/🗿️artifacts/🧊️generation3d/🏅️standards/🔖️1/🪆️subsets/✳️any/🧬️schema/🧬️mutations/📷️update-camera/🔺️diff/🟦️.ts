/** 🔺️ generation3d update-camera/🔺️diff — mirror of the whole-artifact camera-field delta builder. */
import type { UpdateCamera, CameraJson } from "../🦠️mutation/🟦️.ts";

export function diff(payload: UpdateCamera): { camera: CameraJson } {
  return { camera: payload.camera };
}
