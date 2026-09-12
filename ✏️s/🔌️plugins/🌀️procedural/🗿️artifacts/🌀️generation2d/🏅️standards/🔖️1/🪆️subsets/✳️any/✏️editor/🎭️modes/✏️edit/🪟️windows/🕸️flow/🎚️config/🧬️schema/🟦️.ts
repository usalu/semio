import { parseViewport2d, type Viewport2d } from "../../../../../../../../../../../../../../../../🧰️framework/🔨️modules/🖱️ui/🪟️viewport/◻️2d/🧬️schema/🟦️.ts";

/** 🎚️ Persisted navigation for one exact Generation2d main window. */
export interface Generation2dMainWindowConfig { viewport: Viewport2d }
export type Generation2dMainWindowConfigMutation = { kind: "snapshot"; config: Generation2dMainWindowConfig };

export function parseGeneration2dMainWindowConfig(value: unknown): Generation2dMainWindowConfig {
  if (value === null || typeof value !== "object" || Array.isArray(value)) throw new TypeError("$ must be an object");
  const row = value as Record<string, unknown>;
  if (Object.keys(row).length !== 1 || !("viewport" in row)) throw new TypeError("$ must contain only viewport");
  return { viewport: parseViewport2d(row.viewport) };
}

export function applyGeneration2dMainWindowConfigMutation(_base: Generation2dMainWindowConfig, mutation: Generation2dMainWindowConfigMutation): Generation2dMainWindowConfig {
  return parseGeneration2dMainWindowConfig(mutation.config);
}
