import { parseViewport2d, type Viewport2d } from "../../../../../../../../../../../../../../../../🧰️framework/🔨️modules/🖱️ui/🪟️viewport/◻️2d/🧬️schema/🟦️.ts";

/** 🎚️ Persisted navigation for one exact Generation2d edit preview window. */
export interface Generation2dEditPreviewWindowConfig { viewport: Viewport2d }
export type Generation2dEditPreviewWindowConfigMutation = { kind: "snapshot"; config: Generation2dEditPreviewWindowConfig };

export function parseGeneration2dEditPreviewWindowConfig(value: unknown): Generation2dEditPreviewWindowConfig {
  if (value === null || typeof value !== "object" || Array.isArray(value)) throw new TypeError("$ must be an object");
  const row = value as Record<string, unknown>;
  if (Object.keys(row).length !== 1 || !("viewport" in row)) throw new TypeError("$ must contain only viewport");
  return { viewport: parseViewport2d(row.viewport) };
}

export function applyGeneration2dEditPreviewWindowConfigMutation(_base: Generation2dEditPreviewWindowConfig, mutation: Generation2dEditPreviewWindowConfigMutation): Generation2dEditPreviewWindowConfig {
  return parseGeneration2dEditPreviewWindowConfig(mutation.config);
}
