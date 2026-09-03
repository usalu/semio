/** 👁️ Generation3d editor — Preview window (edit mode): typed twin of `🦀️.rs`'s view-model.
 * Mirrors the pane's `render(document: &Generation3dSnapshot, config: &Generation3dConfig, session:
 * &FlowEvalSession, activeUtility: &str)` boundary — the tessellated evaluated-fixture world-3d scene
 * plus the transform-gumball utility state a mutation-capable surface carries (absent entirely from
 * the viewer's read-only twin, see `👁️viewer/…/🟦️.ts`). */

/** ✏️ The Preview window's typed view-model — mirrors the Rust `render()` boundary's inputs. */
export interface Generation3dPreviewViewModel {
  windowKindId: "procedural-preview";
  bodyKey: "procedural.play.preview";
  surfaceId: "procedural.play.preview";
  /** 🎚️ Level-of-detail tessellation mode: `""` (default/medium) | "coarse" | "medium" | "fine". */
  lodMode: string;
  /** 👁️ Shading mode: "shaded" | "shaded+edges" | "wireframe" | "points". */
  showMode: string;
  /** 🧰️ The active transform-gumball utility id ("move" | "rotate" | "scale"), or null if none. */
  activeUtilityId: string | null;
}

export const GENERATION3D_PLAY_PREVIEW_WINDOW_KIND_ID = "procedural-preview" as const;
export const GENERATION3D_PLAY_PREVIEW_BODY_KEY = "procedural.play.preview" as const;
export const GENERATION3D_PLAY_PREVIEW_SURFACE_ID = "procedural.play.preview" as const;
