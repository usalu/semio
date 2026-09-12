export type LayoutCamera = { x: number; y: number; zoom: number };
export type LayoutWindowConfig = { activePageId: string; camera: LayoutCamera };
export type LayoutWindowConfigMutation = { kind: "snapshot"; config: LayoutWindowConfig };

export function parseLayoutWindowConfig(value: unknown): LayoutWindowConfig {
  if (!value || typeof value !== "object") throw new Error("layout-window-config-object-required");
  const record = value as Record<string, unknown>;
  const camera = record.camera as Record<string, unknown> | undefined;
  if (typeof record.activePageId !== "string" || !camera || typeof camera.x !== "number" || typeof camera.y !== "number" || typeof camera.zoom !== "number" || camera.zoom <= 0) {
    throw new Error("layout-window-config-invalid");
  }
  return value as LayoutWindowConfig;
}

export function applyLayoutWindowConfigMutation(_base: LayoutWindowConfig, mutation: LayoutWindowConfigMutation): LayoutWindowConfig {
  if (mutation.kind !== "snapshot") throw new Error("layout-window-config-mutation-invalid");
  return parseLayoutWindowConfig(mutation.config);
}
