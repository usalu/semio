export interface WiresCanvasCamera { x: number; y: number; zoom: number }
export interface WiresCanvasWindowConfig { camera: WiresCanvasCamera }

export function parseWiresCanvasWindowConfig(value: unknown): WiresCanvasWindowConfig {
  if (typeof value !== "object" || value === null || Array.isArray(value)) throw new TypeError("WiresCanvasWindowConfig must be an object");
  const camera = (value as Record<string, unknown>).camera;
  if (typeof camera !== "object" || camera === null || Array.isArray(camera)) throw new TypeError("WiresCanvasWindowConfig requires camera");
  const row = camera as Record<string, unknown>;
  for (const field of ["x", "y", "zoom"]) if (typeof row[field] !== "number" || !Number.isFinite(row[field])) throw new TypeError(`camera.${field} must be finite`);
  if ((row.zoom as number) <= 0) throw new TypeError("camera.zoom must be positive");
  return value as WiresCanvasWindowConfig;
}
