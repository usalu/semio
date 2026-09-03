/** 👁️ Generation3d viewer — Preview window: typed twin of `🦀️.rs`'s view-model. Mirrors
 * the pane's `render(document: &Generation3dSnapshot)` boundary — the read-only, freshly-evaluated
 * `MeshWindowKit` scene (contract §2.6). No selection, no gumball, no per-session camera: a viewer
 * has no persisted per-session state (`Config = NoConfig`). */

/** ✏️ The Preview window's typed view-model — mirrors the shared `framework.window.mesh` kind's
 * `MeshView` payload this window builds. */
export interface Generation3dViewPreviewViewModel {
  windowKindId: "framework.window.mesh";
  bodyKey: "framework.window.mesh";
  /** 🧊️ Evaluated + tessellated preview meshes, JSON-encoded (`MeshData[]` keyed by id). */
  meshesJson: string;
  /** 🧊️ World-placed instances referencing `meshesJson` entries, JSON-encoded. */
  instancesJson: string;
}

export const GENERATION3D_VIEW_PREVIEW_WINDOW_KIND_ID = "framework.window.mesh" as const;
export const GENERATION3D_VIEW_PREVIEW_BODY_KEY = "framework.window.mesh" as const;
