export type SequenceCamera = { x: number; y: number; zoom: number };
export type SequenceMainWindowConfig = { orientation: "leftRight" | "topBottom"; camera: SequenceCamera };
export type SequenceMainWindowConfigMutation =
  | { kind: "set-orientation"; value: SequenceMainWindowConfig["orientation"] }
  | { kind: "set-camera"; value: SequenceCamera };

export function parseSequenceMainWindowConfig(value: unknown): SequenceMainWindowConfig {
  if (!value || typeof value !== "object") throw new Error("sequence-main-window-config-object-required");
  const record = value as Record<string, unknown>;
  const camera = record.camera as Record<string, unknown> | undefined;
  if ((record.orientation !== "leftRight" && record.orientation !== "topBottom") || !camera || typeof camera.x !== "number" || typeof camera.y !== "number" || typeof camera.zoom !== "number" || camera.zoom <= 0) {
    throw new Error("sequence-main-window-config-invalid");
  }
  return value as SequenceMainWindowConfig;
}

export function applySequenceMainWindowConfigMutation(base: SequenceMainWindowConfig, mutation: SequenceMainWindowConfigMutation): SequenceMainWindowConfig {
  if (mutation.kind === "set-orientation") return parseSequenceMainWindowConfig({ ...base, orientation: mutation.value });
  if (mutation.kind === "set-camera") return parseSequenceMainWindowConfig({ ...base, camera: mutation.value });
  throw new Error("sequence-main-window-config-mutation-invalid");
}
