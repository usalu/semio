import { parseViewport2d, type Viewport2d } from "../../../../../../../../../../../../../../../../🧰️framework/🔨️modules/🖱️ui/🪟️viewport/◻️2d/🧬️schema/🟦️.ts";

/** 🎚️ Persisted navigation for one exact Generation2d generate preview window. */
export interface Generation2dGeneratePreviewWindowConfig { viewport: Viewport2d }
export type Generation2dGeneratePreviewWindowConfigMutation = { kind: "snapshot"; config: Generation2dGeneratePreviewWindowConfig };

export function parseGeneration2dGeneratePreviewWindowConfig(value: unknown): Generation2dGeneratePreviewWindowConfig {
  if (value === null || typeof value !== "object" || Array.isArray(value)) throw new TypeError("$ must be an object");
  const row = value as Record<string, unknown>;
  if (Object.keys(row).length !== 1 || !("viewport" in row)) throw new TypeError("$ must contain only viewport");
  return { viewport: parseViewport2d(row.viewport) };
}

export function applyGeneration2dGeneratePreviewWindowConfigMutation(_base: Generation2dGeneratePreviewWindowConfig, mutation: Generation2dGeneratePreviewWindowConfigMutation): Generation2dGeneratePreviewWindowConfig {
  return parseGeneration2dGeneratePreviewWindowConfig(mutation.config);
}
