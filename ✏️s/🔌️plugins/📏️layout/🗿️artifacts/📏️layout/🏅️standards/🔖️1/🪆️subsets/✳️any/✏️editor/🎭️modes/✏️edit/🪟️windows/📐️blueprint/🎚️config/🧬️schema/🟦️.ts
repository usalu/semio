export type LayoutCamera = { x: number; y: number; zoom: number };
export type LayoutWindowConfig = { activePageId: string; activeUtility: string; camera: LayoutCamera };
export type LayoutWindowConfigPatch = { [Key in keyof LayoutWindowConfig]?: LayoutWindowConfig[Key] | null };
export type LayoutWindowConfigMutation = { kind: "update"; patch: LayoutWindowConfigPatch };

export function parseLayoutWindowConfig(value: unknown): LayoutWindowConfig {
  if (!value || typeof value !== "object") throw new Error("layout-window-config-object-required");
  const record = value as Record<string, unknown>;
  const camera = record.camera as Record<string, unknown> | undefined;
  if (typeof record.activePageId !== "string" || typeof record.activeUtility !== "string" || !camera || typeof camera.x !== "number" || typeof camera.y !== "number" || typeof camera.zoom !== "number" || camera.zoom <= 0) {
    throw new Error("layout-window-config-invalid");
  }
  return value as LayoutWindowConfig;
}

export function applyLayoutWindowConfigMutation(base: LayoutWindowConfig, mutation: LayoutWindowConfigMutation): LayoutWindowConfig {
  if (mutation.kind !== "update" || !mutation.patch) throw new Error("layout-window-config-mutation-invalid");
  return parseLayoutWindowConfig({
    activePageId: mutation.patch.activePageId ?? base.activePageId,
    activeUtility: mutation.patch.activeUtility ?? base.activeUtility,
    camera: mutation.patch.camera ?? base.camera,
  });
}
