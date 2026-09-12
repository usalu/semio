export interface CadWorldProjection {
  kind: string;
  orthographicView: string;
  axonometricVariant: string;
  axonometricAngleA: number;
  axonometricAngleB: number;
  axonometricQuadrant: string;
  obliqueVariant: string;
  obliqueAngle: number;
  obliqueDepth: number;
  onePointAxis: string;
  fov: number;
  twoPointShift: number;
  curvilinearFov: number;
  curvilinearStrength: number;
  curvilinearMapping: string;
}

export interface CadWorldCamera {
  position: [number, number, number];
  target: [number, number, number];
  zoom: number;
  fov: number;
  projection: CadWorldProjection;
}

export interface CadWorldSun {
  enabled: boolean;
  azimuth: number;
  elevation: number;
  intensity: number;
  color: string;
}

export interface CadWorldDislocateOptions {
  moveEnabled: boolean;
  rotateEnabled: boolean;
}

export interface CadWorldWindowConfig {
  camera: CadWorldCamera;
  sun: CadWorldSun;
  dislocateOptions: CadWorldDislocateOptions;
}

const exact = (value: unknown, at: string, keys: readonly string[]): Record<string, unknown> => {
  if (value === null || typeof value !== "object" || Array.isArray(value)) throw new TypeError(`${at} must be an object`);
  const row = value as Record<string, unknown>;
  for (const key of Object.keys(row)) if (!keys.includes(key)) throw new TypeError(`${at}.${key} is unknown`);
  for (const key of keys) if (!(key in row)) throw new TypeError(`${at}.${key} is required`);
  return row;
};

const string = (value: unknown, at: string): string => {
  if (typeof value !== "string") throw new TypeError(`${at} must be a string`);
  return value;
};

const number = (value: unknown, at: string): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) throw new TypeError(`${at} must be finite`);
  return value;
};

const bool = (value: unknown, at: string): boolean => {
  if (typeof value !== "boolean") throw new TypeError(`${at} must be a boolean`);
  return value;
};

const point = (value: unknown, at: string): [number, number, number] => {
  if (!Array.isArray(value) || value.length !== 3) throw new TypeError(`${at} must contain three coordinates`);
  return [number(value[0], `${at}[0]`), number(value[1], `${at}[1]`), number(value[2], `${at}[2]`)];
};

const projection = (value: unknown, at: string): CadWorldProjection => {
  const keys = ["kind", "orthographicView", "axonometricVariant", "axonometricAngleA", "axonometricAngleB", "axonometricQuadrant", "obliqueVariant", "obliqueAngle", "obliqueDepth", "onePointAxis", "fov", "twoPointShift", "curvilinearFov", "curvilinearStrength", "curvilinearMapping"] as const;
  const row = exact(value, at, keys);
  return {
    kind: string(row.kind, `${at}.kind`), orthographicView: string(row.orthographicView, `${at}.orthographicView`), axonometricVariant: string(row.axonometricVariant, `${at}.axonometricVariant`),
    axonometricAngleA: number(row.axonometricAngleA, `${at}.axonometricAngleA`), axonometricAngleB: number(row.axonometricAngleB, `${at}.axonometricAngleB`), axonometricQuadrant: string(row.axonometricQuadrant, `${at}.axonometricQuadrant`),
    obliqueVariant: string(row.obliqueVariant, `${at}.obliqueVariant`), obliqueAngle: number(row.obliqueAngle, `${at}.obliqueAngle`), obliqueDepth: number(row.obliqueDepth, `${at}.obliqueDepth`), onePointAxis: string(row.onePointAxis, `${at}.onePointAxis`),
    fov: number(row.fov, `${at}.fov`), twoPointShift: number(row.twoPointShift, `${at}.twoPointShift`), curvilinearFov: number(row.curvilinearFov, `${at}.curvilinearFov`), curvilinearStrength: number(row.curvilinearStrength, `${at}.curvilinearStrength`), curvilinearMapping: string(row.curvilinearMapping, `${at}.curvilinearMapping`),
  };
};

export function parseCadWorldWindowConfig(value: unknown): CadWorldWindowConfig {
  const row = exact(value, "$", ["camera", "sun", "dislocateOptions"]);
  const camera = exact(row.camera, "$.camera", ["position", "target", "zoom", "fov", "projection"]);
  const sun = exact(row.sun, "$.sun", ["enabled", "azimuth", "elevation", "intensity", "color"]);
  const options = exact(row.dislocateOptions, "$.dislocateOptions", ["moveEnabled", "rotateEnabled"]);
  return {
    camera: { position: point(camera.position, "$.camera.position"), target: point(camera.target, "$.camera.target"), zoom: number(camera.zoom, "$.camera.zoom"), fov: number(camera.fov, "$.camera.fov"), projection: projection(camera.projection, "$.camera.projection") },
    sun: { enabled: bool(sun.enabled, "$.sun.enabled"), azimuth: number(sun.azimuth, "$.sun.azimuth"), elevation: number(sun.elevation, "$.sun.elevation"), intensity: number(sun.intensity, "$.sun.intensity"), color: string(sun.color, "$.sun.color") },
    dislocateOptions: { moveEnabled: bool(options.moveEnabled, "$.dislocateOptions.moveEnabled"), rotateEnabled: bool(options.rotateEnabled, "$.dislocateOptions.rotateEnabled") },
  };
}
