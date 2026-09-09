export interface WriterCamera { x: number; y: number; zoom: number }
export interface WriterEditorSettings { showLineNumbers: boolean; fontPx: number; lineHeight: number; tabSize: number }
export interface WriterMainWindowConfig { camera: WriterCamera; editorSettings: WriterEditorSettings }

export function parseWriterMainWindowConfig(value: unknown): WriterMainWindowConfig {
  if (typeof value !== "object" || value === null || Array.isArray(value)) throw new TypeError("WriterMainWindowConfig must be an object");
  const row = value as Record<string, unknown>;
  const camera = row.camera as Record<string, unknown>;
  const settings = row.editorSettings as Record<string, unknown>;
  if (!camera || typeof camera !== "object" || !settings || typeof settings !== "object") throw new TypeError("WriterMainWindowConfig requires camera and editorSettings");
  for (const field of ["x", "y", "zoom"]) if (typeof camera[field] !== "number") throw new TypeError(`camera.${field} must be numeric`);
  if ((camera.zoom as number) <= 0) throw new TypeError("camera.zoom must be positive");
  if (typeof settings.showLineNumbers !== "boolean") throw new TypeError("editorSettings.showLineNumbers must be boolean");
  for (const field of ["fontPx", "lineHeight", "tabSize"]) if (!Number.isInteger(settings[field]) || (settings[field] as number) < 1) throw new TypeError(`editorSettings.${field} must be a positive integer`);
  return value as WriterMainWindowConfig;
}
