export interface EquationCamera { x: number; y: number; zoom: number }
export interface EquationGraphWindowConfig { camera: EquationCamera }

export function parseEquationGraphWindowConfig(value: unknown): EquationGraphWindowConfig {
  if (typeof value !== "object" || value === null || Array.isArray(value)) throw new TypeError("EquationGraphWindowConfig must be an object");
  const camera = (value as Record<string, unknown>).camera;
  if (typeof camera !== "object" || camera === null || Array.isArray(camera)) throw new TypeError("EquationGraphWindowConfig requires camera");
  const row = camera as Record<string, unknown>;
  for (const field of ["x", "y", "zoom"]) if (typeof row[field] !== "number" || !Number.isFinite(row[field])) throw new TypeError(`camera.${field} must be finite`);
  if ((row.zoom as number) <= 0) throw new TypeError("camera.zoom must be positive");
  return value as EquationGraphWindowConfig;
}
