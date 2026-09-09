import type { WiresCanvasCamera, WiresCanvasWindowConfig } from "../🟦️";

export type WiresCanvasWindowConfigMutation = { kind: "set-camera"; camera: WiresCanvasCamera };

export function applyWiresCanvasWindowConfigMutation(_state: WiresCanvasWindowConfig, mutation: WiresCanvasWindowConfigMutation): WiresCanvasWindowConfig {
  return { camera: mutation.camera };
}
