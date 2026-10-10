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
  /** 🎨️ What this surface is looking at: absent = the opened document, `""` = the picker's
   * `No example` row, anything else = that bundled example.
   * @state config */
  activeExampleId?: string;
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
