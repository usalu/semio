/** 🎚️ Generation3dViewConfig — typed twin of `🦀️.rs` / `🔣️.json`, the read-only surface's own
 * persisted view state. Hover and selection are absent by design: they belong to the framework's
 * `graph` interaction domain, never to a surface's config. */

/** 🧬️ Generation3dViewConfig */
export interface Generation3dViewConfig {
  /** @state config */
  lodMode: string;
  /** @state config */
  showMode: string;
  /** @state config */
  previewCamera: Generation3dViewCamera;
  /** @state config */
  sunJson: string;
  /** 🎨️ The bundled example this surface is looking at (`""` = the opened document itself).
   * @state config */
  activeExampleId: string;
}

export type Generation3dViewCamera = {
  position: number[];
  target: number[];
  fov: number;
};

export const GENERATION3D_VIEW_SHOW_MODES = ["shaded", "shaded+edges", "wireframe", "points"] as const;
export type Generation3dViewShowMode = (typeof GENERATION3D_VIEW_SHOW_MODES)[number];

export const GENERATION3D_VIEW_LOD_MODES = ["", "coarse", "fine"] as const;
export type Generation3dViewLodMode = (typeof GENERATION3D_VIEW_LOD_MODES)[number];

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every parser below rejects with. */
export class Generation3dViewConfigGuardRefusal extends Error {
  constructor(
    readonly at: string,
    readonly why: string,
  ) {
    super(`${at}: ${why}`);
  }
}

const reject = (at: string, why: string): never => {
  throw new Generation3dViewConfigGuardRefusal(at, why);
};

const guardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : reject(at, "value is not an object");

const guardString = (value: unknown, at: string): string => (typeof value === "string" ? value : reject(at, "value is not a string"));

const guardNumber = (value: unknown, at: string): number => (typeof value === "number" && Number.isFinite(value) ? value : reject(at, "value is not a finite number"));

const guardTriple = (value: unknown, at: string): number[] => {
  if (!Array.isArray(value) || value.length !== 3) return reject(at, "value is not a 3-element array");
  return value.map((item, index) => guardNumber(item, `${at}[${index}]`));
};

export function parseGeneration3dViewCamera(value: unknown, at = "$"): Generation3dViewCamera {
  const row = guardObject(value, at);
  return { position: guardTriple(row["position"], `${at}.position`), target: guardTriple(row["target"], `${at}.target`), fov: guardNumber(row["fov"], `${at}.fov`) };
}

export function parseGeneration3dViewConfig(value: unknown, at = "$"): Generation3dViewConfig {
  const row = guardObject(value, at);
  return {
    lodMode: guardString(row["lodMode"], `${at}.lodMode`),
    showMode: guardString(row["showMode"], `${at}.showMode`),
    previewCamera: parseGeneration3dViewCamera(row["previewCamera"], `${at}.previewCamera`),
    sunJson: guardString(row["sunJson"], `${at}.sunJson`),
    activeExampleId: guardString(row["activeExampleId"], `${at}.activeExampleId`),
  };
}
//#endregion 🚪️Parsers
